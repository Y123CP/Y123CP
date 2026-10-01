use core::ffi::*;
use crate::src::xmlstring::xmlStrEqual;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;
pub use crate::src::dict::_xmlDict;
extern "C" {
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr, nameSpace: *const xmlChar) -> xmlNsPtr;
    fn xmlGetNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> *mut xmlChar;
}

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

pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type xmlNsPtr = *mut xmlNs;
pub type xlinkHRef = *mut xmlChar;
pub type xlinkRole = *mut xmlChar;
pub type xlinkTitle = *mut xmlChar;
pub type xlinkType = c_uint;
pub const XLINK_TYPE_EXTENDED_SET: xlinkType = 3;
pub const XLINK_TYPE_EXTENDED: xlinkType = 2;
pub const XLINK_TYPE_SIMPLE: xlinkType = 1;
pub const XLINK_TYPE_NONE: xlinkType = 0;
pub type xlinkShow = c_uint;
pub const XLINK_SHOW_REPLACE: xlinkShow = 3;
pub const XLINK_SHOW_EMBED: xlinkShow = 2;
pub const XLINK_SHOW_NEW: xlinkShow = 1;
pub const XLINK_SHOW_NONE: xlinkShow = 0;
pub type xlinkActuate = c_uint;
pub const XLINK_ACTUATE_ONREQUEST: xlinkActuate = 2;
pub const XLINK_ACTUATE_AUTO: xlinkActuate = 1;
pub const XLINK_ACTUATE_NONE: xlinkActuate = 0;
pub type xlinkNodeDetectFunc =
    Option<unsafe extern "C" fn(*mut c_void, xmlNodePtr) -> ()>;
pub type xlinkSimpleLinkFunk = Option<
    unsafe extern "C" fn(
        *mut c_void,
        xmlNodePtr,
        xlinkHRef,
        xlinkRole,
        xlinkTitle,
    ) -> (),
>;
pub type xlinkExtendedLinkFunk = Option<
    unsafe extern "C" fn(
        *mut c_void,
        xmlNodePtr,
        c_int,
        *const xlinkHRef,
        *const xlinkRole,
        c_int,
        *const xlinkRole,
        *const xlinkRole,
        *mut xlinkShow,
        *mut xlinkActuate,
        c_int,
        *const xlinkTitle,
        *mut *const xmlChar,
    ) -> (),
>;
pub type xlinkExtendedLinkSetFunk = Option<
    unsafe extern "C" fn(
        *mut c_void,
        xmlNodePtr,
        c_int,
        *const xlinkHRef,
        *const xlinkRole,
        c_int,
        *const xlinkTitle,
        *mut *const xmlChar,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xlinkHandler {
    pub simple: xlinkSimpleLinkFunk,
    pub extended: xlinkExtendedLinkFunk,
    pub set: xlinkExtendedLinkSetFunk,
}
pub type xlinkHandler = _xlinkHandler;
pub type xlinkHandlerPtr = *mut xlinkHandler;

pub const XLINK_NAMESPACE: *mut xmlChar = b"http://www.w3.org/1999/xlink/namespace/\0" as *const u8
    as *const c_char as *mut xmlChar;
pub const XHTML_NAMESPACE: *mut xmlChar =
    b"http://www.w3.org/1999/xhtml/\0" as *const u8 as *const c_char as *mut xmlChar;
static mut xlinkDefaultHandler: xlinkHandlerPtr =
    ::core::ptr::null::<xlinkHandler>() as *mut xlinkHandler;
static mut xlinkDefaultDetect: xlinkNodeDetectFunc = None;
#[inline]
pub fn xlinkGetDefaultHandler() -> xlinkHandlerPtr { unsafe {
    return xlinkDefaultHandler;
} }
#[inline]
pub fn xlinkSetDefaultHandler(mut handler: xlinkHandlerPtr) { unsafe {
    xlinkDefaultHandler = handler;
} }
#[inline]
pub fn xlinkGetDefaultDetect() -> xlinkNodeDetectFunc { unsafe {
    return xlinkDefaultDetect;
} }
#[inline]
pub fn xlinkSetDefaultDetect(mut func: xlinkNodeDetectFunc) { unsafe {
    xlinkDefaultDetect = func;
} }
#[inline]
pub fn xlinkIsLink(mut doc: xmlDocPtr, mut node: xmlNodePtr) -> xlinkType { unsafe {
    let mut type_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut role: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ret: xlinkType = XLINK_TYPE_NONE;
    if node.is_null() {
        return XLINK_TYPE_NONE;
    }
    if doc.is_null() {
        doc = (*node).doc as xmlDocPtr;
    }
    if !(!doc.is_null()
        && (*doc).type_0 as c_uint
            == XML_HTML_DOCUMENT_NODE as c_int as c_uint)
    {
        !(*node).ns.is_null() && xmlStrEqual((*(*node).ns).href, XHTML_NAMESPACE) != 0;
    }
    type_0 = xmlGetNsProp(
        node as *const xmlNode,
        b"type\0" as *const u8 as *const c_char as *mut xmlChar,
        XLINK_NAMESPACE,
    );
    if !type_0.is_null() {
        if xmlStrEqual(
            type_0,
            b"simple\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
        {
            ret = XLINK_TYPE_SIMPLE;
        } else if xmlStrEqual(
            type_0,
            b"extended\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
        {
            role = xmlGetNsProp(
                node as *const xmlNode,
                b"role\0" as *const u8 as *const c_char as *mut xmlChar,
                XLINK_NAMESPACE,
            );
            if !role.is_null() {
                let mut xlink: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                xlink = xmlSearchNs(doc, node, XLINK_NAMESPACE);
                if xlink.is_null() {
                    if xmlStrEqual(
                        role,
                        b"xlink:external-linkset\0" as *const u8 as *const c_char
                            as *mut xmlChar,
                    ) != 0
                    {
                        ret = XLINK_TYPE_EXTENDED_SET;
                    }
                } else {
                    let mut buf: [xmlChar; 200] = [0; 200];
                    snprintf(
                        &raw mut buf as *mut xmlChar as *mut c_char,
                        ::core::mem::size_of::<[xmlChar; 200]>() as size_t,
                        b"%s:external-linkset\0" as *const u8 as *const c_char,
                        (*xlink).prefix as *mut c_char,
                    );
                    buf[(::core::mem::size_of::<[xmlChar; 200]>() as usize).wrapping_sub(1 as usize)
                        as usize] = 0 as xmlChar;
                    if xmlStrEqual(role, &raw mut buf as *mut xmlChar) != 0 {
                        ret = XLINK_TYPE_EXTENDED_SET;
                    }
                }
            }
            ret = XLINK_TYPE_EXTENDED;
        }
    }
    if !type_0.is_null() {
        xmlFree.expect("non-null function pointer")(type_0 as *mut c_void);
    }
    if !role.is_null() {
        xmlFree.expect("non-null function pointer")(role as *mut c_void);
    }
    return ret;
} }
