use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliSharedDictionaryStruct {
    pub num_prefix: uint32_t,
    pub prefix_size: [size_t; 15],
    pub prefix: [*const uint8_t; 15],
    pub context_based: c_int,
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
    pub memory_manager_opaque: *mut c_void,
}

pub type BrotliSharedDictionary = BrotliSharedDictionaryStruct;

pub type BrotliSharedDictionaryInternal = BrotliSharedDictionaryStruct;

#[no_mangle]
pub unsafe extern "C" fn BrotliSharedDictionaryDestroyInstance(
    mut dict: *mut BrotliSharedDictionaryInternal,
) {
    if dict.is_null() {
        return;
    } else {
        let mut free_func: brotli_free_func = (*dict).free_func;
        let mut opaque: *mut c_void = (*dict).memory_manager_opaque;
        free_func.expect("non-null function pointer")(
            opaque,
            (*dict).words_instances as *mut c_void,
        );
        free_func.expect("non-null function pointer")(
            opaque,
            (*dict).transforms_instances as *mut c_void,
        );
        free_func.expect("non-null function pointer")(
            opaque,
            (*dict).prefix_suffix_maps as *mut c_void,
        );
        free_func.expect("non-null function pointer")(opaque, dict as *mut c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliSharedDictionaryAttach(
    mut dict: *mut BrotliSharedDictionaryInternal,
    mut type_0: BrotliSharedDictionaryType,
    mut data_size: size_t,
    mut data: *const uint8_t,
) -> c_int {
    if dict.is_null() {
        return BROTLI_FALSE;
    }
    if type_0 as c_uint
        == BROTLI_SHARED_DICTIONARY_RAW as c_int as c_uint
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
    mut opaque: *mut c_void,
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
        dict as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<BrotliSharedDictionaryInternal>() as size_t,
    );
    (*dict).context_based = BROTLI_FALSE;
    (*dict).num_dictionaries = 1 as uint8_t;
    (*dict).num_word_lists = 0 as uint8_t;
    (*dict).num_transform_lists = 0 as uint8_t;
    (*dict).words[0 as c_int as usize] = BrotliGetDictionary();
    (*dict).transforms[0 as c_int as usize] = BrotliGetTransforms();
    (*dict).alloc_func = if alloc_func.is_some() {
        alloc_func
    } else {
        Some(
            BrotliDefaultAllocFunc
                as unsafe extern "C" fn(
                    *mut c_void,
                    size_t,
                ) -> *mut c_void,
        )
    };
    (*dict).free_func = if free_func.is_some() {
        free_func
    } else {
        Some(
            BrotliDefaultFreeFunc
                as unsafe extern "C" fn(*mut c_void, *mut c_void) -> (),
        )
    };
    (*dict).memory_manager_opaque = opaque;
    return dict as *mut BrotliSharedDictionary;
}
