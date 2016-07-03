/*
extern crate gssapi_sys;
extern crate gssapi;

type ret_flags = *mut u32;

fn client_establish_context(server_fd: isize,
                            service_name: &str,
                            deleg_flag: u32,
                            oid: gssapi_sys::gss_OID) -> gssapi::Result<(gssapi::Context, ret_flags)> {
    Err(gssapi::Error)
}



fn call_server(host: &str,
               port: u16,
               oid: gssapi_sys::gss_OID,
               service_name: &str,
               deleg_flag: u32,
               msg: &str,
               use_file: isize) -> gssapi::Result<()> {
    let server_fd = 0;

    // establish context
    let (context, ret_flags) = try!(client_establish_context(server_fd,
                                                            service_name,
                                                            deleg_flag,
                                                            oid));

    Err(gssapi::Error)
}
*/

fn main() {
}
