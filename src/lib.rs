#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_char,c_int,c_uchar,c_uint,c_ulonglong,c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_session_struct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_string_struct {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_key_struct {
    _unused: [u8; 0],
}
pub type ssh_key = *mut ssh_key_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_scp_struct {
    _unused: [u8; 0],
}

pub type ssh_session = *mut ssh_session_struct;
pub type size_t = c_ulonglong;

pub const SSH_CRYPT: u32 = 2;
pub const SSH_MAC: u32 = 3;
pub const SSH_COMP: u32 = 4;
pub const SSH_LANG: u32 = 5;
pub const SSH_AUTH_METHOD_UNKNOWN: u32 = 0;
pub const SSH_AUTH_METHOD_NONE: u32 = 1;
pub const SSH_AUTH_METHOD_PASSWORD: u32 = 2;
pub const SSH_AUTH_METHOD_PUBLICKEY: u32 = 4;
pub const SSH_AUTH_METHOD_HOSTBASED: u32 = 8;
pub const SSH_AUTH_METHOD_INTERACTIVE: u32 = 16;
pub const SSH_AUTH_METHOD_GSSAPI_MIC: u32 = 32;
pub const SSH_CLOSED: u32 = 1;
pub const SSH_READ_PENDING: u32 = 2;
pub const SSH_CLOSED_ERROR: u32 = 4;
pub const SSH_WRITE_PENDING: u32 = 8;
pub const MD5_DIGEST_LEN: u32 = 16;
pub const SSH_ADDRSTRLEN: u32 = 46;
pub const SSH_OK: u32 = 0;
pub const SSH_ERROR: i32 = -1;
pub const SSH_AGAIN: i32 = -2;
pub const SSH_EOF: i32 = -127;
pub const SSH_LOG_NONE: u32 = 0;
pub const SSH_LOG_WARN: u32 = 1;
pub const SSH_LOG_INFO: u32 = 2;
pub const SSH_LOG_DEBUG: u32 = 3;
pub const SSH_LOG_TRACE: u32 = 4;

pub type ssh_options_e = c_uint;

pub const SSH_OPTIONS_HOST: ssh_options_e = 0;
pub const SSH_OPTIONS_PORT: ssh_options_e = 1;
pub const SSH_OPTIONS_PORT_STR: ssh_options_e = 2;
pub const SSH_OPTIONS_FD: ssh_options_e = 3;
pub const SSH_OPTIONS_USER: ssh_options_e = 4;
pub const SSH_OPTIONS_SSH_DIR: ssh_options_e = 5;
pub const SSH_OPTIONS_IDENTITY: ssh_options_e = 6;
pub const SSH_OPTIONS_ADD_IDENTITY: ssh_options_e = 7;
pub const SSH_OPTIONS_KNOWNHOSTS: ssh_options_e = 8;
pub const SSH_OPTIONS_TIMEOUT: ssh_options_e = 9;
pub const SSH_OPTIONS_TIMEOUT_USEC: ssh_options_e = 10;
pub const SSH_OPTIONS_SSH1: ssh_options_e = 11;
pub const SSH_OPTIONS_SSH2: ssh_options_e = 12;
pub const SSH_OPTIONS_LOG_VERBOSITY: ssh_options_e = 13;
pub const SSH_OPTIONS_LOG_VERBOSITY_STR: ssh_options_e = 14;
pub const SSH_OPTIONS_CIPHERS_C_S: ssh_options_e = 15;
pub const SSH_OPTIONS_CIPHERS_S_C: ssh_options_e = 16;
pub const SSH_OPTIONS_COMPRESSION_C_S: ssh_options_e = 17;
pub const SSH_OPTIONS_COMPRESSION_S_C: ssh_options_e = 18;
pub const SSH_OPTIONS_PROXYCOMMAND: ssh_options_e = 19;
pub const SSH_OPTIONS_BINDADDR: ssh_options_e = 20;
pub const SSH_OPTIONS_STRICTHOSTKEYCHECK: ssh_options_e = 21;
pub const SSH_OPTIONS_COMPRESSION: ssh_options_e = 22;
pub const SSH_OPTIONS_COMPRESSION_LEVEL: ssh_options_e = 23;
pub const SSH_OPTIONS_KEY_EXCHANGE: ssh_options_e = 24;
pub const SSH_OPTIONS_HOSTKEYS: ssh_options_e = 25;
pub const SSH_OPTIONS_GSSAPI_SERVER_IDENTITY: ssh_options_e = 26;
pub const SSH_OPTIONS_GSSAPI_CLIENT_IDENTITY: ssh_options_e = 27;
pub const SSH_OPTIONS_GSSAPI_DELEGATE_CREDENTIALS: ssh_options_e = 28;
pub const SSH_OPTIONS_HMAC_C_S: ssh_options_e = 29;
pub const SSH_OPTIONS_HMAC_S_C: ssh_options_e = 30;
pub const SSH_OPTIONS_PASSWORD_AUTH: ssh_options_e = 31;
pub const SSH_OPTIONS_PUBKEY_AUTH: ssh_options_e = 32;
pub const SSH_OPTIONS_KBDINT_AUTH: ssh_options_e = 33;
pub const SSH_OPTIONS_GSSAPI_AUTH: ssh_options_e = 34;
pub const SSH_OPTIONS_GLOBAL_KNOWNHOSTS: ssh_options_e = 35;
pub const SSH_OPTIONS_NODELAY: ssh_options_e = 36;
pub const SSH_OPTIONS_PUBLICKEY_ACCEPTED_TYPES: ssh_options_e = 37;
pub const SSH_OPTIONS_PROCESS_CONFIG: ssh_options_e = 38;
pub const SSH_OPTIONS_REKEY_DATA: ssh_options_e = 39;
pub const SSH_OPTIONS_REKEY_TIME: ssh_options_e = 40;

