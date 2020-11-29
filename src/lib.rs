#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_char,c_int,c_long,c_short,c_uchar,c_uint,c_ulonglong,c_void,timeval};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_session_struct {
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

pub type ssh_kex_types_e = c_uint;

pub const SSH_KEX: ssh_kex_types_e = 0;
pub const SSH_HOSTKEYS: ssh_kex_types_e = 1;
pub const SSH_CRYPT_C_S: ssh_kex_types_e = 2;
pub const SSH_CRYPT_S_C: ssh_kex_types_e = 3;
pub const SSH_MAC_C_S: ssh_kex_types_e = 4;
pub const SSH_MAC_S_C: ssh_kex_types_e = 5;
pub const SSH_COMP_C_S: ssh_kex_types_e = 6;
pub const SSH_COMP_S_C: ssh_kex_types_e = 7;
pub const SSH_LANG_C_S: ssh_kex_types_e = 8;
pub const SSH_LANG_S_C: ssh_kex_types_e = 9;

pub type ssh_auth_e = c_int;

pub const SSH_AUTH_SUCCESS: ssh_auth_e = 0;
pub const SSH_AUTH_DENIED: ssh_auth_e = 1;
pub const SSH_AUTH_PARTIAL: ssh_auth_e = 2;
pub const SSH_AUTH_INFO: ssh_auth_e = 3;
pub const SSH_AUTH_AGAIN: ssh_auth_e = 4;
pub const SSH_AUTH_ERROR: ssh_auth_e = -1;

pub type ssh_requests_e = c_uint;

pub const SSH_REQUEST_AUTH: ssh_requests_e = 1;
pub const SSH_REQUEST_CHANNEL_OPEN: ssh_requests_e = 2;
pub const SSH_REQUEST_CHANNEL: ssh_requests_e = 3;
pub const SSH_REQUEST_SERVICE: ssh_requests_e = 4;
pub const SSH_REQUEST_GLOBAL: ssh_requests_e = 5;

pub type ssh_channel_type_e = c_uint;

pub const SSH_CHANNEL_UNKNOWN: ssh_channel_type_e = 0;
pub const SSH_CHANNEL_SESSION: ssh_channel_type_e = 1;
pub const SSH_CHANNEL_DIRECT_TCPIP: ssh_channel_type_e = 2;
pub const SSH_CHANNEL_FORWARDED_TCPIP: ssh_channel_type_e = 3;
pub const SSH_CHANNEL_X11: ssh_channel_type_e = 4;
pub const SSH_CHANNEL_AUTH_AGENT: ssh_channel_type_e = 5;

pub type ssh_channel_requests_e = c_uint;

pub const SSH_CHANNEL_REQUEST_UNKNOWN: ssh_channel_requests_e = 0;
pub const SSH_CHANNEL_REQUEST_PTY: ssh_channel_requests_e = 1;
pub const SSH_CHANNEL_REQUEST_EXEC: ssh_channel_requests_e = 2;
pub const SSH_CHANNEL_REQUEST_SHELL: ssh_channel_requests_e = 3;
pub const SSH_CHANNEL_REQUEST_ENV: ssh_channel_requests_e = 4;
pub const SSH_CHANNEL_REQUEST_SUBSYSTEM: ssh_channel_requests_e = 5;
pub const SSH_CHANNEL_REQUEST_WINDOW_CHANGE: ssh_channel_requests_e = 6;
pub const SSH_CHANNEL_REQUEST_X11: ssh_channel_requests_e = 7;

pub type ssh_global_requests_e = c_uint;

pub const SSH_GLOBAL_REQUEST_UNKNOWN: ssh_global_requests_e = 0;
pub const SSH_GLOBAL_REQUEST_TCPIP_FORWARD: ssh_global_requests_e = 1;
pub const SSH_GLOBAL_REQUEST_CANCEL_TCPIP_FORWARD: ssh_global_requests_e = 2;
pub const SSH_GLOBAL_REQUEST_KEEPALIVE: ssh_global_requests_e = 3;

pub type ssh_publickey_state_e = c_int;

pub const SSH_PUBLICKEY_STATE_ERROR: ssh_publickey_state_e = -1;
pub const SSH_PUBLICKEY_STATE_NONE: ssh_publickey_state_e = 0;
pub const SSH_PUBLICKEY_STATE_VALID: ssh_publickey_state_e = 1;
pub const SSH_PUBLICKEY_STATE_WRONG: ssh_publickey_state_e = 2;

pub type ssh_server_known_e = c_int;

