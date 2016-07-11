#![cfg_attr(feature = "unstable-testing", feature(plugin))]
#![cfg_attr(feature = "unstable-testing", plugin(clippy))]

extern crate tempdir;

mod k5realm;
use k5realm::K5Realm;

fn create_k5realm() -> K5Realm {
    let realm = "KRBTEST.COM".to_owned();
    k5realm::K5RealmBuilder::new(realm.clone())
        .add_principal(format!("user@{}", realm), None)
        .build()
        .expect("failed to create realm")
}


#[test]
fn test() {
    let _realm = create_k5realm();
}