pub type ssh_publickey_hash_type = c_uint;

pub const SSH_PUBLICKEY_HASH_SHA1: ssh_publickey_hash_type = 0;
pub const SSH_PUBLICKEY_HASH_MD5: ssh_publickey_hash_type = 1;
pub const SSH_PUBLICKEY_HASH_SHA256: ssh_publickey_hash_type = 2;

pub type ssh_known_hosts_e = c_int;

#[doc = " There had been an error checking the host."]
pub const SSH_KNOWN_HOSTS_ERROR: ssh_known_hosts_e = -2;
#[doc = " The known host file does not exist. The host is thus unknown. File will"]
#[doc = " be created if host key is accepted."]
pub const SSH_KNOWN_HOSTS_NOT_FOUND: ssh_known_hosts_e = -1;
#[doc = " The server is unknown. User should confirm the public key hash is"]
#[doc = " correct."]
pub const SSH_KNOWN_HOSTS_UNKNOWN: ssh_known_hosts_e = 0;
#[doc = " The server is known and has not changed."]
pub const SSH_KNOWN_HOSTS_OK: ssh_known_hosts_e = 1;
#[doc = " The server key has changed. Either you are under attack or the"]
#[doc = " administrator changed the key. You HAVE to warn the user about a"]
#[doc = " possible attack."]
pub const SSH_KNOWN_HOSTS_CHANGED: ssh_known_hosts_e = 2;
#[doc = " The server gave use a key of a type while we had an other type recorded."]
#[doc = " It is a possible attack."]
pub const SSH_KNOWN_HOSTS_OTHER: ssh_known_hosts_e = 3;

extern "C" {
    pub fn ssh_connect(session: ssh_session) -> c_int;
    pub fn ssh_disconnect(session: ssh_session);
    pub fn ssh_free(session: ssh_session);
    pub fn ssh_get_disconnect_message(session: ssh_session) -> *const c_char;
    pub fn ssh_get_error(error: *mut c_void) -> *const c_char;
    pub fn ssh_get_error_code(error: *mut c_void) -> c_int;
    pub fn ssh_get_issue_banner(session: ssh_session) -> *mut c_char;
    pub fn ssh_get_openssh_version(session: ssh_session) -> c_int;
    pub fn ssh_get_server_publickey(session: ssh_session,
        key: *mut ssh_key) -> c_int;
    pub fn ssh_get_publickey_hash(key: ssh_key, type_: ssh_publickey_hash_type,
        hash: *mut *mut c_uchar, hlen: *mut size_t) -> c_int;
    pub fn ssh_key_free(key: ssh_key);
    pub fn ssh_new() -> ssh_session;
    pub fn ssh_options_copy(src: ssh_session, dest: *mut ssh_session) -> c_int;
    pub fn ssh_options_getopt(session: ssh_session, argcptr: *mut c_int, 
                              argv: *mut *mut c_char) -> c_int;
    pub fn ssh_options_set(session: ssh_session, type_: ssh_options_e,
                            value: *const c_void) -> c_int;
    pub fn ssh_session_is_known_server(session: ssh_session) -> ssh_known_hosts_e;
}
