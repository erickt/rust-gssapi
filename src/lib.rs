extern crate gssapi_sys;

use std::ptr;

pub struct Name {
    name: *mut gssapi_sys::gss_name_t,
}

pub struct Credentials {
    cred_handle: *mut gssapi_sys::gss_cred_id_t,
}

impl Credentials {
    fn new(name: Option<Name>, time_req: u32, mechs: ) -> Credentials {
        unsafe {
            let name = match name {
                Some(name) => name.name,
                None => ptr::null(),
            };

            let mut minor_status = 0;
            let time_req = 0;
            let desired_mechs = ptr::null();

            gssapi_sys::gss_acquire_cred(&mut minor_status,
                                         name.name,
                                         time_req,
                                         desired_mechs,
                                         c_usage,
                                         creds,


        }
    }
}

impl Drop for Credentials {
    fn drop(&mut self) {
        unsafe {
            let mut minor_status = 0;
            gssapi_sys::gss_release_cred(&mut minor_status,
                                         &mut self.cred_handle)
        }
    }
}

pub struct SecurityContext;
