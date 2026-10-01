extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn BrotliDefaultAllocFunc(
        opaque: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn BrotliDefaultFreeFunc(opaque: *mut ::core::ffi::c_void, address: *mut ::core::ffi::c_void);
    fn BrotliGetDictionary() -> *const BrotliDictionary;
    fn BrotliGetTransforms() -> *const BrotliTransforms;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliSharedDictionaryStruct {
    pub num_prefix: uint32_t,
    pub prefix_size: [size_t; 15],
    pub prefix: [*const uint8_t; 15],
    pub context_based: ::core::ffi::c_int,
    pub context_map: [uint8_t; 64],
    pub num_dictionaries: uint8_t,
    pub words: [*const BrotliDictionary; 64],
    pub transforms: [*const BrotliTransforms; 64],
    pub num_word_lists: uint8_t,
    pub words_instances: *mut BrotliDictionary,
    pub num_transform_lists: uint8_t,
    pub transforms_instances: *mut BrotliTransforms,
    pub prefix_suffix_maps: *mut uint16_t,
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub memory_manager_opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliTransforms {
    pub prefix_suffix_size: uint16_t,
    pub prefix_suffix: *const uint8_t,
    pub prefix_suffix_map: *const uint16_t,
    pub num_transforms: uint32_t,
    pub transforms: *const uint8_t,
    pub params: *const uint8_t,
    pub cutOffTransforms: [int16_t; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [uint8_t; 32],
    pub offsets_by_length: [uint32_t; 32],
    pub data_size: size_t,
    pub data: *const uint8_t,
}
pub type BrotliSharedDictionary = BrotliSharedDictionaryStruct;
pub type BrotliSharedDictionaryType = ::core::ffi::c_uint;
pub const BROTLI_SHARED_DICTIONARY_SERIALIZED: BrotliSharedDictionaryType = 1;
pub const BROTLI_SHARED_DICTIONARY_RAW: BrotliSharedDictionaryType = 0;
pub type BrotliSharedDictionaryInternal = BrotliSharedDictionaryStruct;
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SHARED_BROTLI_MAX_COMPOUND_DICTS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn BrotliSharedDictionaryDestroyInstance(
    mut dict: *mut BrotliSharedDictionaryInternal,
) {
    if dict.is_null() {
        return;
    } else {
        let mut free_func: brotli_free_func = (*dict).free_func;
        let mut opaque: *mut ::core::ffi::c_void = (*dict).memory_manager_opaque;
        free_func.expect("non-null function pointer")(
            opaque,
            (*dict).words_instances as *mut ::core::ffi::c_void,
        );
        free_func.expect("non-null function pointer")(
            opaque,
            (*dict).transforms_instances as *mut ::core::ffi::c_void,
        );
        free_func.expect("non-null function pointer")(
            opaque,
            (*dict).prefix_suffix_maps as *mut ::core::ffi::c_void,
        );
        free_func.expect("non-null function pointer")(opaque, dict as *mut ::core::ffi::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliSharedDictionaryAttach(
    mut dict: *mut BrotliSharedDictionaryInternal,
    mut type_0: BrotliSharedDictionaryType,
    mut data_size: size_t,
    mut data: *const uint8_t,
) -> ::core::ffi::c_int {
    if dict.is_null() {
        return BROTLI_FALSE;
    }
    if type_0 as ::core::ffi::c_uint
        == BROTLI_SHARED_DICTIONARY_RAW as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*dict).num_prefix >= SHARED_BROTLI_MAX_COMPOUND_DICTS as uint32_t {
            return BROTLI_FALSE;
        }
        (*dict).prefix_size[(*dict).num_prefix as usize] = data_size;
        (*dict).prefix[(*dict).num_prefix as usize] = data as *const uint8_t;
        (*dict).num_prefix = (*dict).num_prefix.wrapping_add(1);
        return BROTLI_TRUE;
    }
    return BROTLI_FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliSharedDictionaryCreateInstance(
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut ::core::ffi::c_void,
) -> *mut BrotliSharedDictionary {
    let mut dict: *mut BrotliSharedDictionaryInternal =
        ::core::ptr::null_mut::<BrotliSharedDictionaryInternal>();
    if alloc_func.is_none() && free_func.is_none() {
        dict = malloc(::core::mem::size_of::<BrotliSharedDictionaryInternal>() as size_t)
            as *mut BrotliSharedDictionaryInternal;
    } else if alloc_func.is_some() && free_func.is_some() {
        dict = alloc_func.expect("non-null function pointer")(
            opaque,
            ::core::mem::size_of::<BrotliSharedDictionaryInternal>() as size_t,
        ) as *mut BrotliSharedDictionaryInternal;
    }
    if dict.is_null() {
        return ::core::ptr::null_mut::<BrotliSharedDictionary>();
    }
    memset(
        dict as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<BrotliSharedDictionaryInternal>() as size_t,
    );
    (*dict).context_based = BROTLI_FALSE;
    (*dict).num_dictionaries = 1 as uint8_t;
    (*dict).num_word_lists = 0 as uint8_t;
    (*dict).num_transform_lists = 0 as uint8_t;
    (*dict).words[0 as ::core::ffi::c_int as usize] = BrotliGetDictionary();
    (*dict).transforms[0 as ::core::ffi::c_int as usize] = BrotliGetTransforms();
    (*dict).alloc_func = if alloc_func.is_some() {
        alloc_func
    } else {
        Some(
            BrotliDefaultAllocFunc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        )
    };
    (*dict).free_func = if free_func.is_some() {
        free_func
    } else {
        Some(
            BrotliDefaultFreeFunc
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        )
    };
    (*dict).memory_manager_opaque = opaque;
    return dict as *mut BrotliSharedDictionary;
}
