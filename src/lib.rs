#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_int,c_char,c_void,c_uint};

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
pub type ssh_options_e = c_uint;
pub type ssh_publickey_hash_type = c_uint;
pub type ssh_session = *mut ssh_session_struct;

pub const ssh_options_e_SSH_OPTIONS_HOST: ssh_options_e = 0;
pub const ssh_options_e_SSH_OPTIONS_PORT: ssh_options_e = 1;
pub const ssh_options_e_SSH_OPTIONS_PORT_STR: ssh_options_e = 2;
pub const ssh_options_e_SSH_OPTIONS_FD: ssh_options_e = 3;
pub const ssh_options_e_SSH_OPTIONS_USER: ssh_options_e = 4;
pub const ssh_options_e_SSH_OPTIONS_SSH_DIR: ssh_options_e = 5;
pub const ssh_options_e_SSH_OPTIONS_IDENTITY: ssh_options_e = 6;
pub const ssh_options_e_SSH_OPTIONS_ADD_IDENTITY: ssh_options_e = 7;
pub const ssh_options_e_SSH_OPTIONS_KNOWNHOSTS: ssh_options_e = 8;
pub const ssh_options_e_SSH_OPTIONS_TIMEOUT: ssh_options_e = 9;
pub const ssh_options_e_SSH_OPTIONS_TIMEOUT_USEC: ssh_options_e = 10;
pub const ssh_options_e_SSH_OPTIONS_SSH1: ssh_options_e = 11;
pub const ssh_options_e_SSH_OPTIONS_SSH2: ssh_options_e = 12;
pub const ssh_options_e_SSH_OPTIONS_LOG_VERBOSITY: ssh_options_e = 13;
pub const ssh_options_e_SSH_OPTIONS_LOG_VERBOSITY_STR: ssh_options_e = 14;
pub const ssh_options_e_SSH_OPTIONS_CIPHERS_C_S: ssh_options_e = 15;
pub const ssh_options_e_SSH_OPTIONS_CIPHERS_S_C: ssh_options_e = 16;
pub const ssh_options_e_SSH_OPTIONS_COMPRESSION_C_S: ssh_options_e = 17;
pub const ssh_options_e_SSH_OPTIONS_COMPRESSION_S_C: ssh_options_e = 18;
pub const ssh_options_e_SSH_OPTIONS_PROXYCOMMAND: ssh_options_e = 19;
pub const ssh_options_e_SSH_OPTIONS_BINDADDR: ssh_options_e = 20;
pub const ssh_options_e_SSH_OPTIONS_STRICTHOSTKEYCHECK: ssh_options_e = 21;
pub const ssh_options_e_SSH_OPTIONS_COMPRESSION: ssh_options_e = 22;
pub const ssh_options_e_SSH_OPTIONS_COMPRESSION_LEVEL: ssh_options_e = 23;
pub const ssh_options_e_SSH_OPTIONS_KEY_EXCHANGE: ssh_options_e = 24;
pub const ssh_options_e_SSH_OPTIONS_HOSTKEYS: ssh_options_e = 25;
pub const ssh_options_e_SSH_OPTIONS_GSSAPI_SERVER_IDENTITY: ssh_options_e = 26;
pub const ssh_options_e_SSH_OPTIONS_GSSAPI_CLIENT_IDENTITY: ssh_options_e = 27;
pub const ssh_options_e_SSH_OPTIONS_GSSAPI_DELEGATE_CREDENTIALS: ssh_options_e = 28;
pub const ssh_options_e_SSH_OPTIONS_HMAC_C_S: ssh_options_e = 29;
pub const ssh_options_e_SSH_OPTIONS_HMAC_S_C: ssh_options_e = 30;
pub const ssh_options_e_SSH_OPTIONS_PASSWORD_AUTH: ssh_options_e = 31;
pub const ssh_options_e_SSH_OPTIONS_PUBKEY_AUTH: ssh_options_e = 32;
pub const ssh_options_e_SSH_OPTIONS_KBDINT_AUTH: ssh_options_e = 33;
pub const ssh_options_e_SSH_OPTIONS_GSSAPI_AUTH: ssh_options_e = 34;
pub const ssh_options_e_SSH_OPTIONS_GLOBAL_KNOWNHOSTS: ssh_options_e = 35;
pub const ssh_options_e_SSH_OPTIONS_NODELAY: ssh_options_e = 36;
pub const ssh_options_e_SSH_OPTIONS_PUBLICKEY_ACCEPTED_TYPES: ssh_options_e = 37;
pub const ssh_options_e_SSH_OPTIONS_PROCESS_CONFIG: ssh_options_e = 38;
pub const ssh_options_e_SSH_OPTIONS_REKEY_DATA: ssh_options_e = 39;
pub const ssh_options_e_SSH_OPTIONS_REKEY_TIME: ssh_options_e = 40;

pub const ssh_publickey_hash_type_SSH_PUBLICKEY_HASH_SHA1: ssh_publickey_hash_type = 0;
pub const ssh_publickey_hash_type_SSH_PUBLICKEY_HASH_MD5: ssh_publickey_hash_type = 1;
pub const ssh_publickey_hash_type_SSH_PUBLICKEY_HASH_SHA256: ssh_publickey_hash_type = 2;

extern "C" {
    pub fn ssh_connect(session: ssh_session) -> c_int;
    pub fn ssh_disconnect(session: ssh_session);
    pub fn ssh_free(session: ssh_session);
    pub fn ssh_get_disconnect_message(session: ssh_session) -> *const c_char;
    pub fn ssh_get_error(error: *mut c_void) -> *const c_char;
    pub fn ssh_get_error_code(error: *mut c_void) -> c_int;
    pub fn ssh_get_issue_banner(session: ssh_session) -> *mut c_char;
    pub fn ssh_get_openssh_version(session: ssh_session) -> c_int;
    pub fn ssh_new() -> ssh_session;
    pub fn ssh_options_copy(src: ssh_session, dest: *mut ssh_session) -> c_int;
    pub fn ssh_options_getopt(session: ssh_session, argcptr: *mut c_int, 
                              argv: *mut *mut c_char) -> c_int;
    pub fn ssh_options_set(session: ssh_session, type_: ssh_options_e,
                            value: *const c_void) -> c_int;
}
