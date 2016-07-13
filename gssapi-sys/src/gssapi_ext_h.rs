// from gssapi_ext.h

use gssapi_h::*;

#[repr(C)]
#[derive(Debug)]
pub struct gss_buffer_set_desc_struct {
    pub count: usize,
    pub elements: *mut gss_buffer_desc,
}
impl ::std::default::Default for gss_buffer_set_desc_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type gss_buffer_set_desc = gss_buffer_set_desc_struct;
pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
#[repr(C)]
#[derive(Debug)]
pub struct gss_iov_buffer_desc_struct {
    pub type_: OM_uint32,
    pub buffer: gss_buffer_desc,
}
impl ::std::default::Default for gss_iov_buffer_desc_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type gss_iov_buffer_desc = gss_iov_buffer_desc_struct;
pub type gss_iov_buffer_t = *mut gss_iov_buffer_desc_struct;
pub enum gss_any { }
pub type gss_any_t = *mut gss_any;
#[repr(C)]
#[derive(Debug)]
pub struct gss_key_value_element_struct {
    pub key: *const ::std::os::raw::c_char,
    pub value: *const ::std::os::raw::c_char,
}
impl ::std::default::Default for gss_key_value_element_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type gss_key_value_element_desc = gss_key_value_element_struct;
#[repr(C)]
#[derive(Debug)]
pub struct gss_key_value_set_struct {
    pub count: OM_uint32,
    pub elements: *mut gss_key_value_element_desc,
}
impl ::std::default::Default for gss_key_value_set_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type gss_key_value_set_desc = gss_key_value_set_struct;
pub type gss_const_key_value_set_t = *const gss_key_value_set_desc;
extern "C" {
    pub static mut GSS_C_INQ_SSPI_SESSION_KEY: gss_OID;
    pub static mut GSS_C_ATTR_LOCAL_LOGIN_USER: gss_buffer_t;
    pub static mut GSS_C_NT_COMPOSITE_EXPORT: gss_OID;
}
extern "C" {
    // Macroed out on non-Windows systems.
    // pub fn gss_pname_to_uid(minor: *mut OM_uint32, name: gss_name_t,
    //                         mech_type: gss_OID, uidOut: *mut uid_t)
    //  -> OM_uint32;
    pub fn gss_localname(minor: *mut OM_uint32, name: gss_name_t,
                         mech_type: gss_const_OID, localname: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_userok(name: gss_name_t,
                      username: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn gss_authorize_localname(minor: *mut OM_uint32, name: gss_name_t,
                                   user: gss_name_t) -> OM_uint32;
    pub fn gss_acquire_cred_with_password(arg1: *mut OM_uint32,
                                          arg2: gss_name_t,
                                          arg3: gss_buffer_t, arg4: OM_uint32,
                                          arg5: gss_OID_set,
                                          arg6: gss_cred_usage_t,
                                          arg7: *mut gss_cred_id_t,
                                          arg8: *mut gss_OID_set,
                                          arg9: *mut OM_uint32) -> OM_uint32;
    pub fn gss_add_cred_with_password(arg1: *mut OM_uint32,
                                      arg2: gss_cred_id_t, arg3: gss_name_t,
                                      arg4: gss_OID, arg5: gss_buffer_t,
                                      arg6: gss_cred_usage_t, arg7: OM_uint32,
                                      arg8: OM_uint32,
                                      arg9: *mut gss_cred_id_t,
                                      arg10: *mut gss_OID_set,
                                      arg11: *mut OM_uint32,
                                      arg12: *mut OM_uint32) -> OM_uint32;
    pub fn gss_create_empty_buffer_set(arg1: *mut OM_uint32,
                                       arg2: *mut gss_buffer_set_t)
     -> OM_uint32;
    pub fn gss_add_buffer_set_member(arg1: *mut OM_uint32, arg2: gss_buffer_t,
                                     arg3: *mut gss_buffer_set_t)
     -> OM_uint32;
    pub fn gss_release_buffer_set(arg1: *mut OM_uint32,
                                  arg2: *mut gss_buffer_set_t) -> OM_uint32;
    pub fn gss_inquire_sec_context_by_oid(arg1: *mut OM_uint32,
                                          arg2: gss_ctx_id_t, arg3: gss_OID,
                                          arg4: *mut gss_buffer_set_t)
     -> OM_uint32;
    pub fn gss_inquire_cred_by_oid(arg1: *mut OM_uint32, arg2: gss_cred_id_t,
                                   arg3: gss_OID, arg4: *mut gss_buffer_set_t)
     -> OM_uint32;
    pub fn gss_set_sec_context_option(arg1: *mut OM_uint32,
                                      arg2: *mut gss_ctx_id_t, arg3: gss_OID,
                                      arg4: gss_buffer_t) -> OM_uint32;
    pub fn gss_set_cred_option(arg1: *mut OM_uint32, arg2: *mut gss_cred_id_t,
                               arg3: gss_OID, arg4: gss_buffer_t)
     -> OM_uint32;
    pub fn gssspi_mech_invoke(arg1: *mut OM_uint32, arg2: gss_OID,
                              arg3: gss_OID, arg4: gss_buffer_t) -> OM_uint32;
    pub fn gss_wrap_aead(arg1: *mut OM_uint32, arg2: gss_ctx_id_t,
                         arg3: ::std::os::raw::c_int, arg4: gss_qop_t,
                         arg5: gss_buffer_t, arg6: gss_buffer_t,
                         arg7: *mut ::std::os::raw::c_int, arg8: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_unwrap_aead(arg1: *mut OM_uint32, arg2: gss_ctx_id_t,
                           arg3: gss_buffer_t, arg4: gss_buffer_t,
                           arg5: gss_buffer_t,
                           arg6: *mut ::std::os::raw::c_int,
                           arg7: *mut gss_qop_t) -> OM_uint32;
    pub fn gss_complete_auth_token(minor_status: *mut OM_uint32,
                                   context_handle: gss_ctx_id_t,
                                   input_message_buffer: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_wrap_iov(arg1: *mut OM_uint32, arg2: gss_ctx_id_t,
                        arg3: ::std::os::raw::c_int, arg4: gss_qop_t,
                        arg5: *mut ::std::os::raw::c_int,
                        arg6: *mut gss_iov_buffer_desc,
                        arg7: ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_unwrap_iov(arg1: *mut OM_uint32, arg2: gss_ctx_id_t,
                          arg3: *mut ::std::os::raw::c_int,
                          arg4: *mut gss_qop_t,
                          arg5: *mut gss_iov_buffer_desc,
                          arg6: ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_wrap_iov_length(arg1: *mut OM_uint32, arg2: gss_ctx_id_t,
                               arg3: ::std::os::raw::c_int, arg4: gss_qop_t,
                               arg5: *mut ::std::os::raw::c_int,
                               arg6: *mut gss_iov_buffer_desc,
                               arg7: ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_get_mic_iov(arg1: *mut OM_uint32, arg2: gss_ctx_id_t,
                           arg3: gss_qop_t, arg4: *mut gss_iov_buffer_desc,
                           arg5: ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_get_mic_iov_length(arg1: *mut OM_uint32, arg2: gss_ctx_id_t,
                                  arg3: gss_qop_t,
                                  arg4: *mut gss_iov_buffer_desc,
                                  arg5: ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_verify_mic_iov(arg1: *mut OM_uint32, arg2: gss_ctx_id_t,
                              arg3: *mut gss_qop_t,
                              arg4: *mut gss_iov_buffer_desc,
                              arg5: ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_release_iov_buffer(arg1: *mut OM_uint32,
                                  arg2: *mut gss_iov_buffer_desc,
                                  arg3: ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_acquire_cred_impersonate_name(arg1: *mut OM_uint32,
                                             arg2: gss_cred_id_t,
                                             arg3: gss_name_t,
                                             arg4: OM_uint32,
                                             arg5: gss_OID_set,
                                             arg6: gss_cred_usage_t,
                                             arg7: *mut gss_cred_id_t,
                                             arg8: *mut gss_OID_set,
                                             arg9: *mut OM_uint32)
     -> OM_uint32;
    pub fn gss_add_cred_impersonate_name(arg1: *mut OM_uint32,
                                         arg2: gss_cred_id_t,
                                         arg3: gss_cred_id_t,
                                         arg4: gss_name_t, arg5: gss_OID,
                                         arg6: gss_cred_usage_t,
                                         arg7: OM_uint32, arg8: OM_uint32,
                                         arg9: *mut gss_cred_id_t,
                                         arg10: *mut gss_OID_set,
                                         arg11: *mut OM_uint32,
                                         arg12: *mut OM_uint32) -> OM_uint32;

    // Naming extensions

    pub fn gss_display_name_ext(minor_status: *mut OM_uint32,
                                name: gss_name_t,
                                display_as_name_type: gss_OID,
                                display_name: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_inquire_name(minor_status: *mut OM_uint32,
                            name: gss_name_t,
                            name_is_MN: *mut ::std::os::raw::c_int,
                            MN_mech: *mut gss_OID,
                            attrs: *mut gss_buffer_set_t)
     -> OM_uint32;
    pub fn gss_get_name_attribute(minor_status: *mut OM_uint32,
                                  name: gss_name_t,
                                  attr: gss_buffer_t,
                                  authenticated: *mut ::std::os::raw::c_int,
                                  complete: *mut ::std::os::raw::c_int,
                                  value: gss_buffer_t,
                                  display_value: gss_buffer_t,
                                  more: *mut ::std::os::raw::c_int)
     -> OM_uint32;
    pub fn gss_set_name_attribute(minor_status: *mut OM_uint32,
                                  name: gss_name_t,
                                  complete: ::std::os::raw::c_int,
                                  attr: gss_buffer_t,
                                  value: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_delete_name_attribute(minor_status: *mut OM_uint32,
                                     name: gss_name_t,
                                     attr: gss_buffer_t) -> OM_uint32;
    pub fn gss_export_name_composite(minor_status: *mut OM_uint32,
                                     name: gss_name_t,
                                     exp_composite_name: gss_buffer_t) -> OM_uint32;
    pub fn gss_map_name_to_any(minor_status: *mut OM_uint32,
                               name: gss_name_t,
                               authenticated: ::std::os::raw::c_int,
                               type_id: gss_buffer_t,
                               output: *mut gss_any_t)
     -> OM_uint32;
    pub fn gss_release_any_name_mapping(minor_status: *mut OM_uint32,
                                        name: gss_name_t,
                                        type_id: gss_buffer_t,
                                        input: *mut gss_any_t) -> OM_uint32;


    // draft-josefsson-gss-capsulate

    pub fn gss_encapsulate_token(input_token: gss_const_buffer_t,
                                 token_oid: gss_const_OID,
                                 output_token: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_decapsulate_token(input_token: gss_const_buffer_t,
                                 token_oid: gss_const_OID,
                                 output_token: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_oid_equal(first_oid: gss_const_OID, second_oid: gss_const_OID)
     -> ::std::os::raw::c_int;
    pub fn gss_acquire_cred_from(minor_status: *mut OM_uint32,
                                 desired_name: gss_name_t,
                                 time_req: OM_uint32,
                                 desired_mechs: gss_OID_set,
                                 cred_usage: gss_cred_usage_t,
                                 cred_store: gss_const_key_value_set_t,
                                 output_cred_handle: *mut gss_cred_id_t,
                                 actual_mechs: *mut gss_OID_set,
                                 time_rec: *mut OM_uint32)
     -> OM_uint32;

    // Credential store extensions

    pub fn gss_add_cred_from(minor_status: *mut OM_uint32,
                             input_cred_handle: gss_cred_id_t,
                             input_usage: gss_cred_usage_t,
                             desired_mech: gss_OID,
                             overwrite_cred: OM_uint32,
                             default_cred: OM_uint32,
                             cred_store: gss_const_key_value_set_t,
                             elements_stored: *mut gss_OID_set,
                             cred_usage_stored: *mut gss_cred_usage_t) -> OM_uint32;
    pub fn gss_store_cred_into(minor_status: *mut OM_uint32,
                               input_cred_handle: gss_cred_id_t,
                               input_usage: gss_cred_usage_t,
                               desired_mech: gss_OID,
                               overwrite_cred: OM_uint32,
                               default_cred: OM_uint32,
                               cred_store: gss_const_key_value_set_t,
                               elements_stored: *mut gss_OID_set,
                               cred_usage_stored: *mut gss_cred_usage_t) -> OM_uint32;
    pub fn gss_export_cred(minor_status: *mut OM_uint32,
                           cred_handle: gss_cred_id_t,
                           token: gss_buffer_t) -> OM_uint32;
    pub fn gss_import_cred(minor_status: *mut OM_uint32,
                           token: gss_buffer_t,
                           cred_handle: *mut gss_cred_id_t) -> OM_uint32;
}