pub const SSH_SERVER_ERROR: ssh_server_known_e = -1;
pub const SSH_SERVER_NOT_KNOWN: ssh_server_known_e = 0;
pub const SSH_SERVER_KNOWN_OK: ssh_server_known_e = 1;
pub const SSH_SERVER_KNOWN_CHANGED: ssh_server_known_e = 2;
pub const SSH_SERVER_FOUND_OTHER: ssh_server_known_e = 3;
pub const SSH_SERVER_FILE_NOT_FOUND: ssh_server_known_e = 4;

pub type ssh_error_types_e = c_uint;

pub const SSH_NO_ERROR: ssh_error_types_e = 0;
pub const SSH_REQUEST_DENIED: ssh_error_types_e = 1;
pub const SSH_FATAL: ssh_error_types_e = 2;
pub const SSH_EINTR: ssh_error_types_e = 3;

pub type ssh_keytypes_e = c_uint;

pub const SSH_KEYTYPE_UNKNOWN: ssh_keytypes_e = 0;
pub const SSH_KEYTYPE_DSS: ssh_keytypes_e = 1;
pub const SSH_KEYTYPE_RSA: ssh_keytypes_e = 2;
pub const SSH_KEYTYPE_RSA1: ssh_keytypes_e = 3;
pub const SSH_KEYTYPE_ECDSA: ssh_keytypes_e = 4;
pub const SSH_KEYTYPE_ED25519: ssh_keytypes_e = 5;
pub const SSH_KEYTYPE_DSS_CERT01: ssh_keytypes_e = 6;
pub const SSH_KEYTYPE_RSA_CERT01: ssh_keytypes_e = 7;
pub const SSH_KEYTYPE_ECDSA_P256: ssh_keytypes_e = 8;
pub const SSH_KEYTYPE_ECDSA_P384: ssh_keytypes_e = 9;
pub const SSH_KEYTYPE_ECDSA_P521: ssh_keytypes_e = 10;
pub const SSH_KEYTYPE_ECDSA_P256_CERT01: ssh_keytypes_e = 11;
pub const SSH_KEYTYPE_ECDSA_P384_CERT01: ssh_keytypes_e = 12;
pub const SSH_KEYTYPE_ECDSA_P521_CERT01: ssh_keytypes_e = 13;
pub const SSH_KEYTYPE_ED25519_CERT01: ssh_keytypes_e = 14;

pub type ssh_keycmp_e = c_uint;

