#![cfg_attr(feature = "unstable-testing", feature(plugin))]
#![cfg_attr(feature = "unstable-testing", plugin(clippy))]

extern crate k5test;
extern crate gssapi;
extern crate gssapi_sys;

fn create_k5realm() -> k5test::K5Realm {
    let realm = "KRBTEST.COM".to_owned();
    k5test::K5RealmBuilder::new(realm.clone())
        .add_principal(format!("user@{}", realm), None)
        .add_principal(format!("impersonator@{}", realm), None)
        .build()
        .expect("failed to create realm")
}

fn import_name(username: &str, realm: &k5test::K5Realm) -> gssapi::Name {
    let user_principal = format!("{}@{}", username, realm.realm());
    gssapi::Name::new(&user_principal, gssapi::OID::nt_krb5_principal_name()).expect("Failed to import name")
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

#[test]
fn test() {
    let realm = create_k5realm();
    
    // Test name creation & duplication.
    let user_name = import_name("user", &realm);

    let _impersonator_name = import_name("impersonator", &realm);
    duplicate_name(&user_name);
    
    // Test OID set creation.
    create_oid_set();
    
    // TODO: test buffer
    
    // Test errors.
    let _err = illegal_operation();
    
    // Test credentials.
    let _cred = acquire_creds(user_name);
}
