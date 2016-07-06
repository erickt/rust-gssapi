extern crate gssapi_sys;
extern crate gssapi;
extern crate resolve;

use gssapi::{Context, ContextInitializer, Name};

/*
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
    let fqdn = resolve::hostname::get_hostname().unwrap();

    let server_hostbased_name = Name::new(
        format!("http@{}", fqdn),
        gssapi::oid::GSS_C_NT_HOSTBASED_SERVICE).unwrap();

    let client_ctx_builder = Context::client_builder(
        server_hostbased_name);

    let mut initializer = client_ctx_builder.step().unwrap();
    let context;

    loop {
        initializer = match initializer {
            ContextInitializer::Continue { initializer, output } => {
                let _ = output;
                let input = gssapi::Buffer::new();
                initializer.step(input).unwrap()
            }
            ContextInitializer::Done { context: context_ }  => {
                context = context_;
                break;
            }
        };
    }

    println!("client: {:?}", context);
}
