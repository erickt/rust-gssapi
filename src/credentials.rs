use error::{Error, Result};
use gssapi_sys;
use name::Name;
use oid_set::OIDSet;
use std::ptr;

pub struct Credentials {
    cred_handle: gssapi_sys::gss_cred_id_t,
    mechs: OIDSet,
    time_rec: u32,
}

impl Credentials {
    pub fn builder<T: Into<Name>>(desired_name: T) -> CredentialsBuilder {
        CredentialsBuilder::new(desired_name)
    }

    pub fn mechs(&self) -> &OIDSet {
        &self.mechs
    }

    pub fn time_rec(&self) -> u32 {
        self.time_rec
    }
}

impl Drop for Credentials {
    fn drop(&mut self) {
        let mut minor_status = 0;

        let major_status = unsafe {
            gssapi_sys::gss_release_cred(
                &mut minor_status,
                &mut self.cred_handle)
        };

        if major_status != gssapi_sys::GSS_S_COMPLETE {
            panic!("{}", Error::new(major_status, minor_status))
        }
    }
}

pub struct CredentialsBuilder {
    desired_name: Name,
    time_req: u32,
    desired_mechs: OIDSet,
    cred_usage: isize,
}

impl CredentialsBuilder {
    pub fn new<T: Into<Name>>(desired_name: T) -> Self {
        CredentialsBuilder {
            desired_name: desired_name.into(),
            time_req: 0,
            desired_mechs: OIDSet::empty().unwrap(),
            cred_usage: 0,
        }
    }

    pub fn time_req(mut self, time_req: u32) -> Self {
        self.time_req = time_req;
        self
    }

    pub fn build(self) -> Result<Credentials> {
        let mut minor_status = 0;
        let mut output_cred_handle: gssapi_sys::gss_cred_id_t = ptr::null_mut();
        let actual_mechs = try!(OIDSet::empty());
        let mut time_rec = 0;

        let major_status = unsafe {
            gssapi_sys::gss_acquire_cred(
                &mut minor_status,
                self.desired_name.get_handle(),
                self.time_req,
                self.desired_mechs.get_handle(),
                self.cred_usage as gssapi_sys::gss_cred_usage_t,
                &mut output_cred_handle,
                &mut actual_mechs.get_handle(),
                &mut time_rec,
            )
        };

        if major_status == gssapi_sys::GSS_S_COMPLETE {
            Ok(Credentials {
                cred_handle: output_cred_handle,
                mechs: actual_mechs,
                time_rec: time_rec,
            })
        } else {
            Err(Error::new(major_status, minor_status))
        }
    }
}
