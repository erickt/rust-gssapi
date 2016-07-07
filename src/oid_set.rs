use gssapi_sys;
use std::ptr;
use super::error::{Error, Result};

pub struct OIDSet {
    oid_set: gssapi_sys::gss_OID_set,
}

impl OIDSet {
    pub fn empty() -> Result<Self> {
        let mut minor_status = 0;
        let mut oid_set = ptr::null_mut();

        let major_status = unsafe {
            gssapi_sys::gss_create_empty_oid_set(
                &mut minor_status,
                &mut oid_set)
        };

        if major_status == gssapi_sys::GSS_S_COMPLETE {
            Ok(OIDSet {
                oid_set: oid_set,
            })
        } else {
            Err(Error::new(major_status, minor_status))
        }
    }

    /// Temporarily get wrapped value.
    pub unsafe fn get_handle(&self) -> gssapi_sys::gss_OID_set {
        self.oid_set
    }
}

impl Drop for OIDSet {
    fn drop(&mut self) {
        let mut minor_status = 0;
        let major_status = unsafe {
            gssapi_sys::gss_release_oid_set(
                &mut minor_status,
                &mut self.oid_set)
        };

        if major_status != gssapi_sys::GSS_S_COMPLETE {
            let err = Error::new(major_status, minor_status);
            panic!("{}", err);
        }
    }
}
