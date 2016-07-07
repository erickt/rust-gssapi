extern crate gssapi_sys;
extern crate gssapi;
extern crate resolve;

use gssapi::{Context, ContextInitializer, Name};

fn main() {
    let fqdn = resolve::hostname::get_hostname().unwrap();

    let server_hostbased_name = Name::new(
        format!("http@{}", fqdn),
        gssapi::oid::GSS_C_NT_HOSTBASED_SERVICE).unwrap();

    let client_ctx_builder = Context::client_builder(
        server_hostbased_name);

    let mut initializer = client_ctx_builder.step();
    let context;

    loop {
        match initializer {
            Ok(ContextInitializer::Continue { initializer: initializer_, output }) => {
                let _ = output;
                let input = gssapi::Buffer::new();
                initializer = initializer_.step(input)
            }
            Ok(ContextInitializer::Done { context: context_ }) => {
                context = context_;
                break;
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }

    println!("client: {:?}", context);
}
