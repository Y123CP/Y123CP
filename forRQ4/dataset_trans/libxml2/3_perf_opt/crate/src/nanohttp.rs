use core::ffi::*;
use crate::src::xmlIO::__xmlIOErr;
use crate::src::xmlstring::xmlCharStrndup;
use crate::src::uri::xmlFreeURI;
use crate::src::uri::xmlParseURIRaw;
use crate::src::xmlstring::xmlStrcat;
use crate::src::xmlstring::xmlStrdup;
use crate::src::xmlstring::xmlStrncasecmp;
use crate::src::xmlstring::xmlStrndup;
use crate::src::xmlstring::xmlStrstr;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::dict::_xmlDict;
extern "C" {
    fn write(__fd: c_int, __buf: *const c_void, __n: size_t) -> ssize_t;
    fn connect(
        __fd: c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> c_int;
    fn send(
        __fd: c_int,
        __buf: *const c_void,
        __n: size_t,
        __flags: c_int,
    ) -> ssize_t;
    fn recv(
        __fd: c_int,
        __buf: *mut c_void,
        __n: size_t,
        __flags: c_int,
    ) -> ssize_t;
    fn getsockopt(
        __fd: c_int,
        __level: c_int,
        __optname: c_int,
        __optval: *mut c_void,
        __optlen: *mut socklen_t,
    ) -> c_int;
    fn __h_errno_location() -> *mut c_int;
    fn gethostbyname(__name: *const c_char) -> *mut hostent;
    fn fcntl(__fd: c_int, __cmd: c_int, ...) -> c_int;
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: c_int,
    ) -> c_int;
    fn __xmlSimpleError(
        domain: c_int,
        code: c_int,
        node: *mut _xmlNode,
        msg: *const c_char,
        extra: *const c_char,
    );
}

pub type __uint16_t = u16;

