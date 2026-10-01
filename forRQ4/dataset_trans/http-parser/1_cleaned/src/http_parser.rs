use core::ffi::*;
pub use crate::src::ffi::*;
use ::c2rust_bitfields;
extern "C" {
    fn memset(
        __s: *mut c_void,
        __c: c_int,
        __n: size_t,
    ) -> *mut c_void;
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "c_uint", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "c_uint", bits = "2..=9")]
    #[bitfield(name = "state", ty = "c_uint", bits = "10..=16")]
    #[bitfield(name = "header_state", ty = "c_uint", bits = "17..=23")]
    #[bitfield(name = "index", ty = "c_uint", bits = "24..=28")]
    #[bitfield(
        name = "uses_transfer_encoding",
        ty = "c_uint",
        bits = "29..=29"
    )]
    #[bitfield(
        name = "allow_chunked_length",
        ty = "c_uint",
        bits = "30..=30"
    )]
    #[bitfield(
        name = "lenient_http_headers",
        ty = "c_uint",
        bits = "31..=31"
    )]
    pub type_0_flags_state_header_state_index_uses_transfer_encoding_allow_chunked_length_lenient_http_headers:
        [u8; 4],
    pub nread: uint32_t,
    pub content_length: uint64_t,
    pub http_major: c_ushort,
    pub http_minor: c_ushort,
    #[bitfield(name = "status_code", ty = "c_uint", bits = "0..=15")]
    #[bitfield(name = "method", ty = "c_uint", bits = "16..=23")]
    #[bitfield(name = "http_errno", ty = "c_uint", bits = "24..=30")]
    #[bitfield(name = "upgrade", ty = "c_uint", bits = "31..=31")]
    pub status_code_method_http_errno_upgrade: [u8; 4],
    pub data: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_settings {
    pub on_message_begin: http_cb,
    pub on_url: http_data_cb,
    pub on_status: http_data_cb,
    pub on_header_field: http_data_cb,
    pub on_header_value: http_data_cb,
    pub on_headers_complete: http_cb,
    pub on_body: http_data_cb,
    pub on_message_complete: http_cb,
    pub on_chunk_header: http_cb,
    pub on_chunk_complete: http_cb,
}
pub type http_cb = Option<unsafe extern "C" fn(*mut http_parser) -> c_int>;
pub type http_data_cb = Option<
    unsafe extern "C" fn(
        *mut http_parser,
        *const c_char,
        size_t,
    ) -> c_int,
>;
pub type http_status = c_uint;
pub const HTTP_STATUS_NETWORK_AUTHENTICATION_REQUIRED: http_status = 511;
pub const HTTP_STATUS_NOT_EXTENDED: http_status = 510;
pub const HTTP_STATUS_LOOP_DETECTED: http_status = 508;
pub const HTTP_STATUS_INSUFFICIENT_STORAGE: http_status = 507;
pub const HTTP_STATUS_VARIANT_ALSO_NEGOTIATES: http_status = 506;
pub const HTTP_STATUS_HTTP_VERSION_NOT_SUPPORTED: http_status = 505;
pub const HTTP_STATUS_GATEWAY_TIMEOUT: http_status = 504;
pub const HTTP_STATUS_SERVICE_UNAVAILABLE: http_status = 503;
pub const HTTP_STATUS_BAD_GATEWAY: http_status = 502;
pub const HTTP_STATUS_NOT_IMPLEMENTED: http_status = 501;
pub const HTTP_STATUS_INTERNAL_SERVER_ERROR: http_status = 500;
pub const HTTP_STATUS_UNAVAILABLE_FOR_LEGAL_REASONS: http_status = 451;
pub const HTTP_STATUS_REQUEST_HEADER_FIELDS_TOO_LARGE: http_status = 431;
pub const HTTP_STATUS_TOO_MANY_REQUESTS: http_status = 429;
pub const HTTP_STATUS_PRECONDITION_REQUIRED: http_status = 428;
pub const HTTP_STATUS_UPGRADE_REQUIRED: http_status = 426;
pub const HTTP_STATUS_FAILED_DEPENDENCY: http_status = 424;
pub const HTTP_STATUS_LOCKED: http_status = 423;
pub const HTTP_STATUS_UNPROCESSABLE_ENTITY: http_status = 422;
pub const HTTP_STATUS_MISDIRECTED_REQUEST: http_status = 421;
pub const HTTP_STATUS_EXPECTATION_FAILED: http_status = 417;
pub const HTTP_STATUS_RANGE_NOT_SATISFIABLE: http_status = 416;
pub const HTTP_STATUS_UNSUPPORTED_MEDIA_TYPE: http_status = 415;
pub const HTTP_STATUS_URI_TOO_LONG: http_status = 414;
pub const HTTP_STATUS_PAYLOAD_TOO_LARGE: http_status = 413;
pub const HTTP_STATUS_PRECONDITION_FAILED: http_status = 412;
pub const HTTP_STATUS_LENGTH_REQUIRED: http_status = 411;
pub const HTTP_STATUS_GONE: http_status = 410;
pub const HTTP_STATUS_CONFLICT: http_status = 409;
pub const HTTP_STATUS_REQUEST_TIMEOUT: http_status = 408;
pub const HTTP_STATUS_PROXY_AUTHENTICATION_REQUIRED: http_status = 407;
pub const HTTP_STATUS_NOT_ACCEPTABLE: http_status = 406;
pub const HTTP_STATUS_METHOD_NOT_ALLOWED: http_status = 405;
pub const HTTP_STATUS_NOT_FOUND: http_status = 404;
pub const HTTP_STATUS_FORBIDDEN: http_status = 403;
pub const HTTP_STATUS_PAYMENT_REQUIRED: http_status = 402;
pub const HTTP_STATUS_UNAUTHORIZED: http_status = 401;
pub const HTTP_STATUS_BAD_REQUEST: http_status = 400;
pub const HTTP_STATUS_PERMANENT_REDIRECT: http_status = 308;
pub const HTTP_STATUS_TEMPORARY_REDIRECT: http_status = 307;
pub const HTTP_STATUS_USE_PROXY: http_status = 305;
pub const HTTP_STATUS_NOT_MODIFIED: http_status = 304;
pub const HTTP_STATUS_SEE_OTHER: http_status = 303;
pub const HTTP_STATUS_FOUND: http_status = 302;
pub const HTTP_STATUS_MOVED_PERMANENTLY: http_status = 301;
pub const HTTP_STATUS_MULTIPLE_CHOICES: http_status = 300;
pub const HTTP_STATUS_IM_USED: http_status = 226;
pub const HTTP_STATUS_ALREADY_REPORTED: http_status = 208;
pub const HTTP_STATUS_MULTI_STATUS: http_status = 207;
pub const HTTP_STATUS_PARTIAL_CONTENT: http_status = 206;
pub const HTTP_STATUS_RESET_CONTENT: http_status = 205;
pub const HTTP_STATUS_NO_CONTENT: http_status = 204;
pub const HTTP_STATUS_NON_AUTHORITATIVE_INFORMATION: http_status = 203;
pub const HTTP_STATUS_ACCEPTED: http_status = 202;
pub const HTTP_STATUS_CREATED: http_status = 201;
pub const HTTP_STATUS_OK: http_status = 200;
pub const HTTP_STATUS_PROCESSING: http_status = 102;
pub const HTTP_STATUS_SWITCHING_PROTOCOLS: http_status = 101;
pub const HTTP_STATUS_CONTINUE: http_status = 100;
pub type http_method = c_uint;
pub const HTTP_SOURCE: http_method = 33;
pub const HTTP_UNLINK: http_method = 32;
pub const HTTP_LINK: http_method = 31;
pub const HTTP_MKCALENDAR: http_method = 30;
pub const HTTP_PURGE: http_method = 29;
pub const HTTP_PATCH: http_method = 28;
pub const HTTP_UNSUBSCRIBE: http_method = 27;
pub const HTTP_SUBSCRIBE: http_method = 26;
pub const HTTP_NOTIFY: http_method = 25;
pub const HTTP_MSEARCH: http_method = 24;
pub const HTTP_MERGE: http_method = 23;
pub const HTTP_CHECKOUT: http_method = 22;
pub const HTTP_MKACTIVITY: http_method = 21;
pub const HTTP_REPORT: http_method = 20;
pub const HTTP_ACL: http_method = 19;
pub const HTTP_UNBIND: http_method = 18;
pub const HTTP_REBIND: http_method = 17;
pub const HTTP_BIND: http_method = 16;
pub const HTTP_UNLOCK: http_method = 15;
pub const HTTP_SEARCH: http_method = 14;
pub const HTTP_PROPPATCH: http_method = 13;
pub const HTTP_PROPFIND: http_method = 12;
pub const HTTP_MOVE: http_method = 11;
pub const HTTP_MKCOL: http_method = 10;
pub const HTTP_LOCK: http_method = 9;
pub const HTTP_COPY: http_method = 8;
pub const HTTP_TRACE: http_method = 7;
pub const HTTP_OPTIONS: http_method = 6;
pub const HTTP_CONNECT: http_method = 5;
pub const HTTP_PUT: http_method = 4;
pub const HTTP_POST: http_method = 3;
pub const HTTP_HEAD: http_method = 2;
pub const HTTP_GET: http_method = 1;
pub const HTTP_DELETE: http_method = 0;
pub type http_parser_type = c_uint;
pub const HTTP_BOTH: http_parser_type = 2;
pub const HTTP_RESPONSE: http_parser_type = 1;
pub const HTTP_REQUEST: http_parser_type = 0;
pub type flags = c_uint;
pub const F_CONTENTLENGTH: flags = 128;
pub const F_SKIPBODY: flags = 64;
pub const F_UPGRADE: flags = 32;
pub const F_TRAILING: flags = 16;
pub const F_CONNECTION_UPGRADE: flags = 8;
pub const F_CONNECTION_CLOSE: flags = 4;
pub const F_CONNECTION_KEEP_ALIVE: flags = 2;
pub const F_CHUNKED: flags = 1;
pub type http_errno = c_uint;
pub const HPE_INVALID_TRANSFER_ENCODING: http_errno = 33;
pub const HPE_UNKNOWN: http_errno = 32;
pub const HPE_PAUSED: http_errno = 31;
pub const HPE_STRICT: http_errno = 30;
pub const HPE_INVALID_INTERNAL_STATE: http_errno = 29;
pub const HPE_INVALID_CONSTANT: http_errno = 28;
pub const HPE_INVALID_CHUNK_SIZE: http_errno = 27;
pub const HPE_UNEXPECTED_CONTENT_LENGTH: http_errno = 26;
pub const HPE_INVALID_CONTENT_LENGTH: http_errno = 25;
pub const HPE_INVALID_HEADER_TOKEN: http_errno = 24;
pub const HPE_LF_EXPECTED: http_errno = 23;
pub const HPE_INVALID_FRAGMENT: http_errno = 22;
pub const HPE_INVALID_QUERY_STRING: http_errno = 21;
pub const HPE_INVALID_PATH: http_errno = 20;
pub const HPE_INVALID_PORT: http_errno = 19;
pub const HPE_INVALID_HOST: http_errno = 18;
pub const HPE_INVALID_URL: http_errno = 17;
pub const HPE_INVALID_METHOD: http_errno = 16;
pub const HPE_INVALID_STATUS: http_errno = 15;
pub const HPE_INVALID_VERSION: http_errno = 14;
pub const HPE_CLOSED_CONNECTION: http_errno = 13;
pub const HPE_HEADER_OVERFLOW: http_errno = 12;
pub const HPE_INVALID_EOF_STATE: http_errno = 11;
pub const HPE_CB_chunk_complete: http_errno = 10;
pub const HPE_CB_chunk_header: http_errno = 9;
pub const HPE_CB_status: http_errno = 8;
pub const HPE_CB_message_complete: http_errno = 7;
pub const HPE_CB_body: http_errno = 6;
pub const HPE_CB_headers_complete: http_errno = 5;
pub const HPE_CB_header_value: http_errno = 4;
pub const HPE_CB_header_field: http_errno = 3;
pub const HPE_CB_url: http_errno = 2;
pub const HPE_CB_message_begin: http_errno = 1;
pub const HPE_OK: http_errno = 0;
pub type http_parser_url_fields = c_uint;
pub const UF_MAX: http_parser_url_fields = 7;
pub const UF_USERINFO: http_parser_url_fields = 6;
pub const UF_FRAGMENT: http_parser_url_fields = 5;
pub const UF_QUERY: http_parser_url_fields = 4;
pub const UF_PATH: http_parser_url_fields = 3;
pub const UF_PORT: http_parser_url_fields = 2;
pub const UF_HOST: http_parser_url_fields = 1;
pub const UF_SCHEMA: http_parser_url_fields = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_url {
    pub field_set: uint16_t,
    pub port: uint16_t,
    pub field_data: [C2RustUnnamed; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub off: uint16_t,
    pub len: uint16_t,
}
pub const s_start_req_or_res: state = 2;
pub const s_start_res: state = 4;
pub const s_start_req: state = 18;
pub type state = c_uint;
pub const s_message_done: state = 64;
pub const s_body_identity_eof: state = 63;
pub const s_body_identity: state = 62;
pub const s_chunk_data_done: state = 61;
pub const s_chunk_data_almost_done: state = 60;
pub const s_chunk_data: state = 59;
pub const s_headers_done: state = 58;
pub const s_headers_almost_done: state = 57;
pub const s_chunk_size_almost_done: state = 56;
pub const s_chunk_parameters: state = 55;
pub const s_chunk_size: state = 54;
pub const s_chunk_size_start: state = 53;
pub const s_header_almost_done: state = 52;
pub const s_header_value_lws: state = 51;
pub const s_header_value: state = 50;
pub const s_header_value_start: state = 49;
pub const s_header_value_discard_lws: state = 48;
pub const s_header_value_discard_ws_almost_done: state = 47;
pub const s_header_value_discard_ws: state = 46;
pub const s_header_field: state = 45;
pub const s_header_field_start: state = 44;
pub const s_req_line_almost_done: state = 43;
pub const s_req_http_end: state = 42;
pub const s_req_http_minor: state = 41;
pub const s_req_http_dot: state = 40;
pub const s_req_http_major: state = 39;
pub const s_req_http_IC: state = 38;
pub const s_req_http_I: state = 37;
pub const s_req_http_HTTP: state = 36;
pub const s_req_http_HTT: state = 35;
pub const s_req_http_HT: state = 34;
pub const s_req_http_H: state = 33;
pub const s_req_http_start: state = 32;
pub const s_req_fragment: state = 31;
pub const s_req_fragment_start: state = 30;
pub const s_req_query_string: state = 29;
pub const s_req_query_string_start: state = 28;
pub const s_req_path: state = 27;
pub const s_req_server_with_at: state = 26;
pub const s_req_server: state = 25;
pub const s_req_server_start: state = 24;
pub const s_req_schema_slash_slash: state = 23;
pub const s_req_schema_slash: state = 22;
pub const s_req_schema: state = 21;
pub const s_req_spaces_before_url: state = 20;
pub const s_req_method: state = 19;
pub const s_res_line_almost_done: state = 17;
pub const s_res_status: state = 16;
pub const s_res_status_start: state = 15;
pub const s_res_status_code: state = 14;
pub const s_res_first_status_code: state = 13;
pub const s_res_http_end: state = 12;
pub const s_res_http_minor: state = 11;
pub const s_res_http_dot: state = 10;
pub const s_res_http_major: state = 9;
pub const s_res_HTTP: state = 8;
pub const s_res_HTT: state = 7;
pub const s_res_HT: state = 6;
pub const s_res_H: state = 5;
pub const s_res_or_resp_H: state = 3;
pub const s_dead: state = 1;
pub const h_content_length: header_states = 10;
pub const h_transfer_encoding_chunked: header_states = 23;
pub const h_connection_upgrade: header_states = 26;
pub const h_connection_close: header_states = 25;
pub const h_connection_keep_alive: header_states = 24;
pub const h_content_length_ws: header_states = 12;
pub const h_content_length_num: header_states = 11;
pub type header_states = c_uint;
pub const h_matching_connection_token: header_states = 22;
pub const h_matching_connection_upgrade: header_states = 21;
pub const h_matching_connection_close: header_states = 20;
pub const h_matching_connection_keep_alive: header_states = 19;
pub const h_matching_connection_token_start: header_states = 18;
pub const h_matching_transfer_encoding_token: header_states = 17;
pub const h_matching_transfer_encoding_chunked: header_states = 16;
pub const h_matching_transfer_encoding_token_start: header_states = 15;
pub const h_upgrade: header_states = 14;
pub const h_transfer_encoding: header_states = 13;
pub const h_connection: header_states = 9;
pub const h_matching_upgrade: header_states = 8;
pub const h_matching_transfer_encoding: header_states = 7;
pub const h_matching_content_length: header_states = 6;
pub const h_matching_proxy_connection: header_states = 5;
pub const h_matching_connection: header_states = 4;
pub const h_CON: header_states = 3;
pub const h_CO: header_states = 2;
pub const h_C: header_states = 1;
pub const h_general: header_states = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub name: *const c_char,
    pub description: *const c_char,
}
pub const s_http_userinfo_start: http_host_state = 2;
pub const s_http_userinfo: http_host_state = 3;
pub const s_http_host_port_start: http_host_state = 11;
pub const s_http_host_v6_zone: http_host_state = 10;
pub const s_http_host_v6_zone_start: http_host_state = 9;
pub const s_http_host_v6: http_host_state = 7;
pub const s_http_host_v6_start: http_host_state = 5;
pub const s_http_host_start: http_host_state = 4;
pub type http_host_state = c_uint;
pub const s_http_host_port: http_host_state = 12;
pub const s_http_host_v6_end: http_host_state = 8;
pub const s_http_host: http_host_state = 6;
pub const s_http_host_dead: http_host_state = 1;
pub const HTTP_PARSER_VERSION_MAJOR: c_int = 2 as c_int;
pub const HTTP_PARSER_VERSION_MINOR: c_int = 9 as c_int;
pub const HTTP_PARSER_VERSION_PATCH: c_int = 4 as c_int;
pub const NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const HTTP_MAX_HEADER_SIZE: c_int =
    80 as c_int * 1024 as c_int;
