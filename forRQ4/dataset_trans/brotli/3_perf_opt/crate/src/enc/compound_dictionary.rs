use core::ffi::*;
use crate::src::c_inlined_fns::BROTLI_UNALIGNED_LOAD_PTR;
use crate::src::enc::memory::BrotliAllocate;
use crate::src::enc::memory::BrotliFree;
use crate::src::c_inlined_fns::BrotliUnalignedRead64;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[inline(always)]
unsafe fn BROTLI_UNALIGNED_STORE_PTR(
    mut p: *mut c_void,
    mut v: *const c_void,
) {
    memcpy(
        p,
        &raw mut v as *const c_void,
        ::core::mem::size_of::<*mut c_void>() as size_t,
    );
}

pub const SHARED_BROTLI_MAX_RAW_DICT_SIZE: c_uint = (1 as c_uint)
    << (23 as usize).wrapping_add(::core::mem::size_of::<size_t>() as usize);
static mut kPreparedDictionaryMagic: uint32_t = 0xdebcede0 as uint32_t;
static mut kLeanPreparedDictionaryMagic: uint32_t = 0xdebcede3 as uint32_t;
static mut kPreparedDictionaryHashMul64Long: uint64_t =
    (0x1fe35a7b as c_uint as uint64_t) << 32 as c_int
        | 0xd3579bd3 as uint64_t;
