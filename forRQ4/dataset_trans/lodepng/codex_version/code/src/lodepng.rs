extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type LodePNGColorType = ::core::ffi::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGState {
    pub decoder: LodePNGDecoderSettings,
    pub encoder: LodePNGEncoderSettings,
    pub info_raw: LodePNGColorMode,
    pub info_png: LodePNGInfo,
    pub error: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGInfo {
    pub compression_method: ::core::ffi::c_uint,
    pub filter_method: ::core::ffi::c_uint,
    pub interlace_method: ::core::ffi::c_uint,
    pub color: LodePNGColorMode,
    pub background_defined: ::core::ffi::c_uint,
    pub background_r: ::core::ffi::c_uint,
    pub background_g: ::core::ffi::c_uint,
    pub background_b: ::core::ffi::c_uint,
    pub text_num: size_t,
    pub text_keys: *mut *mut ::core::ffi::c_char,
    pub text_strings: *mut *mut ::core::ffi::c_char,
    pub itext_num: size_t,
    pub itext_keys: *mut *mut ::core::ffi::c_char,
    pub itext_langtags: *mut *mut ::core::ffi::c_char,
    pub itext_transkeys: *mut *mut ::core::ffi::c_char,
    pub itext_strings: *mut *mut ::core::ffi::c_char,
    pub exif_defined: ::core::ffi::c_uint,
    pub exif: *mut ::core::ffi::c_uchar,
    pub exif_size: ::core::ffi::c_uint,
    pub time_defined: ::core::ffi::c_uint,
    pub time: LodePNGTime,
    pub phys_defined: ::core::ffi::c_uint,
    pub phys_x: ::core::ffi::c_uint,
    pub phys_y: ::core::ffi::c_uint,
    pub phys_unit: ::core::ffi::c_uint,
    pub gama_defined: ::core::ffi::c_uint,
    pub gama_gamma: ::core::ffi::c_uint,
    pub chrm_defined: ::core::ffi::c_uint,
    pub chrm_white_x: ::core::ffi::c_uint,
    pub chrm_white_y: ::core::ffi::c_uint,
    pub chrm_red_x: ::core::ffi::c_uint,
    pub chrm_red_y: ::core::ffi::c_uint,
    pub chrm_green_x: ::core::ffi::c_uint,
    pub chrm_green_y: ::core::ffi::c_uint,
    pub chrm_blue_x: ::core::ffi::c_uint,
    pub chrm_blue_y: ::core::ffi::c_uint,
    pub srgb_defined: ::core::ffi::c_uint,
    pub srgb_intent: ::core::ffi::c_uint,
    pub iccp_defined: ::core::ffi::c_uint,
    pub iccp_name: *mut ::core::ffi::c_char,
    pub iccp_profile: *mut ::core::ffi::c_uchar,
    pub iccp_profile_size: ::core::ffi::c_uint,
    pub cicp_defined: ::core::ffi::c_uint,
    pub cicp_color_primaries: ::core::ffi::c_uint,
    pub cicp_transfer_function: ::core::ffi::c_uint,
    pub cicp_matrix_coefficients: ::core::ffi::c_uint,
    pub cicp_video_full_range_flag: ::core::ffi::c_uint,
    pub mdcv_defined: ::core::ffi::c_uint,
    pub mdcv_red_x: ::core::ffi::c_uint,
    pub mdcv_red_y: ::core::ffi::c_uint,
    pub mdcv_green_x: ::core::ffi::c_uint,
    pub mdcv_green_y: ::core::ffi::c_uint,
    pub mdcv_blue_x: ::core::ffi::c_uint,
    pub mdcv_blue_y: ::core::ffi::c_uint,
    pub mdcv_white_x: ::core::ffi::c_uint,
    pub mdcv_white_y: ::core::ffi::c_uint,
    pub mdcv_max_luminance: ::core::ffi::c_uint,
    pub mdcv_min_luminance: ::core::ffi::c_uint,
    pub clli_defined: ::core::ffi::c_uint,
    pub clli_max_cll: ::core::ffi::c_uint,
    pub clli_max_fall: ::core::ffi::c_uint,
    pub sbit_defined: ::core::ffi::c_uint,
    pub sbit_r: ::core::ffi::c_uint,
    pub sbit_g: ::core::ffi::c_uint,
    pub sbit_b: ::core::ffi::c_uint,
    pub sbit_a: ::core::ffi::c_uint,
    pub unknown_chunks_data: [*mut ::core::ffi::c_uchar; 3],
    pub unknown_chunks_size: [size_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGTime {
    pub year: ::core::ffi::c_uint,
    pub month: ::core::ffi::c_uint,
    pub day: ::core::ffi::c_uint,
    pub hour: ::core::ffi::c_uint,
    pub minute: ::core::ffi::c_uint,
    pub second: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGColorMode {
    pub colortype: LodePNGColorType,
    pub bitdepth: ::core::ffi::c_uint,
    pub palette: *mut ::core::ffi::c_uchar,
    pub palettesize: size_t,
    pub key_defined: ::core::ffi::c_uint,
    pub key_r: ::core::ffi::c_uint,
    pub key_g: ::core::ffi::c_uint,
    pub key_b: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGEncoderSettings {
    pub zlibsettings: LodePNGCompressSettings,
    pub auto_convert: ::core::ffi::c_uint,
    pub filter_palette_zero: ::core::ffi::c_uint,
    pub filter_strategy: LodePNGFilterStrategy,
    pub predefined_filters: *const ::core::ffi::c_uchar,
    pub force_palette: ::core::ffi::c_uint,
    pub add_id: ::core::ffi::c_uint,
    pub text_compression: ::core::ffi::c_uint,
}
pub type LodePNGFilterStrategy = ::core::ffi::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGCompressSettings {
    pub btype: ::core::ffi::c_uint,
    pub use_lz77: ::core::ffi::c_uint,
    pub windowsize: ::core::ffi::c_uint,
    pub minmatch: ::core::ffi::c_uint,
    pub nicematch: ::core::ffi::c_uint,
    pub lazymatching: ::core::ffi::c_uint,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_deflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGCompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_context: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecoderSettings {
    pub zlibsettings: LodePNGDecompressSettings,
    pub ignore_crc: ::core::ffi::c_uint,
    pub ignore_critical: ::core::ffi::c_uint,
    pub ignore_end: ::core::ffi::c_uint,
    pub color_convert: ::core::ffi::c_uint,
    pub read_text_chunks: ::core::ffi::c_uint,
    pub remember_unknown_chunks: ::core::ffi::c_uint,
    pub max_text_size: size_t,
    pub max_icc_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecompressSettings {
    pub ignore_adler32: ::core::ffi::c_uint,
    pub ignore_nlen: ::core::ffi::c_uint,
    pub max_output_size: size_t,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_inflate: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_uchar,
            *mut size_t,
            *const ::core::ffi::c_uchar,
            size_t,
            *const LodePNGDecompressSettings,
        ) -> ::core::ffi::c_uint,
    >,
    pub custom_context: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColorTree {
    pub children: [*mut ColorTree; 16],
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucvector {
    pub data: *mut ::core::ffi::c_uchar,
    pub size: size_t,
    pub allocsize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGBitReader {
    pub data: *const ::core::ffi::c_uchar,
    pub size: size_t,
    pub bitsize: size_t,
    pub bp: size_t,
    pub buffer: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub codes: *mut ::core::ffi::c_uint,
    pub lengths: *mut ::core::ffi::c_uint,
    pub maxbitlen: ::core::ffi::c_uint,
    pub numcodes: ::core::ffi::c_uint,
    pub table_len: *mut ::core::ffi::c_uchar,
    pub table_value: *mut ::core::ffi::c_ushort,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub head: *mut ::core::ffi::c_int,
    pub chain: *mut ::core::ffi::c_ushort,
    pub val: *mut ::core::ffi::c_int,
    pub headz: *mut ::core::ffi::c_int,
    pub chainz: *mut ::core::ffi::c_ushort,
    pub zeros: *mut ::core::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGBitWriter {
    pub data: *mut ucvector,
    pub bp: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uivector {
    pub data: *mut ::core::ffi::c_uint,
    pub size: size_t,
    pub allocsize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BPMNode {
    pub weight: ::core::ffi::c_int,
    pub index: ::core::ffi::c_uint,
    pub tail: *mut BPMNode,
    pub in_use: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BPMLists {
    pub memsize: ::core::ffi::c_uint,
    pub memory: *mut BPMNode,
    pub numfree: ::core::ffi::c_uint,
    pub nextfree: ::core::ffi::c_uint,
    pub freelist: *mut *mut BPMNode,
    pub listsize: ::core::ffi::c_uint,
    pub chains0: *mut *mut BPMNode,
    pub chains1: *mut *mut BPMNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGColorStats {
    pub colored: ::core::ffi::c_uint,
    pub key: ::core::ffi::c_uint,
    pub key_r: ::core::ffi::c_ushort,
    pub key_g: ::core::ffi::c_ushort,
    pub key_b: ::core::ffi::c_ushort,
    pub alpha: ::core::ffi::c_uint,
    pub numcolors: ::core::ffi::c_uint,
    pub palette: [::core::ffi::c_uchar; 1024],
    pub bits: ::core::ffi::c_uint,
    pub numpixels: size_t,
    pub allow_palette: ::core::ffi::c_uint,
    pub allow_greyscale: ::core::ffi::c_uint,
}
static mut mask: ::core::ffi::c_uint = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub static mut LODEPNG_VERSION_STRING: *const ::core::ffi::c_char =
    b"20260119\0" as *const u8 as *const ::core::ffi::c_char;
unsafe extern "C" fn lodepng_malloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    return malloc(size);
}
unsafe extern "C" fn lodepng_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut new_size: size_t,
) -> *mut ::core::ffi::c_void {
    return realloc(ptr, new_size);
}
unsafe extern "C" fn lodepng_free(mut ptr: *mut ::core::ffi::c_void) {
    free(ptr);
}
#[inline(always)]
unsafe extern "C" fn lodepng_memcpy(
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
    mut size: size_t,
) {
    if size != 0 {
        ::core::ptr::copy_nonoverlapping(
            src as *const ::core::ffi::c_uchar,
            dst as *mut ::core::ffi::c_uchar,
            size,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn lodepng_memset(
    mut dst: *mut ::core::ffi::c_void,
    mut value: ::core::ffi::c_int,
    mut num: size_t,
) {
    if num != 0 {
        ::core::ptr::write_bytes(
            dst as *mut ::core::ffi::c_uchar,
            value as ::core::ffi::c_uchar,
            num,
        );
    }
}
unsafe extern "C" fn lodepng_strlen(mut a: *const ::core::ffi::c_char) -> size_t {
    let mut orig: *const ::core::ffi::c_char = a;
    Some(lodepng_strlen as unsafe extern "C" fn(*const ::core::ffi::c_char) -> size_t);
    while *a != 0 {
        a = a.offset(1);
    }
    return a.offset_from(orig) as ::core::ffi::c_long as size_t;
}
unsafe extern "C" fn lodepng_addofl(
    mut a: size_t,
    mut b: size_t,
    mut result: *mut size_t,
) -> ::core::ffi::c_int {
    *result = a.wrapping_add(b);
    return (*result < a) as ::core::ffi::c_int;
}
unsafe extern "C" fn lodepng_mulofl(
    mut a: size_t,
    mut b: size_t,
    mut result: *mut size_t,
) -> ::core::ffi::c_int {
    *result = a.wrapping_mul(b);
    return (a != 0 as size_t && (*result).wrapping_div(a) != b) as ::core::ffi::c_int;
}
unsafe extern "C" fn lodepng_gtofl(
    mut a: size_t,
    mut b: size_t,
    mut c: size_t,
) -> ::core::ffi::c_int {
    let mut d: size_t = 0;
    if lodepng_addofl(a, b, &raw mut d) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    return (d > c) as ::core::ffi::c_int;
}
unsafe extern "C" fn uivector_cleanup(mut p: *mut ::core::ffi::c_void) {
    let ref mut fresh73 = (*(p as *mut uivector)).allocsize;
    *fresh73 = 0 as size_t;
    (*(p as *mut uivector)).size = *fresh73;
    lodepng_free((*(p as *mut uivector)).data as *mut ::core::ffi::c_void);
    let ref mut fresh74 = (*(p as *mut uivector)).data;
    *fresh74 = ::core::ptr::null_mut::<::core::ffi::c_uint>();
}
unsafe extern "C" fn uivector_resize(
    mut p: *mut uivector,
    mut size: size_t,
) -> ::core::ffi::c_uint {
    let mut allocsize: size_t =
        size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t);
    if allocsize > (*p).allocsize {
        let mut newsize: size_t =
            allocsize.wrapping_add((*p).allocsize >> 1 as ::core::ffi::c_uint);
        let mut data: *mut ::core::ffi::c_void =
            lodepng_realloc((*p).data as *mut ::core::ffi::c_void, newsize);
        if !data.is_null() {
            (*p).allocsize = newsize;
            (*p).data = data as *mut ::core::ffi::c_uint;
        } else {
            return 0 as ::core::ffi::c_uint;
        }
    }
    (*p).size = size;
    return 1 as ::core::ffi::c_uint;
}
unsafe extern "C" fn uivector_init(mut p: *mut uivector) {
    (*p).data = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    (*p).allocsize = 0 as size_t;
    (*p).size = (*p).allocsize;
}
#[inline(always)]
unsafe extern "C" fn uivector_reserve(
    mut p: *mut uivector,
    mut size: size_t,
) -> ::core::ffi::c_uint {
    let mut allocsize: size_t =
        size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t);
    if allocsize > (*p).allocsize {
        let mut newsize: size_t =
            allocsize.wrapping_add((*p).allocsize >> 1 as ::core::ffi::c_uint);
        let mut data: *mut ::core::ffi::c_void =
            lodepng_realloc((*p).data as *mut ::core::ffi::c_void, newsize);
        if !data.is_null() {
            (*p).allocsize = newsize;
            (*p).data = data as *mut ::core::ffi::c_uint;
        } else {
            return 0 as ::core::ffi::c_uint;
        }
    }
    return 1 as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn uivector_push_back(
    mut p: *mut uivector,
    mut c: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut pos: size_t = (*p).size;
    let mut needed_size: size_t = pos.wrapping_add(1 as size_t);
    let mut needed_bytes: size_t =
        needed_size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t);
    if needed_bytes > (*p).allocsize && uivector_reserve(p, needed_size) == 0
    {
        return 0 as ::core::ffi::c_uint;
    }
    *(*p).data.offset(pos as isize) = c;
    (*p).size = needed_size;
    return 1 as ::core::ffi::c_uint;
}
unsafe extern "C" fn ucvector_reserve(
    mut p: *mut ucvector,
    mut size: size_t,
) -> ::core::ffi::c_uint {
    if size > (*p).allocsize {
        let mut newsize: size_t = size.wrapping_add((*p).allocsize >> 1 as ::core::ffi::c_uint);
        let mut data: *mut ::core::ffi::c_void =
            lodepng_realloc((*p).data as *mut ::core::ffi::c_void, newsize);
        if !data.is_null() {
            (*p).allocsize = newsize;
            (*p).data = data as *mut ::core::ffi::c_uchar;
        } else {
            return 0 as ::core::ffi::c_uint;
        }
    }
    return 1 as ::core::ffi::c_uint;
}
unsafe extern "C" fn ucvector_resize(
    mut p: *mut ucvector,
    mut size: size_t,
) -> ::core::ffi::c_uint {
    (*p).size = size;
    return ucvector_reserve(p, size);
}
unsafe extern "C" fn ucvector_init(
    mut buffer: *mut ::core::ffi::c_uchar,
    mut size: size_t,
) -> ucvector {
    let mut v: ucvector = ucvector {
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        size: 0,
        allocsize: 0,
    };
    v.data = buffer;
    v.size = size;
    v.allocsize = v.size;
    return v;
}
unsafe extern "C" fn alloc_string_sized(
    mut in_0: *const ::core::ffi::c_char,
    mut insize: size_t,
) -> *mut ::core::ffi::c_char {
    let mut out: *mut ::core::ffi::c_char =
        lodepng_malloc(insize.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if !out.is_null() {
        lodepng_memcpy(
            out as *mut ::core::ffi::c_void,
            in_0 as *const ::core::ffi::c_void,
            insize,
        );
        *out.offset(insize as isize) = 0 as ::core::ffi::c_char;
    }
    return out;
}
unsafe extern "C" fn alloc_string(
    mut in_0: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    return alloc_string_sized(in_0, lodepng_strlen(in_0));
}
unsafe extern "C" fn lodepng_read32bitInt(
    mut buffer: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    return (*buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        << 24 as ::core::ffi::c_uint
        | (*buffer.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            << 16 as ::core::ffi::c_uint
        | (*buffer.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_uint
        | *buffer.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
}
unsafe extern "C" fn lodepng_set32bitInt(
    mut buffer: *mut ::core::ffi::c_uchar,
    mut value: ::core::ffi::c_uint,
) {
    *buffer.offset(0 as ::core::ffi::c_int as isize) =
        (value >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *buffer.offset(1 as ::core::ffi::c_int as isize) =
        (value >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *buffer.offset(2 as ::core::ffi::c_int as isize) =
        (value >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *buffer.offset(3 as ::core::ffi::c_int as isize) =
        (value & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
}
unsafe extern "C" fn lodepng_filesize(mut file: *mut FILE) -> ::core::ffi::c_long {
    let mut size: ::core::ffi::c_long = 0;
    if fseek(file, 0 as ::core::ffi::c_long, SEEK_END) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    size = ftell(file);
    if size == LONG_MAX {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    if fseek(file, 0 as ::core::ffi::c_long, SEEK_SET) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    return size;
}
unsafe extern "C" fn lodepng_load_file_(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut file: *mut FILE,
) -> ::core::ffi::c_uint {
    let mut size: ::core::ffi::c_long = lodepng_filesize(file);
    if size < 0 as ::core::ffi::c_long {
        return 78 as ::core::ffi::c_uint;
    }
    *outsize = size as size_t;
    *out = lodepng_malloc(size as size_t) as *mut ::core::ffi::c_uchar;
    if (*out).is_null() && size > 0 as ::core::ffi::c_long {
        return 83 as ::core::ffi::c_uint;
    }
    if fread(
        *out as *mut ::core::ffi::c_void,
        1 as size_t,
        *outsize,
        file,
    ) as size_t
        != *outsize
    {
        return 78 as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_load_file(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0;
    let mut file: *mut FILE = fopen(filename, b"rb\0" as *const u8 as *const ::core::ffi::c_char);
    if file.is_null() {
        return 78 as ::core::ffi::c_uint;
    }
    error = lodepng_load_file_(out, outsize, file);
    fclose(file);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_save_file(
    mut buffer: *const ::core::ffi::c_uchar,
    mut buffersize: size_t,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut file: *mut FILE = fopen(filename, b"wb\0" as *const u8 as *const ::core::ffi::c_char);
    if file.is_null() {
        return 79 as ::core::ffi::c_uint;
    }
    fwrite(
        buffer as *const ::core::ffi::c_void,
        1 as size_t,
        buffersize,
        file,
    );
    fclose(file);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn LodePNGBitWriter_init(
    mut writer: *mut LodePNGBitWriter,
    mut data: *mut ucvector,
) {
    (*writer).data = data;
    (*writer).bp = 0 as ::core::ffi::c_uchar;
}
unsafe extern "C" fn writeBits(
    mut writer: *mut LodePNGBitWriter,
    mut value: ::core::ffi::c_uint,
    mut nbits: size_t,
) {
    if nbits == 1 as size_t {
        if (*writer).bp as ::core::ffi::c_uint & 7 as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            if ucvector_resize(
                (*writer).data,
                (*(*writer).data).size.wrapping_add(1 as size_t),
            ) == 0
            {
                return;
            }
            *(*(*writer).data)
                .data
                .offset((*(*writer).data).size.wrapping_sub(1 as size_t) as isize) =
                0 as ::core::ffi::c_uchar;
        }
        let ref mut fresh76 = *(*(*writer).data)
            .data
            .offset((*(*writer).data).size.wrapping_sub(1 as size_t) as isize);
        *fresh76 = (*fresh76 as ::core::ffi::c_uint
            | value << ((*writer).bp as ::core::ffi::c_uint & 7 as ::core::ffi::c_uint))
            as ::core::ffi::c_uchar;
        (*writer).bp = (*writer).bp.wrapping_add(1);
    } else {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i != nbits {
            if (*writer).bp as ::core::ffi::c_uint & 7 as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            {
                if ucvector_resize(
                    (*writer).data,
                    (*(*writer).data).size.wrapping_add(1 as size_t),
                ) == 0
                {
                    return;
                }
                *(*(*writer).data)
                    .data
                    .offset((*(*writer).data).size.wrapping_sub(1 as size_t) as isize) =
                    0 as ::core::ffi::c_uchar;
            }
            let ref mut fresh77 = *(*(*writer).data)
                .data
                .offset((*(*writer).data).size.wrapping_sub(1 as size_t) as isize);
            *fresh77 = (*fresh77 as ::core::ffi::c_int
                | ((value >> i & 1 as ::core::ffi::c_uint) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int)
                    << ((*writer).bp as ::core::ffi::c_uint & 7 as ::core::ffi::c_uint))
                as ::core::ffi::c_uchar;
            (*writer).bp = (*writer).bp.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn writeBitsReversed(
    mut writer: *mut LodePNGBitWriter,
    mut value: ::core::ffi::c_uint,
    mut nbits: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i != nbits {
        if (*writer).bp as ::core::ffi::c_uint & 7 as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            if ucvector_resize(
                (*writer).data,
                (*(*writer).data).size.wrapping_add(1 as size_t),
            ) == 0
            {
                return;
            }
            *(*(*writer).data)
                .data
                .offset((*(*writer).data).size.wrapping_sub(1 as size_t) as isize) =
                0 as ::core::ffi::c_uchar;
        }
        let ref mut fresh75 = *(*(*writer).data)
            .data
            .offset((*(*writer).data).size.wrapping_sub(1 as size_t) as isize);
        *fresh75 = (*fresh75 as ::core::ffi::c_int
            | ((value >> nbits.wrapping_sub(1 as size_t).wrapping_sub(i) & 1 as ::core::ffi::c_uint)
                as ::core::ffi::c_uchar as ::core::ffi::c_int)
                << ((*writer).bp as ::core::ffi::c_uint & 7 as ::core::ffi::c_uint))
            as ::core::ffi::c_uchar;
        (*writer).bp = (*writer).bp.wrapping_add(1);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn LodePNGBitReader_init(
    mut reader: *mut LodePNGBitReader,
    mut data: *const ::core::ffi::c_uchar,
    mut size: size_t,
) -> ::core::ffi::c_uint {
    let mut temp: size_t = 0;
    (*reader).data = data;
    (*reader).size = size;
    if lodepng_mulofl(size, 8 as size_t, &raw mut (*reader).bitsize) != 0 {
        return 105 as ::core::ffi::c_uint;
    }
    if lodepng_addofl((*reader).bitsize, 64 as size_t, &raw mut temp) != 0 {
        return 105 as ::core::ffi::c_uint;
    }
    (*reader).bp = 0 as size_t;
    (*reader).buffer = 0 as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn ensureBits9(mut reader: *mut LodePNGBitReader, mut nbits: size_t) {
    let mut start: size_t = (*reader).bp >> 3 as ::core::ffi::c_uint;
    let mut size: size_t = (*reader).size;
    if start.wrapping_add(1 as size_t) < size {
        (*reader).buffer = *(*reader)
            .data
            .offset(start.wrapping_add(0 as size_t) as isize)
            as ::core::ffi::c_uint
            | (*(*reader)
                .data
                .offset(start.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_uint;
        (*reader).buffer >>= (*reader).bp & 7 as size_t;
    } else {
        (*reader).buffer = 0 as ::core::ffi::c_uint;
        if start.wrapping_add(0 as size_t) < size {
            (*reader).buffer = *(*reader)
                .data
                .offset(start.wrapping_add(0 as size_t) as isize)
                as ::core::ffi::c_uint;
        }
        (*reader).buffer >>= (*reader).bp & 7 as size_t;
    };
}
#[inline]
unsafe extern "C" fn ensureBits17(mut reader: *mut LodePNGBitReader, mut nbits: size_t) {
    let mut start: size_t = (*reader).bp >> 3 as ::core::ffi::c_uint;
    let mut size: size_t = (*reader).size;
    if start.wrapping_add(2 as size_t) < size {
        (*reader).buffer = *(*reader)
            .data
            .offset(start.wrapping_add(0 as size_t) as isize)
            as ::core::ffi::c_uint
            | (*(*reader)
                .data
                .offset(start.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_uint
            | (*(*reader)
                .data
                .offset(start.wrapping_add(2 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 16 as ::core::ffi::c_uint;
        (*reader).buffer >>= (*reader).bp & 7 as size_t;
    } else {
        (*reader).buffer = 0 as ::core::ffi::c_uint;
        if start.wrapping_add(0 as size_t) < size {
            (*reader).buffer |= *(*reader)
                .data
                .offset(start.wrapping_add(0 as size_t) as isize)
                as ::core::ffi::c_uint;
        }
        if start.wrapping_add(1 as size_t) < size {
            (*reader).buffer |= (*(*reader)
                .data
                .offset(start.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_uint;
        }
        (*reader).buffer >>= (*reader).bp & 7 as size_t;
    };
}
#[inline]
unsafe extern "C" fn ensureBits25(mut reader: *mut LodePNGBitReader, mut nbits: size_t) {
    let mut start: size_t = (*reader).bp >> 3 as ::core::ffi::c_uint;
    let mut size: size_t = (*reader).size;
    if start.wrapping_add(3 as size_t) < size {
        (*reader).buffer = *(*reader)
            .data
            .offset(start.wrapping_add(0 as size_t) as isize)
            as ::core::ffi::c_uint
            | (*(*reader)
                .data
                .offset(start.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_uint
            | (*(*reader)
                .data
                .offset(start.wrapping_add(2 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 16 as ::core::ffi::c_uint
            | (*(*reader)
                .data
                .offset(start.wrapping_add(3 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 24 as ::core::ffi::c_uint;
        (*reader).buffer >>= (*reader).bp & 7 as size_t;
    } else {
        (*reader).buffer = 0 as ::core::ffi::c_uint;
        if start.wrapping_add(0 as size_t) < size {
            (*reader).buffer |= *(*reader)
                .data
                .offset(start.wrapping_add(0 as size_t) as isize)
                as ::core::ffi::c_uint;
        }
        if start.wrapping_add(1 as size_t) < size {
            (*reader).buffer |= (*(*reader)
                .data
                .offset(start.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_uint;
        }
        if start.wrapping_add(2 as size_t) < size {
            (*reader).buffer |= (*(*reader)
                .data
                .offset(start.wrapping_add(2 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 16 as ::core::ffi::c_uint;
        }
        (*reader).buffer >>= (*reader).bp & 7 as size_t;
    };
}
#[inline]
unsafe extern "C" fn ensureBits32(mut reader: *mut LodePNGBitReader, mut nbits: size_t) {
    let mut start: size_t = (*reader).bp >> 3 as ::core::ffi::c_uint;
    let mut size: size_t = (*reader).size;
    if start.wrapping_add(4 as size_t) < size {
        (*reader).buffer = *(*reader)
            .data
            .offset(start.wrapping_add(0 as size_t) as isize)
            as ::core::ffi::c_uint
            | (*(*reader)
                .data
                .offset(start.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_uint
            | (*(*reader)
                .data
                .offset(start.wrapping_add(2 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 16 as ::core::ffi::c_uint
            | (*(*reader)
                .data
                .offset(start.wrapping_add(3 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 24 as ::core::ffi::c_uint;
        (*reader).buffer >>= (*reader).bp & 7 as size_t;
        (*reader).buffer |= ((*(*reader)
            .data
            .offset(start.wrapping_add(4 as size_t) as isize)
            as ::core::ffi::c_uint)
            << 24 as ::core::ffi::c_uint)
            << (8 as size_t).wrapping_sub((*reader).bp & 7 as size_t);
    } else {
        (*reader).buffer = 0 as ::core::ffi::c_uint;
        if start.wrapping_add(0 as size_t) < size {
            (*reader).buffer |= *(*reader)
                .data
                .offset(start.wrapping_add(0 as size_t) as isize)
                as ::core::ffi::c_uint;
        }
        if start.wrapping_add(1 as size_t) < size {
            (*reader).buffer |= (*(*reader)
                .data
                .offset(start.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_uint;
        }
        if start.wrapping_add(2 as size_t) < size {
            (*reader).buffer |= (*(*reader)
                .data
                .offset(start.wrapping_add(2 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 16 as ::core::ffi::c_uint;
        }
        if start.wrapping_add(3 as size_t) < size {
            (*reader).buffer |= (*(*reader)
                .data
                .offset(start.wrapping_add(3 as size_t) as isize)
                as ::core::ffi::c_uint)
                << 24 as ::core::ffi::c_uint;
        }
        (*reader).buffer >>= (*reader).bp & 7 as size_t;
    };
}
#[inline]
unsafe extern "C" fn peekBits(
    mut reader: *mut LodePNGBitReader,
    mut nbits: size_t,
) -> ::core::ffi::c_uint {
    return (*reader).buffer
        & ((1 as ::core::ffi::c_uint) << nbits).wrapping_sub(1 as ::core::ffi::c_uint);
}
#[inline]
unsafe extern "C" fn advanceBits(mut reader: *mut LodePNGBitReader, mut nbits: size_t) {
    (*reader).buffer >>= nbits;
    (*reader).bp = ((*reader).bp as ::core::ffi::c_ulong)
        .wrapping_add(nbits as ::core::ffi::c_ulong) as size_t as size_t;
}
#[inline]
unsafe extern "C" fn readBits(
    mut reader: *mut LodePNGBitReader,
    mut nbits: size_t,
) -> ::core::ffi::c_uint {
    let mut result: ::core::ffi::c_uint = peekBits(reader, nbits);
    advanceBits(reader, nbits);
    return result;
}
unsafe extern "C" fn reverseBits(
    mut bits: ::core::ffi::c_uint,
    mut num: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    let mut result: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while i < num {
        result |= (bits >> num.wrapping_sub(i).wrapping_sub(1 as ::core::ffi::c_uint)
            & 1 as ::core::ffi::c_uint)
            << i;
        i = i.wrapping_add(1);
    }
    return result;
}
pub const FIRST_LENGTH_CODE_INDEX: ::core::ffi::c_int = 257 as ::core::ffi::c_int;
pub const LAST_LENGTH_CODE_INDEX: ::core::ffi::c_int = 285 as ::core::ffi::c_int;
pub const NUM_DEFLATE_CODE_SYMBOLS: ::core::ffi::c_int = 288 as ::core::ffi::c_int;
pub const NUM_DISTANCE_SYMBOLS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NUM_CODE_LENGTH_CODES: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
static mut LENGTHBASE: [::core::ffi::c_uint; 29] = [
    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    7 as ::core::ffi::c_int as ::core::ffi::c_uint,
    8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    10 as ::core::ffi::c_int as ::core::ffi::c_uint,
    11 as ::core::ffi::c_int as ::core::ffi::c_uint,
    13 as ::core::ffi::c_int as ::core::ffi::c_uint,
    15 as ::core::ffi::c_int as ::core::ffi::c_uint,
    17 as ::core::ffi::c_int as ::core::ffi::c_uint,
    19 as ::core::ffi::c_int as ::core::ffi::c_uint,
    23 as ::core::ffi::c_int as ::core::ffi::c_uint,
    27 as ::core::ffi::c_int as ::core::ffi::c_uint,
    31 as ::core::ffi::c_int as ::core::ffi::c_uint,
    35 as ::core::ffi::c_int as ::core::ffi::c_uint,
    43 as ::core::ffi::c_int as ::core::ffi::c_uint,
    51 as ::core::ffi::c_int as ::core::ffi::c_uint,
    59 as ::core::ffi::c_int as ::core::ffi::c_uint,
    67 as ::core::ffi::c_int as ::core::ffi::c_uint,
    83 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    115 as ::core::ffi::c_int as ::core::ffi::c_uint,
    131 as ::core::ffi::c_int as ::core::ffi::c_uint,
    163 as ::core::ffi::c_int as ::core::ffi::c_uint,
    195 as ::core::ffi::c_int as ::core::ffi::c_uint,
    227 as ::core::ffi::c_int as ::core::ffi::c_uint,
    258 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
static mut LENGTHEXTRA: [::core::ffi::c_uint; 29] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
static mut DISTANCEBASE: [::core::ffi::c_uint; 30] = [
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    7 as ::core::ffi::c_int as ::core::ffi::c_uint,
    9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    13 as ::core::ffi::c_int as ::core::ffi::c_uint,
    17 as ::core::ffi::c_int as ::core::ffi::c_uint,
    25 as ::core::ffi::c_int as ::core::ffi::c_uint,
    33 as ::core::ffi::c_int as ::core::ffi::c_uint,
    49 as ::core::ffi::c_int as ::core::ffi::c_uint,
    65 as ::core::ffi::c_int as ::core::ffi::c_uint,
    97 as ::core::ffi::c_int as ::core::ffi::c_uint,
    129 as ::core::ffi::c_int as ::core::ffi::c_uint,
    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
    257 as ::core::ffi::c_int as ::core::ffi::c_uint,
    385 as ::core::ffi::c_int as ::core::ffi::c_uint,
    513 as ::core::ffi::c_int as ::core::ffi::c_uint,
    769 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1025 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1537 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2049 as ::core::ffi::c_int as ::core::ffi::c_uint,
    3073 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4097 as ::core::ffi::c_int as ::core::ffi::c_uint,
    6145 as ::core::ffi::c_int as ::core::ffi::c_uint,
    8193 as ::core::ffi::c_int as ::core::ffi::c_uint,
    12289 as ::core::ffi::c_int as ::core::ffi::c_uint,
    16385 as ::core::ffi::c_int as ::core::ffi::c_uint,
    24577 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
static mut DISTANCEEXTRA: [::core::ffi::c_uint; 30] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    7 as ::core::ffi::c_int as ::core::ffi::c_uint,
    7 as ::core::ffi::c_int as ::core::ffi::c_uint,
    8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    10 as ::core::ffi::c_int as ::core::ffi::c_uint,
    10 as ::core::ffi::c_int as ::core::ffi::c_uint,
    11 as ::core::ffi::c_int as ::core::ffi::c_uint,
    11 as ::core::ffi::c_int as ::core::ffi::c_uint,
    12 as ::core::ffi::c_int as ::core::ffi::c_uint,
    12 as ::core::ffi::c_int as ::core::ffi::c_uint,
    13 as ::core::ffi::c_int as ::core::ffi::c_uint,
    13 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
static mut CLCL_ORDER: [::core::ffi::c_uint; 19] = [
    16 as ::core::ffi::c_int as ::core::ffi::c_uint,
    17 as ::core::ffi::c_int as ::core::ffi::c_uint,
    18 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    7 as ::core::ffi::c_int as ::core::ffi::c_uint,
    9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    10 as ::core::ffi::c_int as ::core::ffi::c_uint,
    5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    11 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    12 as ::core::ffi::c_int as ::core::ffi::c_uint,
    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    13 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    14 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    15 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
unsafe extern "C" fn HuffmanTree_init(mut tree: *mut HuffmanTree) {
    (*tree).codes = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    (*tree).lengths = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    (*tree).table_len = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*tree).table_value = ::core::ptr::null_mut::<::core::ffi::c_ushort>();
}
unsafe extern "C" fn HuffmanTree_cleanup(mut tree: *mut HuffmanTree) {
    lodepng_free((*tree).codes as *mut ::core::ffi::c_void);
    lodepng_free((*tree).lengths as *mut ::core::ffi::c_void);
    lodepng_free((*tree).table_len as *mut ::core::ffi::c_void);
    lodepng_free((*tree).table_value as *mut ::core::ffi::c_void);
}
pub const FIRSTBITS: ::core::ffi::c_uint = 9 as ::core::ffi::c_uint;
pub const INVALIDSYMBOL: ::core::ffi::c_uint = 65535 as ::core::ffi::c_uint;
unsafe extern "C" fn HuffmanTree_makeTable(mut tree: *mut HuffmanTree) -> ::core::ffi::c_uint {
    static mut headsize: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << FIRSTBITS;
    let mut i: size_t = 0;
    let mut numpresent: size_t = 0;
    let mut pointer: size_t = 0;
    let mut size: size_t = 0;
    let mut maxlens: *mut ::core::ffi::c_uint = lodepng_malloc(
        (headsize as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    if maxlens.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    lodepng_memset(
        maxlens as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (headsize as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    );
    i = 0 as size_t;
    while i < (*tree).numcodes as size_t {
        let mut symbol: ::core::ffi::c_uint = *(*tree).codes.offset(i as isize);
        let mut l: ::core::ffi::c_uint = *(*tree).lengths.offset(i as isize);
        let mut index: ::core::ffi::c_uint = 0;
        if !(l <= FIRSTBITS) {
            index = reverseBits(symbol >> l.wrapping_sub(FIRSTBITS), FIRSTBITS);
            *maxlens.offset(index as isize) = if *maxlens.offset(index as isize) > l {
                *maxlens.offset(index as isize)
            } else {
                l
            };
        }
        i = i.wrapping_add(1);
    }
    size = headsize as size_t;
    i = 0 as size_t;
    while i < headsize as size_t {
        let mut l_0: ::core::ffi::c_uint = *maxlens.offset(i as isize);
        if l_0 > FIRSTBITS {
            size = (size as ::core::ffi::c_ulong).wrapping_add(
                ((1 as ::core::ffi::c_int as size_t) << l_0.wrapping_sub(FIRSTBITS))
                    as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
    }
    (*tree).table_len =
        lodepng_malloc(size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as size_t))
            as *mut ::core::ffi::c_uchar;
    (*tree).table_value = lodepng_malloc(
        size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_ushort>() as size_t),
    ) as *mut ::core::ffi::c_ushort;
    if (*tree).table_len.is_null() || (*tree).table_value.is_null() {
        lodepng_free(maxlens as *mut ::core::ffi::c_void);
        return 83 as ::core::ffi::c_uint;
    }
    i = 0 as size_t;
    while i < size {
        *(*tree).table_len.offset(i as isize) = 16 as ::core::ffi::c_uchar;
        i = i.wrapping_add(1);
    }
    pointer = headsize as size_t;
    i = 0 as size_t;
    while i < headsize as size_t {
        let mut l_1: ::core::ffi::c_uint = *maxlens.offset(i as isize);
        if !(l_1 <= FIRSTBITS) {
            *(*tree).table_len.offset(i as isize) = l_1 as ::core::ffi::c_uchar;
            *(*tree).table_value.offset(i as isize) = pointer as ::core::ffi::c_ushort;
            pointer = (pointer as ::core::ffi::c_ulong).wrapping_add(
                ((1 as ::core::ffi::c_int as size_t) << l_1.wrapping_sub(FIRSTBITS))
                    as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
    }
    lodepng_free(maxlens as *mut ::core::ffi::c_void);
    numpresent = 0 as size_t;
    i = 0 as size_t;
    while i < (*tree).numcodes as size_t {
        let mut l_2: ::core::ffi::c_uint = *(*tree).lengths.offset(i as isize);
        let mut symbol_0: ::core::ffi::c_uint = 0;
        let mut reverse: ::core::ffi::c_uint = 0;
        if !(l_2 == 0 as ::core::ffi::c_uint) {
            symbol_0 = *(*tree).codes.offset(i as isize);
            reverse = reverseBits(symbol_0, l_2);
            numpresent = numpresent.wrapping_add(1);
            if l_2 <= FIRSTBITS {
                let mut num: ::core::ffi::c_uint =
                    (1 as ::core::ffi::c_uint) << FIRSTBITS.wrapping_sub(l_2);
                let mut j: ::core::ffi::c_uint = 0;
                j = 0 as ::core::ffi::c_uint;
                while j < num {
                    let mut index_0: ::core::ffi::c_uint = reverse | j << l_2;
                    if *(*tree).table_len.offset(index_0 as isize) as ::core::ffi::c_int
                        != 16 as ::core::ffi::c_int
                    {
                        return 55 as ::core::ffi::c_uint;
                    }
                    *(*tree).table_len.offset(index_0 as isize) = l_2 as ::core::ffi::c_uchar;
                    *(*tree).table_value.offset(index_0 as isize) = i as ::core::ffi::c_ushort;
                    j = j.wrapping_add(1);
                }
            } else {
                let mut index_1: ::core::ffi::c_uint = reverse & mask;
                let mut maxlen: ::core::ffi::c_uint =
                    *(*tree).table_len.offset(index_1 as isize) as ::core::ffi::c_uint;
                let mut tablelen: ::core::ffi::c_uint = maxlen.wrapping_sub(FIRSTBITS);
                let mut start: ::core::ffi::c_uint =
                    *(*tree).table_value.offset(index_1 as isize) as ::core::ffi::c_uint;
                let mut num_0: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
                    << tablelen.wrapping_sub(l_2.wrapping_sub(FIRSTBITS));
                let mut j_0: ::core::ffi::c_uint = 0;
                if maxlen < l_2 {
                    return 55 as ::core::ffi::c_uint;
                }
                j_0 = 0 as ::core::ffi::c_uint;
                while j_0 < num_0 {
                    let mut reverse2: ::core::ffi::c_uint = reverse >> FIRSTBITS;
                    let mut index2: ::core::ffi::c_uint =
                        start.wrapping_add(reverse2 | j_0 << l_2.wrapping_sub(FIRSTBITS));
                    *(*tree).table_len.offset(index2 as isize) = l_2 as ::core::ffi::c_uchar;
                    *(*tree).table_value.offset(index2 as isize) = i as ::core::ffi::c_ushort;
                    j_0 = j_0.wrapping_add(1);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if numpresent < 2 as size_t {
        i = 0 as size_t;
        while i < size {
            if *(*tree).table_len.offset(i as isize) as ::core::ffi::c_int
                == 16 as ::core::ffi::c_int
            {
                *(*tree).table_len.offset(i as isize) = (if i < headsize as size_t {
                    1 as ::core::ffi::c_uint
                } else {
                    FIRSTBITS.wrapping_add(1 as ::core::ffi::c_uint)
                }) as ::core::ffi::c_uchar;
                *(*tree).table_value.offset(i as isize) = INVALIDSYMBOL as ::core::ffi::c_ushort;
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as size_t;
        while i < size {
            if *(*tree).table_len.offset(i as isize) as ::core::ffi::c_int
                == 16 as ::core::ffi::c_int
            {
                return 55 as ::core::ffi::c_uint;
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn HuffmanTree_makeFromLengths2(
    mut tree: *mut HuffmanTree,
) -> ::core::ffi::c_uint {
    let mut blcount: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut nextcode: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    (*tree).codes = lodepng_malloc(
        ((*tree).numcodes as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    blcount = lodepng_malloc(
        ((*tree).maxbitlen.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    nextcode = lodepng_malloc(
        ((*tree).maxbitlen.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    if (*tree).codes.is_null() || blcount.is_null() || nextcode.is_null() {
        error = 83 as ::core::ffi::c_uint;
    }
    if error == 0 {
        n = 0 as ::core::ffi::c_uint;
        while n != (*tree).maxbitlen.wrapping_add(1 as ::core::ffi::c_uint) {
            let ref mut fresh40 = *nextcode.offset(n as isize);
            *fresh40 = 0 as ::core::ffi::c_uint;
            *blcount.offset(n as isize) = *fresh40;
            n = n.wrapping_add(1);
        }
        bits = 0 as ::core::ffi::c_uint;
        while bits != (*tree).numcodes {
            let ref mut fresh41 = *blcount.offset(*(*tree).lengths.offset(bits as isize) as isize);
            *fresh41 = (*fresh41).wrapping_add(1);
            bits = bits.wrapping_add(1);
        }
        bits = 1 as ::core::ffi::c_uint;
        while bits <= (*tree).maxbitlen {
            *nextcode.offset(bits as isize) = (*nextcode
                .offset(bits.wrapping_sub(1 as ::core::ffi::c_uint) as isize))
            .wrapping_add(*blcount.offset(bits.wrapping_sub(1 as ::core::ffi::c_uint) as isize))
                << 1 as ::core::ffi::c_uint;
            bits = bits.wrapping_add(1);
        }
        n = 0 as ::core::ffi::c_uint;
        while n != (*tree).numcodes {
            if *(*tree).lengths.offset(n as isize) != 0 as ::core::ffi::c_uint {
                let ref mut fresh42 =
                    *nextcode.offset(*(*tree).lengths.offset(n as isize) as isize);
                let fresh43 = *fresh42;
                *fresh42 = (*fresh42).wrapping_add(1);
                *(*tree).codes.offset(n as isize) = fresh43;
                *(*tree).codes.offset(n as isize) &= ((1 as ::core::ffi::c_uint)
                    << *(*tree).lengths.offset(n as isize))
                .wrapping_sub(1 as ::core::ffi::c_uint);
            }
            n = n.wrapping_add(1);
        }
    }
    lodepng_free(blcount as *mut ::core::ffi::c_void);
    lodepng_free(nextcode as *mut ::core::ffi::c_void);
    if error == 0 {
        error = HuffmanTree_makeTable(tree);
    }
    return error;
}
unsafe extern "C" fn HuffmanTree_makeFromLengths(
    mut tree: *mut HuffmanTree,
    mut bitlen: *const ::core::ffi::c_uint,
    mut numcodes: size_t,
    mut maxbitlen: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    (*tree).lengths = lodepng_malloc(
        numcodes.wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    if (*tree).lengths.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    i = 0 as ::core::ffi::c_uint;
    while i as size_t != numcodes {
        *(*tree).lengths.offset(i as isize) = *bitlen.offset(i as isize);
        i = i.wrapping_add(1);
    }
    (*tree).numcodes = numcodes as ::core::ffi::c_uint;
    (*tree).maxbitlen = maxbitlen;
    return HuffmanTree_makeFromLengths2(tree);
}
unsafe extern "C" fn bpmnode_create(
    mut lists: *mut BPMLists,
    mut weight: ::core::ffi::c_int,
    mut index: ::core::ffi::c_uint,
    mut tail: *mut BPMNode,
) -> *mut BPMNode {
    let mut i: ::core::ffi::c_uint = 0;
    let mut result: *mut BPMNode = ::core::ptr::null_mut::<BPMNode>();
    if (*lists).nextfree >= (*lists).numfree {
        i = 0 as ::core::ffi::c_uint;
        while i != (*lists).memsize {
            (*(*lists).memory.offset(i as isize)).in_use = 0 as ::core::ffi::c_int;
            i = i.wrapping_add(1);
        }
        i = 0 as ::core::ffi::c_uint;
        while i != (*lists).listsize {
            let mut node: *mut BPMNode = ::core::ptr::null_mut::<BPMNode>();
            node = *(*lists).chains0.offset(i as isize);
            while !node.is_null() {
                (*node).in_use = 1 as ::core::ffi::c_int;
                node = (*node).tail as *mut BPMNode;
            }
            node = *(*lists).chains1.offset(i as isize);
            while !node.is_null() {
                (*node).in_use = 1 as ::core::ffi::c_int;
                node = (*node).tail as *mut BPMNode;
            }
            i = i.wrapping_add(1);
        }
        (*lists).numfree = 0 as ::core::ffi::c_uint;
        i = 0 as ::core::ffi::c_uint;
        while i != (*lists).memsize {
            if (*(*lists).memory.offset(i as isize)).in_use == 0 {
                let fresh88 = (*lists).numfree;
                (*lists).numfree = (*lists).numfree.wrapping_add(1);
                let ref mut fresh89 = *(*lists).freelist.offset(fresh88 as isize);
                *fresh89 = (*lists).memory.offset(i as isize) as *mut BPMNode;
            }
            i = i.wrapping_add(1);
        }
        (*lists).nextfree = 0 as ::core::ffi::c_uint;
    }
    let fresh90 = (*lists).nextfree;
    (*lists).nextfree = (*lists).nextfree.wrapping_add(1);
    result = *(*lists).freelist.offset(fresh90 as isize);
    (*result).weight = weight;
    (*result).index = index;
    (*result).tail = tail as *mut BPMNode;
    return result;
}
unsafe extern "C" fn bpmnode_sort(mut leaves: *mut BPMNode, mut num: size_t) {
    let mut mem: *mut BPMNode =
        lodepng_malloc((::core::mem::size_of::<BPMNode>() as size_t).wrapping_mul(num))
            as *mut BPMNode;
    let mut width: size_t = 0;
    let mut counter: size_t = 0 as size_t;
    width = 1 as size_t;
    while width < num {
        let mut a: *mut BPMNode = if counter & 1 as size_t != 0 {
            mem
        } else {
            leaves
        };
        let mut b: *mut BPMNode = if counter & 1 as size_t != 0 {
            leaves
        } else {
            mem
        };
        let mut p: size_t = 0;
        p = 0 as size_t;
        while p < num {
            let mut q: size_t = if p.wrapping_add(width) > num {
                num
            } else {
                p.wrapping_add(width)
            };
            let mut r: size_t = if p.wrapping_add((2 as size_t).wrapping_mul(width)) > num {
                num
            } else {
                p.wrapping_add((2 as size_t).wrapping_mul(width))
            };
            let mut i: size_t = p;
            let mut j: size_t = q;
            let mut k: size_t = 0;
            k = p;
            while k < r {
                if i < q
                    && (j >= r || (*a.offset(i as isize)).weight <= (*a.offset(j as isize)).weight)
                {
                    let fresh91 = i;
                    i = i.wrapping_add(1);
                    *b.offset(k as isize) = *a.offset(fresh91 as isize);
                } else {
                    let fresh92 = j;
                    j = j.wrapping_add(1);
                    *b.offset(k as isize) = *a.offset(fresh92 as isize);
                }
                k = k.wrapping_add(1);
            }
            p = (p as ::core::ffi::c_ulong)
                .wrapping_add((2 as size_t).wrapping_mul(width) as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        counter = counter.wrapping_add(1);
        width = (width as ::core::ffi::c_ulong).wrapping_mul(2 as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
    if counter & 1 as size_t != 0 {
        lodepng_memcpy(
            leaves as *mut ::core::ffi::c_void,
            mem as *const ::core::ffi::c_void,
            (::core::mem::size_of::<BPMNode>() as size_t).wrapping_mul(num),
        );
    }
    lodepng_free(mem as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn boundaryPM(
    mut lists: *mut BPMLists,
    mut leaves: *mut BPMNode,
    mut numpresent: size_t,
    mut c: ::core::ffi::c_int,
    mut num: ::core::ffi::c_int,
) {
    let mut lastindex: ::core::ffi::c_uint = (**(*lists).chains1.offset(c as isize)).index;
    if c == 0 as ::core::ffi::c_int {
        if lastindex as size_t >= numpresent {
            return;
        }
        let ref mut fresh83 = *(*lists).chains0.offset(c as isize);
        *fresh83 = *(*lists).chains1.offset(c as isize);
        let ref mut fresh84 = *(*lists).chains1.offset(c as isize);
        *fresh84 = bpmnode_create(
            lists,
            (*leaves.offset(lastindex as isize)).weight,
            lastindex.wrapping_add(1 as ::core::ffi::c_uint),
            ::core::ptr::null_mut::<BPMNode>(),
        );
    } else {
        let mut sum: ::core::ffi::c_int = (**(*lists)
            .chains0
            .offset((c - 1 as ::core::ffi::c_int) as isize))
        .weight
            + (**(*lists)
                .chains1
                .offset((c - 1 as ::core::ffi::c_int) as isize))
            .weight;
        let ref mut fresh85 = *(*lists).chains0.offset(c as isize);
        *fresh85 = *(*lists).chains1.offset(c as isize);
        if (lastindex as size_t) < numpresent && sum > (*leaves.offset(lastindex as isize)).weight {
            let ref mut fresh86 = *(*lists).chains1.offset(c as isize);
            *fresh86 = bpmnode_create(
                lists,
                (*leaves.offset(lastindex as isize)).weight,
                lastindex.wrapping_add(1 as ::core::ffi::c_uint),
                (**(*lists).chains1.offset(c as isize)).tail as *mut BPMNode,
            );
            return;
        }
        let ref mut fresh87 = *(*lists).chains1.offset(c as isize);
        *fresh87 = bpmnode_create(
            lists,
            sum,
            lastindex,
            *(*lists)
                .chains1
                .offset((c - 1 as ::core::ffi::c_int) as isize),
        );
        if (num + 1 as ::core::ffi::c_int)
            < (2 as size_t)
                .wrapping_mul(numpresent)
                .wrapping_sub(2 as size_t) as ::core::ffi::c_int
        {
            boundaryPM(lists, leaves, numpresent, c - 1 as ::core::ffi::c_int, num);
            boundaryPM(lists, leaves, numpresent, c - 1 as ::core::ffi::c_int, num);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_huffman_code_lengths(
    mut lengths: *mut ::core::ffi::c_uint,
    mut frequencies: *const ::core::ffi::c_uint,
    mut numcodes: size_t,
    mut maxbitlen: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint = 0;
    let mut numpresent: size_t = 0 as size_t;
    let mut leaves: *mut BPMNode = ::core::ptr::null_mut::<BPMNode>();
    if numcodes == 0 as size_t {
        return 80 as ::core::ffi::c_uint;
    }
    if (1 as ::core::ffi::c_uint) << maxbitlen < numcodes as ::core::ffi::c_uint {
        return 80 as ::core::ffi::c_uint;
    }
    leaves = lodepng_malloc(numcodes.wrapping_mul(::core::mem::size_of::<BPMNode>() as size_t))
        as *mut BPMNode;
    if leaves.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    i = 0 as ::core::ffi::c_uint;
    while i as size_t != numcodes {
        if *frequencies.offset(i as isize) > 0 as ::core::ffi::c_uint {
            (*leaves.offset(numpresent as isize)).weight =
                *frequencies.offset(i as isize) as ::core::ffi::c_int;
            (*leaves.offset(numpresent as isize)).index = i;
            numpresent = numpresent.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    lodepng_memset(
        lengths as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        numcodes.wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    );
    if numpresent == 0 as size_t {
        let ref mut fresh78 = *lengths.offset(1 as ::core::ffi::c_int as isize);
        *fresh78 = 1 as ::core::ffi::c_uint;
        *lengths.offset(0 as ::core::ffi::c_int as isize) = *fresh78;
    } else if numpresent == 1 as size_t {
        *lengths.offset((*leaves.offset(0 as ::core::ffi::c_int as isize)).index as isize) =
            1 as ::core::ffi::c_uint;
        *lengths.offset(
            (if (*leaves.offset(0 as ::core::ffi::c_int as isize)).index == 0 as ::core::ffi::c_uint
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as isize,
        ) = 1 as ::core::ffi::c_uint;
    } else {
        let mut lists: BPMLists = BPMLists {
            memsize: 0,
            memory: ::core::ptr::null_mut::<BPMNode>(),
            numfree: 0,
            nextfree: 0,
            freelist: ::core::ptr::null_mut::<*mut BPMNode>(),
            listsize: 0,
            chains0: ::core::ptr::null_mut::<*mut BPMNode>(),
            chains1: ::core::ptr::null_mut::<*mut BPMNode>(),
        };
        let mut node: *mut BPMNode = ::core::ptr::null_mut::<BPMNode>();
        bpmnode_sort(leaves, numpresent);
        lists.listsize = maxbitlen;
        lists.memsize = (2 as ::core::ffi::c_uint)
            .wrapping_mul(maxbitlen)
            .wrapping_mul(maxbitlen.wrapping_add(1 as ::core::ffi::c_uint));
        lists.nextfree = 0 as ::core::ffi::c_uint;
        lists.numfree = lists.memsize;
        lists.memory = lodepng_malloc(
            (lists.memsize as size_t).wrapping_mul(::core::mem::size_of::<BPMNode>() as size_t),
        ) as *mut BPMNode;
        lists.freelist = lodepng_malloc(
            (lists.memsize as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut BPMNode>() as size_t),
        ) as *mut *mut BPMNode;
        lists.chains0 = lodepng_malloc(
            (lists.listsize as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut BPMNode>() as size_t),
        ) as *mut *mut BPMNode;
        lists.chains1 = lodepng_malloc(
            (lists.listsize as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut BPMNode>() as size_t),
        ) as *mut *mut BPMNode;
        if lists.memory.is_null()
            || lists.freelist.is_null()
            || lists.chains0.is_null()
            || lists.chains1.is_null()
        {
            error = 83 as ::core::ffi::c_uint;
        }
        if error == 0 {
            i = 0 as ::core::ffi::c_uint;
            while i != lists.memsize {
                let ref mut fresh79 = *lists.freelist.offset(i as isize);
                *fresh79 = lists.memory.offset(i as isize) as *mut BPMNode;
                i = i.wrapping_add(1);
            }
            bpmnode_create(
                &raw mut lists,
                (*leaves.offset(0 as ::core::ffi::c_int as isize)).weight,
                1 as ::core::ffi::c_uint,
                ::core::ptr::null_mut::<BPMNode>(),
            );
            bpmnode_create(
                &raw mut lists,
                (*leaves.offset(1 as ::core::ffi::c_int as isize)).weight,
                2 as ::core::ffi::c_uint,
                ::core::ptr::null_mut::<BPMNode>(),
            );
            i = 0 as ::core::ffi::c_uint;
            while i != lists.listsize {
                let ref mut fresh80 = *lists.chains0.offset(i as isize);
                *fresh80 = lists.memory.offset(0 as ::core::ffi::c_int as isize) as *mut BPMNode;
                let ref mut fresh81 = *lists.chains1.offset(i as isize);
                *fresh81 = lists.memory.offset(1 as ::core::ffi::c_int as isize) as *mut BPMNode;
                i = i.wrapping_add(1);
            }
            i = 2 as ::core::ffi::c_uint;
            while i as size_t
                != (2 as size_t)
                    .wrapping_mul(numpresent)
                    .wrapping_sub(2 as size_t)
            {
                boundaryPM(
                    &raw mut lists,
                    leaves,
                    numpresent,
                    maxbitlen as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                    i as ::core::ffi::c_int,
                );
                i = i.wrapping_add(1);
            }
            node = *lists
                .chains1
                .offset(maxbitlen.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
            while !node.is_null() {
                i = 0 as ::core::ffi::c_uint;
                while i != (*node).index {
                    let ref mut fresh82 =
                        *lengths.offset((*leaves.offset(i as isize)).index as isize);
                    *fresh82 = (*fresh82).wrapping_add(1);
                    i = i.wrapping_add(1);
                }
                node = (*node).tail as *mut BPMNode;
            }
        }
        lodepng_free(lists.memory as *mut ::core::ffi::c_void);
        lodepng_free(lists.freelist as *mut ::core::ffi::c_void);
        lodepng_free(lists.chains0 as *mut ::core::ffi::c_void);
        lodepng_free(lists.chains1 as *mut ::core::ffi::c_void);
    }
    lodepng_free(leaves as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn HuffmanTree_makeFromFrequencies(
    mut tree: *mut HuffmanTree,
    mut frequencies: *const ::core::ffi::c_uint,
    mut mincodes: size_t,
    mut numcodes: size_t,
    mut maxbitlen: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while *frequencies.offset(numcodes.wrapping_sub(1 as size_t) as isize) == 0
        && numcodes > mincodes
    {
        numcodes = numcodes.wrapping_sub(1);
    }
    (*tree).lengths = lodepng_malloc(
        numcodes.wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    if (*tree).lengths.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    (*tree).maxbitlen = maxbitlen;
    (*tree).numcodes = numcodes as ::core::ffi::c_uint;
    error = lodepng_huffman_code_lengths((*tree).lengths, frequencies, numcodes, maxbitlen);
    if error == 0 {
        error = HuffmanTree_makeFromLengths2(tree);
    }
    return error;
}
unsafe extern "C" fn generateFixedLitLenTree(mut tree: *mut HuffmanTree) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut bitlen: *mut ::core::ffi::c_uint = lodepng_malloc(
        (NUM_DEFLATE_CODE_SYMBOLS as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    if bitlen.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    i = 0 as ::core::ffi::c_uint;
    while i <= 143 as ::core::ffi::c_uint {
        *bitlen.offset(i as isize) = 8 as ::core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    i = 144 as ::core::ffi::c_uint;
    while i <= 255 as ::core::ffi::c_uint {
        *bitlen.offset(i as isize) = 9 as ::core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    i = 256 as ::core::ffi::c_uint;
    while i <= 279 as ::core::ffi::c_uint {
        *bitlen.offset(i as isize) = 7 as ::core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    i = 280 as ::core::ffi::c_uint;
    while i <= 287 as ::core::ffi::c_uint {
        *bitlen.offset(i as isize) = 8 as ::core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    error = HuffmanTree_makeFromLengths(
        tree,
        bitlen,
        NUM_DEFLATE_CODE_SYMBOLS as size_t,
        15 as ::core::ffi::c_uint,
    );
    lodepng_free(bitlen as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn generateFixedDistanceTree(mut tree: *mut HuffmanTree) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut bitlen: *mut ::core::ffi::c_uint = lodepng_malloc(
        (NUM_DISTANCE_SYMBOLS as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    if bitlen.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    i = 0 as ::core::ffi::c_uint;
    while i != NUM_DISTANCE_SYMBOLS as ::core::ffi::c_uint {
        *bitlen.offset(i as isize) = 5 as ::core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    error = HuffmanTree_makeFromLengths(
        tree,
        bitlen,
        NUM_DISTANCE_SYMBOLS as size_t,
        15 as ::core::ffi::c_uint,
    );
    lodepng_free(bitlen as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn huffmanDecodeSymbol(
    mut reader: *mut LodePNGBitReader,
    mut codetree: *const HuffmanTree,
) -> ::core::ffi::c_uint {
    let mut code: ::core::ffi::c_ushort =
        peekBits(reader, FIRSTBITS as size_t) as ::core::ffi::c_ushort;
    let mut l: ::core::ffi::c_ushort =
        *(*codetree).table_len.offset(code as isize) as ::core::ffi::c_ushort;
    let mut value: ::core::ffi::c_ushort = *(*codetree).table_value.offset(code as isize);
    if l as ::core::ffi::c_uint <= FIRSTBITS {
        advanceBits(reader, l as size_t);
        return value as ::core::ffi::c_uint;
    } else {
        advanceBits(reader, FIRSTBITS as size_t);
        value = (value as ::core::ffi::c_uint).wrapping_add(peekBits(
            reader,
            (l as ::core::ffi::c_uint).wrapping_sub(FIRSTBITS) as size_t,
        )) as ::core::ffi::c_ushort as ::core::ffi::c_ushort;
        advanceBits(
            reader,
            (*(*codetree).table_len.offset(value as isize) as ::core::ffi::c_uint)
                .wrapping_sub(FIRSTBITS) as size_t,
        );
        return *(*codetree).table_value.offset(value as isize) as ::core::ffi::c_uint;
    };
}
unsafe extern "C" fn getTreeInflateFixed(
    mut tree_ll: *mut HuffmanTree,
    mut tree_d: *mut HuffmanTree,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = generateFixedLitLenTree(tree_ll);
    if error != 0 {
        return error;
    }
    return generateFixedDistanceTree(tree_d);
}
unsafe extern "C" fn getTreeInflateDynamic(
    mut tree_ll: *mut HuffmanTree,
    mut tree_d: *mut HuffmanTree,
    mut reader: *mut LodePNGBitReader,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut n: ::core::ffi::c_uint = 0;
    let mut HLIT: ::core::ffi::c_uint = 0;
    let mut HDIST: ::core::ffi::c_uint = 0;
    let mut HCLEN: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    let mut bitlen_ll: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut bitlen_d: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut bitlen_cl: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut tree_cl: HuffmanTree = HuffmanTree {
        codes: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        lengths: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        maxbitlen: 0,
        numcodes: 0,
        table_len: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        table_value: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
    };
    if (*reader).bitsize.wrapping_sub((*reader).bp) < 14 as size_t {
        return 49 as ::core::ffi::c_uint;
    }
    ensureBits17(reader, 14 as size_t);
    HLIT = readBits(reader, 5 as size_t).wrapping_add(257 as ::core::ffi::c_uint);
    HDIST = readBits(reader, 5 as size_t).wrapping_add(1 as ::core::ffi::c_uint);
    HCLEN = readBits(reader, 4 as size_t).wrapping_add(4 as ::core::ffi::c_uint);
    bitlen_cl = lodepng_malloc(
        (NUM_CODE_LENGTH_CODES as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    if bitlen_cl.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    HuffmanTree_init(&raw mut tree_cl);
    if error == 0 {
        if lodepng_gtofl(
            (*reader).bp,
            HCLEN.wrapping_mul(3 as ::core::ffi::c_uint) as size_t,
            (*reader).bitsize,
        ) != 0
        {
            error = 50 as ::core::ffi::c_uint;
        } else {
            i = 0 as ::core::ffi::c_uint;
            while i != HCLEN {
                ensureBits9(reader, 3 as size_t);
                *bitlen_cl.offset(CLCL_ORDER[i as usize] as isize) = readBits(reader, 3 as size_t);
                i = i.wrapping_add(1);
            }
            i = HCLEN;
            while i != NUM_CODE_LENGTH_CODES as ::core::ffi::c_uint {
                *bitlen_cl.offset(CLCL_ORDER[i as usize] as isize) = 0 as ::core::ffi::c_uint;
                i = i.wrapping_add(1);
            }
            error = HuffmanTree_makeFromLengths(
                &raw mut tree_cl,
                bitlen_cl,
                NUM_CODE_LENGTH_CODES as size_t,
                7 as ::core::ffi::c_uint,
            );
            if !(error != 0) {
                bitlen_ll = lodepng_malloc(
                    (NUM_DEFLATE_CODE_SYMBOLS as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
                ) as *mut ::core::ffi::c_uint;
                bitlen_d = lodepng_malloc(
                    (NUM_DISTANCE_SYMBOLS as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
                ) as *mut ::core::ffi::c_uint;
                if bitlen_ll.is_null() || bitlen_d.is_null() {
                    error = 83 as ::core::ffi::c_uint;
                } else {
                    lodepng_memset(
                        bitlen_ll as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (NUM_DEFLATE_CODE_SYMBOLS as size_t).wrapping_mul(::core::mem::size_of::<
                            ::core::ffi::c_uint,
                        >(
                        )
                            as size_t),
                    );
                    lodepng_memset(
                        bitlen_d as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (NUM_DISTANCE_SYMBOLS as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
                    );
                    i = 0 as ::core::ffi::c_uint;
                    while i < HLIT.wrapping_add(HDIST) {
                        let mut code: ::core::ffi::c_uint = 0;
                        ensureBits25(reader, 22 as size_t);
                        code = huffmanDecodeSymbol(reader, &raw mut tree_cl);
                        if code <= 15 as ::core::ffi::c_uint {
                            if i < HLIT {
                                *bitlen_ll.offset(i as isize) = code;
                            } else {
                                *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) = code;
                            }
                            i = i.wrapping_add(1);
                        } else if code == 16 as ::core::ffi::c_uint {
                            let mut replength: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
                            let mut value: ::core::ffi::c_uint = 0;
                            if i == 0 as ::core::ffi::c_uint {
                                error = 54 as ::core::ffi::c_uint;
                                break;
                            } else {
                                replength = replength.wrapping_add(readBits(reader, 2 as size_t));
                                if i < HLIT.wrapping_add(1 as ::core::ffi::c_uint) {
                                    value = *bitlen_ll
                                        .offset(i.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
                                } else {
                                    value = *bitlen_d.offset(
                                        i.wrapping_sub(HLIT).wrapping_sub(1 as ::core::ffi::c_uint)
                                            as isize,
                                    );
                                }
                                n = 0 as ::core::ffi::c_uint;
                                while n < replength {
                                    if i >= HLIT.wrapping_add(HDIST) {
                                        error = 13 as ::core::ffi::c_uint;
                                        break;
                                    } else {
                                        if i < HLIT {
                                            *bitlen_ll.offset(i as isize) = value;
                                        } else {
                                            *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) = value;
                                        }
                                        i = i.wrapping_add(1);
                                        n = n.wrapping_add(1);
                                    }
                                }
                            }
                        } else if code == 17 as ::core::ffi::c_uint {
                            let mut replength_0: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
                            replength_0 = replength_0.wrapping_add(readBits(reader, 3 as size_t));
                            n = 0 as ::core::ffi::c_uint;
                            while n < replength_0 {
                                if i >= HLIT.wrapping_add(HDIST) {
                                    error = 14 as ::core::ffi::c_uint;
                                    break;
                                } else {
                                    if i < HLIT {
                                        *bitlen_ll.offset(i as isize) = 0 as ::core::ffi::c_uint;
                                    } else {
                                        *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) =
                                            0 as ::core::ffi::c_uint;
                                    }
                                    i = i.wrapping_add(1);
                                    n = n.wrapping_add(1);
                                }
                            }
                        } else if code == 18 as ::core::ffi::c_uint {
                            let mut replength_1: ::core::ffi::c_uint = 11 as ::core::ffi::c_uint;
                            replength_1 = replength_1.wrapping_add(readBits(reader, 7 as size_t));
                            n = 0 as ::core::ffi::c_uint;
                            while n < replength_1 {
                                if i >= HLIT.wrapping_add(HDIST) {
                                    error = 15 as ::core::ffi::c_uint;
                                    break;
                                } else {
                                    if i < HLIT {
                                        *bitlen_ll.offset(i as isize) = 0 as ::core::ffi::c_uint;
                                    } else {
                                        *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) =
                                            0 as ::core::ffi::c_uint;
                                    }
                                    i = i.wrapping_add(1);
                                    n = n.wrapping_add(1);
                                }
                            }
                        } else {
                            error = 16 as ::core::ffi::c_uint;
                            break;
                        }
                        if !((*reader).bp > (*reader).bitsize) {
                            continue;
                        }
                        error = 50 as ::core::ffi::c_uint;
                        break;
                    }
                    if !(error != 0) {
                        if *bitlen_ll.offset(256 as ::core::ffi::c_int as isize)
                            == 0 as ::core::ffi::c_uint
                        {
                            error = 64 as ::core::ffi::c_uint;
                        } else {
                            error = HuffmanTree_makeFromLengths(
                                tree_ll,
                                bitlen_ll,
                                NUM_DEFLATE_CODE_SYMBOLS as size_t,
                                15 as ::core::ffi::c_uint,
                            );
                            if !(error != 0) {
                                error = HuffmanTree_makeFromLengths(
                                    tree_d,
                                    bitlen_d,
                                    NUM_DISTANCE_SYMBOLS as size_t,
                                    15 as ::core::ffi::c_uint,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    lodepng_free(bitlen_cl as *mut ::core::ffi::c_void);
    lodepng_free(bitlen_ll as *mut ::core::ffi::c_void);
    lodepng_free(bitlen_d as *mut ::core::ffi::c_void);
    HuffmanTree_cleanup(&raw mut tree_cl);
    return error;
}
unsafe extern "C" fn inflateHuffmanBlock(
    mut out: *mut ucvector,
    mut reader: *mut LodePNGBitReader,
    mut btype: ::core::ffi::c_uint,
    mut max_output_size: size_t,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut tree_ll: HuffmanTree = HuffmanTree {
        codes: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        lengths: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        maxbitlen: 0,
        numcodes: 0,
        table_len: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        table_value: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
    };
    let mut tree_d: HuffmanTree = HuffmanTree {
        codes: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        lengths: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        maxbitlen: 0,
        numcodes: 0,
        table_len: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        table_value: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
    };
    let reserved_size: size_t = 260 as size_t;
    let mut done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ucvector_reserve(out, (*out).size.wrapping_add(reserved_size)) == 0 {
        return 83 as ::core::ffi::c_uint;
    }
    HuffmanTree_init(&raw mut tree_ll);
    HuffmanTree_init(&raw mut tree_d);
    if btype == 1 as ::core::ffi::c_uint {
        error = getTreeInflateFixed(&raw mut tree_ll, &raw mut tree_d);
    } else {
        error = getTreeInflateDynamic(&raw mut tree_ll, &raw mut tree_d, reader);
    }
    while error == 0 && done == 0 {
        let mut code_ll: ::core::ffi::c_uint = 0;
        ensureBits32(reader, 30 as size_t);
        code_ll = huffmanDecodeSymbol(reader, &raw mut tree_ll);
        if code_ll <= 255 as ::core::ffi::c_uint {
            let fresh36 = (*out).size;
            (*out).size = (*out).size.wrapping_add(1);
            *(*out).data.offset(fresh36 as isize) = code_ll as ::core::ffi::c_uchar;
            code_ll = huffmanDecodeSymbol(reader, &raw mut tree_ll);
        }
        if code_ll <= 255 as ::core::ffi::c_uint {
            let fresh37 = (*out).size;
            (*out).size = (*out).size.wrapping_add(1);
            *(*out).data.offset(fresh37 as isize) = code_ll as ::core::ffi::c_uchar;
        } else if code_ll >= FIRST_LENGTH_CODE_INDEX as ::core::ffi::c_uint
            && code_ll <= LAST_LENGTH_CODE_INDEX as ::core::ffi::c_uint
        {
            let mut code_d: ::core::ffi::c_uint = 0;
            let mut distance: ::core::ffi::c_uint = 0;
            let mut numextrabits_l: ::core::ffi::c_uint = 0;
            let mut numextrabits_d: ::core::ffi::c_uint = 0;
            let mut start: size_t = 0;
            let mut backward: size_t = 0;
            let mut length: size_t = 0;
            length = LENGTHBASE
                [code_ll.wrapping_sub(FIRST_LENGTH_CODE_INDEX as ::core::ffi::c_uint) as usize]
                as size_t;
            numextrabits_l = LENGTHEXTRA
                [code_ll.wrapping_sub(FIRST_LENGTH_CODE_INDEX as ::core::ffi::c_uint) as usize];
            if numextrabits_l != 0 as ::core::ffi::c_uint {
                ensureBits25(reader, 5 as size_t);
                length = (length as ::core::ffi::c_ulong).wrapping_add(readBits(
                    reader,
                    numextrabits_l as size_t,
                )
                    as ::core::ffi::c_ulong) as size_t as size_t;
            }
            ensureBits32(reader, 28 as size_t);
            code_d = huffmanDecodeSymbol(reader, &raw mut tree_d);
            if code_d > 29 as ::core::ffi::c_uint {
                if code_d <= 31 as ::core::ffi::c_uint {
                    error = 18 as ::core::ffi::c_uint;
                    break;
                } else {
                    error = 16 as ::core::ffi::c_uint;
                    break;
                }
            } else {
                distance = DISTANCEBASE[code_d as usize];
                numextrabits_d = DISTANCEEXTRA[code_d as usize];
                if numextrabits_d != 0 as ::core::ffi::c_uint {
                    distance = distance.wrapping_add(readBits(reader, numextrabits_d as size_t));
                }
                start = (*out).size;
                if distance as size_t > start {
                    error = 52 as ::core::ffi::c_uint;
                    break;
                } else {
                    backward = start.wrapping_sub(distance as size_t);
                    (*out).size = ((*out).size as ::core::ffi::c_ulong)
                        .wrapping_add(length as ::core::ffi::c_ulong)
                        as size_t as size_t;
                    if (distance as size_t) < length {
                        let mut forward: size_t = 0;
                        lodepng_memcpy(
                            (*out).data.offset(start as isize) as *mut ::core::ffi::c_void,
                            (*out).data.offset(backward as isize) as *const ::core::ffi::c_void,
                            distance as size_t,
                        );
                        start = (start as ::core::ffi::c_ulong)
                            .wrapping_add(distance as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        forward = distance as size_t;
                        while forward < length {
                            let fresh38 = backward;
                            backward = backward.wrapping_add(1);
                            let fresh39 = start;
                            start = start.wrapping_add(1);
                            *(*out).data.offset(fresh39 as isize) =
                                *(*out).data.offset(fresh38 as isize);
                            forward = forward.wrapping_add(1);
                        }
                    } else {
                        lodepng_memcpy(
                            (*out).data.offset(start as isize) as *mut ::core::ffi::c_void,
                            (*out).data.offset(backward as isize) as *const ::core::ffi::c_void,
                            length,
                        );
                    }
                }
            }
        } else if code_ll == 256 as ::core::ffi::c_uint {
            done = 1 as ::core::ffi::c_int;
        } else {
            error = 16 as ::core::ffi::c_uint;
            break;
        }
        if (*out).allocsize.wrapping_sub((*out).size) < reserved_size {
            if ucvector_reserve(out, (*out).size.wrapping_add(reserved_size)) == 0 {
                error = 83 as ::core::ffi::c_uint;
                break;
            }
        }
        if (*reader).bp > (*reader).bitsize {
            error = 51 as ::core::ffi::c_uint;
            break;
        } else {
            if !(max_output_size != 0 && (*out).size > max_output_size) {
                continue;
            }
            error = 109 as ::core::ffi::c_uint;
            break;
        }
    }
    HuffmanTree_cleanup(&raw mut tree_ll);
    HuffmanTree_cleanup(&raw mut tree_d);
    return error;
}
unsafe extern "C" fn inflateNoCompression(
    mut out: *mut ucvector,
    mut reader: *mut LodePNGBitReader,
    mut settings: *const LodePNGDecompressSettings,
) -> ::core::ffi::c_uint {
    let mut bytepos: size_t = 0;
    let mut size: size_t = (*reader).size;
    let mut LEN: ::core::ffi::c_uint = 0;
    let mut NLEN: ::core::ffi::c_uint = 0;
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    bytepos = (*reader).bp.wrapping_add(7 as size_t) >> 3 as ::core::ffi::c_uint;
    if bytepos.wrapping_add(4 as size_t) >= size {
        return 52 as ::core::ffi::c_uint;
    }
    LEN = (*(*reader).data.offset(bytepos as isize) as ::core::ffi::c_uint).wrapping_add(
        (*(*reader)
            .data
            .offset(bytepos.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_uint,
    );
    bytepos = (bytepos as ::core::ffi::c_ulong).wrapping_add(2 as ::core::ffi::c_ulong) as size_t
        as size_t;
    NLEN = (*(*reader).data.offset(bytepos as isize) as ::core::ffi::c_uint).wrapping_add(
        (*(*reader)
            .data
            .offset(bytepos.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_uint,
    );
    bytepos = (bytepos as ::core::ffi::c_ulong).wrapping_add(2 as ::core::ffi::c_ulong) as size_t
        as size_t;
    if (*settings).ignore_nlen == 0 && LEN.wrapping_add(NLEN) != 65535 as ::core::ffi::c_uint {
        return 21 as ::core::ffi::c_uint;
    }
    if ucvector_resize(out, (*out).size.wrapping_add(LEN as size_t)) == 0 {
        return 83 as ::core::ffi::c_uint;
    }
    if bytepos.wrapping_add(LEN as size_t) > size {
        return 23 as ::core::ffi::c_uint;
    }
    if LEN != 0 {
        lodepng_memcpy(
            (*out)
                .data
                .offset((*out).size as isize)
                .offset(-(LEN as isize)) as *mut ::core::ffi::c_void,
            (*reader).data.offset(bytepos as isize) as *const ::core::ffi::c_void,
            LEN as size_t,
        );
        bytepos = (bytepos as ::core::ffi::c_ulong).wrapping_add(LEN as ::core::ffi::c_ulong)
            as size_t as size_t;
    }
    (*reader).bp = bytepos << 3 as ::core::ffi::c_uint;
    return error;
}
unsafe extern "C" fn lodepng_inflatev(
    mut out: *mut ucvector,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGDecompressSettings,
) -> ::core::ffi::c_uint {
    let mut BFINAL: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut reader: LodePNGBitReader = LodePNGBitReader {
        data: ::core::ptr::null::<::core::ffi::c_uchar>(),
        size: 0,
        bitsize: 0,
        bp: 0,
        buffer: 0,
    };
    let mut error: ::core::ffi::c_uint = LodePNGBitReader_init(&raw mut reader, in_0, insize);
    if error != 0 {
        return error;
    }
    while BFINAL == 0 {
        let mut BTYPE: ::core::ffi::c_uint = 0;
        if reader.bitsize.wrapping_sub(reader.bp) < 3 as size_t {
            return 52 as ::core::ffi::c_uint;
        }
        ensureBits9(&raw mut reader, 3 as size_t);
        BFINAL = readBits(&raw mut reader, 1 as size_t);
        BTYPE = readBits(&raw mut reader, 2 as size_t);
        if BTYPE == 3 as ::core::ffi::c_uint {
            return 20 as ::core::ffi::c_uint;
        } else if BTYPE == 0 as ::core::ffi::c_uint {
            error = inflateNoCompression(out, &raw mut reader, settings);
        } else {
            error = inflateHuffmanBlock(out, &raw mut reader, BTYPE, (*settings).max_output_size);
        }
        if error == 0
            && (*settings).max_output_size != 0
            && (*out).size > (*settings).max_output_size
        {
            error = 109 as ::core::ffi::c_uint;
        }
        if error != 0 {
            break;
        }
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_inflate(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGDecompressSettings,
) -> ::core::ffi::c_uint {
    let mut v: ucvector = ucvector_init(*out, *outsize);
    let mut error: ::core::ffi::c_uint = lodepng_inflatev(&raw mut v, in_0, insize, settings);
    *out = v.data;
    *outsize = v.size;
    return error;
}
unsafe extern "C" fn inflatev(
    mut out: *mut ucvector,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGDecompressSettings,
) -> ::core::ffi::c_uint {
    if (*settings).custom_inflate.is_some() {
        let mut error: ::core::ffi::c_uint = (*settings)
            .custom_inflate
            .expect("non-null function pointer")(
            &raw mut (*out).data,
            &raw mut (*out).size,
            in_0,
            insize,
            settings,
        );
        (*out).allocsize = (*out).size;
        if error != 0 {
            error = 110 as ::core::ffi::c_uint;
            if (*settings).max_output_size != 0 && (*out).size > (*settings).max_output_size {
                error = 109 as ::core::ffi::c_uint;
            }
        }
        return error;
    } else {
        return lodepng_inflatev(out, in_0, insize, settings);
    };
}
static mut MAX_SUPPORTED_DEFLATE_LENGTH: ::core::ffi::c_uint = 258 as ::core::ffi::c_uint;
unsafe extern "C" fn searchCodeIndex(
    mut array: *const ::core::ffi::c_uint,
    mut array_size: size_t,
    mut value: size_t,
) -> size_t {
    let mut left: size_t = 1 as size_t;
    let mut right: size_t = array_size.wrapping_sub(1 as size_t);
    while left <= right {
        let mut mid: size_t = left.wrapping_add(right) >> 1 as ::core::ffi::c_int;
        if *array.offset(mid as isize) as size_t >= value {
            right = mid.wrapping_sub(1 as size_t);
        } else {
            left = mid.wrapping_add(1 as size_t);
        }
    }
    if left >= array_size || *array.offset(left as isize) as size_t > value {
        left = left.wrapping_sub(1);
    }
    return left;
}
unsafe extern "C" fn addLengthDistance(
    mut values: *mut uivector,
    mut length: size_t,
    mut distance: size_t,
) {
    let mut length_code: ::core::ffi::c_uint = searchCodeIndex(
        &raw const LENGTHBASE as *const ::core::ffi::c_uint,
        29 as size_t,
        length,
    ) as ::core::ffi::c_uint;
    let mut extra_length: ::core::ffi::c_uint =
        length.wrapping_sub(LENGTHBASE[length_code as usize] as size_t) as ::core::ffi::c_uint;
    let mut dist_code: ::core::ffi::c_uint = searchCodeIndex(
        &raw const DISTANCEBASE as *const ::core::ffi::c_uint,
        30 as size_t,
        distance,
    ) as ::core::ffi::c_uint;
    let mut extra_distance: ::core::ffi::c_uint =
        distance.wrapping_sub(DISTANCEBASE[dist_code as usize] as size_t) as ::core::ffi::c_uint;
    let mut pos: size_t = (*values).size;
    let mut new_size: size_t = (*values).size.wrapping_add(4 as size_t);
    let mut ok: ::core::ffi::c_uint = uivector_reserve(values, new_size);
    if ok != 0 {
        (*values).size = new_size;
        *(*values)
            .data
            .offset(pos.wrapping_add(0 as size_t) as isize) =
            length_code.wrapping_add(FIRST_LENGTH_CODE_INDEX as ::core::ffi::c_uint);
        *(*values)
            .data
            .offset(pos.wrapping_add(1 as size_t) as isize) = extra_length;
        *(*values)
            .data
            .offset(pos.wrapping_add(2 as size_t) as isize) = dist_code;
        *(*values)
            .data
            .offset(pos.wrapping_add(3 as size_t) as isize) = extra_distance;
    }
}
static mut HASH_NUM_VALUES: ::core::ffi::c_uint =
    65536 as ::core::ffi::c_int as ::core::ffi::c_uint;
static mut HASH_BIT_MASK: ::core::ffi::c_uint = 65535 as ::core::ffi::c_uint;
unsafe extern "C" fn hash_init(
    mut hash: *mut Hash,
    mut windowsize: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    (*hash).head = lodepng_malloc(
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_mul(HASH_NUM_VALUES as size_t),
    ) as *mut ::core::ffi::c_int;
    (*hash).val = lodepng_malloc(
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t).wrapping_mul(windowsize as size_t),
    ) as *mut ::core::ffi::c_int;
    (*hash).chain = lodepng_malloc(
        (::core::mem::size_of::<::core::ffi::c_ushort>() as size_t)
            .wrapping_mul(windowsize as size_t),
    ) as *mut ::core::ffi::c_ushort;
    (*hash).zeros = lodepng_malloc(
        (::core::mem::size_of::<::core::ffi::c_ushort>() as size_t)
            .wrapping_mul(windowsize as size_t),
    ) as *mut ::core::ffi::c_ushort;
    (*hash).headz = lodepng_malloc(
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t).wrapping_mul(
            MAX_SUPPORTED_DEFLATE_LENGTH.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
        ),
    ) as *mut ::core::ffi::c_int;
    (*hash).chainz = lodepng_malloc(
        (::core::mem::size_of::<::core::ffi::c_ushort>() as size_t)
            .wrapping_mul(windowsize as size_t),
    ) as *mut ::core::ffi::c_ushort;
    if (*hash).head.is_null()
        || (*hash).chain.is_null()
        || (*hash).val.is_null()
        || (*hash).headz.is_null()
        || (*hash).chainz.is_null()
        || (*hash).zeros.is_null()
    {
        return 83 as ::core::ffi::c_uint;
    }
    lodepng_memset(
        (*hash).head as *mut ::core::ffi::c_void,
        -(1 as ::core::ffi::c_int),
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_mul(HASH_NUM_VALUES as size_t),
    );
    lodepng_memset(
        (*hash).val as *mut ::core::ffi::c_void,
        -(1 as ::core::ffi::c_int),
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_mul(windowsize as size_t),
    );
    i = 0 as ::core::ffi::c_uint;
    while i != windowsize {
        *(*hash).chain.offset(i as isize) = i as ::core::ffi::c_ushort;
        i = i.wrapping_add(1);
    }
    lodepng_memset(
        (*hash).headz as *mut ::core::ffi::c_void,
        -(1 as ::core::ffi::c_int),
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t).wrapping_mul(
            MAX_SUPPORTED_DEFLATE_LENGTH.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
        ),
    );
    i = 0 as ::core::ffi::c_uint;
    while i != windowsize {
        *(*hash).chainz.offset(i as isize) = i as ::core::ffi::c_ushort;
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn hash_cleanup(mut hash: *mut Hash) {
    lodepng_free((*hash).head as *mut ::core::ffi::c_void);
    lodepng_free((*hash).val as *mut ::core::ffi::c_void);
    lodepng_free((*hash).chain as *mut ::core::ffi::c_void);
    lodepng_free((*hash).zeros as *mut ::core::ffi::c_void);
    lodepng_free((*hash).headz as *mut ::core::ffi::c_void);
    lodepng_free((*hash).chainz as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn getHash(
    mut data: *const ::core::ffi::c_uchar,
    mut size: size_t,
    mut pos: size_t,
) -> ::core::ffi::c_uint {
    let mut result: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if pos.wrapping_add(2 as size_t) < size {
        result ^= (*data.offset(pos.wrapping_add(0 as size_t) as isize) as ::core::ffi::c_uint)
            << 0 as ::core::ffi::c_uint;
        result ^= (*data.offset(pos.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_uint)
            << 4 as ::core::ffi::c_uint;
        result ^= (*data.offset(pos.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_uint;
    } else {
        let mut amount: size_t = 0;
        let mut i: size_t = 0;
        if pos >= size {
            return 0 as ::core::ffi::c_uint;
        }
        amount = size.wrapping_sub(pos);
        i = 0 as size_t;
        while i != amount {
            result ^= (*data.offset(pos.wrapping_add(i) as isize) as ::core::ffi::c_uint)
                << i.wrapping_mul(8 as size_t);
            i = i.wrapping_add(1);
        }
    }
    return result & HASH_BIT_MASK;
}
unsafe extern "C" fn countZeros(
    mut data: *const ::core::ffi::c_uchar,
    mut size: size_t,
    mut pos: size_t,
) -> ::core::ffi::c_uint {
    let mut start: *const ::core::ffi::c_uchar = data.offset(pos as isize);
    let mut end: *const ::core::ffi::c_uchar = start.offset(MAX_SUPPORTED_DEFLATE_LENGTH as isize);
    if end > data.offset(size as isize) {
        end = data.offset(size as isize);
    }
    data = start;
    while data != end && *data as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        data = data.offset(1);
    }
    return data.offset_from(start) as ::core::ffi::c_long as ::core::ffi::c_uint;
}
unsafe extern "C" fn updateHashChain(
    mut hash: *mut Hash,
    mut wpos: size_t,
    mut hashval: ::core::ffi::c_uint,
    mut numzeros: ::core::ffi::c_ushort,
) {
    *(*hash).val.offset(wpos as isize) = hashval as ::core::ffi::c_int;
    if *(*hash).head.offset(hashval as isize) != -(1 as ::core::ffi::c_int) {
        *(*hash).chain.offset(wpos as isize) =
            *(*hash).head.offset(hashval as isize) as ::core::ffi::c_ushort;
    }
    *(*hash).head.offset(hashval as isize) = wpos as ::core::ffi::c_int;
    *(*hash).zeros.offset(wpos as isize) = numzeros;
    if *(*hash).headz.offset(numzeros as isize) != -(1 as ::core::ffi::c_int) {
        *(*hash).chainz.offset(wpos as isize) =
            *(*hash).headz.offset(numzeros as isize) as ::core::ffi::c_ushort;
    }
    *(*hash).headz.offset(numzeros as isize) = wpos as ::core::ffi::c_int;
}
unsafe extern "C" fn encodeLZ77(
    mut out: *mut uivector,
    mut hash: *mut Hash,
    mut in_0: *const ::core::ffi::c_uchar,
    mut inpos: size_t,
    mut insize: size_t,
    mut windowsize: ::core::ffi::c_uint,
    mut minmatch: ::core::ffi::c_uint,
    mut nicematch: ::core::ffi::c_uint,
    mut lazymatching: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut pos: size_t = 0;
    let mut i: ::core::ffi::c_uint = 0;
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut maxchainlength: ::core::ffi::c_uint = if windowsize >= 8192 as ::core::ffi::c_uint {
        windowsize
    } else {
        windowsize.wrapping_div(8 as ::core::ffi::c_uint)
    };
    let mut maxlazymatch: ::core::ffi::c_uint = if windowsize >= 8192 as ::core::ffi::c_uint {
        MAX_SUPPORTED_DEFLATE_LENGTH
    } else {
        64 as ::core::ffi::c_uint
    };
    let mut usezeros: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
    let mut numzeros: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut offset: ::core::ffi::c_uint = 0;
    let mut length: ::core::ffi::c_uint = 0;
    let mut lazy: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut lazylength: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut lazyoffset: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut hashval: ::core::ffi::c_uint = 0;
    let mut current_offset: ::core::ffi::c_uint = 0;
    let mut current_length: ::core::ffi::c_uint = 0;
    let mut prev_offset: ::core::ffi::c_uint = 0;
    let mut lastptr: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut foreptr: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut backptr: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut hashpos: ::core::ffi::c_uint = 0;
    if windowsize == 0 as ::core::ffi::c_uint || windowsize > 32768 as ::core::ffi::c_uint {
        return 60 as ::core::ffi::c_uint;
    }
    if windowsize & windowsize.wrapping_sub(1 as ::core::ffi::c_uint) != 0 as ::core::ffi::c_uint {
        return 90 as ::core::ffi::c_uint;
    }
    if nicematch > MAX_SUPPORTED_DEFLATE_LENGTH {
        nicematch = MAX_SUPPORTED_DEFLATE_LENGTH;
    }
    let mut current_block_78: u64;
    pos = inpos;
    while pos < insize {
        let mut wpos: size_t = pos & windowsize.wrapping_sub(1 as ::core::ffi::c_uint) as size_t;
        let mut chainlength: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        hashval = getHash(in_0, insize, pos);
        if usezeros != 0 && hashval == 0 as ::core::ffi::c_uint {
            if numzeros == 0 as ::core::ffi::c_uint {
                numzeros = countZeros(in_0, insize, pos);
            } else if pos.wrapping_add(numzeros as size_t) > insize
                || *in_0.offset(
                    pos.wrapping_add(numzeros as size_t)
                        .wrapping_sub(1 as size_t) as isize,
                ) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            {
                numzeros = numzeros.wrapping_sub(1);
            }
        } else {
            numzeros = 0 as ::core::ffi::c_uint;
        }
        updateHashChain(hash, wpos, hashval, numzeros as ::core::ffi::c_ushort);
        length = 0 as ::core::ffi::c_uint;
        offset = 0 as ::core::ffi::c_uint;
        hashpos = *(*hash).chain.offset(wpos as isize) as ::core::ffi::c_uint;
        lastptr = in_0.offset(
            (if insize < pos.wrapping_add(MAX_SUPPORTED_DEFLATE_LENGTH as size_t) {
                insize
            } else {
                pos.wrapping_add(MAX_SUPPORTED_DEFLATE_LENGTH as size_t)
            }) as isize,
        ) as *const ::core::ffi::c_uchar;
        prev_offset = 0 as ::core::ffi::c_uint;
        loop {
            let fresh93 = chainlength;
            chainlength = chainlength.wrapping_add(1);
            if fresh93 >= maxchainlength {
                break;
            }
            current_offset = (if hashpos as size_t <= wpos {
                wpos.wrapping_sub(hashpos as size_t)
            } else {
                wpos.wrapping_sub(hashpos as size_t)
                    .wrapping_add(windowsize as size_t)
            }) as ::core::ffi::c_uint;
            if current_offset < prev_offset {
                break;
            }
            prev_offset = current_offset;
            if current_offset > 0 as ::core::ffi::c_uint {
                foreptr = in_0.offset(pos as isize) as *const ::core::ffi::c_uchar;
                backptr = in_0.offset(pos.wrapping_sub(current_offset as size_t) as isize)
                    as *const ::core::ffi::c_uchar;
                if numzeros >= 3 as ::core::ffi::c_uint {
                    let mut skip: ::core::ffi::c_uint =
                        *(*hash).zeros.offset(hashpos as isize) as ::core::ffi::c_uint;
                    if skip > numzeros {
                        skip = numzeros;
                    }
                    backptr = backptr.offset(skip as isize);
                    foreptr = foreptr.offset(skip as isize);
                }
                while foreptr != lastptr
                    && *backptr as ::core::ffi::c_int == *foreptr as ::core::ffi::c_int
                {
                    backptr = backptr.offset(1);
                    foreptr = foreptr.offset(1);
                }
                current_length = foreptr
                    .offset_from(in_0.offset(pos as isize) as *const ::core::ffi::c_uchar)
                    as ::core::ffi::c_long as ::core::ffi::c_uint;
                if current_length > length {
                    length = current_length;
                    offset = current_offset;
                    if current_length >= nicematch {
                        break;
                    }
                }
            }
            if hashpos == *(*hash).chain.offset(hashpos as isize) as ::core::ffi::c_uint {
                break;
            }
            if numzeros >= 3 as ::core::ffi::c_uint && length > numzeros {
                hashpos = *(*hash).chainz.offset(hashpos as isize) as ::core::ffi::c_uint;
                if *(*hash).zeros.offset(hashpos as isize) as ::core::ffi::c_uint != numzeros {
                    break;
                }
            } else {
                hashpos = *(*hash).chain.offset(hashpos as isize) as ::core::ffi::c_uint;
                if *(*hash).val.offset(hashpos as isize) != hashval as ::core::ffi::c_int {
                    break;
                }
            }
        }
        if lazymatching != 0 {
            if lazy == 0
                && length >= 3 as ::core::ffi::c_uint
                && length <= maxlazymatch
                && length < MAX_SUPPORTED_DEFLATE_LENGTH
            {
                lazy = 1 as ::core::ffi::c_uint;
                lazylength = length;
                lazyoffset = offset;
                current_block_78 = 13536709405535804910;
            } else if lazy != 0 {
                lazy = 0 as ::core::ffi::c_uint;
                if pos == 0 as size_t {
                    error = 81 as ::core::ffi::c_uint;
                    break;
                } else if length > lazylength.wrapping_add(1 as ::core::ffi::c_uint) {
                    if uivector_push_back(
                        out,
                        *in_0.offset(pos.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_uint,
                    ) == 0
                    {
                        error = 83 as ::core::ffi::c_uint;
                        break;
                    }
                } else {
                    length = lazylength;
                    offset = lazyoffset;
                    *(*hash).head.offset(hashval as isize) = -(1 as ::core::ffi::c_int);
                    *(*hash).headz.offset(numzeros as isize) = -(1 as ::core::ffi::c_int);
                    pos = pos.wrapping_sub(1);
                }
                current_block_78 = 5028470053297453708;
            } else {
                current_block_78 = 5028470053297453708;
            }
        } else {
            current_block_78 = 5028470053297453708;
        }
        match current_block_78 {
            5028470053297453708 => {
                if length >= 3 as ::core::ffi::c_uint && offset > windowsize {
                    error = 86 as ::core::ffi::c_uint;
                    break;
                } else if length < 3 as ::core::ffi::c_uint {
                    if uivector_push_back(out, *in_0.offset(pos as isize) as ::core::ffi::c_uint)
                        == 0
                    {
                        error = 83 as ::core::ffi::c_uint;
                        break;
                    }
                } else if length < minmatch
                    || length == 3 as ::core::ffi::c_uint && offset > 4096 as ::core::ffi::c_uint
                {
                    if uivector_push_back(out, *in_0.offset(pos as isize) as ::core::ffi::c_uint)
                        == 0
                    {
                        error = 83 as ::core::ffi::c_uint;
                        break;
                    }
                } else {
                    addLengthDistance(out, length as size_t, offset as size_t);
                    i = 1 as ::core::ffi::c_uint;
                    while i < length {
                        pos = pos.wrapping_add(1);
                        wpos = pos & windowsize.wrapping_sub(1 as ::core::ffi::c_uint) as size_t;
                        hashval = getHash(in_0, insize, pos);
                        if usezeros != 0 && hashval == 0 as ::core::ffi::c_uint {
                            if numzeros == 0 as ::core::ffi::c_uint {
                                numzeros = countZeros(in_0, insize, pos);
                            } else if pos.wrapping_add(numzeros as size_t) > insize
                                || *in_0.offset(
                                    pos.wrapping_add(numzeros as size_t)
                                        .wrapping_sub(1 as size_t)
                                        as isize,
                                ) as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                            {
                                numzeros = numzeros.wrapping_sub(1);
                            }
                        } else {
                            numzeros = 0 as ::core::ffi::c_uint;
                        }
                        updateHashChain(hash, wpos, hashval, numzeros as ::core::ffi::c_ushort);
                        i = i.wrapping_add(1);
                    }
                }
            }
            _ => {}
        }
        pos = pos.wrapping_add(1);
    }
    return error;
}
unsafe extern "C" fn deflateNoCompression(
    mut out: *mut ucvector,
    mut data: *const ::core::ffi::c_uchar,
    mut datasize: size_t,
) -> ::core::ffi::c_uint {
    let mut i: size_t = 0;
    let mut numdeflateblocks: size_t = datasize
        .wrapping_add(65534 as size_t)
        .wrapping_div(65535 as size_t);
    let mut datapos: size_t = 0 as size_t;
    i = 0 as size_t;
    while i != numdeflateblocks {
        let mut BFINAL: ::core::ffi::c_uint = 0;
        let mut BTYPE: ::core::ffi::c_uint = 0;
        let mut LEN: ::core::ffi::c_uint = 0;
        let mut NLEN: ::core::ffi::c_uint = 0;
        let mut firstbyte: ::core::ffi::c_uchar = 0;
        let mut pos: size_t = (*out).size;
        BFINAL = (i == numdeflateblocks.wrapping_sub(1 as size_t)) as ::core::ffi::c_int
            as ::core::ffi::c_uint;
        BTYPE = 0 as ::core::ffi::c_uint;
        LEN = 65535 as ::core::ffi::c_uint;
        if datasize.wrapping_sub(datapos) < 65535 as size_t {
            LEN = (datasize as ::core::ffi::c_uint).wrapping_sub(datapos as ::core::ffi::c_uint);
        }
        NLEN = (65535 as ::core::ffi::c_uint).wrapping_sub(LEN);
        if ucvector_resize(
            out,
            (*out)
                .size
                .wrapping_add(LEN as size_t)
                .wrapping_add(5 as size_t),
        ) == 0
        {
            return 83 as ::core::ffi::c_uint;
        }
        firstbyte = BFINAL
            .wrapping_add((BTYPE & 1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_uint)
            .wrapping_add((BTYPE & 2 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_uint)
            as ::core::ffi::c_uchar;
        *(*out).data.offset(pos.wrapping_add(0 as size_t) as isize) = firstbyte;
        *(*out).data.offset(pos.wrapping_add(1 as size_t) as isize) =
            (LEN & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
        *(*out).data.offset(pos.wrapping_add(2 as size_t) as isize) =
            (LEN >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
        *(*out).data.offset(pos.wrapping_add(3 as size_t) as isize) =
            (NLEN & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
        *(*out).data.offset(pos.wrapping_add(4 as size_t) as isize) =
            (NLEN >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
        lodepng_memcpy(
            (*out)
                .data
                .offset(pos as isize)
                .offset(5 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            data.offset(datapos as isize) as *const ::core::ffi::c_void,
            LEN as size_t,
        );
        datapos = (datapos as ::core::ffi::c_ulong).wrapping_add(LEN as ::core::ffi::c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn writeLZ77data(
    mut writer: *mut LodePNGBitWriter,
    mut lz77_encoded: *const uivector,
    mut tree_ll: *const HuffmanTree,
    mut tree_d: *const HuffmanTree,
) {
    let mut i: size_t = 0 as size_t;
    i = 0 as size_t;
    while i != (*lz77_encoded).size {
        let mut val: ::core::ffi::c_uint = *(*lz77_encoded).data.offset(i as isize);
        writeBitsReversed(
            writer,
            *(*tree_ll).codes.offset(val as isize),
            *(*tree_ll).lengths.offset(val as isize) as size_t,
        );
        if val > 256 as ::core::ffi::c_uint {
            let mut length_index: ::core::ffi::c_uint =
                val.wrapping_sub(FIRST_LENGTH_CODE_INDEX as ::core::ffi::c_uint);
            let mut n_length_extra_bits: ::core::ffi::c_uint = LENGTHEXTRA[length_index as usize];
            i = i.wrapping_add(1);
            let mut length_extra_bits: ::core::ffi::c_uint =
                *(*lz77_encoded).data.offset(i as isize);
            i = i.wrapping_add(1);
            let mut distance_code: ::core::ffi::c_uint = *(*lz77_encoded).data.offset(i as isize);
            let mut distance_index: ::core::ffi::c_uint = distance_code;
            let mut n_distance_extra_bits: ::core::ffi::c_uint =
                DISTANCEEXTRA[distance_index as usize];
            i = i.wrapping_add(1);
            let mut distance_extra_bits: ::core::ffi::c_uint =
                *(*lz77_encoded).data.offset(i as isize);
            writeBits(writer, length_extra_bits, n_length_extra_bits as size_t);
            writeBitsReversed(
                writer,
                *(*tree_d).codes.offset(distance_code as isize),
                *(*tree_d).lengths.offset(distance_code as isize) as size_t,
            );
            writeBits(writer, distance_extra_bits, n_distance_extra_bits as size_t);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn deflateDynamic(
    mut writer: *mut LodePNGBitWriter,
    mut hash: *mut Hash,
    mut data: *const ::core::ffi::c_uchar,
    mut datapos: size_t,
    mut dataend: size_t,
    mut settings: *const LodePNGCompressSettings,
    mut final_0: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut lz77_encoded: uivector = uivector {
        data: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        size: 0,
        allocsize: 0,
    };
    let mut tree_ll: HuffmanTree = HuffmanTree {
        codes: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        lengths: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        maxbitlen: 0,
        numcodes: 0,
        table_len: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        table_value: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
    };
    let mut tree_d: HuffmanTree = HuffmanTree {
        codes: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        lengths: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        maxbitlen: 0,
        numcodes: 0,
        table_len: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        table_value: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
    };
    let mut tree_cl: HuffmanTree = HuffmanTree {
        codes: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        lengths: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        maxbitlen: 0,
        numcodes: 0,
        table_len: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        table_value: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
    };
    let mut frequencies_ll: *mut ::core::ffi::c_uint =
        ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut frequencies_d: *mut ::core::ffi::c_uint =
        ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut frequencies_cl: *mut ::core::ffi::c_uint =
        ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut bitlen_lld: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut bitlen_lld_e: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut datasize: size_t = dataend.wrapping_sub(datapos);
    let mut BFINAL: ::core::ffi::c_uint = final_0;
    let mut i: size_t = 0;
    let mut numcodes_ll: size_t = 0;
    let mut numcodes_d: size_t = 0;
    let mut numcodes_lld: size_t = 0;
    let mut numcodes_lld_e: size_t = 0;
    let mut numcodes_cl: size_t = 0;
    let mut HLIT: ::core::ffi::c_uint = 0;
    let mut HDIST: ::core::ffi::c_uint = 0;
    let mut HCLEN: ::core::ffi::c_uint = 0;
    uivector_init(&raw mut lz77_encoded);
    HuffmanTree_init(&raw mut tree_ll);
    HuffmanTree_init(&raw mut tree_d);
    HuffmanTree_init(&raw mut tree_cl);
    frequencies_ll = lodepng_malloc(
        (286 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    frequencies_d = lodepng_malloc(
        (30 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    frequencies_cl = lodepng_malloc(
        (NUM_CODE_LENGTH_CODES as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    if frequencies_ll.is_null() || frequencies_d.is_null() || frequencies_cl.is_null() {
        error = 83 as ::core::ffi::c_uint;
    }
    let mut current_block_113: u64;
    if error == 0 {
        lodepng_memset(
            frequencies_ll as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (286 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
        );
        lodepng_memset(
            frequencies_d as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (30 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
        );
        lodepng_memset(
            frequencies_cl as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (NUM_CODE_LENGTH_CODES as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
        );
        if (*settings).use_lz77 != 0 {
            error = encodeLZ77(
                &raw mut lz77_encoded,
                hash,
                data,
                datapos,
                dataend,
                (*settings).windowsize,
                (*settings).minmatch,
                (*settings).nicematch,
                (*settings).lazymatching,
            );
            if error != 0 {
                current_block_113 = 2652804691515851435;
            } else {
                current_block_113 = 11307063007268554308;
            }
        } else if uivector_resize(&raw mut lz77_encoded, datasize) == 0 {
            error = 83 as ::core::ffi::c_uint;
            current_block_113 = 2652804691515851435;
        } else {
            i = datapos;
            while i < dataend {
                *lz77_encoded.data.offset(i.wrapping_sub(datapos) as isize) =
                    *data.offset(i as isize) as ::core::ffi::c_uint;
                i = i.wrapping_add(1);
            }
            current_block_113 = 11307063007268554308;
        }
        match current_block_113 {
            2652804691515851435 => {}
            _ => {
                i = 0 as size_t;
                while i != lz77_encoded.size {
                    let mut symbol: ::core::ffi::c_uint = *lz77_encoded.data.offset(i as isize);
                    let ref mut fresh60 = *frequencies_ll.offset(symbol as isize);
                    *fresh60 = (*fresh60).wrapping_add(1);
                    if symbol > 256 as ::core::ffi::c_uint {
                        let mut dist: ::core::ffi::c_uint = *lz77_encoded
                            .data
                            .offset(i.wrapping_add(2 as size_t) as isize);
                        let ref mut fresh61 = *frequencies_d.offset(dist as isize);
                        *fresh61 = (*fresh61).wrapping_add(1);
                        i = (i as ::core::ffi::c_ulong).wrapping_add(3 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                    i = i.wrapping_add(1);
                }
                *frequencies_ll.offset(256 as ::core::ffi::c_int as isize) =
                    1 as ::core::ffi::c_uint;
                error = HuffmanTree_makeFromFrequencies(
                    &raw mut tree_ll,
                    frequencies_ll,
                    257 as size_t,
                    286 as size_t,
                    15 as ::core::ffi::c_uint,
                );
                if !(error != 0) {
                    error = HuffmanTree_makeFromFrequencies(
                        &raw mut tree_d,
                        frequencies_d,
                        2 as size_t,
                        30 as size_t,
                        15 as ::core::ffi::c_uint,
                    );
                    if !(error != 0) {
                        numcodes_ll = (if tree_ll.numcodes < 286 as ::core::ffi::c_uint {
                            tree_ll.numcodes
                        } else {
                            286 as ::core::ffi::c_uint
                        }) as size_t;
                        numcodes_d = (if tree_d.numcodes < 30 as ::core::ffi::c_uint {
                            tree_d.numcodes
                        } else {
                            30 as ::core::ffi::c_uint
                        }) as size_t;
                        numcodes_lld = numcodes_ll.wrapping_add(numcodes_d);
                        bitlen_lld = lodepng_malloc(
                            numcodes_lld.wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_uint>() as size_t,
                            ),
                        ) as *mut ::core::ffi::c_uint;
                        bitlen_lld_e = lodepng_malloc(
                            numcodes_lld.wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_uint>() as size_t,
                            ),
                        ) as *mut ::core::ffi::c_uint;
                        if bitlen_lld.is_null() || bitlen_lld_e.is_null() {
                            error = 83 as ::core::ffi::c_uint;
                        } else {
                            numcodes_lld_e = 0 as size_t;
                            i = 0 as size_t;
                            while i != numcodes_ll {
                                *bitlen_lld.offset(i as isize) =
                                    *tree_ll.lengths.offset(i as isize);
                                i = i.wrapping_add(1);
                            }
                            i = 0 as size_t;
                            while i != numcodes_d {
                                *bitlen_lld.offset(numcodes_ll.wrapping_add(i) as isize) =
                                    *tree_d.lengths.offset(i as isize);
                                i = i.wrapping_add(1);
                            }
                            i = 0 as size_t;
                            while i != numcodes_lld {
                                let mut j: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                                while i.wrapping_add(j as size_t).wrapping_add(1 as size_t)
                                    < numcodes_lld
                                    && *bitlen_lld.offset(
                                        i.wrapping_add(j as size_t).wrapping_add(1 as size_t)
                                            as isize,
                                    ) == *bitlen_lld.offset(i as isize)
                                {
                                    j = j.wrapping_add(1);
                                }
                                if *bitlen_lld.offset(i as isize) == 0 as ::core::ffi::c_uint
                                    && j >= 2 as ::core::ffi::c_uint
                                {
                                    j = j.wrapping_add(1);
                                    if j <= 10 as ::core::ffi::c_uint {
                                        let fresh62 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh62 as isize) =
                                            17 as ::core::ffi::c_uint;
                                        let fresh63 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh63 as isize) =
                                            j.wrapping_sub(3 as ::core::ffi::c_uint);
                                    } else {
                                        if j > 138 as ::core::ffi::c_uint {
                                            j = 138 as ::core::ffi::c_uint;
                                        }
                                        let fresh64 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh64 as isize) =
                                            18 as ::core::ffi::c_uint;
                                        let fresh65 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh65 as isize) =
                                            j.wrapping_sub(11 as ::core::ffi::c_uint);
                                    }
                                    i = (i as ::core::ffi::c_ulong)
                                        .wrapping_add(j.wrapping_sub(1 as ::core::ffi::c_uint)
                                            as ::core::ffi::c_ulong)
                                        as size_t as size_t;
                                } else if j >= 3 as ::core::ffi::c_uint {
                                    let mut k: size_t = 0;
                                    let mut num: ::core::ffi::c_uint =
                                        j.wrapping_div(6 as ::core::ffi::c_uint);
                                    let mut rest: ::core::ffi::c_uint =
                                        j.wrapping_rem(6 as ::core::ffi::c_uint);
                                    let fresh66 = numcodes_lld_e;
                                    numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                    *bitlen_lld_e.offset(fresh66 as isize) =
                                        *bitlen_lld.offset(i as isize);
                                    k = 0 as size_t;
                                    while k < num as size_t {
                                        let fresh67 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh67 as isize) =
                                            16 as ::core::ffi::c_uint;
                                        let fresh68 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh68 as isize) =
                                            (6 as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
                                                as ::core::ffi::c_uint;
                                        k = k.wrapping_add(1);
                                    }
                                    if rest >= 3 as ::core::ffi::c_uint {
                                        let fresh69 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh69 as isize) =
                                            16 as ::core::ffi::c_uint;
                                        let fresh70 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh70 as isize) =
                                            rest.wrapping_sub(3 as ::core::ffi::c_uint);
                                    } else {
                                        j = j.wrapping_sub(rest);
                                    }
                                    i = (i as ::core::ffi::c_ulong)
                                        .wrapping_add(j as ::core::ffi::c_ulong)
                                        as size_t as size_t;
                                } else {
                                    let fresh71 = numcodes_lld_e;
                                    numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                    *bitlen_lld_e.offset(fresh71 as isize) =
                                        *bitlen_lld.offset(i as isize);
                                }
                                i = i.wrapping_add(1);
                            }
                            i = 0 as size_t;
                            while i != numcodes_lld_e {
                                let ref mut fresh72 = *frequencies_cl
                                    .offset(*bitlen_lld_e.offset(i as isize) as isize);
                                *fresh72 = (*fresh72).wrapping_add(1);
                                if *bitlen_lld_e.offset(i as isize) >= 16 as ::core::ffi::c_uint {
                                    i = i.wrapping_add(1);
                                }
                                i = i.wrapping_add(1);
                            }
                            error = HuffmanTree_makeFromFrequencies(
                                &raw mut tree_cl,
                                frequencies_cl,
                                NUM_CODE_LENGTH_CODES as size_t,
                                NUM_CODE_LENGTH_CODES as size_t,
                                7 as ::core::ffi::c_uint,
                            );
                            if !(error != 0) {
                                numcodes_cl = NUM_CODE_LENGTH_CODES as size_t;
                                while numcodes_cl > 4 as size_t
                                    && *tree_cl.lengths.offset(
                                        CLCL_ORDER[numcodes_cl.wrapping_sub(1 as size_t) as usize]
                                            as isize,
                                    ) == 0 as ::core::ffi::c_uint
                                {
                                    numcodes_cl = numcodes_cl.wrapping_sub(1);
                                }
                                writeBits(writer, BFINAL, 1 as size_t);
                                writeBits(writer, 0 as ::core::ffi::c_uint, 1 as size_t);
                                writeBits(writer, 1 as ::core::ffi::c_uint, 1 as size_t);
                                HLIT =
                                    numcodes_ll.wrapping_sub(257 as size_t) as ::core::ffi::c_uint;
                                HDIST = numcodes_d.wrapping_sub(1 as size_t) as ::core::ffi::c_uint;
                                HCLEN =
                                    numcodes_cl.wrapping_sub(4 as size_t) as ::core::ffi::c_uint;
                                writeBits(writer, HLIT, 5 as size_t);
                                writeBits(writer, HDIST, 5 as size_t);
                                writeBits(writer, HCLEN, 4 as size_t);
                                i = 0 as size_t;
                                while i != numcodes_cl {
                                    writeBits(
                                        writer,
                                        *tree_cl.lengths.offset(CLCL_ORDER[i as usize] as isize),
                                        3 as size_t,
                                    );
                                    i = i.wrapping_add(1);
                                }
                                i = 0 as size_t;
                                while i != numcodes_lld_e {
                                    writeBitsReversed(
                                        writer,
                                        *tree_cl
                                            .codes
                                            .offset(*bitlen_lld_e.offset(i as isize) as isize),
                                        *tree_cl
                                            .lengths
                                            .offset(*bitlen_lld_e.offset(i as isize) as isize)
                                            as size_t,
                                    );
                                    if *bitlen_lld_e.offset(i as isize) == 16 as ::core::ffi::c_uint
                                    {
                                        i = i.wrapping_add(1);
                                        writeBits(
                                            writer,
                                            *bitlen_lld_e.offset(i as isize),
                                            2 as size_t,
                                        );
                                    } else if *bitlen_lld_e.offset(i as isize)
                                        == 17 as ::core::ffi::c_uint
                                    {
                                        i = i.wrapping_add(1);
                                        writeBits(
                                            writer,
                                            *bitlen_lld_e.offset(i as isize),
                                            3 as size_t,
                                        );
                                    } else if *bitlen_lld_e.offset(i as isize)
                                        == 18 as ::core::ffi::c_uint
                                    {
                                        i = i.wrapping_add(1);
                                        writeBits(
                                            writer,
                                            *bitlen_lld_e.offset(i as isize),
                                            7 as size_t,
                                        );
                                    }
                                    i = i.wrapping_add(1);
                                }
                                writeLZ77data(
                                    writer,
                                    &raw mut lz77_encoded,
                                    &raw mut tree_ll,
                                    &raw mut tree_d,
                                );
                                if *tree_ll.lengths.offset(256 as ::core::ffi::c_int as isize)
                                    == 0 as ::core::ffi::c_uint
                                {
                                    error = 64 as ::core::ffi::c_uint;
                                } else {
                                    writeBitsReversed(
                                        writer,
                                        *tree_ll.codes.offset(256 as ::core::ffi::c_int as isize),
                                        *tree_ll.lengths.offset(256 as ::core::ffi::c_int as isize)
                                            as size_t,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    uivector_cleanup(&raw mut lz77_encoded as *mut ::core::ffi::c_void);
    HuffmanTree_cleanup(&raw mut tree_ll);
    HuffmanTree_cleanup(&raw mut tree_d);
    HuffmanTree_cleanup(&raw mut tree_cl);
    lodepng_free(frequencies_ll as *mut ::core::ffi::c_void);
    lodepng_free(frequencies_d as *mut ::core::ffi::c_void);
    lodepng_free(frequencies_cl as *mut ::core::ffi::c_void);
    lodepng_free(bitlen_lld as *mut ::core::ffi::c_void);
    lodepng_free(bitlen_lld_e as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn deflateFixed(
    mut writer: *mut LodePNGBitWriter,
    mut hash: *mut Hash,
    mut data: *const ::core::ffi::c_uchar,
    mut datapos: size_t,
    mut dataend: size_t,
    mut settings: *const LodePNGCompressSettings,
    mut final_0: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut tree_ll: HuffmanTree = HuffmanTree {
        codes: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        lengths: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        maxbitlen: 0,
        numcodes: 0,
        table_len: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        table_value: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
    };
    let mut tree_d: HuffmanTree = HuffmanTree {
        codes: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        lengths: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        maxbitlen: 0,
        numcodes: 0,
        table_len: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        table_value: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
    };
    let mut BFINAL: ::core::ffi::c_uint = final_0;
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: size_t = 0;
    HuffmanTree_init(&raw mut tree_ll);
    HuffmanTree_init(&raw mut tree_d);
    error = generateFixedLitLenTree(&raw mut tree_ll);
    if error == 0 {
        error = generateFixedDistanceTree(&raw mut tree_d);
    }
    if error == 0 {
        writeBits(writer, BFINAL, 1 as size_t);
        writeBits(writer, 1 as ::core::ffi::c_uint, 1 as size_t);
        writeBits(writer, 0 as ::core::ffi::c_uint, 1 as size_t);
        if (*settings).use_lz77 != 0 {
            let mut lz77_encoded: uivector = uivector {
                data: ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                size: 0,
                allocsize: 0,
            };
            uivector_init(&raw mut lz77_encoded);
            error = encodeLZ77(
                &raw mut lz77_encoded,
                hash,
                data,
                datapos,
                dataend,
                (*settings).windowsize,
                (*settings).minmatch,
                (*settings).nicematch,
                (*settings).lazymatching,
            );
            if error == 0 {
                writeLZ77data(
                    writer,
                    &raw mut lz77_encoded,
                    &raw mut tree_ll,
                    &raw mut tree_d,
                );
            }
            uivector_cleanup(&raw mut lz77_encoded as *mut ::core::ffi::c_void);
        } else {
            i = datapos;
            while i < dataend {
                writeBitsReversed(
                    writer,
                    *tree_ll.codes.offset(*data.offset(i as isize) as isize),
                    *tree_ll.lengths.offset(*data.offset(i as isize) as isize) as size_t,
                );
                i = i.wrapping_add(1);
            }
        }
        if error == 0 {
            writeBitsReversed(
                writer,
                *tree_ll.codes.offset(256 as ::core::ffi::c_int as isize),
                *tree_ll.lengths.offset(256 as ::core::ffi::c_int as isize) as size_t,
            );
        }
    }
    HuffmanTree_cleanup(&raw mut tree_ll);
    HuffmanTree_cleanup(&raw mut tree_d);
    return error;
}
unsafe extern "C" fn lodepng_deflatev(
    mut out: *mut ucvector,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGCompressSettings,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: size_t = 0;
    let mut blocksize: size_t = 0;
    let mut numdeflateblocks: size_t = 0;
    let mut hash: Hash = Hash {
        head: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        chain: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
        val: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        headz: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        chainz: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
        zeros: ::core::ptr::null_mut::<::core::ffi::c_ushort>(),
    };
    let mut writer: LodePNGBitWriter = LodePNGBitWriter {
        data: ::core::ptr::null_mut::<ucvector>(),
        bp: 0,
    };
    LodePNGBitWriter_init(&raw mut writer, out);
    if (*settings).btype > 2 as ::core::ffi::c_uint {
        return 61 as ::core::ffi::c_uint;
    } else if (*settings).btype == 0 as ::core::ffi::c_uint {
        return deflateNoCompression(out, in_0, insize);
    } else if (*settings).btype == 1 as ::core::ffi::c_uint {
        blocksize = insize;
    } else {
        blocksize = insize.wrapping_div(8 as size_t).wrapping_add(8 as size_t);
        if blocksize < 65536 as ::core::ffi::c_int as size_t {
            blocksize = 65536 as ::core::ffi::c_int as size_t;
        }
        if blocksize > 262144 as ::core::ffi::c_int as size_t {
            blocksize = 262144 as ::core::ffi::c_int as size_t;
        }
    }
    numdeflateblocks = insize
        .wrapping_add(blocksize)
        .wrapping_sub(1 as size_t)
        .wrapping_div(blocksize);
    if numdeflateblocks == 0 as size_t {
        numdeflateblocks = 1 as size_t;
    }
    error = hash_init(&raw mut hash, (*settings).windowsize);
    if error == 0 {
        i = 0 as size_t;
        while i != numdeflateblocks && error == 0 {
            let mut final_0: ::core::ffi::c_uint = (i == numdeflateblocks.wrapping_sub(1 as size_t))
                as ::core::ffi::c_int
                as ::core::ffi::c_uint;
            let mut start: size_t = i.wrapping_mul(blocksize);
            let mut end: size_t = start.wrapping_add(blocksize);
            if end > insize {
                end = insize;
            }
            if (*settings).btype == 1 as ::core::ffi::c_uint {
                error = deflateFixed(
                    &raw mut writer,
                    &raw mut hash,
                    in_0,
                    start,
                    end,
                    settings,
                    final_0,
                );
            } else if (*settings).btype == 2 as ::core::ffi::c_uint {
                error = deflateDynamic(
                    &raw mut writer,
                    &raw mut hash,
                    in_0,
                    start,
                    end,
                    settings,
                    final_0,
                );
            }
            i = i.wrapping_add(1);
        }
    }
    hash_cleanup(&raw mut hash);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_deflate(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGCompressSettings,
) -> ::core::ffi::c_uint {
    let mut v: ucvector = ucvector_init(*out, *outsize);
    let mut error: ::core::ffi::c_uint = lodepng_deflatev(&raw mut v, in_0, insize, settings);
    *out = v.data;
    *outsize = v.size;
    return error;
}
unsafe extern "C" fn deflate(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGCompressSettings,
) -> ::core::ffi::c_uint {
    if (*settings).custom_deflate.is_some() {
        let mut error: ::core::ffi::c_uint = (*settings)
            .custom_deflate
            .expect("non-null function pointer")(
            out, outsize, in_0, insize, settings
        );
        return (if error != 0 {
            111 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
    } else {
        return lodepng_deflate(out, outsize, in_0, insize, settings);
    };
}
unsafe extern "C" fn update_adler32(
    mut adler: ::core::ffi::c_uint,
    mut data: *const ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut s1: ::core::ffi::c_uint = adler & 0xffff as ::core::ffi::c_uint;
    let mut s2: ::core::ffi::c_uint =
        adler >> 16 as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint;
    while len != 0 as ::core::ffi::c_uint {
        let mut i: ::core::ffi::c_uint = 0;
        let mut amount: ::core::ffi::c_uint = if len > 5552 as ::core::ffi::c_uint {
            5552 as ::core::ffi::c_uint
        } else {
            len
        };
        len = len.wrapping_sub(amount);
        i = 0 as ::core::ffi::c_uint;
        while i != amount {
            let fresh35 = data;
            data = data.offset(1);
            s1 = s1.wrapping_add(*fresh35 as ::core::ffi::c_uint);
            s2 = s2.wrapping_add(s1);
            i = i.wrapping_add(1);
        }
        s1 = s1.wrapping_rem(65521 as ::core::ffi::c_uint);
        s2 = s2.wrapping_rem(65521 as ::core::ffi::c_uint);
    }
    return s2 << 16 as ::core::ffi::c_uint | s1;
}
unsafe extern "C" fn adler32(
    mut data: *const ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return update_adler32(1 as ::core::ffi::c_uint, data, len);
}
unsafe extern "C" fn lodepng_zlib_decompressv(
    mut out: *mut ucvector,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGDecompressSettings,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut CM: ::core::ffi::c_uint = 0;
    let mut CINFO: ::core::ffi::c_uint = 0;
    let mut FDICT: ::core::ffi::c_uint = 0;
    if insize < 2 as size_t {
        return 53 as ::core::ffi::c_uint;
    }
    if (*in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        * 256 as ::core::ffi::c_int
        + *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        % 31 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        return 24 as ::core::ffi::c_uint;
    }
    CM = (*in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 15 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    CINFO = (*in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int
        & 15 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    FDICT = (*in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        >> 5 as ::core::ffi::c_int
        & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    if CM != 8 as ::core::ffi::c_uint || CINFO > 7 as ::core::ffi::c_uint {
        return 25 as ::core::ffi::c_uint;
    }
    if FDICT != 0 as ::core::ffi::c_uint {
        return 26 as ::core::ffi::c_uint;
    }
    error = inflatev(
        out,
        in_0.offset(2 as ::core::ffi::c_int as isize),
        insize.wrapping_sub(2 as size_t),
        settings,
    );
    if error != 0 {
        return error;
    }
    if (*settings).ignore_adler32 == 0 {
        let mut ADLER32: ::core::ffi::c_uint =
            lodepng_read32bitInt(in_0.offset(insize.wrapping_sub(4 as size_t) as isize)
                as *const ::core::ffi::c_uchar);
        let mut checksum: ::core::ffi::c_uint =
            adler32((*out).data, (*out).size as ::core::ffi::c_uint);
        if checksum != ADLER32 {
            return 58 as ::core::ffi::c_uint;
        }
    }
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_zlib_decompress(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGDecompressSettings,
) -> ::core::ffi::c_uint {
    let mut v: ucvector = ucvector_init(*out, *outsize);
    let mut error: ::core::ffi::c_uint =
        lodepng_zlib_decompressv(&raw mut v, in_0, insize, settings);
    *out = v.data;
    *outsize = v.size;
    return error;
}
unsafe extern "C" fn zlib_decompress(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut expected_size: size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGDecompressSettings,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0;
    if (*settings).custom_zlib.is_some() {
        error = (*settings).custom_zlib.expect("non-null function pointer")(
            out, outsize, in_0, insize, settings,
        );
        if error != 0 {
            error = 110 as ::core::ffi::c_uint;
            if (*settings).max_output_size != 0 && *outsize > (*settings).max_output_size {
                error = 109 as ::core::ffi::c_uint;
            }
        }
    } else {
        let mut v: ucvector = ucvector_init(*out, *outsize);
        if expected_size != 0 {
            ucvector_resize(&raw mut v, (*outsize).wrapping_add(expected_size));
            v.size = *outsize;
        }
        error = lodepng_zlib_decompressv(&raw mut v, in_0, insize, settings);
        *out = v.data;
        *outsize = v.size;
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_zlib_compress(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGCompressSettings,
) -> ::core::ffi::c_uint {
    let mut i: size_t = 0;
    let mut error: ::core::ffi::c_uint = 0;
    let mut deflatedata: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut deflatesize: size_t = 0 as size_t;
    error = deflate(
        &raw mut deflatedata,
        &raw mut deflatesize,
        in_0,
        insize,
        settings,
    );
    *out = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    *outsize = 0 as size_t;
    if error == 0 {
        *outsize = deflatesize.wrapping_add(6 as size_t);
        *out = lodepng_malloc(*outsize) as *mut ::core::ffi::c_uchar;
        if (*out).is_null() {
            error = 83 as ::core::ffi::c_uint;
        }
    }
    if error == 0 {
        let mut ADLER32: ::core::ffi::c_uint = adler32(in_0, insize as ::core::ffi::c_uint);
        let mut CMF: ::core::ffi::c_uint = 120 as ::core::ffi::c_uint;
        let mut FLEVEL: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut FDICT: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut CMFFLG: ::core::ffi::c_uint = (256 as ::core::ffi::c_uint)
            .wrapping_mul(CMF)
            .wrapping_add(FDICT.wrapping_mul(32 as ::core::ffi::c_uint))
            .wrapping_add(FLEVEL.wrapping_mul(64 as ::core::ffi::c_uint));
        let mut FCHECK: ::core::ffi::c_uint = (31 as ::core::ffi::c_uint)
            .wrapping_sub(CMFFLG.wrapping_rem(31 as ::core::ffi::c_uint));
        CMFFLG = CMFFLG.wrapping_add(FCHECK);
        *(*out).offset(0 as ::core::ffi::c_int as isize) =
            (CMFFLG >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *(*out).offset(1 as ::core::ffi::c_int as isize) =
            (CMFFLG & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
        i = 0 as size_t;
        while i != deflatesize {
            *(*out).offset(i.wrapping_add(2 as size_t) as isize) = *deflatedata.offset(i as isize);
            i = i.wrapping_add(1);
        }
        lodepng_set32bitInt(
            (*out).offset((*outsize).wrapping_sub(4 as size_t) as isize)
                as *mut ::core::ffi::c_uchar,
            ADLER32,
        );
    }
    lodepng_free(deflatedata as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn zlib_compress(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut settings: *const LodePNGCompressSettings,
) -> ::core::ffi::c_uint {
    if (*settings).custom_zlib.is_some() {
        let mut error: ::core::ffi::c_uint =
            (*settings).custom_zlib.expect("non-null function pointer")(
                out, outsize, in_0, insize, settings,
            );
        return (if error != 0 {
            111 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
    } else {
        return lodepng_zlib_compress(out, outsize, in_0, insize, settings);
    };
}
pub const DEFAULT_WINDOWSIZE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lodepng_compress_settings_init(
    mut settings: *mut LodePNGCompressSettings,
) {
    (*settings).btype = 2 as ::core::ffi::c_uint;
    (*settings).use_lz77 = 1 as ::core::ffi::c_uint;
    (*settings).windowsize = DEFAULT_WINDOWSIZE as ::core::ffi::c_uint;
    (*settings).minmatch = 3 as ::core::ffi::c_uint;
    (*settings).nicematch = 128 as ::core::ffi::c_uint;
    (*settings).lazymatching = 1 as ::core::ffi::c_uint;
    (*settings).custom_zlib = None;
    (*settings).custom_deflate = None;
    (*settings).custom_context = ::core::ptr::null::<::core::ffi::c_void>();
}
#[no_mangle]
pub static mut lodepng_default_compress_settings: LodePNGCompressSettings =
    LodePNGCompressSettings {
        btype: 2 as ::core::ffi::c_uint,
        use_lz77: 1 as ::core::ffi::c_uint,
        windowsize: DEFAULT_WINDOWSIZE as ::core::ffi::c_uint,
        minmatch: 3 as ::core::ffi::c_uint,
        nicematch: 128 as ::core::ffi::c_uint,
        lazymatching: 1 as ::core::ffi::c_uint,
        custom_zlib: None,
        custom_deflate: None,
        custom_context: ::core::ptr::null::<::core::ffi::c_void>(),
    };
#[no_mangle]
pub unsafe extern "C" fn lodepng_decompress_settings_init(
    mut settings: *mut LodePNGDecompressSettings,
) {
    (*settings).ignore_adler32 = 0 as ::core::ffi::c_uint;
    (*settings).ignore_nlen = 0 as ::core::ffi::c_uint;
    (*settings).max_output_size = 0 as size_t;
    (*settings).custom_zlib = None;
    (*settings).custom_inflate = None;
    (*settings).custom_context = ::core::ptr::null::<::core::ffi::c_void>();
}
#[no_mangle]
pub static mut lodepng_default_decompress_settings: LodePNGDecompressSettings =
    LodePNGDecompressSettings {
        ignore_adler32: 0 as ::core::ffi::c_uint,
        ignore_nlen: 0 as ::core::ffi::c_uint,
        max_output_size: 0 as size_t,
        custom_zlib: None,
        custom_inflate: None,
        custom_context: ::core::ptr::null::<::core::ffi::c_void>(),
    };
static mut lodepng_crc32_table0: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_uint,
    0x77073096 as ::core::ffi::c_uint,
    0xee0e612c as ::core::ffi::c_uint,
    0x990951ba as ::core::ffi::c_uint,
    0x76dc419 as ::core::ffi::c_uint,
    0x706af48f as ::core::ffi::c_uint,
    0xe963a535 as ::core::ffi::c_uint,
    0x9e6495a3 as ::core::ffi::c_uint,
    0xedb8832 as ::core::ffi::c_uint,
    0x79dcb8a4 as ::core::ffi::c_uint,
    0xe0d5e91e as ::core::ffi::c_uint,
    0x97d2d988 as ::core::ffi::c_uint,
    0x9b64c2b as ::core::ffi::c_uint,
    0x7eb17cbd as ::core::ffi::c_uint,
    0xe7b82d07 as ::core::ffi::c_uint,
    0x90bf1d91 as ::core::ffi::c_uint,
    0x1db71064 as ::core::ffi::c_uint,
    0x6ab020f2 as ::core::ffi::c_uint,
    0xf3b97148 as ::core::ffi::c_uint,
    0x84be41de as ::core::ffi::c_uint,
    0x1adad47d as ::core::ffi::c_uint,
    0x6ddde4eb as ::core::ffi::c_uint,
    0xf4d4b551 as ::core::ffi::c_uint,
    0x83d385c7 as ::core::ffi::c_uint,
    0x136c9856 as ::core::ffi::c_uint,
    0x646ba8c0 as ::core::ffi::c_uint,
    0xfd62f97a as ::core::ffi::c_uint,
    0x8a65c9ec as ::core::ffi::c_uint,
    0x14015c4f as ::core::ffi::c_uint,
    0x63066cd9 as ::core::ffi::c_uint,
    0xfa0f3d63 as ::core::ffi::c_uint,
    0x8d080df5 as ::core::ffi::c_uint,
    0x3b6e20c8 as ::core::ffi::c_uint,
    0x4c69105e as ::core::ffi::c_uint,
    0xd56041e4 as ::core::ffi::c_uint,
    0xa2677172 as ::core::ffi::c_uint,
    0x3c03e4d1 as ::core::ffi::c_uint,
    0x4b04d447 as ::core::ffi::c_uint,
    0xd20d85fd as ::core::ffi::c_uint,
    0xa50ab56b as ::core::ffi::c_uint,
    0x35b5a8fa as ::core::ffi::c_uint,
    0x42b2986c as ::core::ffi::c_uint,
    0xdbbbc9d6 as ::core::ffi::c_uint,
    0xacbcf940 as ::core::ffi::c_uint,
    0x32d86ce3 as ::core::ffi::c_uint,
    0x45df5c75 as ::core::ffi::c_uint,
    0xdcd60dcf as ::core::ffi::c_uint,
    0xabd13d59 as ::core::ffi::c_uint,
    0x26d930ac as ::core::ffi::c_uint,
    0x51de003a as ::core::ffi::c_uint,
    0xc8d75180 as ::core::ffi::c_uint,
    0xbfd06116 as ::core::ffi::c_uint,
    0x21b4f4b5 as ::core::ffi::c_uint,
    0x56b3c423 as ::core::ffi::c_uint,
    0xcfba9599 as ::core::ffi::c_uint,
    0xb8bda50f as ::core::ffi::c_uint,
    0x2802b89e as ::core::ffi::c_uint,
    0x5f058808 as ::core::ffi::c_uint,
    0xc60cd9b2 as ::core::ffi::c_uint,
    0xb10be924 as ::core::ffi::c_uint,
    0x2f6f7c87 as ::core::ffi::c_uint,
    0x58684c11 as ::core::ffi::c_uint,
    0xc1611dab as ::core::ffi::c_uint,
    0xb6662d3d as ::core::ffi::c_uint,
    0x76dc4190 as ::core::ffi::c_uint,
    0x1db7106 as ::core::ffi::c_uint,
    0x98d220bc as ::core::ffi::c_uint,
    0xefd5102a as ::core::ffi::c_uint,
    0x71b18589 as ::core::ffi::c_uint,
    0x6b6b51f as ::core::ffi::c_uint,
    0x9fbfe4a5 as ::core::ffi::c_uint,
    0xe8b8d433 as ::core::ffi::c_uint,
    0x7807c9a2 as ::core::ffi::c_uint,
    0xf00f934 as ::core::ffi::c_uint,
    0x9609a88e as ::core::ffi::c_uint,
    0xe10e9818 as ::core::ffi::c_uint,
    0x7f6a0dbb as ::core::ffi::c_uint,
    0x86d3d2d as ::core::ffi::c_uint,
    0x91646c97 as ::core::ffi::c_uint,
    0xe6635c01 as ::core::ffi::c_uint,
    0x6b6b51f4 as ::core::ffi::c_uint,
    0x1c6c6162 as ::core::ffi::c_uint,
    0x856530d8 as ::core::ffi::c_uint,
    0xf262004e as ::core::ffi::c_uint,
    0x6c0695ed as ::core::ffi::c_uint,
    0x1b01a57b as ::core::ffi::c_uint,
    0x8208f4c1 as ::core::ffi::c_uint,
    0xf50fc457 as ::core::ffi::c_uint,
    0x65b0d9c6 as ::core::ffi::c_uint,
    0x12b7e950 as ::core::ffi::c_uint,
    0x8bbeb8ea as ::core::ffi::c_uint,
    0xfcb9887c as ::core::ffi::c_uint,
    0x62dd1ddf as ::core::ffi::c_uint,
    0x15da2d49 as ::core::ffi::c_uint,
    0x8cd37cf3 as ::core::ffi::c_uint,
    0xfbd44c65 as ::core::ffi::c_uint,
    0x4db26158 as ::core::ffi::c_uint,
    0x3ab551ce as ::core::ffi::c_uint,
    0xa3bc0074 as ::core::ffi::c_uint,
    0xd4bb30e2 as ::core::ffi::c_uint,
    0x4adfa541 as ::core::ffi::c_uint,
    0x3dd895d7 as ::core::ffi::c_uint,
    0xa4d1c46d as ::core::ffi::c_uint,
    0xd3d6f4fb as ::core::ffi::c_uint,
    0x4369e96a as ::core::ffi::c_uint,
    0x346ed9fc as ::core::ffi::c_uint,
    0xad678846 as ::core::ffi::c_uint,
    0xda60b8d0 as ::core::ffi::c_uint,
    0x44042d73 as ::core::ffi::c_uint,
    0x33031de5 as ::core::ffi::c_uint,
    0xaa0a4c5f as ::core::ffi::c_uint,
    0xdd0d7cc9 as ::core::ffi::c_uint,
    0x5005713c as ::core::ffi::c_uint,
    0x270241aa as ::core::ffi::c_uint,
    0xbe0b1010 as ::core::ffi::c_uint,
    0xc90c2086 as ::core::ffi::c_uint,
    0x5768b525 as ::core::ffi::c_uint,
    0x206f85b3 as ::core::ffi::c_uint,
    0xb966d409 as ::core::ffi::c_uint,
    0xce61e49f as ::core::ffi::c_uint,
    0x5edef90e as ::core::ffi::c_uint,
    0x29d9c998 as ::core::ffi::c_uint,
    0xb0d09822 as ::core::ffi::c_uint,
    0xc7d7a8b4 as ::core::ffi::c_uint,
    0x59b33d17 as ::core::ffi::c_uint,
    0x2eb40d81 as ::core::ffi::c_uint,
    0xb7bd5c3b as ::core::ffi::c_uint,
    0xc0ba6cad as ::core::ffi::c_uint,
    0xedb88320 as ::core::ffi::c_uint,
    0x9abfb3b6 as ::core::ffi::c_uint,
    0x3b6e20c as ::core::ffi::c_uint,
    0x74b1d29a as ::core::ffi::c_uint,
    0xead54739 as ::core::ffi::c_uint,
    0x9dd277af as ::core::ffi::c_uint,
    0x4db2615 as ::core::ffi::c_uint,
    0x73dc1683 as ::core::ffi::c_uint,
    0xe3630b12 as ::core::ffi::c_uint,
    0x94643b84 as ::core::ffi::c_uint,
    0xd6d6a3e as ::core::ffi::c_uint,
    0x7a6a5aa8 as ::core::ffi::c_uint,
    0xe40ecf0b as ::core::ffi::c_uint,
    0x9309ff9d as ::core::ffi::c_uint,
    0xa00ae27 as ::core::ffi::c_uint,
    0x7d079eb1 as ::core::ffi::c_uint,
    0xf00f9344 as ::core::ffi::c_uint,
    0x8708a3d2 as ::core::ffi::c_uint,
    0x1e01f268 as ::core::ffi::c_uint,
    0x6906c2fe as ::core::ffi::c_uint,
    0xf762575d as ::core::ffi::c_uint,
    0x806567cb as ::core::ffi::c_uint,
    0x196c3671 as ::core::ffi::c_uint,
    0x6e6b06e7 as ::core::ffi::c_uint,
    0xfed41b76 as ::core::ffi::c_uint,
    0x89d32be0 as ::core::ffi::c_uint,
    0x10da7a5a as ::core::ffi::c_uint,
    0x67dd4acc as ::core::ffi::c_uint,
    0xf9b9df6f as ::core::ffi::c_uint,
    0x8ebeeff9 as ::core::ffi::c_uint,
    0x17b7be43 as ::core::ffi::c_uint,
    0x60b08ed5 as ::core::ffi::c_uint,
    0xd6d6a3e8 as ::core::ffi::c_uint,
    0xa1d1937e as ::core::ffi::c_uint,
    0x38d8c2c4 as ::core::ffi::c_uint,
    0x4fdff252 as ::core::ffi::c_uint,
    0xd1bb67f1 as ::core::ffi::c_uint,
    0xa6bc5767 as ::core::ffi::c_uint,
    0x3fb506dd as ::core::ffi::c_uint,
    0x48b2364b as ::core::ffi::c_uint,
    0xd80d2bda as ::core::ffi::c_uint,
    0xaf0a1b4c as ::core::ffi::c_uint,
    0x36034af6 as ::core::ffi::c_uint,
    0x41047a60 as ::core::ffi::c_uint,
    0xdf60efc3 as ::core::ffi::c_uint,
    0xa867df55 as ::core::ffi::c_uint,
    0x316e8eef as ::core::ffi::c_uint,
    0x4669be79 as ::core::ffi::c_uint,
    0xcb61b38c as ::core::ffi::c_uint,
    0xbc66831a as ::core::ffi::c_uint,
    0x256fd2a0 as ::core::ffi::c_uint,
    0x5268e236 as ::core::ffi::c_uint,
    0xcc0c7795 as ::core::ffi::c_uint,
    0xbb0b4703 as ::core::ffi::c_uint,
    0x220216b9 as ::core::ffi::c_uint,
    0x5505262f as ::core::ffi::c_uint,
    0xc5ba3bbe as ::core::ffi::c_uint,
    0xb2bd0b28 as ::core::ffi::c_uint,
    0x2bb45a92 as ::core::ffi::c_uint,
    0x5cb36a04 as ::core::ffi::c_uint,
    0xc2d7ffa7 as ::core::ffi::c_uint,
    0xb5d0cf31 as ::core::ffi::c_uint,
    0x2cd99e8b as ::core::ffi::c_uint,
    0x5bdeae1d as ::core::ffi::c_uint,
    0x9b64c2b0 as ::core::ffi::c_uint,
    0xec63f226 as ::core::ffi::c_uint,
    0x756aa39c as ::core::ffi::c_uint,
    0x26d930a as ::core::ffi::c_uint,
    0x9c0906a9 as ::core::ffi::c_uint,
    0xeb0e363f as ::core::ffi::c_uint,
    0x72076785 as ::core::ffi::c_uint,
    0x5005713 as ::core::ffi::c_uint,
    0x95bf4a82 as ::core::ffi::c_uint,
    0xe2b87a14 as ::core::ffi::c_uint,
    0x7bb12bae as ::core::ffi::c_uint,
    0xcb61b38 as ::core::ffi::c_uint,
    0x92d28e9b as ::core::ffi::c_uint,
    0xe5d5be0d as ::core::ffi::c_uint,
    0x7cdcefb7 as ::core::ffi::c_uint,
    0xbdbdf21 as ::core::ffi::c_uint,
    0x86d3d2d4 as ::core::ffi::c_uint,
    0xf1d4e242 as ::core::ffi::c_uint,
    0x68ddb3f8 as ::core::ffi::c_uint,
    0x1fda836e as ::core::ffi::c_uint,
    0x81be16cd as ::core::ffi::c_uint,
    0xf6b9265b as ::core::ffi::c_uint,
    0x6fb077e1 as ::core::ffi::c_uint,
    0x18b74777 as ::core::ffi::c_uint,
    0x88085ae6 as ::core::ffi::c_uint,
    0xff0f6a70 as ::core::ffi::c_uint,
    0x66063bca as ::core::ffi::c_uint,
    0x11010b5c as ::core::ffi::c_uint,
    0x8f659eff as ::core::ffi::c_uint,
    0xf862ae69 as ::core::ffi::c_uint,
    0x616bffd3 as ::core::ffi::c_uint,
    0x166ccf45 as ::core::ffi::c_uint,
    0xa00ae278 as ::core::ffi::c_uint,
    0xd70dd2ee as ::core::ffi::c_uint,
    0x4e048354 as ::core::ffi::c_uint,
    0x3903b3c2 as ::core::ffi::c_uint,
    0xa7672661 as ::core::ffi::c_uint,
    0xd06016f7 as ::core::ffi::c_uint,
    0x4969474d as ::core::ffi::c_uint,
    0x3e6e77db as ::core::ffi::c_uint,
    0xaed16a4a as ::core::ffi::c_uint,
    0xd9d65adc as ::core::ffi::c_uint,
    0x40df0b66 as ::core::ffi::c_uint,
    0x37d83bf0 as ::core::ffi::c_uint,
    0xa9bcae53 as ::core::ffi::c_uint,
    0xdebb9ec5 as ::core::ffi::c_uint,
    0x47b2cf7f as ::core::ffi::c_uint,
    0x30b5ffe9 as ::core::ffi::c_uint,
    0xbdbdf21c as ::core::ffi::c_uint,
    0xcabac28a as ::core::ffi::c_uint,
    0x53b39330 as ::core::ffi::c_uint,
    0x24b4a3a6 as ::core::ffi::c_uint,
    0xbad03605 as ::core::ffi::c_uint,
    0xcdd70693 as ::core::ffi::c_uint,
    0x54de5729 as ::core::ffi::c_uint,
    0x23d967bf as ::core::ffi::c_uint,
    0xb3667a2e as ::core::ffi::c_uint,
    0xc4614ab8 as ::core::ffi::c_uint,
    0x5d681b02 as ::core::ffi::c_uint,
    0x2a6f2b94 as ::core::ffi::c_uint,
    0xb40bbe37 as ::core::ffi::c_uint,
    0xc30c8ea1 as ::core::ffi::c_uint,
    0x5a05df1b as ::core::ffi::c_uint,
    0x2d02ef8d as ::core::ffi::c_uint,
];
static mut lodepng_crc32_table1: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_uint,
    0x191b3141 as ::core::ffi::c_uint,
    0x32366282 as ::core::ffi::c_uint,
    0x2b2d53c3 as ::core::ffi::c_uint,
    0x646cc504 as ::core::ffi::c_uint,
    0x7d77f445 as ::core::ffi::c_uint,
    0x565aa786 as ::core::ffi::c_uint,
    0x4f4196c7 as ::core::ffi::c_uint,
    0xc8d98a08 as ::core::ffi::c_uint,
    0xd1c2bb49 as ::core::ffi::c_uint,
    0xfaefe88a as ::core::ffi::c_uint,
    0xe3f4d9cb as ::core::ffi::c_uint,
    0xacb54f0c as ::core::ffi::c_uint,
    0xb5ae7e4d as ::core::ffi::c_uint,
    0x9e832d8e as ::core::ffi::c_uint,
    0x87981ccf as ::core::ffi::c_uint,
    0x4ac21251 as ::core::ffi::c_uint,
    0x53d92310 as ::core::ffi::c_uint,
    0x78f470d3 as ::core::ffi::c_uint,
    0x61ef4192 as ::core::ffi::c_uint,
    0x2eaed755 as ::core::ffi::c_uint,
    0x37b5e614 as ::core::ffi::c_uint,
    0x1c98b5d7 as ::core::ffi::c_uint,
    0x5838496 as ::core::ffi::c_uint,
    0x821b9859 as ::core::ffi::c_uint,
    0x9b00a918 as ::core::ffi::c_uint,
    0xb02dfadb as ::core::ffi::c_uint,
    0xa936cb9a as ::core::ffi::c_uint,
    0xe6775d5d as ::core::ffi::c_uint,
    0xff6c6c1c as ::core::ffi::c_uint,
    0xd4413fdf as ::core::ffi::c_uint,
    0xcd5a0e9e as ::core::ffi::c_uint,
    0x958424a2 as ::core::ffi::c_uint,
    0x8c9f15e3 as ::core::ffi::c_uint,
    0xa7b24620 as ::core::ffi::c_uint,
    0xbea97761 as ::core::ffi::c_uint,
    0xf1e8e1a6 as ::core::ffi::c_uint,
    0xe8f3d0e7 as ::core::ffi::c_uint,
    0xc3de8324 as ::core::ffi::c_uint,
    0xdac5b265 as ::core::ffi::c_uint,
    0x5d5daeaa as ::core::ffi::c_uint,
    0x44469feb as ::core::ffi::c_uint,
    0x6f6bcc28 as ::core::ffi::c_uint,
    0x7670fd69 as ::core::ffi::c_uint,
    0x39316bae as ::core::ffi::c_uint,
    0x202a5aef as ::core::ffi::c_uint,
    0xb07092c as ::core::ffi::c_uint,
    0x121c386d as ::core::ffi::c_uint,
    0xdf4636f3 as ::core::ffi::c_uint,
    0xc65d07b2 as ::core::ffi::c_uint,
    0xed705471 as ::core::ffi::c_uint,
    0xf46b6530 as ::core::ffi::c_uint,
    0xbb2af3f7 as ::core::ffi::c_uint,
    0xa231c2b6 as ::core::ffi::c_uint,
    0x891c9175 as ::core::ffi::c_uint,
    0x9007a034 as ::core::ffi::c_uint,
    0x179fbcfb as ::core::ffi::c_uint,
    0xe848dba as ::core::ffi::c_uint,
    0x25a9de79 as ::core::ffi::c_uint,
    0x3cb2ef38 as ::core::ffi::c_uint,
    0x73f379ff as ::core::ffi::c_uint,
    0x6ae848be as ::core::ffi::c_uint,
    0x41c51b7d as ::core::ffi::c_uint,
    0x58de2a3c as ::core::ffi::c_uint,
    0xf0794f05 as ::core::ffi::c_uint,
    0xe9627e44 as ::core::ffi::c_uint,
    0xc24f2d87 as ::core::ffi::c_uint,
    0xdb541cc6 as ::core::ffi::c_uint,
    0x94158a01 as ::core::ffi::c_uint,
    0x8d0ebb40 as ::core::ffi::c_uint,
    0xa623e883 as ::core::ffi::c_uint,
    0xbf38d9c2 as ::core::ffi::c_uint,
    0x38a0c50d as ::core::ffi::c_uint,
    0x21bbf44c as ::core::ffi::c_uint,
    0xa96a78f as ::core::ffi::c_uint,
    0x138d96ce as ::core::ffi::c_uint,
    0x5ccc0009 as ::core::ffi::c_uint,
    0x45d73148 as ::core::ffi::c_uint,
    0x6efa628b as ::core::ffi::c_uint,
    0x77e153ca as ::core::ffi::c_uint,
    0xbabb5d54 as ::core::ffi::c_uint,
    0xa3a06c15 as ::core::ffi::c_uint,
    0x888d3fd6 as ::core::ffi::c_uint,
    0x91960e97 as ::core::ffi::c_uint,
    0xded79850 as ::core::ffi::c_uint,
    0xc7cca911 as ::core::ffi::c_uint,
    0xece1fad2 as ::core::ffi::c_uint,
    0xf5facb93 as ::core::ffi::c_uint,
    0x7262d75c as ::core::ffi::c_uint,
    0x6b79e61d as ::core::ffi::c_uint,
    0x4054b5de as ::core::ffi::c_uint,
    0x594f849f as ::core::ffi::c_uint,
    0x160e1258 as ::core::ffi::c_uint,
    0xf152319 as ::core::ffi::c_uint,
    0x243870da as ::core::ffi::c_uint,
    0x3d23419b as ::core::ffi::c_uint,
    0x65fd6ba7 as ::core::ffi::c_uint,
    0x7ce65ae6 as ::core::ffi::c_uint,
    0x57cb0925 as ::core::ffi::c_uint,
    0x4ed03864 as ::core::ffi::c_uint,
    0x191aea3 as ::core::ffi::c_uint,
    0x188a9fe2 as ::core::ffi::c_uint,
    0x33a7cc21 as ::core::ffi::c_uint,
    0x2abcfd60 as ::core::ffi::c_uint,
    0xad24e1af as ::core::ffi::c_uint,
    0xb43fd0ee as ::core::ffi::c_uint,
    0x9f12832d as ::core::ffi::c_uint,
    0x8609b26c as ::core::ffi::c_uint,
    0xc94824ab as ::core::ffi::c_uint,
    0xd05315ea as ::core::ffi::c_uint,
    0xfb7e4629 as ::core::ffi::c_uint,
    0xe2657768 as ::core::ffi::c_uint,
    0x2f3f79f6 as ::core::ffi::c_uint,
    0x362448b7 as ::core::ffi::c_uint,
    0x1d091b74 as ::core::ffi::c_uint,
    0x4122a35 as ::core::ffi::c_uint,
    0x4b53bcf2 as ::core::ffi::c_uint,
    0x52488db3 as ::core::ffi::c_uint,
    0x7965de70 as ::core::ffi::c_uint,
    0x607eef31 as ::core::ffi::c_uint,
    0xe7e6f3fe as ::core::ffi::c_uint,
    0xfefdc2bf as ::core::ffi::c_uint,
    0xd5d0917c as ::core::ffi::c_uint,
    0xcccba03d as ::core::ffi::c_uint,
    0x838a36fa as ::core::ffi::c_uint,
    0x9a9107bb as ::core::ffi::c_uint,
    0xb1bc5478 as ::core::ffi::c_uint,
    0xa8a76539 as ::core::ffi::c_uint,
    0x3b83984b as ::core::ffi::c_uint,
    0x2298a90a as ::core::ffi::c_uint,
    0x9b5fac9 as ::core::ffi::c_uint,
    0x10aecb88 as ::core::ffi::c_uint,
    0x5fef5d4f as ::core::ffi::c_uint,
    0x46f46c0e as ::core::ffi::c_uint,
    0x6dd93fcd as ::core::ffi::c_uint,
    0x74c20e8c as ::core::ffi::c_uint,
    0xf35a1243 as ::core::ffi::c_uint,
    0xea412302 as ::core::ffi::c_uint,
    0xc16c70c1 as ::core::ffi::c_uint,
    0xd8774180 as ::core::ffi::c_uint,
    0x9736d747 as ::core::ffi::c_uint,
    0x8e2de606 as ::core::ffi::c_uint,
    0xa500b5c5 as ::core::ffi::c_uint,
    0xbc1b8484 as ::core::ffi::c_uint,
    0x71418a1a as ::core::ffi::c_uint,
    0x685abb5b as ::core::ffi::c_uint,
    0x4377e898 as ::core::ffi::c_uint,
    0x5a6cd9d9 as ::core::ffi::c_uint,
    0x152d4f1e as ::core::ffi::c_uint,
    0xc367e5f as ::core::ffi::c_uint,
    0x271b2d9c as ::core::ffi::c_uint,
    0x3e001cdd as ::core::ffi::c_uint,
    0xb9980012 as ::core::ffi::c_uint,
    0xa0833153 as ::core::ffi::c_uint,
    0x8bae6290 as ::core::ffi::c_uint,
    0x92b553d1 as ::core::ffi::c_uint,
    0xddf4c516 as ::core::ffi::c_uint,
    0xc4eff457 as ::core::ffi::c_uint,
    0xefc2a794 as ::core::ffi::c_uint,
    0xf6d996d5 as ::core::ffi::c_uint,
    0xae07bce9 as ::core::ffi::c_uint,
    0xb71c8da8 as ::core::ffi::c_uint,
    0x9c31de6b as ::core::ffi::c_uint,
    0x852aef2a as ::core::ffi::c_uint,
    0xca6b79ed as ::core::ffi::c_uint,
    0xd37048ac as ::core::ffi::c_uint,
    0xf85d1b6f as ::core::ffi::c_uint,
    0xe1462a2e as ::core::ffi::c_uint,
    0x66de36e1 as ::core::ffi::c_uint,
    0x7fc507a0 as ::core::ffi::c_uint,
    0x54e85463 as ::core::ffi::c_uint,
    0x4df36522 as ::core::ffi::c_uint,
    0x2b2f3e5 as ::core::ffi::c_uint,
    0x1ba9c2a4 as ::core::ffi::c_uint,
    0x30849167 as ::core::ffi::c_uint,
    0x299fa026 as ::core::ffi::c_uint,
    0xe4c5aeb8 as ::core::ffi::c_uint,
    0xfdde9ff9 as ::core::ffi::c_uint,
    0xd6f3cc3a as ::core::ffi::c_uint,
    0xcfe8fd7b as ::core::ffi::c_uint,
    0x80a96bbc as ::core::ffi::c_uint,
    0x99b25afd as ::core::ffi::c_uint,
    0xb29f093e as ::core::ffi::c_uint,
    0xab84387f as ::core::ffi::c_uint,
    0x2c1c24b0 as ::core::ffi::c_uint,
    0x350715f1 as ::core::ffi::c_uint,
    0x1e2a4632 as ::core::ffi::c_uint,
    0x7317773 as ::core::ffi::c_uint,
    0x4870e1b4 as ::core::ffi::c_uint,
    0x516bd0f5 as ::core::ffi::c_uint,
    0x7a468336 as ::core::ffi::c_uint,
    0x635db277 as ::core::ffi::c_uint,
    0xcbfad74e as ::core::ffi::c_uint,
    0xd2e1e60f as ::core::ffi::c_uint,
    0xf9ccb5cc as ::core::ffi::c_uint,
    0xe0d7848d as ::core::ffi::c_uint,
    0xaf96124a as ::core::ffi::c_uint,
    0xb68d230b as ::core::ffi::c_uint,
    0x9da070c8 as ::core::ffi::c_uint,
    0x84bb4189 as ::core::ffi::c_uint,
    0x3235d46 as ::core::ffi::c_uint,
    0x1a386c07 as ::core::ffi::c_uint,
    0x31153fc4 as ::core::ffi::c_uint,
    0x280e0e85 as ::core::ffi::c_uint,
    0x674f9842 as ::core::ffi::c_uint,
    0x7e54a903 as ::core::ffi::c_uint,
    0x5579fac0 as ::core::ffi::c_uint,
    0x4c62cb81 as ::core::ffi::c_uint,
    0x8138c51f as ::core::ffi::c_uint,
    0x9823f45e as ::core::ffi::c_uint,
    0xb30ea79d as ::core::ffi::c_uint,
    0xaa1596dc as ::core::ffi::c_uint,
    0xe554001b as ::core::ffi::c_uint,
    0xfc4f315a as ::core::ffi::c_uint,
    0xd7626299 as ::core::ffi::c_uint,
    0xce7953d8 as ::core::ffi::c_uint,
    0x49e14f17 as ::core::ffi::c_uint,
    0x50fa7e56 as ::core::ffi::c_uint,
    0x7bd72d95 as ::core::ffi::c_uint,
    0x62cc1cd4 as ::core::ffi::c_uint,
    0x2d8d8a13 as ::core::ffi::c_uint,
    0x3496bb52 as ::core::ffi::c_uint,
    0x1fbbe891 as ::core::ffi::c_uint,
    0x6a0d9d0 as ::core::ffi::c_uint,
    0x5e7ef3ec as ::core::ffi::c_uint,
    0x4765c2ad as ::core::ffi::c_uint,
    0x6c48916e as ::core::ffi::c_uint,
    0x7553a02f as ::core::ffi::c_uint,
    0x3a1236e8 as ::core::ffi::c_uint,
    0x230907a9 as ::core::ffi::c_uint,
    0x824546a as ::core::ffi::c_uint,
    0x113f652b as ::core::ffi::c_uint,
    0x96a779e4 as ::core::ffi::c_uint,
    0x8fbc48a5 as ::core::ffi::c_uint,
    0xa4911b66 as ::core::ffi::c_uint,
    0xbd8a2a27 as ::core::ffi::c_uint,
    0xf2cbbce0 as ::core::ffi::c_uint,
    0xebd08da1 as ::core::ffi::c_uint,
    0xc0fdde62 as ::core::ffi::c_uint,
    0xd9e6ef23 as ::core::ffi::c_uint,
    0x14bce1bd as ::core::ffi::c_uint,
    0xda7d0fc as ::core::ffi::c_uint,
    0x268a833f as ::core::ffi::c_uint,
    0x3f91b27e as ::core::ffi::c_uint,
    0x70d024b9 as ::core::ffi::c_uint,
    0x69cb15f8 as ::core::ffi::c_uint,
    0x42e6463b as ::core::ffi::c_uint,
    0x5bfd777a as ::core::ffi::c_uint,
    0xdc656bb5 as ::core::ffi::c_uint,
    0xc57e5af4 as ::core::ffi::c_uint,
    0xee530937 as ::core::ffi::c_uint,
    0xf7483876 as ::core::ffi::c_uint,
    0xb809aeb1 as ::core::ffi::c_uint,
    0xa1129ff0 as ::core::ffi::c_uint,
    0x8a3fcc33 as ::core::ffi::c_uint,
    0x9324fd72 as ::core::ffi::c_uint,
];
static mut lodepng_crc32_table2: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_uint,
    0x1c26a37 as ::core::ffi::c_uint,
    0x384d46e as ::core::ffi::c_uint,
    0x246be59 as ::core::ffi::c_uint,
    0x709a8dc as ::core::ffi::c_uint,
    0x6cbc2eb as ::core::ffi::c_uint,
    0x48d7cb2 as ::core::ffi::c_uint,
    0x54f1685 as ::core::ffi::c_uint,
    0xe1351b8 as ::core::ffi::c_uint,
    0xfd13b8f as ::core::ffi::c_uint,
    0xd9785d6 as ::core::ffi::c_uint,
    0xc55efe1 as ::core::ffi::c_uint,
    0x91af964 as ::core::ffi::c_uint,
    0x8d89353 as ::core::ffi::c_uint,
    0xa9e2d0a as ::core::ffi::c_uint,
    0xb5c473d as ::core::ffi::c_uint,
    0x1c26a370 as ::core::ffi::c_uint,
    0x1de4c947 as ::core::ffi::c_uint,
    0x1fa2771e as ::core::ffi::c_uint,
    0x1e601d29 as ::core::ffi::c_uint,
    0x1b2f0bac as ::core::ffi::c_uint,
    0x1aed619b as ::core::ffi::c_uint,
    0x18abdfc2 as ::core::ffi::c_uint,
    0x1969b5f5 as ::core::ffi::c_uint,
    0x1235f2c8 as ::core::ffi::c_uint,
    0x13f798ff as ::core::ffi::c_uint,
    0x11b126a6 as ::core::ffi::c_uint,
    0x10734c91 as ::core::ffi::c_uint,
    0x153c5a14 as ::core::ffi::c_uint,
    0x14fe3023 as ::core::ffi::c_uint,
    0x16b88e7a as ::core::ffi::c_uint,
    0x177ae44d as ::core::ffi::c_uint,
    0x384d46e0 as ::core::ffi::c_uint,
    0x398f2cd7 as ::core::ffi::c_uint,
    0x3bc9928e as ::core::ffi::c_uint,
    0x3a0bf8b9 as ::core::ffi::c_uint,
    0x3f44ee3c as ::core::ffi::c_uint,
    0x3e86840b as ::core::ffi::c_uint,
    0x3cc03a52 as ::core::ffi::c_uint,
    0x3d025065 as ::core::ffi::c_uint,
    0x365e1758 as ::core::ffi::c_uint,
    0x379c7d6f as ::core::ffi::c_uint,
    0x35dac336 as ::core::ffi::c_uint,
    0x3418a901 as ::core::ffi::c_uint,
    0x3157bf84 as ::core::ffi::c_uint,
    0x3095d5b3 as ::core::ffi::c_uint,
    0x32d36bea as ::core::ffi::c_uint,
    0x331101dd as ::core::ffi::c_uint,
    0x246be590 as ::core::ffi::c_uint,
    0x25a98fa7 as ::core::ffi::c_uint,
    0x27ef31fe as ::core::ffi::c_uint,
    0x262d5bc9 as ::core::ffi::c_uint,
    0x23624d4c as ::core::ffi::c_uint,
    0x22a0277b as ::core::ffi::c_uint,
    0x20e69922 as ::core::ffi::c_uint,
    0x2124f315 as ::core::ffi::c_uint,
    0x2a78b428 as ::core::ffi::c_uint,
    0x2bbade1f as ::core::ffi::c_uint,
    0x29fc6046 as ::core::ffi::c_uint,
    0x283e0a71 as ::core::ffi::c_uint,
    0x2d711cf4 as ::core::ffi::c_uint,
    0x2cb376c3 as ::core::ffi::c_uint,
    0x2ef5c89a as ::core::ffi::c_uint,
    0x2f37a2ad as ::core::ffi::c_uint,
    0x709a8dc0 as ::core::ffi::c_uint,
    0x7158e7f7 as ::core::ffi::c_uint,
    0x731e59ae as ::core::ffi::c_uint,
    0x72dc3399 as ::core::ffi::c_uint,
    0x7793251c as ::core::ffi::c_uint,
    0x76514f2b as ::core::ffi::c_uint,
    0x7417f172 as ::core::ffi::c_uint,
    0x75d59b45 as ::core::ffi::c_uint,
    0x7e89dc78 as ::core::ffi::c_uint,
    0x7f4bb64f as ::core::ffi::c_uint,
    0x7d0d0816 as ::core::ffi::c_uint,
    0x7ccf6221 as ::core::ffi::c_uint,
    0x798074a4 as ::core::ffi::c_uint,
    0x78421e93 as ::core::ffi::c_uint,
    0x7a04a0ca as ::core::ffi::c_uint,
    0x7bc6cafd as ::core::ffi::c_uint,
    0x6cbc2eb0 as ::core::ffi::c_uint,
    0x6d7e4487 as ::core::ffi::c_uint,
    0x6f38fade as ::core::ffi::c_uint,
    0x6efa90e9 as ::core::ffi::c_uint,
    0x6bb5866c as ::core::ffi::c_uint,
    0x6a77ec5b as ::core::ffi::c_uint,
    0x68315202 as ::core::ffi::c_uint,
    0x69f33835 as ::core::ffi::c_uint,
    0x62af7f08 as ::core::ffi::c_uint,
    0x636d153f as ::core::ffi::c_uint,
    0x612bab66 as ::core::ffi::c_uint,
    0x60e9c151 as ::core::ffi::c_uint,
    0x65a6d7d4 as ::core::ffi::c_uint,
    0x6464bde3 as ::core::ffi::c_uint,
    0x662203ba as ::core::ffi::c_uint,
    0x67e0698d as ::core::ffi::c_uint,
    0x48d7cb20 as ::core::ffi::c_uint,
    0x4915a117 as ::core::ffi::c_uint,
    0x4b531f4e as ::core::ffi::c_uint,
    0x4a917579 as ::core::ffi::c_uint,
    0x4fde63fc as ::core::ffi::c_uint,
    0x4e1c09cb as ::core::ffi::c_uint,
    0x4c5ab792 as ::core::ffi::c_uint,
    0x4d98dda5 as ::core::ffi::c_uint,
    0x46c49a98 as ::core::ffi::c_uint,
    0x4706f0af as ::core::ffi::c_uint,
    0x45404ef6 as ::core::ffi::c_uint,
    0x448224c1 as ::core::ffi::c_uint,
    0x41cd3244 as ::core::ffi::c_uint,
    0x400f5873 as ::core::ffi::c_uint,
    0x4249e62a as ::core::ffi::c_uint,
    0x438b8c1d as ::core::ffi::c_uint,
    0x54f16850 as ::core::ffi::c_uint,
    0x55330267 as ::core::ffi::c_uint,
    0x5775bc3e as ::core::ffi::c_uint,
    0x56b7d609 as ::core::ffi::c_uint,
    0x53f8c08c as ::core::ffi::c_uint,
    0x523aaabb as ::core::ffi::c_uint,
    0x507c14e2 as ::core::ffi::c_uint,
    0x51be7ed5 as ::core::ffi::c_uint,
    0x5ae239e8 as ::core::ffi::c_uint,
    0x5b2053df as ::core::ffi::c_uint,
    0x5966ed86 as ::core::ffi::c_uint,
    0x58a487b1 as ::core::ffi::c_uint,
    0x5deb9134 as ::core::ffi::c_uint,
    0x5c29fb03 as ::core::ffi::c_uint,
    0x5e6f455a as ::core::ffi::c_uint,
    0x5fad2f6d as ::core::ffi::c_uint,
    0xe1351b80 as ::core::ffi::c_uint,
    0xe0f771b7 as ::core::ffi::c_uint,
    0xe2b1cfee as ::core::ffi::c_uint,
    0xe373a5d9 as ::core::ffi::c_uint,
    0xe63cb35c as ::core::ffi::c_uint,
    0xe7fed96b as ::core::ffi::c_uint,
    0xe5b86732 as ::core::ffi::c_uint,
    0xe47a0d05 as ::core::ffi::c_uint,
    0xef264a38 as ::core::ffi::c_uint,
    0xeee4200f as ::core::ffi::c_uint,
    0xeca29e56 as ::core::ffi::c_uint,
    0xed60f461 as ::core::ffi::c_uint,
    0xe82fe2e4 as ::core::ffi::c_uint,
    0xe9ed88d3 as ::core::ffi::c_uint,
    0xebab368a as ::core::ffi::c_uint,
    0xea695cbd as ::core::ffi::c_uint,
    0xfd13b8f0 as ::core::ffi::c_uint,
    0xfcd1d2c7 as ::core::ffi::c_uint,
    0xfe976c9e as ::core::ffi::c_uint,
    0xff5506a9 as ::core::ffi::c_uint,
    0xfa1a102c as ::core::ffi::c_uint,
    0xfbd87a1b as ::core::ffi::c_uint,
    0xf99ec442 as ::core::ffi::c_uint,
    0xf85cae75 as ::core::ffi::c_uint,
    0xf300e948 as ::core::ffi::c_uint,
    0xf2c2837f as ::core::ffi::c_uint,
    0xf0843d26 as ::core::ffi::c_uint,
    0xf1465711 as ::core::ffi::c_uint,
    0xf4094194 as ::core::ffi::c_uint,
    0xf5cb2ba3 as ::core::ffi::c_uint,
    0xf78d95fa as ::core::ffi::c_uint,
    0xf64fffcd as ::core::ffi::c_uint,
    0xd9785d60 as ::core::ffi::c_uint,
    0xd8ba3757 as ::core::ffi::c_uint,
    0xdafc890e as ::core::ffi::c_uint,
    0xdb3ee339 as ::core::ffi::c_uint,
    0xde71f5bc as ::core::ffi::c_uint,
    0xdfb39f8b as ::core::ffi::c_uint,
    0xddf521d2 as ::core::ffi::c_uint,
    0xdc374be5 as ::core::ffi::c_uint,
    0xd76b0cd8 as ::core::ffi::c_uint,
    0xd6a966ef as ::core::ffi::c_uint,
    0xd4efd8b6 as ::core::ffi::c_uint,
    0xd52db281 as ::core::ffi::c_uint,
    0xd062a404 as ::core::ffi::c_uint,
    0xd1a0ce33 as ::core::ffi::c_uint,
    0xd3e6706a as ::core::ffi::c_uint,
    0xd2241a5d as ::core::ffi::c_uint,
    0xc55efe10 as ::core::ffi::c_uint,
    0xc49c9427 as ::core::ffi::c_uint,
    0xc6da2a7e as ::core::ffi::c_uint,
    0xc7184049 as ::core::ffi::c_uint,
    0xc25756cc as ::core::ffi::c_uint,
    0xc3953cfb as ::core::ffi::c_uint,
    0xc1d382a2 as ::core::ffi::c_uint,
    0xc011e895 as ::core::ffi::c_uint,
    0xcb4dafa8 as ::core::ffi::c_uint,
    0xca8fc59f as ::core::ffi::c_uint,
    0xc8c97bc6 as ::core::ffi::c_uint,
    0xc90b11f1 as ::core::ffi::c_uint,
    0xcc440774 as ::core::ffi::c_uint,
    0xcd866d43 as ::core::ffi::c_uint,
    0xcfc0d31a as ::core::ffi::c_uint,
    0xce02b92d as ::core::ffi::c_uint,
    0x91af9640 as ::core::ffi::c_uint,
    0x906dfc77 as ::core::ffi::c_uint,
    0x922b422e as ::core::ffi::c_uint,
    0x93e92819 as ::core::ffi::c_uint,
    0x96a63e9c as ::core::ffi::c_uint,
    0x976454ab as ::core::ffi::c_uint,
    0x9522eaf2 as ::core::ffi::c_uint,
    0x94e080c5 as ::core::ffi::c_uint,
    0x9fbcc7f8 as ::core::ffi::c_uint,
    0x9e7eadcf as ::core::ffi::c_uint,
    0x9c381396 as ::core::ffi::c_uint,
    0x9dfa79a1 as ::core::ffi::c_uint,
    0x98b56f24 as ::core::ffi::c_uint,
    0x99770513 as ::core::ffi::c_uint,
    0x9b31bb4a as ::core::ffi::c_uint,
    0x9af3d17d as ::core::ffi::c_uint,
    0x8d893530 as ::core::ffi::c_uint,
    0x8c4b5f07 as ::core::ffi::c_uint,
    0x8e0de15e as ::core::ffi::c_uint,
    0x8fcf8b69 as ::core::ffi::c_uint,
    0x8a809dec as ::core::ffi::c_uint,
    0x8b42f7db as ::core::ffi::c_uint,
    0x89044982 as ::core::ffi::c_uint,
    0x88c623b5 as ::core::ffi::c_uint,
    0x839a6488 as ::core::ffi::c_uint,
    0x82580ebf as ::core::ffi::c_uint,
    0x801eb0e6 as ::core::ffi::c_uint,
    0x81dcdad1 as ::core::ffi::c_uint,
    0x8493cc54 as ::core::ffi::c_uint,
    0x8551a663 as ::core::ffi::c_uint,
    0x8717183a as ::core::ffi::c_uint,
    0x86d5720d as ::core::ffi::c_uint,
    0xa9e2d0a0 as ::core::ffi::c_uint,
    0xa820ba97 as ::core::ffi::c_uint,
    0xaa6604ce as ::core::ffi::c_uint,
    0xaba46ef9 as ::core::ffi::c_uint,
    0xaeeb787c as ::core::ffi::c_uint,
    0xaf29124b as ::core::ffi::c_uint,
    0xad6fac12 as ::core::ffi::c_uint,
    0xacadc625 as ::core::ffi::c_uint,
    0xa7f18118 as ::core::ffi::c_uint,
    0xa633eb2f as ::core::ffi::c_uint,
    0xa4755576 as ::core::ffi::c_uint,
    0xa5b73f41 as ::core::ffi::c_uint,
    0xa0f829c4 as ::core::ffi::c_uint,
    0xa13a43f3 as ::core::ffi::c_uint,
    0xa37cfdaa as ::core::ffi::c_uint,
    0xa2be979d as ::core::ffi::c_uint,
    0xb5c473d0 as ::core::ffi::c_uint,
    0xb40619e7 as ::core::ffi::c_uint,
    0xb640a7be as ::core::ffi::c_uint,
    0xb782cd89 as ::core::ffi::c_uint,
    0xb2cddb0c as ::core::ffi::c_uint,
    0xb30fb13b as ::core::ffi::c_uint,
    0xb1490f62 as ::core::ffi::c_uint,
    0xb08b6555 as ::core::ffi::c_uint,
    0xbbd72268 as ::core::ffi::c_uint,
    0xba15485f as ::core::ffi::c_uint,
    0xb853f606 as ::core::ffi::c_uint,
    0xb9919c31 as ::core::ffi::c_uint,
    0xbcde8ab4 as ::core::ffi::c_uint,
    0xbd1ce083 as ::core::ffi::c_uint,
    0xbf5a5eda as ::core::ffi::c_uint,
    0xbe9834ed as ::core::ffi::c_uint,
];
static mut lodepng_crc32_table3: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_uint,
    0xb8bc6765 as ::core::ffi::c_uint,
    0xaa09c88b as ::core::ffi::c_uint,
    0x12b5afee as ::core::ffi::c_uint,
    0x8f629757 as ::core::ffi::c_uint,
    0x37def032 as ::core::ffi::c_uint,
    0x256b5fdc as ::core::ffi::c_uint,
    0x9dd738b9 as ::core::ffi::c_uint,
    0xc5b428ef as ::core::ffi::c_uint,
    0x7d084f8a as ::core::ffi::c_uint,
    0x6fbde064 as ::core::ffi::c_uint,
    0xd7018701 as ::core::ffi::c_uint,
    0x4ad6bfb8 as ::core::ffi::c_uint,
    0xf26ad8dd as ::core::ffi::c_uint,
    0xe0df7733 as ::core::ffi::c_uint,
    0x58631056 as ::core::ffi::c_uint,
    0x5019579f as ::core::ffi::c_uint,
    0xe8a530fa as ::core::ffi::c_uint,
    0xfa109f14 as ::core::ffi::c_uint,
    0x42acf871 as ::core::ffi::c_uint,
    0xdf7bc0c8 as ::core::ffi::c_uint,
    0x67c7a7ad as ::core::ffi::c_uint,
    0x75720843 as ::core::ffi::c_uint,
    0xcdce6f26 as ::core::ffi::c_uint,
    0x95ad7f70 as ::core::ffi::c_uint,
    0x2d111815 as ::core::ffi::c_uint,
    0x3fa4b7fb as ::core::ffi::c_uint,
    0x8718d09e as ::core::ffi::c_uint,
    0x1acfe827 as ::core::ffi::c_uint,
    0xa2738f42 as ::core::ffi::c_uint,
    0xb0c620ac as ::core::ffi::c_uint,
    0x87a47c9 as ::core::ffi::c_uint,
    0xa032af3e as ::core::ffi::c_uint,
    0x188ec85b as ::core::ffi::c_uint,
    0xa3b67b5 as ::core::ffi::c_uint,
    0xb28700d0 as ::core::ffi::c_uint,
    0x2f503869 as ::core::ffi::c_uint,
    0x97ec5f0c as ::core::ffi::c_uint,
    0x8559f0e2 as ::core::ffi::c_uint,
    0x3de59787 as ::core::ffi::c_uint,
    0x658687d1 as ::core::ffi::c_uint,
    0xdd3ae0b4 as ::core::ffi::c_uint,
    0xcf8f4f5a as ::core::ffi::c_uint,
    0x7733283f as ::core::ffi::c_uint,
    0xeae41086 as ::core::ffi::c_uint,
    0x525877e3 as ::core::ffi::c_uint,
    0x40edd80d as ::core::ffi::c_uint,
    0xf851bf68 as ::core::ffi::c_uint,
    0xf02bf8a1 as ::core::ffi::c_uint,
    0x48979fc4 as ::core::ffi::c_uint,
    0x5a22302a as ::core::ffi::c_uint,
    0xe29e574f as ::core::ffi::c_uint,
    0x7f496ff6 as ::core::ffi::c_uint,
    0xc7f50893 as ::core::ffi::c_uint,
    0xd540a77d as ::core::ffi::c_uint,
    0x6dfcc018 as ::core::ffi::c_uint,
    0x359fd04e as ::core::ffi::c_uint,
    0x8d23b72b as ::core::ffi::c_uint,
    0x9f9618c5 as ::core::ffi::c_uint,
    0x272a7fa0 as ::core::ffi::c_uint,
    0xbafd4719 as ::core::ffi::c_uint,
    0x241207c as ::core::ffi::c_uint,
    0x10f48f92 as ::core::ffi::c_uint,
    0xa848e8f7 as ::core::ffi::c_uint,
    0x9b14583d as ::core::ffi::c_uint,
    0x23a83f58 as ::core::ffi::c_uint,
    0x311d90b6 as ::core::ffi::c_uint,
    0x89a1f7d3 as ::core::ffi::c_uint,
    0x1476cf6a as ::core::ffi::c_uint,
    0xaccaa80f as ::core::ffi::c_uint,
    0xbe7f07e1 as ::core::ffi::c_uint,
    0x6c36084 as ::core::ffi::c_uint,
    0x5ea070d2 as ::core::ffi::c_uint,
    0xe61c17b7 as ::core::ffi::c_uint,
    0xf4a9b859 as ::core::ffi::c_uint,
    0x4c15df3c as ::core::ffi::c_uint,
    0xd1c2e785 as ::core::ffi::c_uint,
    0x697e80e0 as ::core::ffi::c_uint,
    0x7bcb2f0e as ::core::ffi::c_uint,
    0xc377486b as ::core::ffi::c_uint,
    0xcb0d0fa2 as ::core::ffi::c_uint,
    0x73b168c7 as ::core::ffi::c_uint,
    0x6104c729 as ::core::ffi::c_uint,
    0xd9b8a04c as ::core::ffi::c_uint,
    0x446f98f5 as ::core::ffi::c_uint,
    0xfcd3ff90 as ::core::ffi::c_uint,
    0xee66507e as ::core::ffi::c_uint,
    0x56da371b as ::core::ffi::c_uint,
    0xeb9274d as ::core::ffi::c_uint,
    0xb6054028 as ::core::ffi::c_uint,
    0xa4b0efc6 as ::core::ffi::c_uint,
    0x1c0c88a3 as ::core::ffi::c_uint,
    0x81dbb01a as ::core::ffi::c_uint,
    0x3967d77f as ::core::ffi::c_uint,
    0x2bd27891 as ::core::ffi::c_uint,
    0x936e1ff4 as ::core::ffi::c_uint,
    0x3b26f703 as ::core::ffi::c_uint,
    0x839a9066 as ::core::ffi::c_uint,
    0x912f3f88 as ::core::ffi::c_uint,
    0x299358ed as ::core::ffi::c_uint,
    0xb4446054 as ::core::ffi::c_uint,
    0xcf80731 as ::core::ffi::c_uint,
    0x1e4da8df as ::core::ffi::c_uint,
    0xa6f1cfba as ::core::ffi::c_uint,
    0xfe92dfec as ::core::ffi::c_uint,
    0x462eb889 as ::core::ffi::c_uint,
    0x549b1767 as ::core::ffi::c_uint,
    0xec277002 as ::core::ffi::c_uint,
    0x71f048bb as ::core::ffi::c_uint,
    0xc94c2fde as ::core::ffi::c_uint,
    0xdbf98030 as ::core::ffi::c_uint,
    0x6345e755 as ::core::ffi::c_uint,
    0x6b3fa09c as ::core::ffi::c_uint,
    0xd383c7f9 as ::core::ffi::c_uint,
    0xc1366817 as ::core::ffi::c_uint,
    0x798a0f72 as ::core::ffi::c_uint,
    0xe45d37cb as ::core::ffi::c_uint,
    0x5ce150ae as ::core::ffi::c_uint,
    0x4e54ff40 as ::core::ffi::c_uint,
    0xf6e89825 as ::core::ffi::c_uint,
    0xae8b8873 as ::core::ffi::c_uint,
    0x1637ef16 as ::core::ffi::c_uint,
    0x48240f8 as ::core::ffi::c_uint,
    0xbc3e279d as ::core::ffi::c_uint,
    0x21e91f24 as ::core::ffi::c_uint,
    0x99557841 as ::core::ffi::c_uint,
    0x8be0d7af as ::core::ffi::c_uint,
    0x335cb0ca as ::core::ffi::c_uint,
    0xed59b63b as ::core::ffi::c_uint,
    0x55e5d15e as ::core::ffi::c_uint,
    0x47507eb0 as ::core::ffi::c_uint,
    0xffec19d5 as ::core::ffi::c_uint,
    0x623b216c as ::core::ffi::c_uint,
    0xda874609 as ::core::ffi::c_uint,
    0xc832e9e7 as ::core::ffi::c_uint,
    0x708e8e82 as ::core::ffi::c_uint,
    0x28ed9ed4 as ::core::ffi::c_uint,
    0x9051f9b1 as ::core::ffi::c_uint,
    0x82e4565f as ::core::ffi::c_uint,
    0x3a58313a as ::core::ffi::c_uint,
    0xa78f0983 as ::core::ffi::c_uint,
    0x1f336ee6 as ::core::ffi::c_uint,
    0xd86c108 as ::core::ffi::c_uint,
    0xb53aa66d as ::core::ffi::c_uint,
    0xbd40e1a4 as ::core::ffi::c_uint,
    0x5fc86c1 as ::core::ffi::c_uint,
    0x1749292f as ::core::ffi::c_uint,
    0xaff54e4a as ::core::ffi::c_uint,
    0x322276f3 as ::core::ffi::c_uint,
    0x8a9e1196 as ::core::ffi::c_uint,
    0x982bbe78 as ::core::ffi::c_uint,
    0x2097d91d as ::core::ffi::c_uint,
    0x78f4c94b as ::core::ffi::c_uint,
    0xc048ae2e as ::core::ffi::c_uint,
    0xd2fd01c0 as ::core::ffi::c_uint,
    0x6a4166a5 as ::core::ffi::c_uint,
    0xf7965e1c as ::core::ffi::c_uint,
    0x4f2a3979 as ::core::ffi::c_uint,
    0x5d9f9697 as ::core::ffi::c_uint,
    0xe523f1f2 as ::core::ffi::c_uint,
    0x4d6b1905 as ::core::ffi::c_uint,
    0xf5d77e60 as ::core::ffi::c_uint,
    0xe762d18e as ::core::ffi::c_uint,
    0x5fdeb6eb as ::core::ffi::c_uint,
    0xc2098e52 as ::core::ffi::c_uint,
    0x7ab5e937 as ::core::ffi::c_uint,
    0x680046d9 as ::core::ffi::c_uint,
    0xd0bc21bc as ::core::ffi::c_uint,
    0x88df31ea as ::core::ffi::c_uint,
    0x3063568f as ::core::ffi::c_uint,
    0x22d6f961 as ::core::ffi::c_uint,
    0x9a6a9e04 as ::core::ffi::c_uint,
    0x7bda6bd as ::core::ffi::c_uint,
    0xbf01c1d8 as ::core::ffi::c_uint,
    0xadb46e36 as ::core::ffi::c_uint,
    0x15080953 as ::core::ffi::c_uint,
    0x1d724e9a as ::core::ffi::c_uint,
    0xa5ce29ff as ::core::ffi::c_uint,
    0xb77b8611 as ::core::ffi::c_uint,
    0xfc7e174 as ::core::ffi::c_uint,
    0x9210d9cd as ::core::ffi::c_uint,
    0x2aacbea8 as ::core::ffi::c_uint,
    0x38191146 as ::core::ffi::c_uint,
    0x80a57623 as ::core::ffi::c_uint,
    0xd8c66675 as ::core::ffi::c_uint,
    0x607a0110 as ::core::ffi::c_uint,
    0x72cfaefe as ::core::ffi::c_uint,
    0xca73c99b as ::core::ffi::c_uint,
    0x57a4f122 as ::core::ffi::c_uint,
    0xef189647 as ::core::ffi::c_uint,
    0xfdad39a9 as ::core::ffi::c_uint,
    0x45115ecc as ::core::ffi::c_uint,
    0x764dee06 as ::core::ffi::c_uint,
    0xcef18963 as ::core::ffi::c_uint,
    0xdc44268d as ::core::ffi::c_uint,
    0x64f841e8 as ::core::ffi::c_uint,
    0xf92f7951 as ::core::ffi::c_uint,
    0x41931e34 as ::core::ffi::c_uint,
    0x5326b1da as ::core::ffi::c_uint,
    0xeb9ad6bf as ::core::ffi::c_uint,
    0xb3f9c6e9 as ::core::ffi::c_uint,
    0xb45a18c as ::core::ffi::c_uint,
    0x19f00e62 as ::core::ffi::c_uint,
    0xa14c6907 as ::core::ffi::c_uint,
    0x3c9b51be as ::core::ffi::c_uint,
    0x842736db as ::core::ffi::c_uint,
    0x96929935 as ::core::ffi::c_uint,
    0x2e2efe50 as ::core::ffi::c_uint,
    0x2654b999 as ::core::ffi::c_uint,
    0x9ee8defc as ::core::ffi::c_uint,
    0x8c5d7112 as ::core::ffi::c_uint,
    0x34e11677 as ::core::ffi::c_uint,
    0xa9362ece as ::core::ffi::c_uint,
    0x118a49ab as ::core::ffi::c_uint,
    0x33fe645 as ::core::ffi::c_uint,
    0xbb838120 as ::core::ffi::c_uint,
    0xe3e09176 as ::core::ffi::c_uint,
    0x5b5cf613 as ::core::ffi::c_uint,
    0x49e959fd as ::core::ffi::c_uint,
    0xf1553e98 as ::core::ffi::c_uint,
    0x6c820621 as ::core::ffi::c_uint,
    0xd43e6144 as ::core::ffi::c_uint,
    0xc68bceaa as ::core::ffi::c_uint,
    0x7e37a9cf as ::core::ffi::c_uint,
    0xd67f4138 as ::core::ffi::c_uint,
    0x6ec3265d as ::core::ffi::c_uint,
    0x7c7689b3 as ::core::ffi::c_uint,
    0xc4caeed6 as ::core::ffi::c_uint,
    0x591dd66f as ::core::ffi::c_uint,
    0xe1a1b10a as ::core::ffi::c_uint,
    0xf3141ee4 as ::core::ffi::c_uint,
    0x4ba87981 as ::core::ffi::c_uint,
    0x13cb69d7 as ::core::ffi::c_uint,
    0xab770eb2 as ::core::ffi::c_uint,
    0xb9c2a15c as ::core::ffi::c_uint,
    0x17ec639 as ::core::ffi::c_uint,
    0x9ca9fe80 as ::core::ffi::c_uint,
    0x241599e5 as ::core::ffi::c_uint,
    0x36a0360b as ::core::ffi::c_uint,
    0x8e1c516e as ::core::ffi::c_uint,
    0x866616a7 as ::core::ffi::c_uint,
    0x3eda71c2 as ::core::ffi::c_uint,
    0x2c6fde2c as ::core::ffi::c_uint,
    0x94d3b949 as ::core::ffi::c_uint,
    0x90481f0 as ::core::ffi::c_uint,
    0xb1b8e695 as ::core::ffi::c_uint,
    0xa30d497b as ::core::ffi::c_uint,
    0x1bb12e1e as ::core::ffi::c_uint,
    0x43d23e48 as ::core::ffi::c_uint,
    0xfb6e592d as ::core::ffi::c_uint,
    0xe9dbf6c3 as ::core::ffi::c_uint,
    0x516791a6 as ::core::ffi::c_uint,
    0xccb0a91f as ::core::ffi::c_uint,
    0x740cce7a as ::core::ffi::c_uint,
    0x66b96194 as ::core::ffi::c_uint,
    0xde0506f1 as ::core::ffi::c_uint,
];
static mut lodepng_crc32_table4: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_uint,
    0x3d6029b0 as ::core::ffi::c_uint,
    0x7ac05360 as ::core::ffi::c_uint,
    0x47a07ad0 as ::core::ffi::c_uint,
    0xf580a6c0 as ::core::ffi::c_uint,
    0xc8e08f70 as ::core::ffi::c_uint,
    0x8f40f5a0 as ::core::ffi::c_uint,
    0xb220dc10 as ::core::ffi::c_uint,
    0x30704bc1 as ::core::ffi::c_uint,
    0xd106271 as ::core::ffi::c_uint,
    0x4ab018a1 as ::core::ffi::c_uint,
    0x77d03111 as ::core::ffi::c_uint,
    0xc5f0ed01 as ::core::ffi::c_uint,
    0xf890c4b1 as ::core::ffi::c_uint,
    0xbf30be61 as ::core::ffi::c_uint,
    0x825097d1 as ::core::ffi::c_uint,
    0x60e09782 as ::core::ffi::c_uint,
    0x5d80be32 as ::core::ffi::c_uint,
    0x1a20c4e2 as ::core::ffi::c_uint,
    0x2740ed52 as ::core::ffi::c_uint,
    0x95603142 as ::core::ffi::c_uint,
    0xa80018f2 as ::core::ffi::c_uint,
    0xefa06222 as ::core::ffi::c_uint,
    0xd2c04b92 as ::core::ffi::c_uint,
    0x5090dc43 as ::core::ffi::c_uint,
    0x6df0f5f3 as ::core::ffi::c_uint,
    0x2a508f23 as ::core::ffi::c_uint,
    0x1730a693 as ::core::ffi::c_uint,
    0xa5107a83 as ::core::ffi::c_uint,
    0x98705333 as ::core::ffi::c_uint,
    0xdfd029e3 as ::core::ffi::c_uint,
    0xe2b00053 as ::core::ffi::c_uint,
    0xc1c12f04 as ::core::ffi::c_uint,
    0xfca106b4 as ::core::ffi::c_uint,
    0xbb017c64 as ::core::ffi::c_uint,
    0x866155d4 as ::core::ffi::c_uint,
    0x344189c4 as ::core::ffi::c_uint,
    0x921a074 as ::core::ffi::c_uint,
    0x4e81daa4 as ::core::ffi::c_uint,
    0x73e1f314 as ::core::ffi::c_uint,
    0xf1b164c5 as ::core::ffi::c_uint,
    0xccd14d75 as ::core::ffi::c_uint,
    0x8b7137a5 as ::core::ffi::c_uint,
    0xb6111e15 as ::core::ffi::c_uint,
    0x431c205 as ::core::ffi::c_uint,
    0x3951ebb5 as ::core::ffi::c_uint,
    0x7ef19165 as ::core::ffi::c_uint,
    0x4391b8d5 as ::core::ffi::c_uint,
    0xa121b886 as ::core::ffi::c_uint,
    0x9c419136 as ::core::ffi::c_uint,
    0xdbe1ebe6 as ::core::ffi::c_uint,
    0xe681c256 as ::core::ffi::c_uint,
    0x54a11e46 as ::core::ffi::c_uint,
    0x69c137f6 as ::core::ffi::c_uint,
    0x2e614d26 as ::core::ffi::c_uint,
    0x13016496 as ::core::ffi::c_uint,
    0x9151f347 as ::core::ffi::c_uint,
    0xac31daf7 as ::core::ffi::c_uint,
    0xeb91a027 as ::core::ffi::c_uint,
    0xd6f18997 as ::core::ffi::c_uint,
    0x64d15587 as ::core::ffi::c_uint,
    0x59b17c37 as ::core::ffi::c_uint,
    0x1e1106e7 as ::core::ffi::c_uint,
    0x23712f57 as ::core::ffi::c_uint,
    0x58f35849 as ::core::ffi::c_uint,
    0x659371f9 as ::core::ffi::c_uint,
    0x22330b29 as ::core::ffi::c_uint,
    0x1f532299 as ::core::ffi::c_uint,
    0xad73fe89 as ::core::ffi::c_uint,
    0x9013d739 as ::core::ffi::c_uint,
    0xd7b3ade9 as ::core::ffi::c_uint,
    0xead38459 as ::core::ffi::c_uint,
    0x68831388 as ::core::ffi::c_uint,
    0x55e33a38 as ::core::ffi::c_uint,
    0x124340e8 as ::core::ffi::c_uint,
    0x2f236958 as ::core::ffi::c_uint,
    0x9d03b548 as ::core::ffi::c_uint,
    0xa0639cf8 as ::core::ffi::c_uint,
    0xe7c3e628 as ::core::ffi::c_uint,
    0xdaa3cf98 as ::core::ffi::c_uint,
    0x3813cfcb as ::core::ffi::c_uint,
    0x573e67b as ::core::ffi::c_uint,
    0x42d39cab as ::core::ffi::c_uint,
    0x7fb3b51b as ::core::ffi::c_uint,
    0xcd93690b as ::core::ffi::c_uint,
    0xf0f340bb as ::core::ffi::c_uint,
    0xb7533a6b as ::core::ffi::c_uint,
    0x8a3313db as ::core::ffi::c_uint,
    0x863840a as ::core::ffi::c_uint,
    0x3503adba as ::core::ffi::c_uint,
    0x72a3d76a as ::core::ffi::c_uint,
    0x4fc3feda as ::core::ffi::c_uint,
    0xfde322ca as ::core::ffi::c_uint,
    0xc0830b7a as ::core::ffi::c_uint,
    0x872371aa as ::core::ffi::c_uint,
    0xba43581a as ::core::ffi::c_uint,
    0x9932774d as ::core::ffi::c_uint,
    0xa4525efd as ::core::ffi::c_uint,
    0xe3f2242d as ::core::ffi::c_uint,
    0xde920d9d as ::core::ffi::c_uint,
    0x6cb2d18d as ::core::ffi::c_uint,
    0x51d2f83d as ::core::ffi::c_uint,
    0x167282ed as ::core::ffi::c_uint,
    0x2b12ab5d as ::core::ffi::c_uint,
    0xa9423c8c as ::core::ffi::c_uint,
    0x9422153c as ::core::ffi::c_uint,
    0xd3826fec as ::core::ffi::c_uint,
    0xeee2465c as ::core::ffi::c_uint,
    0x5cc29a4c as ::core::ffi::c_uint,
    0x61a2b3fc as ::core::ffi::c_uint,
    0x2602c92c as ::core::ffi::c_uint,
    0x1b62e09c as ::core::ffi::c_uint,
    0xf9d2e0cf as ::core::ffi::c_uint,
    0xc4b2c97f as ::core::ffi::c_uint,
    0x8312b3af as ::core::ffi::c_uint,
    0xbe729a1f as ::core::ffi::c_uint,
    0xc52460f as ::core::ffi::c_uint,
    0x31326fbf as ::core::ffi::c_uint,
    0x7692156f as ::core::ffi::c_uint,
    0x4bf23cdf as ::core::ffi::c_uint,
    0xc9a2ab0e as ::core::ffi::c_uint,
    0xf4c282be as ::core::ffi::c_uint,
    0xb362f86e as ::core::ffi::c_uint,
    0x8e02d1de as ::core::ffi::c_uint,
    0x3c220dce as ::core::ffi::c_uint,
    0x142247e as ::core::ffi::c_uint,
    0x46e25eae as ::core::ffi::c_uint,
    0x7b82771e as ::core::ffi::c_uint,
    0xb1e6b092 as ::core::ffi::c_uint,
    0x8c869922 as ::core::ffi::c_uint,
    0xcb26e3f2 as ::core::ffi::c_uint,
    0xf646ca42 as ::core::ffi::c_uint,
    0x44661652 as ::core::ffi::c_uint,
    0x79063fe2 as ::core::ffi::c_uint,
    0x3ea64532 as ::core::ffi::c_uint,
    0x3c66c82 as ::core::ffi::c_uint,
    0x8196fb53 as ::core::ffi::c_uint,
    0xbcf6d2e3 as ::core::ffi::c_uint,
    0xfb56a833 as ::core::ffi::c_uint,
    0xc6368183 as ::core::ffi::c_uint,
    0x74165d93 as ::core::ffi::c_uint,
    0x49767423 as ::core::ffi::c_uint,
    0xed60ef3 as ::core::ffi::c_uint,
    0x33b62743 as ::core::ffi::c_uint,
    0xd1062710 as ::core::ffi::c_uint,
    0xec660ea0 as ::core::ffi::c_uint,
    0xabc67470 as ::core::ffi::c_uint,
    0x96a65dc0 as ::core::ffi::c_uint,
    0x248681d0 as ::core::ffi::c_uint,
    0x19e6a860 as ::core::ffi::c_uint,
    0x5e46d2b0 as ::core::ffi::c_uint,
    0x6326fb00 as ::core::ffi::c_uint,
    0xe1766cd1 as ::core::ffi::c_uint,
    0xdc164561 as ::core::ffi::c_uint,
    0x9bb63fb1 as ::core::ffi::c_uint,
    0xa6d61601 as ::core::ffi::c_uint,
    0x14f6ca11 as ::core::ffi::c_uint,
    0x2996e3a1 as ::core::ffi::c_uint,
    0x6e369971 as ::core::ffi::c_uint,
    0x5356b0c1 as ::core::ffi::c_uint,
    0x70279f96 as ::core::ffi::c_uint,
    0x4d47b626 as ::core::ffi::c_uint,
    0xae7ccf6 as ::core::ffi::c_uint,
    0x3787e546 as ::core::ffi::c_uint,
    0x85a73956 as ::core::ffi::c_uint,
    0xb8c710e6 as ::core::ffi::c_uint,
    0xff676a36 as ::core::ffi::c_uint,
    0xc2074386 as ::core::ffi::c_uint,
    0x4057d457 as ::core::ffi::c_uint,
    0x7d37fde7 as ::core::ffi::c_uint,
    0x3a978737 as ::core::ffi::c_uint,
    0x7f7ae87 as ::core::ffi::c_uint,
    0xb5d77297 as ::core::ffi::c_uint,
    0x88b75b27 as ::core::ffi::c_uint,
    0xcf1721f7 as ::core::ffi::c_uint,
    0xf2770847 as ::core::ffi::c_uint,
    0x10c70814 as ::core::ffi::c_uint,
    0x2da721a4 as ::core::ffi::c_uint,
    0x6a075b74 as ::core::ffi::c_uint,
    0x576772c4 as ::core::ffi::c_uint,
    0xe547aed4 as ::core::ffi::c_uint,
    0xd8278764 as ::core::ffi::c_uint,
    0x9f87fdb4 as ::core::ffi::c_uint,
    0xa2e7d404 as ::core::ffi::c_uint,
    0x20b743d5 as ::core::ffi::c_uint,
    0x1dd76a65 as ::core::ffi::c_uint,
    0x5a7710b5 as ::core::ffi::c_uint,
    0x67173905 as ::core::ffi::c_uint,
    0xd537e515 as ::core::ffi::c_uint,
    0xe857cca5 as ::core::ffi::c_uint,
    0xaff7b675 as ::core::ffi::c_uint,
    0x92979fc5 as ::core::ffi::c_uint,
    0xe915e8db as ::core::ffi::c_uint,
    0xd475c16b as ::core::ffi::c_uint,
    0x93d5bbbb as ::core::ffi::c_uint,
    0xaeb5920b as ::core::ffi::c_uint,
    0x1c954e1b as ::core::ffi::c_uint,
    0x21f567ab as ::core::ffi::c_uint,
    0x66551d7b as ::core::ffi::c_uint,
    0x5b3534cb as ::core::ffi::c_uint,
    0xd965a31a as ::core::ffi::c_uint,
    0xe4058aaa as ::core::ffi::c_uint,
    0xa3a5f07a as ::core::ffi::c_uint,
    0x9ec5d9ca as ::core::ffi::c_uint,
    0x2ce505da as ::core::ffi::c_uint,
    0x11852c6a as ::core::ffi::c_uint,
    0x562556ba as ::core::ffi::c_uint,
    0x6b457f0a as ::core::ffi::c_uint,
    0x89f57f59 as ::core::ffi::c_uint,
    0xb49556e9 as ::core::ffi::c_uint,
    0xf3352c39 as ::core::ffi::c_uint,
    0xce550589 as ::core::ffi::c_uint,
    0x7c75d999 as ::core::ffi::c_uint,
    0x4115f029 as ::core::ffi::c_uint,
    0x6b58af9 as ::core::ffi::c_uint,
    0x3bd5a349 as ::core::ffi::c_uint,
    0xb9853498 as ::core::ffi::c_uint,
    0x84e51d28 as ::core::ffi::c_uint,
    0xc34567f8 as ::core::ffi::c_uint,
    0xfe254e48 as ::core::ffi::c_uint,
    0x4c059258 as ::core::ffi::c_uint,
    0x7165bbe8 as ::core::ffi::c_uint,
    0x36c5c138 as ::core::ffi::c_uint,
    0xba5e888 as ::core::ffi::c_uint,
    0x28d4c7df as ::core::ffi::c_uint,
    0x15b4ee6f as ::core::ffi::c_uint,
    0x521494bf as ::core::ffi::c_uint,
    0x6f74bd0f as ::core::ffi::c_uint,
    0xdd54611f as ::core::ffi::c_uint,
    0xe03448af as ::core::ffi::c_uint,
    0xa794327f as ::core::ffi::c_uint,
    0x9af41bcf as ::core::ffi::c_uint,
    0x18a48c1e as ::core::ffi::c_uint,
    0x25c4a5ae as ::core::ffi::c_uint,
    0x6264df7e as ::core::ffi::c_uint,
    0x5f04f6ce as ::core::ffi::c_uint,
    0xed242ade as ::core::ffi::c_uint,
    0xd044036e as ::core::ffi::c_uint,
    0x97e479be as ::core::ffi::c_uint,
    0xaa84500e as ::core::ffi::c_uint,
    0x4834505d as ::core::ffi::c_uint,
    0x755479ed as ::core::ffi::c_uint,
    0x32f4033d as ::core::ffi::c_uint,
    0xf942a8d as ::core::ffi::c_uint,
    0xbdb4f69d as ::core::ffi::c_uint,
    0x80d4df2d as ::core::ffi::c_uint,
    0xc774a5fd as ::core::ffi::c_uint,
    0xfa148c4d as ::core::ffi::c_uint,
    0x78441b9c as ::core::ffi::c_uint,
    0x4524322c as ::core::ffi::c_uint,
    0x28448fc as ::core::ffi::c_uint,
    0x3fe4614c as ::core::ffi::c_uint,
    0x8dc4bd5c as ::core::ffi::c_uint,
    0xb0a494ec as ::core::ffi::c_uint,
    0xf704ee3c as ::core::ffi::c_uint,
    0xca64c78c as ::core::ffi::c_uint,
];
static mut lodepng_crc32_table5: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_uint,
    0xcb5cd3a5 as ::core::ffi::c_uint,
    0x4dc8a10b as ::core::ffi::c_uint,
    0x869472ae as ::core::ffi::c_uint,
    0x9b914216 as ::core::ffi::c_uint,
    0x50cd91b3 as ::core::ffi::c_uint,
    0xd659e31d as ::core::ffi::c_uint,
    0x1d0530b8 as ::core::ffi::c_uint,
    0xec53826d as ::core::ffi::c_uint,
    0x270f51c8 as ::core::ffi::c_uint,
    0xa19b2366 as ::core::ffi::c_uint,
    0x6ac7f0c3 as ::core::ffi::c_uint,
    0x77c2c07b as ::core::ffi::c_uint,
    0xbc9e13de as ::core::ffi::c_uint,
    0x3a0a6170 as ::core::ffi::c_uint,
    0xf156b2d5 as ::core::ffi::c_uint,
    0x3d6029b as ::core::ffi::c_uint,
    0xc88ad13e as ::core::ffi::c_uint,
    0x4e1ea390 as ::core::ffi::c_uint,
    0x85427035 as ::core::ffi::c_uint,
    0x9847408d as ::core::ffi::c_uint,
    0x531b9328 as ::core::ffi::c_uint,
    0xd58fe186 as ::core::ffi::c_uint,
    0x1ed33223 as ::core::ffi::c_uint,
    0xef8580f6 as ::core::ffi::c_uint,
    0x24d95353 as ::core::ffi::c_uint,
    0xa24d21fd as ::core::ffi::c_uint,
    0x6911f258 as ::core::ffi::c_uint,
    0x7414c2e0 as ::core::ffi::c_uint,
    0xbf481145 as ::core::ffi::c_uint,
    0x39dc63eb as ::core::ffi::c_uint,
    0xf280b04e as ::core::ffi::c_uint,
    0x7ac0536 as ::core::ffi::c_uint,
    0xccf0d693 as ::core::ffi::c_uint,
    0x4a64a43d as ::core::ffi::c_uint,
    0x81387798 as ::core::ffi::c_uint,
    0x9c3d4720 as ::core::ffi::c_uint,
    0x57619485 as ::core::ffi::c_uint,
    0xd1f5e62b as ::core::ffi::c_uint,
    0x1aa9358e as ::core::ffi::c_uint,
    0xebff875b as ::core::ffi::c_uint,
    0x20a354fe as ::core::ffi::c_uint,
    0xa6372650 as ::core::ffi::c_uint,
    0x6d6bf5f5 as ::core::ffi::c_uint,
    0x706ec54d as ::core::ffi::c_uint,
    0xbb3216e8 as ::core::ffi::c_uint,
    0x3da66446 as ::core::ffi::c_uint,
    0xf6fab7e3 as ::core::ffi::c_uint,
    0x47a07ad as ::core::ffi::c_uint,
    0xcf26d408 as ::core::ffi::c_uint,
    0x49b2a6a6 as ::core::ffi::c_uint,
    0x82ee7503 as ::core::ffi::c_uint,
    0x9feb45bb as ::core::ffi::c_uint,
    0x54b7961e as ::core::ffi::c_uint,
    0xd223e4b0 as ::core::ffi::c_uint,
    0x197f3715 as ::core::ffi::c_uint,
    0xe82985c0 as ::core::ffi::c_uint,
    0x23755665 as ::core::ffi::c_uint,
    0xa5e124cb as ::core::ffi::c_uint,
    0x6ebdf76e as ::core::ffi::c_uint,
    0x73b8c7d6 as ::core::ffi::c_uint,
    0xb8e41473 as ::core::ffi::c_uint,
    0x3e7066dd as ::core::ffi::c_uint,
    0xf52cb578 as ::core::ffi::c_uint,
    0xf580a6c as ::core::ffi::c_uint,
    0xc404d9c9 as ::core::ffi::c_uint,
    0x4290ab67 as ::core::ffi::c_uint,
    0x89cc78c2 as ::core::ffi::c_uint,
    0x94c9487a as ::core::ffi::c_uint,
    0x5f959bdf as ::core::ffi::c_uint,
    0xd901e971 as ::core::ffi::c_uint,
    0x125d3ad4 as ::core::ffi::c_uint,
    0xe30b8801 as ::core::ffi::c_uint,
    0x28575ba4 as ::core::ffi::c_uint,
    0xaec3290a as ::core::ffi::c_uint,
    0x659ffaaf as ::core::ffi::c_uint,
    0x789aca17 as ::core::ffi::c_uint,
    0xb3c619b2 as ::core::ffi::c_uint,
    0x35526b1c as ::core::ffi::c_uint,
    0xfe0eb8b9 as ::core::ffi::c_uint,
    0xc8e08f7 as ::core::ffi::c_uint,
    0xc7d2db52 as ::core::ffi::c_uint,
    0x4146a9fc as ::core::ffi::c_uint,
    0x8a1a7a59 as ::core::ffi::c_uint,
    0x971f4ae1 as ::core::ffi::c_uint,
    0x5c439944 as ::core::ffi::c_uint,
    0xdad7ebea as ::core::ffi::c_uint,
    0x118b384f as ::core::ffi::c_uint,
    0xe0dd8a9a as ::core::ffi::c_uint,
    0x2b81593f as ::core::ffi::c_uint,
    0xad152b91 as ::core::ffi::c_uint,
    0x6649f834 as ::core::ffi::c_uint,
    0x7b4cc88c as ::core::ffi::c_uint,
    0xb0101b29 as ::core::ffi::c_uint,
    0x36846987 as ::core::ffi::c_uint,
    0xfdd8ba22 as ::core::ffi::c_uint,
    0x8f40f5a as ::core::ffi::c_uint,
    0xc3a8dcff as ::core::ffi::c_uint,
    0x453cae51 as ::core::ffi::c_uint,
    0x8e607df4 as ::core::ffi::c_uint,
    0x93654d4c as ::core::ffi::c_uint,
    0x58399ee9 as ::core::ffi::c_uint,
    0xdeadec47 as ::core::ffi::c_uint,
    0x15f13fe2 as ::core::ffi::c_uint,
    0xe4a78d37 as ::core::ffi::c_uint,
    0x2ffb5e92 as ::core::ffi::c_uint,
    0xa96f2c3c as ::core::ffi::c_uint,
    0x6233ff99 as ::core::ffi::c_uint,
    0x7f36cf21 as ::core::ffi::c_uint,
    0xb46a1c84 as ::core::ffi::c_uint,
    0x32fe6e2a as ::core::ffi::c_uint,
    0xf9a2bd8f as ::core::ffi::c_uint,
    0xb220dc1 as ::core::ffi::c_uint,
    0xc07ede64 as ::core::ffi::c_uint,
    0x46eaacca as ::core::ffi::c_uint,
    0x8db67f6f as ::core::ffi::c_uint,
    0x90b34fd7 as ::core::ffi::c_uint,
    0x5bef9c72 as ::core::ffi::c_uint,
    0xdd7beedc as ::core::ffi::c_uint,
    0x16273d79 as ::core::ffi::c_uint,
    0xe7718fac as ::core::ffi::c_uint,
    0x2c2d5c09 as ::core::ffi::c_uint,
    0xaab92ea7 as ::core::ffi::c_uint,
    0x61e5fd02 as ::core::ffi::c_uint,
    0x7ce0cdba as ::core::ffi::c_uint,
    0xb7bc1e1f as ::core::ffi::c_uint,
    0x31286cb1 as ::core::ffi::c_uint,
    0xfa74bf14 as ::core::ffi::c_uint,
    0x1eb014d8 as ::core::ffi::c_uint,
    0xd5ecc77d as ::core::ffi::c_uint,
    0x5378b5d3 as ::core::ffi::c_uint,
    0x98246676 as ::core::ffi::c_uint,
    0x852156ce as ::core::ffi::c_uint,
    0x4e7d856b as ::core::ffi::c_uint,
    0xc8e9f7c5 as ::core::ffi::c_uint,
    0x3b52460 as ::core::ffi::c_uint,
    0xf2e396b5 as ::core::ffi::c_uint,
    0x39bf4510 as ::core::ffi::c_uint,
    0xbf2b37be as ::core::ffi::c_uint,
    0x7477e41b as ::core::ffi::c_uint,
    0x6972d4a3 as ::core::ffi::c_uint,
    0xa22e0706 as ::core::ffi::c_uint,
    0x24ba75a8 as ::core::ffi::c_uint,
    0xefe6a60d as ::core::ffi::c_uint,
    0x1d661643 as ::core::ffi::c_uint,
    0xd63ac5e6 as ::core::ffi::c_uint,
    0x50aeb748 as ::core::ffi::c_uint,
    0x9bf264ed as ::core::ffi::c_uint,
    0x86f75455 as ::core::ffi::c_uint,
    0x4dab87f0 as ::core::ffi::c_uint,
    0xcb3ff55e as ::core::ffi::c_uint,
    0x6326fb as ::core::ffi::c_uint,
    0xf135942e as ::core::ffi::c_uint,
    0x3a69478b as ::core::ffi::c_uint,
    0xbcfd3525 as ::core::ffi::c_uint,
    0x77a1e680 as ::core::ffi::c_uint,
    0x6aa4d638 as ::core::ffi::c_uint,
    0xa1f8059d as ::core::ffi::c_uint,
    0x276c7733 as ::core::ffi::c_uint,
    0xec30a496 as ::core::ffi::c_uint,
    0x191c11ee as ::core::ffi::c_uint,
    0xd240c24b as ::core::ffi::c_uint,
    0x54d4b0e5 as ::core::ffi::c_uint,
    0x9f886340 as ::core::ffi::c_uint,
    0x828d53f8 as ::core::ffi::c_uint,
    0x49d1805d as ::core::ffi::c_uint,
    0xcf45f2f3 as ::core::ffi::c_uint,
    0x4192156 as ::core::ffi::c_uint,
    0xf54f9383 as ::core::ffi::c_uint,
    0x3e134026 as ::core::ffi::c_uint,
    0xb8873288 as ::core::ffi::c_uint,
    0x73dbe12d as ::core::ffi::c_uint,
    0x6eded195 as ::core::ffi::c_uint,
    0xa5820230 as ::core::ffi::c_uint,
    0x2316709e as ::core::ffi::c_uint,
    0xe84aa33b as ::core::ffi::c_uint,
    0x1aca1375 as ::core::ffi::c_uint,
    0xd196c0d0 as ::core::ffi::c_uint,
    0x5702b27e as ::core::ffi::c_uint,
    0x9c5e61db as ::core::ffi::c_uint,
    0x815b5163 as ::core::ffi::c_uint,
    0x4a0782c6 as ::core::ffi::c_uint,
    0xcc93f068 as ::core::ffi::c_uint,
    0x7cf23cd as ::core::ffi::c_uint,
    0xf6999118 as ::core::ffi::c_uint,
    0x3dc542bd as ::core::ffi::c_uint,
    0xbb513013 as ::core::ffi::c_uint,
    0x700de3b6 as ::core::ffi::c_uint,
    0x6d08d30e as ::core::ffi::c_uint,
    0xa65400ab as ::core::ffi::c_uint,
    0x20c07205 as ::core::ffi::c_uint,
    0xeb9ca1a0 as ::core::ffi::c_uint,
    0x11e81eb4 as ::core::ffi::c_uint,
    0xdab4cd11 as ::core::ffi::c_uint,
    0x5c20bfbf as ::core::ffi::c_uint,
    0x977c6c1a as ::core::ffi::c_uint,
    0x8a795ca2 as ::core::ffi::c_uint,
    0x41258f07 as ::core::ffi::c_uint,
    0xc7b1fda9 as ::core::ffi::c_uint,
    0xced2e0c as ::core::ffi::c_uint,
    0xfdbb9cd9 as ::core::ffi::c_uint,
    0x36e74f7c as ::core::ffi::c_uint,
    0xb0733dd2 as ::core::ffi::c_uint,
    0x7b2fee77 as ::core::ffi::c_uint,
    0x662adecf as ::core::ffi::c_uint,
    0xad760d6a as ::core::ffi::c_uint,
    0x2be27fc4 as ::core::ffi::c_uint,
    0xe0beac61 as ::core::ffi::c_uint,
    0x123e1c2f as ::core::ffi::c_uint,
    0xd962cf8a as ::core::ffi::c_uint,
    0x5ff6bd24 as ::core::ffi::c_uint,
    0x94aa6e81 as ::core::ffi::c_uint,
    0x89af5e39 as ::core::ffi::c_uint,
    0x42f38d9c as ::core::ffi::c_uint,
    0xc467ff32 as ::core::ffi::c_uint,
    0xf3b2c97 as ::core::ffi::c_uint,
    0xfe6d9e42 as ::core::ffi::c_uint,
    0x35314de7 as ::core::ffi::c_uint,
    0xb3a53f49 as ::core::ffi::c_uint,
    0x78f9ecec as ::core::ffi::c_uint,
    0x65fcdc54 as ::core::ffi::c_uint,
    0xaea00ff1 as ::core::ffi::c_uint,
    0x28347d5f as ::core::ffi::c_uint,
    0xe368aefa as ::core::ffi::c_uint,
    0x16441b82 as ::core::ffi::c_uint,
    0xdd18c827 as ::core::ffi::c_uint,
    0x5b8cba89 as ::core::ffi::c_uint,
    0x90d0692c as ::core::ffi::c_uint,
    0x8dd55994 as ::core::ffi::c_uint,
    0x46898a31 as ::core::ffi::c_uint,
    0xc01df89f as ::core::ffi::c_uint,
    0xb412b3a as ::core::ffi::c_uint,
    0xfa1799ef as ::core::ffi::c_uint,
    0x314b4a4a as ::core::ffi::c_uint,
    0xb7df38e4 as ::core::ffi::c_uint,
    0x7c83eb41 as ::core::ffi::c_uint,
    0x6186dbf9 as ::core::ffi::c_uint,
    0xaada085c as ::core::ffi::c_uint,
    0x2c4e7af2 as ::core::ffi::c_uint,
    0xe712a957 as ::core::ffi::c_uint,
    0x15921919 as ::core::ffi::c_uint,
    0xdececabc as ::core::ffi::c_uint,
    0x585ab812 as ::core::ffi::c_uint,
    0x93066bb7 as ::core::ffi::c_uint,
    0x8e035b0f as ::core::ffi::c_uint,
    0x455f88aa as ::core::ffi::c_uint,
    0xc3cbfa04 as ::core::ffi::c_uint,
    0x89729a1 as ::core::ffi::c_uint,
    0xf9c19b74 as ::core::ffi::c_uint,
    0x329d48d1 as ::core::ffi::c_uint,
    0xb4093a7f as ::core::ffi::c_uint,
    0x7f55e9da as ::core::ffi::c_uint,
    0x6250d962 as ::core::ffi::c_uint,
    0xa90c0ac7 as ::core::ffi::c_uint,
    0x2f987869 as ::core::ffi::c_uint,
    0xe4c4abcc as ::core::ffi::c_uint,
];
static mut lodepng_crc32_table6: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_uint,
    0xa6770bb4 as ::core::ffi::c_uint,
    0x979f1129 as ::core::ffi::c_uint,
    0x31e81a9d as ::core::ffi::c_uint,
    0xf44f2413 as ::core::ffi::c_uint,
    0x52382fa7 as ::core::ffi::c_uint,
    0x63d0353a as ::core::ffi::c_uint,
    0xc5a73e8e as ::core::ffi::c_uint,
    0x33ef4e67 as ::core::ffi::c_uint,
    0x959845d3 as ::core::ffi::c_uint,
    0xa4705f4e as ::core::ffi::c_uint,
    0x20754fa as ::core::ffi::c_uint,
    0xc7a06a74 as ::core::ffi::c_uint,
    0x61d761c0 as ::core::ffi::c_uint,
    0x503f7b5d as ::core::ffi::c_uint,
    0xf64870e9 as ::core::ffi::c_uint,
    0x67de9cce as ::core::ffi::c_uint,
    0xc1a9977a as ::core::ffi::c_uint,
    0xf0418de7 as ::core::ffi::c_uint,
    0x56368653 as ::core::ffi::c_uint,
    0x9391b8dd as ::core::ffi::c_uint,
    0x35e6b369 as ::core::ffi::c_uint,
    0x40ea9f4 as ::core::ffi::c_uint,
    0xa279a240 as ::core::ffi::c_uint,
    0x5431d2a9 as ::core::ffi::c_uint,
    0xf246d91d as ::core::ffi::c_uint,
    0xc3aec380 as ::core::ffi::c_uint,
    0x65d9c834 as ::core::ffi::c_uint,
    0xa07ef6ba as ::core::ffi::c_uint,
    0x609fd0e as ::core::ffi::c_uint,
    0x37e1e793 as ::core::ffi::c_uint,
    0x9196ec27 as ::core::ffi::c_uint,
    0xcfbd399c as ::core::ffi::c_uint,
    0x69ca3228 as ::core::ffi::c_uint,
    0x582228b5 as ::core::ffi::c_uint,
    0xfe552301 as ::core::ffi::c_uint,
    0x3bf21d8f as ::core::ffi::c_uint,
    0x9d85163b as ::core::ffi::c_uint,
    0xac6d0ca6 as ::core::ffi::c_uint,
    0xa1a0712 as ::core::ffi::c_uint,
    0xfc5277fb as ::core::ffi::c_uint,
    0x5a257c4f as ::core::ffi::c_uint,
    0x6bcd66d2 as ::core::ffi::c_uint,
    0xcdba6d66 as ::core::ffi::c_uint,
    0x81d53e8 as ::core::ffi::c_uint,
    0xae6a585c as ::core::ffi::c_uint,
    0x9f8242c1 as ::core::ffi::c_uint,
    0x39f54975 as ::core::ffi::c_uint,
    0xa863a552 as ::core::ffi::c_uint,
    0xe14aee6 as ::core::ffi::c_uint,
    0x3ffcb47b as ::core::ffi::c_uint,
    0x998bbfcf as ::core::ffi::c_uint,
    0x5c2c8141 as ::core::ffi::c_uint,
    0xfa5b8af5 as ::core::ffi::c_uint,
    0xcbb39068 as ::core::ffi::c_uint,
    0x6dc49bdc as ::core::ffi::c_uint,
    0x9b8ceb35 as ::core::ffi::c_uint,
    0x3dfbe081 as ::core::ffi::c_uint,
    0xc13fa1c as ::core::ffi::c_uint,
    0xaa64f1a8 as ::core::ffi::c_uint,
    0x6fc3cf26 as ::core::ffi::c_uint,
    0xc9b4c492 as ::core::ffi::c_uint,
    0xf85cde0f as ::core::ffi::c_uint,
    0x5e2bd5bb as ::core::ffi::c_uint,
    0x440b7579 as ::core::ffi::c_uint,
    0xe27c7ecd as ::core::ffi::c_uint,
    0xd3946450 as ::core::ffi::c_uint,
    0x75e36fe4 as ::core::ffi::c_uint,
    0xb044516a as ::core::ffi::c_uint,
    0x16335ade as ::core::ffi::c_uint,
    0x27db4043 as ::core::ffi::c_uint,
    0x81ac4bf7 as ::core::ffi::c_uint,
    0x77e43b1e as ::core::ffi::c_uint,
    0xd19330aa as ::core::ffi::c_uint,
    0xe07b2a37 as ::core::ffi::c_uint,
    0x460c2183 as ::core::ffi::c_uint,
    0x83ab1f0d as ::core::ffi::c_uint,
    0x25dc14b9 as ::core::ffi::c_uint,
    0x14340e24 as ::core::ffi::c_uint,
    0xb2430590 as ::core::ffi::c_uint,
    0x23d5e9b7 as ::core::ffi::c_uint,
    0x85a2e203 as ::core::ffi::c_uint,
    0xb44af89e as ::core::ffi::c_uint,
    0x123df32a as ::core::ffi::c_uint,
    0xd79acda4 as ::core::ffi::c_uint,
    0x71edc610 as ::core::ffi::c_uint,
    0x4005dc8d as ::core::ffi::c_uint,
    0xe672d739 as ::core::ffi::c_uint,
    0x103aa7d0 as ::core::ffi::c_uint,
    0xb64dac64 as ::core::ffi::c_uint,
    0x87a5b6f9 as ::core::ffi::c_uint,
    0x21d2bd4d as ::core::ffi::c_uint,
    0xe47583c3 as ::core::ffi::c_uint,
    0x42028877 as ::core::ffi::c_uint,
    0x73ea92ea as ::core::ffi::c_uint,
    0xd59d995e as ::core::ffi::c_uint,
    0x8bb64ce5 as ::core::ffi::c_uint,
    0x2dc14751 as ::core::ffi::c_uint,
    0x1c295dcc as ::core::ffi::c_uint,
    0xba5e5678 as ::core::ffi::c_uint,
    0x7ff968f6 as ::core::ffi::c_uint,
    0xd98e6342 as ::core::ffi::c_uint,
    0xe86679df as ::core::ffi::c_uint,
    0x4e11726b as ::core::ffi::c_uint,
    0xb8590282 as ::core::ffi::c_uint,
    0x1e2e0936 as ::core::ffi::c_uint,
    0x2fc613ab as ::core::ffi::c_uint,
    0x89b1181f as ::core::ffi::c_uint,
    0x4c162691 as ::core::ffi::c_uint,
    0xea612d25 as ::core::ffi::c_uint,
    0xdb8937b8 as ::core::ffi::c_uint,
    0x7dfe3c0c as ::core::ffi::c_uint,
    0xec68d02b as ::core::ffi::c_uint,
    0x4a1fdb9f as ::core::ffi::c_uint,
    0x7bf7c102 as ::core::ffi::c_uint,
    0xdd80cab6 as ::core::ffi::c_uint,
    0x1827f438 as ::core::ffi::c_uint,
    0xbe50ff8c as ::core::ffi::c_uint,
    0x8fb8e511 as ::core::ffi::c_uint,
    0x29cfeea5 as ::core::ffi::c_uint,
    0xdf879e4c as ::core::ffi::c_uint,
    0x79f095f8 as ::core::ffi::c_uint,
    0x48188f65 as ::core::ffi::c_uint,
    0xee6f84d1 as ::core::ffi::c_uint,
    0x2bc8ba5f as ::core::ffi::c_uint,
    0x8dbfb1eb as ::core::ffi::c_uint,
    0xbc57ab76 as ::core::ffi::c_uint,
    0x1a20a0c2 as ::core::ffi::c_uint,
    0x8816eaf2 as ::core::ffi::c_uint,
    0x2e61e146 as ::core::ffi::c_uint,
    0x1f89fbdb as ::core::ffi::c_uint,
    0xb9fef06f as ::core::ffi::c_uint,
    0x7c59cee1 as ::core::ffi::c_uint,
    0xda2ec555 as ::core::ffi::c_uint,
    0xebc6dfc8 as ::core::ffi::c_uint,
    0x4db1d47c as ::core::ffi::c_uint,
    0xbbf9a495 as ::core::ffi::c_uint,
    0x1d8eaf21 as ::core::ffi::c_uint,
    0x2c66b5bc as ::core::ffi::c_uint,
    0x8a11be08 as ::core::ffi::c_uint,
    0x4fb68086 as ::core::ffi::c_uint,
    0xe9c18b32 as ::core::ffi::c_uint,
    0xd82991af as ::core::ffi::c_uint,
    0x7e5e9a1b as ::core::ffi::c_uint,
    0xefc8763c as ::core::ffi::c_uint,
    0x49bf7d88 as ::core::ffi::c_uint,
    0x78576715 as ::core::ffi::c_uint,
    0xde206ca1 as ::core::ffi::c_uint,
    0x1b87522f as ::core::ffi::c_uint,
    0xbdf0599b as ::core::ffi::c_uint,
    0x8c184306 as ::core::ffi::c_uint,
    0x2a6f48b2 as ::core::ffi::c_uint,
    0xdc27385b as ::core::ffi::c_uint,
    0x7a5033ef as ::core::ffi::c_uint,
    0x4bb82972 as ::core::ffi::c_uint,
    0xedcf22c6 as ::core::ffi::c_uint,
    0x28681c48 as ::core::ffi::c_uint,
    0x8e1f17fc as ::core::ffi::c_uint,
    0xbff70d61 as ::core::ffi::c_uint,
    0x198006d5 as ::core::ffi::c_uint,
    0x47abd36e as ::core::ffi::c_uint,
    0xe1dcd8da as ::core::ffi::c_uint,
    0xd034c247 as ::core::ffi::c_uint,
    0x7643c9f3 as ::core::ffi::c_uint,
    0xb3e4f77d as ::core::ffi::c_uint,
    0x1593fcc9 as ::core::ffi::c_uint,
    0x247be654 as ::core::ffi::c_uint,
    0x820cede0 as ::core::ffi::c_uint,
    0x74449d09 as ::core::ffi::c_uint,
    0xd23396bd as ::core::ffi::c_uint,
    0xe3db8c20 as ::core::ffi::c_uint,
    0x45ac8794 as ::core::ffi::c_uint,
    0x800bb91a as ::core::ffi::c_uint,
    0x267cb2ae as ::core::ffi::c_uint,
    0x1794a833 as ::core::ffi::c_uint,
    0xb1e3a387 as ::core::ffi::c_uint,
    0x20754fa0 as ::core::ffi::c_uint,
    0x86024414 as ::core::ffi::c_uint,
    0xb7ea5e89 as ::core::ffi::c_uint,
    0x119d553d as ::core::ffi::c_uint,
    0xd43a6bb3 as ::core::ffi::c_uint,
    0x724d6007 as ::core::ffi::c_uint,
    0x43a57a9a as ::core::ffi::c_uint,
    0xe5d2712e as ::core::ffi::c_uint,
    0x139a01c7 as ::core::ffi::c_uint,
    0xb5ed0a73 as ::core::ffi::c_uint,
    0x840510ee as ::core::ffi::c_uint,
    0x22721b5a as ::core::ffi::c_uint,
    0xe7d525d4 as ::core::ffi::c_uint,
    0x41a22e60 as ::core::ffi::c_uint,
    0x704a34fd as ::core::ffi::c_uint,
    0xd63d3f49 as ::core::ffi::c_uint,
    0xcc1d9f8b as ::core::ffi::c_uint,
    0x6a6a943f as ::core::ffi::c_uint,
    0x5b828ea2 as ::core::ffi::c_uint,
    0xfdf58516 as ::core::ffi::c_uint,
    0x3852bb98 as ::core::ffi::c_uint,
    0x9e25b02c as ::core::ffi::c_uint,
    0xafcdaab1 as ::core::ffi::c_uint,
    0x9baa105 as ::core::ffi::c_uint,
    0xfff2d1ec as ::core::ffi::c_uint,
    0x5985da58 as ::core::ffi::c_uint,
    0x686dc0c5 as ::core::ffi::c_uint,
    0xce1acb71 as ::core::ffi::c_uint,
    0xbbdf5ff as ::core::ffi::c_uint,
    0xadcafe4b as ::core::ffi::c_uint,
    0x9c22e4d6 as ::core::ffi::c_uint,
    0x3a55ef62 as ::core::ffi::c_uint,
    0xabc30345 as ::core::ffi::c_uint,
    0xdb408f1 as ::core::ffi::c_uint,
    0x3c5c126c as ::core::ffi::c_uint,
    0x9a2b19d8 as ::core::ffi::c_uint,
    0x5f8c2756 as ::core::ffi::c_uint,
    0xf9fb2ce2 as ::core::ffi::c_uint,
    0xc813367f as ::core::ffi::c_uint,
    0x6e643dcb as ::core::ffi::c_uint,
    0x982c4d22 as ::core::ffi::c_uint,
    0x3e5b4696 as ::core::ffi::c_uint,
    0xfb35c0b as ::core::ffi::c_uint,
    0xa9c457bf as ::core::ffi::c_uint,
    0x6c636931 as ::core::ffi::c_uint,
    0xca146285 as ::core::ffi::c_uint,
    0xfbfc7818 as ::core::ffi::c_uint,
    0x5d8b73ac as ::core::ffi::c_uint,
    0x3a0a617 as ::core::ffi::c_uint,
    0xa5d7ada3 as ::core::ffi::c_uint,
    0x943fb73e as ::core::ffi::c_uint,
    0x3248bc8a as ::core::ffi::c_uint,
    0xf7ef8204 as ::core::ffi::c_uint,
    0x519889b0 as ::core::ffi::c_uint,
    0x6070932d as ::core::ffi::c_uint,
    0xc6079899 as ::core::ffi::c_uint,
    0x304fe870 as ::core::ffi::c_uint,
    0x9638e3c4 as ::core::ffi::c_uint,
    0xa7d0f959 as ::core::ffi::c_uint,
    0x1a7f2ed as ::core::ffi::c_uint,
    0xc400cc63 as ::core::ffi::c_uint,
    0x6277c7d7 as ::core::ffi::c_uint,
    0x539fdd4a as ::core::ffi::c_uint,
    0xf5e8d6fe as ::core::ffi::c_uint,
    0x647e3ad9 as ::core::ffi::c_uint,
    0xc209316d as ::core::ffi::c_uint,
    0xf3e12bf0 as ::core::ffi::c_uint,
    0x55962044 as ::core::ffi::c_uint,
    0x90311eca as ::core::ffi::c_uint,
    0x3646157e as ::core::ffi::c_uint,
    0x7ae0fe3 as ::core::ffi::c_uint,
    0xa1d90457 as ::core::ffi::c_uint,
    0x579174be as ::core::ffi::c_uint,
    0xf1e67f0a as ::core::ffi::c_uint,
    0xc00e6597 as ::core::ffi::c_uint,
    0x66796e23 as ::core::ffi::c_uint,
    0xa3de50ad as ::core::ffi::c_uint,
    0x5a95b19 as ::core::ffi::c_uint,
    0x34414184 as ::core::ffi::c_uint,
    0x92364a30 as ::core::ffi::c_uint,
];
static mut lodepng_crc32_table7: [::core::ffi::c_uint; 256] = [
    0 as ::core::ffi::c_uint,
    0xccaa009e as ::core::ffi::c_uint,
    0x4225077d as ::core::ffi::c_uint,
    0x8e8f07e3 as ::core::ffi::c_uint,
    0x844a0efa as ::core::ffi::c_uint,
    0x48e00e64 as ::core::ffi::c_uint,
    0xc66f0987 as ::core::ffi::c_uint,
    0xac50919 as ::core::ffi::c_uint,
    0xd3e51bb5 as ::core::ffi::c_uint,
    0x1f4f1b2b as ::core::ffi::c_uint,
    0x91c01cc8 as ::core::ffi::c_uint,
    0x5d6a1c56 as ::core::ffi::c_uint,
    0x57af154f as ::core::ffi::c_uint,
    0x9b0515d1 as ::core::ffi::c_uint,
    0x158a1232 as ::core::ffi::c_uint,
    0xd92012ac as ::core::ffi::c_uint,
    0x7cbb312b as ::core::ffi::c_uint,
    0xb01131b5 as ::core::ffi::c_uint,
    0x3e9e3656 as ::core::ffi::c_uint,
    0xf23436c8 as ::core::ffi::c_uint,
    0xf8f13fd1 as ::core::ffi::c_uint,
    0x345b3f4f as ::core::ffi::c_uint,
    0xbad438ac as ::core::ffi::c_uint,
    0x767e3832 as ::core::ffi::c_uint,
    0xaf5e2a9e as ::core::ffi::c_uint,
    0x63f42a00 as ::core::ffi::c_uint,
    0xed7b2de3 as ::core::ffi::c_uint,
    0x21d12d7d as ::core::ffi::c_uint,
    0x2b142464 as ::core::ffi::c_uint,
    0xe7be24fa as ::core::ffi::c_uint,
    0x69312319 as ::core::ffi::c_uint,
    0xa59b2387 as ::core::ffi::c_uint,
    0xf9766256 as ::core::ffi::c_uint,
    0x35dc62c8 as ::core::ffi::c_uint,
    0xbb53652b as ::core::ffi::c_uint,
    0x77f965b5 as ::core::ffi::c_uint,
    0x7d3c6cac as ::core::ffi::c_uint,
    0xb1966c32 as ::core::ffi::c_uint,
    0x3f196bd1 as ::core::ffi::c_uint,
    0xf3b36b4f as ::core::ffi::c_uint,
    0x2a9379e3 as ::core::ffi::c_uint,
    0xe639797d as ::core::ffi::c_uint,
    0x68b67e9e as ::core::ffi::c_uint,
    0xa41c7e00 as ::core::ffi::c_uint,
    0xaed97719 as ::core::ffi::c_uint,
    0x62737787 as ::core::ffi::c_uint,
    0xecfc7064 as ::core::ffi::c_uint,
    0x205670fa as ::core::ffi::c_uint,
    0x85cd537d as ::core::ffi::c_uint,
    0x496753e3 as ::core::ffi::c_uint,
    0xc7e85400 as ::core::ffi::c_uint,
    0xb42549e as ::core::ffi::c_uint,
    0x1875d87 as ::core::ffi::c_uint,
    0xcd2d5d19 as ::core::ffi::c_uint,
    0x43a25afa as ::core::ffi::c_uint,
    0x8f085a64 as ::core::ffi::c_uint,
    0x562848c8 as ::core::ffi::c_uint,
    0x9a824856 as ::core::ffi::c_uint,
    0x140d4fb5 as ::core::ffi::c_uint,
    0xd8a74f2b as ::core::ffi::c_uint,
    0xd2624632 as ::core::ffi::c_uint,
    0x1ec846ac as ::core::ffi::c_uint,
    0x9047414f as ::core::ffi::c_uint,
    0x5ced41d1 as ::core::ffi::c_uint,
    0x299dc2ed as ::core::ffi::c_uint,
    0xe537c273 as ::core::ffi::c_uint,
    0x6bb8c590 as ::core::ffi::c_uint,
    0xa712c50e as ::core::ffi::c_uint,
    0xadd7cc17 as ::core::ffi::c_uint,
    0x617dcc89 as ::core::ffi::c_uint,
    0xeff2cb6a as ::core::ffi::c_uint,
    0x2358cbf4 as ::core::ffi::c_uint,
    0xfa78d958 as ::core::ffi::c_uint,
    0x36d2d9c6 as ::core::ffi::c_uint,
    0xb85dde25 as ::core::ffi::c_uint,
    0x74f7debb as ::core::ffi::c_uint,
    0x7e32d7a2 as ::core::ffi::c_uint,
    0xb298d73c as ::core::ffi::c_uint,
    0x3c17d0df as ::core::ffi::c_uint,
    0xf0bdd041 as ::core::ffi::c_uint,
    0x5526f3c6 as ::core::ffi::c_uint,
    0x998cf358 as ::core::ffi::c_uint,
    0x1703f4bb as ::core::ffi::c_uint,
    0xdba9f425 as ::core::ffi::c_uint,
    0xd16cfd3c as ::core::ffi::c_uint,
    0x1dc6fda2 as ::core::ffi::c_uint,
    0x9349fa41 as ::core::ffi::c_uint,
    0x5fe3fadf as ::core::ffi::c_uint,
    0x86c3e873 as ::core::ffi::c_uint,
    0x4a69e8ed as ::core::ffi::c_uint,
    0xc4e6ef0e as ::core::ffi::c_uint,
    0x84cef90 as ::core::ffi::c_uint,
    0x289e689 as ::core::ffi::c_uint,
    0xce23e617 as ::core::ffi::c_uint,
    0x40ace1f4 as ::core::ffi::c_uint,
    0x8c06e16a as ::core::ffi::c_uint,
    0xd0eba0bb as ::core::ffi::c_uint,
    0x1c41a025 as ::core::ffi::c_uint,
    0x92cea7c6 as ::core::ffi::c_uint,
    0x5e64a758 as ::core::ffi::c_uint,
    0x54a1ae41 as ::core::ffi::c_uint,
    0x980baedf as ::core::ffi::c_uint,
    0x1684a93c as ::core::ffi::c_uint,
    0xda2ea9a2 as ::core::ffi::c_uint,
    0x30ebb0e as ::core::ffi::c_uint,
    0xcfa4bb90 as ::core::ffi::c_uint,
    0x412bbc73 as ::core::ffi::c_uint,
    0x8d81bced as ::core::ffi::c_uint,
    0x8744b5f4 as ::core::ffi::c_uint,
    0x4beeb56a as ::core::ffi::c_uint,
    0xc561b289 as ::core::ffi::c_uint,
    0x9cbb217 as ::core::ffi::c_uint,
    0xac509190 as ::core::ffi::c_uint,
    0x60fa910e as ::core::ffi::c_uint,
    0xee7596ed as ::core::ffi::c_uint,
    0x22df9673 as ::core::ffi::c_uint,
    0x281a9f6a as ::core::ffi::c_uint,
    0xe4b09ff4 as ::core::ffi::c_uint,
    0x6a3f9817 as ::core::ffi::c_uint,
    0xa6959889 as ::core::ffi::c_uint,
    0x7fb58a25 as ::core::ffi::c_uint,
    0xb31f8abb as ::core::ffi::c_uint,
    0x3d908d58 as ::core::ffi::c_uint,
    0xf13a8dc6 as ::core::ffi::c_uint,
    0xfbff84df as ::core::ffi::c_uint,
    0x37558441 as ::core::ffi::c_uint,
    0xb9da83a2 as ::core::ffi::c_uint,
    0x7570833c as ::core::ffi::c_uint,
    0x533b85da as ::core::ffi::c_uint,
    0x9f918544 as ::core::ffi::c_uint,
    0x111e82a7 as ::core::ffi::c_uint,
    0xddb48239 as ::core::ffi::c_uint,
    0xd7718b20 as ::core::ffi::c_uint,
    0x1bdb8bbe as ::core::ffi::c_uint,
    0x95548c5d as ::core::ffi::c_uint,
    0x59fe8cc3 as ::core::ffi::c_uint,
    0x80de9e6f as ::core::ffi::c_uint,
    0x4c749ef1 as ::core::ffi::c_uint,
    0xc2fb9912 as ::core::ffi::c_uint,
    0xe51998c as ::core::ffi::c_uint,
    0x4949095 as ::core::ffi::c_uint,
    0xc83e900b as ::core::ffi::c_uint,
    0x46b197e8 as ::core::ffi::c_uint,
    0x8a1b9776 as ::core::ffi::c_uint,
    0x2f80b4f1 as ::core::ffi::c_uint,
    0xe32ab46f as ::core::ffi::c_uint,
    0x6da5b38c as ::core::ffi::c_uint,
    0xa10fb312 as ::core::ffi::c_uint,
    0xabcaba0b as ::core::ffi::c_uint,
    0x6760ba95 as ::core::ffi::c_uint,
    0xe9efbd76 as ::core::ffi::c_uint,
    0x2545bde8 as ::core::ffi::c_uint,
    0xfc65af44 as ::core::ffi::c_uint,
    0x30cfafda as ::core::ffi::c_uint,
    0xbe40a839 as ::core::ffi::c_uint,
    0x72eaa8a7 as ::core::ffi::c_uint,
    0x782fa1be as ::core::ffi::c_uint,
    0xb485a120 as ::core::ffi::c_uint,
    0x3a0aa6c3 as ::core::ffi::c_uint,
    0xf6a0a65d as ::core::ffi::c_uint,
    0xaa4de78c as ::core::ffi::c_uint,
    0x66e7e712 as ::core::ffi::c_uint,
    0xe868e0f1 as ::core::ffi::c_uint,
    0x24c2e06f as ::core::ffi::c_uint,
    0x2e07e976 as ::core::ffi::c_uint,
    0xe2ade9e8 as ::core::ffi::c_uint,
    0x6c22ee0b as ::core::ffi::c_uint,
    0xa088ee95 as ::core::ffi::c_uint,
    0x79a8fc39 as ::core::ffi::c_uint,
    0xb502fca7 as ::core::ffi::c_uint,
    0x3b8dfb44 as ::core::ffi::c_uint,
    0xf727fbda as ::core::ffi::c_uint,
    0xfde2f2c3 as ::core::ffi::c_uint,
    0x3148f25d as ::core::ffi::c_uint,
    0xbfc7f5be as ::core::ffi::c_uint,
    0x736df520 as ::core::ffi::c_uint,
    0xd6f6d6a7 as ::core::ffi::c_uint,
    0x1a5cd639 as ::core::ffi::c_uint,
    0x94d3d1da as ::core::ffi::c_uint,
    0x5879d144 as ::core::ffi::c_uint,
    0x52bcd85d as ::core::ffi::c_uint,
    0x9e16d8c3 as ::core::ffi::c_uint,
    0x1099df20 as ::core::ffi::c_uint,
    0xdc33dfbe as ::core::ffi::c_uint,
    0x513cd12 as ::core::ffi::c_uint,
    0xc9b9cd8c as ::core::ffi::c_uint,
    0x4736ca6f as ::core::ffi::c_uint,
    0x8b9ccaf1 as ::core::ffi::c_uint,
    0x8159c3e8 as ::core::ffi::c_uint,
    0x4df3c376 as ::core::ffi::c_uint,
    0xc37cc495 as ::core::ffi::c_uint,
    0xfd6c40b as ::core::ffi::c_uint,
    0x7aa64737 as ::core::ffi::c_uint,
    0xb60c47a9 as ::core::ffi::c_uint,
    0x3883404a as ::core::ffi::c_uint,
    0xf42940d4 as ::core::ffi::c_uint,
    0xfeec49cd as ::core::ffi::c_uint,
    0x32464953 as ::core::ffi::c_uint,
    0xbcc94eb0 as ::core::ffi::c_uint,
    0x70634e2e as ::core::ffi::c_uint,
    0xa9435c82 as ::core::ffi::c_uint,
    0x65e95c1c as ::core::ffi::c_uint,
    0xeb665bff as ::core::ffi::c_uint,
    0x27cc5b61 as ::core::ffi::c_uint,
    0x2d095278 as ::core::ffi::c_uint,
    0xe1a352e6 as ::core::ffi::c_uint,
    0x6f2c5505 as ::core::ffi::c_uint,
    0xa386559b as ::core::ffi::c_uint,
    0x61d761c as ::core::ffi::c_uint,
    0xcab77682 as ::core::ffi::c_uint,
    0x44387161 as ::core::ffi::c_uint,
    0x889271ff as ::core::ffi::c_uint,
    0x825778e6 as ::core::ffi::c_uint,
    0x4efd7878 as ::core::ffi::c_uint,
    0xc0727f9b as ::core::ffi::c_uint,
    0xcd87f05 as ::core::ffi::c_uint,
    0xd5f86da9 as ::core::ffi::c_uint,
    0x19526d37 as ::core::ffi::c_uint,
    0x97dd6ad4 as ::core::ffi::c_uint,
    0x5b776a4a as ::core::ffi::c_uint,
    0x51b26353 as ::core::ffi::c_uint,
    0x9d1863cd as ::core::ffi::c_uint,
    0x1397642e as ::core::ffi::c_uint,
    0xdf3d64b0 as ::core::ffi::c_uint,
    0x83d02561 as ::core::ffi::c_uint,
    0x4f7a25ff as ::core::ffi::c_uint,
    0xc1f5221c as ::core::ffi::c_uint,
    0xd5f2282 as ::core::ffi::c_uint,
    0x79a2b9b as ::core::ffi::c_uint,
    0xcb302b05 as ::core::ffi::c_uint,
    0x45bf2ce6 as ::core::ffi::c_uint,
    0x89152c78 as ::core::ffi::c_uint,
    0x50353ed4 as ::core::ffi::c_uint,
    0x9c9f3e4a as ::core::ffi::c_uint,
    0x121039a9 as ::core::ffi::c_uint,
    0xdeba3937 as ::core::ffi::c_uint,
    0xd47f302e as ::core::ffi::c_uint,
    0x18d530b0 as ::core::ffi::c_uint,
    0x965a3753 as ::core::ffi::c_uint,
    0x5af037cd as ::core::ffi::c_uint,
    0xff6b144a as ::core::ffi::c_uint,
    0x33c114d4 as ::core::ffi::c_uint,
    0xbd4e1337 as ::core::ffi::c_uint,
    0x71e413a9 as ::core::ffi::c_uint,
    0x7b211ab0 as ::core::ffi::c_uint,
    0xb78b1a2e as ::core::ffi::c_uint,
    0x39041dcd as ::core::ffi::c_uint,
    0xf5ae1d53 as ::core::ffi::c_uint,
    0x2c8e0fff as ::core::ffi::c_uint,
    0xe0240f61 as ::core::ffi::c_uint,
    0x6eab0882 as ::core::ffi::c_uint,
    0xa201081c as ::core::ffi::c_uint,
    0xa8c40105 as ::core::ffi::c_uint,
    0x646e019b as ::core::ffi::c_uint,
    0xeae10678 as ::core::ffi::c_uint,
    0x264b06e6 as ::core::ffi::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn lodepng_crc32(
    mut data: *const ::core::ffi::c_uchar,
    mut length: size_t,
) -> ::core::ffi::c_uint {
    let mut r: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    while length >= 8 as size_t {
        r = lodepng_crc32_table7[(*data.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_uint
            ^ r & 0xff as ::core::ffi::c_uint) as usize]
            ^ lodepng_crc32_table6[(*data.offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_uint
                ^ r >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint)
                as usize]
            ^ lodepng_crc32_table5[(*data.offset(2 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_uint
                ^ r >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint)
                as usize]
            ^ lodepng_crc32_table4[(*data.offset(3 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_uint
                ^ r >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint)
                as usize]
            ^ lodepng_crc32_table3[*data.offset(4 as ::core::ffi::c_int as isize) as usize]
            ^ lodepng_crc32_table2[*data.offset(5 as ::core::ffi::c_int as isize) as usize]
            ^ lodepng_crc32_table1[*data.offset(6 as ::core::ffi::c_int as isize) as usize]
            ^ lodepng_crc32_table0[*data.offset(7 as ::core::ffi::c_int as isize) as usize];
        data = data.offset(8 as ::core::ffi::c_int as isize);
        length = (length as ::core::ffi::c_ulong).wrapping_sub(8 as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
    loop {
        let fresh44 = length;
        length = length.wrapping_sub(1);
        if !(fresh44 != 0) {
            break;
        }
        let fresh45 = data;
        data = data.offset(1);
        r = lodepng_crc32_table0
            [((r ^ *fresh45 as ::core::ffi::c_uint) & 0xff as ::core::ffi::c_uint) as usize]
            ^ r >> 8 as ::core::ffi::c_int;
    }
    return r ^ 0xffffffff as ::core::ffi::c_uint;
}
unsafe extern "C" fn readBitFromReversedStream(
    mut bitpointer: *mut size_t,
    mut bitstream: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut result: ::core::ffi::c_uchar =
        (*bitstream.offset((*bitpointer >> 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            >> (7 as size_t).wrapping_sub(*bitpointer & 0x7 as size_t)
            & 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    *bitpointer = (*bitpointer).wrapping_add(1);
    return result;
}
unsafe extern "C" fn readBitsFromReversedStream(
    mut bitpointer: *mut size_t,
    mut bitstream: *const ::core::ffi::c_uchar,
    mut nbits: size_t,
) -> ::core::ffi::c_uint {
    let mut result: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < nbits {
        result <<= 1 as ::core::ffi::c_uint;
        result |= readBitFromReversedStream(bitpointer, bitstream) as ::core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    return result;
}
unsafe extern "C" fn setBitOfReversedStream(
    mut bitpointer: *mut size_t,
    mut bitstream: *mut ::core::ffi::c_uchar,
    mut bit: ::core::ffi::c_uchar,
) {
    if bit as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let ref mut fresh31 = *bitstream.offset((*bitpointer >> 3 as ::core::ffi::c_uint) as isize);
        *fresh31 = (*fresh31 as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_uint) << (7 as size_t).wrapping_sub(*bitpointer & 7 as size_t))
                as ::core::ffi::c_uchar as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
    } else {
        let ref mut fresh32 = *bitstream.offset((*bitpointer >> 3 as ::core::ffi::c_uint) as isize);
        *fresh32 = (*fresh32 as ::core::ffi::c_uint
            | (1 as ::core::ffi::c_uint) << (7 as size_t).wrapping_sub(*bitpointer & 7 as size_t))
            as ::core::ffi::c_uchar;
    }
    *bitpointer = (*bitpointer).wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_length(
    mut chunk: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    return lodepng_read32bitInt(chunk);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_type(
    mut type_0: *mut ::core::ffi::c_char,
    mut chunk: *const ::core::ffi::c_uchar,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i != 4 as ::core::ffi::c_uint {
        *type_0.offset(i as isize) = *chunk
            .offset((4 as ::core::ffi::c_uint).wrapping_add(i) as isize)
            as ::core::ffi::c_char;
        i = i.wrapping_add(1);
    }
    *type_0.offset(4 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_type_equals(
    mut chunk: *const ::core::ffi::c_uchar,
    mut type_0: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    if lodepng_strlen(type_0) != 4 as size_t {
        return 0 as ::core::ffi::c_uchar;
    }
    return (*chunk.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == *type_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        && *chunk.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == *type_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        && *chunk.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == *type_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        && *chunk.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == *type_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
unsafe extern "C" fn lodepng_chunk_type_name_valid(
    mut chunk: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i != 4 as ::core::ffi::c_uint {
        let mut c: ::core::ffi::c_char = *chunk
            .offset((4 as ::core::ffi::c_uint).wrapping_add(i) as isize)
            as ::core::ffi::c_char;
        if !(c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'z' as i32
            || c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'Z' as i32)
        {
            return 0 as ::core::ffi::c_uchar;
        }
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_ancillary(
    mut chunk: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    return (*chunk.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 32 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_private(
    mut chunk: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    return (*chunk.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 32 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
unsafe extern "C" fn lodepng_chunk_reserved(
    mut chunk: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    return (*chunk.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 32 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_safetocopy(
    mut chunk: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    return (*chunk.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 32 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_data(
    mut chunk: *mut ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_uchar {
    return chunk.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_data_const(
    mut chunk: *const ::core::ffi::c_uchar,
) -> *const ::core::ffi::c_uchar {
    return chunk.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_check_crc(
    mut chunk: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    let mut length: ::core::ffi::c_uint = lodepng_chunk_length(chunk);
    let mut crc: ::core::ffi::c_uint = lodepng_read32bitInt(
        chunk.offset(length.wrapping_add(8 as ::core::ffi::c_uint) as isize)
            as *const ::core::ffi::c_uchar,
    );
    let mut checksum: ::core::ffi::c_uint = lodepng_crc32(
        chunk.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_uchar,
        length.wrapping_add(4 as ::core::ffi::c_uint) as size_t,
    );
    if crc != checksum {
        return 1 as ::core::ffi::c_uint;
    } else {
        return 0 as ::core::ffi::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_generate_crc(mut chunk: *mut ::core::ffi::c_uchar) {
    let mut length: ::core::ffi::c_uint = lodepng_chunk_length(chunk);
    let mut crc: ::core::ffi::c_uint = lodepng_crc32(
        chunk.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
        length.wrapping_add(4 as ::core::ffi::c_uint) as size_t,
    );
    lodepng_set32bitInt(
        chunk
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(length as isize),
        crc,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_next(
    mut chunk: *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_uchar {
    let mut available_size: size_t = end.offset_from(chunk) as ::core::ffi::c_long as size_t;
    if chunk >= end || available_size < 12 as size_t {
        return end;
    }
    if *chunk.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0x89 as ::core::ffi::c_int
        && *chunk.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x50 as ::core::ffi::c_int
        && *chunk.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x4e as ::core::ffi::c_int
        && *chunk.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x47 as ::core::ffi::c_int
        && *chunk.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xd as ::core::ffi::c_int
        && *chunk.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xa as ::core::ffi::c_int
        && *chunk.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x1a as ::core::ffi::c_int
        && *chunk.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xa as ::core::ffi::c_int
    {
        return chunk.offset(8 as ::core::ffi::c_int as isize);
    } else {
        let mut total_chunk_length: size_t = 0;
        if lodepng_addofl(
            lodepng_chunk_length(chunk) as size_t,
            12 as size_t,
            &raw mut total_chunk_length,
        ) != 0
        {
            return end;
        }
        if total_chunk_length > available_size {
            return end;
        }
        return chunk.offset(total_chunk_length as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_next_const(
    mut chunk: *const ::core::ffi::c_uchar,
    mut end: *const ::core::ffi::c_uchar,
) -> *const ::core::ffi::c_uchar {
    let mut available_size: size_t = end.offset_from(chunk) as ::core::ffi::c_long as size_t;
    if chunk >= end || available_size < 12 as size_t {
        return end;
    }
    if *chunk.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0x89 as ::core::ffi::c_int
        && *chunk.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x50 as ::core::ffi::c_int
        && *chunk.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x4e as ::core::ffi::c_int
        && *chunk.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x47 as ::core::ffi::c_int
        && *chunk.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xd as ::core::ffi::c_int
        && *chunk.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xa as ::core::ffi::c_int
        && *chunk.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x1a as ::core::ffi::c_int
        && *chunk.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xa as ::core::ffi::c_int
    {
        return chunk.offset(8 as ::core::ffi::c_int as isize);
    } else {
        let mut total_chunk_length: size_t = 0;
        if lodepng_addofl(
            lodepng_chunk_length(chunk) as size_t,
            12 as size_t,
            &raw mut total_chunk_length,
        ) != 0
        {
            return end;
        }
        if total_chunk_length > available_size {
            return end;
        }
        return chunk.offset(total_chunk_length as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_find(
    mut chunk: *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut type_0: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_uchar {
    loop {
        if chunk >= end
            || (end.offset_from(chunk) as ::core::ffi::c_long) < 12 as ::core::ffi::c_long
        {
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if lodepng_chunk_type_equals(chunk, type_0 as *const ::core::ffi::c_char) != 0 {
            return chunk;
        }
        chunk = lodepng_chunk_next(chunk, end);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_find_const(
    mut chunk: *const ::core::ffi::c_uchar,
    mut end: *const ::core::ffi::c_uchar,
    mut type_0: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_uchar {
    loop {
        if chunk >= end
            || (end.offset_from(chunk) as ::core::ffi::c_long) < 12 as ::core::ffi::c_long
        {
            return ::core::ptr::null::<::core::ffi::c_uchar>();
        }
        if lodepng_chunk_type_equals(chunk, type_0 as *const ::core::ffi::c_char) != 0 {
            return chunk;
        }
        chunk = lodepng_chunk_next_const(chunk, end);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_append(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut chunk: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    let mut total_chunk_length: size_t = 0;
    let mut new_length: size_t = 0;
    let mut chunk_start: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut new_buffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if lodepng_chunk_type_name_valid(chunk) == 0 {
        return 121 as ::core::ffi::c_uint;
    }
    if lodepng_chunk_reserved(chunk) != 0 {
        return 122 as ::core::ffi::c_uint;
    }
    if lodepng_addofl(
        lodepng_chunk_length(chunk) as size_t,
        12 as size_t,
        &raw mut total_chunk_length,
    ) != 0
    {
        return 77 as ::core::ffi::c_uint;
    }
    if lodepng_addofl(*outsize, total_chunk_length, &raw mut new_length) != 0 {
        return 77 as ::core::ffi::c_uint;
    }
    new_buffer =
        lodepng_realloc(*out as *mut ::core::ffi::c_void, new_length) as *mut ::core::ffi::c_uchar;
    if new_buffer.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    *out = new_buffer;
    *outsize = new_length;
    chunk_start = (*out).offset(new_length.wrapping_sub(total_chunk_length) as isize)
        as *mut ::core::ffi::c_uchar;
    i = 0 as ::core::ffi::c_uint;
    while i as size_t != total_chunk_length {
        *chunk_start.offset(i as isize) = *chunk.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn lodepng_chunk_init(
    mut chunk: *mut *mut ::core::ffi::c_uchar,
    mut out: *mut ucvector,
    mut length: size_t,
    mut type_0: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut new_length: size_t = (*out).size;
    if lodepng_addofl(new_length, length, &raw mut new_length) != 0 {
        return 77 as ::core::ffi::c_uint;
    }
    if lodepng_addofl(new_length, 12 as size_t, &raw mut new_length) != 0 {
        return 77 as ::core::ffi::c_uint;
    }
    if ucvector_resize(out, new_length) == 0 {
        return 83 as ::core::ffi::c_uint;
    }
    *chunk = (*out)
        .data
        .offset(new_length as isize)
        .offset(-(length as isize))
        .offset(-(12 as ::core::ffi::c_uint as isize));
    lodepng_set32bitInt(*chunk, length as ::core::ffi::c_uint);
    lodepng_memcpy(
        (*chunk).offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        type_0 as *const ::core::ffi::c_void,
        4 as size_t,
    );
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn lodepng_chunk_createv(
    mut out: *mut ucvector,
    mut length: size_t,
    mut type_0: *const ::core::ffi::c_char,
    mut data: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(&raw mut chunk, out, length, type_0);
    if error_ != 0 {
        return error_;
    }
    lodepng_memcpy(
        chunk.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        length,
    );
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_create(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut length: size_t,
    mut type_0: *const ::core::ffi::c_char,
    mut data: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    let mut v: ucvector = ucvector_init(*out, *outsize);
    let mut error: ::core::ffi::c_uint = lodepng_chunk_createv(&raw mut v, length, type_0, data);
    *out = v.data;
    *outsize = v.size;
    return error;
}
unsafe extern "C" fn checkColorValidity(
    mut colortype: LodePNGColorType,
    mut bd: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    match colortype as ::core::ffi::c_uint {
        0 => {
            if !(bd == 1 as ::core::ffi::c_uint
                || bd == 2 as ::core::ffi::c_uint
                || bd == 4 as ::core::ffi::c_uint
                || bd == 8 as ::core::ffi::c_uint
                || bd == 16 as ::core::ffi::c_uint)
            {
                return 37 as ::core::ffi::c_uint;
            }
        }
        2 => {
            if !(bd == 8 as ::core::ffi::c_uint || bd == 16 as ::core::ffi::c_uint) {
                return 37 as ::core::ffi::c_uint;
            }
        }
        3 => {
            if !(bd == 1 as ::core::ffi::c_uint
                || bd == 2 as ::core::ffi::c_uint
                || bd == 4 as ::core::ffi::c_uint
                || bd == 8 as ::core::ffi::c_uint)
            {
                return 37 as ::core::ffi::c_uint;
            }
        }
        4 => {
            if !(bd == 8 as ::core::ffi::c_uint || bd == 16 as ::core::ffi::c_uint) {
                return 37 as ::core::ffi::c_uint;
            }
        }
        6 => {
            if !(bd == 8 as ::core::ffi::c_uint || bd == 16 as ::core::ffi::c_uint) {
                return 37 as ::core::ffi::c_uint;
            }
        }
        255 => return 31 as ::core::ffi::c_uint,
        _ => return 31 as ::core::ffi::c_uint,
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn getNumColorChannels(mut colortype: LodePNGColorType) -> ::core::ffi::c_uint {
    match colortype as ::core::ffi::c_uint {
        0 => return 1 as ::core::ffi::c_uint,
        2 => return 3 as ::core::ffi::c_uint,
        3 => return 1 as ::core::ffi::c_uint,
        4 => return 2 as ::core::ffi::c_uint,
        6 => return 4 as ::core::ffi::c_uint,
        255 => return 0 as ::core::ffi::c_uint,
        _ => return 0 as ::core::ffi::c_uint,
    };
}
unsafe extern "C" fn lodepng_get_bpp_lct(
    mut colortype: LodePNGColorType,
    mut bitdepth: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return getNumColorChannels(colortype).wrapping_mul(bitdepth);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_mode_init(mut info: *mut LodePNGColorMode) {
    (*info).key_defined = 0 as ::core::ffi::c_uint;
    (*info).key_b = 0 as ::core::ffi::c_uint;
    (*info).key_g = (*info).key_b;
    (*info).key_r = (*info).key_g;
    (*info).colortype = LCT_RGBA;
    (*info).bitdepth = 8 as ::core::ffi::c_uint;
    (*info).palette = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*info).palettesize = 0 as size_t;
}
unsafe extern "C" fn lodepng_color_mode_alloc_palette(mut info: *mut LodePNGColorMode) {
    let mut i: size_t = 0;
    if (*info).palette.is_null() {
        (*info).palette = lodepng_malloc(1024 as size_t) as *mut ::core::ffi::c_uchar;
    }
    if (*info).palette.is_null() {
        return;
    }
    i = 0 as size_t;
    while i != 256 as size_t {
        *(*info)
            .palette
            .offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize) =
            0 as ::core::ffi::c_uchar;
        *(*info)
            .palette
            .offset(i.wrapping_mul(4 as size_t).wrapping_add(1 as size_t) as isize) =
            0 as ::core::ffi::c_uchar;
        *(*info)
            .palette
            .offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize) =
            0 as ::core::ffi::c_uchar;
        *(*info)
            .palette
            .offset(i.wrapping_mul(4 as size_t).wrapping_add(3 as size_t) as isize) =
            255 as ::core::ffi::c_uchar;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_mode_cleanup(mut info: *mut LodePNGColorMode) {
    lodepng_palette_clear(info);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_mode_copy(
    mut dest: *mut LodePNGColorMode,
    mut source: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    lodepng_color_mode_cleanup(dest);
    lodepng_memcpy(
        dest as *mut ::core::ffi::c_void,
        source as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LodePNGColorMode>() as size_t,
    );
    if !(*source).palette.is_null() {
        (*dest).palette = lodepng_malloc(1024 as size_t) as *mut ::core::ffi::c_uchar;
        if (*dest).palette.is_null() && (*source).palettesize != 0 {
            return 83 as ::core::ffi::c_uint;
        }
        lodepng_memcpy(
            (*dest).palette as *mut ::core::ffi::c_void,
            (*source).palette as *const ::core::ffi::c_void,
            (*source).palettesize.wrapping_mul(4 as size_t),
        );
    }
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_mode_make(
    mut colortype: LodePNGColorType,
    mut bitdepth: ::core::ffi::c_uint,
) -> LodePNGColorMode {
    let mut result: LodePNGColorMode = LodePNGColorMode {
        colortype: LCT_GREY,
        bitdepth: 0,
        palette: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        palettesize: 0,
        key_defined: 0,
        key_r: 0,
        key_g: 0,
        key_b: 0,
    };
    lodepng_color_mode_init(&raw mut result);
    result.colortype = colortype;
    result.bitdepth = bitdepth;
    return result;
}
unsafe extern "C" fn lodepng_color_mode_equal(
    mut a: *const LodePNGColorMode,
    mut b: *const LodePNGColorMode,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    if (*a).colortype as ::core::ffi::c_uint != (*b).colortype as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    if (*a).bitdepth != (*b).bitdepth {
        return 0 as ::core::ffi::c_int;
    }
    if (*a).key_defined != (*b).key_defined {
        return 0 as ::core::ffi::c_int;
    }
    if (*a).key_defined != 0 {
        if (*a).key_r != (*b).key_r {
            return 0 as ::core::ffi::c_int;
        }
        if (*a).key_g != (*b).key_g {
            return 0 as ::core::ffi::c_int;
        }
        if (*a).key_b != (*b).key_b {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*a).palettesize != (*b).palettesize {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as size_t;
    while i != (*a).palettesize.wrapping_mul(4 as size_t) {
        if *(*a).palette.offset(i as isize) as ::core::ffi::c_int
            != *(*b).palette.offset(i as isize) as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_palette_clear(mut info: *mut LodePNGColorMode) {
    if !(*info).palette.is_null() {
        lodepng_free((*info).palette as *mut ::core::ffi::c_void);
    }
    (*info).palette = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*info).palettesize = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_palette_add(
    mut info: *mut LodePNGColorMode,
    mut r: ::core::ffi::c_uchar,
    mut g: ::core::ffi::c_uchar,
    mut b: ::core::ffi::c_uchar,
    mut a: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    if (*info).palette.is_null() {
        lodepng_color_mode_alloc_palette(info);
        if (*info).palette.is_null() {
            return 83 as ::core::ffi::c_uint;
        }
    }
    if (*info).palettesize >= 256 as size_t {
        return 108 as ::core::ffi::c_uint;
    }
    *(*info).palette.offset(
        (4 as size_t)
            .wrapping_mul((*info).palettesize)
            .wrapping_add(0 as size_t) as isize,
    ) = r;
    *(*info).palette.offset(
        (4 as size_t)
            .wrapping_mul((*info).palettesize)
            .wrapping_add(1 as size_t) as isize,
    ) = g;
    *(*info).palette.offset(
        (4 as size_t)
            .wrapping_mul((*info).palettesize)
            .wrapping_add(2 as size_t) as isize,
    ) = b;
    *(*info).palette.offset(
        (4 as size_t)
            .wrapping_mul((*info).palettesize)
            .wrapping_add(3 as size_t) as isize,
    ) = a;
    (*info).palettesize = (*info).palettesize.wrapping_add(1);
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_get_bpp(mut info: *const LodePNGColorMode) -> ::core::ffi::c_uint {
    return lodepng_get_bpp_lct((*info).colortype, (*info).bitdepth);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_get_channels(
    mut info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    return getNumColorChannels((*info).colortype);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_is_greyscale_type(
    mut info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    return ((*info).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*info).colortype as ::core::ffi::c_uint
            == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_is_alpha_type(
    mut info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    return ((*info).colortype as ::core::ffi::c_uint & 4 as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_is_palette_type(
    mut info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    return ((*info).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_has_palette_alpha(
    mut info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i != (*info).palettesize {
        if (*(*info)
            .palette
            .offset(i.wrapping_mul(4 as size_t).wrapping_add(3 as size_t) as isize)
            as ::core::ffi::c_int)
            < 255 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_uint;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_can_have_alpha(
    mut info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    return ((*info).key_defined != 0
        || lodepng_is_alpha_type(info) != 0
        || lodepng_has_palette_alpha(info) != 0) as ::core::ffi::c_int
        as ::core::ffi::c_uint;
}
unsafe extern "C" fn lodepng_get_raw_size_lct(
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut colortype: LodePNGColorType,
    mut bitdepth: ::core::ffi::c_uint,
) -> size_t {
    let mut bpp: size_t = lodepng_get_bpp_lct(colortype, bitdepth) as size_t;
    let mut n: size_t = (w as size_t).wrapping_mul(h as size_t);
    return n.wrapping_div(8 as size_t).wrapping_mul(bpp).wrapping_add(
        (n & 7 as size_t)
            .wrapping_mul(bpp)
            .wrapping_add(7 as size_t)
            .wrapping_div(8 as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_get_raw_size(
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut color: *const LodePNGColorMode,
) -> size_t {
    return lodepng_get_raw_size_lct(w, h, (*color).colortype, (*color).bitdepth);
}
unsafe extern "C" fn lodepng_get_raw_size_idat(
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut bpp: ::core::ffi::c_uint,
) -> size_t {
    let mut line: size_t = (w.wrapping_div(8 as ::core::ffi::c_uint) as size_t)
        .wrapping_mul(bpp as size_t)
        .wrapping_add(1 as size_t)
        .wrapping_add(
            (w & 7 as ::core::ffi::c_uint)
                .wrapping_mul(bpp)
                .wrapping_add(7 as ::core::ffi::c_uint)
                .wrapping_div(8 as ::core::ffi::c_uint) as size_t,
        );
    return (h as size_t).wrapping_mul(line);
}
unsafe extern "C" fn lodepng_pixel_overflow(
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut pngcolor: *const LodePNGColorMode,
    mut rawcolor: *const LodePNGColorMode,
) -> ::core::ffi::c_int {
    let mut bpp: size_t = (if lodepng_get_bpp(pngcolor) > lodepng_get_bpp(rawcolor) {
        lodepng_get_bpp(pngcolor)
    } else {
        lodepng_get_bpp(rawcolor)
    }) as size_t;
    let mut numpixels: size_t = 0;
    let mut total: size_t = 0;
    let mut line: size_t = 0;
    if lodepng_mulofl(w as size_t, h as size_t, &raw mut numpixels) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if lodepng_mulofl(numpixels, 8 as size_t, &raw mut total) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if lodepng_mulofl(
        w.wrapping_div(8 as ::core::ffi::c_uint) as size_t,
        bpp,
        &raw mut line,
    ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if lodepng_addofl(
        line,
        ((w & 7 as ::core::ffi::c_uint) as size_t)
            .wrapping_mul(bpp)
            .wrapping_add(7 as size_t)
            .wrapping_div(8 as size_t),
        &raw mut line,
    ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if lodepng_addofl(line, 5 as size_t, &raw mut line) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if lodepng_mulofl(line, h as size_t, &raw mut total) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn LodePNGUnknownChunks_init(mut info: *mut LodePNGInfo) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i != 3 as ::core::ffi::c_uint {
        (*info).unknown_chunks_data[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        i = i.wrapping_add(1);
    }
    i = 0 as ::core::ffi::c_uint;
    while i != 3 as ::core::ffi::c_uint {
        (*info).unknown_chunks_size[i as usize] = 0 as size_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn LodePNGUnknownChunks_cleanup(mut info: *mut LodePNGInfo) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i != 3 as ::core::ffi::c_uint {
        lodepng_free((*info).unknown_chunks_data[i as usize] as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn LodePNGUnknownChunks_copy(
    mut dest: *mut LodePNGInfo,
    mut src: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    LodePNGUnknownChunks_cleanup(dest);
    i = 0 as ::core::ffi::c_uint;
    while i != 3 as ::core::ffi::c_uint {
        let mut j: size_t = 0;
        (*dest).unknown_chunks_size[i as usize] = (*src).unknown_chunks_size[i as usize];
        (*dest).unknown_chunks_data[i as usize] =
            lodepng_malloc((*src).unknown_chunks_size[i as usize]) as *mut ::core::ffi::c_uchar;
        if (*dest).unknown_chunks_data[i as usize].is_null()
            && (*dest).unknown_chunks_size[i as usize] != 0
        {
            return 83 as ::core::ffi::c_uint;
        }
        j = 0 as size_t;
        while j < (*src).unknown_chunks_size[i as usize] {
            *(*dest).unknown_chunks_data[i as usize].offset(j as isize) =
                *(*src).unknown_chunks_data[i as usize].offset(j as isize);
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn LodePNGText_init(mut info: *mut LodePNGInfo) {
    (*info).text_num = 0 as size_t;
    (*info).text_keys = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*info).text_strings = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
}
unsafe extern "C" fn LodePNGText_cleanup(mut info: *mut LodePNGInfo) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i != (*info).text_num {
        lodepng_free(*(*info).text_keys.offset(i as isize) as *mut ::core::ffi::c_void);
        lodepng_free(*(*info).text_strings.offset(i as isize) as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    lodepng_free((*info).text_keys as *mut ::core::ffi::c_void);
    lodepng_free((*info).text_strings as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn LodePNGText_copy(
    mut dest: *mut LodePNGInfo,
    mut source: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut i: size_t = 0 as size_t;
    (*dest).text_keys = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*dest).text_strings = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*dest).text_num = 0 as size_t;
    i = 0 as size_t;
    while i != (*source).text_num {
        let mut error_: ::core::ffi::c_uint = lodepng_add_text(
            dest,
            *(*source).text_keys.offset(i as isize),
            *(*source).text_strings.offset(i as isize),
        );
        if error_ != 0 {
            return error_;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn lodepng_add_text_sized(
    mut info: *mut LodePNGInfo,
    mut key: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_uint {
    let mut new_keys: *mut *mut ::core::ffi::c_char = lodepng_realloc(
        (*info).text_keys as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul((*info).text_num.wrapping_add(1 as size_t)),
    ) as *mut *mut ::core::ffi::c_char;
    let mut new_strings: *mut *mut ::core::ffi::c_char = lodepng_realloc(
        (*info).text_strings as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul((*info).text_num.wrapping_add(1 as size_t)),
    ) as *mut *mut ::core::ffi::c_char;
    if !new_keys.is_null() {
        (*info).text_keys = new_keys;
    }
    if !new_strings.is_null() {
        (*info).text_strings = new_strings;
    }
    if new_keys.is_null() || new_strings.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    (*info).text_num = (*info).text_num.wrapping_add(1);
    let ref mut fresh50 = *(*info)
        .text_keys
        .offset((*info).text_num.wrapping_sub(1 as size_t) as isize);
    *fresh50 = alloc_string(key);
    let ref mut fresh51 = *(*info)
        .text_strings
        .offset((*info).text_num.wrapping_sub(1 as size_t) as isize);
    *fresh51 = alloc_string_sized(str, size);
    if (*(*info)
        .text_keys
        .offset((*info).text_num.wrapping_sub(1 as size_t) as isize))
    .is_null()
        || (*(*info)
            .text_strings
            .offset((*info).text_num.wrapping_sub(1 as size_t) as isize))
        .is_null()
    {
        return 83 as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_add_text(
    mut info: *mut LodePNGInfo,
    mut key: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    return lodepng_add_text_sized(info, key, str, lodepng_strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_clear_text(mut info: *mut LodePNGInfo) {
    LodePNGText_cleanup(info);
    LodePNGText_init(info);
}
unsafe extern "C" fn LodePNGIText_init(mut info: *mut LodePNGInfo) {
    (*info).itext_num = 0 as size_t;
    (*info).itext_keys = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*info).itext_langtags = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*info).itext_transkeys = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*info).itext_strings = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
}
unsafe extern "C" fn LodePNGIText_cleanup(mut info: *mut LodePNGInfo) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i != (*info).itext_num {
        lodepng_free(*(*info).itext_keys.offset(i as isize) as *mut ::core::ffi::c_void);
        lodepng_free(*(*info).itext_langtags.offset(i as isize) as *mut ::core::ffi::c_void);
        lodepng_free(*(*info).itext_transkeys.offset(i as isize) as *mut ::core::ffi::c_void);
        lodepng_free(*(*info).itext_strings.offset(i as isize) as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    lodepng_free((*info).itext_keys as *mut ::core::ffi::c_void);
    lodepng_free((*info).itext_langtags as *mut ::core::ffi::c_void);
    lodepng_free((*info).itext_transkeys as *mut ::core::ffi::c_void);
    lodepng_free((*info).itext_strings as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn LodePNGIText_copy(
    mut dest: *mut LodePNGInfo,
    mut source: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut i: size_t = 0 as size_t;
    (*dest).itext_keys = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*dest).itext_langtags = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*dest).itext_transkeys = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*dest).itext_strings = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*dest).itext_num = 0 as size_t;
    i = 0 as size_t;
    while i != (*source).itext_num {
        let mut error_: ::core::ffi::c_uint = lodepng_add_itext(
            dest,
            *(*source).itext_keys.offset(i as isize),
            *(*source).itext_langtags.offset(i as isize),
            *(*source).itext_transkeys.offset(i as isize),
            *(*source).itext_strings.offset(i as isize),
        );
        if error_ != 0 {
            return error_;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_clear_itext(mut info: *mut LodePNGInfo) {
    LodePNGIText_cleanup(info);
    LodePNGIText_init(info);
}
unsafe extern "C" fn lodepng_add_itext_sized(
    mut info: *mut LodePNGInfo,
    mut key: *const ::core::ffi::c_char,
    mut langtag: *const ::core::ffi::c_char,
    mut transkey: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_uint {
    let mut new_keys: *mut *mut ::core::ffi::c_char = lodepng_realloc(
        (*info).itext_keys as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul((*info).itext_num.wrapping_add(1 as size_t)),
    ) as *mut *mut ::core::ffi::c_char;
    let mut new_langtags: *mut *mut ::core::ffi::c_char = lodepng_realloc(
        (*info).itext_langtags as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul((*info).itext_num.wrapping_add(1 as size_t)),
    ) as *mut *mut ::core::ffi::c_char;
    let mut new_transkeys: *mut *mut ::core::ffi::c_char = lodepng_realloc(
        (*info).itext_transkeys as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul((*info).itext_num.wrapping_add(1 as size_t)),
    ) as *mut *mut ::core::ffi::c_char;
    let mut new_strings: *mut *mut ::core::ffi::c_char = lodepng_realloc(
        (*info).itext_strings as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul((*info).itext_num.wrapping_add(1 as size_t)),
    ) as *mut *mut ::core::ffi::c_char;
    if !new_keys.is_null() {
        (*info).itext_keys = new_keys;
    }
    if !new_langtags.is_null() {
        (*info).itext_langtags = new_langtags;
    }
    if !new_transkeys.is_null() {
        (*info).itext_transkeys = new_transkeys;
    }
    if !new_strings.is_null() {
        (*info).itext_strings = new_strings;
    }
    if new_keys.is_null()
        || new_langtags.is_null()
        || new_transkeys.is_null()
        || new_strings.is_null()
    {
        return 83 as ::core::ffi::c_uint;
    }
    (*info).itext_num = (*info).itext_num.wrapping_add(1);
    let ref mut fresh46 = *(*info)
        .itext_keys
        .offset((*info).itext_num.wrapping_sub(1 as size_t) as isize);
    *fresh46 = alloc_string(key);
    let ref mut fresh47 = *(*info)
        .itext_langtags
        .offset((*info).itext_num.wrapping_sub(1 as size_t) as isize);
    *fresh47 = alloc_string(langtag);
    let ref mut fresh48 = *(*info)
        .itext_transkeys
        .offset((*info).itext_num.wrapping_sub(1 as size_t) as isize);
    *fresh48 = alloc_string(transkey);
    let ref mut fresh49 = *(*info)
        .itext_strings
        .offset((*info).itext_num.wrapping_sub(1 as size_t) as isize);
    *fresh49 = alloc_string_sized(str, size);
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_add_itext(
    mut info: *mut LodePNGInfo,
    mut key: *const ::core::ffi::c_char,
    mut langtag: *const ::core::ffi::c_char,
    mut transkey: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    return lodepng_add_itext_sized(info, key, langtag, transkey, str, lodepng_strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_set_icc(
    mut info: *mut LodePNGInfo,
    mut name: *const ::core::ffi::c_char,
    mut profile: *const ::core::ffi::c_uchar,
    mut profile_size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if (*info).iccp_defined != 0 {
        lodepng_clear_icc(info);
    }
    if profile_size == 0 as ::core::ffi::c_uint {
        return 123 as ::core::ffi::c_uint;
    }
    (*info).iccp_name = alloc_string(name);
    if (*info).iccp_name.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    (*info).iccp_profile = lodepng_malloc(profile_size as size_t) as *mut ::core::ffi::c_uchar;
    if (*info).iccp_profile.is_null() {
        lodepng_free((*info).iccp_name as *mut ::core::ffi::c_void);
        return 83 as ::core::ffi::c_uint;
    }
    lodepng_memcpy(
        (*info).iccp_profile as *mut ::core::ffi::c_void,
        profile as *const ::core::ffi::c_void,
        profile_size as size_t,
    );
    (*info).iccp_profile_size = profile_size;
    (*info).iccp_defined = 1 as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn lodepng_init_icc(mut info: *mut LodePNGInfo) {
    (*info).iccp_defined = 0 as ::core::ffi::c_uint;
    (*info).iccp_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*info).iccp_profile = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*info).iccp_profile_size = 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_clear_icc(mut info: *mut LodePNGInfo) {
    lodepng_free((*info).iccp_name as *mut ::core::ffi::c_void);
    lodepng_free((*info).iccp_profile as *mut ::core::ffi::c_void);
    lodepng_init_icc(info);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_set_exif(
    mut info: *mut LodePNGInfo,
    mut exif: *const ::core::ffi::c_uchar,
    mut exif_size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if (*info).exif_defined != 0 {
        lodepng_clear_exif(info);
    }
    (*info).exif = lodepng_malloc(exif_size as size_t) as *mut ::core::ffi::c_uchar;
    if (*info).exif.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    lodepng_memcpy(
        (*info).exif as *mut ::core::ffi::c_void,
        exif as *const ::core::ffi::c_void,
        exif_size as size_t,
    );
    (*info).exif_size = exif_size;
    (*info).exif_defined = 1 as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn lodepng_init_exif(mut info: *mut LodePNGInfo) {
    (*info).exif_defined = 0 as ::core::ffi::c_uint;
    (*info).exif = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*info).exif_size = 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_clear_exif(mut info: *mut LodePNGInfo) {
    lodepng_free((*info).exif as *mut ::core::ffi::c_void);
    lodepng_init_exif(info);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_info_init(mut info: *mut LodePNGInfo) {
    lodepng_color_mode_init(&raw mut (*info).color);
    (*info).interlace_method = 0 as ::core::ffi::c_uint;
    (*info).compression_method = 0 as ::core::ffi::c_uint;
    (*info).filter_method = 0 as ::core::ffi::c_uint;
    (*info).background_defined = 0 as ::core::ffi::c_uint;
    (*info).background_b = 0 as ::core::ffi::c_uint;
    (*info).background_g = (*info).background_b;
    (*info).background_r = (*info).background_g;
    LodePNGText_init(info);
    LodePNGIText_init(info);
    lodepng_init_icc(info);
    lodepng_init_exif(info);
    (*info).time_defined = 0 as ::core::ffi::c_uint;
    (*info).phys_defined = 0 as ::core::ffi::c_uint;
    (*info).gama_defined = 0 as ::core::ffi::c_uint;
    (*info).chrm_defined = 0 as ::core::ffi::c_uint;
    (*info).srgb_defined = 0 as ::core::ffi::c_uint;
    (*info).cicp_defined = 0 as ::core::ffi::c_uint;
    (*info).cicp_color_primaries = 0 as ::core::ffi::c_uint;
    (*info).cicp_transfer_function = 0 as ::core::ffi::c_uint;
    (*info).cicp_matrix_coefficients = 0 as ::core::ffi::c_uint;
    (*info).cicp_video_full_range_flag = 0 as ::core::ffi::c_uint;
    (*info).mdcv_defined = 0 as ::core::ffi::c_uint;
    (*info).mdcv_red_x = 0 as ::core::ffi::c_uint;
    (*info).mdcv_red_y = 0 as ::core::ffi::c_uint;
    (*info).mdcv_green_x = 0 as ::core::ffi::c_uint;
    (*info).mdcv_green_y = 0 as ::core::ffi::c_uint;
    (*info).mdcv_blue_x = 0 as ::core::ffi::c_uint;
    (*info).mdcv_blue_y = 0 as ::core::ffi::c_uint;
    (*info).mdcv_white_x = 0 as ::core::ffi::c_uint;
    (*info).mdcv_white_y = 0 as ::core::ffi::c_uint;
    (*info).mdcv_max_luminance = 0 as ::core::ffi::c_uint;
    (*info).mdcv_min_luminance = 0 as ::core::ffi::c_uint;
    (*info).clli_defined = 0 as ::core::ffi::c_uint;
    (*info).clli_max_cll = 0 as ::core::ffi::c_uint;
    (*info).clli_max_fall = 0 as ::core::ffi::c_uint;
    (*info).sbit_defined = 0 as ::core::ffi::c_uint;
    (*info).sbit_a = 0 as ::core::ffi::c_uint;
    (*info).sbit_b = (*info).sbit_a;
    (*info).sbit_g = (*info).sbit_b;
    (*info).sbit_r = (*info).sbit_g;
    LodePNGUnknownChunks_init(info);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_info_cleanup(mut info: *mut LodePNGInfo) {
    lodepng_color_mode_cleanup(&raw mut (*info).color);
    LodePNGText_cleanup(info);
    LodePNGIText_cleanup(info);
    lodepng_clear_icc(info);
    lodepng_clear_exif(info);
    LodePNGUnknownChunks_cleanup(info);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_info_copy(
    mut dest: *mut LodePNGInfo,
    mut source: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    lodepng_info_cleanup(dest);
    lodepng_memcpy(
        dest as *mut ::core::ffi::c_void,
        source as *const ::core::ffi::c_void,
        ::core::mem::size_of::<LodePNGInfo>() as size_t,
    );
    lodepng_color_mode_init(&raw mut (*dest).color);
    LodePNGText_init(dest);
    LodePNGIText_init(dest);
    lodepng_init_icc(dest);
    lodepng_init_exif(dest);
    LodePNGUnknownChunks_init(dest);
    let mut error_: ::core::ffi::c_uint =
        lodepng_color_mode_copy(&raw mut (*dest).color, &raw const (*source).color);
    if error_ != 0 {
        return error_;
    }
    let mut error__0: ::core::ffi::c_uint = LodePNGText_copy(dest, source);
    if error__0 != 0 {
        return error__0;
    }
    let mut error__1: ::core::ffi::c_uint = LodePNGIText_copy(dest, source);
    if error__1 != 0 {
        return error__1;
    }
    if (*source).iccp_defined != 0 {
        let mut error__2: ::core::ffi::c_uint = lodepng_set_icc(
            dest,
            (*source).iccp_name,
            (*source).iccp_profile,
            (*source).iccp_profile_size,
        );
        if error__2 != 0 {
            return error__2;
        }
    }
    if (*source).exif_defined != 0 {
        let mut error__3: ::core::ffi::c_uint =
            lodepng_set_exif(dest, (*source).exif, (*source).exif_size);
        if error__3 != 0 {
            return error__3;
        }
    }
    let mut error__4: ::core::ffi::c_uint = LodePNGUnknownChunks_copy(dest, source);
    if error__4 != 0 {
        return error__4;
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addColorBits(
    mut out: *mut ::core::ffi::c_uchar,
    mut index: size_t,
    mut bits: ::core::ffi::c_uint,
    mut in_0: ::core::ffi::c_uint,
) {
    let mut m: ::core::ffi::c_uint = (if bits == 1 as ::core::ffi::c_uint {
        7 as ::core::ffi::c_int
    } else if bits == 2 as ::core::ffi::c_uint {
        3 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    let mut p: ::core::ffi::c_uint = (index & m as size_t) as ::core::ffi::c_uint;
    in_0 &= ((1 as ::core::ffi::c_uint) << bits).wrapping_sub(1 as ::core::ffi::c_uint);
    in_0 = in_0 << bits.wrapping_mul(m.wrapping_sub(p));
    if p == 0 as ::core::ffi::c_uint {
        *out.offset(index.wrapping_mul(bits as size_t).wrapping_div(8 as size_t) as isize) =
            in_0 as ::core::ffi::c_uchar;
    } else {
        let ref mut fresh10 =
            *out.offset(index.wrapping_mul(bits as size_t).wrapping_div(8 as size_t) as isize);
        *fresh10 = (*fresh10 as ::core::ffi::c_uint | in_0) as ::core::ffi::c_uchar;
    };
}
unsafe extern "C" fn color_tree_init(mut tree: *mut ColorTree) {
    lodepng_memset(
        &raw mut (*tree).children as *mut *mut ColorTree as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (16 as size_t).wrapping_mul(::core::mem::size_of::<*mut ColorTree>() as size_t),
    );
    (*tree).index = -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn color_tree_cleanup(mut tree: *mut ColorTree) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i != 16 as ::core::ffi::c_int {
        if !(*tree).children[i as usize].is_null() {
            color_tree_cleanup((*tree).children[i as usize]);
            lodepng_free((*tree).children[i as usize] as *mut ::core::ffi::c_void);
        }
        i += 1;
    }
}
unsafe extern "C" fn color_tree_get(
    mut tree: *mut ColorTree,
    mut r: ::core::ffi::c_uchar,
    mut g: ::core::ffi::c_uchar,
    mut b: ::core::ffi::c_uchar,
    mut a: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut bit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    bit = 0 as ::core::ffi::c_int;
    while bit < 8 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 8 as ::core::ffi::c_int
            * (r as ::core::ffi::c_int >> bit & 1 as ::core::ffi::c_int)
            + 4 as ::core::ffi::c_int * (g as ::core::ffi::c_int >> bit & 1 as ::core::ffi::c_int)
            + 2 as ::core::ffi::c_int * (b as ::core::ffi::c_int >> bit & 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int * (a as ::core::ffi::c_int >> bit & 1 as ::core::ffi::c_int);
        if (*tree).children[i as usize].is_null() {
            return -(1 as ::core::ffi::c_int);
        } else {
            tree = (*tree).children[i as usize];
        }
        bit += 1;
    }
    return if !tree.is_null() {
        (*tree).index
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
unsafe extern "C" fn color_tree_has(
    mut tree: *mut ColorTree,
    mut r: ::core::ffi::c_uchar,
    mut g: ::core::ffi::c_uchar,
    mut b: ::core::ffi::c_uchar,
    mut a: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return (color_tree_get(tree, r, g, b, a) >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn color_tree_add(
    mut tree: *mut ColorTree,
    mut r: ::core::ffi::c_uchar,
    mut g: ::core::ffi::c_uchar,
    mut b: ::core::ffi::c_uchar,
    mut a: ::core::ffi::c_uchar,
    mut index: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut bit: ::core::ffi::c_int = 0;
    bit = 0 as ::core::ffi::c_int;
    while bit < 8 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 8 as ::core::ffi::c_int
            * (r as ::core::ffi::c_int >> bit & 1 as ::core::ffi::c_int)
            + 4 as ::core::ffi::c_int * (g as ::core::ffi::c_int >> bit & 1 as ::core::ffi::c_int)
            + 2 as ::core::ffi::c_int * (b as ::core::ffi::c_int >> bit & 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int * (a as ::core::ffi::c_int >> bit & 1 as ::core::ffi::c_int);
        if (*tree).children[i as usize].is_null() {
            (*tree).children[i as usize] =
                lodepng_malloc(::core::mem::size_of::<ColorTree>() as size_t) as *mut ColorTree;
            if (*tree).children[i as usize].is_null() {
                return 83 as ::core::ffi::c_uint;
            }
            color_tree_init((*tree).children[i as usize]);
        }
        tree = (*tree).children[i as usize];
        bit += 1;
    }
    (*tree).index = index as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn rgba8ToPixel(
    mut out: *mut ::core::ffi::c_uchar,
    mut i: size_t,
    mut mode: *const LodePNGColorMode,
    mut tree: *mut ColorTree,
    mut r: ::core::ffi::c_uchar,
    mut g: ::core::ffi::c_uchar,
    mut b: ::core::ffi::c_uchar,
    mut a: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut gray: ::core::ffi::c_uchar = r;
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            *out.offset(i as isize) = gray;
        } else if (*mode).bitdepth == 16 as ::core::ffi::c_uint {
            let ref mut fresh0 =
                *out.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize);
            *fresh0 = gray;
            *out.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize) = *fresh0;
        } else {
            gray = (gray as ::core::ffi::c_uint
                >> (8 as ::core::ffi::c_uint).wrapping_sub((*mode).bitdepth)
                & ((1 as ::core::ffi::c_uint) << (*mode).bitdepth)
                    .wrapping_sub(1 as ::core::ffi::c_uint))
                as ::core::ffi::c_uchar;
            addColorBits(out, i, (*mode).bitdepth, gray as ::core::ffi::c_uint);
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            *out.offset(i.wrapping_mul(3 as size_t).wrapping_add(0 as size_t) as isize) = r;
            *out.offset(i.wrapping_mul(3 as size_t).wrapping_add(1 as size_t) as isize) = g;
            *out.offset(i.wrapping_mul(3 as size_t).wrapping_add(2 as size_t) as isize) = b;
        } else {
            let ref mut fresh1 =
                *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(1 as size_t) as isize);
            *fresh1 = r;
            *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(0 as size_t) as isize) = *fresh1;
            let ref mut fresh2 =
                *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(3 as size_t) as isize);
            *fresh2 = g;
            *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(2 as size_t) as isize) = *fresh2;
            let ref mut fresh3 =
                *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(5 as size_t) as isize);
            *fresh3 = b;
            *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(4 as size_t) as isize) = *fresh3;
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut index: ::core::ffi::c_int = color_tree_get(tree, r, g, b, a);
        if index < 0 as ::core::ffi::c_int {
            return 82 as ::core::ffi::c_uint;
        }
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            *out.offset(i as isize) = index as ::core::ffi::c_uchar;
        } else {
            addColorBits(out, i, (*mode).bitdepth, index as ::core::ffi::c_uint);
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut gray_0: ::core::ffi::c_uchar = r;
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            *out.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize) = gray_0;
            *out.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize) = a;
        } else if (*mode).bitdepth == 16 as ::core::ffi::c_uint {
            let ref mut fresh4 =
                *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(1 as size_t) as isize);
            *fresh4 = gray_0;
            *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize) = *fresh4;
            let ref mut fresh5 =
                *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(3 as size_t) as isize);
            *fresh5 = a;
            *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize) = *fresh5;
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize) = r;
            *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(1 as size_t) as isize) = g;
            *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize) = b;
            *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(3 as size_t) as isize) = a;
        } else {
            let ref mut fresh6 =
                *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(1 as size_t) as isize);
            *fresh6 = r;
            *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(0 as size_t) as isize) = *fresh6;
            let ref mut fresh7 =
                *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(3 as size_t) as isize);
            *fresh7 = g;
            *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(2 as size_t) as isize) = *fresh7;
            let ref mut fresh8 =
                *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(5 as size_t) as isize);
            *fresh8 = b;
            *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(4 as size_t) as isize) = *fresh8;
            let ref mut fresh9 =
                *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(7 as size_t) as isize);
            *fresh9 = a;
            *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(6 as size_t) as isize) = *fresh9;
        }
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn rgba16ToPixel(
    mut out: *mut ::core::ffi::c_uchar,
    mut i: size_t,
    mut mode: *const LodePNGColorMode,
    mut r: ::core::ffi::c_ushort,
    mut g: ::core::ffi::c_ushort,
    mut b: ::core::ffi::c_ushort,
    mut a: ::core::ffi::c_ushort,
) {
    if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut gray: ::core::ffi::c_ushort = r;
        *out.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize) =
            (gray as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize) =
            (gray as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(0 as size_t) as isize) =
            (r as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(1 as size_t) as isize) =
            (r as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(2 as size_t) as isize) =
            (g as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(3 as size_t) as isize) =
            (g as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(4 as size_t) as isize) =
            (b as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(6 as size_t).wrapping_add(5 as size_t) as isize) =
            (b as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut gray_0: ::core::ffi::c_ushort = r;
        *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize) =
            (gray_0 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(1 as size_t) as isize) =
            (gray_0 as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize) =
            (a as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(4 as size_t).wrapping_add(3 as size_t) as isize) =
            (a as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(0 as size_t) as isize) =
            (r as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(1 as size_t) as isize) =
            (r as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(2 as size_t) as isize) =
            (g as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(3 as size_t) as isize) =
            (g as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(4 as size_t) as isize) =
            (b as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(5 as size_t) as isize) =
            (b as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(6 as size_t) as isize) =
            (a as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        *out.offset(i.wrapping_mul(8 as size_t).wrapping_add(7 as size_t) as isize) =
            (a as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
}
unsafe extern "C" fn getPixelColorRGBA8(
    mut r: *mut ::core::ffi::c_uchar,
    mut g: *mut ::core::ffi::c_uchar,
    mut b: *mut ::core::ffi::c_uchar,
    mut a: *mut ::core::ffi::c_uchar,
    mut in_0: *const ::core::ffi::c_uchar,
    mut i: size_t,
    mut mode: *const LodePNGColorMode,
) {
    if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            *b = *in_0.offset(i as isize);
            *g = *b;
            *r = *g;
            if (*mode).key_defined != 0 && *r as ::core::ffi::c_uint == (*mode).key_r {
                *a = 0 as ::core::ffi::c_uchar;
            } else {
                *a = 255 as ::core::ffi::c_uchar;
            }
        } else if (*mode).bitdepth == 16 as ::core::ffi::c_uint {
            *b = *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize);
            *g = *b;
            *r = *g;
            if (*mode).key_defined != 0
                && (256 as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_uint,
                    )
                    == (*mode).key_r
            {
                *a = 0 as ::core::ffi::c_uchar;
            } else {
                *a = 255 as ::core::ffi::c_uchar;
            }
        } else {
            let mut highest: ::core::ffi::c_uint = ((1 as ::core::ffi::c_uint) << (*mode).bitdepth)
                .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut j: size_t = i.wrapping_mul((*mode).bitdepth as size_t);
            let mut value: ::core::ffi::c_uint =
                readBitsFromReversedStream(&raw mut j, in_0, (*mode).bitdepth as size_t);
            *b = value
                .wrapping_mul(255 as ::core::ffi::c_uint)
                .wrapping_div(highest) as ::core::ffi::c_uchar;
            *g = *b;
            *r = *g;
            if (*mode).key_defined != 0 && value == (*mode).key_r {
                *a = 0 as ::core::ffi::c_uchar;
            } else {
                *a = 255 as ::core::ffi::c_uchar;
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            *r = *in_0.offset(i.wrapping_mul(3 as size_t).wrapping_add(0 as size_t) as isize);
            *g = *in_0.offset(i.wrapping_mul(3 as size_t).wrapping_add(1 as size_t) as isize);
            *b = *in_0.offset(i.wrapping_mul(3 as size_t).wrapping_add(2 as size_t) as isize);
            if (*mode).key_defined != 0
                && *r as ::core::ffi::c_uint == (*mode).key_r
                && *g as ::core::ffi::c_uint == (*mode).key_g
                && *b as ::core::ffi::c_uint == (*mode).key_b
            {
                *a = 0 as ::core::ffi::c_uchar;
            } else {
                *a = 255 as ::core::ffi::c_uchar;
            }
        } else {
            *r = *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(0 as size_t) as isize);
            *g = *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(2 as size_t) as isize);
            *b = *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(4 as size_t) as isize);
            if (*mode).key_defined != 0
                && (256 as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(0 as size_t) as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_uint,
                    )
                    == (*mode).key_r
                && (256 as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(2 as size_t) as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(3 as size_t) as isize)
                            as ::core::ffi::c_uint,
                    )
                    == (*mode).key_g
                && (256 as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(4 as size_t) as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(5 as size_t) as isize)
                            as ::core::ffi::c_uint,
                    )
                    == (*mode).key_b
            {
                *a = 0 as ::core::ffi::c_uchar;
            } else {
                *a = 255 as ::core::ffi::c_uchar;
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut index: ::core::ffi::c_uint = 0;
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            index = *in_0.offset(i as isize) as ::core::ffi::c_uint;
        } else {
            let mut j_0: size_t = i.wrapping_mul((*mode).bitdepth as size_t);
            index = readBitsFromReversedStream(&raw mut j_0, in_0, (*mode).bitdepth as size_t);
        }
        *r = *(*mode).palette.offset(
            index
                .wrapping_mul(4 as ::core::ffi::c_uint)
                .wrapping_add(0 as ::core::ffi::c_uint) as isize,
        );
        *g = *(*mode).palette.offset(
            index
                .wrapping_mul(4 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as isize,
        );
        *b = *(*mode).palette.offset(
            index
                .wrapping_mul(4 as ::core::ffi::c_uint)
                .wrapping_add(2 as ::core::ffi::c_uint) as isize,
        );
        *a = *(*mode).palette.offset(
            index
                .wrapping_mul(4 as ::core::ffi::c_uint)
                .wrapping_add(3 as ::core::ffi::c_uint) as isize,
        );
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            *b = *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize);
            *g = *b;
            *r = *g;
            *a = *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize);
        } else {
            *b = *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize);
            *g = *b;
            *r = *g;
            *a = *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize);
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            *r = *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize);
            *g = *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(1 as size_t) as isize);
            *b = *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize);
            *a = *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(3 as size_t) as isize);
        } else {
            *r = *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(0 as size_t) as isize);
            *g = *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(2 as size_t) as isize);
            *b = *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(4 as size_t) as isize);
            *a = *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(6 as size_t) as isize);
        }
    }
}
unsafe extern "C" fn getPixelColorsRGBA8(
    mut buffer: *mut ::core::ffi::c_uchar,
    mut numpixels: size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut mode: *const LodePNGColorMode,
) {
    let mut num_channels: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
    let mut i: size_t = 0;
    if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                let ref mut fresh21 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh21 = *in_0.offset(i as isize);
                let ref mut fresh22 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh22 = *fresh21;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh22;
                *buffer.offset(3 as ::core::ffi::c_int as isize) = 255 as ::core::ffi::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
            if (*mode).key_defined != 0 {
                buffer = buffer.offset(-(numpixels.wrapping_mul(num_channels as size_t) as isize));
                i = 0 as size_t;
                while i != numpixels {
                    if *buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                        == (*mode).key_r
                    {
                        *buffer.offset(3 as ::core::ffi::c_int as isize) =
                            0 as ::core::ffi::c_uchar;
                    }
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*mode).bitdepth == 16 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                let ref mut fresh23 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh23 = *in_0.offset(i.wrapping_mul(2 as size_t) as isize);
                let ref mut fresh24 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh24 = *fresh23;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh24;
                *buffer.offset(3 as ::core::ffi::c_int as isize) =
                    (if (*mode).key_defined != 0
                        && (256 as ::core::ffi::c_uint)
                            .wrapping_mul(*in_0.offset(
                                i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize,
                            ) as ::core::ffi::c_uint)
                            .wrapping_add(*in_0.offset(
                                i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize,
                            ) as ::core::ffi::c_uint)
                            == (*mode).key_r
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        255 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            let mut highest: ::core::ffi::c_uint = ((1 as ::core::ffi::c_uint) << (*mode).bitdepth)
                .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut j: size_t = 0 as size_t;
            i = 0 as size_t;
            while i != numpixels {
                let mut value: ::core::ffi::c_uint =
                    readBitsFromReversedStream(&raw mut j, in_0, (*mode).bitdepth as size_t);
                let ref mut fresh25 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh25 = value
                    .wrapping_mul(255 as ::core::ffi::c_uint)
                    .wrapping_div(highest) as ::core::ffi::c_uchar;
                let ref mut fresh26 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh26 = *fresh25;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh26;
                *buffer.offset(3 as ::core::ffi::c_int as isize) =
                    (if (*mode).key_defined != 0 && value == (*mode).key_r {
                        0 as ::core::ffi::c_int
                    } else {
                        255 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                lodepng_memcpy(
                    buffer as *mut ::core::ffi::c_void,
                    in_0.offset(i.wrapping_mul(3 as size_t) as isize) as *const ::core::ffi::c_uchar
                        as *const ::core::ffi::c_void,
                    3 as size_t,
                );
                *buffer.offset(3 as ::core::ffi::c_int as isize) = 255 as ::core::ffi::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
            if (*mode).key_defined != 0 {
                buffer = buffer.offset(-(numpixels.wrapping_mul(num_channels as size_t) as isize));
                i = 0 as size_t;
                while i != numpixels {
                    if *buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                        == (*mode).key_r
                        && *buffer.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                            == (*mode).key_g
                        && *buffer.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                            == (*mode).key_b
                    {
                        *buffer.offset(3 as ::core::ffi::c_int as isize) =
                            0 as ::core::ffi::c_uchar;
                    }
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else {
            i = 0 as size_t;
            while i != numpixels {
                *buffer.offset(0 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(0 as size_t) as isize);
                *buffer.offset(1 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(2 as size_t) as isize);
                *buffer.offset(2 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(4 as size_t) as isize);
                *buffer.offset(3 as ::core::ffi::c_int as isize) =
                    (if (*mode).key_defined != 0
                        && (256 as ::core::ffi::c_uint)
                            .wrapping_mul(*in_0.offset(
                                i.wrapping_mul(6 as size_t).wrapping_add(0 as size_t) as isize,
                            ) as ::core::ffi::c_uint)
                            .wrapping_add(*in_0.offset(
                                i.wrapping_mul(6 as size_t).wrapping_add(1 as size_t) as isize,
                            ) as ::core::ffi::c_uint)
                            == (*mode).key_r
                        && (256 as ::core::ffi::c_uint)
                            .wrapping_mul(*in_0.offset(
                                i.wrapping_mul(6 as size_t).wrapping_add(2 as size_t) as isize,
                            ) as ::core::ffi::c_uint)
                            .wrapping_add(*in_0.offset(
                                i.wrapping_mul(6 as size_t).wrapping_add(3 as size_t) as isize,
                            ) as ::core::ffi::c_uint)
                            == (*mode).key_g
                        && (256 as ::core::ffi::c_uint)
                            .wrapping_mul(*in_0.offset(
                                i.wrapping_mul(6 as size_t).wrapping_add(4 as size_t) as isize,
                            ) as ::core::ffi::c_uint)
                            .wrapping_add(*in_0.offset(
                                i.wrapping_mul(6 as size_t).wrapping_add(5 as size_t) as isize,
                            ) as ::core::ffi::c_uint)
                            == (*mode).key_b
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        255 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                let mut index: ::core::ffi::c_uint =
                    *in_0.offset(i as isize) as ::core::ffi::c_uint;
                lodepng_memcpy(
                    buffer as *mut ::core::ffi::c_void,
                    (*mode)
                        .palette
                        .offset(index.wrapping_mul(4 as ::core::ffi::c_uint) as isize)
                        as *mut ::core::ffi::c_uchar
                        as *const ::core::ffi::c_void,
                    4 as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            let mut j_0: size_t = 0 as size_t;
            i = 0 as size_t;
            while i != numpixels {
                let mut index_0: ::core::ffi::c_uint =
                    readBitsFromReversedStream(&raw mut j_0, in_0, (*mode).bitdepth as size_t);
                lodepng_memcpy(
                    buffer as *mut ::core::ffi::c_void,
                    (*mode)
                        .palette
                        .offset(index_0.wrapping_mul(4 as ::core::ffi::c_uint) as isize)
                        as *mut ::core::ffi::c_uchar
                        as *const ::core::ffi::c_void,
                    4 as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                let ref mut fresh27 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh27 =
                    *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize);
                let ref mut fresh28 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh28 = *fresh27;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh28;
                *buffer.offset(3 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize);
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            i = 0 as size_t;
            while i != numpixels {
                let ref mut fresh29 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh29 =
                    *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize);
                let ref mut fresh30 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh30 = *fresh29;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh30;
                *buffer.offset(3 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize);
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            lodepng_memcpy(
                buffer as *mut ::core::ffi::c_void,
                in_0 as *const ::core::ffi::c_void,
                numpixels.wrapping_mul(4 as size_t),
            );
        } else {
            i = 0 as size_t;
            while i != numpixels {
                *buffer.offset(0 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(0 as size_t) as isize);
                *buffer.offset(1 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(2 as size_t) as isize);
                *buffer.offset(2 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(4 as size_t) as isize);
                *buffer.offset(3 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(6 as size_t) as isize);
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    }
}
unsafe extern "C" fn getPixelColorsRGB8(
    mut buffer: *mut ::core::ffi::c_uchar,
    mut numpixels: size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut mode: *const LodePNGColorMode,
) {
    let num_channels: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
    let mut i: size_t = 0;
    if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                let ref mut fresh11 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh11 = *in_0.offset(i as isize);
                let ref mut fresh12 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh12 = *fresh11;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh12;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else if (*mode).bitdepth == 16 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                let ref mut fresh13 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh13 = *in_0.offset(i.wrapping_mul(2 as size_t) as isize);
                let ref mut fresh14 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh14 = *fresh13;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh14;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            let mut highest: ::core::ffi::c_uint = ((1 as ::core::ffi::c_uint) << (*mode).bitdepth)
                .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut j: size_t = 0 as size_t;
            i = 0 as size_t;
            while i != numpixels {
                let mut value: ::core::ffi::c_uint =
                    readBitsFromReversedStream(&raw mut j, in_0, (*mode).bitdepth as size_t);
                let ref mut fresh15 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh15 = value
                    .wrapping_mul(255 as ::core::ffi::c_uint)
                    .wrapping_div(highest) as ::core::ffi::c_uchar;
                let ref mut fresh16 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh16 = *fresh15;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh16;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            lodepng_memcpy(
                buffer as *mut ::core::ffi::c_void,
                in_0 as *const ::core::ffi::c_void,
                numpixels.wrapping_mul(3 as size_t),
            );
        } else {
            i = 0 as size_t;
            while i != numpixels {
                *buffer.offset(0 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(0 as size_t) as isize);
                *buffer.offset(1 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(2 as size_t) as isize);
                *buffer.offset(2 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(4 as size_t) as isize);
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                let mut index: ::core::ffi::c_uint =
                    *in_0.offset(i as isize) as ::core::ffi::c_uint;
                lodepng_memcpy(
                    buffer as *mut ::core::ffi::c_void,
                    (*mode)
                        .palette
                        .offset(index.wrapping_mul(4 as ::core::ffi::c_uint) as isize)
                        as *mut ::core::ffi::c_uchar
                        as *const ::core::ffi::c_void,
                    3 as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            let mut j_0: size_t = 0 as size_t;
            i = 0 as size_t;
            while i != numpixels {
                let mut index_0: ::core::ffi::c_uint =
                    readBitsFromReversedStream(&raw mut j_0, in_0, (*mode).bitdepth as size_t);
                lodepng_memcpy(
                    buffer as *mut ::core::ffi::c_void,
                    (*mode)
                        .palette
                        .offset(index_0.wrapping_mul(4 as ::core::ffi::c_uint) as isize)
                        as *mut ::core::ffi::c_uchar
                        as *const ::core::ffi::c_void,
                    3 as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                let ref mut fresh17 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh17 =
                    *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize);
                let ref mut fresh18 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh18 = *fresh17;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh18;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            i = 0 as size_t;
            while i != numpixels {
                let ref mut fresh19 = *buffer.offset(2 as ::core::ffi::c_int as isize);
                *fresh19 =
                    *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize);
                let ref mut fresh20 = *buffer.offset(1 as ::core::ffi::c_int as isize);
                *fresh20 = *fresh19;
                *buffer.offset(0 as ::core::ffi::c_int as isize) = *fresh20;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*mode).bitdepth == 8 as ::core::ffi::c_uint {
            i = 0 as size_t;
            while i != numpixels {
                lodepng_memcpy(
                    buffer as *mut ::core::ffi::c_void,
                    in_0.offset(i.wrapping_mul(4 as size_t) as isize) as *const ::core::ffi::c_uchar
                        as *const ::core::ffi::c_void,
                    3 as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            i = 0 as size_t;
            while i != numpixels {
                *buffer.offset(0 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(0 as size_t) as isize);
                *buffer.offset(1 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(2 as size_t) as isize);
                *buffer.offset(2 as ::core::ffi::c_int as isize) =
                    *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(4 as size_t) as isize);
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    }
}
unsafe extern "C" fn getPixelColorRGBA16(
    mut r: *mut ::core::ffi::c_ushort,
    mut g: *mut ::core::ffi::c_ushort,
    mut b: *mut ::core::ffi::c_ushort,
    mut a: *mut ::core::ffi::c_ushort,
    mut in_0: *const ::core::ffi::c_uchar,
    mut i: size_t,
    mut mode: *const LodePNGColorMode,
) {
    if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *b = (256 as ::core::ffi::c_int
            * *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize)
                as ::core::ffi::c_int
            + *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int) as ::core::ffi::c_ushort;
        *g = *b;
        *r = *g;
        if (*mode).key_defined != 0
            && (256 as ::core::ffi::c_uint)
                .wrapping_mul(
                    *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(0 as size_t) as isize)
                        as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    *in_0.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize)
                        as ::core::ffi::c_uint,
                )
                == (*mode).key_r
        {
            *a = 0 as ::core::ffi::c_ushort;
        } else {
            *a = 65535 as ::core::ffi::c_ushort;
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *r = (256 as ::core::ffi::c_uint)
            .wrapping_mul(
                *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(0 as size_t) as isize)
                    as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
        *g = (256 as ::core::ffi::c_uint)
            .wrapping_mul(
                *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(2 as size_t) as isize)
                    as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(3 as size_t) as isize)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
        *b = (256 as ::core::ffi::c_uint)
            .wrapping_mul(
                *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(4 as size_t) as isize)
                    as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(5 as size_t) as isize)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
        if (*mode).key_defined != 0
            && (256 as ::core::ffi::c_uint)
                .wrapping_mul(
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(0 as size_t) as isize)
                        as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(1 as size_t) as isize)
                        as ::core::ffi::c_uint,
                )
                == (*mode).key_r
            && (256 as ::core::ffi::c_uint)
                .wrapping_mul(
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(2 as size_t) as isize)
                        as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(3 as size_t) as isize)
                        as ::core::ffi::c_uint,
                )
                == (*mode).key_g
            && (256 as ::core::ffi::c_uint)
                .wrapping_mul(
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(4 as size_t) as isize)
                        as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    *in_0.offset(i.wrapping_mul(6 as size_t).wrapping_add(5 as size_t) as isize)
                        as ::core::ffi::c_uint,
                )
                == (*mode).key_b
        {
            *a = 0 as ::core::ffi::c_ushort;
        } else {
            *a = 65535 as ::core::ffi::c_ushort;
        }
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *b = (256 as ::core::ffi::c_uint)
            .wrapping_mul(
                *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize)
                    as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
        *g = *b;
        *r = *g;
        *a = (256 as ::core::ffi::c_uint)
            .wrapping_mul(
                *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize)
                    as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *in_0.offset(i.wrapping_mul(4 as size_t).wrapping_add(3 as size_t) as isize)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
    } else if (*mode).colortype as ::core::ffi::c_uint
        == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *r = (256 as ::core::ffi::c_uint)
            .wrapping_mul(
                *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(0 as size_t) as isize)
                    as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
        *g = (256 as ::core::ffi::c_uint)
            .wrapping_mul(
                *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(2 as size_t) as isize)
                    as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(3 as size_t) as isize)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
        *b = (256 as ::core::ffi::c_uint)
            .wrapping_mul(
                *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(4 as size_t) as isize)
                    as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(5 as size_t) as isize)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
        *a = (256 as ::core::ffi::c_uint)
            .wrapping_mul(
                *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(6 as size_t) as isize)
                    as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *in_0.offset(i.wrapping_mul(8 as size_t).wrapping_add(7 as size_t) as isize)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ushort;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_convert(
    mut out: *mut ::core::ffi::c_uchar,
    mut in_0: *const ::core::ffi::c_uchar,
    mut mode_out: *const LodePNGColorMode,
    mut mode_in: *const LodePNGColorMode,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut i: size_t = 0;
    let mut tree: ColorTree = ColorTree {
        children: [::core::ptr::null_mut::<ColorTree>(); 16],
        index: 0,
    };
    let mut numpixels: size_t = (w as size_t).wrapping_mul(h as size_t);
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if (*mode_in).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*mode_in).palette.is_null()
    {
        return 107 as ::core::ffi::c_uint;
    }
    if lodepng_color_mode_equal(mode_out, mode_in) != 0 {
        let mut numbytes: size_t = lodepng_get_raw_size(w, h, mode_in);
        lodepng_memcpy(
            out as *mut ::core::ffi::c_void,
            in_0 as *const ::core::ffi::c_void,
            numbytes,
        );
        return 0 as ::core::ffi::c_uint;
    }
    if (*mode_out).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut palettesize: size_t = (*mode_out).palettesize;
        let mut palette: *const ::core::ffi::c_uchar = (*mode_out).palette;
        let mut palsize: size_t = (1 as ::core::ffi::c_uint as size_t) << (*mode_out).bitdepth;
        if palettesize == 0 as size_t {
            palettesize = (*mode_in).palettesize;
            palette = (*mode_in).palette;
            if (*mode_in).colortype as ::core::ffi::c_uint
                == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*mode_in).bitdepth == (*mode_out).bitdepth
            {
                let mut numbytes_0: size_t = lodepng_get_raw_size(w, h, mode_in);
                lodepng_memcpy(
                    out as *mut ::core::ffi::c_void,
                    in_0 as *const ::core::ffi::c_void,
                    numbytes_0,
                );
                return 0 as ::core::ffi::c_uint;
            }
        }
        if palettesize < palsize {
            palsize = palettesize;
        }
        color_tree_init(&raw mut tree);
        i = 0 as size_t;
        while i != palsize {
            let mut p: *const ::core::ffi::c_uchar =
                palette.offset(i.wrapping_mul(4 as size_t) as isize) as *const ::core::ffi::c_uchar;
            error = color_tree_add(
                &raw mut tree,
                *p.offset(0 as ::core::ffi::c_int as isize),
                *p.offset(1 as ::core::ffi::c_int as isize),
                *p.offset(2 as ::core::ffi::c_int as isize),
                *p.offset(3 as ::core::ffi::c_int as isize),
                i as ::core::ffi::c_uint,
            );
            if error != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
    }
    if error == 0 {
        if (*mode_in).bitdepth == 16 as ::core::ffi::c_uint
            && (*mode_out).bitdepth == 16 as ::core::ffi::c_uint
        {
            i = 0 as size_t;
            while i != numpixels {
                let mut r: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                let mut g: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                let mut b: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                let mut a: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                getPixelColorRGBA16(
                    &raw mut r, &raw mut g, &raw mut b, &raw mut a, in_0, i, mode_in,
                );
                rgba16ToPixel(out, i, mode_out, r, g, b, a);
                i = i.wrapping_add(1);
            }
        } else if (*mode_out).bitdepth == 8 as ::core::ffi::c_uint
            && (*mode_out).colortype as ::core::ffi::c_uint
                == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            getPixelColorsRGBA8(out, numpixels, in_0, mode_in);
        } else if (*mode_out).bitdepth == 8 as ::core::ffi::c_uint
            && (*mode_out).colortype as ::core::ffi::c_uint
                == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            getPixelColorsRGB8(out, numpixels, in_0, mode_in);
        } else {
            let mut r_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
            let mut g_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
            let mut b_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
            let mut a_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
            i = 0 as size_t;
            while i != numpixels {
                getPixelColorRGBA8(
                    &raw mut r_0,
                    &raw mut g_0,
                    &raw mut b_0,
                    &raw mut a_0,
                    in_0,
                    i,
                    mode_in,
                );
                error = rgba8ToPixel(out, i, mode_out, &raw mut tree, r_0, g_0, b_0, a_0);
                if error != 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
    }
    if (*mode_out).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        color_tree_cleanup(&raw mut tree);
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_convert_rgb(
    mut r_out: *mut ::core::ffi::c_uint,
    mut g_out: *mut ::core::ffi::c_uint,
    mut b_out: *mut ::core::ffi::c_uint,
    mut r_in: ::core::ffi::c_uint,
    mut g_in: ::core::ffi::c_uint,
    mut b_in: ::core::ffi::c_uint,
    mut mode_out: *const LodePNGColorMode,
    mut mode_in: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let mut r: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut g: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut b: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut mul: ::core::ffi::c_uint = (65535 as ::core::ffi::c_uint).wrapping_div(
        ((1 as ::core::ffi::c_uint) << (*mode_in).bitdepth).wrapping_sub(1 as ::core::ffi::c_uint),
    );
    let mut shift: ::core::ffi::c_uint =
        (16 as ::core::ffi::c_uint).wrapping_sub((*mode_out).bitdepth);
    if (*mode_in).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*mode_in).colortype as ::core::ffi::c_uint
            == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        b = r_in.wrapping_mul(mul);
        g = b;
        r = g;
    } else if (*mode_in).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*mode_in).colortype as ::core::ffi::c_uint
            == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        r = r_in.wrapping_mul(mul);
        g = g_in.wrapping_mul(mul);
        b = b_in.wrapping_mul(mul);
    } else if (*mode_in).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if r_in as size_t >= (*mode_in).palettesize {
            return 82 as ::core::ffi::c_uint;
        }
        r = (*(*mode_in).palette.offset(
            r_in.wrapping_mul(4 as ::core::ffi::c_uint)
                .wrapping_add(0 as ::core::ffi::c_uint) as isize,
        ) as ::core::ffi::c_uint)
            .wrapping_mul(257 as ::core::ffi::c_uint);
        g = (*(*mode_in).palette.offset(
            r_in.wrapping_mul(4 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as isize,
        ) as ::core::ffi::c_uint)
            .wrapping_mul(257 as ::core::ffi::c_uint);
        b = (*(*mode_in).palette.offset(
            r_in.wrapping_mul(4 as ::core::ffi::c_uint)
                .wrapping_add(2 as ::core::ffi::c_uint) as isize,
        ) as ::core::ffi::c_uint)
            .wrapping_mul(257 as ::core::ffi::c_uint);
    } else {
        return 31 as ::core::ffi::c_uint;
    }
    if (*mode_out).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*mode_out).colortype as ::core::ffi::c_uint
            == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *r_out = r >> shift;
    } else if (*mode_out).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*mode_out).colortype as ::core::ffi::c_uint
            == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *r_out = r >> shift;
        *g_out = g >> shift;
        *b_out = b >> shift;
    } else if (*mode_out).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut i: ::core::ffi::c_uint = 0;
        if r >> 8 as ::core::ffi::c_int != r & 255 as ::core::ffi::c_uint
            || g >> 8 as ::core::ffi::c_int != g & 255 as ::core::ffi::c_uint
            || b >> 8 as ::core::ffi::c_int != b & 255 as ::core::ffi::c_uint
        {
            return 82 as ::core::ffi::c_uint;
        }
        i = 0 as ::core::ffi::c_uint;
        while (i as size_t) < (*mode_out).palettesize {
            let mut j: ::core::ffi::c_uint = i.wrapping_mul(4 as ::core::ffi::c_uint);
            if r >> 8 as ::core::ffi::c_int
                == *(*mode_out)
                    .palette
                    .offset(j.wrapping_add(0 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_uint
                && g >> 8 as ::core::ffi::c_int
                    == *(*mode_out)
                        .palette
                        .offset(j.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_uint
                && b >> 8 as ::core::ffi::c_int
                    == *(*mode_out)
                        .palette
                        .offset(j.wrapping_add(2 as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_uint
            {
                *r_out = i;
                return 0 as ::core::ffi::c_uint;
            }
            i = i.wrapping_add(1);
        }
        return 82 as ::core::ffi::c_uint;
    } else {
        return 31 as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_stats_init(mut stats: *mut LodePNGColorStats) {
    (*stats).colored = 0 as ::core::ffi::c_uint;
    (*stats).key = 0 as ::core::ffi::c_uint;
    (*stats).key_b = 0 as ::core::ffi::c_ushort;
    (*stats).key_g = (*stats).key_b;
    (*stats).key_r = (*stats).key_g;
    (*stats).alpha = 0 as ::core::ffi::c_uint;
    (*stats).numcolors = 0 as ::core::ffi::c_uint;
    (*stats).bits = 1 as ::core::ffi::c_uint;
    (*stats).numpixels = 0 as size_t;
    (*stats).allow_palette = 1 as ::core::ffi::c_uint;
    (*stats).allow_greyscale = 1 as ::core::ffi::c_uint;
}
unsafe extern "C" fn getValueRequiredBits(mut value: ::core::ffi::c_uchar) -> ::core::ffi::c_uint {
    if value as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || value as ::core::ffi::c_int == 255 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_uint;
    }
    if value as ::core::ffi::c_int % 17 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return (if value as ::core::ffi::c_int % 85 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            2 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
    }
    return 8 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_compute_color_stats(
    mut stats: *mut LodePNGColorStats,
    mut in_0: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut mode_in: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let mut current_block: u64;
    let mut i: size_t = 0;
    let mut tree: ColorTree = ColorTree {
        children: [::core::ptr::null_mut::<ColorTree>(); 16],
        index: 0,
    };
    let mut numpixels: size_t = (w as size_t).wrapping_mul(h as size_t);
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut colored_done: ::core::ffi::c_uint = (if lodepng_is_greyscale_type(mode_in) != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    let mut alpha_done: ::core::ffi::c_uint = (if lodepng_can_have_alpha(mode_in) != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    let mut numcolors_done: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut bpp: ::core::ffi::c_uint = lodepng_get_bpp(mode_in);
    let mut bits_done: ::core::ffi::c_uint =
        (if (*stats).bits == 1 as ::core::ffi::c_uint && bpp == 1 as ::core::ffi::c_uint {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
    let mut sixteen: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut maxnumcolors: ::core::ffi::c_uint = 257 as ::core::ffi::c_uint;
    if bpp <= 8 as ::core::ffi::c_uint {
        maxnumcolors = if (257 as ::core::ffi::c_uint)
            < (*stats)
                .numcolors
                .wrapping_add((1 as ::core::ffi::c_uint) << bpp)
        {
            257 as ::core::ffi::c_uint
        } else {
            (*stats)
                .numcolors
                .wrapping_add((1 as ::core::ffi::c_uint) << bpp)
        };
    }
    (*stats).numpixels = ((*stats).numpixels as ::core::ffi::c_ulong)
        .wrapping_add(numpixels as ::core::ffi::c_ulong) as size_t
        as size_t;
    if (*stats).allow_palette == 0 {
        numcolors_done = 1 as ::core::ffi::c_uint;
    }
    color_tree_init(&raw mut tree);
    if (*stats).alpha != 0 {
        alpha_done = 1 as ::core::ffi::c_uint;
    }
    if (*stats).colored != 0 {
        colored_done = 1 as ::core::ffi::c_uint;
    }
    if (*stats).bits == 16 as ::core::ffi::c_uint {
        numcolors_done = 1 as ::core::ffi::c_uint;
    }
    if (*stats).bits >= bpp {
        bits_done = 1 as ::core::ffi::c_uint;
    }
    if (*stats).numcolors >= maxnumcolors {
        numcolors_done = 1 as ::core::ffi::c_uint;
    }
    if numcolors_done == 0 {
        i = 0 as size_t;
        loop {
            if !(i < (*stats).numcolors as size_t) {
                current_block = 14576567515993809846;
                break;
            }
            let mut color: *const ::core::ffi::c_uchar = (&raw mut (*stats).palette
                as *mut ::core::ffi::c_uchar)
                .offset(i.wrapping_mul(4 as size_t) as isize)
                as *mut ::core::ffi::c_uchar;
            error = color_tree_add(
                &raw mut tree,
                *color.offset(0 as ::core::ffi::c_int as isize),
                *color.offset(1 as ::core::ffi::c_int as isize),
                *color.offset(2 as ::core::ffi::c_int as isize),
                *color.offset(3 as ::core::ffi::c_int as isize),
                i as ::core::ffi::c_uint,
            );
            if error != 0 {
                current_block = 2950568455507082615;
                break;
            }
            i = i.wrapping_add(1);
        }
    } else {
        current_block = 14576567515993809846;
    }
    match current_block {
        14576567515993809846 => {
            if (*mode_in).bitdepth == 16 as ::core::ffi::c_uint && sixteen == 0 {
                let mut r: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                let mut g: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                let mut b: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                let mut a: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                i = 0 as size_t;
                while i != numpixels {
                    getPixelColorRGBA16(
                        &raw mut r, &raw mut g, &raw mut b, &raw mut a, in_0, i, mode_in,
                    );
                    if r as ::core::ffi::c_int & 255 as ::core::ffi::c_int
                        != r as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                            & 255 as ::core::ffi::c_int
                        || g as ::core::ffi::c_int & 255 as ::core::ffi::c_int
                            != g as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                                & 255 as ::core::ffi::c_int
                        || b as ::core::ffi::c_int & 255 as ::core::ffi::c_int
                            != b as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                                & 255 as ::core::ffi::c_int
                        || a as ::core::ffi::c_int & 255 as ::core::ffi::c_int
                            != a as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                                & 255 as ::core::ffi::c_int
                    {
                        (*stats).bits = 16 as ::core::ffi::c_uint;
                        sixteen = 1 as ::core::ffi::c_uint;
                        bits_done = 1 as ::core::ffi::c_uint;
                        numcolors_done = 1 as ::core::ffi::c_uint;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
            }
            if sixteen != 0 {
                let mut r_0: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                let mut g_0: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                let mut b_0: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                let mut a_0: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
                i = 0 as size_t;
                while i != numpixels {
                    getPixelColorRGBA16(
                        &raw mut r_0,
                        &raw mut g_0,
                        &raw mut b_0,
                        &raw mut a_0,
                        in_0,
                        i,
                        mode_in,
                    );
                    if colored_done == 0
                        && (r_0 as ::core::ffi::c_int != g_0 as ::core::ffi::c_int
                            || r_0 as ::core::ffi::c_int != b_0 as ::core::ffi::c_int)
                    {
                        (*stats).colored = 1 as ::core::ffi::c_uint;
                        colored_done = 1 as ::core::ffi::c_uint;
                    }
                    if alpha_done == 0 {
                        let mut matchkey: ::core::ffi::c_uint = (r_0 as ::core::ffi::c_int
                            == (*stats).key_r as ::core::ffi::c_int
                            && g_0 as ::core::ffi::c_int == (*stats).key_g as ::core::ffi::c_int
                            && b_0 as ::core::ffi::c_int == (*stats).key_b as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_uint;
                        if a_0 as ::core::ffi::c_int != 65535 as ::core::ffi::c_int
                            && (a_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                || (*stats).key != 0 && matchkey == 0)
                        {
                            (*stats).alpha = 1 as ::core::ffi::c_uint;
                            (*stats).key = 0 as ::core::ffi::c_uint;
                            alpha_done = 1 as ::core::ffi::c_uint;
                        } else if a_0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && (*stats).alpha == 0
                            && (*stats).key == 0
                        {
                            (*stats).key = 1 as ::core::ffi::c_uint;
                            (*stats).key_r = r_0;
                            (*stats).key_g = g_0;
                            (*stats).key_b = b_0;
                        } else if a_0 as ::core::ffi::c_int == 65535 as ::core::ffi::c_int
                            && (*stats).key != 0
                            && matchkey != 0
                        {
                            (*stats).alpha = 1 as ::core::ffi::c_uint;
                            (*stats).key = 0 as ::core::ffi::c_uint;
                            alpha_done = 1 as ::core::ffi::c_uint;
                        }
                    }
                    if alpha_done != 0 && numcolors_done != 0 && colored_done != 0 && bits_done != 0
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if (*stats).key != 0 && (*stats).alpha == 0 {
                    i = 0 as size_t;
                    while i != numpixels {
                        getPixelColorRGBA16(
                            &raw mut r_0,
                            &raw mut g_0,
                            &raw mut b_0,
                            &raw mut a_0,
                            in_0,
                            i,
                            mode_in,
                        );
                        if a_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                            && r_0 as ::core::ffi::c_int == (*stats).key_r as ::core::ffi::c_int
                            && g_0 as ::core::ffi::c_int == (*stats).key_g as ::core::ffi::c_int
                            && b_0 as ::core::ffi::c_int == (*stats).key_b as ::core::ffi::c_int
                        {
                            (*stats).alpha = 1 as ::core::ffi::c_uint;
                            (*stats).key = 0 as ::core::ffi::c_uint;
                            alpha_done = 1 as ::core::ffi::c_uint;
                        }
                        i = i.wrapping_add(1);
                    }
                }
            } else {
                let mut r_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut g_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut b_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut a_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut pr: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut pg: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut pb: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut pa: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                i = 0 as size_t;
                loop {
                    if !(i != numpixels) {
                        current_block = 7639320476250304355;
                        break;
                    }
                    getPixelColorRGBA8(
                        &raw mut r_1,
                        &raw mut g_1,
                        &raw mut b_1,
                        &raw mut a_1,
                        in_0,
                        i,
                        mode_in,
                    );
                    if !(i != 0 as size_t
                        && r_1 as ::core::ffi::c_int == pr as ::core::ffi::c_int
                        && g_1 as ::core::ffi::c_int == pg as ::core::ffi::c_int
                        && b_1 as ::core::ffi::c_int == pb as ::core::ffi::c_int
                        && a_1 as ::core::ffi::c_int == pa as ::core::ffi::c_int)
                    {
                        pr = r_1;
                        pg = g_1;
                        pb = b_1;
                        pa = a_1;
                        if bits_done == 0 && (*stats).bits < 8 as ::core::ffi::c_uint {
                            let mut bits: ::core::ffi::c_uint = getValueRequiredBits(r_1);
                            if bits > (*stats).bits {
                                (*stats).bits = bits;
                            }
                        }
                        bits_done =
                            ((*stats).bits >= bpp) as ::core::ffi::c_int as ::core::ffi::c_uint;
                        if colored_done == 0
                            && (r_1 as ::core::ffi::c_int != g_1 as ::core::ffi::c_int
                                || r_1 as ::core::ffi::c_int != b_1 as ::core::ffi::c_int)
                        {
                            (*stats).colored = 1 as ::core::ffi::c_uint;
                            colored_done = 1 as ::core::ffi::c_uint;
                            if (*stats).bits < 8 as ::core::ffi::c_uint {
                                (*stats).bits = 8 as ::core::ffi::c_uint;
                            }
                        }
                        if alpha_done == 0 {
                            let mut matchkey_0: ::core::ffi::c_uint = (r_1 as ::core::ffi::c_int
                                == (*stats).key_r as ::core::ffi::c_int
                                && g_1 as ::core::ffi::c_int
                                    == (*stats).key_g as ::core::ffi::c_int
                                && b_1 as ::core::ffi::c_int
                                    == (*stats).key_b as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint;
                            if a_1 as ::core::ffi::c_int != 255 as ::core::ffi::c_int
                                && (a_1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                    || (*stats).key != 0 && matchkey_0 == 0)
                            {
                                (*stats).alpha = 1 as ::core::ffi::c_uint;
                                (*stats).key = 0 as ::core::ffi::c_uint;
                                alpha_done = 1 as ::core::ffi::c_uint;
                                if (*stats).bits < 8 as ::core::ffi::c_uint {
                                    (*stats).bits = 8 as ::core::ffi::c_uint;
                                }
                            } else if a_1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                && (*stats).alpha == 0
                                && (*stats).key == 0
                            {
                                (*stats).key = 1 as ::core::ffi::c_uint;
                                (*stats).key_r = r_1 as ::core::ffi::c_ushort;
                                (*stats).key_g = g_1 as ::core::ffi::c_ushort;
                                (*stats).key_b = b_1 as ::core::ffi::c_ushort;
                            } else if a_1 as ::core::ffi::c_int == 255 as ::core::ffi::c_int
                                && (*stats).key != 0
                                && matchkey_0 != 0
                            {
                                (*stats).alpha = 1 as ::core::ffi::c_uint;
                                (*stats).key = 0 as ::core::ffi::c_uint;
                                alpha_done = 1 as ::core::ffi::c_uint;
                                if (*stats).bits < 8 as ::core::ffi::c_uint {
                                    (*stats).bits = 8 as ::core::ffi::c_uint;
                                }
                            }
                        }
                        if numcolors_done == 0 {
                            if color_tree_has(&raw mut tree, r_1, g_1, b_1, a_1) == 0 {
                                error = color_tree_add(
                                    &raw mut tree,
                                    r_1,
                                    g_1,
                                    b_1,
                                    a_1,
                                    (*stats).numcolors,
                                );
                                if error != 0 {
                                    current_block = 2950568455507082615;
                                    break;
                                }
                                if (*stats).numcolors < 256 as ::core::ffi::c_uint {
                                    let mut p: *mut ::core::ffi::c_uchar =
                                        &raw mut (*stats).palette as *mut ::core::ffi::c_uchar;
                                    let mut n: ::core::ffi::c_uint = (*stats).numcolors;
                                    *p.offset(
                                        n.wrapping_mul(4 as ::core::ffi::c_uint)
                                            .wrapping_add(0 as ::core::ffi::c_uint)
                                            as isize,
                                    ) = r_1;
                                    *p.offset(
                                        n.wrapping_mul(4 as ::core::ffi::c_uint)
                                            .wrapping_add(1 as ::core::ffi::c_uint)
                                            as isize,
                                    ) = g_1;
                                    *p.offset(
                                        n.wrapping_mul(4 as ::core::ffi::c_uint)
                                            .wrapping_add(2 as ::core::ffi::c_uint)
                                            as isize,
                                    ) = b_1;
                                    *p.offset(
                                        n.wrapping_mul(4 as ::core::ffi::c_uint)
                                            .wrapping_add(3 as ::core::ffi::c_uint)
                                            as isize,
                                    ) = a_1;
                                }
                                (*stats).numcolors = (*stats).numcolors.wrapping_add(1);
                                numcolors_done = ((*stats).numcolors >= maxnumcolors)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_uint;
                            }
                        }
                        if alpha_done != 0
                            && numcolors_done != 0
                            && colored_done != 0
                            && bits_done != 0
                        {
                            current_block = 7639320476250304355;
                            break;
                        }
                    }
                    i = i.wrapping_add(1);
                }
                match current_block {
                    2950568455507082615 => {}
                    _ => {
                        if (*stats).key != 0 && (*stats).alpha == 0 {
                            i = 0 as size_t;
                            while i != numpixels {
                                getPixelColorRGBA8(
                                    &raw mut r_1,
                                    &raw mut g_1,
                                    &raw mut b_1,
                                    &raw mut a_1,
                                    in_0,
                                    i,
                                    mode_in,
                                );
                                if a_1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                    && r_1 as ::core::ffi::c_int
                                        == (*stats).key_r as ::core::ffi::c_int
                                    && g_1 as ::core::ffi::c_int
                                        == (*stats).key_g as ::core::ffi::c_int
                                    && b_1 as ::core::ffi::c_int
                                        == (*stats).key_b as ::core::ffi::c_int
                                {
                                    (*stats).alpha = 1 as ::core::ffi::c_uint;
                                    (*stats).key = 0 as ::core::ffi::c_uint;
                                    alpha_done = 1 as ::core::ffi::c_uint;
                                    if (*stats).bits < 8 as ::core::ffi::c_uint {
                                        (*stats).bits = 8 as ::core::ffi::c_uint;
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                        }
                        (*stats).key_r = ((*stats).key_r as ::core::ffi::c_int
                            + (((*stats).key_r as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
                            as ::core::ffi::c_ushort;
                        (*stats).key_g = ((*stats).key_g as ::core::ffi::c_int
                            + (((*stats).key_g as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
                            as ::core::ffi::c_ushort;
                        (*stats).key_b = ((*stats).key_b as ::core::ffi::c_int
                            + (((*stats).key_b as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
                            as ::core::ffi::c_ushort;
                    }
                }
            }
        }
        _ => {}
    }
    color_tree_cleanup(&raw mut tree);
    return error;
}
unsafe extern "C" fn lodepng_color_stats_add(
    mut stats: *mut LodePNGColorStats,
    mut r: ::core::ffi::c_uint,
    mut g: ::core::ffi::c_uint,
    mut b: ::core::ffi::c_uint,
    mut a: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut image: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut mode: LodePNGColorMode = LodePNGColorMode {
        colortype: LCT_GREY,
        bitdepth: 0,
        palette: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        palettesize: 0,
        key_defined: 0,
        key_r: 0,
        key_g: 0,
        key_b: 0,
    };
    lodepng_color_mode_init(&raw mut mode);
    image[0 as ::core::ffi::c_int as usize] =
        (r >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    image[1 as ::core::ffi::c_int as usize] = r as ::core::ffi::c_uchar;
    image[2 as ::core::ffi::c_int as usize] =
        (g >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    image[3 as ::core::ffi::c_int as usize] = g as ::core::ffi::c_uchar;
    image[4 as ::core::ffi::c_int as usize] =
        (b >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    image[5 as ::core::ffi::c_int as usize] = b as ::core::ffi::c_uchar;
    image[6 as ::core::ffi::c_int as usize] =
        (a >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    image[7 as ::core::ffi::c_int as usize] = a as ::core::ffi::c_uchar;
    mode.bitdepth = 16 as ::core::ffi::c_uint;
    mode.colortype = LCT_RGBA;
    error = lodepng_compute_color_stats(
        stats,
        &raw mut image as *mut ::core::ffi::c_uchar,
        1 as ::core::ffi::c_uint,
        1 as ::core::ffi::c_uint,
        &raw mut mode,
    );
    lodepng_color_mode_cleanup(&raw mut mode);
    return error;
}
unsafe extern "C" fn auto_choose_color(
    mut mode_out: *mut LodePNGColorMode,
    mut mode_in: *const LodePNGColorMode,
    mut stats: *const LodePNGColorStats,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut palettebits: ::core::ffi::c_uint = 0;
    let mut i: size_t = 0;
    let mut n: size_t = 0;
    let mut numpixels: size_t = (*stats).numpixels;
    let mut palette_ok: ::core::ffi::c_uint = 0;
    let mut gray_ok: ::core::ffi::c_uint = 0;
    let mut alpha: ::core::ffi::c_uint = (*stats).alpha;
    let mut key: ::core::ffi::c_uint = (*stats).key;
    let mut bits: ::core::ffi::c_uint = (*stats).bits;
    (*mode_out).key_defined = 0 as ::core::ffi::c_uint;
    if key != 0 && numpixels <= 16 as size_t {
        alpha = 1 as ::core::ffi::c_uint;
        key = 0 as ::core::ffi::c_uint;
        if bits < 8 as ::core::ffi::c_uint {
            bits = 8 as ::core::ffi::c_uint;
        }
    }
    gray_ok = ((*stats).colored == 0) as ::core::ffi::c_int as ::core::ffi::c_uint;
    if (*stats).allow_greyscale == 0 {
        gray_ok = 0 as ::core::ffi::c_uint;
    }
    if gray_ok == 0 && bits < 8 as ::core::ffi::c_uint {
        bits = 8 as ::core::ffi::c_uint;
    }
    n = (*stats).numcolors as size_t;
    palettebits = (if n <= 2 as size_t {
        1 as ::core::ffi::c_int
    } else if n <= 4 as size_t {
        2 as ::core::ffi::c_int
    } else if n <= 16 as size_t {
        4 as ::core::ffi::c_int
    } else {
        8 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    palette_ok = (n <= 256 as size_t && bits <= 8 as ::core::ffi::c_uint && n != 0 as size_t)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
    if numpixels < n.wrapping_mul(2 as size_t) {
        palette_ok = 0 as ::core::ffi::c_uint;
    }
    if gray_ok != 0 && alpha == 0 && bits <= palettebits {
        palette_ok = 0 as ::core::ffi::c_uint;
    }
    if (*stats).allow_palette == 0 {
        palette_ok = 0 as ::core::ffi::c_uint;
    }
    if palette_ok != 0 {
        let mut p: *const ::core::ffi::c_uchar =
            &raw const (*stats).palette as *const ::core::ffi::c_uchar;
        lodepng_palette_clear(mode_out);
        i = 0 as size_t;
        while i != (*stats).numcolors as size_t {
            error = lodepng_palette_add(
                mode_out,
                *p.offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize),
                *p.offset(i.wrapping_mul(4 as size_t).wrapping_add(1 as size_t) as isize),
                *p.offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize),
                *p.offset(i.wrapping_mul(4 as size_t).wrapping_add(3 as size_t) as isize),
            );
            if error != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        (*mode_out).colortype = LCT_PALETTE;
        (*mode_out).bitdepth = palettebits;
        if (*mode_in).colortype as ::core::ffi::c_uint
            == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*mode_in).palettesize >= (*mode_out).palettesize
            && (*mode_in).bitdepth == (*mode_out).bitdepth
        {
            lodepng_color_mode_cleanup(mode_out);
            lodepng_color_mode_copy(mode_out, mode_in);
        }
    } else {
        (*mode_out).bitdepth = bits;
        (*mode_out).colortype = (if alpha != 0 {
            if gray_ok != 0 {
                LCT_GREY_ALPHA as ::core::ffi::c_int
            } else {
                LCT_RGBA as ::core::ffi::c_int
            }
        } else if gray_ok != 0 {
            LCT_GREY as ::core::ffi::c_int
        } else {
            LCT_RGB as ::core::ffi::c_int
        }) as LodePNGColorType;
        if key != 0 {
            let mut mask_0: ::core::ffi::c_uint = ((1 as ::core::ffi::c_uint)
                << (*mode_out).bitdepth)
                .wrapping_sub(1 as ::core::ffi::c_uint);
            (*mode_out).key_r = (*stats).key_r as ::core::ffi::c_uint & mask_0;
            (*mode_out).key_g = (*stats).key_g as ::core::ffi::c_uint & mask_0;
            (*mode_out).key_b = (*stats).key_b as ::core::ffi::c_uint & mask_0;
            (*mode_out).key_defined = 1 as ::core::ffi::c_uint;
        }
    }
    return error;
}
unsafe extern "C" fn paethPredictor(
    mut a: ::core::ffi::c_uchar,
    mut b: ::core::ffi::c_uchar,
    mut c: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut pa: ::core::ffi::c_short =
        (if (b as ::core::ffi::c_int - c as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            -(b as ::core::ffi::c_int - c as ::core::ffi::c_int)
        } else {
            b as ::core::ffi::c_int - c as ::core::ffi::c_int
        }) as ::core::ffi::c_short;
    let mut pb: ::core::ffi::c_short =
        (if (a as ::core::ffi::c_int - c as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            -(a as ::core::ffi::c_int - c as ::core::ffi::c_int)
        } else {
            a as ::core::ffi::c_int - c as ::core::ffi::c_int
        }) as ::core::ffi::c_short;
    let mut pc: ::core::ffi::c_short = (if (a as ::core::ffi::c_int + b as ::core::ffi::c_int
        - c as ::core::ffi::c_int
        - c as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        -(a as ::core::ffi::c_int + b as ::core::ffi::c_int
            - c as ::core::ffi::c_int
            - c as ::core::ffi::c_int)
    } else {
        a as ::core::ffi::c_int + b as ::core::ffi::c_int
            - c as ::core::ffi::c_int
            - c as ::core::ffi::c_int
    }) as ::core::ffi::c_short;
    if (pb as ::core::ffi::c_int) < pa as ::core::ffi::c_int {
        a = b;
        pa = pb;
    }
    return (if (pc as ::core::ffi::c_int) < pa as ::core::ffi::c_int {
        c as ::core::ffi::c_int
    } else {
        a as ::core::ffi::c_int
    }) as ::core::ffi::c_uchar;
}
static mut ADAM7_IX: [::core::ffi::c_uint; 7] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
static mut ADAM7_IY: [::core::ffi::c_uint; 7] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
static mut ADAM7_DX: [::core::ffi::c_uint; 7] = [
    8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    1 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
static mut ADAM7_DY: [::core::ffi::c_uint; 7] = [
    8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
unsafe extern "C" fn Adam7_getpassvalues(
    mut passw: *mut ::core::ffi::c_uint,
    mut passh: *mut ::core::ffi::c_uint,
    mut filter_passstart: *mut size_t,
    mut padded_passstart: *mut size_t,
    mut passstart: *mut size_t,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut bpp: ::core::ffi::c_uint,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i != 7 as ::core::ffi::c_uint {
        *passw.offset(i as isize) = w
            .wrapping_add(ADAM7_DX[i as usize])
            .wrapping_sub(ADAM7_IX[i as usize])
            .wrapping_sub(1 as ::core::ffi::c_uint)
            .wrapping_div(ADAM7_DX[i as usize]);
        *passh.offset(i as isize) = h
            .wrapping_add(ADAM7_DY[i as usize])
            .wrapping_sub(ADAM7_IY[i as usize])
            .wrapping_sub(1 as ::core::ffi::c_uint)
            .wrapping_div(ADAM7_DY[i as usize]);
        if *passw.offset(i as isize) == 0 as ::core::ffi::c_uint {
            *passh.offset(i as isize) = 0 as ::core::ffi::c_uint;
        }
        if *passh.offset(i as isize) == 0 as ::core::ffi::c_uint {
            *passw.offset(i as isize) = 0 as ::core::ffi::c_uint;
        }
        i = i.wrapping_add(1);
    }
    let ref mut fresh33 = *passstart.offset(0 as ::core::ffi::c_int as isize);
    *fresh33 = 0 as size_t;
    let ref mut fresh34 = *padded_passstart.offset(0 as ::core::ffi::c_int as isize);
    *fresh34 = *fresh33;
    *filter_passstart.offset(0 as ::core::ffi::c_int as isize) = *fresh34;
    i = 0 as ::core::ffi::c_uint;
    while i != 7 as ::core::ffi::c_uint {
        *filter_passstart.offset(i.wrapping_add(1 as ::core::ffi::c_uint) as isize) =
            (*filter_passstart.offset(i as isize)).wrapping_add(
                (if *passw.offset(i as isize) != 0 && *passh.offset(i as isize) != 0 {
                    (*passh.offset(i as isize)).wrapping_mul(
                        (1 as ::core::ffi::c_uint).wrapping_add(
                            (*passw.offset(i as isize))
                                .wrapping_mul(bpp)
                                .wrapping_add(7 as ::core::ffi::c_uint)
                                .wrapping_div(8 as ::core::ffi::c_uint),
                        ),
                    )
                } else {
                    0 as ::core::ffi::c_uint
                }) as size_t,
            );
        *padded_passstart.offset(i.wrapping_add(1 as ::core::ffi::c_uint) as isize) =
            (*padded_passstart.offset(i as isize)).wrapping_add(
                (*passh.offset(i as isize)).wrapping_mul(
                    (*passw.offset(i as isize))
                        .wrapping_mul(bpp)
                        .wrapping_add(7 as ::core::ffi::c_uint)
                        .wrapping_div(8 as ::core::ffi::c_uint),
                ) as size_t,
            );
        *passstart.offset(i.wrapping_add(1 as ::core::ffi::c_uint) as isize) =
            (*passstart.offset(i as isize)).wrapping_add(
                (*passh.offset(i as isize))
                    .wrapping_mul(*passw.offset(i as isize))
                    .wrapping_mul(bpp)
                    .wrapping_add(7 as ::core::ffi::c_uint)
                    .wrapping_div(8 as ::core::ffi::c_uint) as size_t,
            );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_inspect(
    mut w: *mut ::core::ffi::c_uint,
    mut h: *mut ::core::ffi::c_uint,
    mut state: *mut LodePNGState,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
) -> ::core::ffi::c_uint {
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut info: *mut LodePNGInfo = &raw mut (*state).info_png;
    if insize == 0 as size_t || in_0.is_null() {
        (*state).error = 48 as ::core::ffi::c_uint;
        return 48 as ::core::ffi::c_uint;
    }
    if insize < 33 as size_t {
        (*state).error = 27 as ::core::ffi::c_uint;
        return 27 as ::core::ffi::c_uint;
    }
    lodepng_info_cleanup(info);
    lodepng_info_init(info);
    if *in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != 137 as ::core::ffi::c_int
        || *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 80 as ::core::ffi::c_int
        || *in_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 78 as ::core::ffi::c_int
        || *in_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 71 as ::core::ffi::c_int
        || *in_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 13 as ::core::ffi::c_int
        || *in_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 10 as ::core::ffi::c_int
        || *in_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 26 as ::core::ffi::c_int
        || *in_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 10 as ::core::ffi::c_int
    {
        (*state).error = 28 as ::core::ffi::c_uint;
        return 28 as ::core::ffi::c_uint;
    }
    if lodepng_chunk_length(in_0.offset(8 as ::core::ffi::c_int as isize))
        != 13 as ::core::ffi::c_uint
    {
        (*state).error = 94 as ::core::ffi::c_uint;
        return 94 as ::core::ffi::c_uint;
    }
    if lodepng_chunk_type_equals(
        in_0.offset(8 as ::core::ffi::c_int as isize),
        b"IHDR\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*state).error = 29 as ::core::ffi::c_uint;
        return 29 as ::core::ffi::c_uint;
    }
    width = lodepng_read32bitInt(
        in_0.offset(16 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_uchar
    );
    height = lodepng_read32bitInt(
        in_0.offset(20 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_uchar
    );
    if !w.is_null() {
        *w = width;
    }
    if !h.is_null() {
        *h = height;
    }
    (*info).color.bitdepth = *in_0.offset(24 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).color.colortype = *in_0.offset(25 as ::core::ffi::c_int as isize) as LodePNGColorType;
    (*info).compression_method =
        *in_0.offset(26 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).filter_method = *in_0.offset(27 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).interlace_method =
        *in_0.offset(28 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    if width == 0 as ::core::ffi::c_uint || height == 0 as ::core::ffi::c_uint {
        (*state).error = 93 as ::core::ffi::c_uint;
        return 93 as ::core::ffi::c_uint;
    }
    (*state).error = checkColorValidity((*info).color.colortype, (*info).color.bitdepth);
    if (*state).error != 0 {
        return (*state).error;
    }
    if (*info).compression_method != 0 as ::core::ffi::c_uint {
        (*state).error = 32 as ::core::ffi::c_uint;
        return 32 as ::core::ffi::c_uint;
    }
    if (*info).filter_method != 0 as ::core::ffi::c_uint {
        (*state).error = 33 as ::core::ffi::c_uint;
        return 33 as ::core::ffi::c_uint;
    }
    if (*info).interlace_method > 1 as ::core::ffi::c_uint {
        (*state).error = 34 as ::core::ffi::c_uint;
        return 34 as ::core::ffi::c_uint;
    }
    if (*state).decoder.ignore_crc == 0 {
        let mut crc: ::core::ffi::c_uint = lodepng_read32bitInt(
            in_0.offset(29 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_uchar,
        );
        let mut checksum: ::core::ffi::c_uint = lodepng_crc32(
            in_0.offset(12 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_uchar,
            17 as size_t,
        );
        if crc != checksum {
            (*state).error = 57 as ::core::ffi::c_uint;
            return 57 as ::core::ffi::c_uint;
        }
    }
    return (*state).error;
}
unsafe extern "C" fn unfilterScanline(
    mut recon: *mut ::core::ffi::c_uchar,
    mut scanline: *const ::core::ffi::c_uchar,
    mut precon: *const ::core::ffi::c_uchar,
    mut bytewidth: size_t,
    mut filterType: ::core::ffi::c_uchar,
    mut length: size_t,
) -> ::core::ffi::c_uint {
    let mut i: size_t = 0;
    match filterType as ::core::ffi::c_int {
        0 => {
            lodepng_memcpy(
                recon as *mut ::core::ffi::c_void,
                scanline as *const ::core::ffi::c_void,
                length,
            );
        }
        1 => {
            let mut j: size_t = 0 as size_t;
            i = 0 as size_t;
            while i != bytewidth {
                *recon.offset(i as isize) = *scanline.offset(i as isize);
                i = i.wrapping_add(1);
            }
            i = bytewidth;
            while i != length {
                *recon.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                    + *recon.offset(j as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
                i = i.wrapping_add(1);
                j = j.wrapping_add(1);
            }
        }
        2 => {
            if !precon.is_null() {
                i = 0 as size_t;
                while i != length {
                    *recon.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        + *precon.offset(i as isize) as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
            } else {
                lodepng_memcpy(
                    recon as *mut ::core::ffi::c_void,
                    scanline as *const ::core::ffi::c_void,
                    length,
                );
            }
        }
        3 => {
            if !precon.is_null() {
                let mut j_0: size_t = 0 as size_t;
                i = 0 as size_t;
                while i != bytewidth {
                    *recon.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        + (*precon.offset(i as isize) as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_uint))
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
                if bytewidth >= 4 as size_t {
                    while i.wrapping_add(3 as size_t) < length {
                        let mut s0: ::core::ffi::c_uchar =
                            *scanline.offset(i.wrapping_add(0 as size_t) as isize);
                        let mut s1: ::core::ffi::c_uchar =
                            *scanline.offset(i.wrapping_add(1 as size_t) as isize);
                        let mut s2: ::core::ffi::c_uchar =
                            *scanline.offset(i.wrapping_add(2 as size_t) as isize);
                        let mut s3: ::core::ffi::c_uchar =
                            *scanline.offset(i.wrapping_add(3 as size_t) as isize);
                        let mut r0: ::core::ffi::c_uchar =
                            *recon.offset(j_0.wrapping_add(0 as size_t) as isize);
                        let mut r1: ::core::ffi::c_uchar =
                            *recon.offset(j_0.wrapping_add(1 as size_t) as isize);
                        let mut r2: ::core::ffi::c_uchar =
                            *recon.offset(j_0.wrapping_add(2 as size_t) as isize);
                        let mut r3: ::core::ffi::c_uchar =
                            *recon.offset(j_0.wrapping_add(3 as size_t) as isize);
                        let mut p0: ::core::ffi::c_uchar =
                            *precon.offset(i.wrapping_add(0 as size_t) as isize);
                        let mut p1: ::core::ffi::c_uchar =
                            *precon.offset(i.wrapping_add(1 as size_t) as isize);
                        let mut p2: ::core::ffi::c_uchar =
                            *precon.offset(i.wrapping_add(2 as size_t) as isize);
                        let mut p3: ::core::ffi::c_uchar =
                            *precon.offset(i.wrapping_add(3 as size_t) as isize);
                        *recon.offset(i.wrapping_add(0 as size_t) as isize) = (s0
                            as ::core::ffi::c_int
                            + (r0 as ::core::ffi::c_int + p0 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(1 as size_t) as isize) = (s1
                            as ::core::ffi::c_int
                            + (r1 as ::core::ffi::c_int + p1 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(2 as size_t) as isize) = (s2
                            as ::core::ffi::c_int
                            + (r2 as ::core::ffi::c_int + p2 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(3 as size_t) as isize) = (s3
                            as ::core::ffi::c_int
                            + (r3 as ::core::ffi::c_int + p3 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_uchar;
                        i = (i as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        j_0 = (j_0 as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else if bytewidth >= 3 as size_t {
                    while i.wrapping_add(2 as size_t) < length {
                        let mut s0_0: ::core::ffi::c_uchar =
                            *scanline.offset(i.wrapping_add(0 as size_t) as isize);
                        let mut s1_0: ::core::ffi::c_uchar =
                            *scanline.offset(i.wrapping_add(1 as size_t) as isize);
                        let mut s2_0: ::core::ffi::c_uchar =
                            *scanline.offset(i.wrapping_add(2 as size_t) as isize);
                        let mut r0_0: ::core::ffi::c_uchar =
                            *recon.offset(j_0.wrapping_add(0 as size_t) as isize);
                        let mut r1_0: ::core::ffi::c_uchar =
                            *recon.offset(j_0.wrapping_add(1 as size_t) as isize);
                        let mut r2_0: ::core::ffi::c_uchar =
                            *recon.offset(j_0.wrapping_add(2 as size_t) as isize);
                        let mut p0_0: ::core::ffi::c_uchar =
                            *precon.offset(i.wrapping_add(0 as size_t) as isize);
                        let mut p1_0: ::core::ffi::c_uchar =
                            *precon.offset(i.wrapping_add(1 as size_t) as isize);
                        let mut p2_0: ::core::ffi::c_uchar =
                            *precon.offset(i.wrapping_add(2 as size_t) as isize);
                        *recon.offset(i.wrapping_add(0 as size_t) as isize) = (s0_0
                            as ::core::ffi::c_int
                            + (r0_0 as ::core::ffi::c_int + p0_0 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(1 as size_t) as isize) = (s1_0
                            as ::core::ffi::c_int
                            + (r1_0 as ::core::ffi::c_int + p1_0 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(2 as size_t) as isize) = (s2_0
                            as ::core::ffi::c_int
                            + (r2_0 as ::core::ffi::c_int + p2_0 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_uchar;
                        i = (i as ::core::ffi::c_ulong).wrapping_add(3 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        j_0 = (j_0 as ::core::ffi::c_ulong).wrapping_add(3 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else if bytewidth >= 2 as size_t {
                    while i.wrapping_add(1 as size_t) < length {
                        let mut s0_1: ::core::ffi::c_uchar =
                            *scanline.offset(i.wrapping_add(0 as size_t) as isize);
                        let mut s1_1: ::core::ffi::c_uchar =
                            *scanline.offset(i.wrapping_add(1 as size_t) as isize);
                        let mut r0_1: ::core::ffi::c_uchar =
                            *recon.offset(j_0.wrapping_add(0 as size_t) as isize);
                        let mut r1_1: ::core::ffi::c_uchar =
                            *recon.offset(j_0.wrapping_add(1 as size_t) as isize);
                        let mut p0_1: ::core::ffi::c_uchar =
                            *precon.offset(i.wrapping_add(0 as size_t) as isize);
                        let mut p1_1: ::core::ffi::c_uchar =
                            *precon.offset(i.wrapping_add(1 as size_t) as isize);
                        *recon.offset(i.wrapping_add(0 as size_t) as isize) = (s0_1
                            as ::core::ffi::c_int
                            + (r0_1 as ::core::ffi::c_int + p0_1 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(1 as size_t) as isize) = (s1_1
                            as ::core::ffi::c_int
                            + (r1_1 as ::core::ffi::c_int + p1_1 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_uchar;
                        i = (i as ::core::ffi::c_ulong).wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        j_0 = (j_0 as ::core::ffi::c_ulong).wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                }
                while i != length {
                    *recon.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        + (*recon.offset(j_0 as isize) as ::core::ffi::c_int
                            + *precon.offset(i as isize) as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_uint))
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                    j_0 = j_0.wrapping_add(1);
                }
            } else {
                let mut j_1: size_t = 0 as size_t;
                i = 0 as size_t;
                while i != bytewidth {
                    *recon.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i != length {
                    *recon.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        + (*recon.offset(j_1 as isize) as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_uint))
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                    j_1 = j_1.wrapping_add(1);
                }
            }
        }
        4 => {
            if !precon.is_null() {
                if bytewidth == 8 as size_t {
                    let mut a0: ::core::ffi::c_uchar = 0;
                    let mut b0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c0: ::core::ffi::c_uchar = 0;
                    let mut d0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a1: ::core::ffi::c_uchar = 0;
                    let mut b1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c1: ::core::ffi::c_uchar = 0;
                    let mut d1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a2: ::core::ffi::c_uchar = 0;
                    let mut b2: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c2: ::core::ffi::c_uchar = 0;
                    let mut d2: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a3: ::core::ffi::c_uchar = 0;
                    let mut b3: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c3: ::core::ffi::c_uchar = 0;
                    let mut d3: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a4: ::core::ffi::c_uchar = 0;
                    let mut b4: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c4: ::core::ffi::c_uchar = 0;
                    let mut d4: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a5: ::core::ffi::c_uchar = 0;
                    let mut b5: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c5: ::core::ffi::c_uchar = 0;
                    let mut d5: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a6: ::core::ffi::c_uchar = 0;
                    let mut b6: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c6: ::core::ffi::c_uchar = 0;
                    let mut d6: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a7: ::core::ffi::c_uchar = 0;
                    let mut b7: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c7: ::core::ffi::c_uchar = 0;
                    let mut d7: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    i = 0 as size_t;
                    while i.wrapping_add(7 as size_t) < length {
                        c0 = b0;
                        c1 = b1;
                        c2 = b2;
                        c3 = b3;
                        c4 = b4;
                        c5 = b5;
                        c6 = b6;
                        c7 = b7;
                        b0 = *precon.offset(i.wrapping_add(0 as size_t) as isize);
                        b1 = *precon.offset(i.wrapping_add(1 as size_t) as isize);
                        b2 = *precon.offset(i.wrapping_add(2 as size_t) as isize);
                        b3 = *precon.offset(i.wrapping_add(3 as size_t) as isize);
                        b4 = *precon.offset(i.wrapping_add(4 as size_t) as isize);
                        b5 = *precon.offset(i.wrapping_add(5 as size_t) as isize);
                        b6 = *precon.offset(i.wrapping_add(6 as size_t) as isize);
                        b7 = *precon.offset(i.wrapping_add(7 as size_t) as isize);
                        a0 = d0;
                        a1 = d1;
                        a2 = d2;
                        a3 = d3;
                        a4 = d4;
                        a5 = d5;
                        a6 = d6;
                        a7 = d7;
                        d0 = (*scanline.offset(i.wrapping_add(0 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a0, b0, c0) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d1 = (*scanline.offset(i.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a1, b1, c1) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d2 = (*scanline.offset(i.wrapping_add(2 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a2, b2, c2) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d3 = (*scanline.offset(i.wrapping_add(3 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a3, b3, c3) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d4 = (*scanline.offset(i.wrapping_add(4 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a4, b4, c4) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d5 = (*scanline.offset(i.wrapping_add(5 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a5, b5, c5) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d6 = (*scanline.offset(i.wrapping_add(6 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a6, b6, c6) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d7 = (*scanline.offset(i.wrapping_add(7 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a7, b7, c7) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(0 as size_t) as isize) = d0;
                        *recon.offset(i.wrapping_add(1 as size_t) as isize) = d1;
                        *recon.offset(i.wrapping_add(2 as size_t) as isize) = d2;
                        *recon.offset(i.wrapping_add(3 as size_t) as isize) = d3;
                        *recon.offset(i.wrapping_add(4 as size_t) as isize) = d4;
                        *recon.offset(i.wrapping_add(5 as size_t) as isize) = d5;
                        *recon.offset(i.wrapping_add(6 as size_t) as isize) = d6;
                        *recon.offset(i.wrapping_add(7 as size_t) as isize) = d7;
                        i = (i as ::core::ffi::c_ulong).wrapping_add(8 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else if bytewidth == 6 as size_t {
                    let mut a0_0: ::core::ffi::c_uchar = 0;
                    let mut b0_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c0_0: ::core::ffi::c_uchar = 0;
                    let mut d0_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a1_0: ::core::ffi::c_uchar = 0;
                    let mut b1_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c1_0: ::core::ffi::c_uchar = 0;
                    let mut d1_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a2_0: ::core::ffi::c_uchar = 0;
                    let mut b2_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c2_0: ::core::ffi::c_uchar = 0;
                    let mut d2_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a3_0: ::core::ffi::c_uchar = 0;
                    let mut b3_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c3_0: ::core::ffi::c_uchar = 0;
                    let mut d3_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a4_0: ::core::ffi::c_uchar = 0;
                    let mut b4_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c4_0: ::core::ffi::c_uchar = 0;
                    let mut d4_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a5_0: ::core::ffi::c_uchar = 0;
                    let mut b5_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c5_0: ::core::ffi::c_uchar = 0;
                    let mut d5_0: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    i = 0 as size_t;
                    while i.wrapping_add(5 as size_t) < length {
                        c0_0 = b0_0;
                        c1_0 = b1_0;
                        c2_0 = b2_0;
                        c3_0 = b3_0;
                        c4_0 = b4_0;
                        c5_0 = b5_0;
                        b0_0 = *precon.offset(i.wrapping_add(0 as size_t) as isize);
                        b1_0 = *precon.offset(i.wrapping_add(1 as size_t) as isize);
                        b2_0 = *precon.offset(i.wrapping_add(2 as size_t) as isize);
                        b3_0 = *precon.offset(i.wrapping_add(3 as size_t) as isize);
                        b4_0 = *precon.offset(i.wrapping_add(4 as size_t) as isize);
                        b5_0 = *precon.offset(i.wrapping_add(5 as size_t) as isize);
                        a0_0 = d0_0;
                        a1_0 = d1_0;
                        a2_0 = d2_0;
                        a3_0 = d3_0;
                        a4_0 = d4_0;
                        a5_0 = d5_0;
                        d0_0 = (*scanline.offset(i.wrapping_add(0 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a0_0, b0_0, c0_0) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d1_0 = (*scanline.offset(i.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a1_0, b1_0, c1_0) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d2_0 = (*scanline.offset(i.wrapping_add(2 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a2_0, b2_0, c2_0) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d3_0 = (*scanline.offset(i.wrapping_add(3 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a3_0, b3_0, c3_0) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d4_0 = (*scanline.offset(i.wrapping_add(4 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a4_0, b4_0, c4_0) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d5_0 = (*scanline.offset(i.wrapping_add(5 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a5_0, b5_0, c5_0) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(0 as size_t) as isize) = d0_0;
                        *recon.offset(i.wrapping_add(1 as size_t) as isize) = d1_0;
                        *recon.offset(i.wrapping_add(2 as size_t) as isize) = d2_0;
                        *recon.offset(i.wrapping_add(3 as size_t) as isize) = d3_0;
                        *recon.offset(i.wrapping_add(4 as size_t) as isize) = d4_0;
                        *recon.offset(i.wrapping_add(5 as size_t) as isize) = d5_0;
                        i = (i as ::core::ffi::c_ulong).wrapping_add(6 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else if bytewidth == 4 as size_t {
                    let mut a0_1: ::core::ffi::c_uchar = 0;
                    let mut b0_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c0_1: ::core::ffi::c_uchar = 0;
                    let mut d0_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a1_1: ::core::ffi::c_uchar = 0;
                    let mut b1_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c1_1: ::core::ffi::c_uchar = 0;
                    let mut d1_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a2_1: ::core::ffi::c_uchar = 0;
                    let mut b2_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c2_1: ::core::ffi::c_uchar = 0;
                    let mut d2_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a3_1: ::core::ffi::c_uchar = 0;
                    let mut b3_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c3_1: ::core::ffi::c_uchar = 0;
                    let mut d3_1: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    i = 0 as size_t;
                    while i.wrapping_add(3 as size_t) < length {
                        c0_1 = b0_1;
                        c1_1 = b1_1;
                        c2_1 = b2_1;
                        c3_1 = b3_1;
                        b0_1 = *precon.offset(i.wrapping_add(0 as size_t) as isize);
                        b1_1 = *precon.offset(i.wrapping_add(1 as size_t) as isize);
                        b2_1 = *precon.offset(i.wrapping_add(2 as size_t) as isize);
                        b3_1 = *precon.offset(i.wrapping_add(3 as size_t) as isize);
                        a0_1 = d0_1;
                        a1_1 = d1_1;
                        a2_1 = d2_1;
                        a3_1 = d3_1;
                        d0_1 = (*scanline.offset(i.wrapping_add(0 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a0_1, b0_1, c0_1) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d1_1 = (*scanline.offset(i.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a1_1, b1_1, c1_1) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d2_1 = (*scanline.offset(i.wrapping_add(2 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a2_1, b2_1, c2_1) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d3_1 = (*scanline.offset(i.wrapping_add(3 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a3_1, b3_1, c3_1) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(0 as size_t) as isize) = d0_1;
                        *recon.offset(i.wrapping_add(1 as size_t) as isize) = d1_1;
                        *recon.offset(i.wrapping_add(2 as size_t) as isize) = d2_1;
                        *recon.offset(i.wrapping_add(3 as size_t) as isize) = d3_1;
                        i = (i as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else if bytewidth == 3 as size_t {
                    let mut a0_2: ::core::ffi::c_uchar = 0;
                    let mut b0_2: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c0_2: ::core::ffi::c_uchar = 0;
                    let mut d0_2: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a1_2: ::core::ffi::c_uchar = 0;
                    let mut b1_2: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c1_2: ::core::ffi::c_uchar = 0;
                    let mut d1_2: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a2_2: ::core::ffi::c_uchar = 0;
                    let mut b2_2: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c2_2: ::core::ffi::c_uchar = 0;
                    let mut d2_2: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    i = 0 as size_t;
                    while i.wrapping_add(2 as size_t) < length {
                        c0_2 = b0_2;
                        c1_2 = b1_2;
                        c2_2 = b2_2;
                        b0_2 = *precon.offset(i.wrapping_add(0 as size_t) as isize);
                        b1_2 = *precon.offset(i.wrapping_add(1 as size_t) as isize);
                        b2_2 = *precon.offset(i.wrapping_add(2 as size_t) as isize);
                        a0_2 = d0_2;
                        a1_2 = d1_2;
                        a2_2 = d2_2;
                        d0_2 = (*scanline.offset(i.wrapping_add(0 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a0_2, b0_2, c0_2) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d1_2 = (*scanline.offset(i.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a1_2, b1_2, c1_2) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d2_2 = (*scanline.offset(i.wrapping_add(2 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a2_2, b2_2, c2_2) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(0 as size_t) as isize) = d0_2;
                        *recon.offset(i.wrapping_add(1 as size_t) as isize) = d1_2;
                        *recon.offset(i.wrapping_add(2 as size_t) as isize) = d2_2;
                        i = (i as ::core::ffi::c_ulong).wrapping_add(3 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else if bytewidth == 2 as size_t {
                    let mut a0_3: ::core::ffi::c_uchar = 0;
                    let mut b0_3: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c0_3: ::core::ffi::c_uchar = 0;
                    let mut d0_3: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut a1_3: ::core::ffi::c_uchar = 0;
                    let mut b1_3: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c1_3: ::core::ffi::c_uchar = 0;
                    let mut d1_3: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    i = 0 as size_t;
                    while i.wrapping_add(1 as size_t) < length {
                        c0_3 = b0_3;
                        c1_3 = b1_3;
                        b0_3 = *precon.offset(i.wrapping_add(0 as size_t) as isize);
                        b1_3 = *precon.offset(i.wrapping_add(1 as size_t) as isize);
                        a0_3 = d0_3;
                        a1_3 = d1_3;
                        d0_3 = (*scanline.offset(i.wrapping_add(0 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a0_3, b0_3, c0_3) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        d1_3 = (*scanline.offset(i.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int
                            + paethPredictor(a1_3, b1_3, c1_3) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        *recon.offset(i.wrapping_add(0 as size_t) as isize) = d0_3;
                        *recon.offset(i.wrapping_add(1 as size_t) as isize) = d1_3;
                        i = (i as ::core::ffi::c_ulong).wrapping_add(2 as ::core::ffi::c_ulong)
                            as size_t as size_t;
                    }
                } else if bytewidth == 1 as size_t {
                    let mut a: ::core::ffi::c_uchar = 0;
                    let mut b: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    let mut c: ::core::ffi::c_uchar = 0;
                    let mut d: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    i = 0 as size_t;
                    while i != length {
                        c = b;
                        b = *precon.offset(i as isize);
                        a = d;
                        d = (*scanline.offset(i as isize) as ::core::ffi::c_int
                            + paethPredictor(a, b, c) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        *recon.offset(i as isize) = d;
                        i = i.wrapping_add(1);
                    }
                } else {
                    i = 0 as size_t;
                    while i != bytewidth {
                        *recon.offset(i as isize) = (*scanline.offset(i as isize)
                            as ::core::ffi::c_int
                            + *precon.offset(i as isize) as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        i = i.wrapping_add(1);
                    }
                }
                while i != length {
                    *recon.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        + paethPredictor(
                            *recon.offset(i.wrapping_sub(bytewidth) as isize),
                            *precon.offset(i as isize),
                            *precon.offset(i.wrapping_sub(bytewidth) as isize),
                        ) as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
            } else {
                let mut j_2: size_t = 0 as size_t;
                i = 0 as size_t;
                while i != bytewidth {
                    *recon.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i != length {
                    *recon.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        + *recon.offset(j_2 as isize) as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                    j_2 = j_2.wrapping_add(1);
                }
            }
        }
        _ => return 36 as ::core::ffi::c_uint,
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn unfilter(
    mut out: *mut ::core::ffi::c_uchar,
    mut in_0: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut bpp: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut y: ::core::ffi::c_uint = 0;
    let mut prevline: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bytewidth: size_t = bpp
        .wrapping_add(7 as ::core::ffi::c_uint)
        .wrapping_div(8 as ::core::ffi::c_uint) as size_t;
    let mut linebytes: size_t =
        lodepng_get_raw_size_idat(w, 1 as ::core::ffi::c_uint, bpp).wrapping_sub(1 as size_t);
    y = 0 as ::core::ffi::c_uint;
    while y < h {
        let mut outindex: size_t = linebytes.wrapping_mul(y as size_t);
        let mut inindex: size_t = (1 as size_t)
            .wrapping_add(linebytes)
            .wrapping_mul(y as size_t);
        let mut filterType: ::core::ffi::c_uchar = *in_0.offset(inindex as isize);
        let mut error_: ::core::ffi::c_uint = unfilterScanline(
            out.offset(outindex as isize) as *mut ::core::ffi::c_uchar,
            in_0.offset(inindex.wrapping_add(1 as size_t) as isize) as *const ::core::ffi::c_uchar,
            prevline,
            bytewidth,
            filterType,
            linebytes,
        );
        if error_ != 0 {
            return error_;
        }
        prevline = out.offset(outindex as isize) as *mut ::core::ffi::c_uchar;
        y = y.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn Adam7_deinterlace(
    mut out: *mut ::core::ffi::c_uchar,
    mut in_0: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut bpp: ::core::ffi::c_uint,
) {
    let mut passw: [::core::ffi::c_uint; 7] = [0; 7];
    let mut passh: [::core::ffi::c_uint; 7] = [0; 7];
    let mut filter_passstart: [size_t; 8] = [0; 8];
    let mut padded_passstart: [size_t; 8] = [0; 8];
    let mut passstart: [size_t; 8] = [0; 8];
    let mut i: ::core::ffi::c_uint = 0;
    Adam7_getpassvalues(
        &raw mut passw as *mut ::core::ffi::c_uint,
        &raw mut passh as *mut ::core::ffi::c_uint,
        &raw mut filter_passstart as *mut size_t,
        &raw mut padded_passstart as *mut size_t,
        &raw mut passstart as *mut size_t,
        w,
        h,
        bpp,
    );
    if bpp >= 8 as ::core::ffi::c_uint {
        i = 0 as ::core::ffi::c_uint;
        while i != 7 as ::core::ffi::c_uint {
            let mut x: ::core::ffi::c_uint = 0;
            let mut y: ::core::ffi::c_uint = 0;
            let mut b: ::core::ffi::c_uint = 0;
            let mut bytewidth: size_t = bpp.wrapping_div(8 as ::core::ffi::c_uint) as size_t;
            y = 0 as ::core::ffi::c_uint;
            while y < passh[i as usize] {
                x = 0 as ::core::ffi::c_uint;
                while x < passw[i as usize] {
                    let mut pixelinstart: size_t = passstart[i as usize].wrapping_add(
                        (y.wrapping_mul(passw[i as usize]).wrapping_add(x) as size_t)
                            .wrapping_mul(bytewidth),
                    );
                    let mut pixeloutstart: size_t = (ADAM7_IY[i as usize] as size_t)
                        .wrapping_add((y as size_t).wrapping_mul(ADAM7_DY[i as usize] as size_t))
                        .wrapping_mul(w as size_t)
                        .wrapping_add(ADAM7_IX[i as usize] as size_t)
                        .wrapping_add((x as size_t).wrapping_mul(ADAM7_DX[i as usize] as size_t))
                        .wrapping_mul(bytewidth);
                    b = 0 as ::core::ffi::c_uint;
                    while (b as size_t) < bytewidth {
                        *out.offset(pixeloutstart.wrapping_add(b as size_t) as isize) =
                            *in_0.offset(pixelinstart.wrapping_add(b as size_t) as isize);
                        b = b.wrapping_add(1);
                    }
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as ::core::ffi::c_uint;
        while i != 7 as ::core::ffi::c_uint {
            let mut x_0: ::core::ffi::c_uint = 0;
            let mut y_0: ::core::ffi::c_uint = 0;
            let mut b_0: ::core::ffi::c_uint = 0;
            let mut ilinebits: ::core::ffi::c_uint = bpp.wrapping_mul(passw[i as usize]);
            let mut olinebits: ::core::ffi::c_uint = bpp.wrapping_mul(w);
            let mut obp: size_t = 0;
            let mut ibp: size_t = 0;
            y_0 = 0 as ::core::ffi::c_uint;
            while y_0 < passh[i as usize] {
                x_0 = 0 as ::core::ffi::c_uint;
                while x_0 < passw[i as usize] {
                    ibp = (8 as size_t)
                        .wrapping_mul(passstart[i as usize])
                        .wrapping_add(
                            y_0.wrapping_mul(ilinebits)
                                .wrapping_add(x_0.wrapping_mul(bpp))
                                as size_t,
                        );
                    obp = (ADAM7_IY[i as usize] as size_t)
                        .wrapping_add((y_0 as size_t).wrapping_mul(ADAM7_DY[i as usize] as size_t))
                        .wrapping_mul(olinebits as size_t)
                        .wrapping_add(
                            (ADAM7_IX[i as usize] as size_t)
                                .wrapping_add(
                                    (x_0 as size_t).wrapping_mul(ADAM7_DX[i as usize] as size_t),
                                )
                                .wrapping_mul(bpp as size_t),
                        );
                    b_0 = 0 as ::core::ffi::c_uint;
                    while b_0 < bpp {
                        let mut bit: ::core::ffi::c_uchar =
                            readBitFromReversedStream(&raw mut ibp, in_0);
                        setBitOfReversedStream(&raw mut obp, out, bit);
                        b_0 = b_0.wrapping_add(1);
                    }
                    x_0 = x_0.wrapping_add(1);
                }
                y_0 = y_0.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn removePaddingBits(
    mut out: *mut ::core::ffi::c_uchar,
    mut in_0: *const ::core::ffi::c_uchar,
    mut olinebits: size_t,
    mut ilinebits: size_t,
    mut h: ::core::ffi::c_uint,
) {
    let mut y: ::core::ffi::c_uint = 0;
    let mut diff: size_t = ilinebits.wrapping_sub(olinebits);
    let mut ibp: size_t = 0 as size_t;
    let mut obp: size_t = 0 as size_t;
    y = 0 as ::core::ffi::c_uint;
    while y < h {
        let mut x: size_t = 0;
        x = 0 as size_t;
        while x < olinebits {
            let mut bit: ::core::ffi::c_uchar = readBitFromReversedStream(&raw mut ibp, in_0);
            setBitOfReversedStream(&raw mut obp, out, bit);
            x = x.wrapping_add(1);
        }
        ibp = (ibp as ::core::ffi::c_ulong).wrapping_add(diff as ::core::ffi::c_ulong) as size_t
            as size_t;
        y = y.wrapping_add(1);
    }
}
unsafe extern "C" fn postProcessScanlines(
    mut out: *mut ::core::ffi::c_uchar,
    mut in_0: *mut ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut info_png: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut bpp: ::core::ffi::c_uint = lodepng_get_bpp(&raw const (*info_png).color);
    if bpp == 0 as ::core::ffi::c_uint {
        return 31 as ::core::ffi::c_uint;
    }
    if (*info_png).interlace_method == 0 as ::core::ffi::c_uint {
        if bpp < 8 as ::core::ffi::c_uint
            && w.wrapping_mul(bpp)
                != w.wrapping_mul(bpp)
                    .wrapping_add(7 as ::core::ffi::c_uint)
                    .wrapping_div(8 as ::core::ffi::c_uint)
                    .wrapping_mul(8 as ::core::ffi::c_uint)
        {
            let mut error_: ::core::ffi::c_uint = unfilter(in_0, in_0, w, h, bpp);
            if error_ != 0 {
                return error_;
            }
            removePaddingBits(
                out,
                in_0,
                w.wrapping_mul(bpp) as size_t,
                w.wrapping_mul(bpp)
                    .wrapping_add(7 as ::core::ffi::c_uint)
                    .wrapping_div(8 as ::core::ffi::c_uint)
                    .wrapping_mul(8 as ::core::ffi::c_uint) as size_t,
                h,
            );
        } else {
            let mut error__0: ::core::ffi::c_uint = unfilter(out, in_0, w, h, bpp);
            if error__0 != 0 {
                return error__0;
            }
        }
    } else {
        let mut passw: [::core::ffi::c_uint; 7] = [0; 7];
        let mut passh: [::core::ffi::c_uint; 7] = [0; 7];
        let mut filter_passstart: [size_t; 8] = [0; 8];
        let mut padded_passstart: [size_t; 8] = [0; 8];
        let mut passstart: [size_t; 8] = [0; 8];
        let mut i: ::core::ffi::c_uint = 0;
        Adam7_getpassvalues(
            &raw mut passw as *mut ::core::ffi::c_uint,
            &raw mut passh as *mut ::core::ffi::c_uint,
            &raw mut filter_passstart as *mut size_t,
            &raw mut padded_passstart as *mut size_t,
            &raw mut passstart as *mut size_t,
            w,
            h,
            bpp,
        );
        i = 0 as ::core::ffi::c_uint;
        while i != 7 as ::core::ffi::c_uint {
            let mut error__1: ::core::ffi::c_uint = unfilter(
                in_0.offset(*(&raw mut padded_passstart as *mut size_t).offset(i as isize) as isize)
                    as *mut ::core::ffi::c_uchar,
                in_0.offset(*(&raw mut filter_passstart as *mut size_t).offset(i as isize) as isize)
                    as *mut ::core::ffi::c_uchar,
                passw[i as usize],
                passh[i as usize],
                bpp,
            );
            if error__1 != 0 {
                return error__1;
            }
            if bpp < 8 as ::core::ffi::c_uint {
                removePaddingBits(
                    in_0.offset(*(&raw mut passstart as *mut size_t).offset(i as isize) as isize)
                        as *mut ::core::ffi::c_uchar,
                    in_0.offset(
                        *(&raw mut padded_passstart as *mut size_t).offset(i as isize) as isize,
                    ) as *mut ::core::ffi::c_uchar,
                    passw[i as usize].wrapping_mul(bpp) as size_t,
                    passw[i as usize]
                        .wrapping_mul(bpp)
                        .wrapping_add(7 as ::core::ffi::c_uint)
                        .wrapping_div(8 as ::core::ffi::c_uint)
                        .wrapping_mul(8 as ::core::ffi::c_uint) as size_t,
                    passh[i as usize],
                );
            }
            i = i.wrapping_add(1);
        }
        Adam7_deinterlace(out, in_0, w, h, bpp);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_PLTE(
    mut color: *mut LodePNGColorMode,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    let mut pos: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint = 0;
    (*color).palettesize = chunkLength.wrapping_div(3 as size_t);
    if (*color).palettesize == 0 as size_t || (*color).palettesize > 256 as size_t {
        return 38 as ::core::ffi::c_uint;
    }
    lodepng_color_mode_alloc_palette(color);
    if (*color).palette.is_null() && (*color).palettesize != 0 {
        (*color).palettesize = 0 as size_t;
        return 83 as ::core::ffi::c_uint;
    }
    i = 0 as ::core::ffi::c_uint;
    while i as size_t != (*color).palettesize {
        let fresh52 = pos;
        pos = pos.wrapping_add(1);
        *(*color).palette.offset(
            (4 as ::core::ffi::c_uint)
                .wrapping_mul(i)
                .wrapping_add(0 as ::core::ffi::c_uint) as isize,
        ) = *data.offset(fresh52 as isize);
        let fresh53 = pos;
        pos = pos.wrapping_add(1);
        *(*color).palette.offset(
            (4 as ::core::ffi::c_uint)
                .wrapping_mul(i)
                .wrapping_add(1 as ::core::ffi::c_uint) as isize,
        ) = *data.offset(fresh53 as isize);
        let fresh54 = pos;
        pos = pos.wrapping_add(1);
        *(*color).palette.offset(
            (4 as ::core::ffi::c_uint)
                .wrapping_mul(i)
                .wrapping_add(2 as ::core::ffi::c_uint) as isize,
        ) = *data.offset(fresh54 as isize);
        *(*color).palette.offset(
            (4 as ::core::ffi::c_uint)
                .wrapping_mul(i)
                .wrapping_add(3 as ::core::ffi::c_uint) as isize,
        ) = 255 as ::core::ffi::c_uchar;
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_tRNS(
    mut color: *mut LodePNGColorMode,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    if (*color).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength > (*color).palettesize {
            return 39 as ::core::ffi::c_uint;
        }
        i = 0 as ::core::ffi::c_uint;
        while i as size_t != chunkLength {
            *(*color).palette.offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(3 as ::core::ffi::c_uint) as isize,
            ) = *data.offset(i as isize);
            i = i.wrapping_add(1);
        }
    } else if (*color).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength != 2 as size_t {
            return 30 as ::core::ffi::c_uint;
        }
        (*color).key_defined = 1 as ::core::ffi::c_uint;
        (*color).key_b = (256 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
        (*color).key_g = (*color).key_b;
        (*color).key_r = (*color).key_g;
    } else if (*color).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength != 6 as size_t {
            return 41 as ::core::ffi::c_uint;
        }
        (*color).key_defined = 1 as ::core::ffi::c_uint;
        (*color).key_r = (256 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
        (*color).key_g = (256 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
        (*color).key_b = (256 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(*data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    } else {
        return 42 as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_bKGD(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength != 1 as size_t {
            return 43 as ::core::ffi::c_uint;
        }
        if *data.offset(0 as ::core::ffi::c_int as isize) as size_t >= (*info).color.palettesize {
            return 103 as ::core::ffi::c_uint;
        }
        (*info).background_defined = 1 as ::core::ffi::c_uint;
        (*info).background_b =
            *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        (*info).background_g = (*info).background_b;
        (*info).background_r = (*info).background_g;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*info).color.colortype as ::core::ffi::c_uint
            == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength != 2 as size_t {
            return 44 as ::core::ffi::c_uint;
        }
        (*info).background_defined = 1 as ::core::ffi::c_uint;
        (*info).background_b = (256 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
        (*info).background_g = (*info).background_b;
        (*info).background_r = (*info).background_g;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*info).color.colortype as ::core::ffi::c_uint
            == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength != 6 as size_t {
            return 45 as ::core::ffi::c_uint;
        }
        (*info).background_defined = 1 as ::core::ffi::c_uint;
        (*info).background_r = (256 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
        (*info).background_g = (256 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
        (*info).background_b = (256 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(*data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_tEXt(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if error == 0 {
        let mut length: ::core::ffi::c_uint = 0;
        let mut string2_begin: ::core::ffi::c_uint = 0;
        length = 0 as ::core::ffi::c_uint;
        while (length as size_t) < chunkLength
            && *data.offset(length as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            length = length.wrapping_add(1);
        }
        if length < 1 as ::core::ffi::c_uint || length > 79 as ::core::ffi::c_uint {
            error = 89 as ::core::ffi::c_uint;
        } else {
            key = lodepng_malloc(length.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                as *mut ::core::ffi::c_char;
            if key.is_null() {
                error = 83 as ::core::ffi::c_uint;
            } else {
                lodepng_memcpy(
                    key as *mut ::core::ffi::c_void,
                    data as *const ::core::ffi::c_void,
                    length as size_t,
                );
                *key.offset(length as isize) = 0 as ::core::ffi::c_char;
                string2_begin = length.wrapping_add(1 as ::core::ffi::c_uint);
                length = (if chunkLength < string2_begin as size_t {
                    0 as size_t
                } else {
                    chunkLength.wrapping_sub(string2_begin as size_t)
                }) as ::core::ffi::c_uint;
                str = lodepng_malloc(length.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                    as *mut ::core::ffi::c_char;
                if str.is_null() {
                    error = 83 as ::core::ffi::c_uint;
                } else {
                    lodepng_memcpy(
                        str as *mut ::core::ffi::c_void,
                        data.offset(string2_begin as isize) as *const ::core::ffi::c_void,
                        length as size_t,
                    );
                    *str.offset(length as isize) = 0 as ::core::ffi::c_char;
                    error = lodepng_add_text(info, key, str);
                }
            }
        }
    }
    lodepng_free(key as *mut ::core::ffi::c_void);
    lodepng_free(str as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn readChunk_zTXt(
    mut info: *mut LodePNGInfo,
    mut decoder: *const LodePNGDecoderSettings,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut zlibsettings: LodePNGDecompressSettings = (*decoder).zlibsettings;
    let mut length: ::core::ffi::c_uint = 0;
    let mut string2_begin: ::core::ffi::c_uint = 0;
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut str: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut size: size_t = 0 as size_t;
    if error == 0 {
        length = 0 as ::core::ffi::c_uint;
        while (length as size_t) < chunkLength
            && *data.offset(length as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            length = length.wrapping_add(1);
        }
        if length.wrapping_add(2 as ::core::ffi::c_uint) as size_t >= chunkLength {
            error = 75 as ::core::ffi::c_uint;
        } else if length < 1 as ::core::ffi::c_uint || length > 79 as ::core::ffi::c_uint {
            error = 89 as ::core::ffi::c_uint;
        } else {
            key = lodepng_malloc(length.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                as *mut ::core::ffi::c_char;
            if key.is_null() {
                error = 83 as ::core::ffi::c_uint;
            } else {
                lodepng_memcpy(
                    key as *mut ::core::ffi::c_void,
                    data as *const ::core::ffi::c_void,
                    length as size_t,
                );
                *key.offset(length as isize) = 0 as ::core::ffi::c_char;
                if *data.offset(length.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    error = 72 as ::core::ffi::c_uint;
                } else {
                    string2_begin = length.wrapping_add(2 as ::core::ffi::c_uint);
                    if string2_begin as size_t > chunkLength {
                        error = 75 as ::core::ffi::c_uint;
                    } else {
                        length = (chunkLength as ::core::ffi::c_uint).wrapping_sub(string2_begin);
                        zlibsettings.max_output_size = (*decoder).max_text_size;
                        error = zlib_decompress(
                            &raw mut str,
                            &raw mut size,
                            0 as size_t,
                            data.offset(string2_begin as isize) as *const ::core::ffi::c_uchar,
                            length as size_t,
                            &raw mut zlibsettings,
                        );
                        if error != 0 && size > zlibsettings.max_output_size {
                            error = 112 as ::core::ffi::c_uint;
                        }
                        if !(error != 0) {
                            error = lodepng_add_text_sized(
                                info,
                                key,
                                str as *mut ::core::ffi::c_char,
                                size,
                            );
                        }
                    }
                }
            }
        }
    }
    lodepng_free(key as *mut ::core::ffi::c_void);
    lodepng_free(str as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn readChunk_iTXt(
    mut info: *mut LodePNGInfo,
    mut decoder: *const LodePNGDecoderSettings,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint = 0;
    let mut zlibsettings: LodePNGDecompressSettings = (*decoder).zlibsettings;
    let mut length: ::core::ffi::c_uint = 0;
    let mut begin: ::core::ffi::c_uint = 0;
    let mut compressed: ::core::ffi::c_uint = 0;
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut langtag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut transkey: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if error == 0 {
        if chunkLength < 5 as size_t {
            error = 30 as ::core::ffi::c_uint;
        } else {
            length = 0 as ::core::ffi::c_uint;
            while (length as size_t) < chunkLength
                && *data.offset(length as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                length = length.wrapping_add(1);
            }
            if length.wrapping_add(3 as ::core::ffi::c_uint) as size_t >= chunkLength {
                error = 75 as ::core::ffi::c_uint;
            } else if length < 1 as ::core::ffi::c_uint || length > 79 as ::core::ffi::c_uint {
                error = 89 as ::core::ffi::c_uint;
            } else {
                key = lodepng_malloc(length.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                    as *mut ::core::ffi::c_char;
                if key.is_null() {
                    error = 83 as ::core::ffi::c_uint;
                } else {
                    lodepng_memcpy(
                        key as *mut ::core::ffi::c_void,
                        data as *const ::core::ffi::c_void,
                        length as size_t,
                    );
                    *key.offset(length as isize) = 0 as ::core::ffi::c_char;
                    compressed = *data
                        .offset(length.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_uint;
                    if *data.offset(length.wrapping_add(2 as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        error = 72 as ::core::ffi::c_uint;
                    } else {
                        begin = length.wrapping_add(3 as ::core::ffi::c_uint);
                        length = 0 as ::core::ffi::c_uint;
                        i = begin;
                        while (i as size_t) < chunkLength
                            && *data.offset(i as isize) as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int
                        {
                            length = length.wrapping_add(1);
                            i = i.wrapping_add(1);
                        }
                        langtag =
                            lodepng_malloc(length.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                                as *mut ::core::ffi::c_char;
                        if langtag.is_null() {
                            error = 83 as ::core::ffi::c_uint;
                        } else {
                            lodepng_memcpy(
                                langtag as *mut ::core::ffi::c_void,
                                data.offset(begin as isize) as *const ::core::ffi::c_void,
                                length as size_t,
                            );
                            *langtag.offset(length as isize) = 0 as ::core::ffi::c_char;
                            begin =
                                begin.wrapping_add(length.wrapping_add(1 as ::core::ffi::c_uint));
                            length = 0 as ::core::ffi::c_uint;
                            i = begin;
                            while (i as size_t) < chunkLength
                                && *data.offset(i as isize) as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                            {
                                length = length.wrapping_add(1);
                                i = i.wrapping_add(1);
                            }
                            transkey = lodepng_malloc(
                                length.wrapping_add(1 as ::core::ffi::c_uint) as size_t
                            ) as *mut ::core::ffi::c_char;
                            if transkey.is_null() {
                                error = 83 as ::core::ffi::c_uint;
                            } else {
                                lodepng_memcpy(
                                    transkey as *mut ::core::ffi::c_void,
                                    data.offset(begin as isize) as *const ::core::ffi::c_void,
                                    length as size_t,
                                );
                                *transkey.offset(length as isize) = 0 as ::core::ffi::c_char;
                                begin = begin
                                    .wrapping_add(length.wrapping_add(1 as ::core::ffi::c_uint));
                                length = if (chunkLength as ::core::ffi::c_uint) < begin {
                                    0 as ::core::ffi::c_uint
                                } else {
                                    (chunkLength as ::core::ffi::c_uint).wrapping_sub(begin)
                                };
                                if compressed != 0 {
                                    let mut str: *mut ::core::ffi::c_uchar =
                                        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    let mut size: size_t = 0 as size_t;
                                    zlibsettings.max_output_size = (*decoder).max_text_size;
                                    error = zlib_decompress(
                                        &raw mut str,
                                        &raw mut size,
                                        0 as size_t,
                                        data.offset(begin as isize) as *const ::core::ffi::c_uchar,
                                        length as size_t,
                                        &raw mut zlibsettings,
                                    );
                                    if error != 0 && size > zlibsettings.max_output_size {
                                        error = 112 as ::core::ffi::c_uint;
                                    }
                                    if error == 0 {
                                        error = lodepng_add_itext_sized(
                                            info,
                                            key,
                                            langtag,
                                            transkey,
                                            str as *mut ::core::ffi::c_char,
                                            size,
                                        );
                                    }
                                    lodepng_free(str as *mut ::core::ffi::c_void);
                                } else {
                                    error = lodepng_add_itext_sized(
                                        info,
                                        key,
                                        langtag,
                                        transkey,
                                        data.offset(begin as isize) as *const ::core::ffi::c_char,
                                        length as size_t,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    lodepng_free(key as *mut ::core::ffi::c_void);
    lodepng_free(langtag as *mut ::core::ffi::c_void);
    lodepng_free(transkey as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn readChunk_tIME(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 7 as size_t {
        return 73 as ::core::ffi::c_uint;
    }
    (*info).time_defined = 1 as ::core::ffi::c_uint;
    (*info).time.year = (256 as ::core::ffi::c_uint)
        .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        .wrapping_add(*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).time.month = *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).time.day = *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).time.hour = *data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).time.minute = *data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).time.second = *data.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_pHYs(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 9 as size_t {
        return 74 as ::core::ffi::c_uint;
    }
    (*info).phys_defined = 1 as ::core::ffi::c_uint;
    (*info).phys_x =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(
                (65536 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(
                (256 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).phys_y =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(
                (65536 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(
                (256 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(*data.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).phys_unit = *data.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_gAMA(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 4 as size_t {
        return 96 as ::core::ffi::c_uint;
    }
    (*info).gama_defined = 1 as ::core::ffi::c_uint;
    (*info).gama_gamma =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(
                (65536 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(
                (256 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_cHRM(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 32 as size_t {
        return 97 as ::core::ffi::c_uint;
    }
    (*info).chrm_defined = 1 as ::core::ffi::c_uint;
    (*info).chrm_white_x =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(
                (65536 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(
                (256 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).chrm_white_y =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(
                (65536 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(
                (256 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(*data.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).chrm_red_x =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(
                (65536 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add((256 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add(*data.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).chrm_red_y =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add((65536 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add((256 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add(*data.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).chrm_green_x =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add((65536 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(17 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add((256 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(18 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add(*data.offset(19 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).chrm_green_y =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(20 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add((65536 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(21 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add((256 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(22 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add(*data.offset(23 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).chrm_blue_x =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(24 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add((65536 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(25 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add((256 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(26 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add(*data.offset(27 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).chrm_blue_y =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(28 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add((65536 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(29 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add((256 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(30 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add(*data.offset(31 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_sRGB(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 1 as size_t {
        return 98 as ::core::ffi::c_uint;
    }
    (*info).srgb_defined = 1 as ::core::ffi::c_uint;
    (*info).srgb_intent = *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_iCCP(
    mut info: *mut LodePNGInfo,
    mut decoder: *const LodePNGDecoderSettings,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint = 0;
    let mut size: size_t = 0 as size_t;
    let mut zlibsettings: LodePNGDecompressSettings = (*decoder).zlibsettings;
    let mut length: ::core::ffi::c_uint = 0;
    let mut string2_begin: ::core::ffi::c_uint = 0;
    if (*info).iccp_defined != 0 {
        lodepng_clear_icc(info);
    }
    length = 0 as ::core::ffi::c_uint;
    while (length as size_t) < chunkLength
        && *data.offset(length as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        length = length.wrapping_add(1);
    }
    if length.wrapping_add(2 as ::core::ffi::c_uint) as size_t >= chunkLength {
        return 75 as ::core::ffi::c_uint;
    }
    if length < 1 as ::core::ffi::c_uint || length > 79 as ::core::ffi::c_uint {
        return 89 as ::core::ffi::c_uint;
    }
    (*info).iccp_name = lodepng_malloc(length.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
        as *mut ::core::ffi::c_char;
    if (*info).iccp_name.is_null() {
        return 83 as ::core::ffi::c_uint;
    }
    *(*info).iccp_name.offset(length as isize) = 0 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_uint;
    while i != length {
        *(*info).iccp_name.offset(i as isize) = *data.offset(i as isize) as ::core::ffi::c_char;
        i = i.wrapping_add(1);
    }
    if *data.offset(length.wrapping_add(1 as ::core::ffi::c_uint) as isize) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        return 72 as ::core::ffi::c_uint;
    }
    string2_begin = length.wrapping_add(2 as ::core::ffi::c_uint);
    if string2_begin as size_t > chunkLength {
        return 75 as ::core::ffi::c_uint;
    }
    length = (chunkLength as ::core::ffi::c_uint).wrapping_sub(string2_begin);
    zlibsettings.max_output_size = (*decoder).max_icc_size;
    error = zlib_decompress(
        &raw mut (*info).iccp_profile,
        &raw mut size,
        0 as size_t,
        data.offset(string2_begin as isize) as *const ::core::ffi::c_uchar,
        length as size_t,
        &raw mut zlibsettings,
    );
    if error != 0 && size > zlibsettings.max_output_size {
        error = 113 as ::core::ffi::c_uint;
    }
    (*info).iccp_profile_size = size as ::core::ffi::c_uint;
    if error == 0 && (*info).iccp_profile_size == 0 {
        error = 123 as ::core::ffi::c_uint;
    }
    if error == 0 {
        (*info).iccp_defined = 1 as ::core::ffi::c_uint;
    }
    return error;
}
unsafe extern "C" fn readChunk_cICP(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 4 as size_t {
        return 117 as ::core::ffi::c_uint;
    }
    (*info).cicp_defined = 1 as ::core::ffi::c_uint;
    (*info).cicp_color_primaries =
        *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).cicp_transfer_function =
        *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).cicp_matrix_coefficients =
        *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    (*info).cicp_video_full_range_flag =
        *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_mDCV(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 24 as size_t {
        return 119 as ::core::ffi::c_uint;
    }
    (*info).mdcv_defined = 1 as ::core::ffi::c_uint;
    (*info).mdcv_red_x = (256 as ::core::ffi::c_uint)
        .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        .wrapping_add(*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).mdcv_red_y = (256 as ::core::ffi::c_uint)
        .wrapping_mul(*data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).mdcv_green_x = (256 as ::core::ffi::c_uint)
        .wrapping_mul(*data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        .wrapping_add(*data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).mdcv_green_y = (256 as ::core::ffi::c_uint)
        .wrapping_mul(*data.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        .wrapping_add(*data.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).mdcv_blue_x = (256 as ::core::ffi::c_uint)
        .wrapping_mul(*data.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        .wrapping_add(*data.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).mdcv_blue_y = (256 as ::core::ffi::c_uint)
        .wrapping_mul(*data.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        .wrapping_add(*data.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).mdcv_white_x = (256 as ::core::ffi::c_uint)
        .wrapping_mul(*data.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        .wrapping_add(*data.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).mdcv_white_y = (256 as ::core::ffi::c_uint)
        .wrapping_mul(*data.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
        .wrapping_add(*data.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).mdcv_max_luminance =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add((65536 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(17 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add((256 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(18 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add(*data.offset(19 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).mdcv_min_luminance =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(20 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add((65536 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(21 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add((256 as ::core::ffi::c_uint).wrapping_mul(
                *data.offset(22 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ))
            .wrapping_add(*data.offset(23 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_cLLI(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 8 as size_t {
        return 120 as ::core::ffi::c_uint;
    }
    (*info).clli_defined = 1 as ::core::ffi::c_uint;
    (*info).clli_max_cll =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(
                (65536 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(
                (256 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    (*info).clli_max_fall =
        (16777216 as ::core::ffi::c_uint)
            .wrapping_mul(*data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(
                (65536 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(
                (256 as ::core::ffi::c_uint).wrapping_mul(
                    *data.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
            )
            .wrapping_add(*data.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn readChunk_eXIf(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    return lodepng_set_exif(info, data, chunkLength as ::core::ffi::c_uint);
}
unsafe extern "C" fn readChunk_sBIT(
    mut info: *mut LodePNGInfo,
    mut data: *const ::core::ffi::c_uchar,
    mut chunkLength: size_t,
) -> ::core::ffi::c_uint {
    let mut bitdepth: ::core::ffi::c_uint = if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        8 as ::core::ffi::c_uint
    } else {
        (*info).color.bitdepth
    };
    if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength != 1 as size_t {
            return 114 as ::core::ffi::c_uint;
        }
        if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
        {
            return 115 as ::core::ffi::c_uint;
        }
        (*info).sbit_defined = 1 as ::core::ffi::c_uint;
        (*info).sbit_b = *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        (*info).sbit_g = (*info).sbit_b;
        (*info).sbit_r = (*info).sbit_g;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*info).color.colortype as ::core::ffi::c_uint
            == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength != 3 as size_t {
            return 114 as ::core::ffi::c_uint;
        }
        if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            || *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return 115 as ::core::ffi::c_uint;
        }
        if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
            || *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
            || *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
        {
            return 115 as ::core::ffi::c_uint;
        }
        (*info).sbit_defined = 1 as ::core::ffi::c_uint;
        (*info).sbit_r = *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        (*info).sbit_g = *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        (*info).sbit_b = *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength != 2 as size_t {
            return 114 as ::core::ffi::c_uint;
        }
        if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return 115 as ::core::ffi::c_uint;
        }
        if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
            || *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
        {
            return 115 as ::core::ffi::c_uint;
        }
        (*info).sbit_defined = 1 as ::core::ffi::c_uint;
        (*info).sbit_b = *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        (*info).sbit_g = (*info).sbit_b;
        (*info).sbit_r = (*info).sbit_g;
        (*info).sbit_a = *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if chunkLength != 4 as size_t {
            return 114 as ::core::ffi::c_uint;
        }
        if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            || *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            || *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return 115 as ::core::ffi::c_uint;
        }
        if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
            || *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
            || *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
            || *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint > bitdepth
        {
            return 115 as ::core::ffi::c_uint;
        }
        (*info).sbit_defined = 1 as ::core::ffi::c_uint;
        (*info).sbit_r = *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        (*info).sbit_g = *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        (*info).sbit_b = *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        (*info).sbit_a = *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_inspect_chunk(
    mut state: *mut LodePNGState,
    mut pos: size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
) -> ::core::ffi::c_uint {
    let mut chunk: *const ::core::ffi::c_uchar = in_0.offset(pos as isize);
    let mut chunkLength: ::core::ffi::c_uint = 0;
    let mut data: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut unhandled: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if pos.wrapping_add(4 as size_t) > insize {
        return 30 as ::core::ffi::c_uint;
    }
    chunkLength = lodepng_chunk_length(chunk);
    if chunkLength > 2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint {
        return 63 as ::core::ffi::c_uint;
    }
    data = lodepng_chunk_data_const(chunk);
    if chunkLength.wrapping_add(12 as ::core::ffi::c_uint) as size_t > insize.wrapping_sub(pos) {
        return 30 as ::core::ffi::c_uint;
    }
    if lodepng_chunk_type_equals(chunk, b"PLTE\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        error = readChunk_PLTE(
            &raw mut (*state).info_png.color,
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(chunk, b"tRNS\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_tRNS(
            &raw mut (*state).info_png.color,
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(chunk, b"bKGD\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_bKGD(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"tEXt\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_tEXt(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"zTXt\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_zTXt(
            &raw mut (*state).info_png,
            &raw mut (*state).decoder,
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(chunk, b"iTXt\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_iTXt(
            &raw mut (*state).info_png,
            &raw mut (*state).decoder,
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(chunk, b"tIME\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_tIME(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"pHYs\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_pHYs(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"gAMA\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_gAMA(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"cHRM\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_cHRM(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"sRGB\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_sRGB(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"iCCP\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_iCCP(
            &raw mut (*state).info_png,
            &raw mut (*state).decoder,
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(chunk, b"cICP\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_cICP(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"mDCV\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_mDCV(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"cLLI\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_cLLI(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"eXIf\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_eXIf(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(chunk, b"sBIT\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        error = readChunk_sBIT(&raw mut (*state).info_png, data, chunkLength as size_t);
    } else {
        unhandled = 1 as ::core::ffi::c_uint;
    }
    if error == 0 && unhandled == 0 && (*state).decoder.ignore_crc == 0 {
        if lodepng_chunk_check_crc(chunk) != 0 {
            return 57 as ::core::ffi::c_uint;
        }
    }
    return error;
}
unsafe extern "C" fn decodeGeneric(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut w: *mut ::core::ffi::c_uint,
    mut h: *mut ::core::ffi::c_uint,
    mut state: *mut LodePNGState,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
) {
    let mut IEND: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut chunk: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut idat: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut idatsize: size_t = 0 as size_t;
    let mut scanlines: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut scanlines_size: size_t = 0 as size_t;
    let mut expected_size: size_t = 0 as size_t;
    let mut outsize: size_t = 0 as size_t;
    let mut unknown: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut critical_pos: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
    *out = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    *h = 0 as ::core::ffi::c_uint;
    *w = *h;
    (*state).error = lodepng_inspect(w, h, state, in_0, insize);
    if (*state).error != 0 {
        return;
    }
    if lodepng_pixel_overflow(
        *w,
        *h,
        &raw mut (*state).info_png.color,
        &raw mut (*state).info_raw,
    ) != 0
    {
        (*state).error = 92 as ::core::ffi::c_uint;
        return;
    }
    idat = lodepng_malloc(insize) as *mut ::core::ffi::c_uchar;
    if idat.is_null() {
        (*state).error = 83 as ::core::ffi::c_uint;
        return;
    }
    chunk = in_0.offset(33 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_uchar;
    while IEND == 0 && (*state).error == 0 {
        let mut chunkLength: ::core::ffi::c_uint = 0;
        let mut data: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
        let mut pos: size_t = chunk.offset_from(in_0) as ::core::ffi::c_long as size_t;
        if chunk < in_0 || pos.wrapping_add(12 as size_t) > insize {
            if (*state).decoder.ignore_end != 0 {
                break;
            }
            (*state).error = 30 as ::core::ffi::c_uint;
            break;
        } else {
            chunkLength = lodepng_chunk_length(chunk);
            if chunkLength > 2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint {
                if (*state).decoder.ignore_end != 0 {
                    break;
                }
                (*state).error = 63 as ::core::ffi::c_uint;
                break;
            } else if pos
                .wrapping_add(chunkLength as size_t)
                .wrapping_add(12 as size_t)
                > insize
                || pos
                    .wrapping_add(chunkLength as size_t)
                    .wrapping_add(12 as size_t)
                    < pos
            {
                (*state).error = 64 as ::core::ffi::c_uint;
                break;
            } else {
                data = lodepng_chunk_data_const(chunk);
                unknown = 0 as ::core::ffi::c_uint;
                if lodepng_chunk_type_equals(
                    chunk,
                    b"IDAT\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    let mut newsize: size_t = 0;
                    if lodepng_addofl(idatsize, chunkLength as size_t, &raw mut newsize) != 0 {
                        (*state).error = 95 as ::core::ffi::c_uint;
                        break;
                    } else if newsize > insize {
                        (*state).error = 95 as ::core::ffi::c_uint;
                        break;
                    } else {
                        lodepng_memcpy(
                            idat.offset(idatsize as isize) as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            chunkLength as size_t,
                        );
                        idatsize = (idatsize as ::core::ffi::c_ulong)
                            .wrapping_add(chunkLength as ::core::ffi::c_ulong)
                            as size_t as size_t;
                        critical_pos = 3 as ::core::ffi::c_uint;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"IEND\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    IEND = 1 as ::core::ffi::c_uchar;
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"PLTE\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error = readChunk_PLTE(
                        &raw mut (*state).info_png.color,
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                    critical_pos = 2 as ::core::ffi::c_uint;
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"tRNS\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error = readChunk_tRNS(
                        &raw mut (*state).info_png.color,
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"bKGD\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_bKGD(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"tEXt\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    if (*state).decoder.read_text_chunks != 0 {
                        (*state).error =
                            readChunk_tEXt(&raw mut (*state).info_png, data, chunkLength as size_t);
                        if (*state).error != 0 {
                            break;
                        }
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"zTXt\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    if (*state).decoder.read_text_chunks != 0 {
                        (*state).error = readChunk_zTXt(
                            &raw mut (*state).info_png,
                            &raw mut (*state).decoder,
                            data,
                            chunkLength as size_t,
                        );
                        if (*state).error != 0 {
                            break;
                        }
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"iTXt\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    if (*state).decoder.read_text_chunks != 0 {
                        (*state).error = readChunk_iTXt(
                            &raw mut (*state).info_png,
                            &raw mut (*state).decoder,
                            data,
                            chunkLength as size_t,
                        );
                        if (*state).error != 0 {
                            break;
                        }
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"tIME\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_tIME(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"pHYs\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_pHYs(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"gAMA\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_gAMA(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"cHRM\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_cHRM(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"sRGB\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_sRGB(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"iCCP\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error = readChunk_iCCP(
                        &raw mut (*state).info_png,
                        &raw mut (*state).decoder,
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"cICP\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_cICP(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"mDCV\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_mDCV(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"cLLI\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_cLLI(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"eXIf\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_eXIf(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"sBIT\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                {
                    (*state).error =
                        readChunk_sBIT(&raw mut (*state).info_png, data, chunkLength as size_t);
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_name_valid(chunk) == 0 {
                    (*state).error = 121 as ::core::ffi::c_uint;
                    break;
                } else if lodepng_chunk_reserved(chunk) != 0 {
                    (*state).error = 122 as ::core::ffi::c_uint;
                    break;
                } else if (*state).decoder.ignore_critical == 0
                    && lodepng_chunk_ancillary(chunk) == 0
                {
                    (*state).error = 69 as ::core::ffi::c_uint;
                    break;
                } else {
                    unknown = 1 as ::core::ffi::c_uint;
                    if (*state).decoder.remember_unknown_chunks != 0 {
                        (*state).error =
                            lodepng_chunk_append(
                                (&raw mut (*state).info_png.unknown_chunks_data
                                    as *mut *mut ::core::ffi::c_uchar)
                                    .offset(critical_pos.wrapping_sub(1 as ::core::ffi::c_uint)
                                        as isize)
                                    as *mut *mut ::core::ffi::c_uchar,
                                (&raw mut (*state).info_png.unknown_chunks_size as *mut size_t)
                                    .offset(critical_pos.wrapping_sub(1 as ::core::ffi::c_uint)
                                        as isize) as *mut size_t,
                                chunk,
                            );
                        if (*state).error != 0 {
                            break;
                        }
                    }
                }
                if (*state).decoder.ignore_crc == 0 && unknown == 0 {
                    if lodepng_chunk_check_crc(chunk) != 0 {
                        (*state).error = 57 as ::core::ffi::c_uint;
                        break;
                    }
                }
                if IEND == 0 {
                    chunk = lodepng_chunk_next_const(chunk, in_0.offset(insize as isize));
                }
            }
        }
    }
    if (*state).error == 0
        && (*state).info_png.color.colortype as ::core::ffi::c_uint
            == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*state).info_png.color.palette.is_null()
    {
        (*state).error = 106 as ::core::ffi::c_uint;
    }
    if (*state).error == 0 {
        if (*state).info_png.interlace_method == 0 as ::core::ffi::c_uint {
            let mut bpp: ::core::ffi::c_uint = lodepng_get_bpp(&raw mut (*state).info_png.color);
            expected_size = lodepng_get_raw_size_idat(*w, *h, bpp);
        } else {
            let mut bpp_0: ::core::ffi::c_uint = lodepng_get_bpp(&raw mut (*state).info_png.color);
            expected_size = 0 as size_t;
            expected_size = (expected_size as ::core::ffi::c_ulong).wrapping_add(
                lodepng_get_raw_size_idat(
                    (*w).wrapping_add(7 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int,
                    (*h).wrapping_add(7 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int,
                    bpp_0,
                ) as ::core::ffi::c_ulong,
            ) as size_t as size_t;
            if *w > 4 as ::core::ffi::c_uint {
                expected_size = (expected_size as ::core::ffi::c_ulong).wrapping_add(
                    lodepng_get_raw_size_idat(
                        (*w).wrapping_add(3 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int,
                        (*h).wrapping_add(7 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int,
                        bpp_0,
                    ) as ::core::ffi::c_ulong,
                ) as size_t as size_t;
            }
            expected_size = (expected_size as ::core::ffi::c_ulong).wrapping_add(
                lodepng_get_raw_size_idat(
                    (*w).wrapping_add(3 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int,
                    (*h).wrapping_add(3 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int,
                    bpp_0,
                ) as ::core::ffi::c_ulong,
            ) as size_t as size_t;
            if *w > 2 as ::core::ffi::c_uint {
                expected_size = (expected_size as ::core::ffi::c_ulong).wrapping_add(
                    lodepng_get_raw_size_idat(
                        (*w).wrapping_add(1 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int,
                        (*h).wrapping_add(3 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int,
                        bpp_0,
                    ) as ::core::ffi::c_ulong,
                ) as size_t as size_t;
            }
            expected_size = (expected_size as ::core::ffi::c_ulong).wrapping_add(
                lodepng_get_raw_size_idat(
                    (*w).wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int,
                    (*h).wrapping_add(1 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int,
                    bpp_0,
                ) as ::core::ffi::c_ulong,
            ) as size_t as size_t;
            if *w > 1 as ::core::ffi::c_uint {
                expected_size = (expected_size as ::core::ffi::c_ulong).wrapping_add(
                    lodepng_get_raw_size_idat(
                        (*w).wrapping_add(0 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int,
                        (*h).wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int,
                        bpp_0,
                    ) as ::core::ffi::c_ulong,
                ) as size_t as size_t;
            }
            expected_size = (expected_size as ::core::ffi::c_ulong).wrapping_add(
                lodepng_get_raw_size_idat(
                    (*w).wrapping_add(0 as ::core::ffi::c_uint),
                    (*h).wrapping_add(0 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int,
                    bpp_0,
                ) as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        }
        (*state).error = zlib_decompress(
            &raw mut scanlines,
            &raw mut scanlines_size,
            expected_size,
            idat,
            idatsize,
            &raw mut (*state).decoder.zlibsettings,
        );
    }
    if (*state).error == 0 && scanlines_size != expected_size {
        (*state).error = 91 as ::core::ffi::c_uint;
    }
    lodepng_free(idat as *mut ::core::ffi::c_void);
    if (*state).error == 0 {
        outsize = lodepng_get_raw_size(*w, *h, &raw mut (*state).info_png.color);
        *out = lodepng_malloc(outsize) as *mut ::core::ffi::c_uchar;
        if (*out).is_null() {
            (*state).error = 83 as ::core::ffi::c_uint;
        }
    }
    if (*state).error == 0 {
        lodepng_memset(
            *out as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            outsize,
        );
        (*state).error = postProcessScanlines(*out, scanlines, *w, *h, &raw mut (*state).info_png);
    }
    lodepng_free(scanlines as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut w: *mut ::core::ffi::c_uint,
    mut h: *mut ::core::ffi::c_uint,
    mut state: *mut LodePNGState,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
) -> ::core::ffi::c_uint {
    *out = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    decodeGeneric(out, w, h, state, in_0, insize);
    if (*state).error != 0 {
        return (*state).error;
    }
    if (*state).decoder.color_convert == 0
        || lodepng_color_mode_equal(&raw mut (*state).info_raw, &raw mut (*state).info_png.color)
            != 0
    {
        if (*state).decoder.color_convert == 0 {
            (*state).error = lodepng_color_mode_copy(
                &raw mut (*state).info_raw,
                &raw mut (*state).info_png.color,
            );
            if (*state).error != 0 {
                return (*state).error;
            }
        }
    } else {
        let mut data: *mut ::core::ffi::c_uchar = *out;
        let mut outsize: size_t = 0;
        if !((*state).info_raw.colortype as ::core::ffi::c_uint
            == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*state).info_raw.colortype as ::core::ffi::c_uint
                == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint)
            && !((*state).info_raw.bitdepth == 8 as ::core::ffi::c_uint)
        {
            return 56 as ::core::ffi::c_uint;
        }
        outsize = lodepng_get_raw_size(*w, *h, &raw mut (*state).info_raw);
        *out = lodepng_malloc(outsize) as *mut ::core::ffi::c_uchar;
        if (*out).is_null() {
            (*state).error = 83 as ::core::ffi::c_uint;
        } else {
            (*state).error = lodepng_convert(
                *out,
                data,
                &raw mut (*state).info_raw,
                &raw mut (*state).info_png.color,
                *w,
                *h,
            );
        }
        lodepng_free(data as *mut ::core::ffi::c_void);
    }
    return (*state).error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode_memory(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut w: *mut ::core::ffi::c_uint,
    mut h: *mut ::core::ffi::c_uint,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
    mut colortype: LodePNGColorType,
    mut bitdepth: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0;
    let mut state: LodePNGState = ::core::mem::zeroed();
    lodepng_state_init(&raw mut state);
    state.info_raw.colortype = colortype;
    state.info_raw.bitdepth = bitdepth;
    state.decoder.read_text_chunks = 0 as ::core::ffi::c_uint;
    state.decoder.remember_unknown_chunks = 0 as ::core::ffi::c_uint;
    error = lodepng_decode(out, w, h, &raw mut state, in_0, insize);
    lodepng_state_cleanup(&raw mut state);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode32(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut w: *mut ::core::ffi::c_uint,
    mut h: *mut ::core::ffi::c_uint,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
) -> ::core::ffi::c_uint {
    return lodepng_decode_memory(out, w, h, in_0, insize, LCT_RGBA, 8 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode24(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut w: *mut ::core::ffi::c_uint,
    mut h: *mut ::core::ffi::c_uint,
    mut in_0: *const ::core::ffi::c_uchar,
    mut insize: size_t,
) -> ::core::ffi::c_uint {
    return lodepng_decode_memory(out, w, h, in_0, insize, LCT_RGB, 8 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode_file(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut w: *mut ::core::ffi::c_uint,
    mut h: *mut ::core::ffi::c_uint,
    mut filename: *const ::core::ffi::c_char,
    mut colortype: LodePNGColorType,
    mut bitdepth: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut buffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut buffersize: size_t = 0;
    let mut error: ::core::ffi::c_uint = 0;
    *out = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    *h = 0 as ::core::ffi::c_uint;
    *w = *h;
    error = lodepng_load_file(&raw mut buffer, &raw mut buffersize, filename);
    if error == 0 {
        error = lodepng_decode_memory(out, w, h, buffer, buffersize, colortype, bitdepth);
    }
    lodepng_free(buffer as *mut ::core::ffi::c_void);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode32_file(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut w: *mut ::core::ffi::c_uint,
    mut h: *mut ::core::ffi::c_uint,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    return lodepng_decode_file(out, w, h, filename, LCT_RGBA, 8 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode24_file(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut w: *mut ::core::ffi::c_uint,
    mut h: *mut ::core::ffi::c_uint,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    return lodepng_decode_file(out, w, h, filename, LCT_RGB, 8 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decoder_settings_init(mut settings: *mut LodePNGDecoderSettings) {
    (*settings).color_convert = 1 as ::core::ffi::c_uint;
    (*settings).read_text_chunks = 1 as ::core::ffi::c_uint;
    (*settings).remember_unknown_chunks = 0 as ::core::ffi::c_uint;
    (*settings).max_text_size = 16777216 as ::core::ffi::c_int as size_t;
    (*settings).max_icc_size = 16777216 as ::core::ffi::c_int as size_t;
    (*settings).ignore_crc = 0 as ::core::ffi::c_uint;
    (*settings).ignore_critical = 0 as ::core::ffi::c_uint;
    (*settings).ignore_end = 0 as ::core::ffi::c_uint;
    lodepng_decompress_settings_init(&raw mut (*settings).zlibsettings);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_state_init(mut state: *mut LodePNGState) {
    lodepng_decoder_settings_init(&raw mut (*state).decoder);
    lodepng_encoder_settings_init(&raw mut (*state).encoder);
    lodepng_color_mode_init(&raw mut (*state).info_raw);
    lodepng_info_init(&raw mut (*state).info_png);
    (*state).error = 1 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_state_cleanup(mut state: *mut LodePNGState) {
    lodepng_color_mode_cleanup(&raw mut (*state).info_raw);
    lodepng_info_cleanup(&raw mut (*state).info_png);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_state_copy(
    mut dest: *mut LodePNGState,
    mut source: *const LodePNGState,
) -> ::core::ffi::c_uint {
    lodepng_state_cleanup(dest);
    *dest = *source;
    lodepng_color_mode_init(&raw mut (*dest).info_raw);
    lodepng_info_init(&raw mut (*dest).info_png);
    (*dest).error =
        lodepng_color_mode_copy(&raw mut (*dest).info_raw, &raw const (*source).info_raw);
    if (*dest).error != 0 {
        return (*dest).error;
    }
    (*dest).error = lodepng_info_copy(&raw mut (*dest).info_png, &raw const (*source).info_png);
    return (*dest).error;
}
unsafe extern "C" fn writeSignature(mut out: *mut ucvector) -> ::core::ffi::c_uint {
    let mut pos: size_t = (*out).size;
    let signature: [::core::ffi::c_uchar; 8] = [
        137 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        78 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    if ucvector_resize(out, (*out).size.wrapping_add(8 as size_t)) == 0 {
        return 83 as ::core::ffi::c_uint;
    }
    lodepng_memcpy(
        (*out).data.offset(pos as isize) as *mut ::core::ffi::c_void,
        &raw const signature as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        8 as size_t,
    );
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_IHDR(
    mut out: *mut ucvector,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut colortype: LodePNGColorType,
    mut bitdepth: ::core::ffi::c_uint,
    mut interlace_method: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        13 as size_t,
        b"IHDR\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    data = chunk.offset(8 as ::core::ffi::c_int as isize);
    lodepng_set32bitInt(data.offset(0 as ::core::ffi::c_int as isize), w);
    lodepng_set32bitInt(data.offset(4 as ::core::ffi::c_int as isize), h);
    *data.offset(8 as ::core::ffi::c_int as isize) = bitdepth as ::core::ffi::c_uchar;
    *data.offset(9 as ::core::ffi::c_int as isize) = colortype as ::core::ffi::c_uchar;
    *data.offset(10 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_uchar;
    *data.offset(11 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_uchar;
    *data.offset(12 as ::core::ffi::c_int as isize) = interlace_method as ::core::ffi::c_uchar;
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_PLTE(
    mut out: *mut ucvector,
    mut info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut i: size_t = 0;
    let mut j: size_t = 8 as size_t;
    if (*info).palettesize == 0 as size_t || (*info).palettesize > 256 as size_t {
        return 68 as ::core::ffi::c_uint;
    }
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        (*info).palettesize.wrapping_mul(3 as size_t),
        b"PLTE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    i = 0 as size_t;
    while i != (*info).palettesize {
        let fresh94 = j;
        j = j.wrapping_add(1);
        *chunk.offset(fresh94 as isize) = *(*info)
            .palette
            .offset(i.wrapping_mul(4 as size_t).wrapping_add(0 as size_t) as isize);
        let fresh95 = j;
        j = j.wrapping_add(1);
        *chunk.offset(fresh95 as isize) = *(*info)
            .palette
            .offset(i.wrapping_mul(4 as size_t).wrapping_add(1 as size_t) as isize);
        let fresh96 = j;
        j = j.wrapping_add(1);
        *chunk.offset(fresh96 as isize) = *(*info)
            .palette
            .offset(i.wrapping_mul(4 as size_t).wrapping_add(2 as size_t) as isize);
        i = i.wrapping_add(1);
    }
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_tRNS(
    mut out: *mut ucvector,
    mut info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if (*info).colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut i: size_t = 0;
        let mut amount: size_t = (*info).palettesize;
        i = (*info).palettesize;
        while i != 0 as size_t {
            if *(*info).palette.offset(
                (4 as size_t)
                    .wrapping_mul(i.wrapping_sub(1 as size_t))
                    .wrapping_add(3 as size_t) as isize,
            ) as ::core::ffi::c_int
                != 255 as ::core::ffi::c_int
            {
                break;
            }
            amount = amount.wrapping_sub(1);
            i = i.wrapping_sub(1);
        }
        if amount != 0 {
            let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
                &raw mut chunk,
                out,
                amount,
                b"tRNS\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if error_ != 0 {
                return error_;
            }
            i = 0 as size_t;
            while i != amount {
                *chunk.offset((8 as size_t).wrapping_add(i) as isize) = *(*info)
                    .palette
                    .offset((4 as size_t).wrapping_mul(i).wrapping_add(3 as size_t) as isize);
                i = i.wrapping_add(1);
            }
        }
    } else if (*info).colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*info).key_defined != 0 {
            let mut error__0: ::core::ffi::c_uint = lodepng_chunk_init(
                &raw mut chunk,
                out,
                2 as size_t,
                b"tRNS\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if error__0 != 0 {
                return error__0;
            }
            *chunk.offset(8 as ::core::ffi::c_int as isize) =
                ((*info).key_r >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *chunk.offset(9 as ::core::ffi::c_int as isize) =
                ((*info).key_r & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
        }
    } else if (*info).colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*info).key_defined != 0 {
            let mut error__1: ::core::ffi::c_uint = lodepng_chunk_init(
                &raw mut chunk,
                out,
                6 as size_t,
                b"tRNS\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if error__1 != 0 {
                return error__1;
            }
            *chunk.offset(8 as ::core::ffi::c_int as isize) =
                ((*info).key_r >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *chunk.offset(9 as ::core::ffi::c_int as isize) =
                ((*info).key_r & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
            *chunk.offset(10 as ::core::ffi::c_int as isize) =
                ((*info).key_g >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *chunk.offset(11 as ::core::ffi::c_int as isize) =
                ((*info).key_g & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
            *chunk.offset(12 as ::core::ffi::c_int as isize) =
                ((*info).key_b >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *chunk.offset(13 as ::core::ffi::c_int as isize) =
                ((*info).key_b & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
        }
    }
    if !chunk.is_null() {
        lodepng_chunk_generate_crc(chunk);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_IDAT(
    mut out: *mut ucvector,
    mut data: *const ::core::ffi::c_uchar,
    mut datasize: size_t,
    mut zlibsettings: *const LodePNGCompressSettings,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut zlib: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pos: size_t = 0 as size_t;
    let mut zlibsize: size_t = 0 as size_t;
    let max_chunk_length: size_t = 2147483647 as ::core::ffi::c_uint as size_t;
    error = zlib_compress(
        &raw mut zlib,
        &raw mut zlibsize,
        data,
        datasize,
        zlibsettings,
    );
    while error == 0 {
        if zlibsize.wrapping_sub(pos) > max_chunk_length {
            error = lodepng_chunk_createv(
                out,
                max_chunk_length,
                b"IDAT\0" as *const u8 as *const ::core::ffi::c_char,
                zlib.offset(pos as isize),
            );
            pos = (pos as ::core::ffi::c_ulong)
                .wrapping_add(max_chunk_length as ::core::ffi::c_ulong) as size_t
                as size_t;
        } else {
            error = lodepng_chunk_createv(
                out,
                zlibsize.wrapping_sub(pos),
                b"IDAT\0" as *const u8 as *const ::core::ffi::c_char,
                zlib.offset(pos as isize),
            );
            break;
        }
    }
    lodepng_free(zlib as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn addChunk_IEND(mut out: *mut ucvector) -> ::core::ffi::c_uint {
    return lodepng_chunk_createv(
        out,
        0 as size_t,
        b"IEND\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_uchar>(),
    );
}
unsafe extern "C" fn addChunk_tEXt(
    mut out: *mut ucvector,
    mut keyword: *const ::core::ffi::c_char,
    mut textstring: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut keysize: size_t = lodepng_strlen(keyword);
    let mut textsize: size_t = lodepng_strlen(textstring);
    let mut size: size_t = keysize.wrapping_add(1 as size_t).wrapping_add(textsize);
    if keysize < 1 as size_t || keysize > 79 as size_t {
        return 89 as ::core::ffi::c_uint;
    }
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        size,
        b"tEXt\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    lodepng_memcpy(
        chunk.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        keyword as *const ::core::ffi::c_void,
        keysize,
    );
    *chunk.offset((8 as size_t).wrapping_add(keysize) as isize) = 0 as ::core::ffi::c_uchar;
    lodepng_memcpy(
        chunk
            .offset(9 as ::core::ffi::c_int as isize)
            .offset(keysize as isize) as *mut ::core::ffi::c_void,
        textstring as *const ::core::ffi::c_void,
        textsize,
    );
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_zTXt(
    mut out: *mut ucvector,
    mut keyword: *const ::core::ffi::c_char,
    mut textstring: *const ::core::ffi::c_char,
    mut zlibsettings: *const LodePNGCompressSettings,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut compressed: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut compressedsize: size_t = 0 as size_t;
    let mut textsize: size_t = lodepng_strlen(textstring);
    let mut keysize: size_t = lodepng_strlen(keyword);
    if keysize < 1 as size_t || keysize > 79 as size_t {
        return 89 as ::core::ffi::c_uint;
    }
    error = zlib_compress(
        &raw mut compressed,
        &raw mut compressedsize,
        textstring as *const ::core::ffi::c_uchar,
        textsize,
        zlibsettings,
    );
    if error == 0 {
        let mut size: size_t = keysize
            .wrapping_add(2 as size_t)
            .wrapping_add(compressedsize);
        error = lodepng_chunk_init(
            &raw mut chunk,
            out,
            size,
            b"zTXt\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if error == 0 {
        lodepng_memcpy(
            chunk.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            keyword as *const ::core::ffi::c_void,
            keysize,
        );
        *chunk.offset((8 as size_t).wrapping_add(keysize) as isize) = 0 as ::core::ffi::c_uchar;
        *chunk.offset((9 as size_t).wrapping_add(keysize) as isize) = 0 as ::core::ffi::c_uchar;
        lodepng_memcpy(
            chunk
                .offset(10 as ::core::ffi::c_int as isize)
                .offset(keysize as isize) as *mut ::core::ffi::c_void,
            compressed as *const ::core::ffi::c_void,
            compressedsize,
        );
        lodepng_chunk_generate_crc(chunk);
    }
    lodepng_free(compressed as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn addChunk_iTXt(
    mut out: *mut ucvector,
    mut compress: ::core::ffi::c_uint,
    mut keyword: *const ::core::ffi::c_char,
    mut langtag: *const ::core::ffi::c_char,
    mut transkey: *const ::core::ffi::c_char,
    mut textstring: *const ::core::ffi::c_char,
    mut zlibsettings: *const LodePNGCompressSettings,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut compressed: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut compressedsize: size_t = 0 as size_t;
    let mut textsize: size_t = lodepng_strlen(textstring);
    let mut keysize: size_t = lodepng_strlen(keyword);
    let mut langsize: size_t = lodepng_strlen(langtag);
    let mut transsize: size_t = lodepng_strlen(transkey);
    if keysize < 1 as size_t || keysize > 79 as size_t {
        return 89 as ::core::ffi::c_uint;
    }
    if compress != 0 {
        error = zlib_compress(
            &raw mut compressed,
            &raw mut compressedsize,
            textstring as *const ::core::ffi::c_uchar,
            textsize,
            zlibsettings,
        );
    }
    if error == 0 {
        let mut size: size_t = keysize
            .wrapping_add(3 as size_t)
            .wrapping_add(langsize)
            .wrapping_add(1 as size_t)
            .wrapping_add(transsize)
            .wrapping_add(1 as size_t)
            .wrapping_add(
                (if compress != 0 {
                    compressedsize
                } else {
                    textsize
                }),
            );
        error = lodepng_chunk_init(
            &raw mut chunk,
            out,
            size,
            b"iTXt\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if error == 0 {
        let mut pos: size_t = 8 as size_t;
        lodepng_memcpy(
            chunk.offset(pos as isize) as *mut ::core::ffi::c_void,
            keyword as *const ::core::ffi::c_void,
            keysize,
        );
        pos = (pos as ::core::ffi::c_ulong).wrapping_add(keysize as ::core::ffi::c_ulong) as size_t
            as size_t;
        let fresh55 = pos;
        pos = pos.wrapping_add(1);
        *chunk.offset(fresh55 as isize) = 0 as ::core::ffi::c_uchar;
        let fresh56 = pos;
        pos = pos.wrapping_add(1);
        *chunk.offset(fresh56 as isize) = (if compress != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uchar;
        let fresh57 = pos;
        pos = pos.wrapping_add(1);
        *chunk.offset(fresh57 as isize) = 0 as ::core::ffi::c_uchar;
        lodepng_memcpy(
            chunk.offset(pos as isize) as *mut ::core::ffi::c_void,
            langtag as *const ::core::ffi::c_void,
            langsize,
        );
        pos = (pos as ::core::ffi::c_ulong).wrapping_add(langsize as ::core::ffi::c_ulong) as size_t
            as size_t;
        let fresh58 = pos;
        pos = pos.wrapping_add(1);
        *chunk.offset(fresh58 as isize) = 0 as ::core::ffi::c_uchar;
        lodepng_memcpy(
            chunk.offset(pos as isize) as *mut ::core::ffi::c_void,
            transkey as *const ::core::ffi::c_void,
            transsize,
        );
        pos = (pos as ::core::ffi::c_ulong).wrapping_add(transsize as ::core::ffi::c_ulong)
            as size_t as size_t;
        let fresh59 = pos;
        pos = pos.wrapping_add(1);
        *chunk.offset(fresh59 as isize) = 0 as ::core::ffi::c_uchar;
        if compress != 0 {
            lodepng_memcpy(
                chunk.offset(pos as isize) as *mut ::core::ffi::c_void,
                compressed as *const ::core::ffi::c_void,
                compressedsize,
            );
        } else {
            lodepng_memcpy(
                chunk.offset(pos as isize) as *mut ::core::ffi::c_void,
                textstring as *const ::core::ffi::c_void,
                textsize,
            );
        }
        lodepng_chunk_generate_crc(chunk);
    }
    lodepng_free(compressed as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn addChunk_bKGD(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*info).color.colortype as ::core::ffi::c_uint
            == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
            &raw mut chunk,
            out,
            2 as size_t,
            b"bKGD\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if error_ != 0 {
            return error_;
        }
        *chunk.offset(8 as ::core::ffi::c_int as isize) =
            ((*info).background_r >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *chunk.offset(9 as ::core::ffi::c_int as isize) =
            ((*info).background_r & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*info).color.colortype as ::core::ffi::c_uint
            == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut error__0: ::core::ffi::c_uint = lodepng_chunk_init(
            &raw mut chunk,
            out,
            6 as size_t,
            b"bKGD\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if error__0 != 0 {
            return error__0;
        }
        *chunk.offset(8 as ::core::ffi::c_int as isize) =
            ((*info).background_r >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *chunk.offset(9 as ::core::ffi::c_int as isize) =
            ((*info).background_r & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
        *chunk.offset(10 as ::core::ffi::c_int as isize) =
            ((*info).background_g >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *chunk.offset(11 as ::core::ffi::c_int as isize) =
            ((*info).background_g & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
        *chunk.offset(12 as ::core::ffi::c_int as isize) =
            ((*info).background_b >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *chunk.offset(13 as ::core::ffi::c_int as isize) =
            ((*info).background_b & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut error__1: ::core::ffi::c_uint = lodepng_chunk_init(
            &raw mut chunk,
            out,
            1 as size_t,
            b"bKGD\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if error__1 != 0 {
            return error__1;
        }
        *chunk.offset(8 as ::core::ffi::c_int as isize) =
            ((*info).background_r & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    }
    if !chunk.is_null() {
        lodepng_chunk_generate_crc(chunk);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_tIME(
    mut out: *mut ucvector,
    mut time: *const LodePNGTime,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        7 as size_t,
        b"tIME\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    *chunk.offset(8 as ::core::ffi::c_int as isize) =
        ((*time).year >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    *chunk.offset(9 as ::core::ffi::c_int as isize) =
        ((*time).year & 255 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *chunk.offset(10 as ::core::ffi::c_int as isize) = (*time).month as ::core::ffi::c_uchar;
    *chunk.offset(11 as ::core::ffi::c_int as isize) = (*time).day as ::core::ffi::c_uchar;
    *chunk.offset(12 as ::core::ffi::c_int as isize) = (*time).hour as ::core::ffi::c_uchar;
    *chunk.offset(13 as ::core::ffi::c_int as isize) = (*time).minute as ::core::ffi::c_uchar;
    *chunk.offset(14 as ::core::ffi::c_int as isize) = (*time).second as ::core::ffi::c_uchar;
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_pHYs(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        9 as size_t,
        b"pHYs\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    lodepng_set32bitInt(
        chunk.offset(8 as ::core::ffi::c_int as isize),
        (*info).phys_x,
    );
    lodepng_set32bitInt(
        chunk.offset(12 as ::core::ffi::c_int as isize),
        (*info).phys_y,
    );
    *chunk.offset(16 as ::core::ffi::c_int as isize) = (*info).phys_unit as ::core::ffi::c_uchar;
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_gAMA(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        4 as size_t,
        b"gAMA\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    lodepng_set32bitInt(
        chunk.offset(8 as ::core::ffi::c_int as isize),
        (*info).gama_gamma,
    );
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_cHRM(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        32 as size_t,
        b"cHRM\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    lodepng_set32bitInt(
        chunk.offset(8 as ::core::ffi::c_int as isize),
        (*info).chrm_white_x,
    );
    lodepng_set32bitInt(
        chunk.offset(12 as ::core::ffi::c_int as isize),
        (*info).chrm_white_y,
    );
    lodepng_set32bitInt(
        chunk.offset(16 as ::core::ffi::c_int as isize),
        (*info).chrm_red_x,
    );
    lodepng_set32bitInt(
        chunk.offset(20 as ::core::ffi::c_int as isize),
        (*info).chrm_red_y,
    );
    lodepng_set32bitInt(
        chunk.offset(24 as ::core::ffi::c_int as isize),
        (*info).chrm_green_x,
    );
    lodepng_set32bitInt(
        chunk.offset(28 as ::core::ffi::c_int as isize),
        (*info).chrm_green_y,
    );
    lodepng_set32bitInt(
        chunk.offset(32 as ::core::ffi::c_int as isize),
        (*info).chrm_blue_x,
    );
    lodepng_set32bitInt(
        chunk.offset(36 as ::core::ffi::c_int as isize),
        (*info).chrm_blue_y,
    );
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_sRGB(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut data: ::core::ffi::c_uchar = (*info).srgb_intent as ::core::ffi::c_uchar;
    return lodepng_chunk_createv(
        out,
        1 as size_t,
        b"sRGB\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut data,
    );
}
unsafe extern "C" fn addChunk_iCCP(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
    mut zlibsettings: *const LodePNGCompressSettings,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut compressed: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut compressedsize: size_t = 0 as size_t;
    let mut keysize: size_t = lodepng_strlen((*info).iccp_name);
    if keysize < 1 as size_t || keysize > 79 as size_t {
        return 89 as ::core::ffi::c_uint;
    }
    error = zlib_compress(
        &raw mut compressed,
        &raw mut compressedsize,
        (*info).iccp_profile,
        (*info).iccp_profile_size as size_t,
        zlibsettings,
    );
    if error == 0 {
        let mut size: size_t = keysize
            .wrapping_add(2 as size_t)
            .wrapping_add(compressedsize);
        error = lodepng_chunk_init(
            &raw mut chunk,
            out,
            size,
            b"iCCP\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if error == 0 {
        lodepng_memcpy(
            chunk.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*info).iccp_name as *const ::core::ffi::c_void,
            keysize,
        );
        *chunk.offset((8 as size_t).wrapping_add(keysize) as isize) = 0 as ::core::ffi::c_uchar;
        *chunk.offset((9 as size_t).wrapping_add(keysize) as isize) = 0 as ::core::ffi::c_uchar;
        lodepng_memcpy(
            chunk
                .offset(10 as ::core::ffi::c_int as isize)
                .offset(keysize as isize) as *mut ::core::ffi::c_void,
            compressed as *const ::core::ffi::c_void,
            compressedsize,
        );
        lodepng_chunk_generate_crc(chunk);
    }
    lodepng_free(compressed as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C" fn addChunk_cICP(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if (*info).cicp_color_primaries > 255 as ::core::ffi::c_uint {
        return 116 as ::core::ffi::c_uint;
    }
    if (*info).cicp_transfer_function > 255 as ::core::ffi::c_uint {
        return 116 as ::core::ffi::c_uint;
    }
    if (*info).cicp_matrix_coefficients > 255 as ::core::ffi::c_uint {
        return 116 as ::core::ffi::c_uint;
    }
    if (*info).cicp_video_full_range_flag > 255 as ::core::ffi::c_uint {
        return 116 as ::core::ffi::c_uint;
    }
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        4 as size_t,
        b"cICP\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    *chunk.offset((8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
        (*info).cicp_color_primaries as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
        (*info).cicp_transfer_function as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
        (*info).cicp_matrix_coefficients as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
        (*info).cicp_video_full_range_flag as ::core::ffi::c_uchar;
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_mDCV(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if (*info).mdcv_red_x > 65535 as ::core::ffi::c_uint {
        return 118 as ::core::ffi::c_uint;
    }
    if (*info).mdcv_red_y > 65535 as ::core::ffi::c_uint {
        return 118 as ::core::ffi::c_uint;
    }
    if (*info).mdcv_green_x > 65535 as ::core::ffi::c_uint {
        return 118 as ::core::ffi::c_uint;
    }
    if (*info).mdcv_green_y > 65535 as ::core::ffi::c_uint {
        return 118 as ::core::ffi::c_uint;
    }
    if (*info).mdcv_blue_x > 65535 as ::core::ffi::c_uint {
        return 118 as ::core::ffi::c_uint;
    }
    if (*info).mdcv_blue_y > 65535 as ::core::ffi::c_uint {
        return 118 as ::core::ffi::c_uint;
    }
    if (*info).mdcv_white_x > 65535 as ::core::ffi::c_uint {
        return 118 as ::core::ffi::c_uint;
    }
    if (*info).mdcv_white_y > 65535 as ::core::ffi::c_uint {
        return 118 as ::core::ffi::c_uint;
    }
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        24 as size_t,
        b"mDCV\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    *chunk.offset((8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
        ((*info).mdcv_red_x >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
        (*info).mdcv_red_x as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
        ((*info).mdcv_red_y >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
        (*info).mdcv_red_y as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize) =
        ((*info).mdcv_green_x >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize) =
        (*info).mdcv_green_x as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize) =
        ((*info).mdcv_green_y >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize) =
        (*info).mdcv_green_y as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize) =
        ((*info).mdcv_blue_x >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize) =
        (*info).mdcv_blue_x as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize) =
        ((*info).mdcv_blue_y >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize) =
        (*info).mdcv_blue_y as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize) =
        ((*info).mdcv_white_x >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize) =
        (*info).mdcv_white_x as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize) =
        ((*info).mdcv_white_y >> 8 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    *chunk.offset((8 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize) =
        (*info).mdcv_white_y as ::core::ffi::c_uchar;
    lodepng_set32bitInt(
        chunk
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(16 as ::core::ffi::c_int as isize),
        (*info).mdcv_max_luminance,
    );
    lodepng_set32bitInt(
        chunk
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(20 as ::core::ffi::c_int as isize),
        (*info).mdcv_min_luminance,
    );
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_cLLI(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
        &raw mut chunk,
        out,
        8 as size_t,
        b"cLLI\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if error_ != 0 {
        return error_;
    }
    lodepng_set32bitInt(
        chunk
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(0 as ::core::ffi::c_int as isize),
        (*info).clli_max_cll,
    );
    lodepng_set32bitInt(
        chunk
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(4 as ::core::ffi::c_int as isize),
        (*info).clli_max_fall,
    );
    lodepng_chunk_generate_crc(chunk);
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn addChunk_eXIf(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    return lodepng_chunk_createv(
        out,
        (*info).exif_size as size_t,
        b"eXIf\0" as *const u8 as *const ::core::ffi::c_char,
        (*info).exif,
    );
}
unsafe extern "C" fn addChunk_sBIT(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
) -> ::core::ffi::c_uint {
    let mut bitdepth: ::core::ffi::c_uint = if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        8 as ::core::ffi::c_uint
    } else {
        (*info).color.bitdepth
    };
    let mut chunk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*info).sbit_r == 0 as ::core::ffi::c_uint || (*info).sbit_r > bitdepth {
            return 115 as ::core::ffi::c_uint;
        }
        let mut error_: ::core::ffi::c_uint = lodepng_chunk_init(
            &raw mut chunk,
            out,
            1 as size_t,
            b"sBIT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if error_ != 0 {
            return error_;
        }
        *chunk.offset(8 as ::core::ffi::c_int as isize) = (*info).sbit_r as ::core::ffi::c_uchar;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*info).color.colortype as ::core::ffi::c_uint
            == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*info).sbit_r == 0 as ::core::ffi::c_uint
            || (*info).sbit_g == 0 as ::core::ffi::c_uint
            || (*info).sbit_b == 0 as ::core::ffi::c_uint
        {
            return 115 as ::core::ffi::c_uint;
        }
        if (*info).sbit_r > bitdepth || (*info).sbit_g > bitdepth || (*info).sbit_b > bitdepth {
            return 115 as ::core::ffi::c_uint;
        }
        let mut error__0: ::core::ffi::c_uint = lodepng_chunk_init(
            &raw mut chunk,
            out,
            3 as size_t,
            b"sBIT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if error__0 != 0 {
            return error__0;
        }
        *chunk.offset(8 as ::core::ffi::c_int as isize) = (*info).sbit_r as ::core::ffi::c_uchar;
        *chunk.offset(9 as ::core::ffi::c_int as isize) = (*info).sbit_g as ::core::ffi::c_uchar;
        *chunk.offset(10 as ::core::ffi::c_int as isize) = (*info).sbit_b as ::core::ffi::c_uchar;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*info).sbit_r == 0 as ::core::ffi::c_uint || (*info).sbit_a == 0 as ::core::ffi::c_uint
        {
            return 115 as ::core::ffi::c_uint;
        }
        if (*info).sbit_r > bitdepth || (*info).sbit_a > bitdepth {
            return 115 as ::core::ffi::c_uint;
        }
        let mut error__1: ::core::ffi::c_uint = lodepng_chunk_init(
            &raw mut chunk,
            out,
            2 as size_t,
            b"sBIT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if error__1 != 0 {
            return error__1;
        }
        *chunk.offset(8 as ::core::ffi::c_int as isize) = (*info).sbit_r as ::core::ffi::c_uchar;
        *chunk.offset(9 as ::core::ffi::c_int as isize) = (*info).sbit_a as ::core::ffi::c_uchar;
    } else if (*info).color.colortype as ::core::ffi::c_uint
        == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*info).sbit_r == 0 as ::core::ffi::c_uint
            || (*info).sbit_g == 0 as ::core::ffi::c_uint
            || (*info).sbit_b == 0 as ::core::ffi::c_uint
            || (*info).sbit_a == 0 as ::core::ffi::c_uint
            || (*info).sbit_r > bitdepth
            || (*info).sbit_g > bitdepth
            || (*info).sbit_b > bitdepth
            || (*info).sbit_a > bitdepth
        {
            return 115 as ::core::ffi::c_uint;
        }
        let mut error__2: ::core::ffi::c_uint = lodepng_chunk_init(
            &raw mut chunk,
            out,
            4 as size_t,
            b"sBIT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if error__2 != 0 {
            return error__2;
        }
        *chunk.offset(8 as ::core::ffi::c_int as isize) = (*info).sbit_r as ::core::ffi::c_uchar;
        *chunk.offset(9 as ::core::ffi::c_int as isize) = (*info).sbit_g as ::core::ffi::c_uchar;
        *chunk.offset(10 as ::core::ffi::c_int as isize) = (*info).sbit_b as ::core::ffi::c_uchar;
        *chunk.offset(11 as ::core::ffi::c_int as isize) = (*info).sbit_a as ::core::ffi::c_uchar;
    }
    if !chunk.is_null() {
        lodepng_chunk_generate_crc(chunk);
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn filterScanline(
    mut out: *mut ::core::ffi::c_uchar,
    mut scanline: *const ::core::ffi::c_uchar,
    mut prevline: *const ::core::ffi::c_uchar,
    mut length: size_t,
    mut bytewidth: size_t,
    mut filterType: ::core::ffi::c_uchar,
) {
    let mut i: size_t = 0;
    match filterType as ::core::ffi::c_int {
        0 => {
            lodepng_memcpy(
                out as *mut ::core::ffi::c_void,
                scanline as *const ::core::ffi::c_void,
                length,
            );
        }
        1 => {
            i = 0 as size_t;
            while i != bytewidth {
                *out.offset(i as isize) = *scanline.offset(i as isize);
                i = i.wrapping_add(1);
            }
            i = bytewidth;
            while i < length {
                *out.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                    - *scanline.offset(i.wrapping_sub(bytewidth) as isize) as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
                i = i.wrapping_add(1);
            }
        }
        2 => {
            if !prevline.is_null() {
                i = 0 as size_t;
                while i != length {
                    *out.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        - *prevline.offset(i as isize) as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
            } else {
                lodepng_memcpy(
                    out as *mut ::core::ffi::c_void,
                    scanline as *const ::core::ffi::c_void,
                    length,
                );
            }
        }
        3 => {
            if !prevline.is_null() {
                i = 0 as size_t;
                while i != bytewidth {
                    *out.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        - (*prevline.offset(i as isize) as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int))
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i < length {
                    *out.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        - (*scanline.offset(i.wrapping_sub(bytewidth) as isize)
                            as ::core::ffi::c_int
                            + *prevline.offset(i as isize) as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int))
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
            } else {
                i = 0 as size_t;
                while i != bytewidth {
                    *out.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i < length {
                    *out.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        - (*scanline.offset(i.wrapping_sub(bytewidth) as isize)
                            as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int))
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
            }
        }
        4 => {
            if !prevline.is_null() {
                i = 0 as size_t;
                while i != bytewidth {
                    *out.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        - *prevline.offset(i as isize) as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i < length {
                    *out.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        - paethPredictor(
                            *scanline.offset(i.wrapping_sub(bytewidth) as isize),
                            *prevline.offset(i as isize),
                            *prevline.offset(i.wrapping_sub(bytewidth) as isize),
                        ) as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
            } else {
                i = 0 as size_t;
                while i != bytewidth {
                    *out.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i < length {
                    *out.offset(i as isize) = (*scanline.offset(i as isize) as ::core::ffi::c_int
                        - *scanline.offset(i.wrapping_sub(bytewidth) as isize)
                            as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
            }
        }
        _ => return,
    };
}
unsafe extern "C" fn ilog2(mut i: size_t) -> size_t {
    let mut result: size_t = 0 as size_t;
    if i >= 65536 as ::core::ffi::c_int as size_t {
        result = (result as ::core::ffi::c_ulong).wrapping_add(16 as ::core::ffi::c_ulong) as size_t
            as size_t;
        i >>= 16 as ::core::ffi::c_int;
    }
    if i >= 256 as size_t {
        result = (result as ::core::ffi::c_ulong).wrapping_add(8 as ::core::ffi::c_ulong) as size_t
            as size_t;
        i >>= 8 as ::core::ffi::c_int;
    }
    if i >= 16 as size_t {
        result = (result as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong) as size_t
            as size_t;
        i >>= 4 as ::core::ffi::c_int;
    }
    if i >= 4 as size_t {
        result = (result as ::core::ffi::c_ulong).wrapping_add(2 as ::core::ffi::c_ulong) as size_t
            as size_t;
        i >>= 2 as ::core::ffi::c_int;
    }
    if i >= 2 as size_t {
        result = (result as ::core::ffi::c_ulong).wrapping_add(1 as ::core::ffi::c_ulong) as size_t
            as size_t;
    }
    return result;
}
unsafe extern "C" fn ilog2i(mut i: size_t) -> size_t {
    let mut l: size_t = 0;
    if i == 0 as size_t {
        return 0 as size_t;
    }
    l = ilog2(i);
    return i.wrapping_mul(l).wrapping_add(
        i.wrapping_sub((1 as ::core::ffi::c_int as size_t) << l) << 1 as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn filter(
    mut out: *mut ::core::ffi::c_uchar,
    mut in_0: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut color: *const LodePNGColorMode,
    mut settings: *const LodePNGEncoderSettings,
) -> ::core::ffi::c_uint {
    let mut bpp: ::core::ffi::c_uint = lodepng_get_bpp(color);
    let mut linebytes: size_t =
        lodepng_get_raw_size_idat(w, 1 as ::core::ffi::c_uint, bpp).wrapping_sub(1 as size_t);
    let mut bytewidth: size_t = bpp
        .wrapping_add(7 as ::core::ffi::c_uint)
        .wrapping_div(8 as ::core::ffi::c_uint) as size_t;
    let mut prevline: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut x: ::core::ffi::c_uint = 0;
    let mut y: ::core::ffi::c_uint = 0;
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut strategy: LodePNGFilterStrategy = (*settings).filter_strategy;
    if (*settings).filter_palette_zero != 0
        && ((*color).colortype as ::core::ffi::c_uint
            == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*color).bitdepth < 8 as ::core::ffi::c_uint)
    {
        strategy = LFS_ZERO;
    }
    if bpp == 0 as ::core::ffi::c_uint {
        return 31 as ::core::ffi::c_uint;
    }
    if strategy as ::core::ffi::c_uint >= LFS_ZERO as ::core::ffi::c_int as ::core::ffi::c_uint
        && strategy as ::core::ffi::c_uint <= LFS_FOUR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut type_0: ::core::ffi::c_uchar = strategy as ::core::ffi::c_uchar;
        y = 0 as ::core::ffi::c_uint;
        while y != h {
            let mut outindex: size_t = (1 as size_t)
                .wrapping_add(linebytes)
                .wrapping_mul(y as size_t);
            let mut inindex: size_t = linebytes.wrapping_mul(y as size_t);
            *out.offset(outindex as isize) = type_0;
            filterScanline(
                out.offset(outindex.wrapping_add(1 as size_t) as isize)
                    as *mut ::core::ffi::c_uchar,
                in_0.offset(inindex as isize) as *const ::core::ffi::c_uchar,
                prevline,
                linebytes,
                bytewidth,
                type_0,
            );
            prevline = in_0.offset(inindex as isize) as *const ::core::ffi::c_uchar;
            y = y.wrapping_add(1);
        }
    } else if strategy as ::core::ffi::c_uint
        == LFS_MINSUM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut attempt: [*mut ::core::ffi::c_uchar; 5] =
            [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 5];
        let mut smallest: size_t = 0 as size_t;
        let mut type_1: ::core::ffi::c_uchar = 0;
        let mut bestType: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
        type_1 = 0 as ::core::ffi::c_uchar;
        while type_1 as ::core::ffi::c_int != 5 as ::core::ffi::c_int {
            attempt[type_1 as usize] = lodepng_malloc(linebytes) as *mut ::core::ffi::c_uchar;
            if attempt[type_1 as usize].is_null() {
                error = 83 as ::core::ffi::c_uint;
            }
            type_1 = type_1.wrapping_add(1);
        }
        if error == 0 {
            y = 0 as ::core::ffi::c_uint;
            while y != h {
                type_1 = 0 as ::core::ffi::c_uchar;
                while type_1 as ::core::ffi::c_int != 5 as ::core::ffi::c_int {
                    let mut sum: size_t = 0 as size_t;
                    filterScanline(
                        attempt[type_1 as usize],
                        in_0.offset((y as size_t).wrapping_mul(linebytes) as isize)
                            as *const ::core::ffi::c_uchar,
                        prevline,
                        linebytes,
                        bytewidth,
                        type_1,
                    );
                    if type_1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        x = 0 as ::core::ffi::c_uint;
                        while x as size_t != linebytes {
                            sum = (sum as ::core::ffi::c_ulong)
                                .wrapping_add(*attempt[type_1 as usize].offset(x as isize)
                                    as ::core::ffi::c_ulong)
                                as size_t as size_t;
                            x = x.wrapping_add(1);
                        }
                    } else {
                        x = 0 as ::core::ffi::c_uint;
                        while x as size_t != linebytes {
                            let mut s: ::core::ffi::c_uchar =
                                *attempt[type_1 as usize].offset(x as isize);
                            sum = (sum as ::core::ffi::c_ulong).wrapping_add(
                                (if (s as ::core::ffi::c_int) < 128 as ::core::ffi::c_int {
                                    s as ::core::ffi::c_uint
                                } else {
                                    (255 as ::core::ffi::c_uint)
                                        .wrapping_sub(s as ::core::ffi::c_uint)
                                }) as ::core::ffi::c_ulong,
                            ) as size_t as size_t;
                            x = x.wrapping_add(1);
                        }
                    }
                    if type_1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int || sum < smallest {
                        bestType = type_1;
                        smallest = sum;
                    }
                    type_1 = type_1.wrapping_add(1);
                }
                prevline = in_0.offset((y as size_t).wrapping_mul(linebytes) as isize)
                    as *const ::core::ffi::c_uchar;
                *out.offset(
                    (y as size_t).wrapping_mul(linebytes.wrapping_add(1 as size_t)) as isize,
                ) = bestType;
                lodepng_memcpy(
                    out.offset(
                        (y as size_t)
                            .wrapping_mul(linebytes.wrapping_add(1 as size_t))
                            .wrapping_add(1 as size_t) as isize,
                    ) as *mut ::core::ffi::c_void,
                    attempt[bestType as usize] as *const ::core::ffi::c_void,
                    linebytes,
                );
                y = y.wrapping_add(1);
            }
        }
        type_1 = 0 as ::core::ffi::c_uchar;
        while type_1 as ::core::ffi::c_int != 5 as ::core::ffi::c_int {
            lodepng_free(attempt[type_1 as usize] as *mut ::core::ffi::c_void);
            type_1 = type_1.wrapping_add(1);
        }
    } else if strategy as ::core::ffi::c_uint
        == LFS_ENTROPY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut attempt_0: [*mut ::core::ffi::c_uchar; 5] =
            [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 5];
        let mut bestSum: size_t = 0 as size_t;
        let mut type_2: ::core::ffi::c_uint = 0;
        let mut bestType_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut count: [::core::ffi::c_uint; 256] = [0; 256];
        type_2 = 0 as ::core::ffi::c_uint;
        while type_2 != 5 as ::core::ffi::c_uint {
            attempt_0[type_2 as usize] = lodepng_malloc(linebytes) as *mut ::core::ffi::c_uchar;
            if attempt_0[type_2 as usize].is_null() {
                error = 83 as ::core::ffi::c_uint;
            }
            type_2 = type_2.wrapping_add(1);
        }
        if error == 0 {
            y = 0 as ::core::ffi::c_uint;
            while y != h {
                type_2 = 0 as ::core::ffi::c_uint;
                while type_2 != 5 as ::core::ffi::c_uint {
                    let mut sum_0: size_t = 0 as size_t;
                    filterScanline(
                        attempt_0[type_2 as usize],
                        in_0.offset((y as size_t).wrapping_mul(linebytes) as isize)
                            as *const ::core::ffi::c_uchar,
                        prevline,
                        linebytes,
                        bytewidth,
                        type_2 as ::core::ffi::c_uchar,
                    );
                    lodepng_memset(
                        &raw mut count as *mut ::core::ffi::c_uint as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (256 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
                    );
                    x = 0 as ::core::ffi::c_uint;
                    while x as size_t != linebytes {
                        count[*attempt_0[type_2 as usize].offset(x as isize) as usize] = count
                            [*attempt_0[type_2 as usize].offset(x as isize) as usize]
                            .wrapping_add(1);
                        x = x.wrapping_add(1);
                    }
                    count[type_2 as usize] = count[type_2 as usize].wrapping_add(1);
                    x = 0 as ::core::ffi::c_uint;
                    while x != 256 as ::core::ffi::c_uint {
                        sum_0 = (sum_0 as ::core::ffi::c_ulong)
                            .wrapping_add(
                                ilog2i(count[x as usize] as size_t) as ::core::ffi::c_ulong
                            ) as size_t as size_t;
                        x = x.wrapping_add(1);
                    }
                    if type_2 == 0 as ::core::ffi::c_uint || sum_0 > bestSum {
                        bestType_0 = type_2;
                        bestSum = sum_0;
                    }
                    type_2 = type_2.wrapping_add(1);
                }
                prevline = in_0.offset((y as size_t).wrapping_mul(linebytes) as isize)
                    as *const ::core::ffi::c_uchar;
                *out.offset(
                    (y as size_t).wrapping_mul(linebytes.wrapping_add(1 as size_t)) as isize,
                ) = bestType_0 as ::core::ffi::c_uchar;
                lodepng_memcpy(
                    out.offset(
                        (y as size_t)
                            .wrapping_mul(linebytes.wrapping_add(1 as size_t))
                            .wrapping_add(1 as size_t) as isize,
                    ) as *mut ::core::ffi::c_void,
                    attempt_0[bestType_0 as usize] as *const ::core::ffi::c_void,
                    linebytes,
                );
                y = y.wrapping_add(1);
            }
        }
        type_2 = 0 as ::core::ffi::c_uint;
        while type_2 != 5 as ::core::ffi::c_uint {
            lodepng_free(attempt_0[type_2 as usize] as *mut ::core::ffi::c_void);
            type_2 = type_2.wrapping_add(1);
        }
    } else if strategy as ::core::ffi::c_uint
        == LFS_PREDEFINED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        y = 0 as ::core::ffi::c_uint;
        while y != h {
            let mut outindex_0: size_t = (1 as size_t)
                .wrapping_add(linebytes)
                .wrapping_mul(y as size_t);
            let mut inindex_0: size_t = linebytes.wrapping_mul(y as size_t);
            let mut type_3: ::core::ffi::c_uchar =
                *(*settings).predefined_filters.offset(y as isize);
            *out.offset(outindex_0 as isize) = type_3;
            filterScanline(
                out.offset(outindex_0.wrapping_add(1 as size_t) as isize)
                    as *mut ::core::ffi::c_uchar,
                in_0.offset(inindex_0 as isize) as *const ::core::ffi::c_uchar,
                prevline,
                linebytes,
                bytewidth,
                type_3,
            );
            prevline = in_0.offset(inindex_0 as isize) as *const ::core::ffi::c_uchar;
            y = y.wrapping_add(1);
        }
    } else if strategy as ::core::ffi::c_uint
        == LFS_BRUTE_FORCE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut size: [size_t; 5] = [0; 5];
        let mut attempt_1: [*mut ::core::ffi::c_uchar; 5] =
            [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 5];
        let mut smallest_0: size_t = 0 as size_t;
        let mut type_4: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut bestType_1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut dummy: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut zlibsettings: LodePNGCompressSettings = LodePNGCompressSettings {
            btype: 0,
            use_lz77: 0,
            windowsize: 0,
            minmatch: 0,
            nicematch: 0,
            lazymatching: 0,
            custom_zlib: None,
            custom_deflate: None,
            custom_context: ::core::ptr::null::<::core::ffi::c_void>(),
        };
        lodepng_memcpy(
            &raw mut zlibsettings as *mut ::core::ffi::c_void,
            &raw const (*settings).zlibsettings as *const ::core::ffi::c_void,
            ::core::mem::size_of::<LodePNGCompressSettings>() as size_t,
        );
        zlibsettings.btype = 1 as ::core::ffi::c_uint;
        zlibsettings.custom_zlib = None;
        zlibsettings.custom_deflate = None;
        type_4 = 0 as ::core::ffi::c_uint;
        while type_4 != 5 as ::core::ffi::c_uint {
            attempt_1[type_4 as usize] = lodepng_malloc(linebytes) as *mut ::core::ffi::c_uchar;
            if attempt_1[type_4 as usize].is_null() {
                error = 83 as ::core::ffi::c_uint;
            }
            type_4 = type_4.wrapping_add(1);
        }
        if error == 0 {
            y = 0 as ::core::ffi::c_uint;
            while y != h {
                type_4 = 0 as ::core::ffi::c_uint;
                while type_4 != 5 as ::core::ffi::c_uint {
                    let mut testsize: ::core::ffi::c_uint = linebytes as ::core::ffi::c_uint;
                    filterScanline(
                        attempt_1[type_4 as usize],
                        in_0.offset((y as size_t).wrapping_mul(linebytes) as isize)
                            as *const ::core::ffi::c_uchar,
                        prevline,
                        linebytes,
                        bytewidth,
                        type_4 as ::core::ffi::c_uchar,
                    );
                    size[type_4 as usize] = 0 as size_t;
                    dummy = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    zlib_compress(
                        &raw mut dummy,
                        (&raw mut size as *mut size_t).offset(type_4 as isize) as *mut size_t,
                        attempt_1[type_4 as usize],
                        testsize as size_t,
                        &raw mut zlibsettings,
                    );
                    lodepng_free(dummy as *mut ::core::ffi::c_void);
                    if type_4 == 0 as ::core::ffi::c_uint || size[type_4 as usize] < smallest_0 {
                        bestType_1 = type_4;
                        smallest_0 = size[type_4 as usize];
                    }
                    type_4 = type_4.wrapping_add(1);
                }
                prevline = in_0.offset((y as size_t).wrapping_mul(linebytes) as isize)
                    as *const ::core::ffi::c_uchar;
                *out.offset(
                    (y as size_t).wrapping_mul(linebytes.wrapping_add(1 as size_t)) as isize,
                ) = bestType_1 as ::core::ffi::c_uchar;
                lodepng_memcpy(
                    out.offset(
                        (y as size_t)
                            .wrapping_mul(linebytes.wrapping_add(1 as size_t))
                            .wrapping_add(1 as size_t) as isize,
                    ) as *mut ::core::ffi::c_void,
                    attempt_1[bestType_1 as usize] as *const ::core::ffi::c_void,
                    linebytes,
                );
                y = y.wrapping_add(1);
            }
        }
        type_4 = 0 as ::core::ffi::c_uint;
        while type_4 != 5 as ::core::ffi::c_uint {
            lodepng_free(attempt_1[type_4 as usize] as *mut ::core::ffi::c_void);
            type_4 = type_4.wrapping_add(1);
        }
    } else {
        return 88 as ::core::ffi::c_uint;
    }
    return error;
}
unsafe extern "C" fn addPaddingBits(
    mut out: *mut ::core::ffi::c_uchar,
    mut in_0: *const ::core::ffi::c_uchar,
    mut olinebits: size_t,
    mut ilinebits: size_t,
    mut h: ::core::ffi::c_uint,
) {
    let mut y: ::core::ffi::c_uint = 0;
    let mut diff: size_t = olinebits.wrapping_sub(ilinebits);
    let mut obp: size_t = 0 as size_t;
    let mut ibp: size_t = 0 as size_t;
    y = 0 as ::core::ffi::c_uint;
    while y != h {
        let mut x: size_t = 0;
        x = 0 as size_t;
        while x < ilinebits {
            let mut bit: ::core::ffi::c_uchar = readBitFromReversedStream(&raw mut ibp, in_0);
            setBitOfReversedStream(&raw mut obp, out, bit);
            x = x.wrapping_add(1);
        }
        x = 0 as size_t;
        while x != diff {
            setBitOfReversedStream(&raw mut obp, out, 0 as ::core::ffi::c_uchar);
            x = x.wrapping_add(1);
        }
        y = y.wrapping_add(1);
    }
}
unsafe extern "C" fn Adam7_interlace(
    mut out: *mut ::core::ffi::c_uchar,
    mut in_0: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut bpp: ::core::ffi::c_uint,
) {
    let mut passw: [::core::ffi::c_uint; 7] = [0; 7];
    let mut passh: [::core::ffi::c_uint; 7] = [0; 7];
    let mut filter_passstart: [size_t; 8] = [0; 8];
    let mut padded_passstart: [size_t; 8] = [0; 8];
    let mut passstart: [size_t; 8] = [0; 8];
    let mut i: ::core::ffi::c_uint = 0;
    Adam7_getpassvalues(
        &raw mut passw as *mut ::core::ffi::c_uint,
        &raw mut passh as *mut ::core::ffi::c_uint,
        &raw mut filter_passstart as *mut size_t,
        &raw mut padded_passstart as *mut size_t,
        &raw mut passstart as *mut size_t,
        w,
        h,
        bpp,
    );
    if bpp >= 8 as ::core::ffi::c_uint {
        i = 0 as ::core::ffi::c_uint;
        while i != 7 as ::core::ffi::c_uint {
            let mut x: ::core::ffi::c_uint = 0;
            let mut y: ::core::ffi::c_uint = 0;
            let mut b: ::core::ffi::c_uint = 0;
            let mut bytewidth: size_t = bpp.wrapping_div(8 as ::core::ffi::c_uint) as size_t;
            y = 0 as ::core::ffi::c_uint;
            while y < passh[i as usize] {
                x = 0 as ::core::ffi::c_uint;
                while x < passw[i as usize] {
                    let mut pixelinstart: size_t = (ADAM7_IY[i as usize]
                        .wrapping_add(y.wrapping_mul(ADAM7_DY[i as usize]))
                        .wrapping_mul(w)
                        .wrapping_add(ADAM7_IX[i as usize])
                        .wrapping_add(x.wrapping_mul(ADAM7_DX[i as usize]))
                        as size_t)
                        .wrapping_mul(bytewidth);
                    let mut pixeloutstart: size_t = passstart[i as usize].wrapping_add(
                        (y.wrapping_mul(passw[i as usize]).wrapping_add(x) as size_t)
                            .wrapping_mul(bytewidth),
                    );
                    b = 0 as ::core::ffi::c_uint;
                    while (b as size_t) < bytewidth {
                        *out.offset(pixeloutstart.wrapping_add(b as size_t) as isize) =
                            *in_0.offset(pixelinstart.wrapping_add(b as size_t) as isize);
                        b = b.wrapping_add(1);
                    }
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as ::core::ffi::c_uint;
        while i != 7 as ::core::ffi::c_uint {
            let mut x_0: ::core::ffi::c_uint = 0;
            let mut y_0: ::core::ffi::c_uint = 0;
            let mut b_0: ::core::ffi::c_uint = 0;
            let mut ilinebits: ::core::ffi::c_uint = bpp.wrapping_mul(passw[i as usize]);
            let mut olinebits: ::core::ffi::c_uint = bpp.wrapping_mul(w);
            let mut obp: size_t = 0;
            let mut ibp: size_t = 0;
            y_0 = 0 as ::core::ffi::c_uint;
            while y_0 < passh[i as usize] {
                x_0 = 0 as ::core::ffi::c_uint;
                while x_0 < passw[i as usize] {
                    ibp = ADAM7_IY[i as usize]
                        .wrapping_add(y_0.wrapping_mul(ADAM7_DY[i as usize]))
                        .wrapping_mul(olinebits)
                        .wrapping_add(
                            ADAM7_IX[i as usize]
                                .wrapping_add(x_0.wrapping_mul(ADAM7_DX[i as usize]))
                                .wrapping_mul(bpp),
                        ) as size_t;
                    obp = (8 as size_t)
                        .wrapping_mul(passstart[i as usize])
                        .wrapping_add(
                            y_0.wrapping_mul(ilinebits)
                                .wrapping_add(x_0.wrapping_mul(bpp))
                                as size_t,
                        );
                    b_0 = 0 as ::core::ffi::c_uint;
                    while b_0 < bpp {
                        let mut bit: ::core::ffi::c_uchar =
                            readBitFromReversedStream(&raw mut ibp, in_0);
                        setBitOfReversedStream(&raw mut obp, out, bit);
                        b_0 = b_0.wrapping_add(1);
                    }
                    x_0 = x_0.wrapping_add(1);
                }
                y_0 = y_0.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn preProcessScanlines(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut in_0: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut info_png: *const LodePNGInfo,
    mut settings: *const LodePNGEncoderSettings,
) -> ::core::ffi::c_uint {
    let mut bpp: size_t = lodepng_get_bpp(&raw const (*info_png).color) as size_t;
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if (*info_png).interlace_method == 0 as ::core::ffi::c_uint {
        *outsize = (h as size_t).wrapping_add(
            (h as size_t).wrapping_mul(
                (w as size_t)
                    .wrapping_mul(bpp)
                    .wrapping_add(7 as size_t)
                    .wrapping_div(8 as size_t),
            ),
        );
        *out = lodepng_malloc(*outsize) as *mut ::core::ffi::c_uchar;
        if (*out).is_null() && *outsize != 0 {
            error = 83 as ::core::ffi::c_uint;
        }
        if error == 0 {
            if bpp < 8 as size_t
                && (w as size_t).wrapping_mul(bpp)
                    != (w as size_t)
                        .wrapping_mul(bpp)
                        .wrapping_add(7 as size_t)
                        .wrapping_div(8 as size_t)
                        .wrapping_mul(8 as size_t)
            {
                let mut padded: *mut ::core::ffi::c_uchar = lodepng_malloc(
                    (h as size_t).wrapping_mul(
                        (w as size_t)
                            .wrapping_mul(bpp)
                            .wrapping_add(7 as size_t)
                            .wrapping_div(8 as size_t),
                    ),
                )
                    as *mut ::core::ffi::c_uchar;
                if padded.is_null() {
                    error = 83 as ::core::ffi::c_uint;
                }
                if error == 0 {
                    addPaddingBits(
                        padded,
                        in_0,
                        (w as size_t)
                            .wrapping_mul(bpp)
                            .wrapping_add(7 as size_t)
                            .wrapping_div(8 as size_t)
                            .wrapping_mul(8 as size_t),
                        (w as size_t).wrapping_mul(bpp),
                        h,
                    );
                    error = filter(*out, padded, w, h, &raw const (*info_png).color, settings);
                }
                lodepng_free(padded as *mut ::core::ffi::c_void);
            } else {
                error = filter(*out, in_0, w, h, &raw const (*info_png).color, settings);
            }
        }
    } else {
        let mut passw: [::core::ffi::c_uint; 7] = [0; 7];
        let mut passh: [::core::ffi::c_uint; 7] = [0; 7];
        let mut filter_passstart: [size_t; 8] = [0; 8];
        let mut padded_passstart: [size_t; 8] = [0; 8];
        let mut passstart: [size_t; 8] = [0; 8];
        let mut adam7: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        Adam7_getpassvalues(
            &raw mut passw as *mut ::core::ffi::c_uint,
            &raw mut passh as *mut ::core::ffi::c_uint,
            &raw mut filter_passstart as *mut size_t,
            &raw mut padded_passstart as *mut size_t,
            &raw mut passstart as *mut size_t,
            w,
            h,
            bpp as ::core::ffi::c_uint,
        );
        *outsize = filter_passstart[7 as ::core::ffi::c_int as usize];
        *out = lodepng_malloc(*outsize) as *mut ::core::ffi::c_uchar;
        if (*out).is_null() {
            error = 83 as ::core::ffi::c_uint;
        }
        adam7 = lodepng_malloc(passstart[7 as ::core::ffi::c_int as usize])
            as *mut ::core::ffi::c_uchar;
        if adam7.is_null() && passstart[7 as ::core::ffi::c_int as usize] != 0 {
            error = 83 as ::core::ffi::c_uint;
        }
        if error == 0 {
            let mut i: ::core::ffi::c_uint = 0;
            Adam7_interlace(adam7, in_0, w, h, bpp as ::core::ffi::c_uint);
            i = 0 as ::core::ffi::c_uint;
            while i != 7 as ::core::ffi::c_uint {
                if bpp < 8 as size_t {
                    let mut padded_0: *mut ::core::ffi::c_uchar = lodepng_malloc(
                        padded_passstart[i.wrapping_add(1 as ::core::ffi::c_uint) as usize]
                            .wrapping_sub(padded_passstart[i as usize]),
                    )
                        as *mut ::core::ffi::c_uchar;
                    if padded_0.is_null() {
                        error = 83 as ::core::ffi::c_uint;
                        break;
                    } else {
                        addPaddingBits(
                            padded_0,
                            adam7
                                .offset(*(&raw mut passstart as *mut size_t).offset(i as isize)
                                    as isize)
                                as *mut ::core::ffi::c_uchar,
                            (passw[i as usize] as size_t)
                                .wrapping_mul(bpp)
                                .wrapping_add(7 as size_t)
                                .wrapping_div(8 as size_t)
                                .wrapping_mul(8 as size_t),
                            (passw[i as usize] as size_t).wrapping_mul(bpp),
                            passh[i as usize],
                        );
                        error = filter(
                            (*out).offset(
                                *(&raw mut filter_passstart as *mut size_t).offset(i as isize)
                                    as isize,
                            ) as *mut ::core::ffi::c_uchar,
                            padded_0,
                            passw[i as usize],
                            passh[i as usize],
                            &raw const (*info_png).color,
                            settings,
                        );
                        lodepng_free(padded_0 as *mut ::core::ffi::c_void);
                    }
                } else {
                    error = filter(
                        (*out).offset(
                            *(&raw mut filter_passstart as *mut size_t).offset(i as isize) as isize,
                        ) as *mut ::core::ffi::c_uchar,
                        adam7.offset(
                            *(&raw mut padded_passstart as *mut size_t).offset(i as isize) as isize,
                        ) as *mut ::core::ffi::c_uchar,
                        passw[i as usize],
                        passh[i as usize],
                        &raw const (*info_png).color,
                        settings,
                    );
                }
                if error != 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        lodepng_free(adam7 as *mut ::core::ffi::c_void);
    }
    return error;
}
unsafe extern "C" fn addUnknownChunks(
    mut out: *mut ucvector,
    mut data: *mut ::core::ffi::c_uchar,
    mut datasize: size_t,
) -> ::core::ffi::c_uint {
    let mut inchunk: *mut ::core::ffi::c_uchar = data;
    while (inchunk.offset_from(data) as ::core::ffi::c_long as size_t) < datasize {
        let mut error_: ::core::ffi::c_uint =
            lodepng_chunk_append(&raw mut (*out).data, &raw mut (*out).size, inchunk);
        if error_ != 0 {
            return error_;
        }
        (*out).allocsize = (*out).size;
        inchunk = lodepng_chunk_next(inchunk, data.offset(datasize as isize));
    }
    return 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn isGrayICCProfile(
    mut profile: *const ::core::ffi::c_uchar,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if size < 20 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_uint;
    }
    return (*profile.offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'G' as i32
        && *profile.offset(17 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'R' as i32
        && *profile.offset(18 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'A' as i32
        && *profile.offset(19 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'Y' as i32)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
}
unsafe extern "C" fn isRGBICCProfile(
    mut profile: *const ::core::ffi::c_uchar,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if size < 20 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_uint;
    }
    return (*profile.offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'R' as i32
        && *profile.offset(17 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'G' as i32
        && *profile.offset(18 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'B' as i32
        && *profile.offset(19 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut image: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut state: *mut LodePNGState,
) -> ::core::ffi::c_uint {
    let mut current_block: u64;
    let mut data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut datasize: size_t = 0 as size_t;
    let mut outv: ucvector =
        ucvector_init(::core::ptr::null_mut::<::core::ffi::c_uchar>(), 0 as size_t);
    let mut info: LodePNGInfo = LodePNGInfo {
        compression_method: 0,
        filter_method: 0,
        interlace_method: 0,
        color: LodePNGColorMode {
            colortype: LCT_GREY,
            bitdepth: 0,
            palette: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            palettesize: 0,
            key_defined: 0,
            key_r: 0,
            key_g: 0,
            key_b: 0,
        },
        background_defined: 0,
        background_r: 0,
        background_g: 0,
        background_b: 0,
        text_num: 0,
        text_keys: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        text_strings: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        itext_num: 0,
        itext_keys: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        itext_langtags: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        itext_transkeys: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        itext_strings: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        exif_defined: 0,
        exif: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        exif_size: 0,
        time_defined: 0,
        time: LodePNGTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
        },
        phys_defined: 0,
        phys_x: 0,
        phys_y: 0,
        phys_unit: 0,
        gama_defined: 0,
        gama_gamma: 0,
        chrm_defined: 0,
        chrm_white_x: 0,
        chrm_white_y: 0,
        chrm_red_x: 0,
        chrm_red_y: 0,
        chrm_green_x: 0,
        chrm_green_y: 0,
        chrm_blue_x: 0,
        chrm_blue_y: 0,
        srgb_defined: 0,
        srgb_intent: 0,
        iccp_defined: 0,
        iccp_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        iccp_profile: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        iccp_profile_size: 0,
        cicp_defined: 0,
        cicp_color_primaries: 0,
        cicp_transfer_function: 0,
        cicp_matrix_coefficients: 0,
        cicp_video_full_range_flag: 0,
        mdcv_defined: 0,
        mdcv_red_x: 0,
        mdcv_red_y: 0,
        mdcv_green_x: 0,
        mdcv_green_y: 0,
        mdcv_blue_x: 0,
        mdcv_blue_y: 0,
        mdcv_white_x: 0,
        mdcv_white_y: 0,
        mdcv_max_luminance: 0,
        mdcv_min_luminance: 0,
        clli_defined: 0,
        clli_max_cll: 0,
        clli_max_fall: 0,
        sbit_defined: 0,
        sbit_r: 0,
        sbit_g: 0,
        sbit_b: 0,
        sbit_a: 0,
        unknown_chunks_data: [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3],
        unknown_chunks_size: [0; 3],
    };
    let mut info_png: *const LodePNGInfo = &raw mut (*state).info_png;
    let mut auto_color: LodePNGColorMode = LodePNGColorMode {
        colortype: LCT_GREY,
        bitdepth: 0,
        palette: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        palettesize: 0,
        key_defined: 0,
        key_r: 0,
        key_g: 0,
        key_b: 0,
    };
    let mut error: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    lodepng_info_init(&raw mut info);
    lodepng_color_mode_init(&raw mut auto_color);
    *out = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    *outsize = 0 as size_t;
    if ((*info_png).color.colortype as ::core::ffi::c_uint
        == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*state).encoder.force_palette != 0)
        && ((*info_png).color.palettesize == 0 as size_t
            || (*info_png).color.palettesize > 256 as size_t)
    {
        error = 68 as ::core::ffi::c_uint;
    } else if (*state).encoder.zlibsettings.btype > 2 as ::core::ffi::c_uint {
        error = 61 as ::core::ffi::c_uint;
    } else if (*info_png).interlace_method > 1 as ::core::ffi::c_uint {
        error = 71 as ::core::ffi::c_uint;
    } else {
        error = checkColorValidity((*info_png).color.colortype, (*info_png).color.bitdepth);
        if !(error != 0) {
            error = checkColorValidity((*state).info_raw.colortype, (*state).info_raw.bitdepth);
            if !(error != 0) {
                let mut error_: ::core::ffi::c_uint =
                    lodepng_info_copy(&raw mut info, &raw mut (*state).info_png);
                if error_ != 0 {
                    return error_;
                }
                if (*state).encoder.auto_convert != 0 {
                    let mut stats: LodePNGColorStats = LodePNGColorStats {
                        colored: 0,
                        key: 0,
                        key_r: 0,
                        key_g: 0,
                        key_b: 0,
                        alpha: 0,
                        numcolors: 0,
                        palette: [0; 1024],
                        bits: 0,
                        numpixels: 0,
                        allow_palette: 0,
                        allow_greyscale: 0,
                    };
                    let mut allow_convert: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
                    lodepng_color_stats_init(&raw mut stats);
                    if (*info_png).iccp_defined != 0
                        && isGrayICCProfile((*info_png).iccp_profile, (*info_png).iccp_profile_size)
                            != 0
                    {
                        stats.allow_palette = 0 as ::core::ffi::c_uint;
                    }
                    if (*info_png).iccp_defined != 0
                        && isRGBICCProfile((*info_png).iccp_profile, (*info_png).iccp_profile_size)
                            != 0
                    {
                        stats.allow_greyscale = 0 as ::core::ffi::c_uint;
                    }
                    error = lodepng_compute_color_stats(
                        &raw mut stats,
                        image,
                        w,
                        h,
                        &raw mut (*state).info_raw,
                    );
                    if error != 0 {
                        current_block = 11067230341960327308;
                    } else {
                        if (*info_png).background_defined != 0 {
                            let mut r: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                            let mut g: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                            let mut b: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                            let mut mode16: LodePNGColorMode =
                                lodepng_color_mode_make(LCT_RGB, 16 as ::core::ffi::c_uint);
                            lodepng_convert_rgb(
                                &raw mut r,
                                &raw mut g,
                                &raw mut b,
                                (*info_png).background_r,
                                (*info_png).background_g,
                                (*info_png).background_b,
                                &raw mut mode16,
                                &raw const (*info_png).color,
                            );
                            error = lodepng_color_stats_add(
                                &raw mut stats,
                                r,
                                g,
                                b,
                                65535 as ::core::ffi::c_uint,
                            );
                            if error != 0 {
                                current_block = 11067230341960327308;
                            } else {
                                current_block = 11932355480408055363;
                            }
                        } else {
                            current_block = 11932355480408055363;
                        }
                        match current_block {
                            11067230341960327308 => {}
                            _ => {
                                error = auto_choose_color(
                                    &raw mut auto_color,
                                    &raw mut (*state).info_raw,
                                    &raw mut stats,
                                );
                                if error != 0 {
                                    current_block = 11067230341960327308;
                                } else {
                                    if (*info_png).sbit_defined != 0 {
                                        let mut sbit_max: ::core::ffi::c_uint =
                                            if (if (if (*info_png).sbit_r > (*info_png).sbit_g {
                                                (*info_png).sbit_r
                                            } else {
                                                (*info_png).sbit_g
                                            }) > (*info_png).sbit_b
                                            {
                                                (if (*info_png).sbit_r > (*info_png).sbit_g {
                                                    (*info_png).sbit_r
                                                } else {
                                                    (*info_png).sbit_g
                                                })
                                            } else {
                                                (*info_png).sbit_b
                                            }) > (*info_png).sbit_a
                                            {
                                                if (if (*info_png).sbit_r > (*info_png).sbit_g {
                                                    (*info_png).sbit_r
                                                } else {
                                                    (*info_png).sbit_g
                                                }) > (*info_png).sbit_b
                                                {
                                                    if (*info_png).sbit_r > (*info_png).sbit_g {
                                                        (*info_png).sbit_r
                                                    } else {
                                                        (*info_png).sbit_g
                                                    }
                                                } else {
                                                    (*info_png).sbit_b
                                                }
                                            } else {
                                                (*info_png).sbit_a
                                            };
                                        let mut equal: ::core::ffi::c_uint = (((*info_png).sbit_g
                                            == 0
                                            || (*info_png).sbit_g == (*info_png).sbit_r)
                                            && ((*info_png).sbit_b == 0
                                                || (*info_png).sbit_b == (*info_png).sbit_r)
                                            && ((*info_png).sbit_a == 0
                                                || (*info_png).sbit_a == (*info_png).sbit_r))
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uint;
                                        allow_convert = 0 as ::core::ffi::c_uint;
                                        if info.color.colortype as ::core::ffi::c_uint
                                            == LCT_PALETTE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                            && auto_color.colortype as ::core::ffi::c_uint
                                                == LCT_PALETTE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                        {
                                            allow_convert = 1 as ::core::ffi::c_uint;
                                        }
                                        if info.color.colortype as ::core::ffi::c_uint
                                            == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                                            && auto_color.colortype as ::core::ffi::c_uint
                                                == LCT_PALETTE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            && sbit_max <= 8 as ::core::ffi::c_uint
                                        {
                                            allow_convert = 1 as ::core::ffi::c_uint;
                                        }
                                        if info.color.colortype as ::core::ffi::c_uint
                                            == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
                                            && auto_color.colortype as ::core::ffi::c_uint
                                                == LCT_PALETTE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            && (*info_png).sbit_a == 8 as ::core::ffi::c_uint
                                            && sbit_max <= 8 as ::core::ffi::c_uint
                                        {
                                            allow_convert = 1 as ::core::ffi::c_uint;
                                        }
                                        if (info.color.colortype as ::core::ffi::c_uint
                                            == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                                            || info.color.colortype as ::core::ffi::c_uint
                                                == LCT_RGBA as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint)
                                            && info.color.bitdepth == 16 as ::core::ffi::c_uint
                                            && auto_color.colortype as ::core::ffi::c_uint
                                                == info.color.colortype as ::core::ffi::c_uint
                                            && auto_color.bitdepth == 8 as ::core::ffi::c_uint
                                            && sbit_max <= 8 as ::core::ffi::c_uint
                                        {
                                            allow_convert = 1 as ::core::ffi::c_uint;
                                        }
                                        if info.color.colortype as ::core::ffi::c_uint
                                            != LCT_PALETTE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                            && auto_color.colortype as ::core::ffi::c_uint
                                                != LCT_PALETTE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            && equal != 0
                                            && (*info_png).sbit_r == auto_color.bitdepth
                                        {
                                            allow_convert = 1 as ::core::ffi::c_uint;
                                        }
                                    }
                                    if (*state).encoder.force_palette != 0 {
                                        if info.color.colortype as ::core::ffi::c_uint
                                            != LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
                                            && info.color.colortype as ::core::ffi::c_uint
                                                != LCT_GREY_ALPHA as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            && (auto_color.colortype as ::core::ffi::c_uint
                                                == LCT_GREY as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                                || auto_color.colortype as ::core::ffi::c_uint
                                                    == LCT_GREY_ALPHA as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint)
                                        {
                                            allow_convert = 0 as ::core::ffi::c_uint;
                                        }
                                    }
                                    if allow_convert != 0 {
                                        lodepng_color_mode_copy(
                                            &raw mut info.color,
                                            &raw mut auto_color,
                                        );
                                        if (*info_png).background_defined != 0 {
                                            if lodepng_convert_rgb(
                                                &raw mut info.background_r,
                                                &raw mut info.background_g,
                                                &raw mut info.background_b,
                                                (*info_png).background_r,
                                                (*info_png).background_g,
                                                (*info_png).background_b,
                                                &raw mut info.color,
                                                &raw const (*info_png).color,
                                            ) != 0
                                            {
                                                error = 104 as ::core::ffi::c_uint;
                                                current_block = 11067230341960327308;
                                            } else {
                                                current_block = 5235537862154438448;
                                            }
                                        } else {
                                            current_block = 5235537862154438448;
                                        }
                                    } else {
                                        current_block = 5235537862154438448;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    current_block = 5235537862154438448;
                }
                match current_block {
                    11067230341960327308 => {}
                    _ => {
                        if (*info_png).iccp_defined != 0 {
                            let mut gray_icc: ::core::ffi::c_uint = isGrayICCProfile(
                                (*info_png).iccp_profile,
                                (*info_png).iccp_profile_size,
                            );
                            let mut rgb_icc: ::core::ffi::c_uint = isRGBICCProfile(
                                (*info_png).iccp_profile,
                                (*info_png).iccp_profile_size,
                            );
                            let mut gray_png: ::core::ffi::c_uint = (info.color.colortype
                                as ::core::ffi::c_uint
                                == LCT_GREY as ::core::ffi::c_int as ::core::ffi::c_uint
                                || info.color.colortype as ::core::ffi::c_uint
                                    == LCT_GREY_ALPHA as ::core::ffi::c_int as ::core::ffi::c_uint)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint;
                            if gray_icc == 0 && rgb_icc == 0 {
                                error = 100 as ::core::ffi::c_uint;
                                current_block = 11067230341960327308;
                            } else if gray_icc != gray_png {
                                error = (if (*state).encoder.auto_convert != 0 {
                                    102 as ::core::ffi::c_int
                                } else {
                                    101 as ::core::ffi::c_int
                                }) as ::core::ffi::c_uint;
                                current_block = 11067230341960327308;
                            } else {
                                current_block = 3580086814630675314;
                            }
                        } else {
                            current_block = 3580086814630675314;
                        }
                        match current_block {
                            11067230341960327308 => {}
                            _ => {
                                if lodepng_color_mode_equal(
                                    &raw mut (*state).info_raw,
                                    &raw mut info.color,
                                ) == 0
                                {
                                    let mut converted: *mut ::core::ffi::c_uchar =
                                        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    let mut size: size_t =
                                        (w as size_t)
                                            .wrapping_mul(h as size_t)
                                            .wrapping_mul(
                                                lodepng_get_bpp(&raw mut info.color) as size_t
                                            )
                                            .wrapping_add(7 as size_t)
                                            .wrapping_div(8 as size_t);
                                    converted = lodepng_malloc(size) as *mut ::core::ffi::c_uchar;
                                    if converted.is_null() && size != 0 {
                                        error = 83 as ::core::ffi::c_uint;
                                    }
                                    if error == 0 {
                                        error = lodepng_convert(
                                            converted,
                                            image,
                                            &raw mut info.color,
                                            &raw mut (*state).info_raw,
                                            w,
                                            h,
                                        );
                                    }
                                    if error == 0 {
                                        error = preProcessScanlines(
                                            &raw mut data,
                                            &raw mut datasize,
                                            converted,
                                            w,
                                            h,
                                            &raw mut info,
                                            &raw mut (*state).encoder,
                                        );
                                    }
                                    lodepng_free(converted as *mut ::core::ffi::c_void);
                                    if error != 0 {
                                        current_block = 11067230341960327308;
                                    } else {
                                        current_block = 5854763015135596753;
                                    }
                                } else {
                                    error = preProcessScanlines(
                                        &raw mut data,
                                        &raw mut datasize,
                                        image,
                                        w,
                                        h,
                                        &raw mut info,
                                        &raw mut (*state).encoder,
                                    );
                                    if error != 0 {
                                        current_block = 11067230341960327308;
                                    } else {
                                        current_block = 5854763015135596753;
                                    }
                                }
                                match current_block {
                                    11067230341960327308 => {}
                                    _ => {
                                        let mut i: size_t = 0;
                                        error = writeSignature(&raw mut outv);
                                        if !(error != 0) {
                                            error = addChunk_IHDR(
                                                &raw mut outv,
                                                w,
                                                h,
                                                info.color.colortype,
                                                info.color.bitdepth,
                                                info.interlace_method,
                                            );
                                            if !(error != 0) {
                                                if !info.unknown_chunks_data
                                                    [0 as ::core::ffi::c_int as usize]
                                                    .is_null()
                                                {
                                                    error = addUnknownChunks(
                                                        &raw mut outv,
                                                        info.unknown_chunks_data
                                                            [0 as ::core::ffi::c_int as usize],
                                                        info.unknown_chunks_size
                                                            [0 as ::core::ffi::c_int as usize],
                                                    );
                                                    if error != 0 {
                                                        current_block = 11067230341960327308;
                                                    } else {
                                                        current_block = 9879896046554623444;
                                                    }
                                                } else {
                                                    current_block = 9879896046554623444;
                                                }
                                                match current_block {
                                                    11067230341960327308 => {}
                                                    _ => {
                                                        if info.cicp_defined != 0 {
                                                            error = addChunk_cICP(
                                                                &raw mut outv,
                                                                &raw mut info,
                                                            );
                                                            if error != 0 {
                                                                current_block =
                                                                    11067230341960327308;
                                                            } else {
                                                                current_block =
                                                                    10393716428851982524;
                                                            }
                                                        } else {
                                                            current_block = 10393716428851982524;
                                                        }
                                                        match current_block {
                                                            11067230341960327308 => {}
                                                            _ => {
                                                                if info.mdcv_defined != 0 {
                                                                    error = addChunk_mDCV(
                                                                        &raw mut outv,
                                                                        &raw mut info,
                                                                    );
                                                                    if error != 0 {
                                                                        current_block =
                                                                            11067230341960327308;
                                                                    } else {
                                                                        current_block =
                                                                            12070711452894729854;
                                                                    }
                                                                } else {
                                                                    current_block =
                                                                        12070711452894729854;
                                                                }
                                                                match current_block {
                                                                    11067230341960327308 => {}
                                                                    _ => {
                                                                        if info.clli_defined != 0 {
                                                                            error = addChunk_cLLI(
                                                                                &raw mut outv,
                                                                                &raw mut info,
                                                                            );
                                                                            if error != 0 {
                                                                                current_block = 11067230341960327308;
                                                                            } else {
                                                                                current_block = 11071260907632769126;
                                                                            }
                                                                        } else {
                                                                            current_block = 11071260907632769126;
                                                                        }
                                                                        match current_block {
                                                                            11067230341960327308 => {}
                                                                            _ => {
                                                                                if info.iccp_defined != 0 {
                                                                                    error = addChunk_iCCP(
                                                                                        &raw mut outv,
                                                                                        &raw mut info,
                                                                                        &raw mut (*state).encoder.zlibsettings,
                                                                                    );
                                                                                    if error != 0 {
                                                                                        current_block = 11067230341960327308;
                                                                                    } else {
                                                                                        current_block = 18425699056680496821;
                                                                                    }
                                                                                } else {
                                                                                    current_block = 18425699056680496821;
                                                                                }
                                                                                match current_block {
                                                                                    11067230341960327308 => {}
                                                                                    _ => {
                                                                                        if info.srgb_defined != 0 {
                                                                                            error = addChunk_sRGB(&raw mut outv, &raw mut info);
                                                                                            if error != 0 {
                                                                                                current_block = 11067230341960327308;
                                                                                            } else {
                                                                                                current_block = 13824533195664196414;
                                                                                            }
                                                                                        } else {
                                                                                            current_block = 13824533195664196414;
                                                                                        }
                                                                                        match current_block {
                                                                                            11067230341960327308 => {}
                                                                                            _ => {
                                                                                                if info.gama_defined != 0 {
                                                                                                    error = addChunk_gAMA(&raw mut outv, &raw mut info);
                                                                                                    if error != 0 {
                                                                                                        current_block = 11067230341960327308;
                                                                                                    } else {
                                                                                                        current_block = 4691324637564808323;
                                                                                                    }
                                                                                                } else {
                                                                                                    current_block = 4691324637564808323;
                                                                                                }
                                                                                                match current_block {
                                                                                                    11067230341960327308 => {}
                                                                                                    _ => {
                                                                                                        if info.chrm_defined != 0 {
                                                                                                            error = addChunk_cHRM(&raw mut outv, &raw mut info);
                                                                                                            if error != 0 {
                                                                                                                current_block = 11067230341960327308;
                                                                                                            } else {
                                                                                                                current_block = 9521147444787763968;
                                                                                                            }
                                                                                                        } else {
                                                                                                            current_block = 9521147444787763968;
                                                                                                        }
                                                                                                        match current_block {
                                                                                                            11067230341960327308 => {}
                                                                                                            _ => {
                                                                                                                if (*info_png).sbit_defined != 0 {
                                                                                                                    error = addChunk_sBIT(&raw mut outv, &raw mut info);
                                                                                                                    if error != 0 {
                                                                                                                        current_block = 11067230341960327308;
                                                                                                                    } else {
                                                                                                                        current_block = 5697748000427295508;
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    current_block = 5697748000427295508;
                                                                                                                }
                                                                                                                match current_block {
                                                                                                                    11067230341960327308 => {}
                                                                                                                    _ => {
                                                                                                                        if info.exif_defined != 0 {
                                                                                                                            error = addChunk_eXIf(&raw mut outv, &raw mut info);
                                                                                                                            if error != 0 {
                                                                                                                                current_block = 11067230341960327308;
                                                                                                                            } else {
                                                                                                                                current_block = 6733407218104445560;
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            current_block = 6733407218104445560;
                                                                                                                        }
                                                                                                                        match current_block {
                                                                                                                            11067230341960327308 => {}
                                                                                                                            _ => {
                                                                                                                                if info.color.colortype as ::core::ffi::c_uint
                                                                                                                                    == LCT_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                                                {
                                                                                                                                    error = addChunk_PLTE(&raw mut outv, &raw mut info.color);
                                                                                                                                    if error != 0 {
                                                                                                                                        current_block = 11067230341960327308;
                                                                                                                                    } else {
                                                                                                                                        current_block = 12543410360505780601;
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    current_block = 12543410360505780601;
                                                                                                                                }
                                                                                                                                match current_block {
                                                                                                                                    11067230341960327308 => {}
                                                                                                                                    _ => {
                                                                                                                                        if (*state).encoder.force_palette != 0
                                                                                                                                            && (info.color.colortype as ::core::ffi::c_uint
                                                                                                                                                == LCT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                                                                || info.color.colortype as ::core::ffi::c_uint
                                                                                                                                                    == LCT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint)
                                                                                                                                        {
                                                                                                                                            error = addChunk_PLTE(&raw mut outv, &raw mut info.color);
                                                                                                                                            if error != 0 {
                                                                                                                                                current_block = 11067230341960327308;
                                                                                                                                            } else {
                                                                                                                                                current_block = 11718254377427810743;
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            current_block = 11718254377427810743;
                                                                                                                                        }
                                                                                                                                        match current_block {
                                                                                                                                            11067230341960327308 => {}
                                                                                                                                            _ => {
                                                                                                                                                error = addChunk_tRNS(&raw mut outv, &raw mut info.color);
                                                                                                                                                if !(error != 0) {
                                                                                                                                                    if info.background_defined != 0 {
                                                                                                                                                        error = addChunk_bKGD(&raw mut outv, &raw mut info);
                                                                                                                                                        if error != 0 {
                                                                                                                                                            current_block = 11067230341960327308;
                                                                                                                                                        } else {
                                                                                                                                                            current_block = 11162283542402356847;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        current_block = 11162283542402356847;
                                                                                                                                                    }
                                                                                                                                                    match current_block {
                                                                                                                                                        11067230341960327308 => {}
                                                                                                                                                        _ => {
                                                                                                                                                            if info.phys_defined != 0 {
                                                                                                                                                                error = addChunk_pHYs(&raw mut outv, &raw mut info);
                                                                                                                                                                if error != 0 {
                                                                                                                                                                    current_block = 11067230341960327308;
                                                                                                                                                                } else {
                                                                                                                                                                    current_block = 16696653877814833746;
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                current_block = 16696653877814833746;
                                                                                                                                                            }
                                                                                                                                                            match current_block {
                                                                                                                                                                11067230341960327308 => {}
                                                                                                                                                                _ => {
                                                                                                                                                                    if !info
                                                                                                                                                                        .unknown_chunks_data[1 as ::core::ffi::c_int as usize]
                                                                                                                                                                        .is_null()
                                                                                                                                                                    {
                                                                                                                                                                        error = addUnknownChunks(
                                                                                                                                                                            &raw mut outv,
                                                                                                                                                                            info.unknown_chunks_data[1 as ::core::ffi::c_int as usize],
                                                                                                                                                                            info.unknown_chunks_size[1 as ::core::ffi::c_int as usize],
                                                                                                                                                                        );
                                                                                                                                                                        if error != 0 {
                                                                                                                                                                            current_block = 11067230341960327308;
                                                                                                                                                                        } else {
                                                                                                                                                                            current_block = 7079180960716815705;
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        current_block = 7079180960716815705;
                                                                                                                                                                    }
                                                                                                                                                                    match current_block {
                                                                                                                                                                        11067230341960327308 => {}
                                                                                                                                                                        _ => {
                                                                                                                                                                            error = addChunk_IDAT(
                                                                                                                                                                                &raw mut outv,
                                                                                                                                                                                data,
                                                                                                                                                                                datasize,
                                                                                                                                                                                &raw mut (*state).encoder.zlibsettings,
                                                                                                                                                                            );
                                                                                                                                                                            if !(error != 0) {
                                                                                                                                                                                if info.time_defined != 0 {
                                                                                                                                                                                    error = addChunk_tIME(&raw mut outv, &raw mut info.time);
                                                                                                                                                                                    if error != 0 {
                                                                                                                                                                                        current_block = 11067230341960327308;
                                                                                                                                                                                    } else {
                                                                                                                                                                                        current_block = 10945915984064580713;
                                                                                                                                                                                    }
                                                                                                                                                                                } else {
                                                                                                                                                                                    current_block = 10945915984064580713;
                                                                                                                                                                                }
                                                                                                                                                                                match current_block {
                                                                                                                                                                                    11067230341960327308 => {}
                                                                                                                                                                                    _ => {
                                                                                                                                                                                        i = 0 as size_t;
                                                                                                                                                                                        loop {
                                                                                                                                                                                            if !(i != info.text_num) {
                                                                                                                                                                                                current_block = 5151888778912688305;
                                                                                                                                                                                                break;
                                                                                                                                                                                            }
                                                                                                                                                                                            if lodepng_strlen(*info.text_keys.offset(i as isize))
                                                                                                                                                                                                > 79 as size_t
                                                                                                                                                                                            {
                                                                                                                                                                                                error = 66 as ::core::ffi::c_uint;
                                                                                                                                                                                                current_block = 11067230341960327308;
                                                                                                                                                                                                break;
                                                                                                                                                                                            } else if lodepng_strlen(*info.text_keys.offset(i as isize))
                                                                                                                                                                                                < 1 as size_t
                                                                                                                                                                                            {
                                                                                                                                                                                                error = 67 as ::core::ffi::c_uint;
                                                                                                                                                                                                current_block = 11067230341960327308;
                                                                                                                                                                                                break;
                                                                                                                                                                                            } else {
                                                                                                                                                                                                if (*state).encoder.text_compression != 0 {
                                                                                                                                                                                                    error = addChunk_zTXt(
                                                                                                                                                                                                        &raw mut outv,
                                                                                                                                                                                                        *info.text_keys.offset(i as isize),
                                                                                                                                                                                                        *info.text_strings.offset(i as isize),
                                                                                                                                                                                                        &raw mut (*state).encoder.zlibsettings,
                                                                                                                                                                                                    );
                                                                                                                                                                                                    if error != 0 {
                                                                                                                                                                                                        current_block = 11067230341960327308;
                                                                                                                                                                                                        break;
                                                                                                                                                                                                    }
                                                                                                                                                                                                } else {
                                                                                                                                                                                                    error = addChunk_tEXt(
                                                                                                                                                                                                        &raw mut outv,
                                                                                                                                                                                                        *info.text_keys.offset(i as isize),
                                                                                                                                                                                                        *info.text_strings.offset(i as isize),
                                                                                                                                                                                                    );
                                                                                                                                                                                                    if error != 0 {
                                                                                                                                                                                                        current_block = 11067230341960327308;
                                                                                                                                                                                                        break;
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                                i = i.wrapping_add(1);
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                        match current_block {
                                                                                                                                                                                            11067230341960327308 => {}
                                                                                                                                                                                            _ => {
                                                                                                                                                                                                if (*state).encoder.add_id != 0 {
                                                                                                                                                                                                    let mut already_added_id_text: ::core::ffi::c_uint = 0
                                                                                                                                                                                                        as ::core::ffi::c_uint;
                                                                                                                                                                                                    i = 0 as size_t;
                                                                                                                                                                                                    while i != info.text_num {
                                                                                                                                                                                                        let mut k: *const ::core::ffi::c_char = *info
                                                                                                                                                                                                            .text_keys
                                                                                                                                                                                                            .offset(i as isize);
                                                                                                                                                                                                        if *k.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                            as ::core::ffi::c_int == 'L' as i32
                                                                                                                                                                                                            && *k.offset(1 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as ::core::ffi::c_int == 'o' as i32
                                                                                                                                                                                                            && *k.offset(2 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as ::core::ffi::c_int == 'd' as i32
                                                                                                                                                                                                            && *k.offset(3 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as ::core::ffi::c_int == 'e' as i32
                                                                                                                                                                                                            && *k.offset(4 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as ::core::ffi::c_int == 'P' as i32
                                                                                                                                                                                                            && *k.offset(5 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as ::core::ffi::c_int == 'N' as i32
                                                                                                                                                                                                            && *k.offset(6 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as ::core::ffi::c_int == 'G' as i32
                                                                                                                                                                                                            && *k.offset(7 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as ::core::ffi::c_int == '\0' as i32
                                                                                                                                                                                                        {
                                                                                                                                                                                                            already_added_id_text = 1 as ::core::ffi::c_uint;
                                                                                                                                                                                                            break;
                                                                                                                                                                                                        } else {
                                                                                                                                                                                                            i = i.wrapping_add(1);
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                    if already_added_id_text == 0 as ::core::ffi::c_uint {
                                                                                                                                                                                                        error = addChunk_tEXt(
                                                                                                                                                                                                            &raw mut outv,
                                                                                                                                                                                                            b"LodePNG\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                            LODEPNG_VERSION_STRING,
                                                                                                                                                                                                        );
                                                                                                                                                                                                        if error != 0 {
                                                                                                                                                                                                            current_block = 11067230341960327308;
                                                                                                                                                                                                        } else {
                                                                                                                                                                                                            current_block = 1745632252074978848;
                                                                                                                                                                                                        }
                                                                                                                                                                                                    } else {
                                                                                                                                                                                                        current_block = 1745632252074978848;
                                                                                                                                                                                                    }
                                                                                                                                                                                                } else {
                                                                                                                                                                                                    current_block = 1745632252074978848;
                                                                                                                                                                                                }
                                                                                                                                                                                                match current_block {
                                                                                                                                                                                                    11067230341960327308 => {}
                                                                                                                                                                                                    _ => {
                                                                                                                                                                                                        i = 0 as size_t;
                                                                                                                                                                                                        loop {
                                                                                                                                                                                                            if !(i != info.itext_num) {
                                                                                                                                                                                                                current_block = 919396821984190499;
                                                                                                                                                                                                                break;
                                                                                                                                                                                                            }
                                                                                                                                                                                                            if lodepng_strlen(*info.itext_keys.offset(i as isize))
                                                                                                                                                                                                                > 79 as size_t
                                                                                                                                                                                                            {
                                                                                                                                                                                                                error = 66 as ::core::ffi::c_uint;
                                                                                                                                                                                                                current_block = 11067230341960327308;
                                                                                                                                                                                                                break;
                                                                                                                                                                                                            } else if lodepng_strlen(
                                                                                                                                                                                                                *info.itext_keys.offset(i as isize),
                                                                                                                                                                                                            ) < 1 as size_t
                                                                                                                                                                                                            {
                                                                                                                                                                                                                error = 67 as ::core::ffi::c_uint;
                                                                                                                                                                                                                current_block = 11067230341960327308;
                                                                                                                                                                                                                break;
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                error = addChunk_iTXt(
                                                                                                                                                                                                                    &raw mut outv,
                                                                                                                                                                                                                    (*state).encoder.text_compression,
                                                                                                                                                                                                                    *info.itext_keys.offset(i as isize),
                                                                                                                                                                                                                    *info.itext_langtags.offset(i as isize),
                                                                                                                                                                                                                    *info.itext_transkeys.offset(i as isize),
                                                                                                                                                                                                                    *info.itext_strings.offset(i as isize),
                                                                                                                                                                                                                    &raw mut (*state).encoder.zlibsettings,
                                                                                                                                                                                                                );
                                                                                                                                                                                                                if error != 0 {
                                                                                                                                                                                                                    current_block = 11067230341960327308;
                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                }
                                                                                                                                                                                                                i = i.wrapping_add(1);
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                        match current_block {
                                                                                                                                                                                                            11067230341960327308 => {}
                                                                                                                                                                                                            _ => {
                                                                                                                                                                                                                if !info
                                                                                                                                                                                                                    .unknown_chunks_data[2 as ::core::ffi::c_int as usize]
                                                                                                                                                                                                                    .is_null()
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    error = addUnknownChunks(
                                                                                                                                                                                                                        &raw mut outv,
                                                                                                                                                                                                                        info.unknown_chunks_data[2 as ::core::ffi::c_int as usize],
                                                                                                                                                                                                                        info.unknown_chunks_size[2 as ::core::ffi::c_int as usize],
                                                                                                                                                                                                                    );
                                                                                                                                                                                                                    if error != 0 {
                                                                                                                                                                                                                        current_block = 11067230341960327308;
                                                                                                                                                                                                                    } else {
                                                                                                                                                                                                                        current_block = 1918110639124887667;
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                    current_block = 1918110639124887667;
                                                                                                                                                                                                                }
                                                                                                                                                                                                                match current_block {
                                                                                                                                                                                                                    11067230341960327308 => {}
                                                                                                                                                                                                                    _ => {
                                                                                                                                                                                                                        error = addChunk_IEND(&raw mut outv);
                                                                                                                                                                                                                        error != 0;
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    lodepng_info_cleanup(&raw mut info);
    lodepng_free(data as *mut ::core::ffi::c_void);
    lodepng_color_mode_cleanup(&raw mut auto_color);
    *out = outv.data;
    *outsize = outv.size;
    (*state).error = error;
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode_memory(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut image: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut colortype: LodePNGColorType,
    mut bitdepth: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut error: ::core::ffi::c_uint = 0;
    let mut state: LodePNGState = ::core::mem::zeroed();
    lodepng_state_init(&raw mut state);
    state.info_raw.colortype = colortype;
    state.info_raw.bitdepth = bitdepth;
    state.info_png.color.colortype = colortype;
    state.info_png.color.bitdepth = bitdepth;
    error = lodepng_encode(out, outsize, image, w, h, &raw mut state);
    lodepng_state_cleanup(&raw mut state);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode32(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut image: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return lodepng_encode_memory(
        out,
        outsize,
        image,
        w,
        h,
        LCT_RGBA,
        8 as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode24(
    mut out: *mut *mut ::core::ffi::c_uchar,
    mut outsize: *mut size_t,
    mut image: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return lodepng_encode_memory(out, outsize, image, w, h, LCT_RGB, 8 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode_file(
    mut filename: *const ::core::ffi::c_char,
    mut image: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut colortype: LodePNGColorType,
    mut bitdepth: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut buffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut buffersize: size_t = 0;
    let mut error: ::core::ffi::c_uint = lodepng_encode_memory(
        &raw mut buffer,
        &raw mut buffersize,
        image,
        w,
        h,
        colortype,
        bitdepth,
    );
    if error == 0 {
        error = lodepng_save_file(buffer, buffersize, filename);
    }
    lodepng_free(buffer as *mut ::core::ffi::c_void);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode32_file(
    mut filename: *const ::core::ffi::c_char,
    mut image: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return lodepng_encode_file(filename, image, w, h, LCT_RGBA, 8 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode24_file(
    mut filename: *const ::core::ffi::c_char,
    mut image: *const ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return lodepng_encode_file(filename, image, w, h, LCT_RGB, 8 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encoder_settings_init(mut settings: *mut LodePNGEncoderSettings) {
    lodepng_compress_settings_init(&raw mut (*settings).zlibsettings);
    (*settings).filter_palette_zero = 1 as ::core::ffi::c_uint;
    (*settings).filter_strategy = LFS_MINSUM;
    (*settings).auto_convert = 1 as ::core::ffi::c_uint;
    (*settings).force_palette = 0 as ::core::ffi::c_uint;
    (*settings).predefined_filters = ::core::ptr::null::<::core::ffi::c_uchar>();
    (*settings).add_id = 0 as ::core::ffi::c_uint;
    (*settings).text_compression = 1 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_error_text(
    mut code: ::core::ffi::c_uint,
) -> *const ::core::ffi::c_char {
    match code {
        0 => {
            return b"no error, everything went ok\0" as *const u8 as *const ::core::ffi::c_char;
        }
        1 => return b"nothing done yet\0" as *const u8 as *const ::core::ffi::c_char,
        10 => {
            return b"end of input memory reached without huffman end code\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        11 => {
            return b"error in code tree made it jump outside of huffman tree\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        13 => {
            return b"problem while processing dynamic deflate block\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        14 => {
            return b"problem while processing dynamic deflate block\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        15 => {
            return b"problem while processing dynamic deflate block\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        16 => {
            return b"invalid code while processing dynamic deflate block\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        17 => {
            return b"end of out buffer memory reached while inflating\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        18 => {
            return b"invalid distance code while inflating\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        19 => {
            return b"end of out buffer memory reached while inflating\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        20 => {
            return b"invalid deflate block BTYPE encountered while decoding\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        21 => {
            return b"NLEN is not ones complement of LEN in a deflate block\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        22 => {
            return b"end of out buffer memory reached while inflating\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        23 => {
            return b"end of in buffer memory reached while inflating\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        24 => {
            return b"invalid FCHECK in zlib header\0" as *const u8 as *const ::core::ffi::c_char;
        }
        25 => {
            return b"invalid compression method in zlib header\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        26 => {
            return b"FDICT encountered in zlib header while it's not used for PNG\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        27 => {
            return b"PNG file is smaller than a PNG header\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        28 => {
            return b"incorrect PNG signature, it's no PNG or corrupted\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        29 => {
            return b"first chunk is not the header chunk\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        30 => {
            return b"chunk length too large, chunk broken off at end of file\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        31 => {
            return b"illegal PNG color type or bpp\0" as *const u8 as *const ::core::ffi::c_char;
        }
        32 => {
            return b"illegal PNG compression method\0" as *const u8 as *const ::core::ffi::c_char;
        }
        33 => {
            return b"illegal PNG filter method\0" as *const u8 as *const ::core::ffi::c_char;
        }
        34 => {
            return b"illegal PNG interlace method\0" as *const u8 as *const ::core::ffi::c_char;
        }
        35 => {
            return b"chunk length of a chunk is too large or the chunk too small\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        36 => {
            return b"illegal PNG filter type encountered\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        37 => {
            return b"illegal bit depth for this color type given\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        38 => {
            return b"the palette is too small or too big\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        39 => {
            return b"tRNS chunk before PLTE or has more entries than palette size\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        40 => {
            return b"tRNS chunk has wrong size for grayscale image\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        41 => {
            return b"tRNS chunk has wrong size for RGB image\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        42 => {
            return b"tRNS chunk appeared while it was not allowed for this color type\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        43 => {
            return b"bKGD chunk has wrong size for palette image\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        44 => {
            return b"bKGD chunk has wrong size for grayscale image\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        45 => {
            return b"bKGD chunk has wrong size for RGB image\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        48 => {
            return b"empty input buffer given to decoder. Maybe caused by non-existing file?\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        49 => {
            return b"jumped past memory while generating dynamic huffman tree\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        50 => {
            return b"jumped past memory while generating dynamic huffman tree\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        51 => {
            return b"jumped past memory while inflating huffman block\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        52 => {
            return b"jumped past memory while inflating\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        53 => {
            return b"size of zlib data too small\0" as *const u8 as *const ::core::ffi::c_char;
        }
        54 => {
            return b"repeat symbol in tree while there was no value symbol yet\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        55 => {
            return b"jumped past tree while generating huffman tree\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        56 => {
            return b"given output image colortype or bitdepth not supported for color conversion\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        57 => {
            return b"invalid CRC encountered (checking CRC can be disabled)\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        58 => {
            return b"invalid ADLER32 encountered (checking ADLER32 can be disabled)\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        59 => {
            return b"requested color conversion not supported\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        60 => {
            return b"invalid window size given in the settings of the encoder (must be 0-32768)\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        61 => {
            return b"invalid BTYPE given in the settings of the encoder (only 0, 1 and 2 are allowed)\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        62 => {
            return b"conversion from color to grayscale not supported\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        63 => {
            return b"length of a chunk too long, max allowed for PNG is 2147483647 bytes per chunk\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        64 => {
            return b"the length of the END symbol 256 in the Huffman tree is 0\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        66 => {
            return b"the length of a text chunk keyword given to the encoder is longer than the maximum of 79 bytes\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        67 => {
            return b"the length of a text chunk keyword given to the encoder is smaller than the minimum of 1 byte\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        68 => {
            return b"tried to encode a PLTE chunk with a palette that has less than 1 or more than 256 colors\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        69 => {
            return b"unknown chunk type with 'critical' flag encountered by the decoder\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        71 => {
            return b"invalid interlace mode given to encoder (must be 0 or 1)\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        72 => {
            return b"while decoding, invalid compression method encountered in zTXt, iTXt or iCCP chunk (it must be 0)\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        73 => {
            return b"invalid tIME chunk size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        74 => {
            return b"invalid pHYs chunk size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        75 => {
            return b"no null termination char found while decoding text chunk\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        76 => {
            return b"iTXt chunk too short to contain required bytes\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        77 => {
            return b"integer overflow in buffer size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        78 => {
            return b"failed to open file for reading\0" as *const u8 as *const ::core::ffi::c_char;
        }
        79 => {
            return b"failed to open file for writing\0" as *const u8 as *const ::core::ffi::c_char;
        }
        80 => {
            return b"tried creating a tree of 0 symbols\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        81 => {
            return b"lazy matching at pos 0 is impossible\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        82 => {
            return b"color conversion to palette requested while a color isn't in palette, or index out of bounds\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        83 => {
            return b"memory allocation failed\0" as *const u8 as *const ::core::ffi::c_char;
        }
        84 => {
            return b"given image too small to contain all pixels to be encoded\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        86 => {
            return b"impossible offset in lz77 encoding (internal bug)\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        87 => {
            return b"must provide custom zlib function pointer if LODEPNG_COMPILE_ZLIB is not defined\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        88 => {
            return b"invalid filter strategy given for LodePNGEncoderSettings.filter_strategy\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        89 => {
            return b"text chunk keyword too short or long: must have size 1-79\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        90 => {
            return b"windowsize must be a power of two\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        91 => {
            return b"invalid decompressed idat size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        92 => {
            return b"integer overflow due to too many pixels\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        93 => {
            return b"zero width or height is invalid\0" as *const u8 as *const ::core::ffi::c_char;
        }
        94 => {
            return b"header chunk must have a size of 13 bytes\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        95 => {
            return b"integer overflow with combined idat chunk size\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        96 => {
            return b"invalid gAMA chunk size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        97 => {
            return b"invalid cHRM chunk size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        98 => {
            return b"invalid sRGB chunk size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        99 => {
            return b"invalid sRGB rendering intent\0" as *const u8 as *const ::core::ffi::c_char;
        }
        100 => {
            return b"invalid ICC profile color type, the PNG specification only allows RGB or GRAY\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        101 => {
            return b"PNG specification does not allow RGB ICC profile on gray color types and vice versa\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        102 => {
            return b"not allowed to set grayscale ICC profile with colored pixels by PNG specification\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        103 => {
            return b"invalid palette index in bKGD chunk. Maybe it came before PLTE chunk?\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        104 => {
            return b"invalid bKGD color while encoding (e.g. palette index out of range)\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        105 => {
            return b"integer overflow of bitsize\0" as *const u8 as *const ::core::ffi::c_char;
        }
        106 => {
            return b"PNG file must have PLTE chunk if color type is palette\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        107 => {
            return b"color convert from palette mode requested without setting the palette data in it\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        108 => {
            return b"tried to add more than 256 values to a palette\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        109 => {
            return b"tried to decompress zlib or deflate data larger than desired max_output_size\0"
                as *const u8 as *const ::core::ffi::c_char;
        }
        110 => {
            return b"custom zlib or inflate decompression failed\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        111 => {
            return b"custom zlib or deflate compression failed\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        112 => {
            return b"compressed text unreasonably large\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        113 => {
            return b"ICC profile unreasonably large\0" as *const u8 as *const ::core::ffi::c_char;
        }
        114 => {
            return b"sBIT chunk has wrong size for the color type of the image\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        115 => {
            return b"sBIT value out of range\0" as *const u8 as *const ::core::ffi::c_char;
        }
        116 => {
            return b"cICP value out of range\0" as *const u8 as *const ::core::ffi::c_char;
        }
        117 => {
            return b"invalid cICP chunk size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        118 => {
            return b"mDCV value out of range\0" as *const u8 as *const ::core::ffi::c_char;
        }
        119 => {
            return b"invalid mDCV chunk size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        120 => {
            return b"invalid cLLI chunk size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        121 => {
            return b"invalid chunk type name: may only contain [a-zA-Z]\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        122 => {
            return b"invalid chunk type name: third character must be uppercase\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        123 => {
            return b"invalid ICC profile size\0" as *const u8 as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown error code\0" as *const u8 as *const ::core::ffi::c_char;
}
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
unsafe extern "C" fn run_static_initializers() {
    mask = ((1 as ::core::ffi::c_uint) << FIRSTBITS).wrapping_sub(1 as ::core::ffi::c_uint);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