pub type __uint32_t = u32;
pub type __socklen_t = c_uint;
pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const _ISalnum: C2RustUnnamed_htdd24ee73 = 8;
pub const _ISpunct: C2RustUnnamed_htdd24ee73 = 4;
pub const _IScntrl: C2RustUnnamed_htdd24ee73 = 2;
pub const _ISblank: C2RustUnnamed_htdd24ee73 = 1;
pub const _ISgraph: C2RustUnnamed_htdd24ee73 = 32768;
pub const _ISprint: C2RustUnnamed_htdd24ee73 = 16384;
pub const _ISspace: C2RustUnnamed_htdd24ee73 = 8192;
pub const _ISxdigit: C2RustUnnamed_htdd24ee73 = 4096;
pub const _ISdigit: C2RustUnnamed_htdd24ee73 = 2048;
pub const _ISalpha: C2RustUnnamed_htdd24ee73 = 1024;
pub const _ISlower: C2RustUnnamed_htdd24ee73 = 512;
pub const _ISupper: C2RustUnnamed_htdd24ee73 = 256;
pub type ssize_t = isize;
pub type socklen_t = __socklen_t;
pub type __socket_type = c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub const IPPROTO_MAX: C2RustUnnamed_htdd24ee73 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_htdd24ee73 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_htdd24ee73 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_htdd24ee73 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_htdd24ee73 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_htdd24ee73 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_htdd24ee73 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_htdd24ee73 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_htdd24ee73 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_htdd24ee73 = 92;
pub const IPPROTO_AH: C2RustUnnamed_htdd24ee73 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_htdd24ee73 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_htdd24ee73 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_htdd24ee73 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_htdd24ee73 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_htdd24ee73 = 33;
pub const IPPROTO_TP: C2RustUnnamed_htdd24ee73 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_htdd24ee73 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_htdd24ee73 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_htdd24ee73 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_htdd24ee73 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_htdd24ee73 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_htdd24ee73 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_htdd24ee73 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_htdd24ee73 = 1;
pub const IPPROTO_IP: C2RustUnnamed_htdd24ee73 = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut c_char,
    pub h_aliases: *mut *mut c_char,
    pub h_addrtype: c_int,
    pub h_length: c_int,
    pub h_addr_list: *mut *mut c_char,
}
pub type nfds_t = c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}
pub const XML_FROM_URI: C2RustUnnamed_htdd24ee73 = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed_htdd24ee73 = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed_htdd24ee73 = 28;
pub const XML_FROM_I18N: C2RustUnnamed_htdd24ee73 = 27;
pub const XML_FROM_MODULE: C2RustUnnamed_htdd24ee73 = 26;
pub const XML_FROM_WRITER: C2RustUnnamed_htdd24ee73 = 25;
pub const XML_FROM_CHECK: C2RustUnnamed_htdd24ee73 = 24;
pub const XML_FROM_VALID: C2RustUnnamed_htdd24ee73 = 23;
pub const XML_FROM_XSLT: C2RustUnnamed_htdd24ee73 = 22;
pub const XML_FROM_C14N: C2RustUnnamed_htdd24ee73 = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed_htdd24ee73 = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed_htdd24ee73 = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed_htdd24ee73 = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed_htdd24ee73 = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed_htdd24ee73 = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed_htdd24ee73 = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed_htdd24ee73 = 13;
pub const XML_FROM_XPATH: C2RustUnnamed_htdd24ee73 = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed_htdd24ee73 = 11;
pub const XML_FROM_HTTP: C2RustUnnamed_htdd24ee73 = 10;
pub const XML_FROM_FTP: C2RustUnnamed_htdd24ee73 = 9;
pub const XML_FROM_IO: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed_htdd24ee73 = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed_htdd24ee73 = 6;
pub const XML_FROM_HTML: C2RustUnnamed_htdd24ee73 = 5;
pub const XML_FROM_DTD: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed_htdd24ee73 = 3;
pub const XML_FROM_TREE: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_FROM_PARSER: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_FROM_NONE: C2RustUnnamed_htdd24ee73 = 0;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_htdd24ee73 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_htdd24ee73 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_htdd24ee73 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_htdd24ee73 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_htdd24ee73 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_htdd24ee73 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_htdd24ee73 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_htdd24ee73 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_htdd24ee73 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_htdd24ee73 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_htdd24ee73 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_htdd24ee73 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_htdd24ee73 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_htdd24ee73 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_htdd24ee73 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_htdd24ee73 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_htdd24ee73 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_htdd24ee73 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_htdd24ee73 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_htdd24ee73 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_htdd24ee73 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_htdd24ee73 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_htdd24ee73 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_htdd24ee73 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_htdd24ee73 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_htdd24ee73 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_htdd24ee73 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_htdd24ee73 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_htdd24ee73 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_htdd24ee73 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_htdd24ee73 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_htdd24ee73 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_htdd24ee73 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_htdd24ee73 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_htdd24ee73 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_htdd24ee73 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_htdd24ee73 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_htdd24ee73 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_htdd24ee73 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_htdd24ee73 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_htdd24ee73 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_htdd24ee73 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_htdd24ee73 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_htdd24ee73 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_htdd24ee73 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_htdd24ee73 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_htdd24ee73 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_htdd24ee73 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_htdd24ee73 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_htdd24ee73 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_htdd24ee73 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_htdd24ee73 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_htdd24ee73 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_htdd24ee73 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_htdd24ee73 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_htdd24ee73 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_htdd24ee73 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_htdd24ee73 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_htdd24ee73 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_htdd24ee73 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_htdd24ee73 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_htdd24ee73 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_htdd24ee73 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_htdd24ee73 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_htdd24ee73 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_htdd24ee73 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_htdd24ee73 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_htdd24ee73 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_htdd24ee73 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_htdd24ee73 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_htdd24ee73 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_htdd24ee73 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_htdd24ee73 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_htdd24ee73 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_htdd24ee73 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_htdd24ee73 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_htdd24ee73 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_htdd24ee73 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_htdd24ee73 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_htdd24ee73 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_htdd24ee73 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_htdd24ee73 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_htdd24ee73 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_htdd24ee73 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_htdd24ee73 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_htdd24ee73 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_htdd24ee73 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_htdd24ee73 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_htdd24ee73 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_htdd24ee73 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_htdd24ee73 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_htdd24ee73 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_htdd24ee73 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_htdd24ee73 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_htdd24ee73 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_htdd24ee73 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_htdd24ee73 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_htdd24ee73 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_htdd24ee73 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_htdd24ee73 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_htdd24ee73 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_htdd24ee73 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_htdd24ee73 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_htdd24ee73 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_htdd24ee73 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_htdd24ee73 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_htdd24ee73 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_htdd24ee73 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_htdd24ee73 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_htdd24ee73 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_htdd24ee73 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_htdd24ee73 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_htdd24ee73 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_htdd24ee73 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_htdd24ee73 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_htdd24ee73 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_htdd24ee73 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_htdd24ee73 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_htdd24ee73 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_htdd24ee73 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_htdd24ee73 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_htdd24ee73 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_htdd24ee73 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_htdd24ee73 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_htdd24ee73 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_htdd24ee73 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_htdd24ee73 = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_htdd24ee73 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_htdd24ee73 = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_htdd24ee73 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_htdd24ee73 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_htdd24ee73 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_htdd24ee73 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_htdd24ee73 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_htdd24ee73 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_htdd24ee73 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_htdd24ee73 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_htdd24ee73 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_htdd24ee73 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_htdd24ee73 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_htdd24ee73 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_htdd24ee73 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_htdd24ee73 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_htdd24ee73 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_htdd24ee73 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_htdd24ee73 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_htdd24ee73 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_htdd24ee73 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_htdd24ee73 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_htdd24ee73 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_htdd24ee73 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_htdd24ee73 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_htdd24ee73 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_htdd24ee73 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_htdd24ee73 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_htdd24ee73 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_htdd24ee73 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_htdd24ee73 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_htdd24ee73 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_htdd24ee73 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_htdd24ee73 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_htdd24ee73 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_htdd24ee73 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_htdd24ee73 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_htdd24ee73 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_htdd24ee73 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_htdd24ee73 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_htdd24ee73 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_htdd24ee73 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_htdd24ee73 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_htdd24ee73 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_htdd24ee73 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_htdd24ee73 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_htdd24ee73 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_htdd24ee73 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_htdd24ee73 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_htdd24ee73 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_htdd24ee73 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_htdd24ee73 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_htdd24ee73 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_htdd24ee73 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_htdd24ee73 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_htdd24ee73 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_htdd24ee73 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_htdd24ee73 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_htdd24ee73 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_htdd24ee73 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_htdd24ee73 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_htdd24ee73 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_htdd24ee73 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_htdd24ee73 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_htdd24ee73 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_htdd24ee73 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_htdd24ee73 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_htdd24ee73 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_htdd24ee73 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_htdd24ee73 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_htdd24ee73 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_htdd24ee73 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_htdd24ee73 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_htdd24ee73 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_htdd24ee73 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_htdd24ee73 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_htdd24ee73 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_htdd24ee73 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_htdd24ee73 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_htdd24ee73 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_htdd24ee73 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_htdd24ee73 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_htdd24ee73 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_htdd24ee73 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_htdd24ee73 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_htdd24ee73 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_htdd24ee73 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_htdd24ee73 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_htdd24ee73 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_htdd24ee73 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_htdd24ee73 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_htdd24ee73 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_htdd24ee73 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_htdd24ee73 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_htdd24ee73 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_htdd24ee73 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_htdd24ee73 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_htdd24ee73 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_htdd24ee73 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_htdd24ee73 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_htdd24ee73 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_htdd24ee73 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_htdd24ee73 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_htdd24ee73 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_htdd24ee73 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_htdd24ee73 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_htdd24ee73 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_htdd24ee73 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_htdd24ee73 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_htdd24ee73 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_htdd24ee73 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_htdd24ee73 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_htdd24ee73 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_htdd24ee73 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_htdd24ee73 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_htdd24ee73 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_htdd24ee73 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_htdd24ee73 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_htdd24ee73 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_htdd24ee73 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_htdd24ee73 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_htdd24ee73 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_htdd24ee73 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_htdd24ee73 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_htdd24ee73 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_htdd24ee73 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_htdd24ee73 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_htdd24ee73 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_htdd24ee73 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_htdd24ee73 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_htdd24ee73 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_htdd24ee73 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_htdd24ee73 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_htdd24ee73 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_htdd24ee73 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_htdd24ee73 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_htdd24ee73 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_htdd24ee73 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_htdd24ee73 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_htdd24ee73 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_htdd24ee73 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_htdd24ee73 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_htdd24ee73 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_htdd24ee73 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_htdd24ee73 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_htdd24ee73 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_htdd24ee73 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_htdd24ee73 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_htdd24ee73 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_htdd24ee73 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_htdd24ee73 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_htdd24ee73 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_htdd24ee73 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_htdd24ee73 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_htdd24ee73 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_htdd24ee73 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_htdd24ee73 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_htdd24ee73 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_htdd24ee73 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_htdd24ee73 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_htdd24ee73 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_htdd24ee73 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_htdd24ee73 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_htdd24ee73 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_htdd24ee73 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_htdd24ee73 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_htdd24ee73 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_htdd24ee73 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_htdd24ee73 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_htdd24ee73 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_htdd24ee73 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_htdd24ee73 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_htdd24ee73 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_htdd24ee73 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_htdd24ee73 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_htdd24ee73 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_htdd24ee73 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_htdd24ee73 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_htdd24ee73 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_htdd24ee73 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_htdd24ee73 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_htdd24ee73 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_htdd24ee73 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_htdd24ee73 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_htdd24ee73 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_htdd24ee73 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_htdd24ee73 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_htdd24ee73 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_htdd24ee73 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_htdd24ee73 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_htdd24ee73 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_htdd24ee73 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_htdd24ee73 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_htdd24ee73 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_htdd24ee73 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_htdd24ee73 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_htdd24ee73 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_htdd24ee73 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_htdd24ee73 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_htdd24ee73 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_htdd24ee73 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_htdd24ee73 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_htdd24ee73 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_htdd24ee73 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_htdd24ee73 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_htdd24ee73 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_htdd24ee73 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_htdd24ee73 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_htdd24ee73 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_htdd24ee73 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_htdd24ee73 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_htdd24ee73 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_htdd24ee73 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_htdd24ee73 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_htdd24ee73 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_htdd24ee73 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_htdd24ee73 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_htdd24ee73 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_htdd24ee73 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_htdd24ee73 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_htdd24ee73 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_htdd24ee73 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_htdd24ee73 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_htdd24ee73 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_htdd24ee73 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_htdd24ee73 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_htdd24ee73 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_htdd24ee73 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_htdd24ee73 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_htdd24ee73 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_htdd24ee73 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_htdd24ee73 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_htdd24ee73 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_htdd24ee73 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_htdd24ee73 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_htdd24ee73 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_htdd24ee73 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_htdd24ee73 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_htdd24ee73 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_htdd24ee73 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_htdd24ee73 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_htdd24ee73 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_htdd24ee73 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_htdd24ee73 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_htdd24ee73 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_htdd24ee73 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_htdd24ee73 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_htdd24ee73 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_htdd24ee73 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_htdd24ee73 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_htdd24ee73 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_htdd24ee73 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_htdd24ee73 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_htdd24ee73 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_htdd24ee73 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_htdd24ee73 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_htdd24ee73 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_htdd24ee73 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_htdd24ee73 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_htdd24ee73 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_htdd24ee73 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_htdd24ee73 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_htdd24ee73 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_htdd24ee73 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_htdd24ee73 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_htdd24ee73 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_htdd24ee73 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_htdd24ee73 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_htdd24ee73 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_htdd24ee73 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_htdd24ee73 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_htdd24ee73 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_htdd24ee73 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_htdd24ee73 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_htdd24ee73 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_htdd24ee73 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_htdd24ee73 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_htdd24ee73 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_htdd24ee73 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_htdd24ee73 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_htdd24ee73 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_htdd24ee73 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_htdd24ee73 = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_htdd24ee73 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_htdd24ee73 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_htdd24ee73 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_htdd24ee73 = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_htdd24ee73 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_htdd24ee73 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_htdd24ee73 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_htdd24ee73 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_htdd24ee73 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_htdd24ee73 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_htdd24ee73 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_htdd24ee73 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_htdd24ee73 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_htdd24ee73 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_htdd24ee73 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_htdd24ee73 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_htdd24ee73 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_htdd24ee73 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_htdd24ee73 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_htdd24ee73 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_htdd24ee73 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_htdd24ee73 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_htdd24ee73 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_htdd24ee73 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_htdd24ee73 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_htdd24ee73 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_htdd24ee73 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_htdd24ee73 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_htdd24ee73 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_htdd24ee73 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_htdd24ee73 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_htdd24ee73 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_htdd24ee73 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_htdd24ee73 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_htdd24ee73 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_htdd24ee73 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_htdd24ee73 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_htdd24ee73 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_htdd24ee73 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_htdd24ee73 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_htdd24ee73 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_htdd24ee73 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_htdd24ee73 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_htdd24ee73 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_htdd24ee73 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_htdd24ee73 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_htdd24ee73 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_htdd24ee73 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_htdd24ee73 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_htdd24ee73 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_htdd24ee73 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_htdd24ee73 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_htdd24ee73 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_htdd24ee73 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_htdd24ee73 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_htdd24ee73 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_htdd24ee73 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_htdd24ee73 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_htdd24ee73 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_htdd24ee73 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_htdd24ee73 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_htdd24ee73 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_htdd24ee73 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_htdd24ee73 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_htdd24ee73 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_htdd24ee73 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_htdd24ee73 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_htdd24ee73 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_htdd24ee73 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_htdd24ee73 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_htdd24ee73 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_htdd24ee73 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_htdd24ee73 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_htdd24ee73 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_htdd24ee73 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_htdd24ee73 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_htdd24ee73 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_htdd24ee73 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_htdd24ee73 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_htdd24ee73 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_htdd24ee73 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_htdd24ee73 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_htdd24ee73 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_htdd24ee73 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_htdd24ee73 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_htdd24ee73 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_htdd24ee73 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_htdd24ee73 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_htdd24ee73 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_htdd24ee73 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_htdd24ee73 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_htdd24ee73 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_htdd24ee73 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_htdd24ee73 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_htdd24ee73 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_htdd24ee73 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_htdd24ee73 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_htdd24ee73 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_htdd24ee73 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_htdd24ee73 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_htdd24ee73 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_htdd24ee73 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_htdd24ee73 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_htdd24ee73 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_htdd24ee73 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_htdd24ee73 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_htdd24ee73 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_htdd24ee73 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_htdd24ee73 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_htdd24ee73 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_htdd24ee73 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_htdd24ee73 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_htdd24ee73 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_htdd24ee73 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_htdd24ee73 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_htdd24ee73 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_htdd24ee73 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_htdd24ee73 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_htdd24ee73 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_htdd24ee73 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_htdd24ee73 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_htdd24ee73 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_htdd24ee73 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_htdd24ee73 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_htdd24ee73 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_htdd24ee73 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_htdd24ee73 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_htdd24ee73 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_htdd24ee73 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_htdd24ee73 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_htdd24ee73 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_htdd24ee73 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_htdd24ee73 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_htdd24ee73 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_htdd24ee73 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_htdd24ee73 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_htdd24ee73 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_htdd24ee73 = 1000;
pub const XML_HTML_INCORRECTLY_OPENED_COMMENT: C2RustUnnamed_htdd24ee73 = 802;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_htdd24ee73 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_htdd24ee73 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_htdd24ee73 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_htdd24ee73 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_htdd24ee73 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_htdd24ee73 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_htdd24ee73 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_htdd24ee73 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_htdd24ee73 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_htdd24ee73 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_htdd24ee73 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_htdd24ee73 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_htdd24ee73 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_htdd24ee73 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_htdd24ee73 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_htdd24ee73 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_htdd24ee73 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_htdd24ee73 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_htdd24ee73 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_htdd24ee73 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_htdd24ee73 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_htdd24ee73 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_htdd24ee73 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_htdd24ee73 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_htdd24ee73 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_htdd24ee73 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_htdd24ee73 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_htdd24ee73 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_htdd24ee73 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_htdd24ee73 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_htdd24ee73 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_htdd24ee73 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_htdd24ee73 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_htdd24ee73 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_htdd24ee73 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_htdd24ee73 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_htdd24ee73 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_htdd24ee73 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_htdd24ee73 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_htdd24ee73 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_htdd24ee73 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_htdd24ee73 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_htdd24ee73 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_htdd24ee73 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_htdd24ee73 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_htdd24ee73 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_htdd24ee73 = 200;
pub const XML_WAR_ENCODING_MISMATCH: C2RustUnnamed_htdd24ee73 = 113;
pub const XML_ERR_COMMENT_ABRUPTLY_ENDED: C2RustUnnamed_htdd24ee73 = 112;
pub const XML_ERR_USER_STOP: C2RustUnnamed_htdd24ee73 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_htdd24ee73 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_htdd24ee73 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_htdd24ee73 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_htdd24ee73 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_htdd24ee73 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_htdd24ee73 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_htdd24ee73 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_htdd24ee73 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_htdd24ee73 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_htdd24ee73 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_htdd24ee73 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_htdd24ee73 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_htdd24ee73 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_htdd24ee73 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_htdd24ee73 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_htdd24ee73 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_htdd24ee73 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_htdd24ee73 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_htdd24ee73 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_htdd24ee73 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_htdd24ee73 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_htdd24ee73 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_htdd24ee73 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_htdd24ee73 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_htdd24ee73 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_htdd24ee73 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_htdd24ee73 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_htdd24ee73 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_htdd24ee73 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_htdd24ee73 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_htdd24ee73 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_htdd24ee73 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_htdd24ee73 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_htdd24ee73 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_htdd24ee73 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_htdd24ee73 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_htdd24ee73 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_htdd24ee73 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_htdd24ee73 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_htdd24ee73 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_htdd24ee73 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_htdd24ee73 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_htdd24ee73 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_htdd24ee73 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_htdd24ee73 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_htdd24ee73 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_htdd24ee73 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_htdd24ee73 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_htdd24ee73 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_htdd24ee73 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_htdd24ee73 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_htdd24ee73 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_htdd24ee73 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_htdd24ee73 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_htdd24ee73 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_htdd24ee73 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_htdd24ee73 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_htdd24ee73 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_htdd24ee73 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_htdd24ee73 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_htdd24ee73 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_htdd24ee73 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_htdd24ee73 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_htdd24ee73 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_htdd24ee73 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_htdd24ee73 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_htdd24ee73 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_htdd24ee73 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_htdd24ee73 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_htdd24ee73 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_htdd24ee73 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_htdd24ee73 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_htdd24ee73 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_htdd24ee73 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_htdd24ee73 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_htdd24ee73 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_htdd24ee73 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_htdd24ee73 = 3;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_ERR_OK: C2RustUnnamed_htdd24ee73 = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut c_void,
    pub type_0: xmlElementType,
    pub name: *mut c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: c_int,
    pub standalone: c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut c_void,
    pub refs: *mut c_void,
    pub URL: *const xmlChar,
    pub charset: c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut c_void,
    pub parseFlags: c_int,
    pub properties: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut c_void,
    pub context: *mut _xmlDoc,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut c_void,
    pub elements: *mut c_void,
    pub attributes: *mut c_void,
    pub entities: *mut c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut c_void,
    pub line: c_ushort,
    pub extra: c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut c_void,
}

