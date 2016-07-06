use gssapi_sys;
use std::ptr;
use super::error::{Error, Result};
use super::oid;
use super::buffer::Buffer;

#[derive(Debug)]
pub struct Name {
    name: gssapi_sys::gss_name_t,
}

impl Name {
    pub fn new<T: Into<Buffer>>(name: T, name_type: oid::OID) -> Result<Self> {
        let mut name = name.into();
        let mut minor_status = 0;
        let mut gss_name = ptr::null_mut();

        let major_status = unsafe {
            gssapi_sys::gss_import_name(&mut minor_status,
                                        name.get_handle(),
                                        name_type,
                                        &mut gss_name)
        };

        if major_status == gssapi_sys::GSS_S_COMPLETE {
            Ok(Name {
                name: gss_name,
            })
        } else {
            Err(Error::new(major_status, minor_status))
        }
    }

    /// Temporarily get wrapped value.
    pub unsafe fn get_handle<'a>(&self) -> gssapi_sys::gss_name_t {
        self.name
    }
}

impl Drop for Name {
    fn drop(&mut self) {
        let mut min_stat = 0;
        let maj_stat = unsafe {
            gssapi_sys::gss_release_name(&mut min_stat, &mut self.name)
        };

        if maj_stat != gssapi_sys::GSS_S_COMPLETE {
            panic!("failed to release name!");
        }
    }
}
