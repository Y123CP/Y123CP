use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    static kStaticDictionaryBuckets: [uint16_t; 32768];
    static kStaticDictionaryWords: [DictWord; 31705];
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderDictionary {
    pub words: *const BrotliDictionary,
    pub num_transforms: uint32_t,
    pub cutoffTransformsCount: uint32_t,
    pub cutoffTransforms: uint64_t,
    pub hash_table_words: *const uint16_t,
    pub hash_table_lengths: *const uint8_t,
    pub buckets: *const uint16_t,
    pub dict_words: *const DictWord,
    pub trie: BrotliTrie,
    pub has_words_heavy: c_int,
    pub parent: *const ContextualEncoderDictionary,
    pub hash_table_data_words_: *mut uint16_t,
    pub hash_table_data_lengths_: *mut uint8_t,
    pub buckets_alloc_size_: size_t,
    pub buckets_data_: *mut uint16_t,
    pub dict_words_alloc_size_: size_t,
    pub dict_words_data_: *mut DictWord,
    pub words_instance_: *mut BrotliDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContextualEncoderDictionary {
    pub context_based: c_int,
    pub num_dictionaries: uint8_t,
    pub context_map: [uint8_t; 64],
    pub dict: [*const BrotliEncoderDictionary; 64],
    pub num_instances_: size_t,
    pub instance_: BrotliEncoderDictionary,
    pub instances_: *mut BrotliEncoderDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharedEncoderDictionary {
    pub magic: uint32_t,
    pub compound: CompoundDictionary,
    pub contextual: ContextualEncoderDictionary,
    pub max_quality: c_int,
}

unsafe extern "C" fn BrotliTrieInit(mut trie: *mut BrotliTrie) {
    (*trie).pool_capacity = 0 as size_t;
    (*trie).pool_size = 0 as size_t;
    (*trie).pool = ::core::ptr::null_mut::<BrotliTrieNode>();
    (*trie).root.single = 0 as uint8_t;
    (*trie).root.len_ = 0 as uint8_t;
    (*trie).root.idx_ = 0 as uint32_t;
    (*trie).root.sub = 0 as uint32_t;
}
unsafe extern "C" fn BrotliTrieFree(mut m: *mut MemoryManager, mut trie: *mut BrotliTrie) {
    BrotliFree(m, (*trie).pool as *mut c_void);
}
unsafe extern "C" fn InitEncoderDictionary(mut dict: *mut BrotliEncoderDictionary) {
    (*dict).words = BrotliGetDictionary();
    (*dict).num_transforms = (*BrotliGetTransforms()).num_transforms;
    (*dict).hash_table_words = &raw const kStaticDictionaryHashWords as *const uint16_t;
    (*dict).hash_table_lengths = &raw const kStaticDictionaryHashLengths as *const uint8_t;
    (*dict).buckets = &raw const kStaticDictionaryBuckets as *const uint16_t;
    (*dict).dict_words = &raw const kStaticDictionaryWords as *const DictWord;
    (*dict).cutoffTransformsCount = kCutoffTransformsCount;
    (*dict).cutoffTransforms = kCutoffTransforms;
    (*dict).parent = ::core::ptr::null::<ContextualEncoderDictionary>();
    (*dict).hash_table_data_words_ = ::core::ptr::null_mut::<uint16_t>();
    (*dict).hash_table_data_lengths_ = ::core::ptr::null_mut::<uint8_t>();
    (*dict).buckets_alloc_size_ = 0 as size_t;
    (*dict).buckets_data_ = ::core::ptr::null_mut::<uint16_t>();
    (*dict).dict_words_alloc_size_ = 0 as size_t;
    (*dict).dict_words_data_ = ::core::ptr::null_mut::<DictWord>();
    (*dict).words_instance_ = ::core::ptr::null_mut::<BrotliDictionary>();
    (*dict).has_words_heavy = BROTLI_FALSE;
    BrotliTrieInit(&raw mut (*dict).trie);
}
unsafe extern "C" fn BrotliDestroyEncoderDictionary(
    mut m: *mut MemoryManager,
    mut dict: *mut BrotliEncoderDictionary,
) {
    BrotliFree(
        m,
        (*dict).hash_table_data_words_ as *mut c_void,
    );
    BrotliFree(
        m,
        (*dict).hash_table_data_lengths_ as *mut c_void,
    );
    BrotliFree(m, (*dict).buckets_data_ as *mut c_void);
    BrotliFree(m, (*dict).dict_words_data_ as *mut c_void);
    BrotliFree(m, (*dict).words_instance_ as *mut c_void);
    BrotliTrieFree(m, &raw mut (*dict).trie);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliInitSharedEncoderDictionary(mut dict: *mut SharedEncoderDictionary) {
    (*dict).magic = kSharedDictionaryMagic;
    (*dict).compound.num_chunks = 0 as size_t;
    (*dict).compound.total_size = 0 as size_t;
    (*dict).compound.chunk_offsets[0 as c_int as usize] = 0 as size_t;
    (*dict).compound.num_prepared_instances_ = 0 as size_t;
    (*dict).contextual.context_based = 0 as c_int;
    (*dict).contextual.num_dictionaries = 1 as uint8_t;
    (*dict).contextual.instances_ = ::core::ptr::null_mut::<BrotliEncoderDictionary>();
    (*dict).contextual.num_instances_ = 1 as size_t;
    (*dict).contextual.dict[0 as c_int as usize] =
        &raw mut (*dict).contextual.instance_;
    InitEncoderDictionary(&raw mut (*dict).contextual.instance_);
    (*dict).contextual.instance_.parent = &raw mut (*dict).contextual;
    (*dict).max_quality = BROTLI_MAX_QUALITY;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCleanupSharedEncoderDictionary(
    mut m: *mut MemoryManager,
    mut dict: *mut SharedEncoderDictionary,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*dict).compound.num_prepared_instances_ {
        DestroyPreparedDictionary(m, (*dict).compound.prepared_instances_[i as usize]);
        i = i.wrapping_add(1);
    }
    if (*dict).contextual.num_instances_ == 1 as size_t {
        BrotliDestroyEncoderDictionary(m, &raw mut (*dict).contextual.instance_);
    } else if (*dict).contextual.num_instances_ > 1 as size_t {
        i = 0 as size_t;
        while i < (*dict).contextual.num_instances_ {
            BrotliDestroyEncoderDictionary(
                m,
                (*dict).contextual.instances_.offset(i as isize) as *mut BrotliEncoderDictionary,
            );
            i = i.wrapping_add(1);
        }
        BrotliFree(m, (*dict).contextual.instances_ as *mut c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCreateManagedDictionary(
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut c_void,
) -> *mut ManagedDictionary {
    let mut result: *mut ManagedDictionary = BrotliBootstrapAlloc(
        ::core::mem::size_of::<ManagedDictionary>() as size_t,
        alloc_func,
        free_func,
        opaque,
    ) as *mut ManagedDictionary;
    if result.is_null() {
        return ::core::ptr::null_mut::<ManagedDictionary>();
    }
    (*result).magic = kManagedDictionaryMagic;
    BrotliInitMemoryManager(
        &raw mut (*result).memory_manager_,
        alloc_func,
        free_func,
        opaque,
    );
    (*result).dictionary = ::core::ptr::null_mut::<uint32_t>();
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDestroyManagedDictionary(mut dictionary: *mut ManagedDictionary) {
    if dictionary.is_null() {
        return;
    }
    BrotliBootstrapFree(
        dictionary as *mut c_void,
        &raw mut (*dictionary).memory_manager_,
    );
}

static mut kSharedDictionaryMagic: uint32_t = 0xdebcede1 as uint32_t;
static mut kManagedDictionaryMagic: uint32_t = 0xdebcede2 as uint32_t;
static mut kCutoffTransformsCount: uint32_t = 10 as uint32_t;
static mut kCutoffTransforms: uint64_t = (0x71b520a as c_int as uint64_t)
    << 32 as c_int
    | 0xda2d3200 as uint64_t;