pub type xmlNanoHTTPCtxtPtr = *mut xmlNanoHTTPCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlNanoHTTPCtxt {
    pub protocol: *mut c_char,
    pub hostname: *mut c_char,
    pub port: c_int,
    pub path: *mut c_char,
    pub query: *mut c_char,
    pub fd: c_int,
    pub state: c_int,
    pub out: *mut c_char,
    pub outptr: *mut c_char,
    pub in_0: *mut c_char,
    pub content: *mut c_char,
    pub inptr: *mut c_char,
    pub inrptr: *mut c_char,
    pub inlen: c_int,
    pub last: c_int,
    pub returnValue: c_int,
    pub version: c_int,
    pub ContentLength: c_int,
    pub contentType: *mut c_char,
    pub location: *mut c_char,
    pub authHeader: *mut c_char,
    pub encoding: *mut c_char,
    pub mimeType: *mut c_char,
}

pub const EWOULDBLOCK: c_int = EAGAIN;
pub const ECONNRESET: c_int = 104;
pub const ESHUTDOWN: c_int = 108;
pub const EINPROGRESS: c_int = 115;
#[inline]
fn tolower(mut __c: c_int) -> c_int { unsafe {
    return if __c >= -(128 as c_int) && __c < 256 as c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as c_int
    } else {
        __c
    };
} }
#[inline]
fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t { {
    return (__bsx as c_int >> 8 as c_int & 0xff as c_int
        | (__bsx as c_int & 0xff as c_int) << 8 as c_int)
        as __uint16_t;
} }

pub const EAGAIN: c_int = 11;
pub const O_WRONLY: c_int = 0o1 as c_int;
pub const O_CREAT: c_int = 0o100 as c_int;
pub const O_NONBLOCK: c_int = 0o4000 as c_int;
pub const F_GETFL: c_int = 3 as c_int;
pub const F_SETFL: c_int = 4 as c_int;
pub const SOL_SOCKET: c_int = 1 as c_int;
pub const SO_ERROR: c_int = 4 as c_int;
pub const PF_INET: c_int = 2 as c_int;
pub const AF_INET: c_int = PF_INET;
pub const POLLIN: c_int = 0x1 as c_int;
pub const POLLOUT: c_int = 0x4 as c_int;
pub const HOST_NOT_FOUND: c_int = 1;
pub const TRY_AGAIN: c_int = 2;
pub const NO_RECOVERY: c_int = 3;
pub const NO_DATA: c_int = 4;
pub const NO_ADDRESS: c_int = NO_DATA;
pub const INVALID_SOCKET: c_int = -(1 as c_int);
pub const XML_NANO_HTTP_MAX_REDIR: c_int = 10 as c_int;
pub const XML_NANO_HTTP_CHUNK: c_int = 4096 as c_int;
pub const XML_NANO_HTTP_WRITE: c_int = 1 as c_int;
pub const XML_NANO_HTTP_READ: c_int = 2 as c_int;
pub const XML_NANO_HTTP_NONE: c_int = 4 as c_int;
static mut initialized: c_int = 0 as c_int;
static mut proxy: *mut c_char =
    ::core::ptr::null::<c_char>() as *mut c_char;