unsafe fn CreatePreparedDictionaryWithParams(
    mut m: *mut MemoryManager,
    mut source: *const uint8_t,
    mut source_size: size_t,
    mut bucket_bits: uint32_t,
    mut slot_bits: uint32_t,
    mut hash_bits: uint32_t,
    mut bucket_limit: uint16_t,
) -> *mut PreparedDictionary {
    let mut num_slots: uint32_t = (1 as uint32_t) << slot_bits;
    let mut num_buckets: uint32_t = (1 as uint32_t) << bucket_bits;
    let mut hash_shift: uint32_t = (64 as uint32_t).wrapping_sub(bucket_bits);
    let mut hash_mask: uint64_t =
        !(0 as c_uint as uint64_t) >> (64 as uint32_t).wrapping_sub(hash_bits);
    let mut slot_mask: uint32_t = num_slots.wrapping_sub(1 as uint32_t);
    let mut alloc_size: size_t = ((::core::mem::size_of::<uint32_t>() as size_t) << slot_bits)
        .wrapping_add((::core::mem::size_of::<uint32_t>() as size_t) << slot_bits)
        .wrapping_add((::core::mem::size_of::<uint16_t>() as size_t) << bucket_bits)
        .wrapping_add((::core::mem::size_of::<uint32_t>() as size_t) << bucket_bits)
        .wrapping_add((::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(source_size));
    let mut flat: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut result: *mut PreparedDictionary = ::core::ptr::null_mut::<PreparedDictionary>();
    let mut num: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    let mut bucket_heads: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut next_bucket: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut slot_offsets: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut heads: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    let mut items: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut source_ref: *mut *mut uint8_t = ::core::ptr::null_mut::<*mut uint8_t>();
    let mut i: uint32_t = 0;
    let mut slot_size: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut slot_limit: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut total_items: uint32_t = 0 as uint32_t;
    if slot_bits > 16 as uint32_t {
        return ::core::ptr::null_mut::<PreparedDictionary>();
    }
    if slot_bits > bucket_bits {
        return ::core::ptr::null_mut::<PreparedDictionary>();
    }
    if bucket_bits.wrapping_sub(slot_bits) >= 16 as uint32_t {
        return ::core::ptr::null_mut::<PreparedDictionary>();
    }
    flat = if alloc_size > 0 as size_t {
        BrotliAllocate(
            m,
            alloc_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    if 0 as c_int != 0 || 0 as c_int != 0 {
        return ::core::ptr::null_mut::<PreparedDictionary>();
    }
    slot_size = flat as *mut uint32_t;
    slot_limit = slot_size.offset(num_slots as isize) as *mut uint32_t;
    num = slot_limit.offset(num_slots as isize) as *mut uint32_t as *mut uint16_t;
    bucket_heads = num.offset(num_buckets as isize) as *mut uint16_t as *mut uint32_t;
    next_bucket = bucket_heads.offset(num_buckets as isize) as *mut uint32_t;
    memset(
        num as *mut c_void,
        0 as c_int,
        (num_buckets as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
    i = 0 as uint32_t;
    while (i.wrapping_add(7 as uint32_t) as size_t) < source_size {
        let h: uint64_t = (BrotliUnalignedRead64(
            source.offset(i as isize) as *const uint8_t as *const c_void
        ) as uint64_t
            & hash_mask)
            .wrapping_mul(kPreparedDictionaryHashMul64Long);
        let key: uint32_t = (h >> hash_shift) as uint32_t;
        let mut count: uint16_t = *num.offset(key as isize);
        *next_bucket.offset(i as isize) = if count as c_int == 0 as c_int
        {
            -(1 as c_int) as uint32_t
        } else {
            *bucket_heads.offset(key as isize)
        };
        *bucket_heads.offset(key as isize) = i;
        count = count.wrapping_add(1);
        if count as c_int > bucket_limit as c_int {
            count = bucket_limit;
        }
        *num.offset(key as isize) = count;
        i = i.wrapping_add(1);
    }
    i = 0 as uint32_t;
    while i < num_slots {
        let mut overflow: c_int = BROTLI_FALSE;
        *slot_limit.offset(i as isize) = bucket_limit as uint32_t;
        loop {
            let mut limit: uint32_t = *slot_limit.offset(i as isize);
            let mut j: size_t = 0;
            let mut count_0: uint32_t = 0 as uint32_t;
            overflow = BROTLI_FALSE;
            j = i as size_t;
            while j < num_buckets as size_t {
                let mut size: uint32_t = *num.offset(j as isize) as uint32_t;
                if count_0 >= 0xffff as uint32_t {
                    overflow = BROTLI_TRUE;
                    break;
                } else {
                    if size > limit {
                        size = limit;
                    }
                    count_0 = (count_0 as c_uint)
                        .wrapping_add(size as c_uint)
                        as uint32_t as uint32_t;
                    j = (j as c_ulong).wrapping_add(num_slots as c_ulong)
                        as size_t as size_t;
                }
            }
            if overflow == 0 {
                *slot_size.offset(i as isize) = count_0;
                total_items = (total_items as c_uint)
                    .wrapping_add(count_0 as c_uint)
                    as uint32_t as uint32_t;
                break;
            } else {
                let ref mut fresh0 = *slot_limit.offset(i as isize);
                *fresh0 = (*fresh0).wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }
    alloc_size = (::core::mem::size_of::<PreparedDictionary>() as usize)
        .wrapping_add((::core::mem::size_of::<uint32_t>() as usize) << slot_bits)
        .wrapping_add((::core::mem::size_of::<uint16_t>() as usize) << bucket_bits)
        .wrapping_add(
            (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(total_items as usize),
        )
        .wrapping_add(::core::mem::size_of::<*mut uint8_t>() as usize) as size_t;
    result = (if alloc_size > 0 as size_t {
        BrotliAllocate(
            m,
            alloc_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    }) as *mut PreparedDictionary;
    if 0 as c_int != 0 || 0 as c_int != 0 {
        BrotliFree(m, flat as *mut c_void);
        flat = ::core::ptr::null_mut::<uint8_t>();
        return ::core::ptr::null_mut::<PreparedDictionary>();
    }
    slot_offsets =
        result.offset(1 as c_int as isize) as *mut PreparedDictionary as *mut uint32_t;
    heads = slot_offsets.offset(num_slots as isize) as *mut uint32_t as *mut uint16_t;
    items = heads.offset(num_buckets as isize) as *mut uint16_t as *mut uint32_t;
    source_ref = items.offset(total_items as isize) as *mut uint32_t as *mut *mut uint8_t;
    (*result).magic = kLeanPreparedDictionaryMagic;
    (*result).num_items = total_items;
    (*result).source_size = source_size as uint32_t;
    (*result).hash_bits = hash_bits;
    (*result).bucket_bits = bucket_bits;
    (*result).slot_bits = slot_bits;
    BROTLI_UNALIGNED_STORE_PTR(
        source_ref as *mut c_void,
        source as *const c_void,
    );
    total_items = 0 as uint32_t;
    i = 0 as uint32_t;
    while i < num_slots {
        *slot_offsets.offset(i as isize) = total_items;
        total_items = (total_items as c_uint)
            .wrapping_add(*slot_size.offset(i as isize) as c_uint)
            as uint32_t as uint32_t;
        *slot_size.offset(i as isize) = 0 as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as uint32_t;
    while i < num_buckets {
        let mut slot: uint32_t = i & slot_mask;
        let mut count_1: uint32_t = *num.offset(i as isize) as uint32_t;
        let mut pos: uint32_t = 0;
        let mut j_0: size_t = 0;
        let mut cursor: size_t = *slot_size.offset(slot as isize) as size_t;
        if count_1 > *slot_limit.offset(slot as isize) {
            count_1 = *slot_limit.offset(slot as isize);
        }
        if count_1 == 0 as uint32_t {
            *heads.offset(i as isize) = 0xffff as uint16_t;
        } else {
            *heads.offset(i as isize) = cursor as uint16_t;
            cursor = (cursor as c_ulong)
                .wrapping_add(*slot_offsets.offset(slot as isize) as c_ulong)
                as size_t as size_t;
            let ref mut fresh1 = *slot_size.offset(slot as isize);
            *fresh1 = (*fresh1 as c_uint).wrapping_add(count_1 as c_uint)
                as uint32_t as uint32_t;
            pos = *bucket_heads.offset(i as isize);
            j_0 = 0 as size_t;
            while j_0 < count_1 as size_t {
                let fresh2 = cursor;
                cursor = cursor.wrapping_add(1);
                *items.offset(fresh2 as isize) = pos;
                pos = *next_bucket.offset(pos as isize);
                j_0 = j_0.wrapping_add(1);
            }
            let ref mut fresh3 = *items.offset(cursor.wrapping_sub(1 as size_t) as isize);
            *fresh3 =
                (*fresh3 as c_uint | 0x80000000 as c_uint) as uint32_t;
        }
        i = i.wrapping_add(1);
    }
    BrotliFree(m, flat as *mut c_void);
    flat = ::core::ptr::null_mut::<uint8_t>();
    return result;
}
#[inline]
pub unsafe fn CreatePreparedDictionary(
    mut m: *mut MemoryManager,
    mut source: *const uint8_t,
    mut source_size: size_t,
) -> *mut PreparedDictionary {
    let mut bucket_bits: uint32_t = 17 as uint32_t;
    let mut slot_bits: uint32_t = 7 as uint32_t;
    let mut hash_bits: uint32_t = 40 as uint32_t;
    let mut bucket_limit: uint16_t = 32 as uint16_t;
    let mut volume: size_t = ((16 as c_uint) << bucket_bits) as size_t;
    if source_size > SHARED_BROTLI_MAX_RAW_DICT_SIZE as size_t {
        return ::core::ptr::null_mut::<PreparedDictionary>();
    }
    while volume < source_size && bucket_bits < 22 as uint32_t {
        bucket_bits = bucket_bits.wrapping_add(1);
        slot_bits = slot_bits.wrapping_add(1);
        volume <<= 1 as c_int;
    }
    return CreatePreparedDictionaryWithParams(
        m,
        source,
        source_size,
        bucket_bits,
        slot_bits,
        hash_bits,
        bucket_limit,
    );
}
#[inline]
pub unsafe fn DestroyPreparedDictionary(
    mut m: *mut MemoryManager,
    mut dictionary: *mut PreparedDictionary,
) {
    if dictionary.is_null() {
        return;
    }
    BrotliFree(m, dictionary as *mut c_void);
    dictionary = ::core::ptr::null_mut::<PreparedDictionary>();
}
#[inline]
pub unsafe fn AttachPreparedDictionary(
    mut compound: *mut CompoundDictionary,
    mut dictionary: *const PreparedDictionary,
) -> c_int {
    let mut length: size_t = 0 as size_t;
    let mut index: size_t = 0 as size_t;
    if (*compound).num_chunks == SHARED_BROTLI_MAX_COMPOUND_DICTS as size_t {
        return BROTLI_FALSE;
    }
    if dictionary.is_null() {
        return BROTLI_FALSE;
    }
    length = (*dictionary).source_size as size_t;
    if length > (SHARED_BROTLI_MAX_RAW_DICT_SIZE as size_t).wrapping_sub((*compound).total_size) {
        return BROTLI_FALSE;
    }
    index = (*compound).num_chunks;
    (*compound).total_size = ((*compound).total_size as c_ulong)
        .wrapping_add(length as c_ulong) as size_t
        as size_t;
    (*compound).chunks[index as usize] = dictionary;
    (*compound).chunk_offsets[index.wrapping_add(1 as size_t) as usize] = (*compound).total_size;
    let mut slot_offsets: *mut uint32_t = dictionary.offset(1 as c_int as isize)
        as *const PreparedDictionary as *mut uint32_t;
    let mut heads: *mut uint16_t = slot_offsets
        .offset(((1 as c_uint as size_t) << (*dictionary).slot_bits) as isize)
        as *mut uint32_t as *mut uint16_t;
    let mut items: *mut uint32_t = heads
        .offset(((1 as c_uint as size_t) << (*dictionary).bucket_bits) as isize)
        as *mut uint16_t as *mut uint32_t;
    let mut tail: *const c_void =
        items.offset((*dictionary).num_items as isize) as *mut uint32_t as *mut c_void;
    if (*dictionary).magic == kPreparedDictionaryMagic {
        (*compound).chunk_source[index as usize] = tail as *const uint8_t;
    } else {
        (*compound).chunk_source[index as usize] =
            BROTLI_UNALIGNED_LOAD_PTR(tail as *mut *const uint8_t as *const c_void)
                as *const uint8_t;
    }
    (*compound).num_chunks = (*compound).num_chunks.wrapping_add(1);
    return BROTLI_TRUE;
}
