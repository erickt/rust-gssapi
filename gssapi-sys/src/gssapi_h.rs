// from gssapi_h.h

pub enum gss_name_struct { }
pub type gss_name_t = *mut gss_name_struct;
pub enum gss_cred_id_struct { }
pub type gss_cred_id_t = *mut gss_cred_id_struct;
pub enum gss_ctx_id_struct { }
pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
pub type gss_uint32 = u32;
pub type gss_int32 = i32;
pub type OM_uint32 = gss_uint32;
#[repr(C)]
#[derive(Debug)]
pub struct gss_OID_desc_struct {
    pub length: OM_uint32,
    pub elements: *mut ::std::os::raw::c_void,
}
impl ::std::default::Default for gss_OID_desc_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type gss_OID_desc = gss_OID_desc_struct;
pub type gss_OID = *const gss_OID_desc_struct;
#[repr(C)]
#[derive(Debug)]
pub struct gss_OID_set_desc_struct {
    pub count: usize,
    pub elements: gss_OID,
}
impl ::std::default::Default for gss_OID_set_desc_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type gss_OID_set_desc = gss_OID_set_desc_struct;
pub type gss_OID_set = *const gss_OID_set_desc_struct;
#[repr(C)]
#[derive(Debug)]
pub struct gss_buffer_desc_struct {
    pub length: usize,
    pub value: *mut ::std::os::raw::c_void,
}
impl ::std::default::Default for gss_buffer_desc_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type gss_buffer_desc = gss_buffer_desc_struct;
pub type gss_buffer_t = *mut gss_buffer_desc_struct;
#[repr(C)]
#[derive(Debug)]
pub struct gss_channel_bindings_struct {
    pub initiator_addrtype: OM_uint32,
    pub initiator_address: gss_buffer_desc,
    pub acceptor_addrtype: OM_uint32,
    pub acceptor_address: gss_buffer_desc,
    pub application_data: gss_buffer_desc,
}
impl ::std::default::Default for gss_channel_bindings_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
pub type gss_qop_t = OM_uint32;