pub const SSH_KEY_CMP_PUBLIC: ssh_keycmp_e = 0;
pub const SSH_KEY_CMP_PRIVATE: ssh_keycmp_e = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_knownhosts_entry {
    pub hostname: *mut c_char,
    pub unparsed: *mut c_char,
    pub publickey: ssh_key,
    pub comment: *mut c_char,
}
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
pub const SSH_OK: i32 = 0;
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
    pub fn ssh_blocking_flush(session: ssh_session, timeout: c_int) -> c_int;
    pub fn ssh_channel_accept_x11(channel: ssh_channel, timeout_ms: c_int,
        ) -> ssh_channel;
    pub fn ssh_channel_change_pty_size(channel: ssh_channel, cols: c_int,
        rows: c_int) -> c_int;
    pub fn ssh_channel_close(channel: ssh_channel) -> c_int;
    pub fn ssh_channel_free(channel: ssh_channel);
    pub fn ssh_channel_get_exit_status(channel: ssh_channel) -> c_int;
    pub fn ssh_channel_get_session(channel: ssh_channel) -> ssh_session;
    pub fn ssh_channel_is_closed(channel: ssh_channel) -> c_int;
    pub fn ssh_channel_is_eof(channel: ssh_channel) -> c_int;
    pub fn ssh_channel_is_open(channel: ssh_channel) -> c_int;
    pub fn ssh_channel_new(session: ssh_session) -> ssh_channel;
    pub fn ssh_channel_open_auth_agent(channel: ssh_channel) -> c_int;
    pub fn ssh_channel_open_forward(
        channel: ssh_channel,
        remotehost: *const c_char,
        remoteport: c_int,
        sourcehost: *const c_char,
        localport: c_int,
    ) -> c_int;
    pub fn ssh_channel_open_forward_unix(
        channel: ssh_channel,
        remotepath: *const c_char,
        sourcehost: *const c_char,
        localport: c_int,
    ) -> c_int;
    pub fn ssh_clean_pubkey_hash(hash: *mut *mut c_uchar);
    pub fn ssh_key_free(key: ssh_key);
    pub fn ssh_new() -> ssh_session;
    pub fn ssh_options_copy(src: ssh_session, dest: *mut ssh_session) -> c_int;
    pub fn ssh_options_getopt(session: ssh_session, argcptr: *mut c_int, 
        argv: *mut *mut c_char) -> c_int;
    pub fn ssh_options_set(session: ssh_session, type_: ssh_options_e,
        value: *const c_void) -> c_int;
    pub fn ssh_print_hexa(descr: *const c_char, what: *const c_uchar, len: size_t);
    pub fn ssh_session_is_known_server(session: ssh_session) -> ssh_known_hosts_e;
    pub fn ssh_channel_open_session(channel: ssh_channel) -> c_int;
    pub fn ssh_channel_open_x11(
        channel: ssh_channel,
        orig_addr: *const c_char,
        orig_port: c_int,
    ) -> c_int;
    pub fn ssh_channel_poll(
        channel: ssh_channel,
        is_stderr: c_int,
    ) -> c_int;
    pub fn ssh_channel_poll_timeout(
        channel: ssh_channel,
        timeout: c_int,
        is_stderr: c_int,
    ) -> c_int;
    pub fn ssh_channel_read(
        channel: ssh_channel,
        dest: *mut c_void,
        count: u32,
        is_stderr: c_int,
    ) -> c_int;

    pub fn ssh_channel_read_timeout(
        channel: ssh_channel,
        dest: *mut c_void,
        count: u32,
        is_stderr: c_int,
        timeout_ms: c_int,
    ) -> c_int;

    pub fn ssh_channel_read_nonblocking(
        channel: ssh_channel,
        dest: *mut c_void,
        count: u32,
        is_stderr: c_int,
    ) -> c_int;

    pub fn ssh_channel_request_env(
        channel: ssh_channel,
        name: *const c_char,
        value: *const c_char,
    ) -> c_int;

    pub fn ssh_channel_request_exec(
        channel: ssh_channel,
        cmd: *const c_char,
    ) -> c_int;

    pub fn ssh_channel_request_pty(channel: ssh_channel) -> c_int;

    pub fn ssh_channel_request_pty_size(
        channel: ssh_channel,
        term: *const c_char,
        cols: c_int,
        rows: c_int,
    ) -> c_int;

    pub fn ssh_channel_request_shell(channel: ssh_channel) -> c_int;

    pub fn ssh_channel_request_send_signal(
        channel: ssh_channel,
        signum: *const c_char,
    ) -> c_int;

    pub fn ssh_channel_request_send_break(
        channel: ssh_channel,
        length: u32,
    ) -> c_int;

    pub fn ssh_channel_request_sftp(channel: ssh_channel) -> c_int;

    pub fn ssh_channel_request_subsystem(
        channel: ssh_channel,
        subsystem: *const c_char,
    ) -> c_int;

    pub fn ssh_channel_request_x11(
        channel: ssh_channel,
        single_connection: c_int,
        protocol: *const c_char,
        cookie: *const c_char,
        screen_number: c_int,
    ) -> c_int;

    pub fn ssh_channel_request_auth_agent(channel: ssh_channel) -> c_int;

    pub fn ssh_channel_send_eof(channel: ssh_channel) -> c_int;

    pub fn ssh_channel_select(
        readchans: *mut ssh_channel,
        writechans: *mut ssh_channel,
        exceptchans: *mut ssh_channel,
        timeout: *mut timeval,
    ) -> c_int;

    pub fn ssh_channel_set_blocking(channel: ssh_channel, blocking: c_int);

    pub fn ssh_channel_set_counter(channel: ssh_channel, counter: ssh_counter);

    pub fn ssh_channel_write(
        channel: ssh_channel,
        data: *const c_void,
        len: u32,
    ) -> c_int;

    pub fn ssh_channel_write_stderr(
        channel: ssh_channel,
        data: *const c_void,
        len: u32,
    ) -> c_int;

    pub fn ssh_channel_window_size(channel: ssh_channel) -> u32;

    pub fn ssh_basename(path: *const c_char) -> *mut c_char;


    pub fn ssh_connect(session: ssh_session) -> c_int;

    pub fn ssh_connector_new(session: ssh_session) -> ssh_connector;

    pub fn ssh_connector_free(connector: ssh_connector);

    pub fn ssh_connector_set_in_channel(
        connector: ssh_connector,
        channel: ssh_channel,
        flags: ssh_connector_flags_e,
    ) -> c_int;

    pub fn ssh_connector_set_out_channel(
        connector: ssh_connector,
        channel: ssh_channel,
        flags: ssh_connector_flags_e,
    ) -> c_int;

    pub fn ssh_connector_set_in_fd(connector: ssh_connector, fd: socket_t);

    pub fn ssh_connector_set_out_fd(connector: ssh_connector, fd: socket_t);

    pub fn ssh_copyright() -> *const c_char;

    pub fn ssh_disconnect(session: ssh_session);

    pub fn ssh_dirname(path: *const c_char) -> *mut c_char;

    pub fn ssh_finalize() -> c_int;

    pub fn ssh_channel_accept_forward(
        session: ssh_session,
        timeout_ms: c_int,
        destination_port: *mut c_int,
    ) -> ssh_channel;

    pub fn ssh_channel_cancel_forward(
        session: ssh_session,
        address: *const c_char,
        port: c_int,
    ) -> c_int;

    pub fn ssh_channel_listen_forward(
        session: ssh_session,
        address: *const c_char,
        port: c_int,
        bound_port: *mut c_int,
    ) -> c_int;

    pub fn ssh_free(session: ssh_session);

    pub fn ssh_get_disconnect_message(session: ssh_session) -> *const c_char;

    pub fn ssh_get_error(error: *mut c_void) -> *const c_char;

    pub fn ssh_get_error_code(error: *mut c_void) -> c_int;

    pub fn ssh_get_fd(session: ssh_session) -> socket_t;

    pub fn ssh_get_hexa(
        what: *const c_uchar,
        len: size_t,
    ) -> *mut c_char;

    pub fn ssh_get_issue_banner(session: ssh_session) -> *mut c_char;

    pub fn ssh_get_openssh_version(session: ssh_session) -> c_int;

    pub fn ssh_get_server_publickey(
        session: ssh_session,
        key: *mut ssh_key,
    ) -> c_int;

    pub fn ssh_get_publickey_hash(
        key: ssh_key,
        type_: ssh_publickey_hash_type,
        hash: *mut *mut c_uchar,
        hlen: *mut size_t,
    ) -> c_int;

    pub fn ssh_get_pubkey_hash(
        session: ssh_session,
        hash: *mut *mut c_uchar,
    ) -> c_int;

    pub fn ssh_forward_accept(
        session: ssh_session,
        timeout_ms: c_int,
    ) -> ssh_channel;

    pub fn ssh_forward_cancel(
        session: ssh_session,
        address: *const c_char,
        port: c_int,
    ) -> c_int;

    pub fn ssh_forward_listen(
        session: ssh_session,
        address: *const c_char,
        port: c_int,
        bound_port: *mut c_int,
    ) -> c_int;

    pub fn ssh_get_publickey(session: ssh_session, key: *mut ssh_key) -> c_int;

    pub fn ssh_write_knownhost(session: ssh_session) -> c_int;

    pub fn ssh_dump_knownhost(session: ssh_session) -> *mut c_char;

    pub fn ssh_is_server_known(session: ssh_session) -> c_int;

    pub fn ssh_get_random(
        where_: *mut c_void,
        len: c_int,
        strong: c_int,
    ) -> c_int;

    pub fn ssh_get_version(session: ssh_session) -> c_int;

    pub fn ssh_get_status(session: ssh_session) -> c_int;

    pub fn ssh_get_poll_flags(session: ssh_session) -> c_int;

    pub fn ssh_init() -> c_int;

    pub fn ssh_is_blocking(session: ssh_session) -> c_int;

    pub fn ssh_is_connected(session: ssh_session) -> c_int;

    pub fn ssh_knownhosts_entry_free(entry: *mut ssh_knownhosts_entry);

    pub fn ssh_known_hosts_parse_line(
        host: *const c_char,
        line: *const c_char,
        entry: *mut *mut ssh_knownhosts_entry,
    ) -> c_int;

    pub fn ssh_session_has_known_hosts_entry(session: ssh_session) -> ssh_known_hosts_e;

    pub fn ssh_session_export_known_hosts_entry(
        session: ssh_session,
        pentry_string: *mut *mut c_char,
    ) -> c_int;

    pub fn ssh_session_update_known_hosts(session: ssh_session) -> c_int;

    pub fn ssh_session_get_known_hosts_entry(
        session: ssh_session,
        pentry: *mut *mut ssh_knownhosts_entry,
    ) -> ssh_known_hosts_e;

    pub fn ssh_set_log_level(level: c_int) -> c_int;

    pub fn ssh_get_log_level() -> c_int;

    pub fn ssh_get_log_userdata() -> *mut c_void;

    pub fn ssh_set_log_userdata(data: *mut c_void) -> c_int;

    pub fn _ssh_log(
        verbosity: c_int,
        function: *const c_char,
        format: *const c_char,
        ...
    );

    pub fn ssh_log(
        session: ssh_session,
        prioriry: c_int,
        format: *const c_char,
        ...
    );

    pub fn ssh_message_channel_request_open_reply_accept(msg: ssh_message) -> ssh_channel;

    pub fn ssh_message_channel_request_open_reply_accept_channel(
        msg: ssh_message,
        chan: ssh_channel,
    ) -> c_int;

    pub fn ssh_message_channel_request_reply_success(msg: ssh_message) -> c_int;

    pub fn ssh_message_free(msg: ssh_message);

    pub fn ssh_message_get(session: ssh_session) -> ssh_message;

    pub fn ssh_message_subtype(msg: ssh_message) -> c_int;

    pub fn ssh_message_type(msg: ssh_message) -> c_int;

    pub fn ssh_mkdir(
        pathname: *const c_char,
        mode: mode_t,
    ) -> c_int;

    pub fn ssh_options_parse_config(
        session: ssh_session,
        filename: *const c_char,
    ) -> c_int;

    pub fn ssh_options_get(
        session: ssh_session,
        type_: ssh_options_e,
        value: *mut *mut c_char,
    ) -> c_int;

    pub fn ssh_options_get_port(
        session: ssh_session,
        port_target: *mut c_uint,
    ) -> c_int;

    pub fn ssh_pcap_file_close(pcap: ssh_pcap_file) -> c_int;

    pub fn ssh_pcap_file_free(pcap: ssh_pcap_file);

    pub fn ssh_pcap_file_new() -> ssh_pcap_file;

    pub fn ssh_pcap_file_open(
        pcap: ssh_pcap_file,
        filename: *const c_char,
    ) -> c_int;

    #[doc = " @}"]
    pub fn ssh_key_new() -> ssh_key;

    pub fn ssh_key_type(key: ssh_key) -> ssh_keytypes_e;

    pub fn ssh_key_type_to_char(type_: ssh_keytypes_e) -> *const c_char;

    pub fn ssh_key_type_from_name(name: *const c_char) -> ssh_keytypes_e;

    pub fn ssh_key_is_public(k: ssh_key) -> c_int;

    pub fn ssh_key_is_private(k: ssh_key) -> c_int;

    pub fn ssh_key_cmp(k1: ssh_key, k2: ssh_key, what: ssh_keycmp_e) -> c_int;

    pub fn ssh_pki_generate(
        type_: ssh_keytypes_e,
        parameter: c_int,
        pkey: *mut ssh_key,
    ) -> c_int;

    pub fn ssh_pki_import_privkey_base64(
        b64_key: *const c_char,
        passphrase: *const c_char,
        auth_fn: ssh_auth_callback,
        auth_data: *mut c_void,
        pkey: *mut ssh_key,
    ) -> c_int;

    pub fn ssh_pki_export_privkey_base64(
        privkey: ssh_key,
        passphrase: *const c_char,
        auth_fn: ssh_auth_callback,
        auth_data: *mut c_void,
        b64_key: *mut *mut c_char,
    ) -> c_int;

    pub fn ssh_pki_import_privkey_file(
        filename: *const c_char,
        passphrase: *const c_char,
        auth_fn: ssh_auth_callback,
        auth_data: *mut c_void,
        pkey: *mut ssh_key,
    ) -> c_int;

    pub fn ssh_pki_export_privkey_file(
        privkey: ssh_key,
        passphrase: *const c_char,
        auth_fn: ssh_auth_callback,
        auth_data: *mut c_void,
        filename: *const c_char,
    ) -> c_int;

    pub fn ssh_pki_copy_cert_to_privkey(
        cert_key: ssh_key,
        privkey: ssh_key,
    ) -> c_int;

    pub fn ssh_pki_import_pubkey_base64(
        b64_key: *const c_char,
        type_: ssh_keytypes_e,
        pkey: *mut ssh_key,
    ) -> c_int;

    pub fn ssh_pki_import_pubkey_file(
        filename: *const c_char,
        pkey: *mut ssh_key,
    ) -> c_int;

    pub fn ssh_pki_import_cert_base64(
        b64_cert: *const c_char,
        type_: ssh_keytypes_e,
        pkey: *mut ssh_key,
    ) -> c_int;

    pub fn ssh_pki_import_cert_file(
        filename: *const c_char,
        pkey: *mut ssh_key,
    ) -> c_int;

    pub fn ssh_pki_export_privkey_to_pubkey(
        privkey: ssh_key,
        pkey: *mut ssh_key,
    ) -> c_int;

    pub fn ssh_pki_export_pubkey_base64(
        key: ssh_key,
        b64_key: *mut *mut c_char,
    ) -> c_int;

    pub fn ssh_pki_export_pubkey_file(
        key: ssh_key,
        filename: *const c_char,
    ) -> c_int;

    pub fn ssh_pki_key_ecdsa_name(key: ssh_key) -> *const c_char;

    pub fn ssh_get_fingerprint_hash(
        type_: ssh_publickey_hash_type,
        hash: *mut c_uchar,
        len: size_t,
    ) -> *mut c_char;

    pub fn ssh_print_hash(
        type_: ssh_publickey_hash_type,
        hash: *mut c_uchar,
        len: size_t,
    );

    pub fn ssh_send_ignore(
        session: ssh_session,
        data: *const c_char,
    ) -> c_int;

    pub fn ssh_send_debug(
        session: ssh_session,
        message: *const c_char,
        always_display: c_int,
    ) -> c_int;

    pub fn ssh_gssapi_set_creds(session: ssh_session, creds: ssh_gssapi_creds);

    pub fn ssh_scp_accept_request(scp: ssh_scp) -> c_int;

    pub fn ssh_scp_close(scp: ssh_scp) -> c_int;

    pub fn ssh_scp_deny_request(
        scp: ssh_scp,
        reason: *const c_char,
    ) -> c_int;

    pub fn ssh_scp_free(scp: ssh_scp);

    pub fn ssh_scp_init(scp: ssh_scp) -> c_int;

    pub fn ssh_scp_leave_directory(scp: ssh_scp) -> c_int;

    pub fn ssh_scp_new(
        session: ssh_session,
        mode: c_int,
        location: *const c_char,
    ) -> ssh_scp;

    pub fn ssh_scp_pull_request(scp: ssh_scp) -> c_int;

    pub fn ssh_scp_push_directory(
        scp: ssh_scp,
        dirname: *const c_char,
        mode: c_int,
    ) -> c_int;

    pub fn ssh_scp_push_file(
        scp: ssh_scp,
        filename: *const c_char,
        size: size_t,
        perms: c_int,
    ) -> c_int;

    pub fn ssh_scp_push_file64(
        scp: ssh_scp,
        filename: *const c_char,
        size: u64,
        perms: c_int,
    ) -> c_int;

    pub fn ssh_scp_read(
        scp: ssh_scp,
        buffer: *mut c_void,
        size: size_t,
    ) -> c_int;

    pub fn ssh_scp_request_get_filename(scp: ssh_scp) -> *const c_char;

    pub fn ssh_scp_request_get_permissions(scp: ssh_scp) -> c_int;

    pub fn ssh_scp_request_get_size(scp: ssh_scp) -> size_t;

    pub fn ssh_scp_request_get_size64(scp: ssh_scp) -> u64;

    pub fn ssh_scp_request_get_warning(scp: ssh_scp) -> *const c_char;

    pub fn ssh_scp_write(
        scp: ssh_scp,
        buffer: *const c_void,
        len: size_t,
    ) -> c_int;

    pub fn ssh_select(
        channels: *mut ssh_channel,
        outchannels: *mut ssh_channel,
        maxfd: socket_t,
        readfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;

    pub fn ssh_service_request(
        session: ssh_session,
        service: *const c_char,
    ) -> c_int;

    pub fn ssh_set_agent_channel(
        session: ssh_session,
        channel: ssh_channel,
    ) -> c_int;

    pub fn ssh_set_agent_socket(session: ssh_session, fd: socket_t) -> c_int;

    pub fn ssh_set_blocking(session: ssh_session, blocking: c_int);

    pub fn ssh_set_counters(session: ssh_session, scounter: ssh_counter, rcounter: ssh_counter);

    pub fn ssh_set_fd_except(session: ssh_session);

    pub fn ssh_set_fd_toread(session: ssh_session);

    pub fn ssh_set_fd_towrite(session: ssh_session);

    pub fn ssh_silent_disconnect(session: ssh_session);

    pub fn ssh_set_pcap_file(
        session: ssh_session,
        pcapfile: ssh_pcap_file,
    ) -> c_int;

    pub fn ssh_userauth_none(
        session: ssh_session,
        username: *const c_char,
    ) -> c_int;

    pub fn ssh_userauth_list(
        session: ssh_session,
        username: *const c_char,
    ) -> c_int;

    pub fn ssh_userauth_try_publickey(
        session: ssh_session,
        username: *const c_char,
        pubkey: ssh_key,
    ) -> c_int;

    pub fn ssh_userauth_publickey(
        session: ssh_session,
        username: *const c_char,
        privkey: ssh_key,
    ) -> c_int;

    pub fn ssh_userauth_agent(
        session: ssh_session,
        username: *const c_char,
    ) -> c_int;

    pub fn ssh_userauth_publickey_auto(
        session: ssh_session,
        username: *const c_char,
        passphrase: *const c_char,
    ) -> c_int;

    pub fn ssh_userauth_password(
        session: ssh_session,
        username: *const c_char,
        password: *const c_char,
    ) -> c_int;

    pub fn ssh_userauth_kbdint(
        session: ssh_session,
        user: *const c_char,
        submethods: *const c_char,
    ) -> c_int;

    pub fn ssh_userauth_kbdint_getinstruction(
        session: ssh_session,
    ) -> *const c_char;

    pub fn ssh_userauth_kbdint_getname(session: ssh_session) -> *const c_char;

    pub fn ssh_userauth_kbdint_getnprompts(session: ssh_session) -> c_int;

    pub fn ssh_userauth_kbdint_getprompt(
        session: ssh_session,
        i: c_uint,
        echo: *mut c_char,
    ) -> *const c_char;

    pub fn ssh_userauth_kbdint_getnanswers(session: ssh_session) -> c_int;

    pub fn ssh_userauth_kbdint_getanswer(
        session: ssh_session,
        i: c_uint,
    ) -> *const c_char;

    pub fn ssh_userauth_kbdint_setanswer(
        session: ssh_session,
        i: c_uint,
        answer: *const c_char,
    ) -> c_int;

    pub fn ssh_userauth_gssapi(session: ssh_session) -> c_int;

    pub fn ssh_version(req_version: c_int) -> *const c_char;

    pub fn ssh_string_burn(str_: ssh_string);

    pub fn ssh_string_copy(str_: ssh_string) -> ssh_string;

    pub fn ssh_string_data(str_: ssh_string) -> *mut c_void;

    pub fn ssh_string_fill(
        str_: ssh_string,
        data: *const c_void,
        len: size_t,
    ) -> c_int;

    pub fn ssh_string_free(str_: ssh_string);

    pub fn ssh_string_from_char(what: *const c_char) -> ssh_string;

    pub fn ssh_string_len(str_: ssh_string) -> size_t;

    pub fn ssh_string_new(size: size_t) -> ssh_string;

    pub fn ssh_string_get_char(str_: ssh_string) -> *const c_char;

    pub fn ssh_string_to_char(str_: ssh_string) -> *mut c_char;

    pub fn ssh_string_free_char(s: *mut c_char);

    pub fn ssh_getpass(
        prompt: *const c_char,
        buf: *mut c_char,
        len: size_t,
        echo: c_int,
        verify: c_int,
    ) -> c_int;

    pub fn ssh_event_new() -> ssh_event;

    pub fn ssh_event_add_fd(
        event: ssh_event,
        fd: socket_t,
        events: c_short,
        cb: ssh_event_callback,
        userdata: *mut c_void,
    ) -> c_int;

    pub fn ssh_event_add_session(event: ssh_event, session: ssh_session) -> c_int;

    pub fn ssh_event_add_connector(
        event: ssh_event,
        connector: ssh_connector,
    ) -> c_int;

    pub fn ssh_event_dopoll(
        event: ssh_event,
        timeout: c_int,
    ) -> c_int;

    pub fn ssh_event_remove_fd(event: ssh_event, fd: socket_t) -> c_int;

    pub fn ssh_event_remove_session(
        event: ssh_event,
        session: ssh_session,
    ) -> c_int;

    pub fn ssh_event_remove_connector(
        event: ssh_event,
        connector: ssh_connector,
    ) -> c_int;

    pub fn ssh_event_free(event: ssh_event);

    pub fn ssh_get_clientbanner(session: ssh_session) -> *const c_char;

    pub fn ssh_get_serverbanner(session: ssh_session) -> *const c_char;

    pub fn ssh_get_kex_algo(session: ssh_session) -> *const c_char;

    pub fn ssh_get_cipher_in(session: ssh_session) -> *const c_char;

    pub fn ssh_get_cipher_out(session: ssh_session) -> *const c_char;

    pub fn ssh_get_hmac_in(session: ssh_session) -> *const c_char;

    pub fn ssh_get_hmac_out(session: ssh_session) -> *const c_char;

    pub fn ssh_buffer_new() -> ssh_buffer;

    pub fn ssh_buffer_free(buffer: ssh_buffer);

    pub fn ssh_buffer_reinit(buffer: ssh_buffer) -> c_int;

    pub fn ssh_buffer_add_data(
        buffer: ssh_buffer,
        data: *const c_void,
        len: u32,
    ) -> c_int;

    pub fn ssh_buffer_get_data(
        buffer: ssh_buffer,
        data: *mut c_void,
        requestedlen: u32,
    ) -> u32;

    pub fn ssh_buffer_get(buffer: ssh_buffer) -> *mut c_void;

    pub fn ssh_buffer_get_len(buffer: ssh_buffer) -> u32;

    pub fn ssh_auth_list(session: ssh_session) -> c_int;

    pub fn ssh_userauth_offer_pubkey(
        session: ssh_session,
        username: *const c_char,
        type_: c_int,
        publickey: ssh_string,
    ) -> c_int;

    pub fn ssh_userauth_pubkey(
        session: ssh_session,
        username: *const c_char,
        publickey: ssh_string,
        privatekey: ssh_private_key,
    ) -> c_int;

    pub fn ssh_userauth_agent_pubkey(
        session: ssh_session,
        username: *const c_char,
        publickey: ssh_public_key,
    ) -> c_int;

    pub fn ssh_userauth_autopubkey(
        session: ssh_session,
        passphrase: *const c_char,
    ) -> c_int;

    pub fn ssh_userauth_privatekey_file(
        session: ssh_session,
        username: *const c_char,
        filename: *const c_char,
        passphrase: *const c_char,
    ) -> c_int;
}
pub type ssh_event_callback = ::std::option::Option<
    unsafe extern "C" fn(
        fd: socket_t,
        revents: c_int,
        userdata: *mut c_void,
    ) -> c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_private_key_struct {
    _unused: [u8; 0],
}
pub type ssh_private_key = *mut ssh_private_key_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_public_key_struct {
    _unused: [u8; 0],
}
pub type ssh_public_key = *mut ssh_public_key_struct;
#[doc = " @brief SSH authentication callback for password and publickey auth."]
#[doc = ""]
#[doc = " @param prompt        Prompt to be displayed."]
#[doc = " @param buf           Buffer to save the password. You should null-terminate it."]
#[doc = " @param len           Length of the buffer."]
#[doc = " @param echo          Enable or disable the echo of what you type."]
#[doc = " @param verify        Should the password be verified?"]
#[doc = " @param userdata      Userdata to be passed to the callback function. Useful"]
#[doc = "                      for GUI applications."]
#[doc = ""]
#[doc = " @return              0 on success, < 0 on error."]
pub type ssh_auth_callback = ::std::option::Option<
    unsafe extern "C" fn(
        prompt: *const c_char,
        buf: *mut c_char,
        len: size_t,
        echo: c_int,
        verify: c_int,
        userdata: *mut c_void,
    ) -> c_int,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_string_struct {
    _unused: [u8; 0],
}
pub type ssh_string = *mut ssh_string_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_event_struct {
    _unused: [u8; 0],
}
pub type ssh_event = *mut ssh_event_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_connector_struct {
    _unused: [u8; 0],
}
pub type ssh_connector = *mut ssh_connector_struct;
pub type ssh_gssapi_creds = *mut c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_counter_struct {
    pub in_bytes: u64,
    pub out_bytes: u64,
    pub in_packets: u64,
    pub out_packets: u64,
}
pub type ssh_counter = *mut ssh_counter_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_agent_struct {
    _unused: [u8; 0],
}
pub type ssh_agent = *mut ssh_agent_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_buffer_struct {
    _unused: [u8; 0],
}
pub type ssh_buffer = *mut ssh_buffer_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_channel_struct {
    _unused: [u8; 0],
}
pub type ssh_channel = *mut ssh_channel_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_message_struct {
    _unused: [u8; 0],
}
pub type ssh_message = *mut ssh_message_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ssh_pcap_file_struct {
    _unused: [u8; 0],
}
pub type ssh_pcap_file = *mut ssh_pcap_file_struct;
pub type ssh_scp = *mut ssh_scp_struct;
pub type socket_t = c_int;
pub type __fd_mask = c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
pub type mode_t = c_uint;
pub type ssh_connector_flags_e = c_uint;
#[doc = " Only the standard stream of the channel"]
pub const ssh_connector_flags_e_SSH_CONNECTOR_STDOUT: ssh_connector_flags_e = 1;
#[doc = " Only the standard stream of the channel"]
pub const ssh_connector_flags_e_SSH_CONNECTOR_STDINOUT: ssh_connector_flags_e = 1;
#[doc = " Only the exception stream of the channel"]
pub const ssh_connector_flags_e_SSH_CONNECTOR_STDERR: ssh_connector_flags_e = 2;
#[doc = " Merge both standard and exception streams"]
pub const ssh_connector_flags_e_SSH_CONNECTOR_BOTH: ssh_connector_flags_e = 3;
#[doc = " Code is going to write/create remote files"]
pub const SSH_SCP_WRITE: c_uint = 0;
#[doc = " Code is going to read remote files"]
pub const SSH_SCP_READ: c_uint = 1;
#[doc = " Code is going to read remote files"]
pub const SSH_SCP_RECURSIVE: c_uint = 16;

pub type ssh_scp_request_types = c_uint;
#[doc = " A new directory is going to be pulled"]
pub const SSH_SCP_REQUEST_NEWDIR: ssh_scp_request_types = 1;
#[doc = " A new file is going to be pulled"]
pub const SSH_SCP_REQUEST_NEWFILE: ssh_scp_request_types = 2;
#[doc = " End of requests"]
pub const SSH_SCP_REQUEST_EOF: ssh_scp_request_types = 3;
#[doc = " End of directory"]
pub const SSH_SCP_REQUEST_ENDDIR: ssh_scp_request_types = 4;
#[doc = " Warning received"]
pub const SSH_SCP_REQUEST_WARNING: ssh_scp_request_types = 5;
