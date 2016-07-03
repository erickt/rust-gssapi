use gssapi_sys;
use std::ptr;

use super::buffer::Buffer;
use super::error::Result;
use super::name::Name;
use super::oid::OID;

pub struct Context {
    ctx_id: gssapi_sys::gss_ctx_id_t,
}

impl Context {
    pub fn new(service_name: &str,
               target_name: Name,
               oid: OID,
               deleg_flag: u32) -> Result<Self> {
        let mut gss_context = ptr::null_mut();

        let mut send_tok = Buffer::from_str(service_name);
        let target_name = try!(Name::new(&mut send_tok, gssapi_sys::GSS_C_NT_HOSTBASED_SERVICE));

        let recv_tok = gssapi_sys::gss_buffer_desc {
            value: ptr::null_mut(),
            length: 0,
        };

        let mut token_ptr = ptr::null_mut();
        let mut ret_flags = 0;

        // Perform the context-establishment loop.
        loop {
            let mut min_stat = 0;
            let maj_stat = unsafe {
                gssapi_sys::gss_init_sec_context(
                    &mut min_stat,
                    ptr::null_mut(), // no credentials
                    &mut gss_context,
                    target_name.get_handle(),
                    oid,
                    gssapi_sys::GSS_C_MUTUAL_FLAG | gssapi_sys::GSS_C_REPLAY_FLAG | deleg_flag,
                    0,
                    ptr::null_mut(), // no channel bindings
                    token_ptr,
                    ptr::null_mut(), // ignore mech type
                    send_tok.get_handle(),
                    &mut ret_flags,
                    ptr::null_mut(), // ignore time_rec
                )
            };

            if gss_context.is_null() {
                panic!("cannot create context");
            }

            if !token_ptr.is_null() {
                gssapi_sys::gss_release_buffer(&mut min_stat, &recv_tok);
            }

            if maj_stat != gssapi_sys::GSS_S_COMPLETE && maj_stat != gssapi_sys::GSS_S_CONTINUE_NEEDED {
                println!("initializing context");
                panic!("?");
            }

            if send_tok.length != 0 {
                println!("send token");
            }

            gssapi_sys::gss_release_buffer(&min_stat, &send_tok);

            if maj_stat == gssapi_sys::GSS_S_CONTINUE_NEEDED {
                if gssapi_sys::recv_token
                token_ptr = &recv_tok;
            }

            break;
        }

        Ok(Context {
            ctx_id: gss_context,
        })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        
    }
}