pub type gss_cred_usage_t = ::std::os::raw::c_int;
pub type gss_const_buffer_t = *const gss_buffer_desc;
pub type gss_const_channel_bindings_t = *const gss_channel_bindings_struct;
pub type gss_const_ctx_id_t = *const gss_ctx_id_struct;
pub type gss_const_cred_id_t = *const gss_cred_id_struct;
pub type gss_const_name_t = *const gss_name_struct;
pub type gss_const_OID = *const gss_OID_desc;
pub type gss_const_OID_set = *const gss_OID_set_desc;
extern "C" {
    pub static GSS_C_NT_USER_NAME: gss_OID;
    pub static GSS_C_NT_MACHINE_UID_NAME: gss_OID;
    pub static GSS_C_NT_STRING_UID_NAME: gss_OID;
    pub static GSS_C_NT_HOSTBASED_SERVICE_X: gss_OID;
    pub static GSS_C_NT_HOSTBASED_SERVICE: gss_OID;
    pub static GSS_C_NT_ANONYMOUS: gss_OID;
    pub static GSS_C_NT_EXPORT_NAME: gss_OID;
    pub static GSS_C_MA_MECH_CONCRETE: gss_const_OID;
    pub static GSS_C_MA_MECH_PSEUDO: gss_const_OID;
    pub static GSS_C_MA_MECH_COMPOSITE: gss_const_OID;
    pub static GSS_C_MA_MECH_NEGO: gss_const_OID;
    pub static GSS_C_MA_MECH_GLUE: gss_const_OID;
    pub static GSS_C_MA_NOT_MECH: gss_const_OID;
    pub static GSS_C_MA_DEPRECATED: gss_const_OID;
    pub static GSS_C_MA_NOT_DFLT_MECH: gss_const_OID;
    pub static GSS_C_MA_ITOK_FRAMED: gss_const_OID;
    pub static GSS_C_MA_AUTH_INIT: gss_const_OID;
    pub static GSS_C_MA_AUTH_TARG: gss_const_OID;
    pub static GSS_C_MA_AUTH_INIT_INIT: gss_const_OID;
    pub static GSS_C_MA_AUTH_TARG_INIT: gss_const_OID;
    pub static GSS_C_MA_AUTH_INIT_ANON: gss_const_OID;
    pub static GSS_C_MA_AUTH_TARG_ANON: gss_const_OID;
    pub static GSS_C_MA_DELEG_CRED: gss_const_OID;
    pub static GSS_C_MA_INTEG_PROT: gss_const_OID;
    pub static GSS_C_MA_CONF_PROT: gss_const_OID;
    pub static GSS_C_MA_MIC: gss_const_OID;
    pub static GSS_C_MA_WRAP: gss_const_OID;
    pub static GSS_C_MA_PROT_READY: gss_const_OID;
    pub static GSS_C_MA_REPLAY_DET: gss_const_OID;
    pub static GSS_C_MA_OOS_DET: gss_const_OID;
    pub static GSS_C_MA_CBINDINGS: gss_const_OID;
    pub static GSS_C_MA_PFS: gss_const_OID;
    pub static GSS_C_MA_COMPRESS: gss_const_OID;
    pub static GSS_C_MA_CTX_TRANS: gss_const_OID;
}
extern "C" {
    pub fn gss_acquire_cred(minor_status: *mut OM_uint32,
                            desired_name: gss_name_t,
                            time_req: OM_uint32,
                            desired_mechs: gss_OID_set,
                            cred_usage: gss_cred_usage_t,
                            output_cred_handle: *mut gss_cred_id_t,
                            actual_mechs: *mut gss_OID_set,
                            time_rec: *mut OM_uint32)
     -> OM_uint32;
    pub fn gss_release_cred(minor_status: *mut OM_uint32,
                            cred_handle: *mut gss_cred_id_t)
     -> OM_uint32;
    pub fn gss_init_sec_context(minor_status: *mut OM_uint32,
                                claimant_cred_handle: gss_cred_id_t,
                                context_handle: *mut gss_ctx_id_t,
                                target_name: gss_name_t,
                                mech_type: gss_OID,
                                req_flags: OM_uint32,
                                time_req: OM_uint32,
                                input_chan_bindings: gss_channel_bindings_t,
                                input_token: gss_buffer_t,
                                actual_mech_type: *mut gss_OID,
                                output_token: gss_buffer_t,
                                ret_flags: *mut OM_uint32,
                                time_rec: *mut OM_uint32) -> OM_uint32;
    pub fn gss_accept_sec_context(minor_status: *mut OM_uint32,
                                  context_handle: *mut gss_ctx_id_t,
                                  acceptor_cred_handle: gss_cred_id_t,
                                  input_token_buffer: gss_buffer_t,
                                  input_chan_bindings: gss_channel_bindings_t,
                                  src_name: *mut gss_name_t,
                                  mech_type: *mut gss_OID,
                                  output_token: gss_buffer_t,
                                  ret_flags: *mut OM_uint32,
                                  time_rec: *mut OM_uint32,
                                  delegated_cred_handle: *mut gss_cred_id_t) -> OM_uint32;
    pub fn gss_process_context_token(minor_status: *mut OM_uint32,
                                     context_handle: gss_ctx_id_t,
                                     output_token: gss_buffer_t) -> OM_uint32;
    pub fn gss_delete_sec_context(minor_status: *mut OM_uint32,
                                  context_handle: *mut gss_ctx_id_t,
                                  time_rec: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_context_time(minor_status: *mut OM_uint32,
                            context_handle: gss_ctx_id_t,
                            time_rec: *mut OM_uint32) -> OM_uint32;
    pub fn gss_get_mic(minor_status: *mut OM_uint32,
                       context_handle: gss_ctx_id_t,
                       qop_req: gss_qop_t,
                       message_buffer: gss_buffer_t,
                       message_token: gss_buffer_t) -> OM_uint32;
    pub fn gss_verify_mic(minor_status: *mut OM_uint32,
                          context_handle: gss_ctx_id_t,
                          message_buffer: gss_buffer_t,
                          message_token: gss_buffer_t,
                          qop_state: *mut gss_qop_t) -> OM_uint32;
    pub fn gss_wrap(minor_status: *mut OM_uint32,
                    context_handle: gss_ctx_id_t,
                    conf_req_flag: ::std::os::raw::c_int,
                    qop_req: gss_qop_t,
                    input_message_buffer: gss_buffer_t,
                    conf_state: *mut ::std::os::raw::c_int,
                    output_message_buffer: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_unwrap(minor_status: *mut OM_uint32,
                      context_handle: gss_ctx_id_t,
                      input_message_buffer: gss_buffer_t,
                      output_message_buffer: gss_buffer_t,
                      conf_state: *mut ::std::os::raw::c_int,
                      qop_state: *mut gss_qop_t)
     -> OM_uint32;
    pub fn gss_display_status(minor_status: *mut OM_uint32,
                              status_value: OM_uint32,
                              status_type: ::std::os::raw::c_int,
                              mech_type: gss_OID,
                              message_context: *mut OM_uint32,
                              status_string: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_indicate_mechs(minor_status: *mut OM_uint32,
                              mech_set: *mut gss_OID_set)
     -> OM_uint32;
    pub fn gss_compare_name(minor_status: *mut OM_uint32,
                            name1: gss_name_t,
                            name2: gss_name_t,
                            name_equal: *mut ::std::os::raw::c_int)
     -> OM_uint32;
    pub fn gss_display_name(minor_status: *mut OM_uint32,
                            input_name: gss_name_t,
                            output_name_buffer: gss_buffer_t,
                            output_name_type: *mut gss_OID)
     -> OM_uint32;
    pub fn gss_import_name(minor_status: *mut OM_uint32,
                           input_name_buffer: gss_buffer_t,
                           input_name_type: gss_OID,
                           output_name: *mut gss_name_t) -> OM_uint32;
    pub fn gss_release_name(minor_status: *mut OM_uint32,
                            input_name: *mut gss_name_t)
     -> OM_uint32;
    pub fn gss_release_buffer(minor_status: *mut OM_uint32,
                              buffer: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_release_oid_set(minor_status: *mut OM_uint32,
                               set: *mut gss_OID_set)
     -> OM_uint32;
    pub fn gss_inquire_cred(minor_status: *mut OM_uint32,
                            cred_handle: gss_cred_id_t,
                            name: *mut gss_name_t,
                            lifetime: *mut OM_uint32,
                            cred_usage: *mut gss_cred_usage_t,
                            mechanisms: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_inquire_context(minor_status: *mut OM_uint32,
                               context_handle: gss_ctx_id_t,
                               src_name: *mut gss_name_t,
                               targ_name: *mut gss_name_t,
                               lifetime_rec: *mut OM_uint32,
                               mech_type: *mut gss_OID,
                               ctx_flags: *mut OM_uint32,
                               locally_initiated: *mut ::std::os::raw::c_int,
                               open: *mut ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_wrap_size_limit(minor_status: *mut OM_uint32,
                               context_handle: gss_ctx_id_t,
                               conf_req_flag: ::std::os::raw::c_int,
                               qop_req: gss_qop_t,
                               req_output_size: OM_uint32,
                               max_input_size: *mut OM_uint32)
     -> OM_uint32;
    pub fn gss_import_name_object(minor_status: *mut OM_uint32,
                                  input_name: *mut ::std::os::raw::c_void,
                                  input_name_type: gss_OID,
                                  output_name: *mut gss_name_t) -> OM_uint32;
    pub fn gss_export_name_object(minor_status: *mut OM_uint32,
                                  input_name: gss_name_t,
                                  desired_name_type: gss_OID,
                                  output_name: *mut *mut ::std::os::raw::c_void)
     -> OM_uint32;
    pub fn gss_add_cred(minor_status: *mut OM_uint32,
                        input_cred_handle: gss_cred_id_t,
                        desired_name: gss_name_t,
                        desired_mech: gss_OID,
                        cred_usage: gss_cred_usage_t,
                        initiator_time_req: OM_uint32,
                        acceptor_time_req: OM_uint32,
                        output_cred_handle: *mut gss_cred_id_t,
                        actual_mechs: *mut gss_OID_set,
                        initiator_time_rec: *mut OM_uint32,
                        acceptor_time_rec: *mut OM_uint32) -> OM_uint32;
    pub fn gss_inquire_cred_by_mech(minor_status: *mut OM_uint32,
                                    cred_handle: gss_cred_id_t,
                                    mech_type: gss_OID,
                                    name: *mut gss_name_t,
                                    initiator_lifetime: *mut OM_uint32,
                                    acceptor_lifetime: *mut OM_uint32,
                                    cred_usage: *mut gss_cred_usage_t) -> OM_uint32;
    pub fn gss_export_sec_context(minor_status: *mut OM_uint32,
                                  context_handle: *mut gss_ctx_id_t,
                                  interprocess_token: gss_buffer_t)
     -> OM_uint32;
    pub fn gss_import_sec_context(minor_status: *mut OM_uint32,
                                  interprocess_token: gss_buffer_t,
                                  context_handle: *mut gss_ctx_id_t) -> OM_uint32;
    pub fn gss_release_oid(minor_status: *mut OM_uint32,
                           oid: *mut gss_OID)
     -> OM_uint32;
    pub fn gss_create_empty_oid_set(minor_status: *mut OM_uint32,
                                    oid_set: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_add_oid_set_member(minor_status: *mut OM_uint32,
                                  member_oid: gss_OID,
                                  oid_set: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_test_oid_set_member(minor_status: *mut OM_uint32,
                                   member: gss_OID,
                                   set: gss_OID_set,
                                   present: *mut ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_str_to_oid(minor_status: *mut OM_uint32,
                          oid_str: gss_buffer_t,
                          oid: *mut gss_OID) -> OM_uint32;
    pub fn gss_oid_to_str(minor_status: *mut OM_uint32,
                          oid: gss_OID,
                          oid_str: gss_buffer_t) -> OM_uint32;
    pub fn gss_inquire_names_for_mech(minor_status: *mut OM_uint32,
                                      mechanism: gss_OID,
                                      name_types: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_inquire_mechs_for_name(minor_status: *mut OM_uint32,
                                      input_name: gss_name_t,
                                      mech_types: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_sign(minor_status: *mut OM_uint32,
                    context_handle: gss_ctx_id_t,
                    qop_req: ::std::os::raw::c_int,
                    message_buffer: gss_buffer_t,
                    message_token: gss_buffer_t) -> OM_uint32;
    pub fn gss_verify(minor_status: *mut OM_uint32,
                      context_handle: gss_ctx_id_t,
                      message_buffer: gss_buffer_t,
                      token_buffer: gss_buffer_t,
                      qop_state: *mut ::std::os::raw::c_int) -> OM_uint32;
    pub fn gss_seal(minor_status: *mut OM_uint32,
                    context_handle: gss_ctx_id_t,
                    conf_req_flag: ::std::os::raw::c_int,
                    qop_req: ::std::os::raw::c_int,
                    input_message_buffer: gss_buffer_t,
                    conf_state: *mut ::std::os::raw::c_int,
                    output_message_buffer: gss_buffer_t) -> OM_uint32;
    pub fn gss_unseal(minor_status: *mut OM_uint32,
                      context_handle: gss_ctx_id_t,
                      input_message_buffer: gss_buffer_t,
                      output_message_buffer: gss_buffer_t,
                      conf_state: *mut ::std::os::raw::c_int,
                      qop_state: *mut ::std::os::raw::c_int)
     -> OM_uint32;
    pub fn gss_export_name(minor_status: *mut OM_uint32,
                           arg2: gss_name_t,
                           arg3: gss_buffer_t) -> OM_uint32;
    pub fn gss_duplicate_name(minor_status: *mut OM_uint32,
                              input_name: gss_name_t,
                              dest_name: *mut gss_name_t) -> OM_uint32;
    pub fn gss_canonicalize_name(minor_status: *mut OM_uint32,
                                 input_name: gss_name_t,
                                 mech_type: gss_OID,
                                 output_name: *mut gss_name_t)
     -> OM_uint32;
}
