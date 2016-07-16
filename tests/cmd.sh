#!/bin/sh

# This script is automatically run by the test container. Do not run it manually.

set -e

echo "Starting Kerberos setup."

# Note, this configuration may be duplicated in krb5.conf, kdc.conf and Dockerfile.

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L115
KERBEROS_DIR=/kerberos
REALM=KRBTEST.COM
KDC_PORT=61000
KADMIN_PORT=61001
KPASSWD_PORT=61002
IPROP_PORT=61004
SERVER_PORT=61005
HOSTNAME=localhost
USER_PRINC="user@$REALM"
USER_PASS="pw_user"
IMPERSONATOR_PRINC="impersonator@$REALM"
IMPERSONATOR_PASS="pw_impersonator"
IMPERSONATEE_PRINC="impersonatee@$REALM"
IMPERSONATEE_PASS="pw_impersonatee"
ADMIN_PRINC="user/admin@$REALM"
ADMIN_PASS="pw_admin"
HOST_PRINC="host/$HOSTNAME@$REALM"
HOST_PASS="pw_host"
MASTER_PASS="master"
# NFS_PRINC="nfs/$HOSTNAM@$REALM"
KRBTGT_PRINC="krbtgt/$HOSTNAME@$REALM"
KEYTAB="$KERBEROS_DIR/keytab"
KADMIN_CCACHE="$KERBEROS_DIR/kadmin_ccache"

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L132
KRB5_CONFIG=/etc/krb5.conf
KRB5_KDC_PROFILE=/usr/local/var/krb5kdc/kdc.conf
KRB5CCNAME="$KERBEROS_DIR/ccache"
KRB5_KTNAME="$KERBEROS_DIR/keytab"
KRB5_CLIENT_KTNAME="$KERBEROS_DIR/client_keytab"
KRB5CACHEDIR=/kerberos
KPROP_PORT=61003
KPROPD_PORT=$KPROP_PORT

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L143
echo "$ADMIN_PRINC *" > "$KERBEROS_DIR/acl"
echo "kiprop/$HOSTNAME@$REALM p" >> "$KERBEROS_DIR/acl"

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L144
echo "weak_password" > "$KERBEROS_DIR/dictfile"

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L146
# Note, the -W flag causes it to use /dev/urandom rather than /dev/random
# the -s flag causes it to effectively stash the master password into
# /etc/krb5kdc/.k5.KRBTEST.COM, and kadmin.local and krb5kdc read it from there.
echo "Creating database with kdb5_util."
kdb5_util create -W -s /kerberos/stash -P $MASTER_PASS -d /kerberos/db

# Skip https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L148
# krbtgt_keysalt seems optional?

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L151
echo "Adding principals with kadmin.local."
kadmin.local -d /kerberos/db -q "addprinc -pw $USER_PASS $USER_PRINC"
kadmin.local -d /kerberos/db -q "addprinc -pw $IMPERSONATOR_PASS $IMPERSONATOR_PRINC"
kadmin.local -d /kerberos/db -q "addprinc -pw $IMPERSONATEE_PASS $IMPERSONATEE_PRINC"
kadmin.local -d /kerberos/db -q "addprinc -pw $ADMIN_PASS $ADMIN_PRINC"

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L154
kadmin.local -d /kerberos/db -q "addprinc -pw $HOST_PASS $HOST_PRINC"

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L156
kadmin.local -d /kerberos/db -q "ktadd -k $KEYTAB -norandkey $HOST_PRINC"

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L158
# Note, not passing -n will cause it to daemonize.
# Logs will go into $KERBEROS_DIR/kdc.log.
echo "Starting kdc."
krb5kdc -P "$KERBEROS_DIR/kdc.pid" || (echo "Log file contents: " && cat /kerberos/kdc.log && exit 1)

# https://github.com/pythongssapi/k5test/blob/27eae50f/k5test/realm.py#L160
# Note, it includes a lot of undocumented arguments. Leaving them off.
# Note, not passing -nofork causes it to daemonize.
# Logs will go into $KERBEROS_DIR/kadmin5.log.
echo "Starting kadmind."
kadmind -P "$KERDEROS_DIR/kadmind.pid" -F "$KERBEROS_DIR/dump"

echo "Adding the user to the local keytab for gssapi."
kadmin.local -d /kerberos/db -q "ktadd -k /etc/krb5.keytab -norandkey $USER_PRINC"

echo "Daemons started, kinit-ing user principal."
kinit -l 3600s -k -t /etc/krb5.keytab $USER_PRINC

echo "Done setting up Kerberos, running Rust tests."
cargo test