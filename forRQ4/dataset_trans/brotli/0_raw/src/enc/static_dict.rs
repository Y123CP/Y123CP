extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub const BROTLI_TRANSFORM_UPPERCASE_FIRST: BrotliWordTransformType = 10;
pub type BrotliWordTransformType = ::core::ffi::c_uint;
pub const BROTLI_NUM_TRANSFORM_TYPES: BrotliWordTransformType = 23;
pub const BROTLI_TRANSFORM_SHIFT_ALL: BrotliWordTransformType = 22;
pub const BROTLI_TRANSFORM_SHIFT_FIRST: BrotliWordTransformType = 21;
pub const BROTLI_TRANSFORM_OMIT_FIRST_9: BrotliWordTransformType = 20;
pub const BROTLI_TRANSFORM_OMIT_FIRST_8: BrotliWordTransformType = 19;
pub const BROTLI_TRANSFORM_OMIT_FIRST_7: BrotliWordTransformType = 18;
pub const BROTLI_TRANSFORM_OMIT_FIRST_6: BrotliWordTransformType = 17;
pub const BROTLI_TRANSFORM_OMIT_FIRST_5: BrotliWordTransformType = 16;
pub const BROTLI_TRANSFORM_OMIT_FIRST_4: BrotliWordTransformType = 15;
pub const BROTLI_TRANSFORM_OMIT_FIRST_3: BrotliWordTransformType = 14;
pub const BROTLI_TRANSFORM_OMIT_FIRST_2: BrotliWordTransformType = 13;
pub const BROTLI_TRANSFORM_OMIT_FIRST_1: BrotliWordTransformType = 12;
pub const BROTLI_TRANSFORM_UPPERCASE_ALL: BrotliWordTransformType = 11;
pub const BROTLI_TRANSFORM_OMIT_LAST_9: BrotliWordTransformType = 9;
pub const BROTLI_TRANSFORM_OMIT_LAST_8: BrotliWordTransformType = 8;
pub const BROTLI_TRANSFORM_OMIT_LAST_7: BrotliWordTransformType = 7;
pub const BROTLI_TRANSFORM_OMIT_LAST_6: BrotliWordTransformType = 6;
pub const BROTLI_TRANSFORM_OMIT_LAST_5: BrotliWordTransformType = 5;
pub const BROTLI_TRANSFORM_OMIT_LAST_4: BrotliWordTransformType = 4;
pub const BROTLI_TRANSFORM_OMIT_LAST_3: BrotliWordTransformType = 3;
pub const BROTLI_TRANSFORM_OMIT_LAST_2: BrotliWordTransformType = 2;
pub const BROTLI_TRANSFORM_OMIT_LAST_1: BrotliWordTransformType = 1;
pub const BROTLI_TRANSFORM_IDENTITY: BrotliWordTransformType = 0;
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead32(mut p: *const ::core::ffi::c_void) -> uint32_t {
    let mut t: uint32_t = 0;
    memcpy(
        &raw mut t as *mut ::core::ffi::c_void,
        p,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    return t;
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead64(mut p: *const ::core::ffi::c_void) -> uint64_t {
    let mut t: uint64_t = 0;
    memcpy(
        &raw mut t as *mut ::core::ffi::c_void,
        p,
        ::core::mem::size_of::<uint64_t>() as size_t,
    );
    return t;
}
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_max_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_min_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a < b { a } else { b };
}
pub const BROTLI_MAX_STATIC_DICTIONARY_MATCH_LEN: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
static mut kInvalidMatch: uint32_t = 0xfffffff as uint32_t;
#[inline(always)]
unsafe extern "C" fn AddMatch(
    mut distance: size_t,
    mut len: size_t,
    mut len_code: size_t,
    mut matches: *mut uint32_t,
) {
    let mut match_0: uint32_t =
        (distance << 5 as ::core::ffi::c_int).wrapping_add(len_code) as uint32_t;
    *matches.offset(len as isize) = brotli_min_uint32_t(*matches.offset(len as isize), match_0);
}
#[inline(always)]
unsafe extern "C" fn DictMatchLength(
    mut dictionary: *const BrotliDictionary,
    mut data: *const uint8_t,
    mut id: size_t,
    mut len: size_t,
    mut maxlen: size_t,
) -> size_t {
    let offset: size_t = ((*dictionary).offsets_by_length[len as usize] as size_t)
        .wrapping_add(len.wrapping_mul(id));
    return FindMatchLengthWithLimit(
        (*dictionary).data.offset(offset as isize) as *const uint8_t,
        data,
        brotli_min_size_t(len, maxlen),
    );
}
#[inline(always)]
unsafe extern "C" fn IsMatch(
    mut dictionary: *const BrotliDictionary,
    mut w: DictWord,
    mut data: *const uint8_t,
    mut max_length: size_t,
) -> ::core::ffi::c_int {
    if w.len as size_t > max_length {
        return BROTLI_FALSE;
    } else {
        let offset: size_t = ((*dictionary).offsets_by_length[w.len as usize] as size_t)
            .wrapping_add((w.len as size_t).wrapping_mul(w.idx as size_t));
        let mut dict: *const uint8_t = (*dictionary).data.offset(offset as isize) as *const uint8_t;
        if w.transform as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return if FindMatchLengthWithLimit(dict, data, w.len as size_t) == w.len as size_t {
                BROTLI_TRUE
            } else {
                BROTLI_FALSE
            };
        } else if w.transform as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
            return if *dict.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'a' as i32
                && *dict.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 'z' as i32
                && *dict.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ^ 32 as ::core::ffi::c_int
                    == *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                && FindMatchLengthWithLimit(
                    dict.offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                    data.offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                    (w.len as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint) as size_t,
                ) == (w.len as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                    as size_t
            {
                BROTLI_TRUE
            } else {
                BROTLI_FALSE
            };
        } else {
            let mut i: size_t = 0;
            i = 0 as size_t;
            while i < w.len as size_t {
                if *dict.offset(i as isize) as ::core::ffi::c_int >= 'a' as i32
                    && *dict.offset(i as isize) as ::core::ffi::c_int <= 'z' as i32
                {
                    if *dict.offset(i as isize) as ::core::ffi::c_int ^ 32 as ::core::ffi::c_int
                        != *data.offset(i as isize) as ::core::ffi::c_int
                    {
                        return BROTLI_FALSE;
                    }
                } else if *dict.offset(i as isize) as ::core::ffi::c_int
                    != *data.offset(i as isize) as ::core::ffi::c_int
                {
                    return BROTLI_FALSE;
                }
                i = i.wrapping_add(1);
            }
            return BROTLI_TRUE;
        }
    };
}
unsafe extern "C" fn BrotliFindAllStaticDictionaryMatchesFor(
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    mut min_length: size_t,
    mut max_length: size_t,
    mut matches: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut has_found_match: ::core::ffi::c_int = BROTLI_FALSE;
    let mut offset: size_t = *(*dictionary).buckets.offset(Hash15(data) as isize) as size_t;
    let mut end: ::core::ffi::c_int = (offset == 0) as ::core::ffi::c_int;
    while end == 0 {
        let fresh0 = offset;
        offset = offset.wrapping_add(1);
        let mut w: DictWord = *(*dictionary).dict_words.offset(fresh0 as isize);
        let l: size_t = (w.len as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as size_t;
        let n: size_t = (1 as ::core::ffi::c_int as size_t)
            << (*(*dictionary).words).size_bits_by_length[l as usize] as ::core::ffi::c_int;
        let id: size_t = w.idx as size_t;
        end = (w.len as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
        w.len = l as uint8_t;
        if w.transform as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let matchlen: size_t =
                DictMatchLength((*dictionary).words, data, id, l, max_length) as size_t;
            let mut s: *const uint8_t = ::core::ptr::null::<uint8_t>();
            let mut minlen: size_t = 0;
            let mut maxlen: size_t = 0;
            let mut len: size_t = 0;
            if matchlen == l {
                AddMatch(id, l, l, matches);
                has_found_match = BROTLI_TRUE;
            }
            if matchlen >= l.wrapping_sub(1 as size_t) {
                AddMatch(
                    id.wrapping_add((12 as size_t).wrapping_mul(n)),
                    l.wrapping_sub(1 as size_t),
                    l,
                    matches,
                );
                if l.wrapping_add(2 as size_t) < max_length
                    && *data.offset(l.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                        == 'i' as i32
                    && *data.offset(l as isize) as ::core::ffi::c_int == 'n' as i32
                    && *data.offset(l.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                        == 'g' as i32
                    && *data.offset(l.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                        == ' ' as i32
                {
                    AddMatch(
                        id.wrapping_add((49 as size_t).wrapping_mul(n)),
                        l.wrapping_add(3 as size_t),
                        l,
                        matches,
                    );
                }
                has_found_match = BROTLI_TRUE;
            }
            minlen = min_length;
            if l > 9 as size_t {
                minlen = brotli_max_size_t(minlen, l.wrapping_sub(9 as size_t));
            }
            maxlen = brotli_min_size_t(matchlen, l.wrapping_sub(2 as size_t));
            len = minlen;
            while len <= maxlen {
                let mut cut: size_t = l.wrapping_sub(len);
                let mut transform_id: size_t = (cut << 2 as ::core::ffi::c_int).wrapping_add(
                    ((*dictionary).cutoffTransforms >> cut.wrapping_mul(6 as size_t)
                        & 0x3f as uint64_t) as size_t,
                );
                AddMatch(
                    id.wrapping_add(transform_id.wrapping_mul(n)),
                    len,
                    l,
                    matches,
                );
                has_found_match = BROTLI_TRUE;
                len = len.wrapping_add(1);
            }
            if matchlen < l || l.wrapping_add(6 as size_t) >= max_length {
                continue;
            }
            s = data.offset(l as isize) as *const uint8_t;
            if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
                AddMatch(id.wrapping_add(n), l.wrapping_add(1 as size_t), l, matches);
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'a' as i32 {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((28 as size_t).wrapping_mul(n)),
                            l.wrapping_add(3 as size_t),
                            l,
                            matches,
                        );
                    } else if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 's' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((46 as size_t).wrapping_mul(n)),
                                l.wrapping_add(4 as size_t),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 't' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((60 as size_t).wrapping_mul(n)),
                                l.wrapping_add(4 as size_t),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'n' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'd' as i32
                            && *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((10 as size_t).wrapping_mul(n)),
                                l.wrapping_add(5 as size_t),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'b' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'y' as i32
                        && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((38 as size_t).wrapping_mul(n)),
                            l.wrapping_add(4 as size_t),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'i' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'n' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((16 as size_t).wrapping_mul(n)),
                                l.wrapping_add(4 as size_t),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 's' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((47 as size_t).wrapping_mul(n)),
                                l.wrapping_add(4 as size_t),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'f' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'o' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'r' as i32
                            && *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((25 as size_t).wrapping_mul(n)),
                                l.wrapping_add(5 as size_t),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'r' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'o' as i32
                            && *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == 'm' as i32
                            && *s.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((37 as size_t).wrapping_mul(n)),
                                l.wrapping_add(6 as size_t),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'o' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'f' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((8 as size_t).wrapping_mul(n)),
                                l.wrapping_add(4 as size_t),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'n' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((45 as size_t).wrapping_mul(n)),
                                l.wrapping_add(4 as size_t),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'n' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'o' as i32
                        && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 't' as i32
                        && *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((80 as size_t).wrapping_mul(n)),
                            l.wrapping_add(5 as size_t),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 't' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'h' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'e' as i32
                        {
                            if *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == ' ' as i32
                            {
                                AddMatch(
                                    id.wrapping_add((5 as size_t).wrapping_mul(n)),
                                    l.wrapping_add(5 as size_t),
                                    l,
                                    matches,
                                );
                            }
                        } else if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'a' as i32
                        {
                            if *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == 't' as i32
                                && *s.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                    == ' ' as i32
                            {
                                AddMatch(
                                    id.wrapping_add((29 as size_t).wrapping_mul(n)),
                                    l.wrapping_add(6 as size_t),
                                    l,
                                    matches,
                                );
                            }
                        }
                    } else if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'o' as i32
                    {
                        if *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add((17 as size_t).wrapping_mul(n)),
                                l.wrapping_add(4 as size_t),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'w' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'i' as i32
                        && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 't' as i32
                        && *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'h' as i32
                        && *s.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((35 as size_t).wrapping_mul(n)),
                            l.wrapping_add(6 as size_t),
                            l,
                            matches,
                        );
                    }
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '"' as i32
            {
                AddMatch(
                    id.wrapping_add((19 as size_t).wrapping_mul(n)),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32 {
                    AddMatch(
                        id.wrapping_add((21 as size_t).wrapping_mul(n)),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '.' as i32
            {
                AddMatch(
                    id.wrapping_add((20 as size_t).wrapping_mul(n)),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
                    AddMatch(
                        id.wrapping_add((31 as size_t).wrapping_mul(n)),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'T' as i32
                        && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'h' as i32
                    {
                        if *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'e' as i32
                        {
                            if *s.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == ' ' as i32
                            {
                                AddMatch(
                                    id.wrapping_add((43 as size_t).wrapping_mul(n)),
                                    l.wrapping_add(6 as size_t),
                                    l,
                                    matches,
                                );
                            }
                        } else if *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'i' as i32
                        {
                            if *s.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == 's' as i32
                                && *s.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                    == ' ' as i32
                            {
                                AddMatch(
                                    id.wrapping_add((75 as size_t).wrapping_mul(n)),
                                    l.wrapping_add(7 as size_t),
                                    l,
                                    matches,
                                );
                            }
                        }
                    }
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ',' as i32
            {
                AddMatch(
                    id.wrapping_add((76 as size_t).wrapping_mul(n)),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
                    AddMatch(
                        id.wrapping_add((14 as size_t).wrapping_mul(n)),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32
            {
                AddMatch(
                    id.wrapping_add((22 as size_t).wrapping_mul(n)),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\t' as i32
                {
                    AddMatch(
                        id.wrapping_add((50 as size_t).wrapping_mul(n)),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ']' as i32
            {
                AddMatch(
                    id.wrapping_add((24 as size_t).wrapping_mul(n)),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32
            {
                AddMatch(
                    id.wrapping_add((36 as size_t).wrapping_mul(n)),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ':' as i32
            {
                AddMatch(
                    id.wrapping_add((51 as size_t).wrapping_mul(n)),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '(' as i32
            {
                AddMatch(
                    id.wrapping_add((57 as size_t).wrapping_mul(n)),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '=' as i32
            {
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '"' as i32 {
                    AddMatch(
                        id.wrapping_add((70 as size_t).wrapping_mul(n)),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\'' as i32
                {
                    AddMatch(
                        id.wrapping_add((86 as size_t).wrapping_mul(n)),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'a' as i32
            {
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'l' as i32
                    && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                {
                    AddMatch(
                        id.wrapping_add((84 as size_t).wrapping_mul(n)),
                        l.wrapping_add(3 as size_t),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'e' as i32
            {
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'd' as i32 {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((53 as size_t).wrapping_mul(n)),
                            l.wrapping_add(3 as size_t),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'r' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((82 as size_t).wrapping_mul(n)),
                            l.wrapping_add(3 as size_t),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 's' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 't' as i32
                        && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((95 as size_t).wrapping_mul(n)),
                            l.wrapping_add(4 as size_t),
                            l,
                            matches,
                        );
                    }
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'f' as i32
            {
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'u' as i32
                    && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'l' as i32
                    && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                {
                    AddMatch(
                        id.wrapping_add((90 as size_t).wrapping_mul(n)),
                        l.wrapping_add(4 as size_t),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'i' as i32
            {
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'v' as i32 {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'e' as i32
                        && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((92 as size_t).wrapping_mul(n)),
                            l.wrapping_add(4 as size_t),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'z' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'e' as i32
                        && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((100 as size_t).wrapping_mul(n)),
                            l.wrapping_add(4 as size_t),
                            l,
                            matches,
                        );
                    }
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'l' as i32
            {
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'e' as i32 {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 's' as i32
                        && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 's' as i32
                        && *s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((93 as size_t).wrapping_mul(n)),
                            l.wrapping_add(5 as size_t),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'y' as i32
                {
                    if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add((61 as size_t).wrapping_mul(n)),
                            l.wrapping_add(3 as size_t),
                            l,
                            matches,
                        );
                    }
                }
            } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'o' as i32
            {
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'u' as i32
                    && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 's' as i32
                    && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                {
                    AddMatch(
                        id.wrapping_add((106 as size_t).wrapping_mul(n)),
                        l.wrapping_add(4 as size_t),
                        l,
                        matches,
                    );
                }
            }
        } else {
            let is_all_caps: ::core::ffi::c_int = if w.transform as ::core::ffi::c_int
                != BROTLI_TRANSFORM_UPPERCASE_FIRST as ::core::ffi::c_int
            {
                BROTLI_TRUE
            } else {
                BROTLI_FALSE
            };
            let mut s_0: *const uint8_t = ::core::ptr::null::<uint8_t>();
            if IsMatch((*dictionary).words, w, data, max_length) == 0 {
                continue;
            }
            AddMatch(
                id.wrapping_add(
                    ((if is_all_caps != 0 {
                        44 as ::core::ffi::c_int
                    } else {
                        9 as ::core::ffi::c_int
                    }) as size_t)
                        .wrapping_mul(n),
                ),
                l,
                l,
                matches,
            );
            has_found_match = BROTLI_TRUE;
            if l.wrapping_add(1 as size_t) >= max_length {
                continue;
            }
            s_0 = data.offset(l as isize) as *const uint8_t;
            if *s_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
                AddMatch(
                    id.wrapping_add(
                        ((if is_all_caps != 0 {
                            68 as ::core::ffi::c_int
                        } else {
                            4 as ::core::ffi::c_int
                        }) as size_t)
                            .wrapping_mul(n),
                    ),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
            } else if *s_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '"' as i32
            {
                AddMatch(
                    id.wrapping_add(
                        ((if is_all_caps != 0 {
                            87 as ::core::ffi::c_int
                        } else {
                            66 as ::core::ffi::c_int
                        }) as size_t)
                            .wrapping_mul(n),
                    ),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
                if *s_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32
                {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 {
                                97 as ::core::ffi::c_int
                            } else {
                                69 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n),
                        ),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                }
            } else if *s_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '.' as i32
            {
                AddMatch(
                    id.wrapping_add(
                        ((if is_all_caps != 0 {
                            101 as ::core::ffi::c_int
                        } else {
                            79 as ::core::ffi::c_int
                        }) as size_t)
                            .wrapping_mul(n),
                    ),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
                if *s_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
                {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 {
                                114 as ::core::ffi::c_int
                            } else {
                                88 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n),
                        ),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                }
            } else if *s_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ',' as i32
            {
                AddMatch(
                    id.wrapping_add(
                        ((if is_all_caps != 0 {
                            112 as ::core::ffi::c_int
                        } else {
                            99 as ::core::ffi::c_int
                        }) as size_t)
                            .wrapping_mul(n),
                    ),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
                if *s_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
                {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 {
                                107 as ::core::ffi::c_int
                            } else {
                                58 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n),
                        ),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                }
            } else if *s_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32
            {
                AddMatch(
                    id.wrapping_add(
                        ((if is_all_caps != 0 {
                            94 as ::core::ffi::c_int
                        } else {
                            74 as ::core::ffi::c_int
                        }) as size_t)
                            .wrapping_mul(n),
                    ),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
            } else if *s_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '(' as i32
            {
                AddMatch(
                    id.wrapping_add(
                        ((if is_all_caps != 0 {
                            113 as ::core::ffi::c_int
                        } else {
                            78 as ::core::ffi::c_int
                        }) as size_t)
                            .wrapping_mul(n),
                    ),
                    l.wrapping_add(1 as size_t),
                    l,
                    matches,
                );
            } else if *s_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '=' as i32
            {
                if *s_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '"' as i32
                {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 {
                                105 as ::core::ffi::c_int
                            } else {
                                104 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n),
                        ),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                } else if *s_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\'' as i32
                {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 {
                                116 as ::core::ffi::c_int
                            } else {
                                108 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n),
                        ),
                        l.wrapping_add(2 as size_t),
                        l,
                        matches,
                    );
                }
            }
        }
    }
    if max_length >= 5 as size_t
        && (*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
            || *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32)
    {
        let mut is_space: ::core::ffi::c_int =
            if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32 {
                BROTLI_TRUE
            } else {
                BROTLI_FALSE
            };
        let mut offset_0: size_t = *(*dictionary).buckets.offset(Hash15(
            data.offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
        ) as isize) as size_t;
        let mut end_0: ::core::ffi::c_int = (offset_0 == 0) as ::core::ffi::c_int;
        while end_0 == 0 {
            let fresh1 = offset_0;
            offset_0 = offset_0.wrapping_add(1);
            let mut w_0: DictWord = *(*dictionary).dict_words.offset(fresh1 as isize);
            let l_0: size_t =
                (w_0.len as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as size_t;
            let n_0: size_t = (1 as ::core::ffi::c_int as size_t)
                << (*(*dictionary).words).size_bits_by_length[l_0 as usize] as ::core::ffi::c_int;
            let id_0: size_t = w_0.idx as size_t;
            end_0 = (w_0.len as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int;
            w_0.len = l_0 as uint8_t;
            if w_0.transform as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let mut s_1: *const uint8_t = ::core::ptr::null::<uint8_t>();
                if IsMatch(
                    (*dictionary).words,
                    w_0,
                    data.offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                    max_length.wrapping_sub(1 as size_t),
                ) == 0
                {
                    continue;
                }
                AddMatch(
                    id_0.wrapping_add(
                        ((if is_space != 0 {
                            6 as ::core::ffi::c_int
                        } else {
                            32 as ::core::ffi::c_int
                        }) as size_t)
                            .wrapping_mul(n_0),
                    ),
                    l_0.wrapping_add(1 as size_t),
                    l_0,
                    matches,
                );
                has_found_match = BROTLI_TRUE;
                if l_0.wrapping_add(2 as size_t) >= max_length {
                    continue;
                }
                s_1 = data.offset(l_0.wrapping_add(1 as size_t) as isize) as *const uint8_t;
                if *s_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
                {
                    AddMatch(
                        id_0.wrapping_add(
                            ((if is_space != 0 {
                                2 as ::core::ffi::c_int
                            } else {
                                77 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n_0),
                        ),
                        l_0.wrapping_add(2 as size_t),
                        l_0,
                        matches,
                    );
                } else if *s_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '(' as i32
                {
                    AddMatch(
                        id_0.wrapping_add(
                            ((if is_space != 0 {
                                89 as ::core::ffi::c_int
                            } else {
                                67 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n_0),
                        ),
                        l_0.wrapping_add(2 as size_t),
                        l_0,
                        matches,
                    );
                } else if is_space != 0 {
                    if *s_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ',' as i32
                    {
                        AddMatch(
                            id_0.wrapping_add((103 as size_t).wrapping_mul(n_0)),
                            l_0.wrapping_add(2 as size_t),
                            l_0,
                            matches,
                        );
                        if *s_1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id_0.wrapping_add((33 as size_t).wrapping_mul(n_0)),
                                l_0.wrapping_add(3 as size_t),
                                l_0,
                                matches,
                            );
                        }
                    } else if *s_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '.' as i32
                    {
                        AddMatch(
                            id_0.wrapping_add((71 as size_t).wrapping_mul(n_0)),
                            l_0.wrapping_add(2 as size_t),
                            l_0,
                            matches,
                        );
                        if *s_1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id_0.wrapping_add((52 as size_t).wrapping_mul(n_0)),
                                l_0.wrapping_add(3 as size_t),
                                l_0,
                                matches,
                            );
                        }
                    } else if *s_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '=' as i32
                    {
                        if *s_1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '"' as i32
                        {
                            AddMatch(
                                id_0.wrapping_add((81 as size_t).wrapping_mul(n_0)),
                                l_0.wrapping_add(3 as size_t),
                                l_0,
                                matches,
                            );
                        } else if *s_1.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '\'' as i32
                        {
                            AddMatch(
                                id_0.wrapping_add((98 as size_t).wrapping_mul(n_0)),
                                l_0.wrapping_add(3 as size_t),
                                l_0,
                                matches,
                            );
                        }
                    }
                }
            } else {
                if !(is_space != 0) {
                    continue;
                }
                let is_all_caps_0: ::core::ffi::c_int = if w_0.transform as ::core::ffi::c_int
                    != BROTLI_TRANSFORM_UPPERCASE_FIRST as ::core::ffi::c_int
                {
                    BROTLI_TRUE
                } else {
                    BROTLI_FALSE
                };
                let mut s_2: *const uint8_t = ::core::ptr::null::<uint8_t>();
                if IsMatch(
                    (*dictionary).words,
                    w_0,
                    data.offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                    max_length.wrapping_sub(1 as size_t),
                ) == 0
                {
                    continue;
                }
                AddMatch(
                    id_0.wrapping_add(
                        ((if is_all_caps_0 != 0 {
                            85 as ::core::ffi::c_int
                        } else {
                            30 as ::core::ffi::c_int
                        }) as size_t)
                            .wrapping_mul(n_0),
                    ),
                    l_0.wrapping_add(1 as size_t),
                    l_0,
                    matches,
                );
                has_found_match = BROTLI_TRUE;
                if l_0.wrapping_add(2 as size_t) >= max_length {
                    continue;
                }
                s_2 = data.offset(l_0.wrapping_add(1 as size_t) as isize) as *const uint8_t;
                if *s_2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
                {
                    AddMatch(
                        id_0.wrapping_add(
                            ((if is_all_caps_0 != 0 {
                                83 as ::core::ffi::c_int
                            } else {
                                15 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n_0),
                        ),
                        l_0.wrapping_add(2 as size_t),
                        l_0,
                        matches,
                    );
                } else if *s_2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ',' as i32
                {
                    if is_all_caps_0 == 0 {
                        AddMatch(
                            id_0.wrapping_add((109 as size_t).wrapping_mul(n_0)),
                            l_0.wrapping_add(2 as size_t),
                            l_0,
                            matches,
                        );
                    }
                    if *s_2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                    {
                        AddMatch(
                            id_0.wrapping_add(
                                ((if is_all_caps_0 != 0 {
                                    111 as ::core::ffi::c_int
                                } else {
                                    65 as ::core::ffi::c_int
                                }) as size_t)
                                    .wrapping_mul(n_0),
                            ),
                            l_0.wrapping_add(3 as size_t),
                            l_0,
                            matches,
                        );
                    }
                } else if *s_2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '.' as i32
                {
                    AddMatch(
                        id_0.wrapping_add(
                            ((if is_all_caps_0 != 0 {
                                115 as ::core::ffi::c_int
                            } else {
                                96 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n_0),
                        ),
                        l_0.wrapping_add(2 as size_t),
                        l_0,
                        matches,
                    );
                    if *s_2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                    {
                        AddMatch(
                            id_0.wrapping_add(
                                ((if is_all_caps_0 != 0 {
                                    117 as ::core::ffi::c_int
                                } else {
                                    91 as ::core::ffi::c_int
                                }) as size_t)
                                    .wrapping_mul(n_0),
                            ),
                            l_0.wrapping_add(3 as size_t),
                            l_0,
                            matches,
                        );
                    }
                } else if *s_2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '=' as i32
                {
                    if *s_2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '"' as i32
                    {
                        AddMatch(
                            id_0.wrapping_add(
                                ((if is_all_caps_0 != 0 {
                                    110 as ::core::ffi::c_int
                                } else {
                                    118 as ::core::ffi::c_int
                                }) as size_t)
                                    .wrapping_mul(n_0),
                            ),
                            l_0.wrapping_add(3 as size_t),
                            l_0,
                            matches,
                        );
                    } else if *s_2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '\'' as i32
                    {
                        AddMatch(
                            id_0.wrapping_add(
                                ((if is_all_caps_0 != 0 {
                                    119 as ::core::ffi::c_int
                                } else {
                                    120 as ::core::ffi::c_int
                                }) as size_t)
                                    .wrapping_mul(n_0),
                            ),
                            l_0.wrapping_add(3 as size_t),
                            l_0,
                            matches,
                        );
                    }
                }
            }
        }
    }
    if max_length >= 6 as size_t {
        if *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
            && (*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'e' as i32
                || *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 's' as i32
                || *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ',' as i32)
            || *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xc2 as ::core::ffi::c_int
                && *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0xa0 as ::core::ffi::c_int
        {
            let mut offset_1: size_t = *(*dictionary).buckets.offset(Hash15(
                data.offset(2 as ::core::ffi::c_int as isize) as *const uint8_t,
            ) as isize) as size_t;
            let mut end_1: ::core::ffi::c_int = (offset_1 == 0) as ::core::ffi::c_int;
            while end_1 == 0 {
                let fresh2 = offset_1;
                offset_1 = offset_1.wrapping_add(1);
                let mut w_1: DictWord = *(*dictionary).dict_words.offset(fresh2 as isize);
                let l_1: size_t =
                    (w_1.len as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as size_t;
                let n_1: size_t = (1 as ::core::ffi::c_int as size_t)
                    << (*(*dictionary).words).size_bits_by_length[l_1 as usize]
                        as ::core::ffi::c_int;
                let id_1: size_t = w_1.idx as size_t;
                end_1 = (w_1.len as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
                w_1.len = l_1 as uint8_t;
                if w_1.transform as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && IsMatch(
                        (*dictionary).words,
                        w_1,
                        data.offset(2 as ::core::ffi::c_int as isize) as *const uint8_t,
                        max_length.wrapping_sub(2 as size_t),
                    ) != 0
                {
                    if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0xc2 as ::core::ffi::c_int
                    {
                        AddMatch(
                            id_1.wrapping_add((102 as size_t).wrapping_mul(n_1)),
                            l_1.wrapping_add(2 as size_t),
                            l_1,
                            matches,
                        );
                        has_found_match = BROTLI_TRUE;
                    } else if l_1.wrapping_add(2 as size_t) < max_length
                        && *data.offset(l_1.wrapping_add(2 as size_t) as isize)
                            as ::core::ffi::c_int
                            == ' ' as i32
                    {
                        let mut t: size_t = (if *data.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 'e' as i32
                        {
                            18 as ::core::ffi::c_int
                        } else if *data.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 's' as i32
                        {
                            7 as ::core::ffi::c_int
                        } else {
                            13 as ::core::ffi::c_int
                        }) as size_t;
                        AddMatch(
                            id_1.wrapping_add(t.wrapping_mul(n_1)),
                            l_1.wrapping_add(3 as size_t),
                            l_1,
                            matches,
                        );
                        has_found_match = BROTLI_TRUE;
                    }
                }
            }
        }
    }
    if max_length >= 9 as size_t {
        if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
            && *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 't' as i32
            && *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'h' as i32
            && *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'e' as i32
            && *data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
            || *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
                && *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'c' as i32
                && *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'o' as i32
                && *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'm' as i32
                && *data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '/' as i32
        {
            let mut offset_2: size_t = *(*dictionary).buckets.offset(Hash15(
                data.offset(5 as ::core::ffi::c_int as isize) as *const uint8_t,
            ) as isize) as size_t;
            let mut end_2: ::core::ffi::c_int = (offset_2 == 0) as ::core::ffi::c_int;
            while end_2 == 0 {
                let fresh3 = offset_2;
                offset_2 = offset_2.wrapping_add(1);
                let mut w_2: DictWord = *(*dictionary).dict_words.offset(fresh3 as isize);
                let l_2: size_t =
                    (w_2.len as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as size_t;
                let n_2: size_t = (1 as ::core::ffi::c_int as size_t)
                    << (*(*dictionary).words).size_bits_by_length[l_2 as usize]
                        as ::core::ffi::c_int;
                let id_2: size_t = w_2.idx as size_t;
                end_2 = (w_2.len as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
                w_2.len = l_2 as uint8_t;
                if w_2.transform as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && IsMatch(
                        (*dictionary).words,
                        w_2,
                        data.offset(5 as ::core::ffi::c_int as isize) as *const uint8_t,
                        max_length.wrapping_sub(5 as size_t),
                    ) != 0
                {
                    AddMatch(
                        id_2.wrapping_add(
                            ((if *data.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ' ' as i32
                            {
                                41 as ::core::ffi::c_int
                            } else {
                                72 as ::core::ffi::c_int
                            }) as size_t)
                                .wrapping_mul(n_2),
                        ),
                        l_2.wrapping_add(5 as size_t),
                        l_2,
                        matches,
                    );
                    has_found_match = BROTLI_TRUE;
                    if l_2.wrapping_add(5 as size_t) < max_length {
                        let mut s_3: *const uint8_t =
                            data.offset(l_2.wrapping_add(5 as size_t) as isize) as *const uint8_t;
                        if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ' ' as i32
                        {
                            if l_2.wrapping_add(8 as size_t) < max_length
                                && *s_3.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == ' ' as i32
                                && *s_3.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 'o' as i32
                                && *s_3.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 'f' as i32
                                && *s_3.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == ' ' as i32
                            {
                                AddMatch(
                                    id_2.wrapping_add((62 as size_t).wrapping_mul(n_2)),
                                    l_2.wrapping_add(9 as size_t),
                                    l_2,
                                    matches,
                                );
                                if l_2.wrapping_add(12 as size_t) < max_length
                                    && *s_3.offset(4 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 't' as i32
                                    && *s_3.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 'h' as i32
                                    && *s_3.offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 'e' as i32
                                    && *s_3.offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == ' ' as i32
                                {
                                    AddMatch(
                                        id_2.wrapping_add((73 as size_t).wrapping_mul(n_2)),
                                        l_2.wrapping_add(13 as size_t),
                                        l_2,
                                        matches,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return has_found_match;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliFindAllStaticDictionaryMatches(
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    mut min_length: size_t,
    mut max_length: size_t,
    mut matches: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut has_found_match: ::core::ffi::c_int =
        BrotliFindAllStaticDictionaryMatchesFor(dictionary, data, min_length, max_length, matches);
    if !(*dictionary).parent.is_null()
        && (*(*dictionary).parent).num_dictionaries as ::core::ffi::c_int > 1 as ::core::ffi::c_int
    {
        let mut matches2: [uint32_t; 38] = [0; 38];
        let mut l: ::core::ffi::c_int = 0;
        let mut dictionary2: *const BrotliEncoderDictionary =
            (*(*dictionary).parent).dict[0 as ::core::ffi::c_int as usize];
        if dictionary2 == dictionary {
            dictionary2 = (*(*dictionary).parent).dict[1 as ::core::ffi::c_int as usize];
        }
        l = 0 as ::core::ffi::c_int;
        while l < BROTLI_MAX_STATIC_DICTIONARY_MATCH_LEN + 1 as ::core::ffi::c_int {
            matches2[l as usize] = kInvalidMatch;
            l += 1;
        }
        has_found_match |= BrotliFindAllStaticDictionaryMatchesFor(
            dictionary2,
            data,
            min_length,
            max_length,
            &raw mut matches2 as *mut uint32_t,
        );
        l = 0 as ::core::ffi::c_int;
        while l < BROTLI_MAX_STATIC_DICTIONARY_MATCH_LEN + 1 as ::core::ffi::c_int {
            if matches2[l as usize] != kInvalidMatch {
                let mut dist: uint32_t = matches2[l as usize] >> 5 as ::core::ffi::c_int;
                let mut len_code: uint32_t = matches2[l as usize] & 31 as uint32_t;
                let mut skipdist: uint32_t = (((1 as ::core::ffi::c_int)
                    << (*(*dictionary).words).size_bits_by_length[len_code as usize]
                        as ::core::ffi::c_int)
                    as uint32_t
                    & !(1 as uint32_t))
                    .wrapping_mul((*dictionary).num_transforms);
                dist = (dist as ::core::ffi::c_uint).wrapping_add(skipdist as ::core::ffi::c_uint)
                    as uint32_t as uint32_t;
                AddMatch(dist as size_t, l as size_t, len_code as size_t, matches);
            }
            l += 1;
        }
    }
    return has_found_match;
}
static mut kHashMul32: uint32_t = 0x1e35a7bd as uint32_t;
#[inline(always)]
unsafe extern "C" fn Hash15(mut data: *const uint8_t) -> uint32_t {
    let mut h: uint32_t =
        BrotliUnalignedRead32(data as *const ::core::ffi::c_void).wrapping_mul(kHashMul32);
    return h >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn FindMatchLengthWithLimit(
    mut s1: *const uint8_t,
    mut s2: *const uint8_t,
    mut limit: size_t,
) -> size_t {
    let mut s1_orig: *const uint8_t = s1;
    while limit >= 8 as size_t {
        let mut x: uint64_t = BrotliUnalignedRead64(s2 as *const ::core::ffi::c_void)
            ^ BrotliUnalignedRead64(s1 as *const ::core::ffi::c_void);
        s2 = s2.offset(8 as ::core::ffi::c_int as isize);
        if x != 0 as uint64_t {
            let mut matching_bits: size_t =
                (x as ::core::ffi::c_ulonglong).trailing_zeros() as i32 as size_t;
            return (s1.offset_from(s1_orig) as ::core::ffi::c_long as size_t)
                .wrapping_add(matching_bits >> 3 as ::core::ffi::c_int);
        }
        s1 = s1.offset(8 as ::core::ffi::c_int as isize);
        limit = (limit as ::core::ffi::c_ulong).wrapping_sub(8 as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
    while limit != 0 && *s1 as ::core::ffi::c_int == *s2 as ::core::ffi::c_int {
        limit = limit.wrapping_sub(1);
        s2 = s2.offset(1);
        s1 = s1.offset(1);
    }
    return s1.offset_from(s1_orig) as ::core::ffi::c_long as size_t;
}
