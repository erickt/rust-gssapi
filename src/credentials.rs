use error::{Error, Result};
use gssapi_sys;
use name::Name;
use oid::OID;
use oid_set::OIDSet;
use std::ptr;

#[cfg(feature = "gssapi_ext")]
use std::ffi::CString;

#[derive(Debug)]
pub struct Credentials {
    cred_handle: gssapi_sys::gss_cred_id_t,
    mechs: OIDSet,
    time_rec: u32,
}

impl Credentials {
    pub fn accept<T: Into<Name>>(desired_name: T) -> CredentialsBuilder {
        CredentialsBuilder::new(desired_name)
    }

    pub fn mechs(&self) -> &OIDSet {
        &self.mechs
    }

    pub fn time_rec(&self) -> u32 {
        self.time_rec
    }

    pub unsafe fn get_handle(&self) -> gssapi_sys::gss_cred_id_t {
        self.cred_handle
    }
    
    #[cfg(feature = "gssapi_ext")]
    pub fn impersonate<T: Into<Name>>(self, desired_name: T) -> CredentialsBuilder {
        CredentialsBuilder::new(desired_name).impersonator(self)
    }
    
    #[cfg(feature = "gssapi_ext")]
    pub unsafe fn bytes(self) -> Result<Vec<u8>> {
        let mut kvs = gssapi_sys::gss_key_value_set_struct{
            count: 0,
            elements: ptr::null_mut(),
        };
        
        let mut minor_status = 0;

        // Example usage: https://github.com/krb5/krb5/blob/master/src/tests/gssapi/t_credstore.c#L77
        let major_status = gssapi_sys::gss_store_cred_into(
            &mut minor_status, /* minor_status */
            self.cred_handle, /* input_cred_handle */
            0, /* input_usage */
            ptr::null_mut(), /* desired_mech */
            1, /* overwrite_cred */
            0, /* default_cred */
            &mut kvs as gssapi_sys::gss_const_key_value_set_t, /* cred_store */
            ptr::null_mut(), /* elements_stored */
            ptr::null_mut(), /* cred_usage_stored */
        );
        
        // FIXME: How to deallocate what's now pointed to by 'elements' ?
        // https://github.com/krb5/krb5/blob/master/src/tests/gssapi/t_credstore.c#L135
        
        if major_status == gssapi_sys::GSS_S_COMPLETE {
            if kvs.count != 1 {
                // FIXME: How to show some information in this case?
                Err(Error::new(0, 0, OID::empty()))
            } else {
                Ok(CString::from_raw((*kvs.elements).value as *mut i8).into_bytes())
            }
        } else {
            Err(Error::new(major_status, minor_status, OID::empty()))
        }
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
            panic!("{}", Error::new(major_status, minor_status, OID::empty()))
        }
    }
}

#[cfg(feature = "gssapi_ext")]
pub struct CredentialsBuilder {
    desired_name: Name,
    time_req: u32,
    desired_mechs: OIDSet,
    cred_usage: isize,
    impersonator: Option<Credentials>
}

#[cfg(not(feature = "gssapi_ext"))]
pub struct CredentialsBuilder {
    desired_name: Name,
    time_req: u32,
    desired_mechs: OIDSet,
    cred_usage: isize,
}

impl CredentialsBuilder {
    #[cfg(feature = "gssapi_ext")]
    pub fn new<T: Into<Name>>(desired_name: T) -> Self {
        CredentialsBuilder {
            desired_name: desired_name.into(),
            time_req: 0,
            desired_mechs: OIDSet::empty().unwrap(),
            cred_usage: 0,
            impersonator: None
        }
    }

    #[cfg(not(feature = "gssapi_ext"))]
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
    
    #[cfg(feature = "gssapi_ext")]
    pub fn impersonator(mut self, impersonator: Credentials) -> Self {
        self.impersonator = Some(impersonator);
        self
    }

    #[cfg(feature = "gssapi_ext")]
    pub fn build(self) -> Result<Credentials> {
        let mut minor_status = 0;
        let mut output_cred_handle: gssapi_sys::gss_cred_id_t = ptr::null_mut();
        let actual_mechs = try!(OIDSet::empty());
        let mut time_rec = 0;
        
        let major_status = match self.impersonator {
            None => unsafe {
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
            },
            Some(cred) => unsafe {
                gssapi_sys::gss_acquire_cred_impersonate_name(
                    &mut minor_status,        /* minor_status */
                    cred.get_handle(),       /* impersonator_cred_handle */
                    self.desired_name.get_handle(),     /* desired_name */
                    self.time_req,               /* time_req */
                    self.desired_mechs.get_handle(),          /* desired_mechs */
                    self.cred_usage as gssapi_sys::gss_cred_usage_t,                /* cred_usage */
                    &mut output_cred_handle,       /* output_cred_handle */
                    &mut actual_mechs.get_handle(),      /* actual_mechs */
                    &mut time_rec,         /* time_rec */
                )
            },
        };
        
        if major_status == gssapi_sys::GSS_S_COMPLETE {
            Ok(Credentials {
                cred_handle: output_cred_handle,
                mechs: actual_mechs,
                time_rec: time_rec,
            })
        } else {
            Err(Error::new(major_status, minor_status, OID::empty()))
        }
    }

    #[cfg(not(feature = "gssapi_ext"))]
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
            Err(Error::new(major_status, minor_status, OID::empty()))
        }
    }
}
