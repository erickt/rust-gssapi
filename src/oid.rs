use gssapi_sys;
use std::ptr;

#[derive(Clone, Debug)]
pub struct OID {
    oid: gssapi_sys::gss_OID,
}

impl OID {
    pub unsafe fn new(oid: gssapi_sys::gss_OID) -> Self {
        OID {
            oid: oid,
        }
    }

    pub fn empty() -> Self {
        unsafe { OID::new(ptr::null_mut()) }
    }

    pub fn nt_hostbased_service() -> Self {
        unsafe { OID::new(gssapi_sys::GSS_C_NT_HOSTBASED_SERVICE) }
    }

    pub unsafe fn get_handle(&self) -> gssapi_sys::gss_OID {
        self.oid
    }
}
