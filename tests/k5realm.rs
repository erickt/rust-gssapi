// Copyright (C) 2014 by Solly Ross
// Copyright (C) 2010 by the Massachusetts Institute of Technology.
// All rights reserved.
//
// Export of this software from the United States of America may
//   require a specific license from the United States Government.
//   It is the responsibility of any person or organization contemplating
//   export to obtain such a license before exporting.
//
// WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
// distribute this software and its documentation for any purpose and
// without fee is hereby granted, provided that the above copyright
// notice appear in all copies and that both that copyright notice and
// this permission notice appear in supporting documentation, and that
// the name of M.I.T. not be used in advertising or publicity pertaining
// to distribution of the software without specific, written prior
// permission.  Furthermore if you modify this software you must label
// your software as modified software and not distribute it in such a
// fashion that it might be confused with the original M.I.T. software.
// M.I.T. makes no representations about the suitability of
// this software for any purpose.  It is provided "as is" without express
// or implied warranty.

use std::ffi::OsString;
use std::io;
use std::process::Command;
use tempdir;

pub struct K5RealmBuilder {
    realm: String,
    port_base: u16,
    principals: Vec<(String, Option<String>)>,
    kdb5_util: String,
    kadmin_local: String,
}

impl K5RealmBuilder {
    pub fn new(realm: String) -> Self {
        K5RealmBuilder {
            realm: realm,
            port_base: 61000,
            principals: vec![],
            kdb5_util: "kdb5_util".to_string(),
            kadmin_local: "kadmin_local".to_string(),
        }
    }

    pub fn add_principal(mut self, principal: String, password: Option<String>) -> Self {
        self.principals.push((principal, password));
        self
    }

    pub fn build(self) -> io::Result<K5Realm> {
        let dir = tempdir::TempDir::new("").unwrap();

        let env = vec![
            ("KRB5_CONFIG", dir.path().join("krb5.conf").into()),
            ("KRB5_KDC_PROFILE", dir.path().join("kdc.conf").into()),
            ("KRB5CCNAME", dir.path().join("ccache").into()),
            ("KRB5_KTNAME", dir.path().join("keytab").into()),
            ("KRB5_CLIENT_KTNAME", dir.path().join("client_keytab").into()),
            ("KRB5RCACHEDIR", dir.path().into()),
            ("KPROPD_PORT", (self.port_base + 3).to_string().into()),
            ("KPROP_PORT", (self.port_base + 3).to_string().into()),
        ];

        let realm = K5Realm {
            _dir: dir,
            _realm: self.realm,
            kadmin_local: self.kadmin_local,
            kdb5_util: self.kdb5_util,
            env: env,
        };
        println!("here1");

        try!(realm.create_kdb());

        println!("here2");

        for (principal, password) in self.principals {
            try!(realm.add_principal(principal, password));
        }

        println!("here3");

        Ok(realm)
    }
}


pub struct K5Realm {
    _dir: tempdir::TempDir,
    _realm: String,
    kadmin_local: String,
    kdb5_util: String,
    env: Vec<(&'static str, OsString)>,
}

impl K5Realm {
    pub fn add_principal(&self, principal: String, password: Option<String>) -> io::Result<()> {
        match password {
            Some(ref password) => {
                self.run_kadminl(&format!("addprinc -pw {} {}", password, principal))
            }
            None =>{
                self.run_kadminl(&format!("addprinc -randkey {}", principal))
            }
        }
    }

    fn run_command<F>(&self, command: &str, f: F) -> io::Result<()>
        where F: FnOnce(&mut Command) -> &mut Command
    {
        let mut command = Command::new(command);

        for &(ref key, ref value) in &self.env {
            command.env(key, value);
        }

        f(&mut command);

        if try!(command.status()).success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "command failed".to_string()))
        }
    }

    fn create_kdb(&self) -> io::Result<()> {
        self.run_command(&self.kdb5_util, |command| {
            command
                .arg("create")
                .arg("-W")
                .arg("-s")
                .arg("-P")
                .arg("master")
        })
    }

    fn run_kadminl(&self, query: &str) -> io::Result<()> {
        self.run_command(&self.kadmin_local, |command| {
            command
                .arg("-q")
                .arg(query)
        })
    }
}