static mut proxyPort: c_int = 0;
static mut timeout: c_uint = 60 as c_uint;
unsafe fn xmlHTTPErrMemory(mut extra: *const c_char) {
    __xmlSimpleError(
        XML_FROM_HTTP as c_int,
        XML_ERR_NO_MEMORY as c_int,
        ::core::ptr::null_mut::<_xmlNode>(),
        ::core::ptr::null::<c_char>(),
        extra,
    );
}
fn socket_errno() -> c_int { unsafe {
    return *__errno_location();
} }
#[inline]
pub fn xmlNanoHTTPInit() { unsafe {
    let mut env: *const c_char = ::core::ptr::null::<c_char>();
    if initialized != 0 {
        return;
    }
    if proxy.is_null() {
        proxyPort = 80 as c_int;
        env = getenv(b"no_proxy\0" as *const u8 as *const c_char);
        if !(!env.is_null()
            && (*env.offset(0 as c_int as isize) as c_int == '*' as i32
                && *env.offset(1 as c_int as isize) as c_int
                    == 0 as c_int))
        {
            env = getenv(b"http_proxy\0" as *const u8 as *const c_char);
            if !env.is_null() {
                xmlNanoHTTPScanProxy(env);
            } else {
                env = getenv(b"HTTP_PROXY\0" as *const u8 as *const c_char);
                if !env.is_null() {
                    xmlNanoHTTPScanProxy(env);
                }
            }
        }
    }
    initialized = 1 as c_int;
} }
#[inline]
pub fn xmlNanoHTTPCleanup() { unsafe {
    if !proxy.is_null() {
        xmlFree.expect("non-null function pointer")(proxy as *mut c_void);
        proxy = ::core::ptr::null_mut::<c_char>();
    }
    initialized = 0 as c_int;
} }
unsafe fn xmlNanoHTTPScanURL(
    mut ctxt: xmlNanoHTTPCtxtPtr,
    mut URL: *const c_char,
) {
    let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
    let mut len: c_int = 0;
    if !(*ctxt).protocol.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).protocol as *mut c_void);
        (*ctxt).protocol = ::core::ptr::null_mut::<c_char>();
    }
    if !(*ctxt).hostname.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).hostname as *mut c_void);
        (*ctxt).hostname = ::core::ptr::null_mut::<c_char>();
    }
    if !(*ctxt).path.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).path as *mut c_void);
        (*ctxt).path = ::core::ptr::null_mut::<c_char>();
    }
    if !(*ctxt).query.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).query as *mut c_void);
        (*ctxt).query = ::core::ptr::null_mut::<c_char>();
    }
    if URL.is_null() {
        return;
    }
    uri = xmlParseURIRaw(URL, 1 as c_int);
    if uri.is_null() {
        return;
    }
    if (*uri).scheme.is_null() || (*uri).server.is_null() {
        xmlFreeURI(uri);
        return;
    }
    (*ctxt).protocol = xmlMemStrdup.expect("non-null function pointer")((*uri).scheme);
    if !(*uri).server.is_null() && *(*uri).server as c_int == '[' as i32 {
        len = strlen((*uri).server) as c_int;
        if len > 2 as c_int
            && *(*uri)
                .server
                .offset((len - 1 as c_int) as isize)
                as c_int
                == ']' as i32
        {
            (*ctxt).hostname = xmlCharStrndup(
                (*uri).server.offset(1 as c_int as isize),
                len - 2 as c_int,
            ) as *mut c_char;
        } else {
            (*ctxt).hostname = xmlMemStrdup.expect("non-null function pointer")((*uri).server);
        }
    } else {
        (*ctxt).hostname = xmlMemStrdup.expect("non-null function pointer")((*uri).server);
    }
    if !(*uri).path.is_null() {
        (*ctxt).path = xmlMemStrdup.expect("non-null function pointer")((*uri).path);
    } else {
        (*ctxt).path = xmlMemStrdup.expect("non-null function pointer")(
            b"/\0" as *const u8 as *const c_char,
        );
    }
    if !(*uri).query.is_null() {
        (*ctxt).query = xmlMemStrdup.expect("non-null function pointer")((*uri).query);
    }
    if (*uri).port != 0 as c_int {
        (*ctxt).port = (*uri).port;
    }
    xmlFreeURI(uri);
}
#[inline]
pub unsafe fn xmlNanoHTTPScanProxy(mut URL: *const c_char) {
    let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
    if !proxy.is_null() {
        xmlFree.expect("non-null function pointer")(proxy as *mut c_void);
        proxy = ::core::ptr::null_mut::<c_char>();
    }
    proxyPort = 0 as c_int;
    if URL.is_null() {
        return;
    }
    uri = xmlParseURIRaw(URL, 1 as c_int);
    if uri.is_null()
        || (*uri).scheme.is_null()
        || strcmp(
            (*uri).scheme,
            b"http\0" as *const u8 as *const c_char,
        ) != 0
        || (*uri).server.is_null()
    {
        __xmlIOErr(
            XML_FROM_HTTP as c_int,
            XML_HTTP_URL_SYNTAX as c_int,
            b"Syntax Error\n\0" as *const u8 as *const c_char,
        );
        if !uri.is_null() {
            xmlFreeURI(uri);
        }
        return;
    }
    proxy = xmlMemStrdup.expect("non-null function pointer")((*uri).server);
    if (*uri).port != 0 as c_int {
        proxyPort = (*uri).port;
    }
    xmlFreeURI(uri);
}
unsafe fn xmlNanoHTTPNewCtxt(mut URL: *const c_char) -> xmlNanoHTTPCtxtPtr {
    let mut ret: xmlNanoHTTPCtxtPtr = ::core::ptr::null_mut::<xmlNanoHTTPCtxt>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlNanoHTTPCtxt>() as size_t
    ) as xmlNanoHTTPCtxtPtr;
    if ret.is_null() {
        xmlHTTPErrMemory(b"allocating context\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNanoHTTPCtxt>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNanoHTTPCtxt>() as size_t,
    );
    (*ret).port = 80 as c_int;
    (*ret).returnValue = 0 as c_int;
    (*ret).fd = INVALID_SOCKET;
    (*ret).ContentLength = -(1 as c_int);
    xmlNanoHTTPScanURL(ret, URL);
    return ret;
}
fn xmlNanoHTTPFreeCtxt(mut ctxt: xmlNanoHTTPCtxtPtr) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if !(*ctxt).hostname.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).hostname as *mut c_void);
    }
    if !(*ctxt).protocol.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).protocol as *mut c_void);
    }
    if !(*ctxt).path.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).path as *mut c_void);
    }
    if !(*ctxt).query.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).query as *mut c_void);
    }
    if !(*ctxt).out.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).out as *mut c_void);
    }
    if !(*ctxt).in_0.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).in_0 as *mut c_void);
    }
    if !(*ctxt).contentType.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).contentType as *mut c_void,
        );
    }
    if !(*ctxt).encoding.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).encoding as *mut c_void);
    }
    if !(*ctxt).mimeType.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).mimeType as *mut c_void);
    }
    if !(*ctxt).location.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).location as *mut c_void);
    }
    if !(*ctxt).authHeader.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).authHeader as *mut c_void);
    }
    (*ctxt).state = XML_NANO_HTTP_NONE;
    if (*ctxt).fd != INVALID_SOCKET {
        close((*ctxt).fd);
    }
    (*ctxt).fd = INVALID_SOCKET;
    xmlFree.expect("non-null function pointer")(ctxt as *mut c_void);
} }
unsafe fn xmlNanoHTTPSend(
    mut ctxt: xmlNanoHTTPCtxtPtr,
    mut xmt_ptr: *const c_char,
    mut outlen: c_int,
) -> c_int {
    let mut total_sent: c_int = 0 as c_int;
    let mut p: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    if (*ctxt).state & XML_NANO_HTTP_WRITE != 0 && !xmt_ptr.is_null() {
        while total_sent < outlen {
            let mut nsent: c_int = send(
                (*ctxt).fd,
                xmt_ptr.offset(total_sent as isize) as *mut c_char
                    as *const c_void,
                (outlen - total_sent) as size_t,
                0 as c_int,
            ) as c_int;
            if nsent > 0 as c_int {
                total_sent += nsent;
            } else if nsent == -(1 as c_int) && socket_errno() != EWOULDBLOCK {
                __xmlIOErr(
                    XML_FROM_HTTP as c_int,
                    0 as c_int,
                    b"send failed\n\0" as *const u8 as *const c_char,
                );
                if total_sent == 0 as c_int {
                    total_sent = -(1 as c_int);
                }
                break;
            } else {
                p.fd = (*ctxt).fd;
                p.events = POLLOUT as c_short;
                poll(
                    &raw mut p,
                    1 as nfds_t,
                    timeout.wrapping_mul(1000 as c_uint) as c_int,
                );
            }
        }
    }
    return total_sent;
}
fn xmlNanoHTTPRecv(mut ctxt: xmlNanoHTTPCtxtPtr) -> c_int { unsafe {
    let mut p: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    while (*ctxt).state & XML_NANO_HTTP_READ != 0 {
        if (*ctxt).in_0.is_null() {
            (*ctxt).in_0 = xmlMallocAtomic.expect("non-null function pointer")(65000 as size_t)
                as *mut c_char;
            if (*ctxt).in_0.is_null() {
                xmlHTTPErrMemory(b"allocating input\0" as *const u8 as *const c_char);
                (*ctxt).last = -(1 as c_int);
                return -(1 as c_int);
            }
            (*ctxt).inlen = 65000 as c_int;
            (*ctxt).inrptr = (*ctxt).in_0;
            (*ctxt).content = (*ctxt).inrptr;
            (*ctxt).inptr = (*ctxt).content;
        }
        if (*ctxt).inrptr > (*ctxt).in_0.offset(XML_NANO_HTTP_CHUNK as isize) {
            let mut delta: c_int = (*ctxt).inrptr.offset_from((*ctxt).in_0)
                as c_long
                as c_int;
            let mut len: c_int = (*ctxt).inptr.offset_from((*ctxt).inrptr)
                as c_long
                as c_int;
            memmove(
                (*ctxt).in_0 as *mut c_void,
                (*ctxt).inrptr as *const c_void,
                len as size_t,
            );
            (*ctxt).inrptr = (*ctxt).inrptr.offset(-(delta as isize));
            (*ctxt).content = (*ctxt).content.offset(-(delta as isize));
            (*ctxt).inptr = (*ctxt).inptr.offset(-(delta as isize));
        }
        if (*ctxt).in_0.offset((*ctxt).inlen as isize)
            < (*ctxt).inptr.offset(XML_NANO_HTTP_CHUNK as isize)
        {
            let mut d_inptr: c_int = (*ctxt).inptr.offset_from((*ctxt).in_0)
                as c_long
                as c_int;
            let mut d_content: c_int = (*ctxt).content.offset_from((*ctxt).in_0)
                as c_long
                as c_int;
            let mut d_inrptr: c_int = (*ctxt).inrptr.offset_from((*ctxt).in_0)
                as c_long
                as c_int;
            let mut tmp_ptr: *mut c_char = (*ctxt).in_0;
            (*ctxt).inlen *= 2 as c_int;
            (*ctxt).in_0 = xmlRealloc.expect("non-null function pointer")(
                tmp_ptr as *mut c_void,
                (*ctxt).inlen as size_t,
            ) as *mut c_char;
            if (*ctxt).in_0.is_null() {
                xmlHTTPErrMemory(
                    b"allocating input buffer\0" as *const u8 as *const c_char,
                );
                xmlFree.expect("non-null function pointer")(tmp_ptr as *mut c_void);
                (*ctxt).last = -(1 as c_int);
                return -(1 as c_int);
            }
            (*ctxt).inptr = (*ctxt).in_0.offset(d_inptr as isize);
            (*ctxt).content = (*ctxt).in_0.offset(d_content as isize);
            (*ctxt).inrptr = (*ctxt).in_0.offset(d_inrptr as isize);
        }
        (*ctxt).last = recv(
            (*ctxt).fd,
            (*ctxt).inptr as *mut c_void,
            XML_NANO_HTTP_CHUNK as size_t,
            0 as c_int,
        ) as c_int;
        if (*ctxt).last > 0 as c_int {
            (*ctxt).inptr = (*ctxt).inptr.offset((*ctxt).last as isize);
            return (*ctxt).last;
        }
        if (*ctxt).last == 0 as c_int {
            return 0 as c_int;
        }
        if (*ctxt).last == -(1 as c_int) {
            match socket_errno() {
                EINPROGRESS | EWOULDBLOCK => {}
                ECONNRESET | ESHUTDOWN => return 0 as c_int,
                _ => {
                    __xmlIOErr(
                        XML_FROM_HTTP as c_int,
                        0 as c_int,
                        b"recv failed\n\0" as *const u8 as *const c_char,
                    );
                    return -(1 as c_int);
                }
            }
        }
        p.fd = (*ctxt).fd;
        p.events = POLLIN as c_short;
        if poll(
            &raw mut p,
            1 as nfds_t,
            timeout.wrapping_mul(1000 as c_uint) as c_int,
        ) < 1 as c_int
            && *__errno_location() != EINTR
        {
            return 0 as c_int;
        }
    }
    return 0 as c_int;
} }
fn xmlNanoHTTPReadLine(mut ctxt: xmlNanoHTTPCtxtPtr) -> *mut c_char { unsafe {
    let mut buf: [c_char; 4096] = [0; 4096];
    let mut bp: *mut c_char = &raw mut buf as *mut c_char;
    let mut rc: c_int = 0;
    while (bp.offset_from(&raw mut buf as *mut c_char) as c_long)
        < 4095 as c_long
    {
        if (*ctxt).inrptr == (*ctxt).inptr {
            rc = xmlNanoHTTPRecv(ctxt);
            if rc == 0 as c_int {
                if bp == &raw mut buf as *mut c_char {
                    return ::core::ptr::null_mut::<c_char>();
                } else {
                    *bp = 0 as c_char;
                }
                return xmlMemStrdup.expect("non-null function pointer")(
                    &raw mut buf as *mut c_char,
                );
            } else if rc == -(1 as c_int) {
                return ::core::ptr::null_mut::<c_char>();
            }
        }
        let fresh0 = (*ctxt).inrptr;
        (*ctxt).inrptr = (*ctxt).inrptr.offset(1);
        *bp = *fresh0;
        if *bp as c_int == '\n' as i32 {
            *bp = 0 as c_char;
            return xmlMemStrdup.expect("non-null function pointer")(
                &raw mut buf as *mut c_char,
            );
        }
        if *bp as c_int != '\r' as i32 {
            bp = bp.offset(1);
        }
    }
    buf[4095 as c_int as usize] = 0 as c_char;
    return xmlMemStrdup.expect("non-null function pointer")(
        &raw mut buf as *mut c_char,
    );
} }
unsafe fn xmlNanoHTTPScanAnswer(
    mut ctxt: xmlNanoHTTPCtxtPtr,
    mut line: *const c_char,
) {
    let mut cur: *const c_char = line;
    if line.is_null() {
        return;
    }
    if strncmp(
        line,
        b"HTTP/\0" as *const u8 as *const c_char,
        5 as size_t,
    ) == 0
    {
        let mut version: c_int = 0 as c_int;
        let mut ret: c_int = 0 as c_int;
        cur = cur.offset(5 as c_int as isize);
        while *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32 {
            version *= 10 as c_int;
            version += *cur as c_int - '0' as i32;
            cur = cur.offset(1);
        }
        if *cur as c_int == '.' as i32 {
            cur = cur.offset(1);
            if *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32
            {
                version *= 10 as c_int;
                version += *cur as c_int - '0' as i32;
                cur = cur.offset(1);
            }
            while *cur as c_int >= '0' as i32
                && *cur as c_int <= '9' as i32
            {
                cur = cur.offset(1);
            }
        } else {
            version *= 10 as c_int;
        }
        if *cur as c_int != ' ' as i32 && *cur as c_int != '\t' as i32 {
            return;
        }
        while *cur as c_int == ' ' as i32 || *cur as c_int == '\t' as i32
        {
            cur = cur.offset(1);
        }
        if (*cur as c_int) < '0' as i32 || *cur as c_int > '9' as i32 {
            return;
        }
        while *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32 {
            ret *= 10 as c_int;
            ret += *cur as c_int - '0' as i32;
            cur = cur.offset(1);
        }
        if *cur as c_int != 0 as c_int
            && *cur as c_int != ' ' as i32
            && *cur as c_int != '\t' as i32
        {
            return;
        }
        (*ctxt).returnValue = ret;
        (*ctxt).version = version;
    } else if xmlStrncasecmp(
        line as *mut xmlChar,
        b"Content-Type:\0" as *const u8 as *const c_char as *mut xmlChar,
        13 as c_int,
    ) == 0
    {
        let mut charset: *const xmlChar = ::core::ptr::null::<xmlChar>();
        let mut last: *const xmlChar = ::core::ptr::null::<xmlChar>();
        let mut mime: *const xmlChar = ::core::ptr::null::<xmlChar>();
        cur = cur.offset(13 as c_int as isize);
        while *cur as c_int == ' ' as i32 || *cur as c_int == '\t' as i32
        {
            cur = cur.offset(1);
        }
        if !(*ctxt).contentType.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ctxt).contentType as *mut c_void,
            );
        }
        (*ctxt).contentType = xmlMemStrdup.expect("non-null function pointer")(cur);
        mime = cur as *const xmlChar;
        last = mime;
        while *last as c_int != 0 as c_int
            && *last as c_int != ' ' as i32
            && *last as c_int != '\t' as i32
            && *last as c_int != ';' as i32
            && *last as c_int != ',' as i32
        {
            last = last.offset(1);
        }
        if !(*ctxt).mimeType.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ctxt).mimeType as *mut c_void,
            );
        }
        (*ctxt).mimeType = xmlStrndup(
            mime,
            last.offset_from(mime) as c_long as c_int,
        ) as *mut c_char;
        charset = xmlStrstr(
            (*ctxt).contentType as *mut xmlChar,
            b"charset=\0" as *const u8 as *const c_char as *mut xmlChar,
        );
        if !charset.is_null() {
            charset = charset.offset(8 as c_int as isize);
            last = charset;
            while *last as c_int != 0 as c_int
                && *last as c_int != ' ' as i32
                && *last as c_int != '\t' as i32
                && *last as c_int != ';' as i32
                && *last as c_int != ',' as i32
            {
                last = last.offset(1);
            }
            if !(*ctxt).encoding.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*ctxt).encoding as *mut c_void,
                );
            }
            (*ctxt).encoding = xmlStrndup(
                charset,
                last.offset_from(charset) as c_long as c_int,
            ) as *mut c_char;
        }
    } else if xmlStrncasecmp(
        line as *mut xmlChar,
        b"ContentType:\0" as *const u8 as *const c_char as *mut xmlChar,
        12 as c_int,
    ) == 0
    {
        let mut charset_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
        let mut last_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
        let mut mime_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
        cur = cur.offset(12 as c_int as isize);
        if !(*ctxt).contentType.is_null() {
            return;
        }
        while *cur as c_int == ' ' as i32 || *cur as c_int == '\t' as i32
        {
            cur = cur.offset(1);
        }
        (*ctxt).contentType = xmlMemStrdup.expect("non-null function pointer")(cur);
        mime_0 = cur as *const xmlChar;
        last_0 = mime_0;
        while *last_0 as c_int != 0 as c_int
            && *last_0 as c_int != ' ' as i32
            && *last_0 as c_int != '\t' as i32
            && *last_0 as c_int != ';' as i32
            && *last_0 as c_int != ',' as i32
        {
            last_0 = last_0.offset(1);
        }
        if !(*ctxt).mimeType.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ctxt).mimeType as *mut c_void,
            );
        }
        (*ctxt).mimeType = xmlStrndup(
            mime_0,
            last_0.offset_from(mime_0) as c_long as c_int,
        ) as *mut c_char;
        charset_0 = xmlStrstr(
            (*ctxt).contentType as *mut xmlChar,
            b"charset=\0" as *const u8 as *const c_char as *mut xmlChar,
        );
        if !charset_0.is_null() {
            charset_0 = charset_0.offset(8 as c_int as isize);
            last_0 = charset_0;
            while *last_0 as c_int != 0 as c_int
                && *last_0 as c_int != ' ' as i32
                && *last_0 as c_int != '\t' as i32
                && *last_0 as c_int != ';' as i32
                && *last_0 as c_int != ',' as i32
            {
                last_0 = last_0.offset(1);
            }
            if !(*ctxt).encoding.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*ctxt).encoding as *mut c_void,
                );
            }
            (*ctxt).encoding = xmlStrndup(
                charset_0,
                last_0.offset_from(charset_0) as c_long as c_int,
            ) as *mut c_char;
        }
    } else if xmlStrncasecmp(
        line as *mut xmlChar,
        b"Location:\0" as *const u8 as *const c_char as *mut xmlChar,
        9 as c_int,
    ) == 0
    {
        cur = cur.offset(9 as c_int as isize);
        while *cur as c_int == ' ' as i32 || *cur as c_int == '\t' as i32
        {
            cur = cur.offset(1);
        }
        if !(*ctxt).location.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ctxt).location as *mut c_void,
            );
        }
        if *cur as c_int == '/' as i32 {
            let mut tmp_http: *mut xmlChar =
                xmlStrdup(b"http://\0" as *const u8 as *const c_char as *mut xmlChar);
            let mut tmp_loc: *mut xmlChar = xmlStrcat(tmp_http, (*ctxt).hostname as *const xmlChar);
            (*ctxt).location =
                xmlStrcat(tmp_loc, cur as *const xmlChar) as *mut c_char;
        } else {
            (*ctxt).location = xmlMemStrdup.expect("non-null function pointer")(cur);
        }
    } else if xmlStrncasecmp(
        line as *mut xmlChar,
        b"WWW-Authenticate:\0" as *const u8 as *const c_char as *mut xmlChar,
        17 as c_int,
    ) == 0
    {
        cur = cur.offset(17 as c_int as isize);
        while *cur as c_int == ' ' as i32 || *cur as c_int == '\t' as i32
        {
            cur = cur.offset(1);
        }
        if !(*ctxt).authHeader.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ctxt).authHeader as *mut c_void,
            );
        }
        (*ctxt).authHeader = xmlMemStrdup.expect("non-null function pointer")(cur);
    } else if xmlStrncasecmp(
        line as *mut xmlChar,
        b"Proxy-Authenticate:\0" as *const u8 as *const c_char as *mut xmlChar,
        19 as c_int,
    ) == 0
    {
        cur = cur.offset(19 as c_int as isize);
        while *cur as c_int == ' ' as i32 || *cur as c_int == '\t' as i32
        {
            cur = cur.offset(1);
        }
        if !(*ctxt).authHeader.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ctxt).authHeader as *mut c_void,
            );
        }
        (*ctxt).authHeader = xmlMemStrdup.expect("non-null function pointer")(cur);
    } else if xmlStrncasecmp(
        line as *mut xmlChar,
        b"Content-Length:\0" as *const u8 as *const c_char as *mut xmlChar,
        15 as c_int,
    ) == 0
    {
        cur = cur.offset(15 as c_int as isize);
        (*ctxt).ContentLength = strtol(
            cur,
            ::core::ptr::null_mut::<*mut c_char>(),
            10 as c_int,
        ) as c_int;
    }
}
unsafe fn xmlNanoHTTPConnectAttempt(mut addr: *mut sockaddr) -> c_int {
    let mut p: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut status: c_int = 0;
    let mut addrlen: c_int = 0;
    let mut s: c_int = 0;
    s = socket(
        PF_INET,
        SOCK_STREAM as c_int,
        IPPROTO_TCP as c_int,
    );
    addrlen = ::core::mem::size_of::<sockaddr_in>() as c_int;
    if s == INVALID_SOCKET {
        __xmlIOErr(
            XML_FROM_HTTP as c_int,
            0 as c_int,
            b"socket failed\n\0" as *const u8 as *const c_char,
        );
        return INVALID_SOCKET;
    }
    status = fcntl(s, F_GETFL, 0 as c_int);
    if status != -(1 as c_int) {
        status |= O_NONBLOCK;
        status = fcntl(s, F_SETFL, status);
    }
    if status < 0 as c_int {
        __xmlIOErr(
            XML_FROM_HTTP as c_int,
            0 as c_int,
            b"error setting non-blocking IO\n\0" as *const u8 as *const c_char,
        );
        close(s);
        return INVALID_SOCKET;
    }
    if connect(s, addr, addrlen as socklen_t) == -(1 as c_int) {
        match socket_errno() {
            EINPROGRESS | EWOULDBLOCK => {}
            _ => {
                __xmlIOErr(
                    XML_FROM_HTTP as c_int,
                    0 as c_int,
                    b"error connecting to HTTP server\0" as *const u8 as *const c_char,
                );
                close(s);
                return INVALID_SOCKET;
            }
        }
    }
    p.fd = s;
    p.events = POLLOUT as c_short;
    match poll(
        &raw mut p,
        1 as nfds_t,
        timeout.wrapping_mul(1000 as c_uint) as c_int,
    ) {
        0 => {
            __xmlIOErr(
                XML_FROM_HTTP as c_int,
                0 as c_int,
                b"Connect attempt timed out\0" as *const u8 as *const c_char,
            );
            close(s);
            return INVALID_SOCKET;
        }
        -1 => {
            __xmlIOErr(
                XML_FROM_HTTP as c_int,
                0 as c_int,
                b"Connect failed\0" as *const u8 as *const c_char,
            );
            close(s);
            return INVALID_SOCKET;
        }
        _ => {}
    }
    if p.revents as c_int == POLLOUT {
        let mut len: socklen_t = 0;
        len = ::core::mem::size_of::<c_int>() as socklen_t;
        if getsockopt(
            s,
            SOL_SOCKET,
            SO_ERROR,
            &raw mut status as *mut c_char as *mut c_void,
            &raw mut len,
        ) < 0 as c_int
        {
            __xmlIOErr(
                XML_FROM_HTTP as c_int,
                0 as c_int,
                b"getsockopt failed\n\0" as *const u8 as *const c_char,
            );
            close(s);
            return INVALID_SOCKET;
        }
        if status != 0 {
            __xmlIOErr(
                XML_FROM_HTTP as c_int,
                0 as c_int,
                b"Error connecting to remote host\0" as *const u8 as *const c_char,
            );
            close(s);
            *__errno_location() = status;
            return INVALID_SOCKET;
        }
    } else {
        __xmlIOErr(
            XML_FROM_HTTP as c_int,
            0 as c_int,
            b"select failed\n\0" as *const u8 as *const c_char,
        );
        close(s);
        return INVALID_SOCKET;
    }
    return s;
}
unsafe fn xmlNanoHTTPConnectHost(
    mut host: *const c_char,
    mut port: c_int,
) -> c_int {
    let mut addr: *mut sockaddr = ::core::ptr::null_mut::<sockaddr>();
    let mut sockin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut s: c_int = 0;
    memset(
        &raw mut sockin as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<sockaddr_in>() as size_t,
    );
    let mut h: *mut hostent = ::core::ptr::null_mut::<hostent>();
    let mut ia: in_addr = in_addr { s_addr: 0 };
    let mut i: c_int = 0;
    h = gethostbyname(host as *mut c_char);
    if h.is_null() {
        let mut h_err_txt: *const c_char =
            b"\0" as *const u8 as *const c_char;
        match *__h_errno_location() {
            HOST_NOT_FOUND => {
                h_err_txt =
                    b"Authoritative host not found\0" as *const u8 as *const c_char;
            }
            TRY_AGAIN => {
                h_err_txt = b"Non-authoritative host not found or server failure.\0" as *const u8
                    as *const c_char;
            }
            NO_RECOVERY => {
                h_err_txt = b"Non-recoverable errors:  FORMERR, REFUSED, or NOTIMP.\0" as *const u8
                    as *const c_char;
            }
            NO_ADDRESS => {
                h_err_txt = b"Valid name, no data record of requested type.\0" as *const u8
                    as *const c_char;
            }
            _ => {
                h_err_txt = b"No error text defined.\0" as *const u8 as *const c_char;
            }
        }
        __xmlIOErr(
            XML_FROM_HTTP as c_int,
            0 as c_int,
            h_err_txt,
        );
        return INVALID_SOCKET;
    }
    i = 0 as c_int;
    while !(*(*h).h_addr_list.offset(i as isize)).is_null() {
        if !((*h).h_addrtype == AF_INET) {
            break;
        }
        if (*h).h_length as c_uint as usize
            > ::core::mem::size_of::<in_addr>() as usize
        {
            __xmlIOErr(
                XML_FROM_HTTP as c_int,
                0 as c_int,
                b"address size mismatch\n\0" as *const u8 as *const c_char,
            );
            return INVALID_SOCKET;
        }
        memcpy(
            &raw mut ia as *mut c_void,
            *(*h).h_addr_list.offset(i as isize) as *const c_void,
            (*h).h_length as size_t,
        );
        sockin.sin_family = (*h).h_addrtype as sa_family_t;
        sockin.sin_addr = ia;
        sockin.sin_port = __bswap_16(port as __uint16_t) as c_ushort as in_port_t;
        addr = &raw mut sockin as *mut sockaddr;
        s = xmlNanoHTTPConnectAttempt(addr);
        if s != INVALID_SOCKET {
            return s;
        }
        i += 1;
    }
    return INVALID_SOCKET;
}
#[inline]
pub unsafe fn xmlNanoHTTPOpen(
    mut URL: *const c_char,
    mut contentType: *mut *mut c_char,
) -> *mut c_void {
    if !contentType.is_null() {
        *contentType = ::core::ptr::null_mut::<c_char>();
    }
    return xmlNanoHTTPMethod(
        URL,
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        contentType,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
    );
}
#[inline]
pub unsafe fn xmlNanoHTTPOpenRedir(
    mut URL: *const c_char,
    mut contentType: *mut *mut c_char,
    mut redir: *mut *mut c_char,
) -> *mut c_void {
    if !contentType.is_null() {
        *contentType = ::core::ptr::null_mut::<c_char>();
    }
    if !redir.is_null() {
        *redir = ::core::ptr::null_mut::<c_char>();
    }
    return xmlNanoHTTPMethodRedir(
        URL,
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        contentType,
        redir,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
    );
}
#[inline]
pub unsafe fn xmlNanoHTTPRead(
    mut ctx: *mut c_void,
    mut dest: *mut c_void,
    mut len: c_int,
) -> c_int {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ctx as xmlNanoHTTPCtxtPtr;
    if ctx.is_null() {
        return -(1 as c_int);
    }
    if dest.is_null() {
        return -(1 as c_int);
    }
    if len <= 0 as c_int {
        return 0 as c_int;
    }
    while ((*ctxt).inptr.offset_from((*ctxt).inrptr) as c_long)
        < len as c_long
    {
        if xmlNanoHTTPRecv(ctxt) <= 0 as c_int {
            break;
        }
    }
    if ((*ctxt).inptr.offset_from((*ctxt).inrptr) as c_long)
        < len as c_long
    {
        len =
            (*ctxt).inptr.offset_from((*ctxt).inrptr) as c_long as c_int;
    }
    memcpy(
        dest,
        (*ctxt).inrptr as *const c_void,
        len as size_t,
    );
    (*ctxt).inrptr = (*ctxt).inrptr.offset(len as isize);
    return len;
}
#[inline]
pub unsafe fn xmlNanoHTTPClose(mut ctx: *mut c_void) {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ctx as xmlNanoHTTPCtxtPtr;
    if ctx.is_null() {
        return;
    }
    xmlNanoHTTPFreeCtxt(ctxt);
}
unsafe fn xmlNanoHTTPHostnameMatch(
    mut pattern: *const c_char,
    mut hostname: *const c_char,
) -> c_int {
    let mut idx_pattern: c_int = 0;
    let mut idx_hostname: c_int = 0;
    let mut pattern_start: *const c_char = ::core::ptr::null::<c_char>();
    if pattern.is_null() || *pattern as c_int == '\0' as i32 || hostname.is_null() {
        return 0 as c_int;
    }
    if *pattern as c_int == '.' as i32 {
        idx_pattern = strlen(pattern).wrapping_sub(1 as size_t) as c_int;
        pattern_start = pattern.offset(1 as c_int as isize);
    } else {
        idx_pattern = strlen(pattern) as c_int;
        pattern_start = pattern;
    }
    idx_hostname = strlen(hostname) as c_int;
    while idx_pattern >= 0 as c_int && idx_hostname >= 0 as c_int {
        if ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_char>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int =
                        *pattern_start.offset(idx_pattern as isize) as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res =
                        tolower(*pattern_start.offset(idx_pattern as isize) as c_int);
                }
            } else {
                __res =
                    *(*__ctype_tolower_loc()).offset(*pattern_start.offset(idx_pattern as isize)
                        as c_int
                        as isize) as c_int;
            }
            __res
        }) != ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_char>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int =
                        *hostname.offset(idx_hostname as isize) as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res = tolower(*hostname.offset(idx_hostname as isize) as c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*hostname.offset(idx_hostname as isize) as c_int as isize)
                    as c_int;
            }
            __res
        }) {
            break;
        }
        idx_pattern -= 1;
        idx_hostname -= 1;
    }
    return (idx_pattern == -(1 as c_int)
        && (idx_hostname == -(1 as c_int)
            || *hostname.offset(idx_hostname as isize) as c_int == '.' as i32))
        as c_int;
}
unsafe fn xmlNanoHTTPBypassProxy(
    mut hostname: *const c_char,
) -> c_int {
    let mut envlen: size_t = 0;
    let mut env: *mut c_char =
        getenv(b"no_proxy\0" as *const u8 as *const c_char);
    let mut cpy: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut p: *mut c_char = ::core::ptr::null_mut::<c_char>();
    if env.is_null() {
        return 0 as c_int;
    }
    envlen = strlen(env).wrapping_add(1 as size_t);
    cpy = xmlMalloc.expect("non-null function pointer")(envlen) as *mut c_char;
    memcpy(
        cpy as *mut c_void,
        env as *const c_void,
        envlen,
    );
    env = cpy;
    while *(*__ctype_b_loc()).offset(*env as c_int as isize) as c_int
        & _ISspace as c_int as c_ushort as c_int
        != 0
    {
        env = env.offset(1);
    }
    if *env as c_int == '\0' as i32 {
        xmlFree.expect("non-null function pointer")(cpy as *mut c_void);
        return 0 as c_int;
    }
    p = env;
    while *env != 0 {
        if *env as c_int != ',' as i32 {
            env = env.offset(1);
        } else {
            let fresh1 = env;
            env = env.offset(1);
            *fresh1 = '\0' as i32 as c_char;
            if xmlNanoHTTPHostnameMatch(p, hostname) != 0 {
                xmlFree.expect("non-null function pointer")(cpy as *mut c_void);
                return 1 as c_int;
            }
            while *(*__ctype_b_loc()).offset(*env as c_int as isize)
                as c_int
                & _ISspace as c_int as c_ushort as c_int
                != 0
            {
                env = env.offset(1);
            }
            p = env;
        }
    }
    if xmlNanoHTTPHostnameMatch(p, hostname) != 0 {
        xmlFree.expect("non-null function pointer")(cpy as *mut c_void);
        return 1 as c_int;
    }
    xmlFree.expect("non-null function pointer")(cpy as *mut c_void);
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlNanoHTTPMethodRedir(
    mut URL: *const c_char,
    mut method: *const c_char,
    mut input: *const c_char,
    mut contentType: *mut *mut c_char,
    mut redir: *mut *mut c_char,
    mut headers: *const c_char,
    mut ilen: c_int,
) -> *mut c_void {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ::core::ptr::null_mut::<xmlNanoHTTPCtxt>();
    let mut bp: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut p: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut blen: c_int = 0;
    let mut ret: c_int = 0;
    let mut nbRedirects: c_int = 0 as c_int;
    let mut use_proxy: c_int = 0;
    let mut redirURL: *mut c_char = ::core::ptr::null_mut::<c_char>();
    if URL.is_null() {
        return ::core::ptr::null_mut::<c_void>();
    }
    if method.is_null() {
        method = b"GET\0" as *const u8 as *const c_char;
    }
    xmlNanoHTTPInit();
    loop {
        if redirURL.is_null() {
            ctxt = xmlNanoHTTPNewCtxt(URL);
            if ctxt.is_null() {
                return ::core::ptr::null_mut::<c_void>();
            }
        } else {
            ctxt = xmlNanoHTTPNewCtxt(redirURL);
            if ctxt.is_null() {
                return ::core::ptr::null_mut::<c_void>();
            }
            (*ctxt).location = xmlMemStrdup.expect("non-null function pointer")(redirURL);
        }
        if (*ctxt).protocol.is_null()
            || strcmp(
                (*ctxt).protocol,
                b"http\0" as *const u8 as *const c_char,
            ) != 0
        {
            __xmlIOErr(
                XML_FROM_HTTP as c_int,
                XML_HTTP_URL_SYNTAX as c_int,
                b"Not a valid HTTP URI\0" as *const u8 as *const c_char,
            );
            xmlNanoHTTPFreeCtxt(ctxt);
            if !redirURL.is_null() {
                xmlFree.expect("non-null function pointer")(redirURL as *mut c_void);
            }
            return ::core::ptr::null_mut::<c_void>();
        }
        if (*ctxt).hostname.is_null() {
            __xmlIOErr(
                XML_FROM_HTTP as c_int,
                XML_HTTP_UNKNOWN_HOST as c_int,
                b"Failed to identify host in URI\0" as *const u8 as *const c_char,
            );
            xmlNanoHTTPFreeCtxt(ctxt);
            if !redirURL.is_null() {
                xmlFree.expect("non-null function pointer")(redirURL as *mut c_void);
            }
            return ::core::ptr::null_mut::<c_void>();
        }
        use_proxy = (!proxy.is_null() && xmlNanoHTTPBypassProxy((*ctxt).hostname) == 0)
            as c_int;
        if use_proxy != 0 {
            blen = strlen((*ctxt).hostname)
                .wrapping_mul(2 as size_t)
                .wrapping_add(16 as size_t) as c_int;
            ret = xmlNanoHTTPConnectHost(proxy, proxyPort);
        } else {
            blen = strlen((*ctxt).hostname) as c_int;
            ret = xmlNanoHTTPConnectHost((*ctxt).hostname, (*ctxt).port);
        }
        if ret == INVALID_SOCKET {
            xmlNanoHTTPFreeCtxt(ctxt);
            if !redirURL.is_null() {
                xmlFree.expect("non-null function pointer")(redirURL as *mut c_void);
            }
            return ::core::ptr::null_mut::<c_void>();
        }
        (*ctxt).fd = ret;
        if input.is_null() {
            ilen = 0 as c_int;
        } else {
            blen += 36 as c_int;
        }
        if !headers.is_null() {
            blen = (blen as c_ulong)
                .wrapping_add(strlen(headers).wrapping_add(2 as size_t) as c_ulong)
                as c_int as c_int;
        }
        if !contentType.is_null() && !(*contentType).is_null() {
            blen = (blen as c_ulong).wrapping_add(
                strlen(*contentType).wrapping_add(16 as size_t) as c_ulong,
            ) as c_int as c_int;
        }
        if !(*ctxt).query.is_null() {
            blen = (blen as c_ulong).wrapping_add(
                strlen((*ctxt).query).wrapping_add(1 as size_t) as c_ulong,
            ) as c_int as c_int;
        }
        blen = (blen as c_ulong).wrapping_add(
            strlen(method)
                .wrapping_add(strlen((*ctxt).path))
                .wrapping_add(24 as size_t) as c_ulong,
        ) as c_int as c_int;
        if (*ctxt).port != 80 as c_int {
            if use_proxy != 0 {
                blen += 17 as c_int;
            } else {
                blen += 11 as c_int;
            }
        }
        bp = xmlMallocAtomic.expect("non-null function pointer")(blen as size_t)
            as *mut c_char;
        if bp.is_null() {
            xmlNanoHTTPFreeCtxt(ctxt);
            xmlHTTPErrMemory(
                b"allocating header buffer\0" as *const u8 as *const c_char,
            );
            return ::core::ptr::null_mut::<c_void>();
        }
        p = bp;
        if use_proxy != 0 {
            if (*ctxt).port != 80 as c_int {
                p = p.offset(snprintf(
                    p,
                    (blen as c_long - p.offset_from(bp) as c_long)
                        as size_t,
                    b"%s http://%s:%d%s\0" as *const u8 as *const c_char,
                    method,
                    (*ctxt).hostname,
                    (*ctxt).port,
                    (*ctxt).path,
                ) as isize);
            } else {
                p = p.offset(snprintf(
                    p,
                    (blen as c_long - p.offset_from(bp) as c_long)
                        as size_t,
                    b"%s http://%s%s\0" as *const u8 as *const c_char,
                    method,
                    (*ctxt).hostname,
                    (*ctxt).path,
                ) as isize);
            }
        } else {
            p = p.offset(snprintf(
                p,
                (blen as c_long - p.offset_from(bp) as c_long) as size_t,
                b"%s %s\0" as *const u8 as *const c_char,
                method,
                (*ctxt).path,
            ) as isize);
        }
        if !(*ctxt).query.is_null() {
            p = p.offset(snprintf(
                p,
                (blen as c_long - p.offset_from(bp) as c_long) as size_t,
                b"?%s\0" as *const u8 as *const c_char,
                (*ctxt).query,
            ) as isize);
        }
        if (*ctxt).port == 80 as c_int {
            p = p.offset(snprintf(
                p,
                (blen as c_long - p.offset_from(bp) as c_long) as size_t,
                b" HTTP/1.0\r\nHost: %s\r\n\0" as *const u8 as *const c_char,
                (*ctxt).hostname,
            ) as isize);
        } else {
            p = p.offset(snprintf(
                p,
                (blen as c_long - p.offset_from(bp) as c_long) as size_t,
                b" HTTP/1.0\r\nHost: %s:%d\r\n\0" as *const u8 as *const c_char,
                (*ctxt).hostname,
                (*ctxt).port,
            ) as isize);
        }
        if !contentType.is_null() && !(*contentType).is_null() {
            p = p.offset(snprintf(
                p,
                (blen as c_long - p.offset_from(bp) as c_long) as size_t,
                b"Content-Type: %s\r\n\0" as *const u8 as *const c_char,
                *contentType,
            ) as isize);
        }
        if !headers.is_null() {
            p = p.offset(snprintf(
                p,
                (blen as c_long - p.offset_from(bp) as c_long) as size_t,
                b"%s\0" as *const u8 as *const c_char,
                headers,
            ) as isize);
        }
        if !input.is_null() {
            snprintf(
                p,
                (blen as c_long - p.offset_from(bp) as c_long) as size_t,
                b"Content-Length: %d\r\n\r\n\0" as *const u8 as *const c_char,
                ilen,
            );
        } else {
            snprintf(
                p,
                (blen as c_long - p.offset_from(bp) as c_long) as size_t,
                b"\r\n\0" as *const u8 as *const c_char,
            );
        }
        (*ctxt).out = bp;
        (*ctxt).outptr = (*ctxt).out;
        (*ctxt).state = XML_NANO_HTTP_WRITE;
        blen = strlen((*ctxt).out) as c_int;
        xmlNanoHTTPSend(ctxt, (*ctxt).out, blen);
        if !input.is_null() {
            xmlNanoHTTPSend(ctxt, input, ilen);
        }
        (*ctxt).state = XML_NANO_HTTP_READ;
        loop {
            p = xmlNanoHTTPReadLine(ctxt);
            if p.is_null() {
                break;
            }
            if *p as c_int == 0 as c_int {
                (*ctxt).content = (*ctxt).inrptr;
                xmlFree.expect("non-null function pointer")(p as *mut c_void);
                break;
            } else {
                xmlNanoHTTPScanAnswer(ctxt, p);
                xmlFree.expect("non-null function pointer")(p as *mut c_void);
            }
        }
        if !(*ctxt).location.is_null()
            && (*ctxt).returnValue >= 300 as c_int
            && (*ctxt).returnValue < 400 as c_int
        {
            while xmlNanoHTTPRecv(ctxt) > 0 as c_int {}
            if nbRedirects < XML_NANO_HTTP_MAX_REDIR {
                nbRedirects += 1;
                if !redirURL.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        redirURL as *mut c_void,
                    );
                }
                redirURL = xmlMemStrdup.expect("non-null function pointer")((*ctxt).location);
                xmlNanoHTTPFreeCtxt(ctxt);
            } else {
                xmlNanoHTTPFreeCtxt(ctxt);
                if !redirURL.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        redirURL as *mut c_void,
                    );
                }
                return ::core::ptr::null_mut::<c_void>();
            }
        } else {
            if !contentType.is_null() {
                if !(*ctxt).contentType.is_null() {
                    *contentType =
                        xmlMemStrdup.expect("non-null function pointer")((*ctxt).contentType);
                } else {
                    *contentType = ::core::ptr::null_mut::<c_char>();
                }
            }
            if !redir.is_null() && !redirURL.is_null() {
                *redir = redirURL;
            } else {
                if !redirURL.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        redirURL as *mut c_void,
                    );
                }
                if !redir.is_null() {
                    *redir = ::core::ptr::null_mut::<c_char>();
                }
            }
            return ctxt as *mut c_void;
        }
    }
}
#[inline]
pub unsafe fn xmlNanoHTTPMethod(
    mut URL: *const c_char,
    mut method: *const c_char,
    mut input: *const c_char,
    mut contentType: *mut *mut c_char,
    mut headers: *const c_char,
    mut ilen: c_int,
) -> *mut c_void {
    return xmlNanoHTTPMethodRedir(
        URL,
        method,
        input,
        contentType,
        ::core::ptr::null_mut::<*mut c_char>(),
        headers,
        ilen,
    );
}
#[inline]
pub unsafe fn xmlNanoHTTPFetch(
    mut URL: *const c_char,
    mut filename: *const c_char,
    mut contentType: *mut *mut c_char,
) -> c_int {
    let mut ctxt: *mut c_void = NULL;
    let mut buf: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut fd: c_int = 0;
    let mut len: c_int = 0;
    let mut ret: c_int = 0 as c_int;
    if filename.is_null() {
        return -(1 as c_int);
    }
    ctxt = xmlNanoHTTPOpen(URL, contentType);
    if ctxt.is_null() {
        return -(1 as c_int);
    }
    if strcmp(filename, b"-\0" as *const u8 as *const c_char) == 0 {
        fd = 0 as c_int;
    } else {
        fd = open(filename, O_CREAT | O_WRONLY, 0o644 as c_int);
        if fd < 0 as c_int {
            xmlNanoHTTPClose(ctxt);
            if !contentType.is_null() && !(*contentType).is_null() {
                xmlFree.expect("non-null function pointer")(
                    *contentType as *mut c_void,
                );
                *contentType = ::core::ptr::null_mut::<c_char>();
            }
            return -(1 as c_int);
        }
    }
    xmlNanoHTTPFetchContent(ctxt, &raw mut buf, &raw mut len);
    if len > 0 as c_int {
        if write(fd, buf as *const c_void, len as size_t)
            == -(1 as c_int) as ssize_t
        {
            ret = -(1 as c_int);
        }
    }
    xmlNanoHTTPClose(ctxt);
    close(fd);
    return ret;
}
#[inline]
pub unsafe fn xmlNanoHTTPSave(
    mut ctxt: *mut c_void,
    mut filename: *const c_char,
) -> c_int {
    let mut buf: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut fd: c_int = 0;
    let mut len: c_int = 0;
    let mut ret: c_int = 0 as c_int;
    if ctxt.is_null() || filename.is_null() {
        return -(1 as c_int);
    }
    if strcmp(filename, b"-\0" as *const u8 as *const c_char) == 0 {
        fd = 0 as c_int;
    } else {
        fd = open(filename, O_CREAT | O_WRONLY, 0o666 as c_int);
        if fd < 0 as c_int {
            xmlNanoHTTPClose(ctxt);
            return -(1 as c_int);
        }
    }
    xmlNanoHTTPFetchContent(ctxt, &raw mut buf, &raw mut len);
    if len > 0 as c_int {
        if write(fd, buf as *const c_void, len as size_t)
            == -(1 as c_int) as ssize_t
        {
            ret = -(1 as c_int);
        }
    }
    xmlNanoHTTPClose(ctxt);
    close(fd);
    return ret;
}
#[inline]
pub unsafe fn xmlNanoHTTPReturnCode(
    mut ctx: *mut c_void,
) -> c_int {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ctx as xmlNanoHTTPCtxtPtr;
    if ctxt.is_null() {
        return -(1 as c_int);
    }
    return (*ctxt).returnValue;
}
#[inline]
pub unsafe fn xmlNanoHTTPAuthHeader(
    mut ctx: *mut c_void,
) -> *const c_char {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ctx as xmlNanoHTTPCtxtPtr;
    if ctxt.is_null() {
        return ::core::ptr::null::<c_char>();
    }
    return (*ctxt).authHeader;
}
#[inline]
pub unsafe fn xmlNanoHTTPContentLength(
    mut ctx: *mut c_void,
) -> c_int {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ctx as xmlNanoHTTPCtxtPtr;
    return if ctxt.is_null() {
        -(1 as c_int)
    } else {
        (*ctxt).ContentLength
    };
}
#[inline]
pub unsafe fn xmlNanoHTTPRedir(
    mut ctx: *mut c_void,
) -> *const c_char {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ctx as xmlNanoHTTPCtxtPtr;
    return if ctxt.is_null() {
        ::core::ptr::null_mut::<c_char>()
    } else {
        (*ctxt).location
    };
}
#[inline]
pub unsafe fn xmlNanoHTTPEncoding(
    mut ctx: *mut c_void,
) -> *const c_char {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ctx as xmlNanoHTTPCtxtPtr;
    return if ctxt.is_null() {
        ::core::ptr::null_mut::<c_char>()
    } else {
        (*ctxt).encoding
    };
}
#[inline]
pub unsafe fn xmlNanoHTTPMimeType(
    mut ctx: *mut c_void,
) -> *const c_char {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ctx as xmlNanoHTTPCtxtPtr;
    return if ctxt.is_null() {
        ::core::ptr::null_mut::<c_char>()
    } else {
        (*ctxt).mimeType
    };
}
unsafe fn xmlNanoHTTPFetchContent(
    mut ctx: *mut c_void,
    mut ptr: *mut *mut c_char,
    mut len: *mut c_int,
) -> c_int {
    let mut ctxt: xmlNanoHTTPCtxtPtr = ctx as xmlNanoHTTPCtxtPtr;
    let mut rc: c_int = 0 as c_int;
    let mut cur_lgth: c_int = 0;
    let mut rcvd_lgth: c_int = 0;
    let mut dummy_int: c_int = 0;
    let mut dummy_ptr: *mut c_char = ::core::ptr::null_mut::<c_char>();
    if len.is_null() {
        len = &raw mut dummy_int;
    }
    if ptr.is_null() {
        ptr = &raw mut dummy_ptr;
    }
    if ctxt.is_null() || (*ctxt).content.is_null() {
        *len = 0 as c_int;
        *ptr = ::core::ptr::null_mut::<c_char>();
        return -(1 as c_int);
    }
    rcvd_lgth =
        (*ctxt).inptr.offset_from((*ctxt).content) as c_long as c_int;
    loop {
        cur_lgth = xmlNanoHTTPRecv(ctxt);
        if !(cur_lgth > 0 as c_int) {
            break;
        }
        rcvd_lgth += cur_lgth;
        if (*ctxt).ContentLength > 0 as c_int && rcvd_lgth >= (*ctxt).ContentLength {
            break;
        }
    }
    *ptr = (*ctxt).content;
    *len = rcvd_lgth;
    if (*ctxt).ContentLength > 0 as c_int && rcvd_lgth < (*ctxt).ContentLength {
        rc = -(1 as c_int);
    } else if rcvd_lgth == 0 as c_int {
        rc = -(1 as c_int);
    }
    return rc;
}
