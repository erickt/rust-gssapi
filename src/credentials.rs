use gssapi_sys;

pub struct Credentials {
    // FIXME: this shouldn't be public
    pub cred_handle: *mut gssapi_sys::gss_cred_id_t,
}
