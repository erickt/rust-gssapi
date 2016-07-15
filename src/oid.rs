use gssapi_sys;
use std::ptr;
use error::Error;

#[derive(Clone, Debug)]
pub struct OID {
    oid: gssapi_sys::gss_OID,
    owned: bool,
}

impl OID {
    pub unsafe fn new(oid: gssapi_sys::gss_OID) -> Self {
        OID {
            oid: oid,
            owned: true,
        }
    }

    pub fn empty() -> Self {
        unsafe { OID::new(ptr::null_mut()) }
    }

    pub fn nt_hostbased_service() -> Self {
        unsafe {
            OID {
                oid: gssapi_sys::GSS_C_NT_HOSTBASED_SERVICE,
                owned: false,
            }
        }
    }
    
    pub fn nt_user_name() -> Self {
        unsafe {
            OID {
                oid: gssapi_sys::GSS_C_NT_USER_NAME,
                owned: false,
            }
        }
    }

    pub unsafe fn get_handle(&self) -> gssapi_sys::gss_OID {
        self.oid
    }
}

impl Drop for OID {
    fn drop(&mut self) {
        println!("dropping oid");

        /*
        let mut minor_status = 0;
        let major_status = unsafe {
            gssapi_sys::gss_release_oid(
                &mut minor_status,
                &mut self.oid)
        };

        if major_status != gssapi_sys::GSS_S_COMPLETE {
            let err = Error::new(major_status, minor_status, OID::empty());
            panic!("{}", err);
        }
        */

        println!("drop oid");
    }
}
