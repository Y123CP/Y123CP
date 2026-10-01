extern "C" {
    fn BrotliGetDictionary() -> *const BrotliDictionary;
    fn BrotliInitMemoryManager(
        m: *mut MemoryManager,
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut ::core::ffi::c_void,
    );
    fn BrotliFree(m: *mut MemoryManager, p: *mut ::core::ffi::c_void);
    fn BrotliBootstrapAlloc(
        size: size_t,
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn BrotliBootstrapFree(address: *mut ::core::ffi::c_void, m: *mut MemoryManager);
    static kStaticDictionaryBuckets: [uint16_t; 32768];
    static kStaticDictionaryWords: [DictWord; 31705];
    fn DestroyPreparedDictionary(m: *mut MemoryManager, dictionary: *mut PreparedDictionary);
    fn BrotliGetTransforms() -> *const BrotliTransforms;
    static kStaticDictionaryHashWords: [uint16_t; 32768];
    static kStaticDictionaryHashLengths: [uint8_t; 32768];
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [uint8_t; 32],
    pub offsets_by_length: [uint32_t; 32],
    pub data_size: size_t,
    pub data: *const uint8_t,
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
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreparedDictionary {
    pub magic: uint32_t,
    pub num_items: uint32_t,
    pub source_size: uint32_t,
    pub hash_bits: uint32_t,
    pub bucket_bits: uint32_t,
    pub slot_bits: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompoundDictionary {
    pub num_chunks: size_t,
    pub total_size: size_t,
    pub chunks: [*const PreparedDictionary; 16],
    pub chunk_source: [*const uint8_t; 16],
    pub chunk_offsets: [size_t; 16],
    pub num_prepared_instances_: size_t,
    pub prepared_instances_: [*mut PreparedDictionary; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DictWord {
    pub len: uint8_t,
    pub transform: uint8_t,
    pub idx: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliTrieNode {
    pub single: uint8_t,
    pub c: uint8_t,
    pub len_: uint8_t,
    pub idx_: uint32_t,
    pub sub: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliTrie {
    pub pool: *mut BrotliTrieNode,
    pub pool_capacity: size_t,
    pub pool_size: size_t,
    pub root: BrotliTrieNode,
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
    pub has_words_heavy: ::core::ffi::c_int,
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
    pub context_based: ::core::ffi::c_int,
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
    pub max_quality: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ManagedDictionary {
    pub magic: uint32_t,
    pub memory_manager_: MemoryManager,
    pub dictionary: *mut uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    BrotliFree(m, (*trie).pool as *mut ::core::ffi::c_void);
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
        (*dict).hash_table_data_words_ as *mut ::core::ffi::c_void,
    );
    BrotliFree(
        m,
        (*dict).hash_table_data_lengths_ as *mut ::core::ffi::c_void,
    );
    BrotliFree(m, (*dict).buckets_data_ as *mut ::core::ffi::c_void);
    BrotliFree(m, (*dict).dict_words_data_ as *mut ::core::ffi::c_void);
    BrotliFree(m, (*dict).words_instance_ as *mut ::core::ffi::c_void);
    BrotliTrieFree(m, &raw mut (*dict).trie);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliInitSharedEncoderDictionary(mut dict: *mut SharedEncoderDictionary) {
    (*dict).magic = kSharedDictionaryMagic;
    (*dict).compound.num_chunks = 0 as size_t;
    (*dict).compound.total_size = 0 as size_t;
    (*dict).compound.chunk_offsets[0 as ::core::ffi::c_int as usize] = 0 as size_t;
    (*dict).compound.num_prepared_instances_ = 0 as size_t;
    (*dict).contextual.context_based = 0 as ::core::ffi::c_int;
    (*dict).contextual.num_dictionaries = 1 as uint8_t;
    (*dict).contextual.instances_ = ::core::ptr::null_mut::<BrotliEncoderDictionary>();
    (*dict).contextual.num_instances_ = 1 as size_t;
    (*dict).contextual.dict[0 as ::core::ffi::c_int as usize] =
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
        BrotliFree(m, (*dict).contextual.instances_ as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCreateManagedDictionary(
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut ::core::ffi::c_void,
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
        dictionary as *mut ::core::ffi::c_void,
        &raw mut (*dictionary).memory_manager_,
    );
}
pub const BROTLI_MAX_QUALITY: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
static mut kSharedDictionaryMagic: uint32_t = 0xdebcede1 as uint32_t;
static mut kManagedDictionaryMagic: uint32_t = 0xdebcede2 as uint32_t;
static mut kCutoffTransformsCount: uint32_t = 10 as uint32_t;
static mut kCutoffTransforms: uint64_t = (0x71b520a as ::core::ffi::c_int as uint64_t)
    << 32 as ::core::ffi::c_int
    | 0xda2d3200 as uint64_t;