static mut max_header_size: uint32_t = HTTP_MAX_HEADER_SIZE as uint32_t;
pub const PROXY_CONNECTION: [c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [c_char; 17]>(*b"proxy-connection\0")
};
pub const CONNECTION: [c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [c_char; 11]>(*b"connection\0") };
pub const CONTENT_LENGTH: [c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"content-length\0") };
pub const TRANSFER_ENCODING: [c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [c_char; 18]>(*b"transfer-encoding\0")
};
pub const UPGRADE: [c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [c_char; 8]>(*b"upgrade\0") };
pub const CHUNKED: [c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [c_char; 8]>(*b"chunked\0") };
pub const KEEP_ALIVE: [c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [c_char; 11]>(*b"keep-alive\0") };
pub const CLOSE: [c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [c_char; 6]>(*b"close\0") };
static mut method_strings: [*const c_char; 34] = [
    b"DELETE\0" as *const u8 as *const c_char,
    b"GET\0" as *const u8 as *const c_char,
    b"HEAD\0" as *const u8 as *const c_char,
    b"POST\0" as *const u8 as *const c_char,
    b"PUT\0" as *const u8 as *const c_char,
    b"CONNECT\0" as *const u8 as *const c_char,
    b"OPTIONS\0" as *const u8 as *const c_char,
    b"TRACE\0" as *const u8 as *const c_char,
    b"COPY\0" as *const u8 as *const c_char,
    b"LOCK\0" as *const u8 as *const c_char,
    b"MKCOL\0" as *const u8 as *const c_char,
    b"MOVE\0" as *const u8 as *const c_char,
    b"PROPFIND\0" as *const u8 as *const c_char,
    b"PROPPATCH\0" as *const u8 as *const c_char,
    b"SEARCH\0" as *const u8 as *const c_char,
    b"UNLOCK\0" as *const u8 as *const c_char,
    b"BIND\0" as *const u8 as *const c_char,
    b"REBIND\0" as *const u8 as *const c_char,
    b"UNBIND\0" as *const u8 as *const c_char,
    b"ACL\0" as *const u8 as *const c_char,
    b"REPORT\0" as *const u8 as *const c_char,
    b"MKACTIVITY\0" as *const u8 as *const c_char,
    b"CHECKOUT\0" as *const u8 as *const c_char,
    b"MERGE\0" as *const u8 as *const c_char,
    b"M-SEARCH\0" as *const u8 as *const c_char,
    b"NOTIFY\0" as *const u8 as *const c_char,
    b"SUBSCRIBE\0" as *const u8 as *const c_char,
    b"UNSUBSCRIBE\0" as *const u8 as *const c_char,
    b"PATCH\0" as *const u8 as *const c_char,
    b"PURGE\0" as *const u8 as *const c_char,
    b"MKCALENDAR\0" as *const u8 as *const c_char,
    b"LINK\0" as *const u8 as *const c_char,
    b"UNLINK\0" as *const u8 as *const c_char,
    b"SOURCE\0" as *const u8 as *const c_char,
];
static mut tokens: [c_char; 256] = [
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    ' ' as i32 as c_char,
    '!' as i32 as c_char,
    0 as c_int as c_char,
    '#' as i32 as c_char,
    '$' as i32 as c_char,
    '%' as i32 as c_char,
    '&' as i32 as c_char,
    '\'' as i32 as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    '*' as i32 as c_char,
    '+' as i32 as c_char,
    0 as c_int as c_char,
    '-' as i32 as c_char,
    '.' as i32 as c_char,
    0 as c_int as c_char,
    '0' as i32 as c_char,
    '1' as i32 as c_char,
    '2' as i32 as c_char,
    '3' as i32 as c_char,
    '4' as i32 as c_char,
    '5' as i32 as c_char,
    '6' as i32 as c_char,
    '7' as i32 as c_char,
    '8' as i32 as c_char,
    '9' as i32 as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    'a' as i32 as c_char,
    'b' as i32 as c_char,
    'c' as i32 as c_char,
    'd' as i32 as c_char,
    'e' as i32 as c_char,
    'f' as i32 as c_char,
    'g' as i32 as c_char,
    'h' as i32 as c_char,
    'i' as i32 as c_char,
    'j' as i32 as c_char,
    'k' as i32 as c_char,
    'l' as i32 as c_char,
    'm' as i32 as c_char,
    'n' as i32 as c_char,
    'o' as i32 as c_char,
    'p' as i32 as c_char,
    'q' as i32 as c_char,
    'r' as i32 as c_char,
    's' as i32 as c_char,
    't' as i32 as c_char,
    'u' as i32 as c_char,
    'v' as i32 as c_char,
    'w' as i32 as c_char,
    'x' as i32 as c_char,
    'y' as i32 as c_char,
    'z' as i32 as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    0 as c_int as c_char,
    '^' as i32 as c_char,
    '_' as i32 as c_char,
    '`' as i32 as c_char,
    'a' as i32 as c_char,
    'b' as i32 as c_char,
    'c' as i32 as c_char,
    'd' as i32 as c_char,
    'e' as i32 as c_char,
    'f' as i32 as c_char,
    'g' as i32 as c_char,
    'h' as i32 as c_char,
    'i' as i32 as c_char,
    'j' as i32 as c_char,
    'k' as i32 as c_char,
    'l' as i32 as c_char,
    'm' as i32 as c_char,
    'n' as i32 as c_char,
    'o' as i32 as c_char,
    'p' as i32 as c_char,
    'q' as i32 as c_char,
    'r' as i32 as c_char,
    's' as i32 as c_char,
    't' as i32 as c_char,
    'u' as i32 as c_char,
    'v' as i32 as c_char,
    'w' as i32 as c_char,
    'x' as i32 as c_char,
    'y' as i32 as c_char,
    'z' as i32 as c_char,
    0 as c_int as c_char,
    '|' as i32 as c_char,
    0 as c_int as c_char,
    '~' as i32 as c_char,
    0 as c_int as c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut unhex: [int8_t; 256] = [
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    0 as c_int as int8_t,
    1 as c_int as int8_t,
    2 as c_int as int8_t,
    3 as c_int as int8_t,
    4 as c_int as int8_t,
    5 as c_int as int8_t,
    6 as c_int as int8_t,
    7 as c_int as int8_t,
    8 as c_int as int8_t,
    9 as c_int as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    10 as c_int as int8_t,
    11 as c_int as int8_t,
    12 as c_int as int8_t,
    13 as c_int as int8_t,
    14 as c_int as int8_t,
    15 as c_int as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    10 as c_int as int8_t,
    11 as c_int as int8_t,
    12 as c_int as int8_t,
    13 as c_int as int8_t,
    14 as c_int as int8_t,
    15 as c_int as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    -(1 as c_int) as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut normal_url_char: [uint8_t; 32] = [
    (0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int) as uint8_t,
    (0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int) as uint8_t,
    (0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int) as uint8_t,
    (0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int
        | 0 as c_int) as uint8_t,
    (0 as c_int
        | 2 as c_int
        | 4 as c_int
        | 0 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 0 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 128 as c_int) as uint8_t,
    (1 as c_int
        | 2 as c_int
        | 4 as c_int
        | 8 as c_int
        | 16 as c_int
        | 32 as c_int
        | 64 as c_int
        | 0 as c_int) as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub const CR: c_int = '\r' as i32;
pub const LF: c_int = '\n' as i32;
static mut http_strerror_tab: [C2RustUnnamed_0; 34] = [
    C2RustUnnamed_0 {
        name: b"HPE_OK\0" as *const u8 as *const c_char,
        description: b"success\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_message_begin\0" as *const u8 as *const c_char,
        description: b"the on_message_begin callback failed\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_url\0" as *const u8 as *const c_char,
        description: b"the on_url callback failed\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_header_field\0" as *const u8 as *const c_char,
        description: b"the on_header_field callback failed\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_header_value\0" as *const u8 as *const c_char,
        description: b"the on_header_value callback failed\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_headers_complete\0" as *const u8 as *const c_char,
        description: b"the on_headers_complete callback failed\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_body\0" as *const u8 as *const c_char,
        description: b"the on_body callback failed\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_message_complete\0" as *const u8 as *const c_char,
        description: b"the on_message_complete callback failed\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_status\0" as *const u8 as *const c_char,
        description: b"the on_status callback failed\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_chunk_header\0" as *const u8 as *const c_char,
        description: b"the on_chunk_header callback failed\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CB_chunk_complete\0" as *const u8 as *const c_char,
        description: b"the on_chunk_complete callback failed\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_EOF_STATE\0" as *const u8 as *const c_char,
        description: b"stream ended at an unexpected time\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_HEADER_OVERFLOW\0" as *const u8 as *const c_char,
        description: b"too many header bytes seen; overflow detected\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_CLOSED_CONNECTION\0" as *const u8 as *const c_char,
        description: b"data received after completed connection: close message\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_VERSION\0" as *const u8 as *const c_char,
        description: b"invalid HTTP version\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_STATUS\0" as *const u8 as *const c_char,
        description: b"invalid HTTP status code\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_METHOD\0" as *const u8 as *const c_char,
        description: b"invalid HTTP method\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_URL\0" as *const u8 as *const c_char,
        description: b"invalid URL\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_HOST\0" as *const u8 as *const c_char,
        description: b"invalid host\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_PORT\0" as *const u8 as *const c_char,
        description: b"invalid port\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_PATH\0" as *const u8 as *const c_char,
        description: b"invalid path\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_QUERY_STRING\0" as *const u8 as *const c_char,
        description: b"invalid query string\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_FRAGMENT\0" as *const u8 as *const c_char,
        description: b"invalid fragment\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_LF_EXPECTED\0" as *const u8 as *const c_char,
        description: b"LF character expected\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_HEADER_TOKEN\0" as *const u8 as *const c_char,
        description: b"invalid character in header\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_CONTENT_LENGTH\0" as *const u8 as *const c_char,
        description: b"invalid character in content-length header\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_UNEXPECTED_CONTENT_LENGTH\0" as *const u8 as *const c_char,
        description: b"unexpected content-length header\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_CHUNK_SIZE\0" as *const u8 as *const c_char,
        description: b"invalid character in chunk size header\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_CONSTANT\0" as *const u8 as *const c_char,
        description: b"invalid constant string\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_INTERNAL_STATE\0" as *const u8 as *const c_char,
        description: b"encountered unexpected internal state\0" as *const u8
            as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_STRICT\0" as *const u8 as *const c_char,
        description: b"strict mode assertion failed\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_PAUSED\0" as *const u8 as *const c_char,
        description: b"parser is paused\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_UNKNOWN\0" as *const u8 as *const c_char,
        description: b"an unknown error occurred\0" as *const u8 as *const c_char,
    },
    C2RustUnnamed_0 {
        name: b"HPE_INVALID_TRANSFER_ENCODING\0" as *const u8 as *const c_char,
        description: b"request has invalid transfer-encoding\0" as *const u8
            as *const c_char,
    },
];
unsafe extern "C" fn parse_url_char(mut s: state, ch: c_char) -> state {
    if ch as c_int == ' ' as i32
        || ch as c_int == '\r' as i32
        || ch as c_int == '\n' as i32
    {
        return s_dead;
    }
    if ch as c_int == '\t' as i32 || ch as c_int == '\u{c}' as i32 {
        return s_dead;
    }
    let mut current_block_61: u64;
    match s as c_uint {
        20 => {
            if ch as c_int == '/' as i32 || ch as c_int == '*' as i32 {
                return s_req_path;
            }
            if (ch as c_int | 0x20 as c_int) as c_uchar
                as c_int
                >= 'a' as i32
                && (ch as c_int | 0x20 as c_int) as c_uchar
                    as c_int
                    <= 'z' as i32
            {
                return s_req_schema;
            }
            current_block_61 = 7018308795614528254;
        }
        21 => {
            if (ch as c_int | 0x20 as c_int) as c_uchar
                as c_int
                >= 'a' as i32
                && (ch as c_int | 0x20 as c_int) as c_uchar
                    as c_int
                    <= 'z' as i32
            {
                return s;
            }
            if ch as c_int == ':' as i32 {
                return s_req_schema_slash;
            }
            current_block_61 = 7018308795614528254;
        }
        22 => {
            if ch as c_int == '/' as i32 {
                return s_req_schema_slash_slash;
            }
            current_block_61 = 7018308795614528254;
        }
        23 => {
            if ch as c_int == '/' as i32 {
                return s_req_server_start;
            }
            current_block_61 = 7018308795614528254;
        }
        26 => {
            if ch as c_int == '@' as i32 {
                return s_dead;
            }
            current_block_61 = 13245741422963388543;
        }
        24 | 25 => {
            current_block_61 = 13245741422963388543;
        }
        27 => {
            if normal_url_char[(ch as c_uchar as c_uint
                >> 3 as c_int) as usize] as c_uint
                & ((1 as c_int)
                    << (ch as c_uchar as c_uint
                        & 7 as c_uint)) as c_uint
                != 0
            {
                return s;
            }
            match ch as c_int {
                63 => return s_req_query_string_start,
                35 => return s_req_fragment_start,
                _ => {}
            }
            current_block_61 = 7018308795614528254;
        }
        28 | 29 => {
            if normal_url_char[(ch as c_uchar as c_uint
                >> 3 as c_int) as usize] as c_uint
                & ((1 as c_int)
                    << (ch as c_uchar as c_uint
                        & 7 as c_uint)) as c_uint
                != 0
            {
                return s_req_query_string;
            }
            match ch as c_int {
                63 => return s_req_query_string,
                35 => return s_req_fragment_start,
                _ => {}
            }
            current_block_61 = 7018308795614528254;
        }
        30 => {
            if normal_url_char[(ch as c_uchar as c_uint
                >> 3 as c_int) as usize] as c_uint
                & ((1 as c_int)
                    << (ch as c_uchar as c_uint
                        & 7 as c_uint)) as c_uint
                != 0
            {
                return s_req_fragment;
            }
            match ch as c_int {
                63 => return s_req_fragment,
                35 => return s,
                _ => {}
            }
            current_block_61 = 7018308795614528254;
        }
        31 => {
            if normal_url_char[(ch as c_uchar as c_uint
                >> 3 as c_int) as usize] as c_uint
                & ((1 as c_int)
                    << (ch as c_uchar as c_uint
                        & 7 as c_uint)) as c_uint
                != 0
            {
                return s;
            }
            match ch as c_int {
                63 | 35 => return s,
                _ => {}
            }
            current_block_61 = 7018308795614528254;
        }
        _ => {
            current_block_61 = 7018308795614528254;
        }
    }
    match current_block_61 {
        13245741422963388543 => {
            if ch as c_int == '/' as i32 {
                return s_req_path;
            }
            if ch as c_int == '?' as i32 {
                return s_req_query_string_start;
            }
            if ch as c_int == '@' as i32 {
                return s_req_server_with_at;
            }
            if (ch as c_int | 0x20 as c_int) as c_uchar
                as c_int
                >= 'a' as i32
                && (ch as c_int | 0x20 as c_int) as c_uchar
                    as c_int
                    <= 'z' as i32
                || ch as c_int >= '0' as i32 && ch as c_int <= '9' as i32
                || (ch as c_int == '-' as i32
                    || ch as c_int == '_' as i32
                    || ch as c_int == '.' as i32
                    || ch as c_int == '!' as i32
                    || ch as c_int == '~' as i32
                    || ch as c_int == '*' as i32
                    || ch as c_int == '\'' as i32
                    || ch as c_int == '(' as i32
                    || ch as c_int == ')' as i32)
                || ch as c_int == '%' as i32
                || ch as c_int == ';' as i32
                || ch as c_int == ':' as i32
                || ch as c_int == '&' as i32
                || ch as c_int == '=' as i32
                || ch as c_int == '+' as i32
                || ch as c_int == '$' as i32
                || ch as c_int == ',' as i32
                || ch as c_int == '[' as i32
                || ch as c_int == ']' as i32
            {
                return s_req_server;
            }
        }
        _ => {}
    }
    return s_dead;
}
#[no_mangle]
pub unsafe extern "C" fn http_parser_execute(
    mut parser: *mut http_parser,
    mut settings: *const http_parser_settings,
    mut data: *const c_char,
    mut len: size_t,
) -> size_t {
    let mut hasBody: c_int = 0;
    let mut start_0: *const c_char = ::core::ptr::null::<c_char>();
    let mut h_state: header_states = h_general;
    let mut start: *const c_char = ::core::ptr::null::<c_char>();
    let mut matcher: *const c_char = ::core::ptr::null::<c_char>();
    let mut t_0: uint64_t = 0;
    let mut current_block: u64;
    let mut c: c_char = 0;
    let mut ch: c_char = 0;
    let mut unhex_val: int8_t = 0;
    let mut p: *const c_char = data;
    let mut header_field_mark: *const c_char =
        ::core::ptr::null::<c_char>();
    let mut header_value_mark: *const c_char =
        ::core::ptr::null::<c_char>();
    let mut url_mark: *const c_char = ::core::ptr::null::<c_char>();
    let mut body_mark: *const c_char = ::core::ptr::null::<c_char>();
    let mut status_mark: *const c_char = ::core::ptr::null::<c_char>();
    let mut p_state: state = (*parser).state() as state;
    let lenient: c_uint = (*parser).lenient_http_headers();
    let allow_chunked_length: c_uint = (*parser).allow_chunked_length();
    let mut nread: uint32_t = (*parser).nread;
    if (*parser).http_errno() as http_errno as c_uint
        != HPE_OK as c_int as c_uint
    {
        return 0 as size_t;
    }
    if len == 0 as size_t {
        match p_state as c_uint {
            63 => {
                '_c2rust_label: {
                    if (*parser).http_errno() as http_errno as c_uint
                        == HPE_OK as c_int as c_uint
                    {
                    } else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const c_char,
                            b"http_parser.c\0" as *const u8
                                as *const c_char,
                            671 as c_uint,
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                as *const u8 as *const c_char,
                        );
                    }
                };
                if (*settings).on_message_complete.is_some() as c_int
                    as c_long
                    != 0
                {
                    (*parser).set_state(p_state as c_uint as c_uint);
                    if (0 as c_int
                        != (*settings)
                            .on_message_complete
                            .expect("non-null function pointer")(parser))
                        as c_int as c_long
                        != 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_CB_message_complete as c_int as c_uint
                                as c_uint,
                        );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as c_uint
                        != HPE_OK as c_int as c_uint)
                        as c_int as c_long
                        != 0
                    {
                        return p.offset_from(data) as c_long as size_t;
                    }
                }
                return 0 as size_t;
            }
            1 | 2 | 4 | 18 => return 0 as size_t,
            _ => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_EOF_STATE as c_int as c_uint
                        as c_uint,
                );
                return 1 as size_t;
            }
        }
    }
    if p_state as c_uint == s_header_field as c_int as c_uint
    {
        header_field_mark = data;
    }
    if p_state as c_uint == s_header_value as c_int as c_uint
    {
        header_value_mark = data;
    }
    match p_state as c_uint {
        27 | 21 | 22 | 23 | 24 | 25 | 26 | 28 | 29 | 30 | 31 => {
            url_mark = data;
        }
        16 => {
            status_mark = data;
        }
        _ => {}
    }
    p = data;
    's_207: loop {
        if !(p != data.offset(len as isize)) {
            current_block = 14662679428699691178;
            break;
        }
        ch = *p;
        if p_state as c_uint
            <= s_headers_done as c_int as c_uint
        {
            nread = nread.wrapping_add(1 as c_int as uint32_t);
            if (nread > max_header_size) as c_int as c_long != 0 {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_HEADER_OVERFLOW as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
        }
        '_reexecute: loop {
            match p_state as c_uint {
                1 => {
                    if (ch as c_int == '\r' as i32
                        || ch as c_int == '\n' as i32)
                        as c_int as c_long
                        != 0
                    {
                        current_block = 18435049525520518667;
                        break;
                    } else {
                        current_block = 13321564401369230990;
                        break;
                    }
                }
                2 => {
                    if ch as c_int == CR || ch as c_int == LF {
                        current_block = 18435049525520518667;
                        break;
                    }
                    (*parser).set_flags(0 as c_uint as c_uint);
                    (*parser).set_uses_transfer_encoding(
                        0 as c_uint as c_uint,
                    );
                    (*parser).content_length = ULLONG_MAX as uint64_t;
                    if ch as c_int == 'H' as i32 {
                        p_state = s_res_or_resp_H;
                        '_c2rust_label_0: {
                            if (*parser).http_errno() as http_errno as c_uint
                                == HPE_OK as c_int as c_uint
                            {
                            } else {
                                __assert_fail(
                                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                        as *const c_char,
                                    b"http_parser.c\0" as *const u8
                                        as *const c_char,
                                    742 as c_uint,
                                    b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                        as *const u8 as *const c_char,
                                );
                            }
                        };
                        if (*settings).on_message_begin.is_some() as c_int
                            as c_long
                            != 0
                        {
                            (*parser)
                                .set_state(p_state as c_uint as c_uint);
                            if (0 as c_int
                                != (*settings)
                                    .on_message_begin
                                    .expect("non-null function pointer")(
                                    parser
                                )) as c_int
                                as c_long
                                != 0
                            {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_CB_message_begin as c_int
                                        as c_uint
                                        as c_uint,
                                );
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as c_uint
                                != HPE_OK as c_int as c_uint)
                                as c_int
                                as c_long
                                != 0
                            {
                                return (p.offset_from(data) as c_long
                                    + 1 as c_long)
                                    as size_t;
                            }
                        }
                        current_block = 18435049525520518667;
                        break;
                    } else {
                        (*parser).set_type_0(
                            HTTP_REQUEST as c_int as c_uint
                                as c_uint,
                        );
                        p_state = s_start_req;
                    }
                }
                3 => {
                    if ch as c_int == 'T' as i32 {
                        current_block = 17485376261910781866;
                        break;
                    } else {
                        current_block = 14648606000749551097;
                        break;
                    }
                }
                4 => {
                    if ch as c_int == CR || ch as c_int == LF {
                        current_block = 18435049525520518667;
                        break;
                    } else {
                        current_block = 3634396408142324656;
                        break;
                    }
                }
                5 => {
                    if ch as c_int != 'T' as i32 {
                        current_block = 1013506999122146761;
                        break;
                    } else {
                        current_block = 15417752026496523887;
                        break;
                    }
                }
                6 => {
                    if ch as c_int != 'T' as i32 {
                        current_block = 9430418855388998878;
                        break;
                    } else {
                        current_block = 13863458367724794628;
                        break;
                    }
                }
                7 => {
                    if ch as c_int != 'P' as i32 {
                        current_block = 7923086311623215889;
                        break;
                    } else {
                        current_block = 9343041660989783267;
                        break;
                    }
                }
                8 => {
                    if ch as c_int != '/' as i32 {
                        current_block = 7545150590528655645;
                        break;
                    } else {
                        current_block = 5250576585193495047;
                        break;
                    }
                }
                9 => {
                    if !(ch as c_int >= '0' as i32
                        && ch as c_int <= '9' as i32)
                        as c_int as c_long
                        != 0
                    {
                        current_block = 4299703460566765016;
                        break;
                    } else {
                        current_block = 6938158527927677584;
                        break;
                    }
                }
                10 => {
                    if (ch as c_int != '.' as i32) as c_int
                        as c_long
                        != 0
                    {
                        current_block = 5388205036907793036;
                        break;
                    } else {
                        current_block = 14874642226861704653;
                        break;
                    }
                }
                11 => {
                    if !(ch as c_int >= '0' as i32
                        && ch as c_int <= '9' as i32)
                        as c_int as c_long
                        != 0
                    {
                        current_block = 11226769033371074123;
                        break;
                    } else {
                        current_block = 9240481512215375588;
                        break;
                    }
                }
                12 => {
                    if (ch as c_int != ' ' as i32) as c_int
                        as c_long
                        != 0
                    {
                        current_block = 3098209481605707636;
                        break;
                    } else {
                        current_block = 9235179519944561532;
                        break;
                    }
                }
                13 => {
                    if !(ch as c_int >= '0' as i32
                        && ch as c_int <= '9' as i32)
                    {
                        current_block = 3813860224257983916;
                        break;
                    } else {
                        current_block = 16512738885216853798;
                        break;
                    }
                }
                14 => {
                    if !(ch as c_int >= '0' as i32
                        && ch as c_int <= '9' as i32)
                    {
                        match ch as c_int {
                            32 => {
                                p_state = s_res_status_start;
                                current_block = 18435049525520518667;
                                break;
                            }
                            CR | LF => {
                                p_state = s_res_status_start;
                            }
                            _ => {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_INVALID_STATUS as c_int as c_uint
                                        as c_uint,
                                );
                                current_block = 13067290550027806311;
                                break 's_207;
                            }
                        }
                    } else {
                        (*parser).set_status_code(
                            (*parser).status_code()
                                * 10 as c_int as c_uint,
                        );
                        (*parser).set_status_code(
                            (*parser).status_code()
                                + (ch as c_int - '0' as i32) as c_uint,
                        );
                        if ((*parser).status_code() as c_int
                            > 999 as c_int)
                            as c_int as c_long
                            != 0
                        {
                            current_block = 11577926782275222206;
                            break;
                        } else {
                            current_block = 18435049525520518667;
                            break;
                        }
                    }
                }
                15 => {
                    if status_mark.is_null() {
                        status_mark = p;
                    }
                    p_state = s_res_status;
                    (*parser).set_index(0 as c_uint as c_uint);
                    if !(ch as c_int == CR || ch as c_int == LF) {
                        current_block = 18435049525520518667;
                        break;
                    }
                }
                16 => {
                    if ch as c_int == CR {
                        current_block = 7188795011561844502;
                        break;
                    } else {
                        current_block = 15609529146834799275;
                        break;
                    }
                }
                17 => {
                    if ch as c_int != '\n' as i32 {
                        current_block = 16832305905353653446;
                        break;
                    } else {
                        current_block = 9028266288740425872;
                        break;
                    }
                }
                18 => {
                    if ch as c_int == CR || ch as c_int == LF {
                        current_block = 18435049525520518667;
                        break;
                    } else {
                        current_block = 12853382357061961387;
                        break;
                    }
                }
                19 => {
                    matcher = ::core::ptr::null::<c_char>();
                    if (ch as c_int == '\0' as i32) as c_int
                        as c_long
                        != 0
                    {
                        current_block = 7926734633677835471;
                        break;
                    } else {
                        current_block = 6341332686669895749;
                        break;
                    }
                }
                20 => {
                    if ch as c_int == ' ' as i32 {
                        current_block = 18435049525520518667;
                        break;
                    } else {
                        current_block = 13755523488868872559;
                        break;
                    }
                }
                21 | 22 | 23 | 24 => match ch as c_int {
                    32 | CR | LF => {
                        current_block = 8953125900534742068;
                        break;
                    }
                    _ => {
                        current_block = 9091570581087047569;
                        break;
                    }
                },
                25 | 26 | 27 | 28 | 29 | 30 | 31 => match ch as c_int {
                    32 => {
                        current_block = 15853744263182998232;
                        break;
                    }
                    CR | LF => {
                        current_block = 246505757814604223;
                        break;
                    }
                    _ => {
                        current_block = 9878511696400499081;
                        break;
                    }
                },
                32 => match ch as c_int {
                    32 => {
                        current_block = 18435049525520518667;
                        break;
                    }
                    72 => {
                        current_block = 12168134392480497817;
                        break;
                    }
                    73 => {
                        current_block = 12770807392901873815;
                        break;
                    }
                    _ => {
                        current_block = 16199754411564412847;
                        break;
                    }
                },
                33 => {
                    if ch as c_int != 'T' as i32 {
                        current_block = 3503188808869013853;
                        break;
                    } else {
                        current_block = 8889999340123292593;
                        break;
                    }
                }
                34 => {
                    if ch as c_int != 'T' as i32 {
                        current_block = 5800290220634814141;
                        break;
                    } else {
                        current_block = 588672170829148014;
                        break;
                    }
                }
                35 => {
                    if ch as c_int != 'P' as i32 {
                        current_block = 7460542724658431689;
                        break;
                    } else {
                        current_block = 9181201145437202872;
                        break;
                    }
                }
                37 => {
                    if ch as c_int != 'C' as i32 {
                        current_block = 17918532803028278290;
                        break;
                    } else {
                        current_block = 10530107118073347621;
                        break;
                    }
                }
                38 => {
                    if ch as c_int != 'E' as i32 {
                        current_block = 16653817520844849459;
                        break;
                    } else {
                        current_block = 5271113972066554886;
                        break;
                    }
                }
                36 => {
                    if ch as c_int != '/' as i32 {
                        current_block = 13845568614582207489;
                        break;
                    } else {
                        current_block = 12952730798101289484;
                        break;
                    }
                }
                39 => {
                    if !(ch as c_int >= '0' as i32
                        && ch as c_int <= '9' as i32)
                        as c_int as c_long
                        != 0
                    {
                        current_block = 9532072207990609994;
                        break;
                    } else {
                        current_block = 12304281332906078777;
                        break;
                    }
                }
                40 => {
                    if (ch as c_int != '.' as i32) as c_int
                        as c_long
                        != 0
                    {
                        current_block = 6452958575495017105;
                        break;
                    } else {
                        current_block = 11644938206506967943;
                        break;
                    }
                }
                41 => {
                    if !(ch as c_int >= '0' as i32
                        && ch as c_int <= '9' as i32)
                        as c_int as c_long
                        != 0
                    {
                        current_block = 16591777222291248247;
                        break;
                    } else {
                        current_block = 6363466913441788494;
                        break;
                    }
                }
                42 => {
                    if ch as c_int == CR {
                        current_block = 4265637299601780024;
                        break;
                    } else {
                        current_block = 18060968742384277177;
                        break;
                    }
                }
                43 => {
                    if (ch as c_int != '\n' as i32) as c_int
                        as c_long
                        != 0
                    {
                        current_block = 3145106269552242860;
                        break;
                    } else {
                        current_block = 13016604015400381741;
                        break;
                    }
                }
                44 => {
                    if ch as c_int == CR {
                        p_state = s_headers_almost_done;
                        current_block = 18435049525520518667;
                        break;
                    } else if ch as c_int == LF {
                        p_state = s_headers_almost_done;
                    } else {
                        c = (if ch as c_int == ' ' as i32 {
                            0 as c_int
                        } else {
                            tokens[ch as c_uchar as usize] as c_int
                        }) as c_char;
                        if (c == 0) as c_int as c_long != 0 {
                            current_block = 15266942463618272814;
                            break;
                        } else {
                            current_block = 7766894571154031170;
                            break;
                        }
                    }
                }
                45 => {
                    start = p;
                    while p != data.offset(len as isize) {
                        ch = *p;
                        c = (if ch as c_int == ' ' as i32 {
                            0 as c_int
                        } else {
                            tokens[ch as c_uchar as usize] as c_int
                        }) as c_char;
                        if c == 0 {
                            break;
                        }
                        match (*parser).header_state() as c_int {
                            0 => {
                                let mut left: size_t = data.offset(len as isize).offset_from(p)
                                    as c_long
                                    as size_t;
                                let mut pe: *const c_char = p.offset(
                                    (if left < max_header_size as size_t {
                                        left
                                    } else {
                                        max_header_size as size_t
                                    }) as isize,
                                );
                                while p.offset(1 as c_int as isize) < pe
                                    && (if *p.offset(1 as c_int as isize)
                                        as c_int
                                        == ' ' as i32
                                    {
                                        0 as c_int
                                    } else {
                                        tokens[*p.offset(1 as c_int as isize)
                                            as c_uchar
                                            as usize]
                                            as c_int
                                    }) != 0
                                {
                                    p = p.offset(1);
                                }
                            }
                            1 => {
                                (*parser).set_index((*parser).index() + 1 as c_uint);
                                (*parser).set_header_state(
                                    (if c as c_int == 'o' as i32 {
                                        h_CO as c_int
                                    } else {
                                        h_general as c_int
                                    }) as c_uint
                                        as c_uint,
                                );
                            }
                            2 => {
                                (*parser).set_index((*parser).index() + 1 as c_uint);
                                (*parser).set_header_state(
                                    (if c as c_int == 'n' as i32 {
                                        h_CON as c_int
                                    } else {
                                        h_general as c_int
                                    }) as c_uint
                                        as c_uint,
                                );
                            }
                            3 => {
                                (*parser).set_index((*parser).index() + 1 as c_uint);
                                match c as c_int {
                                    110 => {
                                        (*parser).set_header_state(
                                            h_matching_connection as c_int
                                                as c_uint
                                                as c_uint,
                                        );
                                    }
                                    116 => {
                                        (*parser).set_header_state(
                                            h_matching_content_length as c_int
                                                as c_uint
                                                as c_uint,
                                        );
                                    }
                                    _ => {
                                        (*parser).set_header_state(
                                            h_general as c_int as c_uint
                                                as c_uint,
                                        );
                                    }
                                }
                            }
                            4 => {
                                (*parser).set_index((*parser).index() + 1 as c_uint);
                                if (*parser).index() as usize
                                    > (::core::mem::size_of::<[c_char; 11]>() as usize)
                                        .wrapping_sub(1 as usize)
                                    || c as c_int
                                        != CONNECTION[(*parser).index() as usize]
                                            as c_int
                                {
                                    (*parser).set_header_state(
                                        h_general as c_int as c_uint
                                            as c_uint,
                                    );
                                } else if (*parser).index() as usize
                                    == (::core::mem::size_of::<[c_char; 11]>()
                                        as usize)
                                        .wrapping_sub(2 as usize)
                                {
                                    (*parser).set_header_state(
                                        h_connection as c_int as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            5 => {
                                (*parser).set_index((*parser).index() + 1 as c_uint);
                                if (*parser).index() as usize
                                    > (::core::mem::size_of::<[c_char; 17]>() as usize)
                                        .wrapping_sub(1 as usize)
                                    || c as c_int
                                        != PROXY_CONNECTION[(*parser).index() as usize]
                                            as c_int
                                {
                                    (*parser).set_header_state(
                                        h_general as c_int as c_uint
                                            as c_uint,
                                    );
                                } else if (*parser).index() as usize
                                    == (::core::mem::size_of::<[c_char; 17]>()
                                        as usize)
                                        .wrapping_sub(2 as usize)
                                {
                                    (*parser).set_header_state(
                                        h_connection as c_int as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            6 => {
                                (*parser).set_index((*parser).index() + 1 as c_uint);
                                if (*parser).index() as usize
                                    > (::core::mem::size_of::<[c_char; 15]>() as usize)
                                        .wrapping_sub(1 as usize)
                                    || c as c_int
                                        != CONTENT_LENGTH[(*parser).index() as usize]
                                            as c_int
                                {
                                    (*parser).set_header_state(
                                        h_general as c_int as c_uint
                                            as c_uint,
                                    );
                                } else if (*parser).index() as usize
                                    == (::core::mem::size_of::<[c_char; 15]>()
                                        as usize)
                                        .wrapping_sub(2 as usize)
                                {
                                    (*parser).set_header_state(
                                        h_content_length as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            7 => {
                                (*parser).set_index((*parser).index() + 1 as c_uint);
                                if (*parser).index() as usize
                                    > (::core::mem::size_of::<[c_char; 18]>() as usize)
                                        .wrapping_sub(1 as usize)
                                    || c as c_int
                                        != TRANSFER_ENCODING[(*parser).index() as usize]
                                            as c_int
                                {
                                    (*parser).set_header_state(
                                        h_general as c_int as c_uint
                                            as c_uint,
                                    );
                                } else if (*parser).index() as usize
                                    == (::core::mem::size_of::<[c_char; 18]>()
                                        as usize)
                                        .wrapping_sub(2 as usize)
                                {
                                    (*parser).set_header_state(
                                        h_transfer_encoding as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    (*parser).set_uses_transfer_encoding(
                                        1 as c_uint as c_uint,
                                    );
                                }
                            }
                            8 => {
                                (*parser).set_index((*parser).index() + 1 as c_uint);
                                if (*parser).index() as usize
                                    > (::core::mem::size_of::<[c_char; 8]>() as usize)
                                        .wrapping_sub(1 as usize)
                                    || c as c_int
                                        != UPGRADE[(*parser).index() as usize] as c_int
                                {
                                    (*parser).set_header_state(
                                        h_general as c_int as c_uint
                                            as c_uint,
                                    );
                                } else if (*parser).index() as usize
                                    == (::core::mem::size_of::<[c_char; 8]>() as usize)
                                        .wrapping_sub(2 as usize)
                                {
                                    (*parser).set_header_state(
                                        h_upgrade as c_int as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            9 | 10 | 13 | 14 => {
                                if ch as c_int != ' ' as i32 {
                                    (*parser).set_header_state(
                                        h_general as c_int as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            _ => {
                                '_c2rust_label_7: {
                                    if 0 as c_int != 0
                                        && !(b"Unknown header_state\0" as *const u8
                                            as *const c_char)
                                            .is_null()
                                    {
                                    } else {
                                        __assert_fail(
                                            b"0 && \"Unknown header_state\"\0" as *const u8
                                                as *const c_char,
                                            b"http_parser.c\0" as *const u8
                                                as *const c_char,
                                            1370 as c_uint,
                                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                                as *const u8 as *const c_char,
                                        );
                                    }
                                };
                            }
                        }
                        p = p.offset(1);
                    }
                    if p == data.offset(len as isize) {
                        current_block = 1795228135880752216;
                        break;
                    } else {
                        current_block = 5570469760270364154;
                        break;
                    }
                }
                46 => {
                    if ch as c_int == ' ' as i32
                        || ch as c_int == '\t' as i32
                    {
                        current_block = 18435049525520518667;
                        break;
                    } else {
                        current_block = 13620783514648996490;
                        break;
                    }
                }
                49 => {
                    current_block = 7825823688640729701;
                    break;
                }
                50 => {
                    start_0 = p;
                    h_state = (*parser).header_state() as header_states;
                    loop {
                        if !(p != data.offset(len as isize)) {
                            current_block = 2468366247994693734;
                            break '_reexecute;
                        }
                        ch = *p;
                        if ch as c_int == CR {
                            p_state = s_header_almost_done;
                            (*parser).set_header_state(
                                h_state as c_uint as c_uint,
                            );
                            '_c2rust_label_9: {
                                if (*parser).http_errno() as http_errno as c_uint
                                    == HPE_OK as c_int as c_uint
                                {
                                } else {
                                    __assert_fail(
                                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                            as *const c_char,
                                        b"http_parser.c\0" as *const u8
                                            as *const c_char,
                                        1491 as c_uint,
                                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                            as *const u8 as *const c_char,
                                    );
                                }
                            };
                            if !header_value_mark.is_null() {
                                if (*settings).on_header_value.is_some() as c_int
                                    as c_long
                                    != 0
                                {
                                    (*parser).set_state(
                                        p_state as c_uint as c_uint,
                                    );
                                    if (0 as c_int
                                        != (*settings)
                                            .on_header_value
                                            .expect("non-null function pointer")(
                                            parser,
                                            header_value_mark,
                                            p.offset_from(header_value_mark) as c_long
                                                as size_t,
                                        ))
                                        as c_int
                                        as c_long
                                        != 0
                                    {
                                        (*parser).nread = nread;
                                        (*parser).set_http_errno(
                                            HPE_CB_header_value as c_int
                                                as c_uint
                                                as c_uint,
                                        );
                                    }
                                    p_state = (*parser).state() as state;
                                    if ((*parser).http_errno() as http_errno as c_uint
                                        != HPE_OK as c_int as c_uint)
                                        as c_int
                                        as c_long
                                        != 0
                                    {
                                        return (p.offset_from(data) as c_long
                                            + 1 as c_long)
                                            as size_t;
                                    }
                                }
                                header_value_mark = ::core::ptr::null::<c_char>();
                            }
                            current_block = 2468366247994693734;
                            break '_reexecute;
                        } else if ch as c_int == LF {
                            p_state = s_header_almost_done;
                            nread = nread.wrapping_add(
                                p.offset_from(start_0) as c_long as uint32_t,
                            );
                            if (nread > max_header_size) as c_int
                                as c_long
                                != 0
                            {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_HEADER_OVERFLOW as c_int as c_uint
                                        as c_uint,
                                );
                                current_block = 13067290550027806311;
                                break 's_207;
                            } else {
                                (*parser).set_header_state(
                                    h_state as c_uint as c_uint,
                                );
                                '_c2rust_label_10: {
                                    if (*parser).http_errno() as http_errno as c_uint
                                        == HPE_OK as c_int as c_uint
                                    {
                                    } else {
                                        __assert_fail(
                                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                                as *const c_char,
                                            b"http_parser.c\0" as *const u8
                                                as *const c_char,
                                            1499 as c_uint,
                                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                                as *const u8 as *const c_char,
                                        );
                                    }
                                };
                                if !header_value_mark.is_null() {
                                    if (*settings).on_header_value.is_some() as c_int
                                        as c_long
                                        != 0
                                    {
                                        (*parser).set_state(
                                            p_state as c_uint as c_uint,
                                        );
                                        if (0 as c_int
                                            != (*settings)
                                                .on_header_value
                                                .expect("non-null function pointer")(
                                                parser,
                                                header_value_mark,
                                                p.offset_from(header_value_mark)
                                                    as c_long
                                                    as size_t,
                                            ))
                                            as c_int
                                            as c_long
                                            != 0
                                        {
                                            (*parser).nread = nread;
                                            (*parser).set_http_errno(
                                                HPE_CB_header_value as c_int
                                                    as c_uint
                                                    as c_uint,
                                            );
                                        }
                                        p_state = (*parser).state() as state;
                                        if ((*parser).http_errno() as http_errno
                                            as c_uint
                                            != HPE_OK as c_int as c_uint)
                                            as c_int
                                            as c_long
                                            != 0
                                        {
                                            return p.offset_from(data) as c_long
                                                as size_t;
                                        }
                                    }
                                    header_value_mark = ::core::ptr::null::<c_char>();
                                }
                                break;
                            }
                        } else if lenient == 0
                            && !(ch as c_int == CR
                                || ch as c_int == LF
                                || ch as c_int == 9 as c_int
                                || ch as c_uchar as c_int
                                    > 31 as c_int
                                    && ch as c_int != 127 as c_int)
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_INVALID_HEADER_TOKEN as c_int
                                    as c_uint
                                    as c_uint,
                            );
                            current_block = 13067290550027806311;
                            break 's_207;
                        } else {
                            c = (ch as c_int | 0x20 as c_int)
                                as c_uchar
                                as c_char;
                            match h_state as c_uint {
                                0 => {
                                    let mut left_0: size_t =
                                        data.offset(len as isize).offset_from(p)
                                            as c_long
                                            as size_t;
                                    let mut pe_0: *const c_char = p.offset(
                                        (if left_0 < max_header_size as size_t {
                                            left_0
                                        } else {
                                            max_header_size as size_t
                                        }) as isize,
                                    );
                                    while p != pe_0 {
                                        ch = *p;
                                        if ch as c_int == CR
                                            || ch as c_int == LF
                                        {
                                            p = p.offset(-1);
                                            break;
                                        } else if lenient == 0
                                            && !(ch as c_int == CR
                                                || ch as c_int == LF
                                                || ch as c_int
                                                    == 9 as c_int
                                                || ch as c_uchar as c_int
                                                    > 31 as c_int
                                                    && ch as c_int
                                                        != 127 as c_int)
                                        {
                                            (*parser).nread = nread;
                                            (*parser).set_http_errno(
                                                HPE_INVALID_HEADER_TOKEN as c_int
                                                    as c_uint
                                                    as c_uint,
                                            );
                                            current_block = 13067290550027806311;
                                            break 's_207;
                                        } else {
                                            p = p.offset(1);
                                        }
                                    }
                                    if p == data.offset(len as isize) {
                                        p = p.offset(-1);
                                    }
                                    current_block = 4576755628229007767;
                                }
                                9 | 13 => {
                                    '_c2rust_label_11: {
                                        if 0 as c_int != 0
                                            && !(b"Shouldn't get here.\0" as *const u8
                                                as *const c_char)
                                                .is_null()
                                        {
                                        } else {
                                            __assert_fail(
                                                b"0 && \"Shouldn't get here.\"\0" as *const u8
                                                    as *const c_char,
                                                b"http_parser.c\0" as *const u8
                                                    as *const c_char,
                                                1534 as c_uint,
                                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                                    as *const u8 as *const c_char,
                                            );
                                        }
                                    };
                                    current_block = 4576755628229007767;
                                }
                                10 => {
                                    if ch as c_int == ' ' as i32 {
                                        current_block = 4576755628229007767;
                                    } else {
                                        h_state = h_content_length_num;
                                        current_block = 5372543090928527736;
                                    }
                                }
                                11 => {
                                    current_block = 5372543090928527736;
                                }
                                12 => {
                                    if ch as c_int == ' ' as i32 {
                                        current_block = 4576755628229007767;
                                    } else {
                                        (*parser).nread = nread;
                                        (*parser).set_http_errno(
                                            HPE_INVALID_CONTENT_LENGTH as c_int
                                                as c_uint
                                                as c_uint,
                                        );
                                        (*parser).set_header_state(
                                            h_state as c_uint as c_uint,
                                        );
                                        current_block = 13067290550027806311;
                                        break 's_207;
                                    }
                                }
                                15 => {
                                    if 'c' as i32 == c as c_int {
                                        h_state = h_matching_transfer_encoding_chunked;
                                    } else if if c as c_int == ' ' as i32 {
                                        0 as c_int
                                    } else {
                                        tokens[c as c_uchar as usize]
                                            as c_int
                                    } != 0
                                    {
                                        h_state = h_matching_transfer_encoding_token;
                                    } else if !(c as c_int == ' ' as i32
                                        || c as c_int == '\t' as i32)
                                    {
                                        h_state = h_general;
                                    }
                                    current_block = 4576755628229007767;
                                }
                                16 => {
                                    (*parser)
                                        .set_index((*parser).index() + 1 as c_uint);
                                    if (*parser).index() as usize
                                        > (::core::mem::size_of::<[c_char; 8]>()
                                            as usize)
                                            .wrapping_sub(1 as usize)
                                        || c as c_int
                                            != CHUNKED[(*parser).index() as usize]
                                                as c_int
                                    {
                                        h_state = h_matching_transfer_encoding_token;
                                    } else if (*parser).index() as usize
                                        == (::core::mem::size_of::<[c_char; 8]>()
                                            as usize)
                                            .wrapping_sub(2 as usize)
                                    {
                                        h_state = h_transfer_encoding_chunked;
                                    }
                                    current_block = 4576755628229007767;
                                }
                                17 => {
                                    if ch as c_int == ',' as i32 {
                                        h_state = h_matching_transfer_encoding_token_start;
                                        (*parser).set_index(
                                            0 as c_uint as c_uint,
                                        );
                                    }
                                    current_block = 4576755628229007767;
                                }
                                18 => {
                                    if c as c_int == 'k' as i32 {
                                        h_state = h_matching_connection_keep_alive;
                                    } else if c as c_int == 'c' as i32 {
                                        h_state = h_matching_connection_close;
                                    } else if c as c_int == 'u' as i32 {
                                        h_state = h_matching_connection_upgrade;
                                    } else if if c as c_int == ' ' as i32 {
                                        0 as c_int
                                    } else {
                                        tokens[c as c_uchar as usize]
                                            as c_int
                                    } != 0
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if !(c as c_int == ' ' as i32
                                        || c as c_int == '\t' as i32)
                                    {
                                        h_state = h_general;
                                    }
                                    current_block = 4576755628229007767;
                                }
                                19 => {
                                    (*parser)
                                        .set_index((*parser).index() + 1 as c_uint);
                                    if (*parser).index() as usize
                                        > (::core::mem::size_of::<[c_char; 11]>()
                                            as usize)
                                            .wrapping_sub(1 as usize)
                                        || c as c_int
                                            != KEEP_ALIVE[(*parser).index() as usize]
                                                as c_int
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if (*parser).index() as usize
                                        == (::core::mem::size_of::<[c_char; 11]>()
                                            as usize)
                                            .wrapping_sub(2 as usize)
                                    {
                                        h_state = h_connection_keep_alive;
                                    }
                                    current_block = 4576755628229007767;
                                }
                                20 => {
                                    (*parser)
                                        .set_index((*parser).index() + 1 as c_uint);
                                    if (*parser).index() as usize
                                        > (::core::mem::size_of::<[c_char; 6]>()
                                            as usize)
                                            .wrapping_sub(1 as usize)
                                        || c as c_int
                                            != CLOSE[(*parser).index() as usize]
                                                as c_int
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if (*parser).index() as usize
                                        == (::core::mem::size_of::<[c_char; 6]>()
                                            as usize)
                                            .wrapping_sub(2 as usize)
                                    {
                                        h_state = h_connection_close;
                                    }
                                    current_block = 4576755628229007767;
                                }
                                21 => {
                                    (*parser)
                                        .set_index((*parser).index() + 1 as c_uint);
                                    if (*parser).index() as usize
                                        > (::core::mem::size_of::<[c_char; 8]>()
                                            as usize)
                                            .wrapping_sub(1 as usize)
                                        || c as c_int
                                            != UPGRADE[(*parser).index() as usize]
                                                as c_int
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if (*parser).index() as usize
                                        == (::core::mem::size_of::<[c_char; 8]>()
                                            as usize)
                                            .wrapping_sub(2 as usize)
                                    {
                                        h_state = h_connection_upgrade;
                                    }
                                    current_block = 4576755628229007767;
                                }
                                22 => {
                                    if ch as c_int == ',' as i32 {
                                        h_state = h_matching_connection_token_start;
                                        (*parser).set_index(
                                            0 as c_uint as c_uint,
                                        );
                                    }
                                    current_block = 4576755628229007767;
                                }
                                23 => {
                                    if ch as c_int != ' ' as i32 {
                                        h_state = h_matching_transfer_encoding_token;
                                    }
                                    current_block = 4576755628229007767;
                                }
                                24 | 25 | 26 => {
                                    if ch as c_int == ',' as i32 {
                                        if h_state as c_uint
                                            == h_connection_keep_alive as c_int
                                                as c_uint
                                        {
                                            (*parser).set_flags(
                                                (*parser).flags()
                                                    | F_CONNECTION_KEEP_ALIVE as c_int
                                                        as c_uint,
                                            );
                                        } else if h_state as c_uint
                                            == h_connection_close as c_int
                                                as c_uint
                                        {
                                            (*parser).set_flags(
                                                (*parser).flags()
                                                    | F_CONNECTION_CLOSE as c_int
                                                        as c_uint,
                                            );
                                        } else if h_state as c_uint
                                            == h_connection_upgrade as c_int
                                                as c_uint
                                        {
                                            (*parser).set_flags(
                                                (*parser).flags()
                                                    | F_CONNECTION_UPGRADE as c_int
                                                        as c_uint,
                                            );
                                        }
                                        h_state = h_matching_connection_token_start;
                                        (*parser).set_index(
                                            0 as c_uint as c_uint,
                                        );
                                    } else if ch as c_int != ' ' as i32 {
                                        h_state = h_matching_connection_token;
                                    }
                                    current_block = 4576755628229007767;
                                }
                                _ => {
                                    p_state = s_header_value;
                                    h_state = h_general;
                                    current_block = 4576755628229007767;
                                }
                            }
                            match current_block {
                                5372543090928527736 => {
                                    let mut t: uint64_t = 0;
                                    if ch as c_int == ' ' as i32 {
                                        h_state = h_content_length_ws;
                                    } else if !(ch as c_int >= '0' as i32
                                        && ch as c_int <= '9' as i32)
                                        as c_int
                                        as c_long
                                        != 0
                                    {
                                        (*parser).nread = nread;
                                        (*parser).set_http_errno(
                                            HPE_INVALID_CONTENT_LENGTH as c_int
                                                as c_uint
                                                as c_uint,
                                        );
                                        (*parser).set_header_state(
                                            h_state as c_uint as c_uint,
                                        );
                                        current_block = 13067290550027806311;
                                        break 's_207;
                                    } else {
                                        t = (*parser).content_length;
                                        t = t.wrapping_mul(10 as uint64_t);
                                        t = t.wrapping_add(
                                            (ch as c_int - '0' as i32) as uint64_t,
                                        );
                                        if ((9223372036854775807 as c_ulonglong)
                                            .wrapping_mul(2 as c_ulonglong)
                                            .wrapping_add(1 as c_ulonglong)
                                            .wrapping_sub(10 as c_ulonglong)
                                            .wrapping_div(10 as c_ulonglong)
                                            < (*parser).content_length as c_ulonglong)
                                            as c_int
                                            as c_long
                                            != 0
                                        {
                                            (*parser).nread = nread;
                                            (*parser).set_http_errno(
                                                HPE_INVALID_CONTENT_LENGTH as c_int
                                                    as c_uint
                                                    as c_uint,
                                            );
                                            (*parser).set_header_state(
                                                h_state as c_uint
                                                    as c_uint,
                                            );
                                            current_block = 13067290550027806311;
                                            break 's_207;
                                        } else {
                                            (*parser).content_length = t;
                                        }
                                    }
                                }
                                _ => {}
                            }
                            p = p.offset(1);
                        }
                    }
                }
                52 => {
                    if (ch as c_int != '\n' as i32) as c_int
                        as c_long
                        != 0
                    {
                        current_block = 16854099570775812386;
                        break;
                    } else {
                        current_block = 14641528508070947041;
                        break;
                    }
                }
                51 => {
                    if ch as c_int == ' ' as i32
                        || ch as c_int == '\t' as i32
                    {
                        if (*parser).header_state() as c_int
                            == h_content_length_num as c_int
                        {
                            (*parser).set_header_state(
                                h_content_length_ws as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = s_header_value_start;
                    } else {
                        match (*parser).header_state() as c_int {
                            24 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_CONNECTION_KEEP_ALIVE as c_int
                                            as c_uint,
                                );
                            }
                            25 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_CONNECTION_CLOSE as c_int
                                            as c_uint,
                                );
                            }
                            23 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_CHUNKED as c_int as c_uint,
                                );
                            }
                            26 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_CONNECTION_UPGRADE as c_int
                                            as c_uint,
                                );
                            }
                            _ => {}
                        }
                        p_state = s_header_field_start;
                    }
                }
                47 => {
                    if ch as c_int != '\n' as i32 {
                        current_block = 12609744167958600007;
                        break;
                    } else {
                        current_block = 2164532986857606317;
                        break;
                    }
                }
                48 => {
                    if ch as c_int == ' ' as i32
                        || ch as c_int == '\t' as i32
                    {
                        p_state = s_header_value_discard_ws;
                        current_block = 18435049525520518667;
                        break;
                    } else {
                        match (*parser).header_state() as c_int {
                            24 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_CONNECTION_KEEP_ALIVE as c_int
                                            as c_uint,
                                );
                            }
                            25 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_CONNECTION_CLOSE as c_int
                                            as c_uint,
                                );
                            }
                            26 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_CONNECTION_UPGRADE as c_int
                                            as c_uint,
                                );
                            }
                            23 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_CHUNKED as c_int as c_uint,
                                );
                            }
                            10 => {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_INVALID_CONTENT_LENGTH as c_int
                                        as c_uint
                                        as c_uint,
                                );
                                current_block = 13067290550027806311;
                                break 's_207;
                            }
                            _ => {}
                        }
                        if header_value_mark.is_null() {
                            header_value_mark = p;
                        }
                        p_state = s_header_field_start;
                        '_c2rust_label_12: {
                            if (*parser).http_errno() as http_errno as c_uint
                                == HPE_OK as c_int as c_uint
                            {
                            } else {
                                __assert_fail(
                                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                        as *const c_char,
                                    b"http_parser.c\0" as *const u8
                                        as *const c_char,
                                    1790 as c_uint,
                                    b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                        as *const u8 as *const c_char,
                                );
                            }
                        };
                        if !header_value_mark.is_null() {
                            if (*settings).on_header_value.is_some() as c_int
                                as c_long
                                != 0
                            {
                                (*parser).set_state(
                                    p_state as c_uint as c_uint,
                                );
                                if (0 as c_int
                                    != (*settings)
                                        .on_header_value
                                        .expect("non-null function pointer")(
                                        parser,
                                        header_value_mark,
                                        p.offset_from(header_value_mark) as c_long
                                            as size_t,
                                    )) as c_int
                                    as c_long
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_CB_header_value as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                                p_state = (*parser).state() as state;
                                if ((*parser).http_errno() as http_errno as c_uint
                                    != HPE_OK as c_int as c_uint)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    return p.offset_from(data) as c_long as size_t;
                                }
                            }
                            header_value_mark = ::core::ptr::null::<c_char>();
                        }
                    }
                }
                57 => {
                    if ch as c_int != '\n' as i32 {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_STRICT as c_int as c_uint
                                as c_uint,
                        );
                        current_block = 13067290550027806311;
                        break 's_207;
                    } else if (*parser).flags() as c_int
                        & F_TRAILING as c_int
                        != 0
                    {
                        p_state = s_message_done;
                        '_c2rust_label_13: {
                            if (*parser).http_errno() as http_errno as c_uint
                                == HPE_OK as c_int as c_uint
                            {
                            } else {
                                __assert_fail(
                                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                        as *const c_char,
                                    b"http_parser.c\0" as *const u8
                                        as *const c_char,
                                    1802 as c_uint,
                                    b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                        as *const u8 as *const c_char,
                                );
                            }
                        };
                        if (*settings).on_chunk_complete.is_some() as c_int
                            as c_long
                            != 0
                        {
                            (*parser)
                                .set_state(p_state as c_uint as c_uint);
                            if (0 as c_int
                                != (*settings)
                                    .on_chunk_complete
                                    .expect("non-null function pointer")(
                                    parser
                                )) as c_int
                                as c_long
                                != 0
                            {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_CB_chunk_complete as c_int
                                        as c_uint
                                        as c_uint,
                                );
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as c_uint
                                != HPE_OK as c_int as c_uint)
                                as c_int
                                as c_long
                                != 0
                            {
                                return p.offset_from(data) as c_long as size_t;
                            }
                        }
                    } else {
                        if (*parser).uses_transfer_encoding() as c_int
                            == 1 as c_int
                            && (*parser).flags() as c_int
                                & F_CONTENTLENGTH as c_int
                                != 0
                        {
                            if (*parser).flags() as c_int
                                & F_CHUNKED as c_int
                                != 0
                            {
                                if allow_chunked_length == 0 {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_UNEXPECTED_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break 's_207;
                                }
                            } else if lenient == 0 {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_UNEXPECTED_CONTENT_LENGTH as c_int
                                        as c_uint
                                        as c_uint,
                                );
                                current_block = 13067290550027806311;
                                break 's_207;
                            }
                        }
                        p_state = s_headers_done;
                        if (*parser).flags() as c_int & F_UPGRADE as c_int
                            != 0
                            && (*parser).flags() as c_int
                                & F_CONNECTION_UPGRADE as c_int
                                != 0
                        {
                            (*parser).set_upgrade(
                                ((*parser).type_0() as c_int
                                    == HTTP_REQUEST as c_int
                                    || (*parser).status_code() as c_int
                                        == 101 as c_int)
                                    as c_int
                                    as c_uint
                                    as c_uint,
                            );
                        } else {
                            (*parser).set_upgrade(
                                ((*parser).method() as c_int
                                    == HTTP_CONNECT as c_int)
                                    as c_int
                                    as c_uint
                                    as c_uint,
                            );
                        }
                        if (*settings).on_headers_complete.is_some() {
                            let mut current_block_938: u64;
                            match (*settings)
                                .on_headers_complete
                                .expect("non-null function pointer")(
                                parser
                            ) {
                                0 => {
                                    current_block_938 = 11474802207773668075;
                                }
                                2 => {
                                    (*parser).set_upgrade(
                                        1 as c_uint as c_uint,
                                    );
                                    current_block_938 = 4584014875562377034;
                                }
                                1 => {
                                    current_block_938 = 4584014875562377034;
                                }
                                _ => {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_CB_headers_complete as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    (*parser).nread = nread;
                                    (*parser).set_state(
                                        p_state as c_uint as c_uint,
                                    );
                                    return p.offset_from(data) as c_long as size_t;
                                }
                            }
                            match current_block_938 {
                                4584014875562377034 => {
                                    (*parser).set_flags(
                                        (*parser).flags()
                                            | F_SKIPBODY as c_int
                                                as c_uint,
                                    );
                                }
                                _ => {}
                            }
                        }
                        if (*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint
                        {
                            (*parser).nread = nread;
                            (*parser)
                                .set_state(p_state as c_uint as c_uint);
                            return p.offset_from(data) as c_long as size_t;
                        }
                    }
                }
                58 => {
                    hasBody = 0;
                    if ch as c_int != '\n' as i32 {
                        current_block = 11707042610706539144;
                        break;
                    } else {
                        current_block = 15753004637962686906;
                        break;
                    }
                }
                62 => {
                    let mut to_read: uint64_t = if (*parser).content_length
                        < data.offset(len as isize).offset_from(p) as c_long
                            as uint64_t
                    {
                        (*parser).content_length
                    } else {
                        data.offset(len as isize).offset_from(p) as c_long as uint64_t
                    };
                    '_c2rust_label_18: {
                        if (*parser).content_length != 0 as uint64_t
                            && (*parser).content_length as c_ulonglong
                                != (9223372036854775807 as c_ulonglong)
                                    .wrapping_mul(2 as c_ulonglong)
                                    .wrapping_add(1 as c_ulonglong)
                        {
                        } else {
                            __assert_fail(
                                b"parser->content_length != 0 && parser->content_length != ULLONG_MAX\0"
                                    as *const u8 as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1950 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if body_mark.is_null() {
                        body_mark = p;
                    }
                    (*parser).content_length = (*parser).content_length.wrapping_sub(to_read);
                    p = p.offset(to_read.wrapping_sub(1 as uint64_t) as isize);
                    if !((*parser).content_length == 0 as uint64_t) {
                        current_block = 18435049525520518667;
                        break;
                    }
                    p_state = s_message_done;
                    '_c2rust_label_19: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1973 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if !body_mark.is_null() {
                        if (*settings).on_body.is_some() as c_int
                            as c_long
                            != 0
                        {
                            (*parser)
                                .set_state(p_state as c_uint as c_uint);
                            if (0 as c_int
                                != (*settings).on_body.expect("non-null function pointer")(
                                    parser,
                                    body_mark,
                                    (p.offset_from(body_mark) as c_long
                                        + 1 as c_long)
                                        as size_t,
                                )) as c_int
                                as c_long
                                != 0
                            {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_CB_body as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as c_uint
                                != HPE_OK as c_int as c_uint)
                                as c_int
                                as c_long
                                != 0
                            {
                                return p.offset_from(data) as c_long as size_t;
                            }
                        }
                        body_mark = ::core::ptr::null::<c_char>();
                    }
                }
                63 => {
                    if body_mark.is_null() {
                        body_mark = p;
                    }
                    p = data
                        .offset(len as isize)
                        .offset(-(1 as c_int as isize));
                    current_block = 18435049525520518667;
                    break;
                }
                64 => {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as c_int
                            == HTTP_REQUEST as c_int
                        {
                            s_start_req as c_int
                        } else {
                            s_start_res as c_int
                        }
                    } else {
                        s_dead as c_int
                    }) as state;
                    '_c2rust_label_20: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1989 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if (*settings).on_message_complete.is_some() as c_int
                        as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings)
                                .on_message_complete
                                .expect("non-null function pointer")(
                                parser
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_message_complete as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                    if (*parser).upgrade() != 0 {
                        (*parser).nread = nread;
                        (*parser).set_state(p_state as c_uint as c_uint);
                        return (p.offset_from(data) as c_long
                            + 1 as c_long) as size_t;
                    }
                    current_block = 18435049525520518667;
                    break;
                }
                53 => {
                    '_c2rust_label_21: {
                        if nread == 1 as uint32_t {
                        } else {
                            __assert_fail(
                                b"nread == 1\0" as *const u8 as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1998 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    '_c2rust_label_22: {
                        if (*parser).flags() as c_int & F_CHUNKED as c_int
                            != 0
                        {
                        } else {
                            __assert_fail(
                                b"parser->flags & F_CHUNKED\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1999 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    unhex_val = unhex[ch as c_uchar as usize];
                    if (unhex_val as c_int == -(1 as c_int))
                        as c_int as c_long
                        != 0
                    {
                        current_block = 12052677347083126924;
                        break;
                    } else {
                        current_block = 15266614649349031085;
                        break;
                    }
                }
                54 => {
                    t_0 = 0;
                    '_c2rust_label_23: {
                        if (*parser).flags() as c_int & F_CHUNKED as c_int
                            != 0
                        {
                        } else {
                            __assert_fail(
                                b"parser->flags & F_CHUNKED\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                2016 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if ch as c_int == CR {
                        current_block = 15050254025307473385;
                        break;
                    } else {
                        current_block = 11817750501484884798;
                        break;
                    }
                }
                55 => {
                    '_c2rust_label_24: {
                        if (*parser).flags() as c_int & F_CHUNKED as c_int
                            != 0
                        {
                        } else {
                            __assert_fail(
                                b"parser->flags & F_CHUNKED\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                2051 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if ch as c_int == CR {
                        current_block = 16218741430612044173;
                        break;
                    } else {
                        current_block = 18435049525520518667;
                        break;
                    }
                }
                56 => {
                    '_c2rust_label_25: {
                        if (*parser).flags() as c_int & F_CHUNKED as c_int
                            != 0
                        {
                        } else {
                            __assert_fail(
                                b"parser->flags & F_CHUNKED\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                2062 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if ch as c_int != '\n' as i32 {
                        current_block = 11297993845810241135;
                        break;
                    } else {
                        current_block = 1790281523818446932;
                        break;
                    }
                }
                59 => {
                    let mut to_read_0: uint64_t = if (*parser).content_length
                        < data.offset(len as isize).offset_from(p) as c_long
                            as uint64_t
                    {
                        (*parser).content_length
                    } else {
                        data.offset(len as isize).offset_from(p) as c_long as uint64_t
                    };
                    '_c2rust_label_27: {
                        if (*parser).flags() as c_int & F_CHUNKED as c_int
                            != 0
                        {
                        } else {
                            __assert_fail(
                                b"parser->flags & F_CHUNKED\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                2083 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    '_c2rust_label_28: {
                        if (*parser).content_length != 0 as uint64_t
                            && (*parser).content_length as c_ulonglong
                                != (9223372036854775807 as c_ulonglong)
                                    .wrapping_mul(2 as c_ulonglong)
                                    .wrapping_add(1 as c_ulonglong)
                        {
                        } else {
                            __assert_fail(
                                b"parser->content_length != 0 && parser->content_length != ULLONG_MAX\0"
                                    as *const u8 as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                2085 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if body_mark.is_null() {
                        body_mark = p;
                    }
                    (*parser).content_length = (*parser).content_length.wrapping_sub(to_read_0);
                    p = p.offset(to_read_0.wrapping_sub(1 as uint64_t) as isize);
                    if (*parser).content_length == 0 as uint64_t {
                        p_state = s_chunk_data_almost_done;
                    }
                    current_block = 18435049525520518667;
                    break;
                }
                60 => {
                    '_c2rust_label_29: {
                        if (*parser).flags() as c_int & F_CHUNKED as c_int
                            != 0
                        {
                        } else {
                            __assert_fail(
                                b"parser->flags & F_CHUNKED\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                2102 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    '_c2rust_label_30: {
                        if (*parser).content_length == 0 as uint64_t {
                        } else {
                            __assert_fail(
                                b"parser->content_length == 0\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                2103 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if ch as c_int != '\r' as i32 {
                        current_block = 6486242724437840664;
                        break;
                    } else {
                        current_block = 5452410253409494665;
                        break;
                    }
                }
                61 => {
                    '_c2rust_label_32: {
                        if (*parser).flags() as c_int & F_CHUNKED as c_int
                            != 0
                        {
                        } else {
                            __assert_fail(
                                b"parser->flags & F_CHUNKED\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                2110 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if ch as c_int != '\n' as i32 {
                        current_block = 1746269980923986677;
                        break;
                    } else {
                        current_block = 17447106894017795187;
                        break;
                    }
                }
                _ => {
                    '_c2rust_label_34: {
                        if 0 as c_int != 0
                            && !(b"unhandled state\0" as *const u8 as *const c_char)
                                .is_null()
                        {
                        } else {
                            __assert_fail(
                                b"0 && \"unhandled state\"\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                2119 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_INTERNAL_STATE as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break 's_207;
                }
            }
        }
        match current_block {
            15266614649349031085 => {
                (*parser).content_length = unhex_val as uint64_t;
                p_state = s_chunk_size;
                current_block = 18435049525520518667;
            }
            15753004637962686906 => {
                (*parser).nread = 0 as uint32_t;
                nread = 0 as uint32_t;
                hasBody =
                    ((*parser).flags() as c_int & F_CHUNKED as c_int != 0
                        || (*parser).content_length > 0 as uint64_t
                            && (*parser).content_length as c_ulonglong != ULLONG_MAX)
                        as c_int;
                if (*parser).upgrade() as c_int != 0
                    && ((*parser).method() as c_int
                        == HTTP_CONNECT as c_int
                        || (*parser).flags() as c_int
                            & F_SKIPBODY as c_int
                            != 0
                        || hasBody == 0)
                {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as c_int
                            == HTTP_REQUEST as c_int
                        {
                            s_start_req as c_int
                        } else {
                            s_start_res as c_int
                        }
                    } else {
                        s_dead as c_int
                    }) as state;
                    '_c2rust_label_14: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1888 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if (*settings).on_message_complete.is_some() as c_int
                        as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings)
                                .on_message_complete
                                .expect("non-null function pointer")(
                                parser
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_message_complete as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                    (*parser).nread = nread;
                    (*parser).set_state(p_state as c_uint as c_uint);
                    return (p.offset_from(data) as c_long + 1 as c_long)
                        as size_t;
                }
                if (*parser).flags() as c_int & F_SKIPBODY as c_int != 0 {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as c_int
                            == HTTP_REQUEST as c_int
                        {
                            s_start_req as c_int
                        } else {
                            s_start_res as c_int
                        }
                    } else {
                        s_dead as c_int
                    }) as state;
                    '_c2rust_label_15: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1894 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if (*settings).on_message_complete.is_some() as c_int
                        as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings)
                                .on_message_complete
                                .expect("non-null function pointer")(
                                parser
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_message_complete as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                } else if (*parser).flags() as c_int & F_CHUNKED as c_int
                    != 0
                {
                    p_state = s_chunk_size_start;
                } else if (*parser).uses_transfer_encoding() as c_int
                    == 1 as c_int
                {
                    if (*parser).type_0() as c_int
                        == HTTP_REQUEST as c_int
                        && lenient == 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_INVALID_TRANSFER_ENCODING as c_int
                                as c_uint
                                as c_uint,
                        );
                        (*parser).nread = nread;
                        (*parser).set_state(p_state as c_uint as c_uint);
                        return p.offset_from(data) as c_long as size_t;
                    } else {
                        p_state = s_body_identity_eof;
                    }
                } else if (*parser).content_length == 0 as uint64_t {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as c_int
                            == HTTP_REQUEST as c_int
                        {
                            s_start_req as c_int
                        } else {
                            s_start_res as c_int
                        }
                    } else {
                        s_dead as c_int
                    }) as state;
                    '_c2rust_label_16: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1925 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if (*settings).on_message_complete.is_some() as c_int
                        as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings)
                                .on_message_complete
                                .expect("non-null function pointer")(
                                parser
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_message_complete as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                } else if (*parser).content_length as c_ulonglong != ULLONG_MAX {
                    p_state = s_body_identity;
                } else if http_message_needs_eof(parser) == 0 {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as c_int
                            == HTTP_REQUEST as c_int
                        {
                            s_start_req as c_int
                        } else {
                            s_start_res as c_int
                        }
                    } else {
                        s_dead as c_int
                    }) as state;
                    '_c2rust_label_17: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1933 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if (*settings).on_message_complete.is_some() as c_int
                        as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings)
                                .on_message_complete
                                .expect("non-null function pointer")(
                                parser
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_message_complete as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                } else {
                    p_state = s_body_identity_eof;
                }
                current_block = 18435049525520518667;
            }
            2468366247994693734 => {
                (*parser).set_header_state(h_state as c_uint as c_uint);
                if p == data.offset(len as isize) {
                    p = p.offset(-1);
                }
                nread =
                    nread.wrapping_add(p.offset_from(start_0) as c_long as uint32_t);
                if (nread > max_header_size) as c_int as c_long != 0 {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_HEADER_OVERFLOW as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                } else {
                    current_block = 18435049525520518667;
                }
            }
            1790281523818446932 => {
                (*parser).nread = 0 as uint32_t;
                nread = 0 as uint32_t;
                if (*parser).content_length == 0 as uint64_t {
                    (*parser).set_flags(
                        (*parser).flags() | F_TRAILING as c_int as c_uint,
                    );
                    p_state = s_header_field_start;
                } else {
                    p_state = s_chunk_data;
                }
                '_c2rust_label_26: {
                    if (*parser).http_errno() as http_errno as c_uint
                        == HPE_OK as c_int as c_uint
                    {
                    } else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const c_char,
                            b"http_parser.c\0" as *const u8
                                as *const c_char,
                            2074 as c_uint,
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                as *const u8 as *const c_char,
                        );
                    }
                };
                if (*settings).on_chunk_header.is_some() as c_int
                    as c_long
                    != 0
                {
                    (*parser).set_state(p_state as c_uint as c_uint);
                    if (0 as c_int
                        != (*settings)
                            .on_chunk_header
                            .expect("non-null function pointer")(parser))
                        as c_int as c_long
                        != 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_CB_chunk_header as c_int as c_uint
                                as c_uint,
                        );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as c_uint
                        != HPE_OK as c_int as c_uint)
                        as c_int as c_long
                        != 0
                    {
                        return (p.offset_from(data) as c_long
                            + 1 as c_long) as size_t;
                    }
                }
                current_block = 18435049525520518667;
            }
            13620783514648996490 => {
                if ch as c_int == CR {
                    p_state = s_header_value_discard_ws_almost_done;
                    current_block = 18435049525520518667;
                } else if ch as c_int == LF {
                    p_state = s_header_value_discard_lws;
                    current_block = 18435049525520518667;
                } else {
                    current_block = 7825823688640729701;
                }
            }
            5570469760270364154 => {
                nread = nread.wrapping_add(p.offset_from(start) as c_long as uint32_t);
                if (nread > max_header_size) as c_int as c_long != 0 {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_HEADER_OVERFLOW as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                } else if ch as c_int == ':' as i32 {
                    p_state = s_header_value_discard_ws;
                    '_c2rust_label_8: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                1385 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if !header_field_mark.is_null() {
                        if (*settings).on_header_field.is_some() as c_int
                            as c_long
                            != 0
                        {
                            (*parser)
                                .set_state(p_state as c_uint as c_uint);
                            if (0 as c_int
                                != (*settings)
                                    .on_header_field
                                    .expect("non-null function pointer")(
                                    parser,
                                    header_field_mark,
                                    p.offset_from(header_field_mark) as c_long
                                        as size_t,
                                )) as c_int
                                as c_long
                                != 0
                            {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_CB_header_field as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as c_uint
                                != HPE_OK as c_int as c_uint)
                                as c_int
                                as c_long
                                != 0
                            {
                                return (p.offset_from(data) as c_long
                                    + 1 as c_long)
                                    as size_t;
                            }
                        }
                        header_field_mark = ::core::ptr::null::<c_char>();
                    }
                } else {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_HEADER_TOKEN as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                }
                current_block = 18435049525520518667;
            }
            1795228135880752216 => {
                p = p.offset(-1);
                nread = nread.wrapping_add(p.offset_from(start) as c_long as uint32_t);
                if (nread > max_header_size) as c_int as c_long != 0 {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_HEADER_OVERFLOW as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                } else {
                    current_block = 18435049525520518667;
                }
            }
            7766894571154031170 => {
                if header_field_mark.is_null() {
                    header_field_mark = p;
                }
                (*parser).set_index(0 as c_uint as c_uint);
                p_state = s_header_field;
                match c as c_int {
                    99 => {
                        (*parser).set_header_state(
                            h_C as c_int as c_uint as c_uint,
                        );
                    }
                    112 => {
                        (*parser).set_header_state(
                            h_matching_proxy_connection as c_int as c_uint
                                as c_uint,
                        );
                    }
                    116 => {
                        (*parser).set_header_state(
                            h_matching_transfer_encoding as c_int
                                as c_uint
                                as c_uint,
                        );
                    }
                    117 => {
                        (*parser).set_header_state(
                            h_matching_upgrade as c_int as c_uint
                                as c_uint,
                        );
                    }
                    _ => {
                        (*parser).set_header_state(
                            h_general as c_int as c_uint
                                as c_uint,
                        );
                    }
                }
                current_block = 18435049525520518667;
            }
            18060968742384277177 => {
                if ch as c_int == LF {
                    p_state = s_header_field_start;
                } else {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_VERSION as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                }
                current_block = 18435049525520518667;
            }
            6363466913441788494 => {
                (*parser).http_minor =
                    (ch as c_int - '0' as i32) as c_ushort;
                p_state = s_req_http_end;
                current_block = 18435049525520518667;
            }
            12304281332906078777 => {
                (*parser).http_major =
                    (ch as c_int - '0' as i32) as c_ushort;
                p_state = s_req_http_dot;
                current_block = 18435049525520518667;
            }
            12770807392901873815 => {
                if (*parser).method() as c_int == HTTP_SOURCE as c_int {
                    p_state = s_req_http_I;
                    current_block = 18435049525520518667;
                } else {
                    current_block = 16199754411564412847;
                }
            }
            9878511696400499081 => {
                p_state = parse_url_char(p_state, ch);
                if (p_state as c_uint
                    == s_dead as c_int as c_uint)
                    as c_int as c_long
                    != 0
                {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_URL as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                } else {
                    current_block = 18435049525520518667;
                }
            }
            246505757814604223 => {
                (*parser).http_major = 0 as c_ushort;
                (*parser).http_minor = 9 as c_ushort;
                p_state = (if ch as c_int == '\r' as i32 {
                    s_req_line_almost_done as c_int
                } else {
                    s_header_field_start as c_int
                }) as state;
                '_c2rust_label_6: {
                    if (*parser).http_errno() as http_errno as c_uint
                        == HPE_OK as c_int as c_uint
                    {
                    } else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const c_char,
                            b"http_parser.c\0" as *const u8
                                as *const c_char,
                            1085 as c_uint,
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                as *const u8 as *const c_char,
                        );
                    }
                };
                if !url_mark.is_null() {
                    if (*settings).on_url.is_some() as c_int as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings).on_url.expect("non-null function pointer")(
                                parser,
                                url_mark,
                                p.offset_from(url_mark) as c_long as size_t,
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_url as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                    url_mark = ::core::ptr::null::<c_char>();
                }
                current_block = 18435049525520518667;
            }
            15853744263182998232 => {
                p_state = s_req_http_start;
                '_c2rust_label_5: {
                    if (*parser).http_errno() as http_errno as c_uint
                        == HPE_OK as c_int as c_uint
                    {
                    } else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const c_char,
                            b"http_parser.c\0" as *const u8
                                as *const c_char,
                            1076 as c_uint,
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                as *const u8 as *const c_char,
                        );
                    }
                };
                if !url_mark.is_null() {
                    if (*settings).on_url.is_some() as c_int as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings).on_url.expect("non-null function pointer")(
                                parser,
                                url_mark,
                                p.offset_from(url_mark) as c_long as size_t,
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_url as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                    url_mark = ::core::ptr::null::<c_char>();
                }
                current_block = 18435049525520518667;
            }
            9091570581087047569 => {
                p_state = parse_url_char(p_state, ch);
                if (p_state as c_uint
                    == s_dead as c_int as c_uint)
                    as c_int as c_long
                    != 0
                {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_URL as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                } else {
                    current_block = 18435049525520518667;
                }
            }
            13755523488868872559 => {
                if url_mark.is_null() {
                    url_mark = p;
                }
                if (*parser).method() as c_int == HTTP_CONNECT as c_int {
                    p_state = s_req_server_start;
                }
                p_state = parse_url_char(p_state, ch);
                if (p_state as c_uint
                    == s_dead as c_int as c_uint)
                    as c_int as c_long
                    != 0
                {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_URL as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                } else {
                    current_block = 18435049525520518667;
                }
            }
            6341332686669895749 => {
                matcher = method_strings[(*parser).method() as usize];
                if ch as c_int == ' ' as i32
                    && *matcher.offset((*parser).index() as isize) as c_int
                        == '\0' as i32
                {
                    p_state = s_req_spaces_before_url;
                } else if !(ch as c_int
                    == *matcher.offset((*parser).index() as isize) as c_int)
                {
                    if ch as c_int >= 'A' as i32
                        && ch as c_int <= 'Z' as i32
                        || ch as c_int == '-' as i32
                    {
                        match ((*parser).method() as c_int) << 16 as c_int
                            | ((*parser).index() as c_int) << 8 as c_int
                            | ch as c_int
                        {
                            196949 => {
                                (*parser).set_method(
                                    HTTP_PUT as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            196929 => {
                                (*parser).set_method(
                                    HTTP_PATCH as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            196946 => {
                                (*parser).set_method(
                                    HTTP_PROPFIND as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            262738 => {
                                (*parser).set_method(
                                    HTTP_PURGE as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            328008 => {
                                (*parser).set_method(
                                    HTTP_CHECKOUT as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            328272 => {
                                (*parser).set_method(
                                    HTTP_COPY as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            655695 => {
                                (*parser).set_method(
                                    HTTP_MOVE as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            655685 => {
                                (*parser).set_method(
                                    HTTP_MERGE as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            655661 => {
                                (*parser).set_method(
                                    HTTP_MSEARCH as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            655937 => {
                                (*parser).set_method(
                                    HTTP_MKACTIVITY as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            656193 => {
                                (*parser).set_method(
                                    HTTP_MKCALENDAR as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            1704261 => {
                                (*parser).set_method(
                                    HTTP_SEARCH as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            1704271 => {
                                (*parser).set_method(
                                    HTTP_SOURCE as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            1311298 => {
                                (*parser).set_method(
                                    HTTP_REBIND as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            787536 => {
                                (*parser).set_method(
                                    HTTP_PROPPATCH as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            590153 => {
                                (*parser).set_method(
                                    HTTP_LINK as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            983635 => {
                                (*parser).set_method(
                                    HTTP_UNSUBSCRIBE as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            983618 => {
                                (*parser).set_method(
                                    HTTP_UNBIND as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            983881 => {
                                (*parser).set_method(
                                    HTTP_UNLINK as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            _ => {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_INVALID_METHOD as c_int as c_uint
                                        as c_uint,
                                );
                                current_block = 13067290550027806311;
                                break;
                            }
                        }
                    } else {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_INVALID_METHOD as c_int as c_uint
                                as c_uint,
                        );
                        current_block = 13067290550027806311;
                        break;
                    }
                }
                (*parser).set_index((*parser).index() + 1 as c_uint);
                current_block = 18435049525520518667;
            }
            11817750501484884798 => {
                unhex_val = unhex[ch as c_uchar as usize];
                if unhex_val as c_int == -(1 as c_int) {
                    if ch as c_int == ';' as i32
                        || ch as c_int == ' ' as i32
                    {
                        p_state = s_chunk_parameters;
                    } else {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_INVALID_CHUNK_SIZE as c_int as c_uint
                                as c_uint,
                        );
                        current_block = 13067290550027806311;
                        break;
                    }
                } else {
                    t_0 = (*parser).content_length;
                    t_0 = t_0.wrapping_mul(16 as uint64_t);
                    t_0 = t_0.wrapping_add(unhex_val as uint64_t);
                    if ((9223372036854775807 as c_ulonglong)
                        .wrapping_mul(2 as c_ulonglong)
                        .wrapping_add(1 as c_ulonglong)
                        .wrapping_sub(16 as c_ulonglong)
                        .wrapping_div(16 as c_ulonglong)
                        < (*parser).content_length as c_ulonglong)
                        as c_int as c_long
                        != 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_INVALID_CONTENT_LENGTH as c_int as c_uint
                                as c_uint,
                        );
                        current_block = 13067290550027806311;
                        break;
                    } else {
                        (*parser).content_length = t_0;
                    }
                }
                current_block = 18435049525520518667;
            }
            12853382357061961387 => {
                (*parser).set_flags(0 as c_uint as c_uint);
                (*parser)
                    .set_uses_transfer_encoding(0 as c_uint as c_uint);
                (*parser).content_length = ULLONG_MAX as uint64_t;
                if !((ch as c_int | 0x20 as c_int) as c_uchar
                    as c_int
                    >= 'a' as i32
                    && (ch as c_int | 0x20 as c_int)
                        as c_uchar as c_int
                        <= 'z' as i32) as c_int
                    as c_long
                    != 0
                {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_METHOD as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                } else {
                    (*parser).set_method(HTTP_DELETE as c_uint as c_uint);
                    (*parser).set_index(1 as c_uint as c_uint);
                    match ch as c_int {
                        65 => {
                            (*parser).set_method(
                                HTTP_ACL as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        66 => {
                            (*parser).set_method(
                                HTTP_BIND as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        67 => {
                            (*parser).set_method(
                                HTTP_CONNECT as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        68 => {
                            (*parser).set_method(
                                HTTP_DELETE as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        71 => {
                            (*parser).set_method(
                                HTTP_GET as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        72 => {
                            (*parser).set_method(
                                HTTP_HEAD as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        76 => {
                            (*parser).set_method(
                                HTTP_LOCK as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        77 => {
                            (*parser).set_method(
                                HTTP_MKCOL as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        78 => {
                            (*parser).set_method(
                                HTTP_NOTIFY as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        79 => {
                            (*parser).set_method(
                                HTTP_OPTIONS as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        80 => {
                            (*parser).set_method(
                                HTTP_POST as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        82 => {
                            (*parser).set_method(
                                HTTP_REPORT as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        83 => {
                            (*parser).set_method(
                                HTTP_SUBSCRIBE as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        84 => {
                            (*parser).set_method(
                                HTTP_TRACE as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        85 => {
                            (*parser).set_method(
                                HTTP_UNLOCK as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        _ => {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_INVALID_METHOD as c_int as c_uint
                                    as c_uint,
                            );
                            current_block = 13067290550027806311;
                            break;
                        }
                    }
                    p_state = s_req_method;
                    '_c2rust_label_4: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                966 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if (*settings).on_message_begin.is_some() as c_int
                        as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings)
                                .on_message_begin
                                .expect("non-null function pointer")(
                                parser
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_message_begin as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                }
                current_block = 18435049525520518667;
            }
            15609529146834799275 => {
                if ch as c_int == LF {
                    p_state = s_header_field_start;
                    '_c2rust_label_3: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                916 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if !status_mark.is_null() {
                        if (*settings).on_status.is_some() as c_int
                            as c_long
                            != 0
                        {
                            (*parser)
                                .set_state(p_state as c_uint as c_uint);
                            if (0 as c_int
                                != (*settings).on_status.expect("non-null function pointer")(
                                    parser,
                                    status_mark,
                                    p.offset_from(status_mark) as c_long as size_t,
                                )) as c_int
                                as c_long
                                != 0
                            {
                                (*parser).nread = nread;
                                (*parser).set_http_errno(
                                    HPE_CB_status as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as c_uint
                                != HPE_OK as c_int as c_uint)
                                as c_int
                                as c_long
                                != 0
                            {
                                return (p.offset_from(data) as c_long
                                    + 1 as c_long)
                                    as size_t;
                            }
                        }
                        status_mark = ::core::ptr::null::<c_char>();
                    }
                    current_block = 18435049525520518667;
                } else {
                    current_block = 18435049525520518667;
                }
            }
            7188795011561844502 => {
                p_state = s_res_line_almost_done;
                '_c2rust_label_2: {
                    if (*parser).http_errno() as http_errno as c_uint
                        == HPE_OK as c_int as c_uint
                    {
                    } else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const c_char,
                            b"http_parser.c\0" as *const u8
                                as *const c_char,
                            910 as c_uint,
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                as *const u8 as *const c_char,
                        );
                    }
                };
                if !status_mark.is_null() {
                    if (*settings).on_status.is_some() as c_int as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings).on_status.expect("non-null function pointer")(
                                parser,
                                status_mark,
                                p.offset_from(status_mark) as c_long as size_t,
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_status as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                    status_mark = ::core::ptr::null::<c_char>();
                }
                current_block = 18435049525520518667;
            }
            16512738885216853798 => {
                (*parser).set_status_code(
                    (ch as c_int - '0' as i32) as c_uint
                        as c_uint,
                );
                p_state = s_res_status_code;
                current_block = 18435049525520518667;
            }
            3813860224257983916 => {
                if ch as c_int == ' ' as i32 {
                    current_block = 18435049525520518667;
                } else {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_STATUS as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                }
            }
            9240481512215375588 => {
                (*parser).http_minor =
                    (ch as c_int - '0' as i32) as c_ushort;
                p_state = s_res_http_end;
                current_block = 18435049525520518667;
            }
            6938158527927677584 => {
                (*parser).http_major =
                    (ch as c_int - '0' as i32) as c_ushort;
                p_state = s_res_http_dot;
                current_block = 18435049525520518667;
            }
            3634396408142324656 => {
                (*parser).set_flags(0 as c_uint as c_uint);
                (*parser)
                    .set_uses_transfer_encoding(0 as c_uint as c_uint);
                (*parser).content_length = ULLONG_MAX as uint64_t;
                if ch as c_int == 'H' as i32 {
                    p_state = s_res_H;
                    '_c2rust_label_1: {
                        if (*parser).http_errno() as http_errno as c_uint
                            == HPE_OK as c_int as c_uint
                        {
                        } else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const c_char,
                                b"http_parser.c\0" as *const u8
                                    as *const c_char,
                                784 as c_uint,
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                    as *const u8 as *const c_char,
                            );
                        }
                    };
                    if (*settings).on_message_begin.is_some() as c_int
                        as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings)
                                .on_message_begin
                                .expect("non-null function pointer")(
                                parser
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_message_begin as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                } else {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_CONSTANT as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                }
                current_block = 18435049525520518667;
            }
            14648606000749551097 => {
                if (ch as c_int != 'E' as i32) as c_int
                    as c_long
                    != 0
                {
                    (*parser).nread = nread;
                    (*parser).set_http_errno(
                        HPE_INVALID_CONSTANT as c_int as c_uint
                            as c_uint,
                    );
                    current_block = 13067290550027806311;
                    break;
                } else {
                    (*parser).set_type_0(
                        HTTP_REQUEST as c_int as c_uint
                            as c_uint,
                    );
                    (*parser).set_method(
                        HTTP_HEAD as c_int as c_uint
                            as c_uint,
                    );
                    (*parser).set_index(2 as c_uint as c_uint);
                    p_state = s_req_method;
                }
                current_block = 18435049525520518667;
            }
            5452410253409494665 => {
                p_state = s_chunk_data_done;
                '_c2rust_label_31: {
                    if (*parser).http_errno() as http_errno as c_uint
                        == HPE_OK as c_int as c_uint
                    {
                    } else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const c_char,
                            b"http_parser.c\0" as *const u8
                                as *const c_char,
                            2106 as c_uint,
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                as *const u8 as *const c_char,
                        );
                    }
                };
                if !body_mark.is_null() {
                    if (*settings).on_body.is_some() as c_int as c_long
                        != 0
                    {
                        (*parser).set_state(p_state as c_uint as c_uint);
                        if (0 as c_int
                            != (*settings).on_body.expect("non-null function pointer")(
                                parser,
                                body_mark,
                                p.offset_from(body_mark) as c_long as size_t,
                            )) as c_int
                            as c_long
                            != 0
                        {
                            (*parser).nread = nread;
                            (*parser).set_http_errno(
                                HPE_CB_body as c_int as c_uint
                                    as c_uint,
                            );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as c_uint
                            != HPE_OK as c_int as c_uint)
                            as c_int as c_long
                            != 0
                        {
                            return (p.offset_from(data) as c_long
                                + 1 as c_long)
                                as size_t;
                        }
                    }
                    body_mark = ::core::ptr::null::<c_char>();
                }
                current_block = 18435049525520518667;
            }
            17447106894017795187 => {
                (*parser).nread = 0 as uint32_t;
                nread = 0 as uint32_t;
                p_state = s_chunk_size_start;
                '_c2rust_label_33: {
                    if (*parser).http_errno() as http_errno as c_uint
                        == HPE_OK as c_int as c_uint
                    {
                    } else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const c_char,
                            b"http_parser.c\0" as *const u8
                                as *const c_char,
                            2115 as c_uint,
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                                as *const u8 as *const c_char,
                        );
                    }
                };
                if (*settings).on_chunk_complete.is_some() as c_int
                    as c_long
                    != 0
                {
                    (*parser).set_state(p_state as c_uint as c_uint);
                    if (0 as c_int
                        != (*settings)
                            .on_chunk_complete
                            .expect("non-null function pointer")(parser))
                        as c_int as c_long
                        != 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_CB_chunk_complete as c_int as c_uint
                                as c_uint,
                        );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as c_uint
                        != HPE_OK as c_int as c_uint)
                        as c_int as c_long
                        != 0
                    {
                        return (p.offset_from(data) as c_long
                            + 1 as c_long) as size_t;
                    }
                }
                current_block = 18435049525520518667;
            }
            7460542724658431689 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            13321564401369230990 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_CLOSED_CONNECTION as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            15417752026496523887 => {
                p_state = s_res_HT;
                current_block = 18435049525520518667;
            }
            13863458367724794628 => {
                p_state = s_res_HTT;
                current_block = 18435049525520518667;
            }
            9343041660989783267 => {
                p_state = s_res_HTTP;
                current_block = 18435049525520518667;
            }
            5250576585193495047 => {
                p_state = s_res_http_major;
                current_block = 18435049525520518667;
            }
            14874642226861704653 => {
                p_state = s_res_http_minor;
                current_block = 18435049525520518667;
            }
            9235179519944561532 => {
                p_state = s_res_first_status_code;
                current_block = 18435049525520518667;
            }
            9028266288740425872 => {
                p_state = s_header_field_start;
                current_block = 18435049525520518667;
            }
            8889999340123292593 => {
                p_state = s_req_http_HT;
                current_block = 18435049525520518667;
            }
            588672170829148014 => {
                p_state = s_req_http_HTT;
                current_block = 18435049525520518667;
            }
            9181201145437202872 => {
                p_state = s_req_http_HTTP;
                current_block = 18435049525520518667;
            }
            10530107118073347621 => {
                p_state = s_req_http_IC;
                current_block = 18435049525520518667;
            }
            5271113972066554886 => {
                p_state = s_req_http_HTTP;
                current_block = 18435049525520518667;
            }
            12952730798101289484 => {
                p_state = s_req_http_major;
                current_block = 18435049525520518667;
            }
            11644938206506967943 => {
                p_state = s_req_http_minor;
                current_block = 18435049525520518667;
            }
            13016604015400381741 => {
                p_state = s_header_field_start;
                current_block = 18435049525520518667;
            }
            14641528508070947041 => {
                p_state = s_header_value_lws;
                current_block = 18435049525520518667;
            }
            2164532986857606317 => {
                p_state = s_header_value_discard_lws;
                current_block = 18435049525520518667;
            }
            11707042610706539144 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            12609744167958600007 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            16218741430612044173 => {
                p_state = s_chunk_size_almost_done;
                current_block = 18435049525520518667;
            }
            11297993845810241135 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            1746269980923986677 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            6486242724437840664 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            11577926782275222206 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_STATUS as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            15050254025307473385 => {
                p_state = s_chunk_size_almost_done;
                current_block = 18435049525520518667;
            }
            12052677347083126924 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_CHUNK_SIZE as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            16854099570775812386 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_LF_EXPECTED as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            15266942463618272814 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_HEADER_TOKEN as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            3145106269552242860 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_LF_EXPECTED as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            4265637299601780024 => {
                p_state = s_req_line_almost_done;
                current_block = 18435049525520518667;
            }
            16591777222291248247 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_VERSION as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            6452958575495017105 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_VERSION as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            9532072207990609994 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_VERSION as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            13845568614582207489 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            16653817520844849459 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            17918532803028278290 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            17485376261910781866 => {
                (*parser).set_type_0(
                    HTTP_RESPONSE as c_int as c_uint
                        as c_uint,
                );
                p_state = s_res_HT;
                current_block = 18435049525520518667;
            }
            1013506999122146761 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            9430418855388998878 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            7923086311623215889 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            7545150590528655645 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            4299703460566765016 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_VERSION as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            5388205036907793036 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_VERSION as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            11226769033371074123 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_VERSION as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            3098209481605707636 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_VERSION as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            16832305905353653446 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            7926734633677835471 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_METHOD as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            8953125900534742068 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_URL as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            12168134392480497817 => {
                p_state = s_req_http_H;
                current_block = 18435049525520518667;
            }
            3503188808869013853 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            5800290220634814141 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_STRICT as c_int as c_uint as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            _ => {}
        }
        match current_block {
            7825823688640729701 => {
                if header_value_mark.is_null() {
                    header_value_mark = p;
                }
                p_state = s_header_value;
                (*parser).set_index(0 as c_uint as c_uint);
                c = (ch as c_int | 0x20 as c_int) as c_uchar
                    as c_char;
                match (*parser).header_state() as c_int {
                    14 => {
                        current_block = 13780492444601646647;
                        match current_block {
                            14262213187466411498 => {
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            13780492444601646647 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_UPGRADE as c_int as c_uint,
                                );
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            1009264062316411214 => {
                                if 'c' as i32 == c as c_int {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_chunked as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            16152950153032035779 => {
                                if c as c_int == 'k' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_keep_alive as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'c' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_close as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'u' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_upgrade as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_connection_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            _ => {
                                if !(ch as c_int >= '0' as i32
                                    && ch as c_int <= '9' as i32)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_INVALID_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else if (*parser).flags() as c_int
                                    & F_CONTENTLENGTH as c_int
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_UNEXPECTED_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else {
                                    (*parser).set_flags(
                                        (*parser).flags()
                                            | F_CONTENTLENGTH as c_int
                                                as c_uint,
                                    );
                                    (*parser).content_length =
                                        (ch as c_int - '0' as i32) as uint64_t;
                                    (*parser).set_header_state(
                                        h_content_length_num as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                        }
                    }
                    13 => {
                        current_block = 1009264062316411214;
                        match current_block {
                            14262213187466411498 => {
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            13780492444601646647 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_UPGRADE as c_int as c_uint,
                                );
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            1009264062316411214 => {
                                if 'c' as i32 == c as c_int {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_chunked as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            16152950153032035779 => {
                                if c as c_int == 'k' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_keep_alive as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'c' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_close as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'u' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_upgrade as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_connection_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            _ => {
                                if !(ch as c_int >= '0' as i32
                                    && ch as c_int <= '9' as i32)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_INVALID_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else if (*parser).flags() as c_int
                                    & F_CONTENTLENGTH as c_int
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_UNEXPECTED_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else {
                                    (*parser).set_flags(
                                        (*parser).flags()
                                            | F_CONTENTLENGTH as c_int
                                                as c_uint,
                                    );
                                    (*parser).content_length =
                                        (ch as c_int - '0' as i32) as uint64_t;
                                    (*parser).set_header_state(
                                        h_content_length_num as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                        }
                    }
                    10 => {
                        current_block = 17892630391282426523;
                        match current_block {
                            14262213187466411498 => {
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            13780492444601646647 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_UPGRADE as c_int as c_uint,
                                );
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            1009264062316411214 => {
                                if 'c' as i32 == c as c_int {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_chunked as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            16152950153032035779 => {
                                if c as c_int == 'k' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_keep_alive as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'c' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_close as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'u' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_upgrade as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_connection_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            _ => {
                                if !(ch as c_int >= '0' as i32
                                    && ch as c_int <= '9' as i32)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_INVALID_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else if (*parser).flags() as c_int
                                    & F_CONTENTLENGTH as c_int
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_UNEXPECTED_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else {
                                    (*parser).set_flags(
                                        (*parser).flags()
                                            | F_CONTENTLENGTH as c_int
                                                as c_uint,
                                    );
                                    (*parser).content_length =
                                        (ch as c_int - '0' as i32) as uint64_t;
                                    (*parser).set_header_state(
                                        h_content_length_num as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                        }
                    }
                    9 => {
                        current_block = 16152950153032035779;
                        match current_block {
                            14262213187466411498 => {
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            13780492444601646647 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_UPGRADE as c_int as c_uint,
                                );
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            1009264062316411214 => {
                                if 'c' as i32 == c as c_int {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_chunked as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            16152950153032035779 => {
                                if c as c_int == 'k' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_keep_alive as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'c' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_close as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'u' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_upgrade as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_connection_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            _ => {
                                if !(ch as c_int >= '0' as i32
                                    && ch as c_int <= '9' as i32)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_INVALID_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else if (*parser).flags() as c_int
                                    & F_CONTENTLENGTH as c_int
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_UNEXPECTED_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else {
                                    (*parser).set_flags(
                                        (*parser).flags()
                                            | F_CONTENTLENGTH as c_int
                                                as c_uint,
                                    );
                                    (*parser).content_length =
                                        (ch as c_int - '0' as i32) as uint64_t;
                                    (*parser).set_header_state(
                                        h_content_length_num as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                        }
                    }
                    15 | 12 | 18 => {}
                    _ => {
                        current_block = 14262213187466411498;
                        match current_block {
                            14262213187466411498 => {
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            13780492444601646647 => {
                                (*parser).set_flags(
                                    (*parser).flags()
                                        | F_UPGRADE as c_int as c_uint,
                                );
                                (*parser).set_header_state(
                                    h_general as c_int as c_uint
                                        as c_uint,
                                );
                            }
                            1009264062316411214 => {
                                if 'c' as i32 == c as c_int {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_chunked as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_transfer_encoding_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            16152950153032035779 => {
                                if c as c_int == 'k' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_keep_alive as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'c' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_close as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else if c as c_int == 'u' as i32 {
                                    (*parser).set_header_state(
                                        h_matching_connection_upgrade as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                } else {
                                    (*parser).set_header_state(
                                        h_matching_connection_token as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                            _ => {
                                if !(ch as c_int >= '0' as i32
                                    && ch as c_int <= '9' as i32)
                                    as c_int
                                    as c_long
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_INVALID_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else if (*parser).flags() as c_int
                                    & F_CONTENTLENGTH as c_int
                                    != 0
                                {
                                    (*parser).nread = nread;
                                    (*parser).set_http_errno(
                                        HPE_UNEXPECTED_CONTENT_LENGTH as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                    current_block = 13067290550027806311;
                                    break;
                                } else {
                                    (*parser).set_flags(
                                        (*parser).flags()
                                            | F_CONTENTLENGTH as c_int
                                                as c_uint,
                                    );
                                    (*parser).content_length =
                                        (ch as c_int - '0' as i32) as uint64_t;
                                    (*parser).set_header_state(
                                        h_content_length_num as c_int
                                            as c_uint
                                            as c_uint,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            16199754411564412847 => {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_INVALID_CONSTANT as c_int as c_uint
                        as c_uint,
                );
                current_block = 13067290550027806311;
                break;
            }
            _ => {}
        }
        p = p.offset(1);
    }
    match current_block {
        13067290550027806311 => {
            if (*parser).http_errno() as http_errno as c_uint
                == HPE_OK as c_int as c_uint
            {
                (*parser).nread = nread;
                (*parser).set_http_errno(
                    HPE_UNKNOWN as c_int as c_uint as c_uint,
                );
            }
            (*parser).nread = nread;
            (*parser).set_state(p_state as c_uint as c_uint);
            return p.offset_from(data) as c_long as size_t;
        }
        _ => {
            '_c2rust_label_35: {
                if (if !header_field_mark.is_null() {
                    1 as c_int
                } else {
                    0 as c_int
                }) + (if !header_value_mark.is_null() {
                    1 as c_int
                } else {
                    0 as c_int
                }) + (if !url_mark.is_null() {
                    1 as c_int
                } else {
                    0 as c_int
                }) + (if !body_mark.is_null() {
                    1 as c_int
                } else {
                    0 as c_int
                }) + (if !status_mark.is_null() {
                    1 as c_int
                } else {
                    0 as c_int
                }) <= 1 as c_int
                {
                } else {
                    __assert_fail(
                        b"((header_field_mark ? 1 : 0) + (header_value_mark ? 1 : 0) + (url_mark ? 1 : 0) + (body_mark ? 1 : 0) + (status_mark ? 1 : 0)) <= 1\0"
                            as *const u8 as *const c_char,
                        b"http_parser.c\0" as *const u8 as *const c_char,
                        2139 as c_uint,
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                            as *const u8 as *const c_char,
                    );
                }
            };
            '_c2rust_label_36: {
                if (*parser).http_errno() as http_errno as c_uint
                    == HPE_OK as c_int as c_uint
                {
                } else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const c_char,
                        b"http_parser.c\0" as *const u8 as *const c_char,
                        2141 as c_uint,
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                            as *const u8 as *const c_char,
                    );
                }
            };
            if !header_field_mark.is_null() {
                if (*settings).on_header_field.is_some() as c_int
                    as c_long
                    != 0
                {
                    (*parser).set_state(p_state as c_uint as c_uint);
                    if (0 as c_int
                        != (*settings)
                            .on_header_field
                            .expect("non-null function pointer")(
                            parser,
                            header_field_mark,
                            p.offset_from(header_field_mark) as c_long as size_t,
                        )) as c_int as c_long
                        != 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_CB_header_field as c_int as c_uint
                                as c_uint,
                        );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as c_uint
                        != HPE_OK as c_int as c_uint)
                        as c_int as c_long
                        != 0
                    {
                        return p.offset_from(data) as c_long as size_t;
                    }
                }
                header_field_mark = ::core::ptr::null::<c_char>();
            }
            '_c2rust_label_37: {
                if (*parser).http_errno() as http_errno as c_uint
                    == HPE_OK as c_int as c_uint
                {
                } else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const c_char,
                        b"http_parser.c\0" as *const u8 as *const c_char,
                        2142 as c_uint,
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                            as *const u8 as *const c_char,
                    );
                }
            };
            if !header_value_mark.is_null() {
                if (*settings).on_header_value.is_some() as c_int
                    as c_long
                    != 0
                {
                    (*parser).set_state(p_state as c_uint as c_uint);
                    if (0 as c_int
                        != (*settings)
                            .on_header_value
                            .expect("non-null function pointer")(
                            parser,
                            header_value_mark,
                            p.offset_from(header_value_mark) as c_long as size_t,
                        )) as c_int as c_long
                        != 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_CB_header_value as c_int as c_uint
                                as c_uint,
                        );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as c_uint
                        != HPE_OK as c_int as c_uint)
                        as c_int as c_long
                        != 0
                    {
                        return p.offset_from(data) as c_long as size_t;
                    }
                }
                header_value_mark = ::core::ptr::null::<c_char>();
            }
            '_c2rust_label_38: {
                if (*parser).http_errno() as http_errno as c_uint
                    == HPE_OK as c_int as c_uint
                {
                } else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const c_char,
                        b"http_parser.c\0" as *const u8 as *const c_char,
                        2143 as c_uint,
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                            as *const u8 as *const c_char,
                    );
                }
            };
            if !url_mark.is_null() {
                if (*settings).on_url.is_some() as c_int as c_long != 0 {
                    (*parser).set_state(p_state as c_uint as c_uint);
                    if (0 as c_int
                        != (*settings).on_url.expect("non-null function pointer")(
                            parser,
                            url_mark,
                            p.offset_from(url_mark) as c_long as size_t,
                        )) as c_int as c_long
                        != 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_CB_url as c_int as c_uint
                                as c_uint,
                        );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as c_uint
                        != HPE_OK as c_int as c_uint)
                        as c_int as c_long
                        != 0
                    {
                        return p.offset_from(data) as c_long as size_t;
                    }
                }
                url_mark = ::core::ptr::null::<c_char>();
            }
            '_c2rust_label_39: {
                if (*parser).http_errno() as http_errno as c_uint
                    == HPE_OK as c_int as c_uint
                {
                } else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const c_char,
                        b"http_parser.c\0" as *const u8 as *const c_char,
                        2144 as c_uint,
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                            as *const u8 as *const c_char,
                    );
                }
            };
            if !body_mark.is_null() {
                if (*settings).on_body.is_some() as c_int as c_long != 0 {
                    (*parser).set_state(p_state as c_uint as c_uint);
                    if (0 as c_int
                        != (*settings).on_body.expect("non-null function pointer")(
                            parser,
                            body_mark,
                            p.offset_from(body_mark) as c_long as size_t,
                        )) as c_int as c_long
                        != 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_CB_body as c_int as c_uint
                                as c_uint,
                        );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as c_uint
                        != HPE_OK as c_int as c_uint)
                        as c_int as c_long
                        != 0
                    {
                        return p.offset_from(data) as c_long as size_t;
                    }
                }
                body_mark = ::core::ptr::null::<c_char>();
            }
            '_c2rust_label_40: {
                if (*parser).http_errno() as http_errno as c_uint
                    == HPE_OK as c_int as c_uint
                {
                } else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const c_char,
                        b"http_parser.c\0" as *const u8 as *const c_char,
                        2145 as c_uint,
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0"
                            as *const u8 as *const c_char,
                    );
                }
            };
            if !status_mark.is_null() {
                if (*settings).on_status.is_some() as c_int as c_long != 0
                {
                    (*parser).set_state(p_state as c_uint as c_uint);
                    if (0 as c_int
                        != (*settings).on_status.expect("non-null function pointer")(
                            parser,
                            status_mark,
                            p.offset_from(status_mark) as c_long as size_t,
                        )) as c_int as c_long
                        != 0
                    {
                        (*parser).nread = nread;
                        (*parser).set_http_errno(
                            HPE_CB_status as c_int as c_uint
                                as c_uint,
                        );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as c_uint
                        != HPE_OK as c_int as c_uint)
                        as c_int as c_long
                        != 0
                    {
                        return p.offset_from(data) as c_long as size_t;
                    }
                }
                status_mark = ::core::ptr::null::<c_char>();
            }
            (*parser).nread = nread;
            (*parser).set_state(p_state as c_uint as c_uint);
            return len;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_message_needs_eof(
    mut parser: *const http_parser,
) -> c_int {
    if (*parser).type_0() as c_int == HTTP_REQUEST as c_int {
        return 0 as c_int;
    }
    if (*parser).status_code() as c_int / 100 as c_int
        == 1 as c_int
        || (*parser).status_code() as c_int == 204 as c_int
        || (*parser).status_code() as c_int == 304 as c_int
        || (*parser).flags() as c_int & F_SKIPBODY as c_int != 0
    {
        return 0 as c_int;
    }
    if (*parser).uses_transfer_encoding() as c_int == 1 as c_int
        && (*parser).flags() as c_int & F_CHUNKED as c_int
            == 0 as c_int
    {
        return 1 as c_int;
    }
    if (*parser).flags() as c_int & F_CHUNKED as c_int != 0
        || (*parser).content_length as c_ulonglong != ULLONG_MAX
    {
        return 0 as c_int;
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_should_keep_alive(
    mut parser: *const http_parser,
) -> c_int {
    if (*parser).http_major as c_int > 0 as c_int
        && (*parser).http_minor as c_int > 0 as c_int
    {
        if (*parser).flags() as c_int & F_CONNECTION_CLOSE as c_int != 0 {
            return 0 as c_int;
        }
    } else if (*parser).flags() as c_int
        & F_CONNECTION_KEEP_ALIVE as c_int
        == 0
    {
        return 0 as c_int;
    }
    return (http_message_needs_eof(parser) == 0) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_method_str(mut m: http_method) -> *const c_char {
    return if (m as c_uint as usize)
        < (::core::mem::size_of::<[*const c_char; 34]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const c_char>() as usize)
    {
        method_strings[m as usize]
    } else {
        b"<unknown>\0" as *const u8 as *const c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_status_str(mut s: http_status) -> *const c_char {
    match s as c_uint {
        100 => return b"Continue\0" as *const u8 as *const c_char,
        101 => return b"Switching Protocols\0" as *const u8 as *const c_char,
        102 => return b"Processing\0" as *const u8 as *const c_char,
        200 => return b"OK\0" as *const u8 as *const c_char,
        201 => return b"Created\0" as *const u8 as *const c_char,
        202 => return b"Accepted\0" as *const u8 as *const c_char,
        203 => {
            return b"Non-Authoritative Information\0" as *const u8 as *const c_char;
        }
        204 => return b"No Content\0" as *const u8 as *const c_char,
        205 => return b"Reset Content\0" as *const u8 as *const c_char,
        206 => return b"Partial Content\0" as *const u8 as *const c_char,
        207 => return b"Multi-Status\0" as *const u8 as *const c_char,
        208 => return b"Already Reported\0" as *const u8 as *const c_char,
        226 => return b"IM Used\0" as *const u8 as *const c_char,
        300 => return b"Multiple Choices\0" as *const u8 as *const c_char,
        301 => return b"Moved Permanently\0" as *const u8 as *const c_char,
        302 => return b"Found\0" as *const u8 as *const c_char,
        303 => return b"See Other\0" as *const u8 as *const c_char,
        304 => return b"Not Modified\0" as *const u8 as *const c_char,
        305 => return b"Use Proxy\0" as *const u8 as *const c_char,
        307 => return b"Temporary Redirect\0" as *const u8 as *const c_char,
        308 => return b"Permanent Redirect\0" as *const u8 as *const c_char,
        400 => return b"Bad Request\0" as *const u8 as *const c_char,
        401 => return b"Unauthorized\0" as *const u8 as *const c_char,
        402 => return b"Payment Required\0" as *const u8 as *const c_char,
        403 => return b"Forbidden\0" as *const u8 as *const c_char,
        404 => return b"Not Found\0" as *const u8 as *const c_char,
        405 => return b"Method Not Allowed\0" as *const u8 as *const c_char,
        406 => return b"Not Acceptable\0" as *const u8 as *const c_char,
        407 => {
            return b"Proxy Authentication Required\0" as *const u8 as *const c_char;
        }
        408 => return b"Request Timeout\0" as *const u8 as *const c_char,
        409 => return b"Conflict\0" as *const u8 as *const c_char,
        410 => return b"Gone\0" as *const u8 as *const c_char,
        411 => return b"Length Required\0" as *const u8 as *const c_char,
        412 => return b"Precondition Failed\0" as *const u8 as *const c_char,
        413 => return b"Payload Too Large\0" as *const u8 as *const c_char,
        414 => return b"URI Too Long\0" as *const u8 as *const c_char,
        415 => {
            return b"Unsupported Media Type\0" as *const u8 as *const c_char;
        }
        416 => {
            return b"Range Not Satisfiable\0" as *const u8 as *const c_char;
        }
        417 => return b"Expectation Failed\0" as *const u8 as *const c_char,
        421 => return b"Misdirected Request\0" as *const u8 as *const c_char,
        422 => {
            return b"Unprocessable Entity\0" as *const u8 as *const c_char;
        }
        423 => return b"Locked\0" as *const u8 as *const c_char,
        424 => return b"Failed Dependency\0" as *const u8 as *const c_char,
        426 => return b"Upgrade Required\0" as *const u8 as *const c_char,
        428 => {
            return b"Precondition Required\0" as *const u8 as *const c_char;
        }
        429 => return b"Too Many Requests\0" as *const u8 as *const c_char,
        431 => {
            return b"Request Header Fields Too Large\0" as *const u8 as *const c_char;
        }
        451 => {
            return b"Unavailable For Legal Reasons\0" as *const u8 as *const c_char;
        }
        500 => {
            return b"Internal Server Error\0" as *const u8 as *const c_char;
        }
        501 => return b"Not Implemented\0" as *const u8 as *const c_char,
        502 => return b"Bad Gateway\0" as *const u8 as *const c_char,
        503 => return b"Service Unavailable\0" as *const u8 as *const c_char,
        504 => return b"Gateway Timeout\0" as *const u8 as *const c_char,
        505 => {
            return b"HTTP Version Not Supported\0" as *const u8 as *const c_char;
        }
        506 => {
            return b"Variant Also Negotiates\0" as *const u8 as *const c_char;
        }
        507 => {
            return b"Insufficient Storage\0" as *const u8 as *const c_char;
        }
        508 => return b"Loop Detected\0" as *const u8 as *const c_char,
        510 => return b"Not Extended\0" as *const u8 as *const c_char,
        511 => {
            return b"Network Authentication Required\0" as *const u8 as *const c_char;
        }
        _ => return b"<unknown>\0" as *const u8 as *const c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_parser_init(mut parser: *mut http_parser, mut t: http_parser_type) {
    let mut data: *mut c_void = (*parser).data;
    memset(
        parser as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<http_parser>() as size_t,
    );
    (*parser).data = data;
    (*parser).set_type_0(t as c_uint as c_uint);
    (*parser).set_state(
        (if t as c_uint == HTTP_REQUEST as c_int as c_uint {
            s_start_req as c_int
        } else if t as c_uint
            == HTTP_RESPONSE as c_int as c_uint
        {
            s_start_res as c_int
        } else {
            s_start_req_or_res as c_int
        }) as c_uint as c_uint,
    );
    (*parser)
        .set_http_errno(HPE_OK as c_int as c_uint as c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn http_parser_settings_init(mut settings: *mut http_parser_settings) {
    memset(
        settings as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<http_parser_settings>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn http_errno_name(mut err: http_errno) -> *const c_char {
    '_c2rust_label: {
        if (err as size_t)
            < (::core::mem::size_of::<[C2RustUnnamed_0; 34]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as usize)
        {
        } else {
            __assert_fail(
                b"((size_t) err) < ARRAY_SIZE(http_strerror_tab)\0" as *const u8
                    as *const c_char,
                b"http_parser.c\0" as *const u8 as *const c_char,
                2243 as c_uint,
                b"const char *http_errno_name(enum http_errno)\0" as *const u8
                    as *const c_char,
            );
        }
    };
    return http_strerror_tab[err as usize].name;
}
#[no_mangle]
pub unsafe extern "C" fn http_errno_description(mut err: http_errno) -> *const c_char {
    '_c2rust_label: {
        if (err as size_t)
            < (::core::mem::size_of::<[C2RustUnnamed_0; 34]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as usize)
        {
        } else {
            __assert_fail(
                b"((size_t) err) < ARRAY_SIZE(http_strerror_tab)\0" as *const u8
                    as *const c_char,
                b"http_parser.c\0" as *const u8 as *const c_char,
                2249 as c_uint,
                b"const char *http_errno_description(enum http_errno)\0" as *const u8
                    as *const c_char,
            );
        }
    };
    return http_strerror_tab[err as usize].description;
}
unsafe extern "C" fn http_parse_host_char(
    mut s: http_host_state,
    ch: c_char,
) -> http_host_state {
    let mut current_block_35: u64;
    match s as c_uint {
        3 | 2 => {
            if ch as c_int == '@' as i32 {
                return s_http_host_start;
            }
            if (ch as c_int | 0x20 as c_int) as c_uchar
                as c_int
                >= 'a' as i32
                && (ch as c_int | 0x20 as c_int) as c_uchar
                    as c_int
                    <= 'z' as i32
                || ch as c_int >= '0' as i32 && ch as c_int <= '9' as i32
                || (ch as c_int == '-' as i32
                    || ch as c_int == '_' as i32
                    || ch as c_int == '.' as i32
                    || ch as c_int == '!' as i32
                    || ch as c_int == '~' as i32
                    || ch as c_int == '*' as i32
                    || ch as c_int == '\'' as i32
                    || ch as c_int == '(' as i32
                    || ch as c_int == ')' as i32)
                || ch as c_int == '%' as i32
                || ch as c_int == ';' as i32
                || ch as c_int == ':' as i32
                || ch as c_int == '&' as i32
                || ch as c_int == '=' as i32
                || ch as c_int == '+' as i32
                || ch as c_int == '$' as i32
                || ch as c_int == ',' as i32
            {
                return s_http_userinfo;
            }
            current_block_35 = 2891135413264362348;
        }
        4 => {
            if ch as c_int == '[' as i32 {
                return s_http_host_v6_start;
            }
            if (ch as c_int | 0x20 as c_int) as c_uchar
                as c_int
                >= 'a' as i32
                && (ch as c_int | 0x20 as c_int) as c_uchar
                    as c_int
                    <= 'z' as i32
                || ch as c_int >= '0' as i32 && ch as c_int <= '9' as i32
                || ch as c_int == '.' as i32
                || ch as c_int == '-' as i32
            {
                return s_http_host;
            }
            current_block_35 = 2891135413264362348;
        }
        6 => {
            if (ch as c_int | 0x20 as c_int) as c_uchar
                as c_int
                >= 'a' as i32
                && (ch as c_int | 0x20 as c_int) as c_uchar
                    as c_int
                    <= 'z' as i32
                || ch as c_int >= '0' as i32 && ch as c_int <= '9' as i32
                || ch as c_int == '.' as i32
                || ch as c_int == '-' as i32
            {
                return s_http_host;
            }
            current_block_35 = 10547029659307801991;
        }
        8 => {
            current_block_35 = 10547029659307801991;
        }
        7 => {
            if ch as c_int == ']' as i32 {
                return s_http_host_v6_end;
            }
            current_block_35 = 10758446703012050218;
        }
        5 => {
            current_block_35 = 10758446703012050218;
        }
        10 => {
            if ch as c_int == ']' as i32 {
                return s_http_host_v6_end;
            }
            current_block_35 = 2016610040826759079;
        }
        9 => {
            current_block_35 = 2016610040826759079;
        }
        12 | 11 => {
            if ch as c_int >= '0' as i32 && ch as c_int <= '9' as i32 {
                return s_http_host_port;
            }
            current_block_35 = 2891135413264362348;
        }
        _ => {
            current_block_35 = 2891135413264362348;
        }
    }
    match current_block_35 {
        10758446703012050218 => {
            if ch as c_int >= '0' as i32 && ch as c_int <= '9' as i32
                || (ch as c_int | 0x20 as c_int) as c_uchar
                    as c_int
                    >= 'a' as i32
                    && (ch as c_int | 0x20 as c_int)
                        as c_uchar as c_int
                        <= 'f' as i32
                || ch as c_int == ':' as i32
                || ch as c_int == '.' as i32
            {
                return s_http_host_v6;
            }
            if s as c_uint
                == s_http_host_v6 as c_int as c_uint
                && ch as c_int == '%' as i32
            {
                return s_http_host_v6_zone_start;
            }
        }
        10547029659307801991 => {
            if ch as c_int == ':' as i32 {
                return s_http_host_port_start;
            }
        }
        2016610040826759079 => {
            if (ch as c_int | 0x20 as c_int) as c_uchar
                as c_int
                >= 'a' as i32
                && (ch as c_int | 0x20 as c_int) as c_uchar
                    as c_int
                    <= 'z' as i32
                || ch as c_int >= '0' as i32 && ch as c_int <= '9' as i32
                || ch as c_int == '%' as i32
                || ch as c_int == '.' as i32
                || ch as c_int == '-' as i32
                || ch as c_int == '_' as i32
                || ch as c_int == '~' as i32
            {
                return s_http_host_v6_zone;
            }
        }
        _ => {}
    }
    return s_http_host_dead;
}
unsafe extern "C" fn http_parse_host(
    mut buf: *const c_char,
    mut u: *mut http_parser_url,
    mut found_at: c_int,
) -> c_int {
    let mut s: http_host_state = 0 as http_host_state;
    let mut p: *const c_char = ::core::ptr::null::<c_char>();
    let mut buflen: size_t = ((*u).field_data[UF_HOST as c_int as usize].off
        as c_int
        + (*u).field_data[UF_HOST as c_int as usize].len as c_int)
        as size_t;
    '_c2rust_label: {
        if (*u).field_set as c_int
            & (1 as c_int) << UF_HOST as c_int
            != 0
        {
        } else {
            __assert_fail(
                b"u->field_set & (1 << UF_HOST)\0" as *const u8 as *const c_char,
                b"http_parser.c\0" as *const u8 as *const c_char,
                2342 as c_uint,
                b"int http_parse_host(const char *, struct http_parser_url *, int)\0" as *const u8
                    as *const c_char,
            );
        }
    };
    (*u).field_data[UF_HOST as c_int as usize].len = 0 as uint16_t;
    s = (if found_at != 0 {
        s_http_userinfo_start as c_int
    } else {
        s_http_host_start as c_int
    }) as http_host_state;
    p = buf.offset(
        (*u).field_data[UF_HOST as c_int as usize].off as c_int as isize,
    );
    while p < buf.offset(buflen as isize) {
        let mut new_s: http_host_state = http_parse_host_char(s, *p);
        if new_s as c_uint
            == s_http_host_dead as c_int as c_uint
        {
            return 1 as c_int;
        }
        match new_s as c_uint {
            6 => {
                if s as c_uint
                    != s_http_host as c_int as c_uint
                {
                    (*u).field_data[UF_HOST as c_int as usize].off =
                        p.offset_from(buf) as c_long as uint16_t;
                }
                (*u).field_data[UF_HOST as c_int as usize].len = (*u).field_data
                    [UF_HOST as c_int as usize]
                    .len
                    .wrapping_add(1);
            }
            7 => {
                if s as c_uint
                    != s_http_host_v6 as c_int as c_uint
                {
                    (*u).field_data[UF_HOST as c_int as usize].off =
                        p.offset_from(buf) as c_long as uint16_t;
                }
                (*u).field_data[UF_HOST as c_int as usize].len = (*u).field_data
                    [UF_HOST as c_int as usize]
                    .len
                    .wrapping_add(1);
            }
            9 | 10 => {
                (*u).field_data[UF_HOST as c_int as usize].len = (*u).field_data
                    [UF_HOST as c_int as usize]
                    .len
                    .wrapping_add(1);
            }
            12 => {
                if s as c_uint
                    != s_http_host_port as c_int as c_uint
                {
                    (*u).field_data[UF_PORT as c_int as usize].off =
                        p.offset_from(buf) as c_long as uint16_t;
                    (*u).field_data[UF_PORT as c_int as usize].len = 0 as uint16_t;
                    (*u).field_set = ((*u).field_set as c_int
                        | (1 as c_int) << UF_PORT as c_int)
                        as uint16_t;
                }
                (*u).field_data[UF_PORT as c_int as usize].len = (*u).field_data
                    [UF_PORT as c_int as usize]
                    .len
                    .wrapping_add(1);
            }
            3 => {
                if s as c_uint
                    != s_http_userinfo as c_int as c_uint
                {
                    (*u).field_data[UF_USERINFO as c_int as usize].off =
                        p.offset_from(buf) as c_long as uint16_t;
                    (*u).field_data[UF_USERINFO as c_int as usize].len = 0 as uint16_t;
                    (*u).field_set = ((*u).field_set as c_int
                        | (1 as c_int) << UF_USERINFO as c_int)
                        as uint16_t;
                }
                (*u).field_data[UF_USERINFO as c_int as usize].len = (*u).field_data
                    [UF_USERINFO as c_int as usize]
                    .len
                    .wrapping_add(1);
            }
            _ => {}
        }
        s = new_s;
        p = p.offset(1);
    }
    match s as c_uint {
        4 | 5 | 7 | 9 | 10 | 11 | 3 | 2 => return 1 as c_int,
        _ => {}
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_parser_url_init(mut u: *mut http_parser_url) {
    memset(
        u as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<http_parser_url>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn http_parser_parse_url(
    mut buf: *const c_char,
    mut buflen: size_t,
    mut is_connect: c_int,
    mut u: *mut http_parser_url,
) -> c_int {
    let mut s: state = 0 as state;
    let mut p: *const c_char = ::core::ptr::null::<c_char>();
    let mut uf: http_parser_url_fields = UF_SCHEMA;
    let mut old_uf: http_parser_url_fields = UF_SCHEMA;
    let mut found_at: c_int = 0 as c_int;
    if buflen == 0 as size_t {
        return 1 as c_int;
    }
    (*u).field_set = 0 as uint16_t;
    (*u).port = (*u).field_set;
    s = (if is_connect != 0 {
        s_req_server_start as c_int
    } else {
        s_req_spaces_before_url as c_int
    }) as state;
    old_uf = UF_MAX;
    let mut current_block_21: u64;
    p = buf;
    while p < buf.offset(buflen as isize) {
        s = parse_url_char(s, *p);
        match s as c_uint {
            1 => return 1 as c_int,
            22 | 23 | 24 | 28 | 30 => {
                current_block_21 = 11875828834189669668;
            }
            21 => {
                uf = UF_SCHEMA;
                current_block_21 = 26972500619410423;
            }
            26 => {
                found_at = 1 as c_int;
                current_block_21 = 731546425226429869;
            }
            25 => {
                current_block_21 = 731546425226429869;
            }
            27 => {
                uf = UF_PATH;
                current_block_21 = 26972500619410423;
            }
            29 => {
                uf = UF_QUERY;
                current_block_21 = 26972500619410423;
            }
            31 => {
                uf = UF_FRAGMENT;
                current_block_21 = 26972500619410423;
            }
            _ => {
                '_c2rust_label: {
                    if (b"Unexpected state\0" as *const u8 as *const c_char).is_null()
                    {
                    } else {
                        __assert_fail(
                            b"!\"Unexpected state\"\0" as *const u8
                                as *const c_char,
                            b"http_parser.c\0" as *const u8
                                as *const c_char,
                            2480 as c_uint,
                            b"int http_parser_parse_url(const char *, size_t, int, struct http_parser_url *)\0"
                                as *const u8 as *const c_char,
                        );
                    }
                };
                return 1 as c_int;
            }
        }
        match current_block_21 {
            731546425226429869 => {
                uf = UF_HOST;
                current_block_21 = 26972500619410423;
            }
            _ => {}
        }
        match current_block_21 {
            26972500619410423 => {
                if uf as c_uint == old_uf as c_uint {
                    (*u).field_data[uf as usize].len =
                        (*u).field_data[uf as usize].len.wrapping_add(1);
                } else {
                    (*u).field_data[uf as usize].off =
                        p.offset_from(buf) as c_long as uint16_t;
                    (*u).field_data[uf as usize].len = 1 as uint16_t;
                    (*u).field_set = ((*u).field_set as c_int
                        | (1 as c_int) << uf as c_uint)
                        as uint16_t;
                    old_uf = uf;
                }
            }
            _ => {}
        }
        p = p.offset(1);
    }
    if (*u).field_set as c_int
        & (1 as c_int) << UF_SCHEMA as c_int
        != 0
        && (*u).field_set as c_int
            & (1 as c_int) << UF_HOST as c_int
            == 0 as c_int
    {
        return 1 as c_int;
    }
    if (*u).field_set as c_int
        & (1 as c_int) << UF_HOST as c_int
        != 0
    {
        if http_parse_host(buf, u, found_at) != 0 as c_int {
            return 1 as c_int;
        }
    }
    if is_connect != 0
        && (*u).field_set as c_int
            != (1 as c_int) << UF_HOST as c_int
                | (1 as c_int) << UF_PORT as c_int
    {
        return 1 as c_int;
    }
    if (*u).field_set as c_int
        & (1 as c_int) << UF_PORT as c_int
        != 0
    {
        let mut off: uint16_t = 0;
        let mut len: uint16_t = 0;
        let mut p_0: *const c_char = ::core::ptr::null::<c_char>();
        let mut end: *const c_char = ::core::ptr::null::<c_char>();
        let mut v: c_ulong = 0;
        off = (*u).field_data[UF_PORT as c_int as usize].off;
        len = (*u).field_data[UF_PORT as c_int as usize].len;
        end = buf
            .offset(off as c_int as isize)
            .offset(len as c_int as isize);
        '_c2rust_label_0: {
            if (off as c_int + len as c_int) as size_t <= buflen
                && !(b"Port number overflow\0" as *const u8 as *const c_char).is_null()
            {
            } else {
                __assert_fail(
                    b"(size_t) (off + len) <= buflen && \"Port number overflow\"\0"
                        as *const u8 as *const c_char,
                    b"http_parser.c\0" as *const u8 as *const c_char,
                    2527 as c_uint,
                    b"int http_parser_parse_url(const char *, size_t, int, struct http_parser_url *)\0"
                        as *const u8 as *const c_char,
                );
            }
        };
        v = 0 as c_ulong;
        p_0 = buf.offset(off as c_int as isize);
        while p_0 < end {
            v = v.wrapping_mul(10 as c_ulong);
            v = v.wrapping_add((*p_0 as c_int - '0' as i32) as c_ulong);
            if v > 0xffff as c_ulong {
                return 1 as c_int;
            }
            p_0 = p_0.offset(1);
        }
        (*u).port = v as uint16_t;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_parser_pause(
    mut parser: *mut http_parser,
    mut paused: c_int,
) {
    if (*parser).http_errno() as http_errno as c_uint
        == HPE_OK as c_int as c_uint
        || (*parser).http_errno() as http_errno as c_uint
            == HPE_PAUSED as c_int as c_uint
    {
        let mut nread: uint32_t = (*parser).nread;
        (*parser).nread = nread;
        (*parser).set_http_errno(
            (if paused != 0 {
                HPE_PAUSED as c_int
            } else {
                HPE_OK as c_int
            }) as c_uint as c_uint,
        );
    } else {
        '_c2rust_label: {
            if 0 as c_int != 0
                && !(b"Attempting to pause parser in error state\0" as *const u8
                    as *const c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"0 && \"Attempting to pause parser in error state\"\0" as *const u8
                        as *const c_char,
                    b"http_parser.c\0" as *const u8 as *const c_char,
                    2556 as c_uint,
                    b"void http_parser_pause(http_parser *, int)\0" as *const u8
                        as *const c_char,
                );
            }
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_body_is_final(mut parser: *const http_parser) -> c_int {
    return ((*parser).state() as c_int == s_message_done as c_int)
        as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_parser_version() -> c_ulong {
    return (HTTP_PARSER_VERSION_MAJOR * 0x10000 as c_int
        | HTTP_PARSER_VERSION_MINOR * 0x100 as c_int
        | HTTP_PARSER_VERSION_PATCH * 0x1 as c_int)
        as c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn http_parser_set_max_header_size(mut size: uint32_t) {
    max_header_size = size;
}
pub const __LONG_LONG_MAX__: c_longlong =
    9223372036854775807 as c_longlong;
pub const ULLONG_MAX: c_ulonglong = (__LONG_LONG_MAX__ as c_ulonglong)
    .wrapping_mul(2 as c_ulonglong)
    .wrapping_add(1 as c_ulonglong);
