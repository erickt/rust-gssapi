use gssapi_sys;
use std::ptr;

use super::buffer::Buffer;
use super::error::{Error, Result};
use super::name::Name;
use super::oid::OID;

#[derive(Debug)]
pub struct SecurityContext {
    context_handle: gssapi_sys::gss_ctx_id_t,
    mech_type: OID,
    time_rec: u32,
    flags: u32,
}

impl SecurityContext {
    pub fn builder<T: Into<Name>>(target_name: T) -> SecurityContextBuilder {
        SecurityContextBuilder::new(target_name)
    }

    pub fn mech_type(&self) -> &OID {
        &self.mech_type
    }

    pub fn time_rec(&self) -> u32 {
        self.time_rec
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }
}

impl Drop for SecurityContext {
    fn drop(&mut self) {
        let mut minor_status = 0;
        let major_status = unsafe {
            gssapi_sys::gss_delete_sec_context(
                &mut minor_status,
                &mut self.context_handle,
                ptr::null_mut())
        };

        if major_status != gssapi_sys::GSS_S_COMPLETE {
            panic!("failed to drop context {} {}", major_status, minor_status)
        }
    }
}


#[derive(Debug)]
pub struct SecurityContextBuilder {
    builder: SecurityContextInitializer,
}

impl SecurityContextBuilder {
    pub fn new<T: Into<Name>>(target_name: T) -> Self {
        let target_name = target_name.into();

        SecurityContextBuilder {
            builder: SecurityContextInitializer {
                target_name: target_name,
                mech_type: ptr::null_mut(),
                flags: 0,
                context_handle: ptr::null_mut(),
            },
        }
    }

    pub fn mech_type(mut self, mech_type: OID) -> Self {
        self.builder.mech_type = mech_type;
        self
    }

    pub fn flags(mut self, flags: u32) -> Self {
        self.builder.flags |= flags;
        self
    }

    pub fn step(self) -> Result<SecurityContextInitializerStep> {
        self.builder.step(Buffer::new())
    }
}


#[derive(Debug)]
pub enum SecurityContextInitializerStep {
    Continue(SecurityContextInitializer, Buffer),
    Done(SecurityContext),
}

#[derive(Debug)]
pub struct SecurityContextInitializer {
    target_name: Name,
    mech_type: OID,
    flags: u32,
    context_handle: gssapi_sys::gss_ctx_id_t,
}

impl SecurityContextInitializer {
    pub fn step(mut self, mut input_token: Buffer) -> Result<SecurityContextInitializerStep> {
        let mut minor_status = 0;
        let initiator_cred_handle = ptr::null_mut(); // no credentials
        let time_req = 0;
        let input_chan_bindings = ptr::null_mut(); // no channel bindings
        let mut actual_mech_type = ptr::null_mut(); // ignore mech type
        let mut output_token = Buffer::new();
        let mut ret_flags = 0;
        let mut time_rec = 0;

        let major_status = unsafe {
            gssapi_sys::gss_init_sec_context(
                &mut minor_status,
                initiator_cred_handle,
                &mut self.context_handle,
                self.target_name.get_handle(),
                self.mech_type,
                self.flags,
                time_req,
                input_chan_bindings,
                input_token.get_handle(),
                &mut actual_mech_type,
                output_token.get_handle(),
                &mut ret_flags,
                &mut time_rec,
            )
        };

        if self.context_handle.is_null() {
            panic!("cannot create context");
        }

        if major_status == gssapi_sys::GSS_S_COMPLETE {
            Ok(SecurityContextInitializerStep::Done(SecurityContext {
                context_handle: self.context_handle,
                mech_type: actual_mech_type,
                time_rec: time_rec,
                flags: ret_flags,
            }))
        } else if major_status == gssapi_sys::GSS_S_CONTINUE_NEEDED {
            Ok(SecurityContextInitializerStep::Continue(self, output_token))
        } else {
            Err(Error::new(major_status, minor_status))
        }
    }
}
