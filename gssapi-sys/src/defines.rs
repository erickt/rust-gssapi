use super::OM_uint32;

extern "C" {
    #[link_name = "GSS_C_DELEG_FLAG_SHIM"]
    pub static GSS_C_DELEG_FLAG: OM_uint32;

    #[link_name = "GSS_C_MUTUAL_FLAG_SHIM"]
    pub static GSS_C_MUTUAL_FLAG: OM_uint32;

    #[link_name = "GSS_C_REPLAY_FLAG_SHIM"]
    pub static GSS_C_REPLAY_FLAG: OM_uint32;

    #[link_name = "GSS_C_SEQUENCE_FLAG_SHIM"]
    pub static GSS_C_SEQUENCE_FLAG: OM_uint32;

    #[link_name = "GSS_C_CONF_FLAG_SHIM"]
    pub static GSS_C_CONF_FLAG: OM_uint32;

    #[link_name = "GSS_C_INTEG_FLAG_SHIM"]
    pub static GSS_C_INTEG_FLAG: OM_uint32;

    #[link_name = "GSS_C_ANON_FLAG_SHIM"]
    pub static GSS_C_ANON_FLAG: OM_uint32;

    #[link_name = "GSS_C_PROT_READY_FLAG_SHIM"]
    pub static GSS_C_PROT_READY_FLAG: OM_uint32;

    #[link_name = "GSS_C_TRANS_FLAG_SHIM"]
    pub static GSS_C_TRANS_FLAG: OM_uint32;

    #[link_name = "GSS_C_DELEG_POLICY_FLAG_SHIM"]
    pub static GSS_C_DELEG_POLICY_FLAG: OM_uint32;

    #[link_name = "GSS_C_NO_UI_FLAG_SHIM"]
    pub static GSS_C_NO_UI_FLAG: OM_uint32;

    /// Status code types for gss_display_status.
    #[link_name = "GSS_C_GSS_CODE_SHIM"]
    pub static GSS_C_GSS_CODE: ::std::os::raw::c_int;

    /// Status code types for gss_display_status.
    #[link_name = "GSS_C_MECH_CODE_SHIM"]
    pub static GSS_C_MECH_CODE: ::std::os::raw::c_int;

    #[link_name = "GSS_S_COMPLETE_SHIM"]
    pub static GSS_S_COMPLETE: OM_uint32;

    #[link_name = "GSS_C_SUPPLEMENTARY_OFFSET_SHIM"]
    pub static GSS_C_SUPPLEMENTARY_OFFSET: OM_uint32;

    #[link_name = "GSS_S_CONTINUE_NEEDED_SHIM"]
    pub static GSS_S_CONTINUE_NEEDED: OM_uint32;

    #[link_name = "GSS_S_DUPLICATE_TOKEN_SHIM"]
    pub static GSS_S_DUPLICATE_TOKEN: OM_uint32;

    #[link_name = "GSS_S_OLD_TOKEN_SHIM"]
    pub static GSS_S_OLD_TOKEN: OM_uint32;

    #[link_name = "GSS_S_UNSEQ_TOKEN_SHIM"]
    pub static GSS_S_UNSEQ_TOKEN: OM_uint32;

    #[link_name = "GSS_S_GAP_TOKEN_SHIM"]
    pub static GSS_S_GAP_TOKEN: OM_uint32;
}
