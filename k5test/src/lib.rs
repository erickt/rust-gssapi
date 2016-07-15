#![cfg_attr(feature = "unstable-testing", feature(plugin))]
#![cfg_attr(feature = "unstable-testing", plugin(clippy))]

extern crate quale;
extern crate resolve;
extern crate tempdir;

use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Output;

pub struct K5RealmBuilder {
    realm: String,
    port_base: u16,
    principals: Vec<(String, Option<String>)>,
    kdb5_util: Option<PathBuf>,
    kadmin_local: Option<PathBuf>,
}

impl K5RealmBuilder {
    pub fn new(realm: String) -> Self {
        K5RealmBuilder {
            realm: realm,
            port_base: 61000,
            principals: vec![],
            kdb5_util: None,
            kadmin_local: None,
        }
    }
    
    pub fn kdb5_util(mut self, path: PathBuf) -> Self {
        self.kdb5_util = Some(path);
        self
    }

    pub fn kadmin_local(mut self, path: PathBuf) -> Self {
        self.kadmin_local = Some(path);
        self
    }

    pub fn add_principal(mut self, principal: String, password: Option<String>) -> Self {
        self.principals.push((principal, password));
        self
    }

    pub fn build(self) -> io::Result<K5Realm> {
        let dir = tempdir::TempDir::new("").unwrap();
        let krb5_conf_path = try!(self.write_krb5_conf(dir.path()));
        let kdc_conf_path = try!(self.write_kdc_conf(dir.path()));

        let env = vec![
            ("KRB5_CONFIG", krb5_conf_path.into()),
            ("KRB5_KDC_PROFILE", kdc_conf_path.into()),
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
            kadmin_local: 
                try!(find_path(
                    "kadmin_local",
                    self.kadmin_local.into_iter().chain(vec![
                        PathBuf::new().join("kadmin_local"),
                        PathBuf::new().join("kadmin.local"),
                    ])
                )),
            kdb5_util: try!(
                find_path(
                    "kdb5_util",
                    self.kdb5_util.into_iter().chain(vec![
                        PathBuf::new().join("kdb5_util"),
                    ])
                )),
            env: env,
        };

        try!(realm.create_kdb());

        for (principal, password) in self.principals {
            try!(realm.add_principal(principal, password));
        }

        Ok(realm)
    }

    fn kdc_port(&self) -> u16 {
        self.port_base
    }

    fn kadmin_port(&self) -> u16 {
        self.port_base + 1
    }

    fn kpasswd_port(&self) -> u16 {
        self.port_base + 2
    }

    fn iprop_port(&self) -> u16 {
        self.port_base + 4
    }

    fn write_krb5_conf(&self, dir: &Path) -> io::Result<PathBuf> {
        let conf_path = dir.join("krb5.conf");
        let hostname = try!(resolve::hostname::get_hostname());

        let mut file = try!(File::create(&conf_path));
        try!(write!(file, "\
[libdefaults]
    default_realm = {realm}
    dns_lookup_kdc = false

[realms]
    {realm} = {{
        kdc = {hostname}:{kdc_port}
        admin_server = {hostname}:{kadmin_port}
        kpasswd_server = {hostname}:{kpasswd_port}
    }}
",
            realm = self.realm,
            hostname = hostname,
            kdc_port = self.kdc_port(),
            kadmin_port = self.kadmin_port(),
            kpasswd_port = self.kpasswd_port(),
        ));

        Ok(conf_path)
    }

    fn write_kdc_conf(&self, dir: &Path) -> io::Result<PathBuf> {
        let conf_path = dir.join("kdc.conf");

        let mut file = try!(File::create(&conf_path));
        try!(write!(file, "\
[realms]
    {realm} = {{
        database_module = db
        iprop_port = {iprop_port}
        key_stash_file = {tmpdir}/stash
        acl_file = {tmpdir}/acl
        dictfile = {tmpdir}/dictfile
        kadmin_port = {kadmin_port}
        kpasswd_port = {kpasswd_port}
        kdc_ports = {kdc_port}
        kdc_tcp_ports = {kdc_port}
        database_name = {tmpdir}/db
    }}
[dbmodule]
    db = {{
        db_library = db2
        database_name = {tmpdir}/db
    }}
[logging]
    admin_server = FILE:{tmpdir}/kadmin5.log
    kdc = FILE:{tmpdir}/kdc.log
    default = FILE:{tmpdir}/others.log
",
            realm = self.realm,
            tmpdir = dir.to_str().expect("tmpdir has non-utf8 characters!"),
            iprop_port = self.iprop_port(),
            kadmin_port = self.kadmin_port(),
            kpasswd_port = self.kpasswd_port(),
            kdc_port = self.kdc_port(),
        ));

        Ok(conf_path)
    }
}

fn find_path<T>(name: &str, paths: T) -> io::Result<PathBuf>
    where T: Iterator<Item=PathBuf>
{
    for path in paths {
        if let Some(path) = quale::which(path) {
            return Ok(path)
        }
    }

    Err(io::Error::new(io::ErrorKind::Other, format!("could not find {}", name)))
}


pub struct K5Realm {
    _dir: tempdir::TempDir,
    _realm: String,
    kadmin_local: PathBuf,
    kdb5_util: PathBuf,
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
    
    pub fn realm(&self) -> String {
        self._realm.clone()
    }

    fn run_command<F>(&self, command: &Path, f: F) -> io::Result<Output>
        where F: FnOnce(&mut Command) -> &mut Command
    {
        let mut command = Command::new(command);

        for &(ref key, ref value) in &self.env {
            command.env(key, value);
        }

        f(&mut command);
        command.output()
    }

    fn create_kdb(&self) -> io::Result<()> {
        let output = try!(self.run_command(&self.kdb5_util, |command| {
            command
                .arg("create")
                .arg("-W")
                .arg("-s")
                .arg("-P")
                .arg("master")
        }));
        println!("kdb status: {}", output.status);
        println!("kdb stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("kdb stderr: {}", String::from_utf8_lossy(&output.stderr));
        Ok(())
    }

    fn run_kadminl(&self, query: &str) -> io::Result<()> {
        try!(self.run_command(&self.kadmin_local, |command| {
            command
                .arg("-q")
                .arg(query)
        }));
        Ok(())
    }
}
