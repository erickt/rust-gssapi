use error::{Error, Result};
use gssapi_sys;
use name::Name;
use oid::OID;
use oid_set::OIDSet;
use std::ptr;

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
    
    pub fn impersonate<T: Into<Name>>(self, desired_name: T) -> CredentialsBuilder {
        CredentialsBuilder::new(desired_name).impersonator(self)
    }
    
    // TODO: wrap gssapi key-value set type
    // TODO: 'write' method that stores cred to file
    // gss_store_cred_into(
    // OM_uint32 *,               /* minor_status */
    // gss_cred_id_t,             /* input_cred_handle */
    // gss_cred_usage_t,          /* input_usage */
    // gss_OID,                   /* desired_mech */
    // OM_uint32,                 /* overwrite_cred */
    // OM_uint32,                 /* default_cred */
    // gss_const_key_value_set_t, /* cred_store */
    // gss_OID_set *,             /* elements_stored */
    // gss_cred_usage_t *);       /* cred_usage_stored */

    // pub fn store()
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

pub struct CredentialsBuilder {
    desired_name: Name,
    time_req: u32,
    desired_mechs: OIDSet,
    cred_usage: isize,
    impersonator: Option<Credentials>
}

impl CredentialsBuilder {
    pub fn new<T: Into<Name>>(desired_name: T) -> Self {
        CredentialsBuilder {
            desired_name: desired_name.into(),
            time_req: 0,
            desired_mechs: OIDSet::empty().unwrap(),
            cred_usage: 0,
            impersonator: None
        }
    }

    pub fn time_req(mut self, time_req: u32) -> Self {
        self.time_req = time_req;
        self
    }
    
    pub fn impersonator(mut self, impersonator: Credentials) -> Self {
        self.impersonator = Some(impersonator);
        self
    }

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
}
