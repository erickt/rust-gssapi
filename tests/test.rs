#![cfg_attr(feature = "unstable-testing", feature(plugin))]
#![cfg_attr(feature = "unstable-testing", plugin(clippy))]

extern crate k5test;
extern crate gssapi;
extern crate gssapi_sys;

use std::ffi::CString;

// fn create_k5realm() -> k5test::K5Realm {
//     let realm = "KRBTEST.COM".to_owned();
//     k5test::K5RealmBuilder::new(realm.clone())
//         .add_principal(format!("user@{}", realm), None)
//         .add_principal(format!("impersonator@{}", realm), None)
//         .build()
//         .expect("failed to create realm")
// }

const USER_PRINC : &'static str = "user@KRBTEST.COM";

fn import_name(user_principal: &str) -> gssapi::Name {
    gssapi::Name::new(user_principal, gssapi::OID::nt_krb5_principal_name()).expect("Failed to import name")
}

fn duplicate_name(name: &gssapi::Name) -> gssapi::Name {
    name.clone().duplicate().expect("Failed to duplicate name")
}

fn create_oid_set() -> gssapi::OIDSet {
    gssapi::OIDSet::empty().expect("Failed to create empty OID set.")
}

fn illegal_operation() -> gssapi::Error {
    // TODO: Do a real illegal operation.
    gssapi::Error::new(0, 0, gssapi::OID::empty())
}

fn acquire_creds(name: gssapi::Name) -> gssapi::Credentials {
    gssapi::Credentials::accept(name)
        .desired_mechs(gssapi::OIDSet::c_no_oid_set())
        .build()
        .expect("Failed to acquire credentials")
}

fn store_cred_into(user_principal: &str, cred: gssapi::Credentials) {    
    let ccache_path = format!("/tmp/{}_ccache", user_principal);
    let CCACHE = CString::new(format!("FILE:{}", ccache_path)).unwrap();
    // NOTE: Keytab does not exist.
    // let KEYTAB = CString::new(format!("FILE:/tmp/{}_keytab", user_principal)).unwrap();
    let store : Vec<(CString, CString)> = vec![
        (CString::new("ccache").unwrap(), CCACHE),
        // (CString::new("keytab").unwrap(), KEYTAB),
    ];
    cred.store_into(&store).expect("Failed to store credentials");
    std::path::Path::new(&ccache_path).metadata().expect("Did not create ccache file");
}

#[test]
fn test() {
    // let realm = create_k5realm();
    
    // Test name creation & duplication.
    let user_name = import_name(USER_PRINC);
    duplicate_name(&user_name);
    
    // Test OID set creation.
    create_oid_set();
    
    // TODO: test buffer
    
    // Test errors.
    let _err = illegal_operation();
    
    // Test credentials.
    let cred = acquire_creds(user_name);
    
    // Test storing
    store_cred_into(USER_PRINC, cred);
    
}
