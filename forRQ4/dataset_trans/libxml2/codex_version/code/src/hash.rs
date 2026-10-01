extern "C" {
    pub type _xmlDict;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn xmlStrQEqual(
        pref: *const xmlChar,
        name: *const xmlChar,
        str: *const xmlChar,
    ) -> ::core::ffi::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlDictReference(dict: xmlDictPtr) -> ::core::ffi::c_int;
    fn xmlDictFree(dict: xmlDictPtr);
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> *const xmlChar;
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlRandom() -> ::core::ffi::c_uint;
    fn xmlInitParser();
}
pub type size_t = usize;
pub type xmlChar = ::core::ffi::c_uchar;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlHashTable {
    pub table: *mut xmlHashEntry,
    pub size: ::core::ffi::c_uint,
    pub nbElems: ::core::ffi::c_uint,
    pub dict: xmlDictPtr,
    pub randomSeed: ::core::ffi::c_uint,
}
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlHashEntry {
    pub hashValue: ::core::ffi::c_uint,
    pub key: *mut xmlChar,
    pub key2: *mut xmlChar,
    pub key3: *mut xmlChar,
    pub payload: *mut ::core::ffi::c_void,
}
pub type xmlHashDeallocator =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type xmlHashCopier = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> *mut ::core::ffi::c_void,
>;
pub type xmlHashScanner = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *const xmlChar) -> (),
>;
pub type xmlHashScannerFull = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stubData {
    pub scan: xmlHashScanner,
    pub data: *mut ::core::ffi::c_void,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIZE_MAX: size_t = -(1 as ::core::ffi::c_int) as size_t;
pub const MAX_FILL_NUM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const MAX_FILL_DENOM: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MIN_HASH_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MAX_HASH_SIZE: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int;
unsafe extern "C" fn xmlHashValue(
    mut seed: ::core::ffi::c_uint,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
    mut lengths: *mut size_t,
) -> ::core::ffi::c_uint {
    let mut h1: ::core::ffi::c_uint = 0;
    let mut h2: ::core::ffi::c_uint = 0;
    let mut i: size_t = 0;
    h1 = seed ^ 0x3b00 as ::core::ffi::c_uint;
    h2 = seed << 15 as ::core::ffi::c_int
        | (seed & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    i = 0 as size_t;
    while *key.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        h1 = h1.wrapping_add(*key.offset(i as isize) as ::core::ffi::c_uint);
        h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    if !lengths.is_null() {
        *lengths.offset(0 as ::core::ffi::c_int as isize) = i;
    }
    h1 = h1.wrapping_add(0 as ::core::ffi::c_uint);
    h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
    h2 = h2.wrapping_add(h1);
    h2 = h2 << 7 as ::core::ffi::c_int
        | (h2 & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
    if !key2.is_null() {
        i = 0 as size_t;
        while *key2.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            h1 = h1.wrapping_add(*key2.offset(i as isize) as ::core::ffi::c_uint);
            h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
            h2 = h2.wrapping_add(h1);
            h2 = h2 << 7 as ::core::ffi::c_int
                | (h2 & 0xffffffff as ::core::ffi::c_uint)
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
            i = i.wrapping_add(1);
        }
        if !lengths.is_null() {
            *lengths.offset(1 as ::core::ffi::c_int as isize) = i;
        }
    }
    h1 = h1.wrapping_add(0 as ::core::ffi::c_uint);
    h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
    h2 = h2.wrapping_add(h1);
    h2 = h2 << 7 as ::core::ffi::c_int
        | (h2 & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
    if !key3.is_null() {
        i = 0 as size_t;
        while *key3.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            h1 = h1.wrapping_add(*key3.offset(i as isize) as ::core::ffi::c_uint);
            h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
            h2 = h2.wrapping_add(h1);
            h2 = h2 << 7 as ::core::ffi::c_int
                | (h2 & 0xffffffff as ::core::ffi::c_uint)
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
            i = i.wrapping_add(1);
        }
        if !lengths.is_null() {
            *lengths.offset(2 as ::core::ffi::c_int as isize) = i;
        }
    }
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 14 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as ::core::ffi::c_uint) >> 6 as ::core::ffi::c_int
            | h1 << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int,
    );
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 5 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
            | h1 << 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int,
    );
    h2 &= 0xffffffff as ::core::ffi::c_uint;
    return h2;
}
unsafe extern "C" fn xmlHashQNameValue(
    mut seed: ::core::ffi::c_uint,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut prefix2: *const xmlChar,
    mut name2: *const xmlChar,
    mut prefix3: *const xmlChar,
    mut name3: *const xmlChar,
) -> ::core::ffi::c_uint {
    let mut h1: ::core::ffi::c_uint = 0;
    let mut h2: ::core::ffi::c_uint = 0;
    let mut ch: ::core::ffi::c_uint = 0;
    h1 = seed ^ 0x3b00 as ::core::ffi::c_uint;
    h2 = seed << 15 as ::core::ffi::c_int
        | (seed & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    if !prefix.is_null() {
        loop {
            let fresh0 = prefix;
            prefix = prefix.offset(1);
            ch = *fresh0 as ::core::ffi::c_uint;
            if !(ch != 0 as ::core::ffi::c_uint) {
                break;
            }
            h1 = h1.wrapping_add(ch);
            h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
            h2 = h2.wrapping_add(h1);
            h2 = h2 << 7 as ::core::ffi::c_int
                | (h2 & 0xffffffff as ::core::ffi::c_uint)
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        }
        h1 = h1.wrapping_add(':' as i32 as ::core::ffi::c_uint);
        h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
    }
    if !name.is_null() {
        loop {
            let fresh1 = name;
            name = name.offset(1);
            ch = *fresh1 as ::core::ffi::c_uint;
            if !(ch != 0 as ::core::ffi::c_uint) {
                break;
            }
            h1 = h1.wrapping_add(ch);
            h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
            h2 = h2.wrapping_add(h1);
            h2 = h2 << 7 as ::core::ffi::c_int
                | (h2 & 0xffffffff as ::core::ffi::c_uint)
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        }
    }
    h1 = h1.wrapping_add(0 as ::core::ffi::c_uint);
    h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
    h2 = h2.wrapping_add(h1);
    h2 = h2 << 7 as ::core::ffi::c_int
        | (h2 & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
    if !prefix2.is_null() {
        loop {
            let fresh2 = prefix2;
            prefix2 = prefix2.offset(1);
            ch = *fresh2 as ::core::ffi::c_uint;
            if !(ch != 0 as ::core::ffi::c_uint) {
                break;
            }
            h1 = h1.wrapping_add(ch);
            h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
            h2 = h2.wrapping_add(h1);
            h2 = h2 << 7 as ::core::ffi::c_int
                | (h2 & 0xffffffff as ::core::ffi::c_uint)
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        }
        h1 = h1.wrapping_add(':' as i32 as ::core::ffi::c_uint);
        h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
    }
    if !name2.is_null() {
        loop {
            let fresh3 = name2;
            name2 = name2.offset(1);
            ch = *fresh3 as ::core::ffi::c_uint;
            if !(ch != 0 as ::core::ffi::c_uint) {
                break;
            }
            h1 = h1.wrapping_add(ch);
            h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
            h2 = h2.wrapping_add(h1);
            h2 = h2 << 7 as ::core::ffi::c_int
                | (h2 & 0xffffffff as ::core::ffi::c_uint)
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        }
    }
    h1 = h1.wrapping_add(0 as ::core::ffi::c_uint);
    h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
    h2 = h2.wrapping_add(h1);
    h2 = h2 << 7 as ::core::ffi::c_int
        | (h2 & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
    if !prefix3.is_null() {
        loop {
            let fresh4 = prefix3;
            prefix3 = prefix3.offset(1);
            ch = *fresh4 as ::core::ffi::c_uint;
            if !(ch != 0 as ::core::ffi::c_uint) {
                break;
            }
            h1 = h1.wrapping_add(ch);
            h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
            h2 = h2.wrapping_add(h1);
            h2 = h2 << 7 as ::core::ffi::c_int
                | (h2 & 0xffffffff as ::core::ffi::c_uint)
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        }
        h1 = h1.wrapping_add(':' as i32 as ::core::ffi::c_uint);
        h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
    }
    if !name3.is_null() {
        loop {
            let fresh5 = name3;
            name3 = name3.offset(1);
            ch = *fresh5 as ::core::ffi::c_uint;
            if !(ch != 0 as ::core::ffi::c_uint) {
                break;
            }
            h1 = h1.wrapping_add(ch);
            h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
            h2 = h2.wrapping_add(h1);
            h2 = h2 << 7 as ::core::ffi::c_int
                | (h2 & 0xffffffff as ::core::ffi::c_uint)
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        }
    }
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 14 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as ::core::ffi::c_uint) >> 6 as ::core::ffi::c_int
            | h1 << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int,
    );
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 5 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
            | h1 << 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int,
    );
    h2 &= 0xffffffff as ::core::ffi::c_uint;
    return h2;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashCreate(mut size: ::core::ffi::c_int) -> xmlHashTablePtr {
    let mut hash: xmlHashTablePtr = ::core::ptr::null_mut::<xmlHashTable>();
    xmlInitParser();
    hash = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlHashTable>() as size_t
    ) as xmlHashTablePtr;
    if hash.is_null() {
        return ::core::ptr::null_mut::<xmlHashTable>();
    }
    (*hash).dict = ::core::ptr::null_mut::<xmlDict>();
    (*hash).size = 0 as ::core::ffi::c_uint;
    (*hash).table = ::core::ptr::null_mut::<xmlHashEntry>();
    (*hash).nbElems = 0 as ::core::ffi::c_uint;
    (*hash).randomSeed = xmlRandom();
    if size > MIN_HASH_SIZE {
        let mut newSize: ::core::ffi::c_uint =
            (MIN_HASH_SIZE * 2 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        while newSize < size as ::core::ffi::c_uint && newSize < MAX_HASH_SIZE {
            newSize = newSize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
        if xmlHashGrow(hash, newSize) != 0 as ::core::ffi::c_int {
            xmlFree.expect("non-null function pointer")(hash as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xmlHashTable>();
        }
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashCreateDict(
    mut size: ::core::ffi::c_int,
    mut dict: xmlDictPtr,
) -> xmlHashTablePtr {
    let mut hash: xmlHashTablePtr = ::core::ptr::null_mut::<xmlHashTable>();
    hash = xmlHashCreate(size);
    if !hash.is_null() {
        (*hash).dict = dict;
        xmlDictReference(dict);
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashFree(mut hash: xmlHashTablePtr, mut dealloc: xmlHashDeallocator) {
    if hash.is_null() {
        return;
    }
    if !(*hash).table.is_null() {
        let mut end: *const xmlHashEntry =
            (*hash).table.offset((*hash).size as isize) as *mut xmlHashEntry;
        let mut entry: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
        entry = (*hash).table;
        while entry < end {
            if !((*entry).hashValue == 0 as ::core::ffi::c_uint) {
                if dealloc.is_some() && !(*entry).payload.is_null() {
                    dealloc.expect("non-null function pointer")((*entry).payload, (*entry).key);
                }
                if (*hash).dict.is_null() {
                    if !(*entry).key.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            (*entry).key as *mut ::core::ffi::c_void,
                        );
                    }
                    if !(*entry).key2.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            (*entry).key2 as *mut ::core::ffi::c_void,
                        );
                    }
                    if !(*entry).key3.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            (*entry).key3 as *mut ::core::ffi::c_void,
                        );
                    }
                }
            }
            entry = entry.offset(1);
        }
        xmlFree.expect("non-null function pointer")((*hash).table as *mut ::core::ffi::c_void);
    }
    if !(*hash).dict.is_null() {
        xmlDictFree((*hash).dict);
    }
    xmlFree.expect("non-null function pointer")(hash as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn xmlFastStrEqual(
    mut s1: *const xmlChar,
    mut s2: *const xmlChar,
) -> ::core::ffi::c_int {
    if s1.is_null() {
        return (s2 == NULL as *const xmlChar) as ::core::ffi::c_int;
    } else {
        return (!s2.is_null()
            && strcmp(
                s1 as *const ::core::ffi::c_char,
                s2 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn xmlHashFindEntry(
    mut hash: *const xmlHashTable,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
    mut hashValue: ::core::ffi::c_uint,
    mut pfound: *mut ::core::ffi::c_int,
) -> *mut xmlHashEntry {
    let mut entry: *mut xmlHashEntry = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut mask: ::core::ffi::c_uint = 0;
    let mut pos: ::core::ffi::c_uint = 0;
    let mut displ: ::core::ffi::c_uint = 0;
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    mask = (*hash).size.wrapping_sub(1 as ::core::ffi::c_uint);
    pos = hashValue & mask;
    entry = (*hash).table.offset(pos as isize) as *mut xmlHashEntry;
    if (*entry).hashValue != 0 as ::core::ffi::c_uint {
        displ = 0 as ::core::ffi::c_uint;
        hashValue |= MAX_HASH_SIZE;
        loop {
            if (*entry).hashValue == hashValue {
                if !(*hash).dict.is_null() {
                    if (*entry).key == key as *mut xmlChar
                        && (*entry).key2 == key2 as *mut xmlChar
                        && (*entry).key3 == key3 as *mut xmlChar
                    {
                        found = 1 as ::core::ffi::c_int;
                        break;
                    }
                }
                if strcmp(
                    (*entry).key as *const ::core::ffi::c_char,
                    key as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    && xmlFastStrEqual((*entry).key2, key2) != 0
                    && xmlFastStrEqual((*entry).key3, key3) != 0
                {
                    found = 1 as ::core::ffi::c_int;
                    break;
                }
            }
            displ = displ.wrapping_add(1);
            pos = pos.wrapping_add(1);
            entry = entry.offset(1);
            if pos & mask == 0 as ::core::ffi::c_uint {
                entry = (*hash).table;
            }
            if !((*entry).hashValue != 0 as ::core::ffi::c_uint
                && pos.wrapping_sub((*entry).hashValue) & mask >= displ)
            {
                break;
            }
        }
    }
    *pfound = found;
    return entry;
}
unsafe extern "C" fn xmlHashGrow(
    mut hash: xmlHashTablePtr,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut oldentry: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut oldend: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut end: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut table: *mut xmlHashEntry = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut oldsize: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    if (size as size_t).wrapping_add(0 as size_t)
        > SIZE_MAX.wrapping_div(::core::mem::size_of::<xmlHashEntry>() as size_t)
    {
        return -(1 as ::core::ffi::c_int);
    }
    table = xmlMalloc.expect("non-null function pointer")(
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
    ) as *mut xmlHashEntry;
    if table.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        table as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
    );
    oldsize = (*hash).size;
    if !(oldsize == 0 as ::core::ffi::c_uint) {
        oldend = (*hash).table.offset(oldsize as isize) as *mut xmlHashEntry;
        end = table.offset(size as isize) as *mut xmlHashEntry;
        oldentry = (*hash).table;
        while (*oldentry).hashValue != 0 as ::core::ffi::c_uint {
            oldentry = oldentry.offset(1);
            if oldentry >= oldend {
                oldentry = (*hash).table;
            }
        }
        i = 0 as ::core::ffi::c_uint;
        while i < oldsize {
            if (*oldentry).hashValue != 0 as ::core::ffi::c_uint {
                let mut entry: *mut xmlHashEntry = table.offset(
                    ((*oldentry).hashValue & size.wrapping_sub(1 as ::core::ffi::c_uint)) as isize,
                ) as *mut xmlHashEntry;
                while (*entry).hashValue != 0 as ::core::ffi::c_uint {
                    entry = entry.offset(1);
                    if entry >= end as *mut xmlHashEntry {
                        entry = table;
                    }
                }
                *entry = *oldentry;
            }
            oldentry = oldentry.offset(1);
            if oldentry >= oldend {
                oldentry = (*hash).table;
            }
            i = i.wrapping_add(1);
        }
        xmlFree.expect("non-null function pointer")((*hash).table as *mut ::core::ffi::c_void);
    }
    (*hash).table = table;
    (*hash).size = size;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlHashUpdateInternal(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
    mut payload: *mut ::core::ffi::c_void,
    mut dealloc: xmlHashDeallocator,
    mut update: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut copy: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut copy2: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut copy3: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut entry: *mut xmlHashEntry = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut lengths: [size_t; 3] = [0; 3];
    let mut hashValue: ::core::ffi::c_uint = 0;
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if hash.is_null() || key.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    hashValue = xmlHashValue(
        (*hash).randomSeed,
        key,
        key2,
        key3,
        &raw mut lengths as *mut size_t,
    );
    if (*hash).size > 0 as ::core::ffi::c_uint {
        entry = xmlHashFindEntry(
            hash as *const xmlHashTable,
            key,
            key2,
            key3,
            hashValue,
            &raw mut found,
        );
    }
    if found != 0 {
        if update != 0 {
            if dealloc.is_some() {
                dealloc.expect("non-null function pointer")((*entry).payload, (*entry).key);
            }
            (*entry).payload = payload;
            return 0 as ::core::ffi::c_int;
        } else {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*hash).nbElems.wrapping_add(1 as ::core::ffi::c_uint)
        > (*hash)
            .size
            .wrapping_div(MAX_FILL_DENOM as ::core::ffi::c_uint)
            .wrapping_mul(MAX_FILL_NUM as ::core::ffi::c_uint)
    {
        let mut newSize: ::core::ffi::c_uint = 0;
        let mut mask: ::core::ffi::c_uint = 0;
        let mut displ: ::core::ffi::c_uint = 0;
        let mut pos: ::core::ffi::c_uint = 0;
        if (*hash).size == 0 as ::core::ffi::c_uint {
            newSize = MIN_HASH_SIZE as ::core::ffi::c_uint;
        } else {
            if (*hash).size >= MAX_HASH_SIZE {
                return -(1 as ::core::ffi::c_int);
            }
            newSize = (*hash).size.wrapping_mul(2 as ::core::ffi::c_uint);
        }
        if xmlHashGrow(hash, newSize) != 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        mask = (*hash).size.wrapping_sub(1 as ::core::ffi::c_uint);
        displ = 0 as ::core::ffi::c_uint;
        pos = hashValue & mask;
        entry = (*hash).table.offset(pos as isize) as *mut xmlHashEntry;
        if (*entry).hashValue != 0 as ::core::ffi::c_uint {
            loop {
                displ = displ.wrapping_add(1);
                pos = pos.wrapping_add(1);
                entry = entry.offset(1);
                if pos & mask == 0 as ::core::ffi::c_uint {
                    entry = (*hash).table;
                }
                if !((*entry).hashValue != 0 as ::core::ffi::c_uint
                    && pos.wrapping_sub((*entry).hashValue) & mask >= displ)
                {
                    break;
                }
            }
        }
    }
    if !(*hash).dict.is_null() {
        if xmlDictOwns((*hash).dict, key) != 0 {
            copy = key as *mut xmlChar;
        } else {
            copy = xmlDictLookup((*hash).dict, key, -(1 as ::core::ffi::c_int)) as *mut xmlChar;
            if copy.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
        if key2.is_null() || xmlDictOwns((*hash).dict, key2) != 0 {
            copy2 = key2 as *mut xmlChar;
        } else {
            copy2 = xmlDictLookup((*hash).dict, key2, -(1 as ::core::ffi::c_int)) as *mut xmlChar;
            if copy2.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
        if key3.is_null() || xmlDictOwns((*hash).dict, key3) != 0 {
            copy3 = key3 as *mut xmlChar;
        } else {
            copy3 = xmlDictLookup((*hash).dict, key3, -(1 as ::core::ffi::c_int)) as *mut xmlChar;
            if copy3.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
    } else {
        copy = xmlMalloc.expect("non-null function pointer")(
            lengths[0 as ::core::ffi::c_int as usize].wrapping_add(1 as size_t),
        ) as *mut xmlChar;
        if copy.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        memcpy(
            copy as *mut ::core::ffi::c_void,
            key as *const ::core::ffi::c_void,
            lengths[0 as ::core::ffi::c_int as usize].wrapping_add(1 as size_t),
        );
        if !key2.is_null() {
            copy2 = xmlMalloc.expect("non-null function pointer")(
                lengths[1 as ::core::ffi::c_int as usize].wrapping_add(1 as size_t),
            ) as *mut xmlChar;
            if copy2.is_null() {
                xmlFree.expect("non-null function pointer")(copy as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            memcpy(
                copy2 as *mut ::core::ffi::c_void,
                key2 as *const ::core::ffi::c_void,
                lengths[1 as ::core::ffi::c_int as usize].wrapping_add(1 as size_t),
            );
        } else {
            copy2 = ::core::ptr::null_mut::<xmlChar>();
        }
        if !key3.is_null() {
            copy3 = xmlMalloc.expect("non-null function pointer")(
                lengths[2 as ::core::ffi::c_int as usize].wrapping_add(1 as size_t),
            ) as *mut xmlChar;
            if copy3.is_null() {
                xmlFree.expect("non-null function pointer")(copy as *mut ::core::ffi::c_void);
                xmlFree.expect("non-null function pointer")(copy2 as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            memcpy(
                copy3 as *mut ::core::ffi::c_void,
                key3 as *const ::core::ffi::c_void,
                lengths[2 as ::core::ffi::c_int as usize].wrapping_add(1 as size_t),
            );
        } else {
            copy3 = ::core::ptr::null_mut::<xmlChar>();
        }
    }
    if (*entry).hashValue != 0 as ::core::ffi::c_uint {
        let mut end: *const xmlHashEntry =
            (*hash).table.offset((*hash).size as isize) as *mut xmlHashEntry;
        let mut cur: *const xmlHashEntry = entry;
        loop {
            cur = cur.offset(1);
            if cur >= end {
                cur = (*hash).table;
            }
            if !((*cur).hashValue != 0 as ::core::ffi::c_uint) {
                break;
            }
        }
        if cur < entry as *const xmlHashEntry {
            memmove(
                (*hash).table.offset(1 as ::core::ffi::c_int as isize) as *mut xmlHashEntry
                    as *mut ::core::ffi::c_void,
                (*hash).table as *const ::core::ffi::c_void,
                (cur as *mut ::core::ffi::c_char)
                    .offset_from((*hash).table as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as size_t,
            );
            cur = end.offset(-(1 as ::core::ffi::c_int as isize));
            *(*hash).table.offset(0 as ::core::ffi::c_int as isize) = *cur;
        }
        memmove(
            entry.offset(1 as ::core::ffi::c_int as isize) as *mut xmlHashEntry
                as *mut ::core::ffi::c_void,
            entry as *const ::core::ffi::c_void,
            (cur as *mut ::core::ffi::c_char).offset_from(entry as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as size_t,
        );
    }
    (*entry).key = copy;
    (*entry).key2 = copy2;
    (*entry).key3 = copy3;
    (*entry).payload = payload;
    (*entry).hashValue = hashValue | MAX_HASH_SIZE;
    (*hash).nbElems = (*hash).nbElems.wrapping_add(1);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashDefaultDeallocator(
    mut entry: *mut ::core::ffi::c_void,
    mut key: *const xmlChar,
) {
    xmlFree.expect("non-null function pointer")(entry);
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashAddEntry(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut payload: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return xmlHashUpdateInternal(
        hash,
        key,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        payload,
        None,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashAddEntry2(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut payload: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return xmlHashUpdateInternal(
        hash,
        key,
        key2,
        ::core::ptr::null::<xmlChar>(),
        payload,
        None,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashAddEntry3(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
    mut payload: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return xmlHashUpdateInternal(
        hash,
        key,
        key2,
        key3,
        payload,
        None,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashUpdateEntry(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut payload: *mut ::core::ffi::c_void,
    mut dealloc: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    return xmlHashUpdateInternal(
        hash,
        key,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        payload,
        dealloc,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashUpdateEntry2(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut payload: *mut ::core::ffi::c_void,
    mut dealloc: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    return xmlHashUpdateInternal(
        hash,
        key,
        key2,
        ::core::ptr::null::<xmlChar>(),
        payload,
        dealloc,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashUpdateEntry3(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
    mut payload: *mut ::core::ffi::c_void,
    mut dealloc: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    return xmlHashUpdateInternal(
        hash,
        key,
        key2,
        key3,
        payload,
        dealloc,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashLookup(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    return xmlHashLookup3(
        hash,
        key,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashLookup2(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    return xmlHashLookup3(hash, key, key2, ::core::ptr::null::<xmlChar>());
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashQLookup(
    mut hash: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    return xmlHashQLookup3(
        hash,
        prefix,
        name,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashQLookup2(
    mut hash: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut prefix2: *const xmlChar,
    mut name2: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    return xmlHashQLookup3(
        hash,
        prefix,
        name,
        prefix2,
        name2,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashLookup3(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    let mut entry: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut hashValue: ::core::ffi::c_uint = 0;
    let mut found: ::core::ffi::c_int = 0;
    if hash.is_null() || (*hash).size == 0 as ::core::ffi::c_uint || key.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    hashValue = xmlHashValue(
        (*hash).randomSeed,
        key,
        key2,
        key3,
        ::core::ptr::null_mut::<size_t>(),
    );
    entry = xmlHashFindEntry(
        hash as *const xmlHashTable,
        key,
        key2,
        key3,
        hashValue,
        &raw mut found,
    );
    if found != 0 {
        return (*entry).payload;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashQLookup3(
    mut hash: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut prefix2: *const xmlChar,
    mut name2: *const xmlChar,
    mut prefix3: *const xmlChar,
    mut name3: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    let mut entry: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut hashValue: ::core::ffi::c_uint = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut pos: ::core::ffi::c_uint = 0;
    let mut displ: ::core::ffi::c_uint = 0;
    if hash.is_null() || (*hash).size == 0 as ::core::ffi::c_uint || name.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    hashValue = xmlHashQNameValue(
        (*hash).randomSeed,
        prefix,
        name,
        prefix2,
        name2,
        prefix3,
        name3,
    );
    mask = (*hash).size.wrapping_sub(1 as ::core::ffi::c_uint);
    pos = hashValue & mask;
    entry = (*hash).table.offset(pos as isize) as *mut xmlHashEntry;
    if (*entry).hashValue != 0 as ::core::ffi::c_uint {
        displ = 0 as ::core::ffi::c_uint;
        hashValue |= MAX_HASH_SIZE;
        loop {
            if hashValue == (*entry).hashValue
                && xmlStrQEqual(prefix, name, (*entry).key) != 0
                && xmlStrQEqual(prefix2, name2, (*entry).key2) != 0
                && xmlStrQEqual(prefix3, name3, (*entry).key3) != 0
            {
                return (*entry).payload;
            }
            displ = displ.wrapping_add(1);
            pos = pos.wrapping_add(1);
            entry = entry.offset(1);
            if pos & mask == 0 as ::core::ffi::c_uint {
                entry = (*hash).table;
            }
            if !((*entry).hashValue != 0 as ::core::ffi::c_uint
                && pos.wrapping_sub((*entry).hashValue) & mask >= displ)
            {
                break;
            }
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn stubHashScannerFull(
    mut payload: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
) {
    let mut sdata: *mut stubData = data as *mut stubData;
    (*sdata).scan.expect("non-null function pointer")(payload, (*sdata).data, key);
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScan(
    mut hash: xmlHashTablePtr,
    mut scan: xmlHashScanner,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut sdata: stubData = stubData {
        scan: None,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    sdata.data = data;
    sdata.scan = scan;
    xmlHashScanFull(
        hash,
        Some(
            stubHashScannerFull
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        &raw mut sdata as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScanFull(
    mut hash: xmlHashTablePtr,
    mut scan: xmlHashScannerFull,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut entry: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut end: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut old: xmlHashEntry = xmlHashEntry {
        hashValue: 0,
        key: ::core::ptr::null_mut::<xmlChar>(),
        key2: ::core::ptr::null_mut::<xmlChar>(),
        key3: ::core::ptr::null_mut::<xmlChar>(),
        payload: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut i: ::core::ffi::c_uint = 0;
    if hash.is_null() || (*hash).size == 0 as ::core::ffi::c_uint || scan.is_none() {
        return;
    }
    entry = (*hash).table;
    end = (*hash).table.offset((*hash).size as isize) as *mut xmlHashEntry;
    while (*entry).hashValue != 0 as ::core::ffi::c_uint {
        entry = entry.offset(1);
        if entry >= end {
            entry = (*hash).table;
        }
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*hash).size {
        if (*entry).hashValue != 0 as ::core::ffi::c_uint && !(*entry).payload.is_null() {
            loop {
                old = *entry;
                scan.expect("non-null function pointer")(
                    (*entry).payload,
                    data,
                    (*entry).key,
                    (*entry).key2,
                    (*entry).key3,
                );
                if !((*entry).hashValue != 0 as ::core::ffi::c_uint
                    && !(*entry).payload.is_null()
                    && ((*entry).key != old.key
                        || (*entry).key2 != old.key2
                        || (*entry).key3 != old.key3))
                {
                    break;
                }
            }
        }
        entry = entry.offset(1);
        if entry >= end {
            entry = (*hash).table;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScan3(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
    mut scan: xmlHashScanner,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut sdata: stubData = stubData {
        scan: None,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    sdata.data = data;
    sdata.scan = scan;
    xmlHashScanFull3(
        hash,
        key,
        key2,
        key3,
        Some(
            stubHashScannerFull
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        &raw mut sdata as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScanFull3(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
    mut scan: xmlHashScannerFull,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut entry: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut end: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut old: xmlHashEntry = xmlHashEntry {
        hashValue: 0,
        key: ::core::ptr::null_mut::<xmlChar>(),
        key2: ::core::ptr::null_mut::<xmlChar>(),
        key3: ::core::ptr::null_mut::<xmlChar>(),
        payload: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut i: ::core::ffi::c_uint = 0;
    if hash.is_null() || (*hash).size == 0 as ::core::ffi::c_uint || scan.is_none() {
        return;
    }
    entry = (*hash).table;
    end = (*hash).table.offset((*hash).size as isize) as *mut xmlHashEntry;
    while (*entry).hashValue != 0 as ::core::ffi::c_uint {
        entry = entry.offset(1);
        if entry >= end {
            entry = (*hash).table;
        }
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*hash).size {
        if (*entry).hashValue != 0 as ::core::ffi::c_uint && !(*entry).payload.is_null() {
            while !(!key.is_null()
                && strcmp(
                    key as *const ::core::ffi::c_char,
                    (*entry).key as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                || !key2.is_null() && xmlFastStrEqual(key2, (*entry).key2) == 0
                || !key3.is_null() && xmlFastStrEqual(key3, (*entry).key3) == 0)
            {
                old = *entry;
                scan.expect("non-null function pointer")(
                    (*entry).payload,
                    data,
                    (*entry).key,
                    (*entry).key2,
                    (*entry).key3,
                );
                if !((*entry).hashValue != 0 as ::core::ffi::c_uint
                    && !(*entry).payload.is_null()
                    && ((*entry).key != old.key
                        || (*entry).key2 != old.key2
                        || (*entry).key3 != old.key3))
                {
                    break;
                }
            }
        }
        entry = entry.offset(1);
        if entry >= end {
            entry = (*hash).table;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashCopy(
    mut hash: xmlHashTablePtr,
    mut copy: xmlHashCopier,
) -> xmlHashTablePtr {
    let mut entry: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut end: *const xmlHashEntry = ::core::ptr::null::<xmlHashEntry>();
    let mut ret: xmlHashTablePtr = ::core::ptr::null_mut::<xmlHashTable>();
    if hash.is_null() || copy.is_none() {
        return ::core::ptr::null_mut::<xmlHashTable>();
    }
    ret = xmlHashCreate((*hash).size as ::core::ffi::c_int);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlHashTable>();
    }
    if (*hash).size == 0 as ::core::ffi::c_uint {
        return ret;
    }
    end = (*hash).table.offset((*hash).size as isize) as *mut xmlHashEntry;
    entry = (*hash).table;
    while entry < end {
        if (*entry).hashValue != 0 as ::core::ffi::c_uint {
            xmlHashAddEntry3(
                ret,
                (*entry).key,
                (*entry).key2,
                (*entry).key3,
                copy.expect("non-null function pointer")((*entry).payload, (*entry).key),
            );
        }
        entry = entry.offset(1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashSize(mut hash: xmlHashTablePtr) -> ::core::ffi::c_int {
    if hash.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return (*hash).nbElems as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashRemoveEntry(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut dealloc: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    return xmlHashRemoveEntry3(
        hash,
        key,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        dealloc,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashRemoveEntry2(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut dealloc: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    return xmlHashRemoveEntry3(hash, key, key2, ::core::ptr::null::<xmlChar>(), dealloc);
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashRemoveEntry3(
    mut hash: xmlHashTablePtr,
    mut key: *const xmlChar,
    mut key2: *const xmlChar,
    mut key3: *const xmlChar,
    mut dealloc: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    let mut entry: *mut xmlHashEntry = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut cur: *mut xmlHashEntry = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut next: *mut xmlHashEntry = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut hashValue: ::core::ffi::c_uint = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut pos: ::core::ffi::c_uint = 0;
    let mut nextpos: ::core::ffi::c_uint = 0;
    let mut found: ::core::ffi::c_int = 0;
    if hash.is_null() || (*hash).size == 0 as ::core::ffi::c_uint || key.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    hashValue = xmlHashValue(
        (*hash).randomSeed,
        key,
        key2,
        key3,
        ::core::ptr::null_mut::<size_t>(),
    );
    entry = xmlHashFindEntry(
        hash as *const xmlHashTable,
        key,
        key2,
        key3,
        hashValue,
        &raw mut found,
    );
    if found == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if dealloc.is_some() && !(*entry).payload.is_null() {
        dealloc.expect("non-null function pointer")((*entry).payload, (*entry).key);
    }
    if (*hash).dict.is_null() {
        if !(*entry).key.is_null() {
            xmlFree.expect("non-null function pointer")((*entry).key as *mut ::core::ffi::c_void);
        }
        if !(*entry).key2.is_null() {
            xmlFree.expect("non-null function pointer")((*entry).key2 as *mut ::core::ffi::c_void);
        }
        if !(*entry).key3.is_null() {
            xmlFree.expect("non-null function pointer")((*entry).key3 as *mut ::core::ffi::c_void);
        }
    }
    mask = (*hash).size.wrapping_sub(1 as ::core::ffi::c_uint);
    pos = entry.offset_from((*hash).table) as ::core::ffi::c_long as ::core::ffi::c_uint;
    cur = entry;
    loop {
        nextpos = pos.wrapping_add(1 as ::core::ffi::c_uint);
        next = cur.offset(1 as ::core::ffi::c_int as isize);
        if nextpos & mask == 0 as ::core::ffi::c_uint {
            next = (*hash).table;
        }
        if (*next).hashValue == 0 as ::core::ffi::c_uint
            || (*next).hashValue.wrapping_sub(nextpos) & mask == 0 as ::core::ffi::c_uint
        {
            break;
        }
        cur = next;
        pos = nextpos;
    }
    next = entry.offset(1 as ::core::ffi::c_int as isize);
    if cur < entry {
        let mut end: *mut xmlHashEntry =
            (*hash).table.offset((*hash).size as isize) as *mut xmlHashEntry;
        memmove(
            entry as *mut ::core::ffi::c_void,
            next as *const ::core::ffi::c_void,
            (end as *mut ::core::ffi::c_char).offset_from(next as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as size_t,
        );
        entry = (*hash).table;
        *end.offset(-(1 as ::core::ffi::c_int) as isize) = *entry;
        next = entry.offset(1 as ::core::ffi::c_int as isize);
    }
    memmove(
        entry as *mut ::core::ffi::c_void,
        next as *const ::core::ffi::c_void,
        (cur as *mut ::core::ffi::c_char).offset_from(entry as *mut ::core::ffi::c_char)
            as ::core::ffi::c_long as size_t,
    );
    (*cur).hashValue = 0 as ::core::ffi::c_uint;
    (*hash).nbElems = (*hash).nbElems.wrapping_sub(1);
    return 0 as ::core::ffi::c_int;
}
