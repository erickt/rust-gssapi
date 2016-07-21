use gssapi_sys;
use std::ptr;

use super::buffer::Buffer;
use super::credentials::Credentials;
use super::error::{Error, Result};
use super::name::Name;
use super::oid::OID;

#[derive(Debug)]
pub struct Context {
    context_handle: gssapi_sys::gss_ctx_id_t,
    mech_type: OID,
    time_rec: u32,
    flags: u32,
}

impl Context {
    pub fn initiate<T: Into<Name>>(target_name: T) -> InitiateContextBuilder {
        InitiateContextBuilder::new(target_name)
    }

    pub fn accept(credentials: Credentials) -> AcceptContextBuilder {
        AcceptContextBuilder::new(credentials)
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

impl Drop for Context {
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
pub struct InitiateContextBuilder {
    state: InitiateContextState,
}

impl InitiateContextBuilder {
    pub fn new<T: Into<Name>>(target_name: T) -> Self {
        InitiateContextBuilder {
            state: InitiateContextState::new(target_name),
        }
    }

    pub fn mech_type(mut self, mech_type: OID) -> Self {
        self.state.mech_type = mech_type;
        self
    }

    pub fn flags(mut self, flags: u32) -> Self {
        self.state.flags |= flags;
        self
    }

    pub fn step(self) -> Result<(InitiateContextState, Buffer)> {
        match try!(self.state.step(Buffer::new())) {
            InitiateContext::Continue { initializer, token } => {
                Ok((initializer, token))
            },
            InitiateContext::Done { .. } => {
                unreachable!("succeeded at connecting without talking to server?")
            }
        }
    }
}

#[derive(Debug)]
pub enum InitiateContext {
    Continue {
        initializer: InitiateContextState,
        token: Buffer,
    },
    Done {
        context: Context,
    },
}

#[derive(Debug)]
pub struct InitiateContextState {
    target_name: Name,
    mech_type: OID,
    flags: u32,
    context_handle: gssapi_sys::gss_ctx_id_t,
}

impl InitiateContextState {
    fn new<T: Into<Name>>(target_name: T) -> Self {
        let target_name = target_name.into();

        InitiateContextState {
            target_name: target_name,
            mech_type: OID::empty(),
            flags: 0,
            context_handle: ptr::null_mut(),
        }
    }

    pub fn step(mut self, mut input_token: Buffer) -> Result<InitiateContext> {
        let mut minor_status = 0;
        let claimant_cred_handle = ptr::null_mut(); // no credentials
        let time_req = 0;
        let input_chan_bindings = ptr::null_mut(); // no channel bindings
        let mut actual_mech_type = OID::empty(); // ignore mech type
        let mut output_token = Buffer::new();
        let mut ret_flags = 0;
        let mut time_rec = 0;

        let major_status = unsafe {
            gssapi_sys::gss_init_sec_context(
                &mut minor_status,
                claimant_cred_handle,
                &mut self.context_handle,
                self.target_name.get_handle(),
                self.mech_type.get_handle(),
                self.flags,
                time_req,
                input_chan_bindings,
                input_token.get_handle(),
                &mut actual_mech_type.get_handle(),
                output_token.get_handle(),
                &mut ret_flags,
                &mut time_rec,
            )
        };

        if self.context_handle.is_null() {
            panic!("cannot create context");
        }

        let actual_mech_type = OID::empty();

        if major_status == gssapi_sys::GSS_S_COMPLETE {
            Ok(InitiateContext::Done {
                context: Context {
                    context_handle: self.context_handle,
                    mech_type: actual_mech_type,
                    time_rec: time_rec,
                    flags: ret_flags,
                },
            })
        } else if major_status == gssapi_sys::GSS_S_CONTINUE_NEEDED {
            Ok(InitiateContext::Continue {
                initializer: self,
                token: output_token,
            })
        } else {
            Err(Error::new(major_status, minor_status, actual_mech_type))
        }
    }

    pub fn final_step(self, input_token: Buffer) -> Result<Context> {
        match try!(self.step(input_token)) {
            InitiateContext::Done { context } => Ok(context),
            InitiateContext::Continue { .. } => {
                panic!("Server is done but client didn't finish?")
            }
        }
    }
}

#[derive(Debug)]
pub struct AcceptContextBuilder {
    state: AcceptContextState,
}

impl AcceptContextBuilder {
    pub fn new(credentials: Credentials) -> Self {
        AcceptContextBuilder {
            state: AcceptContextState::new(credentials),
        }
    }

    pub fn step(self, input_token: Buffer) -> Result<AcceptContext> {
        self.state.step(input_token)
    }
}

#[derive(Debug)]
pub enum AcceptContext {
    Continue {
        acceptor: AcceptContextState,
        token: Buffer,
    },
    Done {
        context: Context,
        token: Buffer,
    },
}

#[derive(Debug)]
pub struct AcceptContextState {
    acceptor_credentials: Credentials,
    context_handle: gssapi_sys::gss_ctx_id_t,
}

impl AcceptContextState {
    fn new(credentials: Credentials) -> Self {
        AcceptContextState {
            acceptor_credentials: credentials,
            context_handle: ptr::null_mut(),
        }
    }

    pub fn step(mut self, mut input_token: Buffer) -> Result<AcceptContext> {
        let mut minor_status = 0;
        let input_chan_bindings = ptr::null_mut(); // no channel bindings
        let mut src_name = ptr::null_mut();
        let mut mech_type = OID::empty(); // ignore mech type
        let mut output_token = Buffer::new();
        let mut ret_flags = 0;
        let mut time_rec = 0;
        let mut delegated_cred_handle = ptr::null_mut();

        let major_status = unsafe {
            gssapi_sys::gss_accept_sec_context(
                &mut minor_status,
                &mut self.context_handle,
                self.acceptor_credentials.get_handle(),
                input_token.get_handle(),
                input_chan_bindings,
                &mut src_name,
                &mut mech_type.get_handle(),
                output_token.get_handle(),
                &mut ret_flags,
                &mut time_rec,
                &mut delegated_cred_handle,
            )
        };

        if self.context_handle.is_null() {
            panic!("cannot create context");
        }

        // let mech_type = unsafe { OID::new(mech_type) };

        if major_status == gssapi_sys::GSS_S_COMPLETE {
            Ok(AcceptContext::Done {
                context: Context {
                    context_handle: self.context_handle,
                    mech_type: mech_type,
                    time_rec: time_rec,
                    flags: ret_flags,
                },
                token: output_token,
            })
        } else if major_status == gssapi_sys::GSS_S_CONTINUE_NEEDED {
            Ok(AcceptContext::Continue {
                acceptor: self,
                token: output_token,
            })
        } else {
            Err(Error::new(major_status, minor_status, mech_type))
        }
    }
}
