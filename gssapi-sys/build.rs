extern crate pkg_config;

use std::env;

fn main() {
    if let Ok(info) = pkg_config::find_library("krb5-gssapi") {
        if info.include_paths.len() > 0 {
            // avoid empty include paths as they are not supported by GCC
            if info.include_paths.len() > 0 {
                let paths = env::join_paths(info.include_paths).unwrap();
                println!("cargo:include={}", paths.to_str().unwrap());
            }
        }
    }
}
