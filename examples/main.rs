extern crate gssapi_sys;
extern crate gssapi;
extern crate resolve;

use gssapi::{
    AcceptContext,
    Context,
    Credentials,
    InitiateContext,
    Name,
    OID,
};

fn main() {
    let fqdn = resolve::hostname::get_hostname().expect("failed to resolve hostname");

    let server_name = Name::new(
        &format!("http@{}", fqdn),
        OID::nt_hostbased_service(),
    ).expect("failed to create server name");

    let server_credentials = Credentials::accept(server_name.clone())
        .build().expect("failed to create credentials");

    let client_ctx_builder = Context::initiate(server_name);
    let server_ctx_builder = Context::accept(server_credentials);

    let (mut initializer, client_token) = client_ctx_builder.step().expect("client initial step failed");

    let client_context;
    let server_context;

    match server_ctx_builder.step(client_token).expect("server initial step failed") {
        AcceptContext::Done { context: server_context_, token } => {
            server_context = server_context_;
            client_context = initializer.final_step(token).expect("initializer final step failed");
        }
        AcceptContext::Continue { mut acceptor, token: mut server_token } => {
            loop {
                match initializer.step(server_token).expect("initializer step failed") {
                    InitiateContext::Done { .. } => {
                        panic!("client done but server is not?");
                    }
                    InitiateContext::Continue { initializer: initializer_, token: client_token } => {
                        initializer = initializer_;

                        match acceptor.step(client_token).expect("acceptor step failed") {
                            AcceptContext::Done { context: server_context_, token } => {
                                server_context = server_context_;
                                client_context = initializer.final_step(token).expect("initializer final step failed");
                                break;
                            }
                            AcceptContext::Continue { acceptor: acceptor_, token: server_token_ } => {
                                acceptor = acceptor_;
                                server_token = server_token_;
                            }
                        }
                    }
                }
            }
        }
    };

    println!("client: {:?}", client_context);
    println!("server: {:?}", server_context);
}
