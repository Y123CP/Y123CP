use core::ffi::*;
use crate::src::optipng::bitset::opng_bitset_count;
use crate::src::optipng::bitset::opng_bitset_find_first;
use crate::src::optipng::ioutil::opng_fgetsize;
use crate::src::optipng::ioutil::opng_fseeko;
use crate::src::optipng::ioutil::opng_ftello;
use crate::src::optipng::ioutil::opng_fwriteo;
use crate::src::optipng::ioutil::opng_os_copy_attr;
use crate::src::optipng::ioutil::opng_os_create_dir;
use crate::src::optipng::ioutil::opng_os_rename;
use crate::src::optipng::ioutil::opng_os_test;
use crate::src::optipng::ioutil::opng_os_test_eq;
use crate::src::optipng::ioutil::opng_os_unlink;
use crate::src::optipng::ioutil::opng_path_make_backup;
use crate::src::optipng::ioutil::opng_path_replace_dir;
use crate::src::optipng::ioutil::opng_path_replace_ext;
use crate::src::opngreduc::opngreduc::opng_reduce_image;
use crate::src::optipng::ratio::opng_ulratio_to_factor_string;
use crate::src::opngreduc::opngreduc::opng_validate_image;
use crate::src::libpng::pngwrite::png_create_write_struct;
use crate::src::libpng::pngwrite::png_destroy_write_struct;
use crate::src::libpng::pngwutil::png_save_uint_32;
use crate::src::libpng::pngwrite::png_set_compression_level;
use crate::src::libpng::pngwrite::png_set_compression_mem_level;
use crate::src::libpng::pngwrite::png_set_compression_strategy;
use crate::src::libpng::pngwrite::png_set_compression_window_bits;
use crate::src::libpng::pngwrite::png_set_filter;
use crate::src::libpng::pngwrite::png_write_png;
use crate::src::pngxtern::pngxmem::pngx_malloc_rows;
use crate::src::pngxtern::pngxread::pngx_read_image;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
pub use crate::src::libpng::pngwrite::png_info_def;
pub use crate::src::libpng::pngwrite::png_struct_def;
extern "C" {
    fn png_create_read_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
    ) -> png_structp;
    fn png_write_sig(png_ptr: png_structrp);
    fn png_write_chunk(
        png_ptr: png_structrp,
        chunk_name: png_const_bytep,
        data: png_const_bytep,
        length: png_size_t,
    );
    fn png_create_info_struct(png_ptr: png_const_structrp) -> png_infop;
    fn png_destroy_read_struct(
        png_ptr_ptr: png_structpp,
        info_ptr_ptr: png_infopp,
        end_info_ptr_ptr: png_infopp,
    );
    fn png_set_write_fn(
        png_ptr: png_structrp,
        io_ptr: png_voidp,
        write_data_fn: png_rw_ptr,
        output_flush_fn: png_flush_ptr,
    );
    fn png_set_read_fn(png_ptr: png_structrp, io_ptr: png_voidp, read_data_fn: png_rw_ptr);
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_data_freer(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        freer: c_int,
        mask: png_uint_32,
    );
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_get_rows(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_bytepp;
    fn png_set_rows(png_ptr: png_const_structrp, info_ptr: png_inforp, row_pointers: png_bytepp);
    fn png_get_image_height(png_ptr: png_const_structrp, info_ptr: png_const_inforp)
        -> png_uint_32;
    fn png_get_bKGD(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        background: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_set_bKGD(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        background: png_const_color_16p,
    );
    fn png_get_hIST(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        hist: *mut png_uint_16p,
    ) -> png_uint_32;
    fn png_set_hIST(png_ptr: png_const_structrp, info_ptr: png_inforp, hist: png_const_uint_16p);
    fn png_get_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        width: *mut png_uint_32,
        height: *mut png_uint_32,
        bit_depth: *mut c_int,
        color_type: *mut c_int,
        interlace_method: *mut c_int,
        compression_method: *mut c_int,
        filter_method: *mut c_int,
    ) -> png_uint_32;
    fn png_set_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        width: png_uint_32,
        height: png_uint_32,
        bit_depth: c_int,
        color_type: c_int,
        interlace_method: c_int,
        compression_method: c_int,
        filter_method: c_int,
    );
    fn png_get_PLTE(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        palette: *mut png_colorp,
        num_palette: *mut c_int,
    ) -> png_uint_32;
    fn png_set_PLTE(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        palette: png_const_colorp,
        num_palette: c_int,
    );
    fn png_get_sBIT(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        sig_bit: *mut png_color_8p,
    ) -> png_uint_32;
    fn png_set_sBIT(png_ptr: png_const_structrp, info_ptr: png_inforp, sig_bit: png_const_color_8p);
    fn png_get_tRNS(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        trans_alpha: *mut png_bytep,
        num_trans: *mut c_int,
        trans_color: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_set_tRNS(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        trans_alpha: png_const_bytep,
        num_trans: c_int,
        trans_color: png_const_color_16p,
    );
    fn png_set_keep_unknown_chunks(
        png_ptr: png_structrp,
        keep: c_int,
        chunk_list: png_const_bytep,
        num_chunks: c_int,
    );
    fn png_handle_as_unknown(
        png_ptr: png_const_structrp,
        chunk_name: png_const_bytep,
    ) -> c_int;
    fn png_set_unknown_chunks(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        unknowns: png_const_unknown_chunkp,
        num_unknowns: c_int,
    );
    fn png_set_unknown_chunk_location(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        chunk: c_int,
        location: c_int,
    );
    fn png_get_unknown_chunks(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        entries: png_unknown_chunkpp,
    ) -> c_int;
    fn png_set_user_limits(
        png_ptr: png_structrp,
        user_width_max: png_uint_32,
        user_height_max: png_uint_32,
    );
    fn png_get_io_state(png_ptr: png_const_structrp) -> png_uint_32;
    fn _setjmp(__env: *mut __jmp_buf_tag) -> c_int;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: c_int) -> !;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_engine_struct {
    pub started: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_summary_struct {
    pub file_count: c_uint,
    pub err_count: c_uint,
    pub fix_count: c_uint,
    pub snip_count: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_image_struct {
    pub width: png_uint_32,
    pub height: png_uint_32,
    pub bit_depth: c_int,
    pub color_type: c_int,
    pub compression_type: c_int,
    pub filter_type: c_int,
    pub interlace_type: c_int,
    pub row_pointers: png_bytepp,
    pub palette: png_colorp,
    pub num_palette: c_int,
    pub background_ptr: png_color_16p,
    pub background: png_color_16,
    pub hist: png_uint_16p,
    pub sig_bit_ptr: png_color_8p,
    pub sig_bit: png_color_8,
    pub trans_alpha: png_bytep,
    pub num_trans: c_int,
    pub trans_color_ptr: png_color_16p,
    pub trans_color: png_color_16,
    pub unknowns: png_unknown_chunkp,
    pub num_unknowns: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub etmp: *const c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exception_context {
    pub penv: *mut jmp_buf,
    pub caught: c_int,
    pub v: C2RustUnnamed,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [c_long; 8];
pub const INPUT_HAS_MULTIPLE_IMAGES: C2RustUnnamed_htdd24ee73 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_process_struct {
    pub status: c_uint,
    pub num_iterations: c_int,
    pub in_datastream_offset: opng_foffset_t,
    pub in_file_size: opng_fsize_t,
    pub out_file_size: opng_fsize_t,
    pub in_idat_size: opng_fsize_t,
    pub out_idat_size: opng_fsize_t,
    pub best_idat_size: opng_fsize_t,
    pub max_idat_size: opng_fsize_t,
    pub in_plte_trns_size: png_uint_32,
    pub out_plte_trns_size: png_uint_32,
    pub reductions: png_uint_32,
    pub compr_level_set: opng_bitset_t,
    pub mem_level_set: opng_bitset_t,
    pub strategy_set: opng_bitset_t,
    pub filter_set: opng_bitset_t,
    pub best_compr_level: c_int,
    pub best_mem_level: c_int,
    pub best_strategy: c_int,
    pub best_filter: c_int,
}

pub const INPUT_HAS_ERRORS: C2RustUnnamed_htdd24ee73 = 256;

pub const INPUT_HAS_PNG_DATASTREAM: C2RustUnnamed_htdd24ee73 = 2;
pub type png_infopp = *mut *mut png_info;
pub type png_info = png_info_def;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
pub type png_structpp = *mut *mut png_struct;

pub type png_const_structrp = *const png_struct;

pub type png_structrp = *mut png_struct;

pub type png_flush_ptr = Option<unsafe extern "C" fn(png_structp) -> ()>;

pub type png_rw_ptr = Option<unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> ()>;
pub const OUTPUT_NEEDS_NEW_IDAT: C2RustUnnamed_htdd24ee73 = 8192;
pub type png_error_ptr = Option<unsafe extern "C" fn(png_structp, png_const_charp) -> ()>;
pub type png_infop = *mut png_info;
pub type png_inforp = *mut png_info;

pub const OUTPUT_NEEDS_NEW_FILE: C2RustUnnamed_htdd24ee73 = 4096;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_preset {
    pub compr_level: *const c_char,
    pub mem_level: *const c_char,
    pub strategy: *const c_char,
    pub filter: *const c_char,
}
pub const INPUT_IS_PNG_FILE: C2RustUnnamed_htdd24ee73 = 1;
pub const INPUT_HAS_STRIPPED_DATA: C2RustUnnamed_htdd24ee73 = 64;
pub const INPUT_HAS_APNG: C2RustUnnamed_htdd24ee73 = 32;
pub const INPUT_HAS_DIGITAL_SIGNATURE: C2RustUnnamed_htdd24ee73 = 8;
pub const INPUT_HAS_PNG_SIGNATURE: C2RustUnnamed_htdd24ee73 = 4;
pub const INPUT_HAS_JUNK: C2RustUnnamed_htdd24ee73 = 128;

pub type png_const_inforp = *const png_info;

pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const OUTPUT_HAS_ERRORS: C2RustUnnamed_htdd24ee73 = 16384;

pub const OPNG_OPTIM_LEVEL_DEFAULT: c_int = 2 as c_int;
pub const OPNG_OPTIM_LEVEL_MAX: c_int = 7 as c_int;
pub const OPNG_COMPR_LEVEL_MIN: c_int = 1 as c_int;
pub const OPNG_COMPR_LEVEL_MAX: c_int = 9 as c_int;

pub const OPNG_MEM_LEVEL_MIN: c_int = 1 as c_int;
pub const OPNG_MEM_LEVEL_MAX: c_int = 9 as c_int;

pub const OPNG_STRATEGY_MIN: c_int = 0 as c_int;
pub const OPNG_STRATEGY_MAX: c_int = 3 as c_int;

pub const OPNG_FILTER_MIN: c_int = 0 as c_int;
pub const OPNG_FILTER_MAX: c_int = 5 as c_int;

#[no_mangle]
pub static mut the_exception_context: [exception_context; 1] = [exception_context {
    penv: ::core::ptr::null::<jmp_buf>() as *mut jmp_buf,
    caught: 0,
    v: C2RustUnnamed {
        etmp: ::core::ptr::null::<c_char>(),
    },
}; 1];
static mut presets: [opng_preset; 8] = [
    opng_preset {
        compr_level: b"\0" as *const u8 as *const c_char,
        mem_level: b"\0" as *const u8 as *const c_char,
        strategy: b"\0" as *const u8 as *const c_char,
        filter: b"\0" as *const u8 as *const c_char,
    },
    opng_preset {
        compr_level: b"\0" as *const u8 as *const c_char,
        mem_level: b"\0" as *const u8 as *const c_char,
        strategy: b"\0" as *const u8 as *const c_char,
        filter: b"\0" as *const u8 as *const c_char,
    },
    opng_preset {
        compr_level: b"9\0" as *const u8 as *const c_char,
        mem_level: b"8\0" as *const u8 as *const c_char,
        strategy: b"0-\0" as *const u8 as *const c_char,
        filter: b"0,5\0" as *const u8 as *const c_char,
    },
    opng_preset {
        compr_level: b"9\0" as *const u8 as *const c_char,
        mem_level: b"8-9\0" as *const u8 as *const c_char,
        strategy: b"0-\0" as *const u8 as *const c_char,
        filter: b"0,5\0" as *const u8 as *const c_char,
    },
    opng_preset {
        compr_level: b"9\0" as *const u8 as *const c_char,
        mem_level: b"8\0" as *const u8 as *const c_char,
        strategy: b"0-\0" as *const u8 as *const c_char,
        filter: b"0-\0" as *const u8 as *const c_char,
    },
    opng_preset {
        compr_level: b"9\0" as *const u8 as *const c_char,
        mem_level: b"8-9\0" as *const u8 as *const c_char,
        strategy: b"0-\0" as *const u8 as *const c_char,
        filter: b"0-\0" as *const u8 as *const c_char,
    },
    opng_preset {
        compr_level: b"1-9\0" as *const u8 as *const c_char,
        mem_level: b"8\0" as *const u8 as *const c_char,
        strategy: b"0-\0" as *const u8 as *const c_char,
        filter: b"0-\0" as *const u8 as *const c_char,
    },
    opng_preset {
        compr_level: b"1-9\0" as *const u8 as *const c_char,
        mem_level: b"8-9\0" as *const u8 as *const c_char,
        strategy: b"0-\0" as *const u8 as *const c_char,
        filter: b"0-\0" as *const u8 as *const c_char,
    },
];
static mut filter_table: [c_int; 6] = [
    PNG_FILTER_NONE,
    PNG_FILTER_SUB,
    PNG_FILTER_UP,
    PNG_FILTER_AVG,
    PNG_FILTER_PAETH,
    PNG_ALL_FILTERS,
];
static mut sig_PLTE: [png_byte; 4] = [
    0x50 as c_int as png_byte,
    0x4c as c_int as png_byte,
    0x54 as c_int as png_byte,
    0x45 as c_int as png_byte,
];
static mut sig_tRNS: [png_byte; 4] = [
    0x74 as c_int as png_byte,
    0x52 as c_int as png_byte,
    0x4e as c_int as png_byte,
    0x53 as c_int as png_byte,
];
static mut sig_IDAT: [png_byte; 4] = [
    0x49 as c_int as png_byte,
    0x44 as c_int as png_byte,
    0x41 as c_int as png_byte,
    0x54 as c_int as png_byte,
];
static mut sig_IEND: [png_byte; 4] = [
    0x49 as c_int as png_byte,
    0x45 as c_int as png_byte,
    0x4e as c_int as png_byte,
    0x44 as c_int as png_byte,
];
static mut sig_bKGD: [png_byte; 4] = [
    0x62 as c_int as png_byte,
    0x4b as c_int as png_byte,
    0x47 as c_int as png_byte,
    0x44 as c_int as png_byte,
];
static mut sig_hIST: [png_byte; 4] = [
    0x68 as c_int as png_byte,
    0x49 as c_int as png_byte,
    0x53 as c_int as png_byte,
    0x54 as c_int as png_byte,
];
static mut sig_sBIT: [png_byte; 4] = [
    0x73 as c_int as png_byte,
    0x42 as c_int as png_byte,
    0x49 as c_int as png_byte,
    0x54 as c_int as png_byte,
];
static mut sig_dSIG: [png_byte; 4] = [
    0x64 as c_int as png_byte,
    0x53 as c_int as png_byte,
    0x49 as c_int as png_byte,
    0x47 as c_int as png_byte,
];
static mut sig_acTL: [png_byte; 4] = [
    0x61 as c_int as png_byte,
    0x63 as c_int as png_byte,
    0x54 as c_int as png_byte,
    0x4c as c_int as png_byte,
];
static mut sig_fcTL: [png_byte; 4] = [
    0x66 as c_int as png_byte,
    0x63 as c_int as png_byte,
    0x54 as c_int as png_byte,
    0x4c as c_int as png_byte,
];
static mut sig_fdAT: [png_byte; 4] = [
    0x66 as c_int as png_byte,
    0x64 as c_int as png_byte,
    0x41 as c_int as png_byte,
    0x54 as c_int as png_byte,
];
static mut engine: opng_engine_struct = opng_engine_struct { started: 0 };
static mut process: opng_process_struct = opng_process_struct {
    status: 0,
    num_iterations: 0,
    in_datastream_offset: 0,
    in_file_size: 0,
    out_file_size: 0,
    in_idat_size: 0,
    out_idat_size: 0,
    best_idat_size: 0,
    max_idat_size: 0,
    in_plte_trns_size: 0,
    out_plte_trns_size: 0,
    reductions: 0,
    compr_level_set: 0,
    mem_level_set: 0,
    strategy_set: 0,
    filter_set: 0,
    best_compr_level: 0,
    best_mem_level: 0,
    best_strategy: 0,
    best_filter: 0,
};
static mut idat_size_max: opng_fsize_t = PNG_UINT_31_MAX as opng_fsize_t;
static mut idat_size_max_string: *const c_char =
    b"2GB\0" as *const u8 as *const c_char;
static mut summary: opng_summary_struct = opng_summary_struct {
    file_count: 0,
    err_count: 0,
    fix_count: 0,
    snip_count: 0,
};
static mut image: opng_image_struct = opng_image_struct {
    width: 0,
    height: 0,
    bit_depth: 0,
    color_type: 0,
    compression_type: 0,
    filter_type: 0,
    interlace_type: 0,
    row_pointers: ::core::ptr::null::<*mut png_byte>() as *mut *mut png_byte,
    palette: ::core::ptr::null::<png_color>() as *mut png_color,
    num_palette: 0,
    background_ptr: ::core::ptr::null::<png_color_16>() as *mut png_color_16,
    background: png_color_16 {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
    },
    hist: ::core::ptr::null::<png_uint_16>() as *mut png_uint_16,
    sig_bit_ptr: ::core::ptr::null::<png_color_8>() as *mut png_color_8,
    sig_bit: png_color_8 {
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
        alpha: 0,
    },
    trans_alpha: ::core::ptr::null::<png_byte>() as *mut png_byte,
    num_trans: 0,
    trans_color_ptr: ::core::ptr::null::<png_color_16>() as *mut png_color_16,
    trans_color: png_color_16 {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
    },
    unknowns: ::core::ptr::null::<png_unknown_chunk>() as *mut png_unknown_chunk,
    num_unknowns: 0,
};
static mut options: opng_options = opng_options {
    backup: 0,
    clobber: 0,
    debug: 0,
    fix: 0,
    force: 0,
    full: 0,
    preserve: 0,
    quiet: 0,
    simulate: 0,
    verbose: 0,
    out_name: ::core::ptr::null::<c_char>(),
    dir_name: ::core::ptr::null::<c_char>(),
    log_name: ::core::ptr::null::<c_char>(),
    interlace: 0,
    nb: 0,
    nc: 0,
    np: 0,
    nz: 0,
    optim_level: 0,
    compr_level_set: 0,
    mem_level_set: 0,
    strategy_set: 0,
    filter_set: 0,
    window_bits: 0,
    snip: 0,
    strip_all: 0,
};
static mut usr_printf: Option<unsafe extern "C" fn(*const c_char, ...) -> ()> = None;
static mut usr_print_cntrl: Option<unsafe extern "C" fn(c_int) -> ()> = None;
static mut usr_progress: Option<
    unsafe extern "C" fn(c_ulong, c_ulong) -> (),
> = None;
static mut usr_panic: Option<unsafe extern "C" fn(*const c_char) -> ()> = None;
static mut read_ptr: png_structp = ::core::ptr::null::<png_struct>() as *mut png_struct;
static mut read_info_ptr: png_infop = ::core::ptr::null::<png_info>() as *mut png_info;
static mut write_ptr: png_structp = ::core::ptr::null::<png_struct>() as *mut png_struct;
static mut write_info_ptr: png_infop = ::core::ptr::null::<png_info>() as *mut png_info;
fn opng_print_fsize_ratio(mut num: opng_fsize_t, mut denom: opng_fsize_t) { unsafe {
    let mut buffer: [c_char; 32] = [0; 32];
    let mut ratio: opng_ulratio = opng_ulratio { num: 0, denom: 0 };
    let mut result: c_int = 0;
    ratio.num = num as c_ulong;
    ratio.denom = denom as c_ulong;
    result = opng_ulratio_to_factor_string(
        &raw mut buffer as *mut c_char,
        ::core::mem::size_of::<[c_char; 32]>() as size_t,
        &raw mut ratio,
    );
    usr_printf.expect("non-null function pointer")(
        b"%s%s\0" as *const u8 as *const c_char,
        &raw mut buffer as *mut c_char,
        if result > 0 as c_int {
            b"\0" as *const u8 as *const c_char
        } else {
            b"...\0" as *const u8 as *const c_char
        },
    );
} }
fn opng_print_fsize_difference(
    mut init_size: opng_fsize_t,
    mut final_size: opng_fsize_t,
    mut show_ratio: c_int,
) { unsafe {
    let mut difference: opng_fsize_t = 0;
    let mut sign: c_int = 0;
    if init_size <= final_size {
        sign = 0 as c_int;
        difference = final_size.wrapping_sub(init_size);
    } else {
        sign = 1 as c_int;
        difference = init_size.wrapping_sub(final_size);
    }
    if difference == 0 as c_ulong {
        usr_printf.expect("non-null function pointer")(
            b"no change\0" as *const u8 as *const c_char,
        );
        return;
    }
    if difference == 1 as c_ulong {
        usr_printf.expect("non-null function pointer")(
            b"1 byte\0" as *const u8 as *const c_char,
        );
    } else {
        usr_printf.expect("non-null function pointer")(
            b"%lu bytes\0" as *const u8 as *const c_char,
            difference,
        );
    }
    if show_ratio != 0 && init_size > 0 as c_ulong {
        usr_printf.expect("non-null function pointer")(
            b" = \0" as *const u8 as *const c_char,
        );
        opng_print_fsize_ratio(difference, init_size);
    }
    usr_printf.expect("non-null function pointer")(if sign == 0 as c_int {
        b" increase\0" as *const u8 as *const c_char
    } else {
        b" decrease\0" as *const u8 as *const c_char
    });
} }
fn opng_print_image_info(
    mut show_dim: c_int,
    mut show_depth: c_int,
    mut show_type: c_int,
    mut show_interlaced: c_int,
) { unsafe {
    static mut type_channels: [c_int; 8] = [
        1 as c_int,
        0 as c_int,
        3 as c_int,
        1 as c_int,
        2 as c_int,
        0 as c_int,
        4 as c_int,
        0 as c_int,
    ];
    let mut channels: c_int = 0;
    let mut printed: c_int = 0;
    printed = 0 as c_int;
    if show_dim != 0 {
        printed = 1 as c_int;
        usr_printf.expect("non-null function pointer")(
            b"%lux%lu pixels\0" as *const u8 as *const c_char,
            image.width as c_ulong,
            image.height as c_ulong,
        );
    }
    if show_depth != 0 {
        if printed != 0 {
            usr_printf.expect("non-null function pointer")(
                b", \0" as *const u8 as *const c_char,
            );
        }
        printed = 1 as c_int;
        channels = type_channels[(image.color_type & 7 as c_int) as usize];
        if channels != 1 as c_int {
            usr_printf.expect("non-null function pointer")(
                b"%dx%d bits/pixel\0" as *const u8 as *const c_char,
                channels,
                image.bit_depth,
            );
        } else if image.bit_depth != 1 as c_int {
            usr_printf.expect("non-null function pointer")(
                b"%d bits/pixel\0" as *const u8 as *const c_char,
                image.bit_depth,
            );
        } else {
            usr_printf.expect("non-null function pointer")(
                b"1 bit/pixel\0" as *const u8 as *const c_char,
            );
        }
    }
    if show_type != 0 {
        if printed != 0 {
            usr_printf.expect("non-null function pointer")(
                b", \0" as *const u8 as *const c_char,
            );
        }
        printed = 1 as c_int;
        if image.color_type & PNG_COLOR_MASK_PALETTE != 0 {
            if image.num_palette == 1 as c_int {
                usr_printf.expect("non-null function pointer")(
                    b"1 color\0" as *const u8 as *const c_char,
                );
            } else {
                usr_printf.expect("non-null function pointer")(
                    b"%d colors\0" as *const u8 as *const c_char,
                    image.num_palette,
                );
            }
            if image.num_trans > 0 as c_int {
                usr_printf.expect("non-null function pointer")(
                    b" (%d transparent)\0" as *const u8 as *const c_char,
                    image.num_trans,
                );
            }
            usr_printf.expect("non-null function pointer")(
                b" in palette\0" as *const u8 as *const c_char,
            );
        } else {
            usr_printf.expect("non-null function pointer")(
                if image.color_type & PNG_COLOR_MASK_COLOR != 0 {
                    b"RGB\0" as *const u8 as *const c_char
                } else {
                    b"grayscale\0" as *const u8 as *const c_char
                },
            );
            if image.color_type & PNG_COLOR_MASK_ALPHA != 0 {
                usr_printf.expect("non-null function pointer")(
                    b"+alpha\0" as *const u8 as *const c_char,
                );
            } else if !image.trans_color_ptr.is_null() {
                usr_printf.expect("non-null function pointer")(
                    b"+transparency\0" as *const u8 as *const c_char,
                );
            }
        }
    }
    if show_interlaced != 0 {
        if image.interlace_type != PNG_INTERLACE_NONE {
            if printed != 0 {
                usr_printf.expect("non-null function pointer")(
                    b", \0" as *const u8 as *const c_char,
                );
            }
            usr_printf.expect("non-null function pointer")(
                b"interlaced\0" as *const u8 as *const c_char,
            );
        }
    }
} }
unsafe fn opng_print_warning(mut msg: *const c_char) {
    usr_print_cntrl.expect("non-null function pointer")('\u{b}' as i32);
    usr_printf.expect("non-null function pointer")(
        b"Warning: %s\n\0" as *const u8 as *const c_char,
        msg,
    );
}
unsafe fn opng_print_error(mut msg: *const c_char) {
    usr_print_cntrl.expect("non-null function pointer")('\u{b}' as i32);
    usr_printf.expect("non-null function pointer")(
        b"Error: %s\n\0" as *const u8 as *const c_char,
        msg,
    );
}
extern "C" fn opng_warning(mut png_ptr: png_structp, mut msg: png_const_charp) { unsafe {
    if png_ptr == read_ptr {
        process.status |= (INPUT_HAS_ERRORS as c_int
            | OUTPUT_NEEDS_NEW_IDAT as c_int)
            as c_uint;
    }
    opng_print_warning(msg as *const c_char);
} }
extern "C" fn opng_error(mut png_ptr: png_structp, mut msg: png_const_charp) { unsafe {
    if png_ptr == read_ptr {
        process.status |= (INPUT_HAS_ERRORS as c_int
            | OUTPUT_NEEDS_NEW_IDAT as c_int)
            as c_uint;
    }
    loop {
        let ref mut fresh35 = (*(&raw mut the_exception_context as *mut exception_context))
            .v
            .etmp;
        ::core::ptr::write_volatile(fresh35, msg as *const c_char);
        longjmp(
            &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                as *mut __jmp_buf_tag,
            1 as c_int,
        );
    }
} }
unsafe fn opng_free(mut ptr: *mut c_void) {
    free(ptr);
}
fn opng_check_idat_size(mut size: opng_fsize_t) { unsafe {
    if size > idat_size_max {
        loop {
            let ref mut fresh34 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(
                fresh34,
                b"IDAT sizes larger than the maximum chunk size are currently unsupported\0"
                    as *const u8 as *const c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
} }
fn opng_set_keep_unknown_chunk(
    mut png_ptr: png_structp,
    mut keep: c_int,
    mut chunk_type: png_bytep,
) { unsafe {
    let mut chunk_name: [png_byte; 5] = [0; 5];
    memcpy(
        &raw mut chunk_name as *mut png_byte as *mut c_void,
        chunk_type as *const c_void,
        4 as size_t,
    );
    chunk_name[4 as c_int as usize] = 0 as png_byte;
    if png_handle_as_unknown(
        png_ptr as png_const_structrp,
        &raw mut chunk_name as *mut png_byte as png_const_bytep,
    ) == 0
    {
        png_set_keep_unknown_chunks(
            png_ptr as png_structrp,
            keep,
            &raw mut chunk_name as *mut png_byte as png_const_bytep,
            1 as c_int,
        );
    }
} }
fn opng_is_image_chunk(mut chunk_type: png_bytep) -> c_int { unsafe {
    if *chunk_type.offset(0 as c_int as isize) as c_int
        & 0x20 as c_int
        == 0 as c_int
    {
        return 1 as c_int;
    }
    if memcmp(
        chunk_type as *const c_void,
        &raw const sig_tRNS as *const png_byte as *const c_void,
        4 as size_t,
    ) == 0 as c_int
    {
        return 1 as c_int;
    }
    return 0 as c_int;
} }
fn opng_is_apng_chunk(mut chunk_type: png_bytep) -> c_int { unsafe {
    if memcmp(
        chunk_type as *const c_void,
        &raw const sig_acTL as *const png_byte as *const c_void,
        4 as size_t,
    ) == 0 as c_int
        || memcmp(
            chunk_type as *const c_void,
            &raw const sig_fcTL as *const png_byte as *const c_void,
            4 as size_t,
        ) == 0 as c_int
        || memcmp(
            chunk_type as *const c_void,
            &raw const sig_fdAT as *const png_byte as *const c_void,
            4 as size_t,
        ) == 0 as c_int
    {
        return 1 as c_int;
    }
    return 0 as c_int;
} }
fn opng_allow_chunk(mut chunk_type: png_bytep) -> c_int { unsafe {
    if opng_is_image_chunk(chunk_type) != 0 {
        return 1 as c_int;
    }
    if options.strip_all != 0 {
        return 0 as c_int;
    }
    if memcmp(
        chunk_type as *const c_void,
        &raw const sig_dSIG as *const png_byte as *const c_void,
        4 as size_t,
    ) == 0 as c_int
    {
        return 0 as c_int;
    }
    if options.snip != 0 && opng_is_apng_chunk(chunk_type) != 0 {
        return 0 as c_int;
    }
    return 1 as c_int;
} }
fn opng_handle_chunk(mut png_ptr: png_structp, mut chunk_type: png_bytep) { unsafe {
    let mut keep: c_int = 0;
    if opng_is_image_chunk(chunk_type) != 0 {
        return;
    }
    if options.strip_all != 0 {
        process.status |= (INPUT_HAS_STRIPPED_DATA as c_int
            | INPUT_HAS_JUNK as c_int)
            as c_uint;
        opng_set_keep_unknown_chunk(png_ptr, PNG_HANDLE_CHUNK_NEVER, chunk_type);
        return;
    }
    if memcmp(
        chunk_type as *const c_void,
        &raw const sig_bKGD as *const png_byte as *const c_void,
        4 as size_t,
    ) == 0 as c_int
        || memcmp(
            chunk_type as *const c_void,
            &raw const sig_hIST as *const png_byte as *const c_void,
            4 as size_t,
        ) == 0 as c_int
        || memcmp(
            chunk_type as *const c_void,
            &raw const sig_sBIT as *const png_byte as *const c_void,
            4 as size_t,
        ) == 0 as c_int
    {
        return;
    }
    keep = PNG_HANDLE_CHUNK_ALWAYS;
    if memcmp(
        chunk_type as *const c_void,
        &raw const sig_dSIG as *const png_byte as *const c_void,
        4 as size_t,
    ) == 0 as c_int
    {
        process.status |= INPUT_HAS_DIGITAL_SIGNATURE as c_int as c_uint;
    } else if opng_is_apng_chunk(chunk_type) != 0 {
        process.status |= INPUT_HAS_APNG as c_int as c_uint;
        if memcmp(
            chunk_type as *const c_void,
            &raw const sig_fdAT as *const png_byte as *const c_void,
            4 as size_t,
        ) == 0 as c_int
        {
            process.status |=
                INPUT_HAS_MULTIPLE_IMAGES as c_int as c_uint;
        }
        if options.snip != 0 {
            process.status |= INPUT_HAS_JUNK as c_int as c_uint;
            keep = PNG_HANDLE_CHUNK_NEVER;
        }
    }
    opng_set_keep_unknown_chunk(png_ptr, keep, chunk_type);
} }
fn opng_init_read_data() { {} }
fn opng_init_write_data() { unsafe {
    process.out_file_size = 0 as opng_fsize_t;
    process.out_plte_trns_size = 0 as png_uint_32;
    process.out_idat_size = 0 as opng_fsize_t;
} }
extern "C" fn opng_read_data(
    mut png_ptr: png_structp,
    mut data: png_bytep,
    mut length: size_t,
) { unsafe {
    let mut stream: *mut FILE = png_get_io_ptr(png_ptr as png_const_structrp) as *mut FILE;
    let mut io_state: c_int =
        png_get_io_state(png_ptr as png_const_structrp) as c_int;
    let mut io_state_loc: c_int = io_state & PNGX_IO_MASK_LOC;
    let mut chunk_sig: png_bytep = ::core::ptr::null_mut::<png_byte>();
    if fread(
        data as *mut c_void,
        1 as size_t,
        length,
        stream,
    ) as size_t
        != length
    {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't read the input file or unexpected end of file\0" as *const u8
                as png_const_charp,
        );
    }
    if process.in_file_size == 0 as c_ulong {
        if !(length == 8 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"PNG I/O must start with the first 8 bytes\0" as *const u8
                    as *const c_char,
            );
        }
        process.in_datastream_offset = (opng_ftello(stream) as c_long
            - 8 as c_long) as opng_foffset_t;
        process.status |= INPUT_HAS_PNG_DATASTREAM as c_int as c_uint;
        if io_state_loc == PNGX_IO_SIGNATURE {
            process.status |= INPUT_HAS_PNG_SIGNATURE as c_int as c_uint;
        }
        if process.in_datastream_offset == 0 as c_long {
            process.status |= INPUT_IS_PNG_FILE as c_int as c_uint;
        } else if process.in_datastream_offset < 0 as c_long {
            png_error(
                png_ptr as png_const_structrp,
                b"Can't get the file-position indicator in input file\0" as *const u8
                    as png_const_charp,
            );
        }
        process.in_file_size = process.in_datastream_offset as opng_fsize_t;
    }
    process.in_file_size = (process.in_file_size as c_ulong)
        .wrapping_add(length as c_ulong) as opng_fsize_t
        as opng_fsize_t;
    if !(io_state & 0x1 as c_int != 0 && io_state_loc != 0 as c_int) {
        usr_panic.expect("non-null function pointer")(
            b"Incorrect info in png_ptr->io_state\0" as *const u8 as *const c_char,
        );
    }
    if io_state_loc == PNGX_IO_CHUNK_HDR {
        if !(length == 8 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"Reading chunk header, expecting 8 bytes\0" as *const u8
                    as *const c_char,
            );
        }
        chunk_sig = data.offset(4 as c_int as isize);
        if memcmp(
            chunk_sig as *const c_void,
            &raw const sig_IDAT as *const png_byte as *const c_void,
            4 as size_t,
        ) == 0 as c_int
        {
            if !(png_ptr == read_ptr) {
                usr_panic.expect("non-null function pointer")(
                    b"Incorrect I/O handler setup\0" as *const u8 as *const c_char,
                );
            }
            if png_get_rows(
                read_ptr as png_const_structrp,
                read_info_ptr as png_const_inforp,
            )
            .is_null()
            {
                if !(process.in_idat_size == 0 as c_ulong) {
                    usr_panic.expect("non-null function pointer")(
                        b"Found IDAT with no rows\0" as *const u8 as *const c_char,
                    );
                }
                if png_get_image_height(
                    read_ptr as png_const_structrp,
                    read_info_ptr as png_const_inforp,
                ) == 0 as c_uint
                {
                    return;
                }
                if pngx_malloc_rows(read_ptr, read_info_ptr, 0 as c_int).is_null() {
                    usr_panic.expect("non-null function pointer")(
                        b"Failed allocation of image rows; unsafe libpng allocator\0" as *const u8
                            as *const c_char,
                    );
                }
                png_data_freer(
                    read_ptr as png_const_structrp,
                    read_info_ptr as png_inforp,
                    PNG_USER_WILL_FREE_DATA,
                    PNG_FREE_ROWS,
                );
            } else {
                process.status |= INPUT_HAS_JUNK as c_int as c_uint;
            }
            process.in_idat_size = (process.in_idat_size as c_ulong).wrapping_add(
                ((*data as png_uint_32) << 24 as c_int)
                    .wrapping_add(
                        (*data.offset(1 as c_int as isize) as png_uint_32)
                            << 16 as c_int,
                    )
                    .wrapping_add(
                        (*data.offset(2 as c_int as isize) as png_uint_32)
                            << 8 as c_int,
                    )
                    .wrapping_add(*data.offset(3 as c_int as isize) as png_uint_32)
                    as c_ulong,
            ) as opng_fsize_t as opng_fsize_t;
        } else if memcmp(
            chunk_sig as *const c_void,
            &raw const sig_PLTE as *const png_byte as *const c_void,
            4 as size_t,
        ) == 0 as c_int
            || memcmp(
                chunk_sig as *const c_void,
                &raw const sig_tRNS as *const png_byte as *const c_void,
                4 as size_t,
            ) == 0 as c_int
        {
            process.in_plte_trns_size = (process.in_plte_trns_size as c_uint)
                .wrapping_add(
                    ((*data as c_uint) << 24 as c_int)
                        .wrapping_add(
                            (*data.offset(1 as c_int as isize) as c_uint)
                                << 16 as c_int,
                        )
                        .wrapping_add(
                            (*data.offset(2 as c_int as isize) as c_uint)
                                << 8 as c_int,
                        )
                        .wrapping_add(
                            *data.offset(3 as c_int as isize) as c_uint
                        )
                        .wrapping_add(12 as c_uint),
                ) as png_uint_32 as png_uint_32;
        } else {
            opng_handle_chunk(png_ptr, chunk_sig);
        }
    } else if io_state_loc == PNGX_IO_CHUNK_CRC {
        if !(length == 4 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"Reading chunk CRC, expecting 4 bytes\0" as *const u8
                    as *const c_char,
            );
        }
    }
} }
extern "C" fn opng_write_data(
    mut png_ptr: png_structp,
    mut data: png_bytep,
    mut length: size_t,
) { unsafe {
    static mut allow_crt_chunk: c_int = 0;
    static mut crt_chunk_is_idat: c_int = 0;
    static mut crt_idat_offset: opng_foffset_t = 0;
    static mut crt_idat_size: opng_fsize_t = 0;
    static mut crt_idat_crc: png_uint_32 = 0;
    let mut stream: *mut FILE = png_get_io_ptr(png_ptr as png_const_structrp) as *mut FILE;
    let mut io_state: c_int =
        png_get_io_state(png_ptr as png_const_structrp) as c_int;
    let mut io_state_loc: c_int = io_state & PNGX_IO_MASK_LOC;
    let mut chunk_sig: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut buf: [png_byte; 4] = [0; 4];
    if !(io_state & 0x2 as c_int != 0 && io_state_loc != 0 as c_int) {
        usr_panic.expect("non-null function pointer")(
            b"Incorrect info in png_ptr->io_state\0" as *const u8 as *const c_char,
        );
    }
    if io_state_loc == PNGX_IO_CHUNK_HDR {
        if !(length == 8 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"Writing chunk header, expecting 8 bytes\0" as *const u8
                    as *const c_char,
            );
        }
        chunk_sig = data.offset(4 as c_int as isize);
        allow_crt_chunk = opng_allow_chunk(chunk_sig);
        if memcmp(
            chunk_sig as *const c_void,
            &raw const sig_IDAT as *const png_byte as *const c_void,
            4 as size_t,
        ) == 0 as c_int
        {
            crt_chunk_is_idat = 1 as c_int;
            process.out_idat_size = (process.out_idat_size as c_ulong).wrapping_add(
                ((*data as png_uint_32) << 24 as c_int)
                    .wrapping_add(
                        (*data.offset(1 as c_int as isize) as png_uint_32)
                            << 16 as c_int,
                    )
                    .wrapping_add(
                        (*data.offset(2 as c_int as isize) as png_uint_32)
                            << 8 as c_int,
                    )
                    .wrapping_add(*data.offset(3 as c_int as isize) as png_uint_32)
                    as c_ulong,
            ) as opng_fsize_t as opng_fsize_t;
            if stream.is_null() {
                if process.out_idat_size > process.max_idat_size {
                    loop {
                        let ref mut fresh33 = (*(&raw mut the_exception_context
                            as *mut exception_context))
                            .v
                            .etmp;
                        ::core::ptr::write_volatile(
                            fresh33,
                            ::core::ptr::null::<c_char>(),
                        );
                        longjmp(
                            &raw mut *(*(&raw mut the_exception_context as *mut exception_context))
                                .penv as *mut __jmp_buf_tag,
                            1 as c_int,
                        );
                    }
                }
            }
        } else {
            crt_chunk_is_idat = 0 as c_int;
            if memcmp(
                chunk_sig as *const c_void,
                &raw const sig_PLTE as *const png_byte as *const c_void,
                4 as size_t,
            ) == 0 as c_int
                || memcmp(
                    chunk_sig as *const c_void,
                    &raw const sig_tRNS as *const png_byte as *const c_void,
                    4 as size_t,
                ) == 0 as c_int
            {
                process.out_plte_trns_size = (process.out_plte_trns_size as c_uint)
                    .wrapping_add(
                        ((*data as c_uint) << 24 as c_int)
                            .wrapping_add(
                                (*data.offset(1 as c_int as isize)
                                    as c_uint)
                                    << 16 as c_int,
                            )
                            .wrapping_add(
                                (*data.offset(2 as c_int as isize)
                                    as c_uint)
                                    << 8 as c_int,
                            )
                            .wrapping_add(*data.offset(3 as c_int as isize)
                                as c_uint)
                            .wrapping_add(12 as c_uint),
                    ) as png_uint_32 as png_uint_32;
            }
        }
    } else if io_state_loc == PNGX_IO_CHUNK_CRC {
        if !(length == 4 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"Writing chunk CRC, expecting 4 bytes\0" as *const u8
                    as *const c_char,
            );
        }
    }
    if stream.is_null() {
        return;
    }
    if io_state_loc != PNGX_IO_SIGNATURE && allow_crt_chunk == 0 {
        return;
    }
    match io_state_loc {
        PNGX_IO_CHUNK_HDR => {
            if crt_chunk_is_idat != 0 {
                if crt_idat_offset == 0 as c_long {
                    crt_idat_offset = opng_ftello(stream);
                    if process.best_idat_size > 0 as c_ulong {
                        crt_idat_size = process.best_idat_size;
                    } else {
                        crt_idat_size = length as opng_fsize_t;
                    }
                    png_save_uint_32(data, crt_idat_size as png_uint_32);
                    crt_idat_crc = crc32(0 as uLong, &raw const sig_IDAT as *const Bytef, 4 as uInt)
                        as png_uint_32;
                } else {
                    return;
                }
            } else if crt_idat_offset != 0 as c_long {
                png_save_uint_32(&raw mut buf as png_bytep, crt_idat_crc);
                if fwrite(
                    &raw mut buf as *mut png_byte as *const c_void,
                    1 as size_t,
                    4 as size_t,
                    stream,
                ) != 4 as c_ulong
                {
                    io_state = 0 as c_int;
                }
                process.out_file_size = (process.out_file_size as c_ulong)
                    .wrapping_add(4 as c_ulong)
                    as opng_fsize_t as opng_fsize_t;
                if process.out_idat_size != crt_idat_size {
                    if !(process.best_idat_size == 0 as c_ulong) {
                        usr_panic.expect("non-null function pointer")(
                            b"Wrong guess of the output IDAT size\0" as *const u8
                                as *const c_char,
                        );
                    }
                    opng_check_idat_size(process.out_idat_size);
                    png_save_uint_32(
                        &raw mut buf as png_bytep,
                        process.out_idat_size as png_uint_32,
                    );
                    if opng_fwriteo(
                        stream,
                        crt_idat_offset,
                        SEEK_SET,
                        &raw mut buf as *mut png_byte as *const c_void,
                        4 as size_t,
                    ) != 4 as size_t
                    {
                        io_state = 0 as c_int;
                    }
                }
                if io_state == 0 as c_int {
                    png_error(
                        png_ptr as png_const_structrp,
                        b"Can't finalize IDAT\0" as *const u8 as png_const_charp,
                    );
                }
                crt_idat_offset = 0 as opng_foffset_t;
            }
        }
        PNGX_IO_CHUNK_DATA => {
            if crt_chunk_is_idat != 0 {
                crt_idat_crc = crc32(crt_idat_crc as uLong, data as *const Bytef, length as uInt)
                    as png_uint_32;
            }
        }
        PNGX_IO_CHUNK_CRC => {
            if crt_chunk_is_idat != 0 {
                return;
            }
        }
        _ => {}
    }
    if fwrite(
        data as *const c_void,
        1 as size_t,
        length,
        stream,
    ) as size_t
        != length
    {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't write the output file\0" as *const u8 as png_const_charp,
        );
    }
    process.out_file_size = (process.out_file_size as c_ulong)
        .wrapping_add(length as c_ulong) as opng_fsize_t
        as opng_fsize_t;
} }
fn opng_clear_image_info() { unsafe {
    memset(
        &raw mut image as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<opng_image_struct>() as size_t,
    );
} }
fn opng_load_image_info(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut load_meta: c_int,
) { unsafe {
    memset(
        &raw mut image as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<opng_image_struct>() as size_t,
    );
    png_get_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        &raw mut image.width,
        &raw mut image.height,
        &raw mut image.bit_depth,
        &raw mut image.color_type,
        &raw mut image.interlace_type,
        &raw mut image.compression_type,
        &raw mut image.filter_type,
    );
    image.row_pointers = png_get_rows(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    png_get_PLTE(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut image.palette,
        &raw mut image.num_palette,
    );
    if png_get_tRNS(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut image.trans_alpha,
        &raw mut image.num_trans,
        &raw mut image.trans_color_ptr,
    ) != 0
    {
        if !image.trans_color_ptr.is_null() {
            image.trans_color = *image.trans_color_ptr;
            image.trans_color_ptr = &raw mut image.trans_color as png_color_16p;
        }
    }
    if load_meta == 0 {
        return;
    }
    if png_get_bKGD(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut image.background_ptr,
    ) != 0
    {
        image.background = *image.background_ptr;
        image.background_ptr = &raw mut image.background as png_color_16p;
    }
    png_get_hIST(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut image.hist,
    );
    if png_get_sBIT(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut image.sig_bit_ptr,
    ) != 0
    {
        image.sig_bit = *image.sig_bit_ptr;
        image.sig_bit_ptr = &raw mut image.sig_bit as png_color_8p;
    }
    image.num_unknowns = png_get_unknown_chunks(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        &raw mut image.unknowns,
    );
} }
fn opng_store_image_info(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut store_meta: c_int,
) { unsafe {
    let mut i: c_int = 0;
    if image.row_pointers.is_null() {
        usr_panic.expect("non-null function pointer")(
            b"No info in image\0" as *const u8 as *const c_char,
        );
    }
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        image.width,
        image.height,
        image.bit_depth,
        image.color_type,
        image.interlace_type,
        image.compression_type,
        image.filter_type,
    );
    png_set_rows(
        write_ptr as png_const_structrp,
        write_info_ptr as png_inforp,
        image.row_pointers,
    );
    if !image.palette.is_null() {
        png_set_PLTE(
            png_ptr as png_structrp,
            info_ptr as png_inforp,
            image.palette as png_const_colorp,
            image.num_palette,
        );
    }
    if !image.trans_alpha.is_null() || !image.trans_color_ptr.is_null() {
        png_set_tRNS(
            png_ptr as png_structrp,
            info_ptr as png_inforp,
            image.trans_alpha as png_const_bytep,
            image.num_trans,
            image.trans_color_ptr as png_const_color_16p,
        );
    }
    if store_meta == 0 {
        return;
    }
    if !image.background_ptr.is_null() {
        png_set_bKGD(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            image.background_ptr as png_const_color_16p,
        );
    }
    if !image.hist.is_null() {
        png_set_hIST(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            image.hist as png_const_uint_16p,
        );
    }
    if !image.sig_bit_ptr.is_null() {
        png_set_sBIT(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            image.sig_bit_ptr as png_const_color_8p,
        );
    }
    if image.num_unknowns != 0 as c_int {
        png_set_unknown_chunks(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            image.unknowns as png_const_unknown_chunkp,
            image.num_unknowns,
        );
        i = 0 as c_int;
        while i < image.num_unknowns {
            png_set_unknown_chunk_location(
                png_ptr as png_const_structrp,
                info_ptr as png_inforp,
                i,
                (*image.unknowns.offset(i as isize)).location as c_int,
            );
            i += 1;
        }
    }
} }
fn opng_destroy_image_info() { unsafe {
    let mut i: png_uint_32 = 0;
    let mut j: c_int = 0;
    if image.row_pointers.is_null() {
        return;
    }
    i = 0 as png_uint_32;
    while i < image.height {
        opng_free(*image.row_pointers.offset(i as isize) as *mut c_void);
        i = i.wrapping_add(1);
    }
    opng_free(image.row_pointers as *mut c_void);
    opng_free(image.palette as *mut c_void);
    opng_free(image.trans_alpha as *mut c_void);
    opng_free(image.hist as *mut c_void);
    j = 0 as c_int;
    while j < image.num_unknowns {
        opng_free((*image.unknowns.offset(j as isize)).data as *mut c_void);
        j += 1;
    }
    opng_free(image.unknowns as *mut c_void);
    memset(
        &raw mut image as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<opng_image_struct>() as size_t,
    );
} }
unsafe fn opng_read_file(mut infile: *mut FILE) {
    let mut fmt_name: *const c_char = ::core::ptr::null::<c_char>();
    let mut num_img: c_int = 0;
    let mut reductions: png_uint_32 = 0;
    let mut err_msg: *const c_char = ::core::ptr::null::<c_char>();
    let mut exception__prev: *mut jmp_buf = ::core::ptr::null_mut::<jmp_buf>();
    let mut exception__env: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    ::core::ptr::write_volatile(
        &mut exception__prev as *mut *mut jmp_buf,
        (*(&raw mut the_exception_context as *mut exception_context)).penv,
    );
    let ref mut fresh41 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh41 = &raw mut exception__env;
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as c_int {
        loop {
            read_ptr = png_create_read_struct(
                PNG_LIBPNG_VER_STRING.as_ptr(),
                NULL,
                Some(opng_error as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
                Some(opng_warning as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
            );
            read_info_ptr = png_create_info_struct(read_ptr as png_const_structrp);
            if read_info_ptr.is_null() {
                loop {
                    let ref mut fresh42 = (*(&raw mut the_exception_context
                        as *mut exception_context))
                        .v
                        .etmp;
                    ::core::ptr::write_volatile(
                        fresh42,
                        b"Out of memory\0" as *const u8 as *const c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as c_int,
                    );
                }
            }
            png_set_keep_unknown_chunks(
                read_ptr as png_structrp,
                PNG_HANDLE_CHUNK_ALWAYS,
                ::core::ptr::null::<png_byte>(),
                0 as c_int,
            );
            png_set_user_limits(read_ptr as png_structrp, PNG_UINT_31_MAX, PNG_UINT_31_MAX);
            opng_init_read_data();
            png_set_read_fn(
                read_ptr as png_structrp,
                infile as png_voidp,
                Some(opng_read_data as unsafe extern "C" fn(png_structp, png_bytep, size_t) -> ()),
            );
            fmt_name = ::core::ptr::null::<c_char>();
            num_img = pngx_read_image(
                read_ptr,
                read_info_ptr,
                &raw mut fmt_name,
                ::core::ptr::null_mut::<*const c_char>(),
            );
            if num_img <= 0 as c_int {
                loop {
                    let ref mut fresh43 = (*(&raw mut the_exception_context
                        as *mut exception_context))
                        .v
                        .etmp;
                    ::core::ptr::write_volatile(
                        fresh43,
                        b"Unrecognized image file format\0" as *const u8
                            as *const c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as c_int,
                    );
                }
            }
            if num_img > 1 as c_int {
                process.status |=
                    INPUT_HAS_MULTIPLE_IMAGES as c_int as c_uint;
            }
            if process.status & INPUT_IS_PNG_FILE as c_int as c_uint != 0
                && process.status
                    & INPUT_HAS_MULTIPLE_IMAGES as c_int as c_uint
                    != 0
            {
                fmt_name = if process.status
                    & INPUT_HAS_PNG_SIGNATURE as c_int as c_uint
                    != 0
                {
                    b"APNG\0" as *const u8 as *const c_char
                } else {
                    b"APNG datastream\0" as *const u8 as *const c_char
                };
            }
            if fmt_name.is_null() {
                usr_panic.expect("non-null function pointer")(
                    b"No format name from pngxtern\0" as *const u8 as *const c_char,
                );
            }
            if process.in_file_size == 0 as c_ulong {
                if opng_fgetsize(infile, &raw mut process.in_file_size) < 0 as c_int {
                    opng_print_warning(
                        b"Can't get the correct file size\0" as *const u8
                            as *const c_char,
                    );
                    process.in_file_size = 0 as opng_fsize_t;
                }
            }
            ::core::ptr::write_volatile(
                &mut err_msg as *mut *const c_char,
                ::core::ptr::null::<c_char>(),
            );
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as c_int;
    }
    let ref mut fresh44 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh44 = exception__prev;
    if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as c_int != 0
    }) {
        if opng_validate_image(read_ptr, read_info_ptr) != 0 {
            png_warning(read_ptr as png_const_structrp, err_msg as png_const_charp);
            ::core::ptr::write_volatile(
                &mut err_msg as *mut *const c_char,
                ::core::ptr::null::<c_char>(),
            );
        }
    }
    let mut exception__prev_0: *mut jmp_buf = ::core::ptr::null_mut::<jmp_buf>();
    let mut exception__env_0: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    ::core::ptr::write_volatile(
        &mut exception__prev_0 as *mut *mut jmp_buf,
        (*(&raw mut the_exception_context as *mut exception_context)).penv,
    );
    let ref mut fresh45 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh45 = &raw mut exception__env_0;
    if _setjmp(&raw mut exception__env_0 as *mut __jmp_buf_tag) == 0 as c_int {
        loop {
            if !err_msg.is_null() {
                loop {
                    let ref mut fresh46 = (*(&raw mut the_exception_context
                        as *mut exception_context))
                        .v
                        .etmp;
                    ::core::ptr::write_volatile(fresh46, err_msg);
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as c_int,
                    );
                }
            }
            if strcmp(
                fmt_name,
                b"PNG\0" as *const u8 as *const c_char,
            ) != 0 as c_int
            {
                usr_printf.expect("non-null function pointer")(
                    b"Importing %s\0" as *const u8 as *const c_char,
                    fmt_name,
                );
                if process.status
                    & INPUT_HAS_MULTIPLE_IMAGES as c_int as c_uint
                    != 0
                {
                    if process.status
                        & INPUT_IS_PNG_FILE as c_int as c_uint
                        == 0
                    {
                        usr_printf.expect("non-null function pointer")(
                            b" (multi-image or animation)\0" as *const u8
                                as *const c_char,
                        );
                    }
                    if options.snip != 0 {
                        usr_printf.expect("non-null function pointer")(
                            b"; snipping...\0" as *const u8 as *const c_char,
                        );
                    }
                }
                usr_printf.expect("non-null function pointer")(
                    b"\n\0" as *const u8 as *const c_char,
                );
            }
            opng_load_image_info(read_ptr, read_info_ptr, 1 as c_int);
            opng_print_image_info(
                1 as c_int,
                1 as c_int,
                1 as c_int,
                1 as c_int,
            );
            usr_printf.expect("non-null function pointer")(
                b"\n\0" as *const u8 as *const c_char,
            );
            reductions = (OPNG_REDUCE_ALL & !OPNG_REDUCE_METADATA) as png_uint_32;
            if options.nb != 0 {
                reductions &= !OPNG_REDUCE_BIT_DEPTH as c_uint;
            }
            if options.nc != 0 {
                reductions &= !OPNG_REDUCE_COLOR_TYPE as c_uint;
            }
            if options.np != 0 {
                reductions &= !OPNG_REDUCE_PALETTE as c_uint;
            }
            if options.nz != 0
                && process.status
                    & INPUT_HAS_PNG_DATASTREAM as c_int as c_uint
                    != 0
            {
                reductions = OPNG_REDUCE_NONE as png_uint_32;
            }
            if process.status
                & INPUT_HAS_DIGITAL_SIGNATURE as c_int as c_uint
                != 0
            {
                reductions = OPNG_REDUCE_NONE as png_uint_32;
            }
            if process.status & INPUT_IS_PNG_FILE as c_int as c_uint != 0
                && process.status
                    & INPUT_HAS_MULTIPLE_IMAGES as c_int as c_uint
                    != 0
                && reductions != OPNG_REDUCE_NONE as c_uint
                && options.snip == 0
            {
                usr_printf
                    .expect(
                        "non-null function pointer",
                    )(
                    b"Can't reliably reduce APNG file; disabling reductions.\n(Did you want to -snip and optimize the first frame?)\n\0"
                        as *const u8 as *const c_char,
                );
                reductions = OPNG_REDUCE_NONE as png_uint_32;
            }
            process.reductions = opng_reduce_image(read_ptr, read_info_ptr, reductions);
            if process.reductions != OPNG_REDUCE_NONE as c_uint {
                opng_load_image_info(read_ptr, read_info_ptr, 1 as c_int);
                usr_printf.expect("non-null function pointer")(
                    b"Reducing image to \0" as *const u8 as *const c_char,
                );
                opng_print_image_info(
                    0 as c_int,
                    1 as c_int,
                    1 as c_int,
                    0 as c_int,
                );
                usr_printf.expect("non-null function pointer")(
                    b"\n\0" as *const u8 as *const c_char,
                );
            }
            if options.interlace >= 0 as c_int
                && image.interlace_type != options.interlace
            {
                image.interlace_type = options.interlace;
                process.status |=
                    OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint;
            }
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as c_int;
    }
    let ref mut fresh47 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh47 = exception__prev_0;
    if (*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as c_int != 0
    } {
    } else {
        png_data_freer(
            read_ptr as png_const_structrp,
            read_info_ptr as png_inforp,
            PNG_DESTROY_WILL_FREE_DATA,
            PNG_FREE_ALL,
        );
        png_destroy_read_struct(
            &raw mut read_ptr,
            &raw mut read_info_ptr,
            ::core::ptr::null_mut::<*mut png_info>(),
        );
        loop {
            let ref mut fresh48 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(fresh48, err_msg);
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
    png_data_freer(
        read_ptr as png_const_structrp,
        read_info_ptr as png_inforp,
        PNG_USER_WILL_FREE_DATA,
        PNG_FREE_ALL,
    );
    png_destroy_read_struct(
        &raw mut read_ptr,
        &raw mut read_info_ptr,
        ::core::ptr::null_mut::<*mut png_info>(),
    );
}
unsafe fn opng_write_file(
    mut outfile: *mut FILE,
    mut compression_level: c_int,
    mut memory_level: c_int,
    mut compression_strategy: c_int,
    mut filter: c_int,
) {
    let mut err_msg: *const c_char = ::core::ptr::null::<c_char>();
    if !(compression_level >= 1 as c_int
        && compression_level <= 9 as c_int
        && memory_level >= 1 as c_int
        && memory_level <= 9 as c_int
        && compression_strategy >= 0 as c_int
        && compression_strategy <= 3 as c_int
        && filter >= 0 as c_int
        && filter <= 5 as c_int)
    {
        usr_panic.expect("non-null function pointer")(
            b"Invalid encoding parameters\0" as *const u8 as *const c_char,
        );
    }
    let mut exception__prev: *mut jmp_buf = ::core::ptr::null_mut::<jmp_buf>();
    let mut exception__env: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    ::core::ptr::write_volatile(
        &mut exception__prev as *mut *mut jmp_buf,
        (*(&raw mut the_exception_context as *mut exception_context)).penv,
    );
    let ref mut fresh36 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh36 = &raw mut exception__env;
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as c_int {
        loop {
            write_ptr = png_create_write_struct(
                PNG_LIBPNG_VER_STRING.as_ptr(),
                NULL,
                Some(opng_error as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
                Some(opng_warning as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
            );
            write_info_ptr = png_create_info_struct(write_ptr as png_const_structrp);
            if write_info_ptr.is_null() {
                loop {
                    let ref mut fresh37 = (*(&raw mut the_exception_context
                        as *mut exception_context))
                        .v
                        .etmp;
                    ::core::ptr::write_volatile(
                        fresh37,
                        b"Out of memory\0" as *const u8 as *const c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as c_int,
                    );
                }
            }
            png_set_compression_level(write_ptr as png_structrp, compression_level);
            png_set_compression_mem_level(write_ptr as png_structrp, memory_level);
            png_set_compression_strategy(write_ptr as png_structrp, compression_strategy);
            png_set_filter(
                write_ptr as png_structrp,
                PNG_FILTER_TYPE_BASE,
                filter_table[filter as usize],
            );
            if compression_strategy != Z_HUFFMAN_ONLY && compression_strategy != Z_RLE {
                if options.window_bits > 0 as c_int {
                    png_set_compression_window_bits(write_ptr as png_structrp, options.window_bits);
                }
            } else {
                png_set_compression_window_bits(write_ptr as png_structrp, 9 as c_int);
            }
            png_set_keep_unknown_chunks(
                write_ptr as png_structrp,
                PNG_HANDLE_CHUNK_ALWAYS,
                ::core::ptr::null::<png_byte>(),
                0 as c_int,
            );
            png_set_user_limits(write_ptr as png_structrp, PNG_UINT_31_MAX, PNG_UINT_31_MAX);
            opng_store_image_info(
                write_ptr,
                write_info_ptr,
                (outfile != NULL as *mut FILE) as c_int,
            );
            opng_init_write_data();
            png_set_write_fn(
                write_ptr as png_structrp,
                outfile as png_voidp,
                Some(opng_write_data as unsafe extern "C" fn(png_structp, png_bytep, size_t) -> ()),
                None,
            );
            png_write_png(
                write_ptr as png_structrp,
                write_info_ptr as png_inforp,
                0 as c_int,
                NULL,
            );
            ::core::ptr::write_volatile(
                &mut err_msg as *mut *const c_char,
                ::core::ptr::null::<c_char>(),
            );
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as c_int;
    }
    let ref mut fresh38 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh38 = exception__prev;
    if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as c_int != 0
    }) {
        process.out_idat_size = (idat_size_max as c_ulong)
            .wrapping_add(1 as c_ulong)
            as opng_fsize_t;
    }
    png_destroy_write_struct(&raw mut write_ptr, &raw mut write_info_ptr);
    if !err_msg.is_null() {
        loop {
            let ref mut fresh39 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(fresh39, err_msg);
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
}
unsafe fn opng_copy_file(mut infile: *mut FILE, mut outfile: *mut FILE) {
    let mut buf: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let buf_size_incr: png_uint_32 = 0x1000 as png_uint_32;
    let mut buf_size: png_uint_32 = 0;
    let mut length: png_uint_32 = 0;
    let mut chunk_hdr: [png_byte; 8] = [0; 8];
    let mut err_msg: *const c_char = ::core::ptr::null::<c_char>();
    write_ptr = png_create_write_struct(
        PNG_LIBPNG_VER_STRING.as_ptr(),
        NULL,
        Some(opng_error as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
        Some(opng_warning as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
    );
    if write_ptr.is_null() {
        loop {
            let ref mut fresh26 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(
                fresh26,
                b"Out of memory\0" as *const u8 as *const c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
    opng_init_write_data();
    png_set_write_fn(
        write_ptr as png_structrp,
        outfile as png_voidp,
        Some(opng_write_data as unsafe extern "C" fn(png_structp, png_bytep, size_t) -> ()),
        None,
    );
    let mut exception__prev: *mut jmp_buf = ::core::ptr::null_mut::<jmp_buf>();
    let mut exception__env: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    ::core::ptr::write_volatile(
        &mut exception__prev as *mut *mut jmp_buf,
        (*(&raw mut the_exception_context as *mut exception_context)).penv,
    );
    let ref mut fresh27 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh27 = &raw mut exception__env;
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as c_int {
        loop {
            ::core::ptr::write_volatile(
                &mut buf as *mut png_bytep,
                ::core::ptr::null_mut::<png_byte>(),
            );
            buf_size = 0 as png_uint_32;
            png_write_sig(write_ptr as png_structrp);
            loop {
                if fread(
                    &raw mut chunk_hdr as *mut png_byte as *mut c_void,
                    8 as size_t,
                    1 as size_t,
                    infile,
                ) != 1 as c_ulong
                {
                    loop {
                        let ref mut fresh28 = (*(&raw mut the_exception_context
                            as *mut exception_context))
                            .v
                            .etmp;
                        ::core::ptr::write_volatile(
                            fresh28,
                            b"Read error\0" as *const u8 as *const c_char,
                        );
                        longjmp(
                            &raw mut *(*(&raw mut the_exception_context as *mut exception_context))
                                .penv as *mut __jmp_buf_tag,
                            1 as c_int,
                        );
                    }
                }
                length = ((*(&raw mut chunk_hdr as *mut png_byte) as png_uint_32)
                    << 24 as c_int)
                    .wrapping_add(
                        (*(&raw mut chunk_hdr as *mut png_byte)
                            .offset(1 as c_int as isize)
                            as png_uint_32)
                            << 16 as c_int,
                    )
                    .wrapping_add(
                        (*(&raw mut chunk_hdr as *mut png_byte)
                            .offset(2 as c_int as isize)
                            as png_uint_32)
                            << 8 as c_int,
                    )
                    .wrapping_add(
                        *(&raw mut chunk_hdr as *mut png_byte)
                            .offset(3 as c_int as isize)
                            as png_uint_32,
                    );
                if length > PNG_UINT_31_MAX {
                    if !(buf.is_null()
                        && length as c_ulong == 0x89504e47 as c_ulong)
                    {
                        loop {
                            let ref mut fresh29 = (*(&raw mut the_exception_context
                                as *mut exception_context))
                                .v
                                .etmp;
                            ::core::ptr::write_volatile(
                                fresh29,
                                b"Data error\0" as *const u8 as *const c_char,
                            );
                            longjmp(
                                &raw mut *(*(&raw mut the_exception_context
                                    as *mut exception_context))
                                    .penv as *mut __jmp_buf_tag,
                                1 as c_int,
                            );
                        }
                    }
                } else {
                    if (length as c_uint).wrapping_add(4 as c_uint)
                        > buf_size
                    {
                        png_free(write_ptr as png_const_structrp, buf as png_voidp);
                        buf_size = length
                            .wrapping_add(4 as png_uint_32)
                            .wrapping_add(buf_size_incr.wrapping_sub(1 as png_uint_32))
                            .wrapping_div(buf_size_incr)
                            .wrapping_mul(buf_size_incr);
                        ::core::ptr::write_volatile(
                            &mut buf as *mut png_bytep,
                            png_malloc(
                                write_ptr as png_const_structrp,
                                buf_size as png_alloc_size_t,
                            ) as png_bytep,
                        );
                    }
                    if fread(
                        buf as *mut c_void,
                        (length as c_uint).wrapping_add(4 as c_uint)
                            as size_t,
                        1 as size_t,
                        infile,
                    ) != 1 as c_ulong
                    {
                        loop {
                            let ref mut fresh30 = (*(&raw mut the_exception_context
                                as *mut exception_context))
                                .v
                                .etmp;
                            ::core::ptr::write_volatile(
                                fresh30,
                                b"Read error\0" as *const u8 as *const c_char,
                            );
                            longjmp(
                                &raw mut *(*(&raw mut the_exception_context
                                    as *mut exception_context))
                                    .penv as *mut __jmp_buf_tag,
                                1 as c_int,
                            );
                        }
                    }
                    png_write_chunk(
                        write_ptr as png_structrp,
                        (&raw mut chunk_hdr as *mut png_byte)
                            .offset(4 as c_int as isize)
                            as png_const_bytep,
                        buf as png_const_bytep,
                        length as png_size_t,
                    );
                }
                if !(memcmp(
                    (&raw mut chunk_hdr as *mut png_byte).offset(4 as c_int as isize)
                        as *const c_void,
                    &raw const sig_IEND as *const png_byte as *const c_void,
                    4 as size_t,
                ) != 0 as c_int)
                {
                    break;
                }
            }
            ::core::ptr::write_volatile(
                &mut err_msg as *mut *const c_char,
                ::core::ptr::null::<c_char>(),
            );
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as c_int;
    }
    let ref mut fresh31 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh31 = exception__prev;
    (*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as c_int != 0
    };
    png_free(write_ptr as png_const_structrp, buf as png_voidp);
    png_destroy_write_struct(&raw mut write_ptr, ::core::ptr::null_mut::<*mut png_info>());
    if !err_msg.is_null() {
        loop {
            let ref mut fresh32 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(fresh32, err_msg);
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
}
unsafe fn opng_init_iteration(
    mut cmdline_set: opng_bitset_t,
    mut mask_set: opng_bitset_t,
    mut preset: *const c_char,
    mut output_set: *mut opng_bitset_t,
) {
    let mut preset_set: opng_bitset_t = 0;
    let mut check: c_int = 0;
    *output_set = cmdline_set & mask_set;
    if *output_set == 0 as c_uint && cmdline_set != 0 as c_uint {
        loop {
            let ref mut fresh40 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(
                fresh40,
                b"Iteration parameter(s) out of range\0" as *const u8 as *const c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
    if *output_set == 0 as c_uint || options.optim_level >= 0 as c_int {
        check = opng_strparse_rangeset_to_bitset(&raw mut preset_set, preset, mask_set);
        if !(check == 0 as c_int) {
            usr_panic.expect("non-null function pointer")(
                b"[internal] Invalid preset\0" as *const u8 as *const c_char,
            );
        }
        *output_set |= (preset_set & mask_set) as c_uint;
    }
}
fn opng_init_iterations() { unsafe {
    let mut compr_level_set: opng_bitset_t = 0;
    let mut mem_level_set: opng_bitset_t = 0;
    let mut strategy_set: opng_bitset_t = 0;
    let mut filter_set: opng_bitset_t = 0;
    let mut strategy_singles_set: opng_bitset_t = 0;
    let mut preset_index: c_int = 0;
    let mut t1: c_int = 0;
    let mut t2: c_int = 0;
    if process.status & OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint != 0
        || options.full != 0
    {
        process.max_idat_size = idat_size_max;
    } else {
        if !(process.in_idat_size > 0 as c_ulong) {
            usr_panic.expect("non-null function pointer")(
                b"No IDAT in input\0" as *const u8 as *const c_char,
            );
        }
        process.max_idat_size = (process.in_idat_size as c_ulong)
            .wrapping_add(process.in_plte_trns_size as c_ulong)
            as opng_fsize_t;
    }
    preset_index = options.optim_level;
    if preset_index < 0 as c_int {
        preset_index = OPNG_OPTIM_LEVEL_DEFAULT;
    } else if preset_index > OPNG_OPTIM_LEVEL_MAX {
        preset_index = OPNG_OPTIM_LEVEL_MAX;
    }
    opng_init_iteration(
        options.compr_level_set,
        OPNG_COMPR_LEVEL_SET_MASK as opng_bitset_t,
        presets[preset_index as usize].compr_level,
        &raw mut compr_level_set,
    );
    opng_init_iteration(
        options.mem_level_set,
        OPNG_MEM_LEVEL_SET_MASK as opng_bitset_t,
        presets[preset_index as usize].mem_level,
        &raw mut mem_level_set,
    );
    opng_init_iteration(
        options.strategy_set,
        OPNG_STRATEGY_SET_MASK as opng_bitset_t,
        presets[preset_index as usize].strategy,
        &raw mut strategy_set,
    );
    opng_init_iteration(
        options.filter_set,
        OPNG_FILTER_SET_MASK as opng_bitset_t,
        presets[preset_index as usize].filter,
        &raw mut filter_set,
    );
    if compr_level_set == 0 as c_uint {
        compr_level_set |= (1 as c_uint) << 9 as c_int;
    }
    if mem_level_set == 0 as c_uint {
        mem_level_set |= (1 as c_uint) << 8 as c_int;
    }
    if image.bit_depth < 8 as c_int || !image.palette.is_null() {
        if strategy_set == 0 as c_uint {
            strategy_set |= (1 as c_uint) << 0 as c_int;
        }
        if filter_set == 0 as c_uint {
            filter_set |= (1 as c_uint) << 0 as c_int;
        }
    } else {
        if strategy_set == 0 as c_uint {
            strategy_set |= (1 as c_uint) << 1 as c_int;
        }
        if filter_set == 0 as c_uint {
            filter_set |= (1 as c_uint) << 5 as c_int;
        }
    }
    process.compr_level_set = compr_level_set;
    process.mem_level_set = mem_level_set;
    process.strategy_set = strategy_set;
    process.filter_set = filter_set;
    strategy_singles_set = ((1 as c_int) << Z_HUFFMAN_ONLY
        | (1 as c_int) << Z_RLE) as opng_bitset_t;
    t1 = opng_bitset_count(compr_level_set)
        .wrapping_mul(opng_bitset_count(strategy_set & !strategy_singles_set))
        as c_int;
    t2 = opng_bitset_count(strategy_set & strategy_singles_set) as c_int;
    process.num_iterations = ((t1 + t2) as c_uint)
        .wrapping_mul(opng_bitset_count(mem_level_set))
        .wrapping_mul(opng_bitset_count(filter_set))
        as c_int;
    if !(process.num_iterations > 0 as c_int) {
        usr_panic.expect("non-null function pointer")(
            b"Invalid iteration parameters\0" as *const u8 as *const c_char,
        );
    }
} }
fn opng_iterate() { unsafe {
    let mut compr_level_set: opng_bitset_t = 0;
    let mut mem_level_set: opng_bitset_t = 0;
    let mut strategy_set: opng_bitset_t = 0;
    let mut filter_set: opng_bitset_t = 0;
    let mut compr_level: c_int = 0;
    let mut mem_level: c_int = 0;
    let mut strategy: c_int = 0;
    let mut filter: c_int = 0;
    let mut counter: c_int = 0;
    let mut line_reused: c_int = 0;
    if !(process.num_iterations > 0 as c_int) {
        usr_panic.expect("non-null function pointer")(
            b"Iterations not initialized\0" as *const u8 as *const c_char,
        );
    }
    compr_level_set = process.compr_level_set;
    mem_level_set = process.mem_level_set;
    strategy_set = process.strategy_set;
    filter_set = process.filter_set;
    if process.num_iterations == 1 as c_int
        && process.status & OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint != 0
    {
        process.best_idat_size = 0 as opng_fsize_t;
        process.best_compr_level = opng_bitset_find_first(compr_level_set);
        process.best_mem_level = opng_bitset_find_first(mem_level_set);
        process.best_strategy = opng_bitset_find_first(strategy_set);
        process.best_filter = opng_bitset_find_first(filter_set);
        return;
    }
    process.best_idat_size = (idat_size_max as c_ulong)
        .wrapping_add(1 as c_ulong) as opng_fsize_t;
    process.best_compr_level = -(1 as c_int);
    process.best_mem_level = -(1 as c_int);
    process.best_strategy = -(1 as c_int);
    process.best_filter = -(1 as c_int);
    usr_printf.expect("non-null function pointer")(
        b"\nTrying:\n\0" as *const u8 as *const c_char,
    );
    line_reused = 0 as c_int;
    counter = 0 as c_int;
    filter = OPNG_FILTER_MIN;
    while filter <= OPNG_FILTER_MAX {
        if filter_set as c_uint & (1 as c_uint) << filter
            != 0 as c_uint
        {
            strategy = OPNG_STRATEGY_MIN;
            while strategy <= OPNG_STRATEGY_MAX {
                if strategy_set as c_uint & (1 as c_uint) << strategy
                    != 0 as c_uint
                {
                    if strategy == Z_HUFFMAN_ONLY {
                        compr_level_set = 0 as opng_bitset_t;
                        compr_level_set |= (1 as c_uint) << 1 as c_int;
                    } else if strategy == Z_RLE {
                        compr_level_set = 0 as opng_bitset_t;
                        compr_level_set |= (1 as c_uint) << 9 as c_int;
                    } else {
                        compr_level_set = process.compr_level_set;
                    }
                    compr_level = OPNG_COMPR_LEVEL_MAX;
                    while compr_level >= OPNG_COMPR_LEVEL_MIN {
                        if compr_level_set as c_uint
                            & (1 as c_uint) << compr_level
                            != 0 as c_uint
                        {
                            mem_level = OPNG_MEM_LEVEL_MAX;
                            while mem_level >= OPNG_MEM_LEVEL_MIN {
                                if mem_level_set as c_uint
                                    & (1 as c_uint) << mem_level
                                    != 0 as c_uint
                                {
                                    usr_printf.expect("non-null function pointer")(
                                        b"  zc = %d  zm = %d  zs = %d  f = %d\0" as *const u8
                                            as *const c_char,
                                        compr_level,
                                        mem_level,
                                        strategy,
                                        filter,
                                    );
                                    usr_progress.expect("non-null function pointer")(
                                        counter as c_ulong,
                                        process.num_iterations as c_ulong,
                                    );
                                    counter += 1;
                                    opng_write_file(
                                        ::core::ptr::null_mut::<FILE>(),
                                        compr_level,
                                        mem_level,
                                        strategy,
                                        filter,
                                    );
                                    if process.out_idat_size > idat_size_max {
                                        if options.verbose != 0 {
                                            usr_printf.expect("non-null function pointer")(
                                                b"\t\tIDAT too big\n\0" as *const u8
                                                    as *const c_char,
                                            );
                                            line_reused = 0 as c_int;
                                        } else {
                                            usr_print_cntrl.expect("non-null function pointer")(
                                                '\r' as i32,
                                            );
                                            line_reused = 1 as c_int;
                                        }
                                    } else {
                                        usr_printf.expect("non-null function pointer")(
                                            b"\t\tIDAT size = %lu\n\0" as *const u8
                                                as *const c_char,
                                            process.out_idat_size,
                                        );
                                        line_reused = 0 as c_int;
                                        if !(process.best_idat_size < process.out_idat_size) {
                                            if !(process.best_idat_size == process.out_idat_size
                                                && (process.best_strategy == Z_HUFFMAN_ONLY
                                                    || process.best_strategy == Z_RLE))
                                            {
                                                process.best_compr_level = compr_level;
                                                process.best_mem_level = mem_level;
                                                process.best_strategy = strategy;
                                                process.best_filter = filter;
                                                process.best_idat_size = process.out_idat_size;
                                                if options.full == 0 {
                                                    process.max_idat_size = process.out_idat_size;
                                                }
                                            }
                                        }
                                    }
                                }
                                mem_level -= 1;
                            }
                        }
                        compr_level -= 1;
                    }
                }
                strategy += 1;
            }
        }
        filter += 1;
    }
    if line_reused != 0 {
        usr_print_cntrl.expect("non-null function pointer")(-(31 as c_int));
    }
    if !(counter == process.num_iterations) {
        usr_panic.expect("non-null function pointer")(
            b"Inconsistent iteration counter\0" as *const u8 as *const c_char,
        );
    }
    usr_progress.expect("non-null function pointer")(
        counter as c_ulong,
        process.num_iterations as c_ulong,
    );
} }
fn opng_finish_iterations() { unsafe {
    if (process.best_idat_size as c_ulong)
        .wrapping_add(process.out_plte_trns_size as c_ulong)
        < (process.in_idat_size as c_ulong)
            .wrapping_add(process.in_plte_trns_size as c_ulong)
    {
        process.status |= OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint;
    }
    if process.status & OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint != 0 {
        if process.best_idat_size <= idat_size_max {
            usr_printf.expect("non-null function pointer")(
                b"\nSelecting parameters:\n\0" as *const u8 as *const c_char,
            );
            usr_printf.expect("non-null function pointer")(
                b"  zc = %d  zm = %d  zs = %d  f = %d\0" as *const u8 as *const c_char,
                process.best_compr_level,
                process.best_mem_level,
                process.best_strategy,
                process.best_filter,
            );
            if process.best_idat_size > 0 as c_ulong {
                usr_printf.expect("non-null function pointer")(
                    b"\t\tIDAT size = %lu\0" as *const u8 as *const c_char,
                    process.best_idat_size,
                );
            }
            usr_printf.expect("non-null function pointer")(
                b"\n\0" as *const u8 as *const c_char,
            );
        } else {
            usr_printf.expect("non-null function pointer")(
                b"  zc = *  zm = *  zs = *  f = *\t\tIDAT size > %s\n\0" as *const u8
                    as *const c_char,
                idat_size_max_string,
            );
        }
    }
} }
unsafe fn opng_optimize_impl(mut infile_name: *const c_char) {
    static mut infile: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
    static mut outfile: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
    static mut infile_name_local: *const c_char =
        ::core::ptr::null::<c_char>();
    static mut outfile_name: *const c_char =
        ::core::ptr::null::<c_char>();
    static mut bakfile_name: *const c_char =
        ::core::ptr::null::<c_char>();
    static mut new_outfile: c_int = 0;
    static mut has_backup: c_int = 0;
    let mut name_buf: [c_char; 4096] = [0; 4096];
    let mut tmp_buf: [c_char; 4096] = [0; 4096];
    let mut err_msg: *const c_char = ::core::ptr::null::<c_char>();
    memset(
        &raw mut process as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<opng_process_struct>() as size_t,
    );
    if options.force != 0 {
        process.status |= OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint;
    }
    ::core::ptr::write_volatile(
        &mut err_msg as *mut *const c_char,
        ::core::ptr::null::<c_char>(),
    );
    infile_name_local = infile_name;
    infile = fopen(
        infile_name_local,
        b"rb\0" as *const u8 as *const c_char,
    );
    if infile.is_null() {
        loop {
            let ref mut fresh2 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(
                fresh2,
                b"Can't open the input file\0" as *const u8 as *const c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
    let mut exception__prev: *mut jmp_buf = ::core::ptr::null_mut::<jmp_buf>();
    let mut exception__env: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    ::core::ptr::write_volatile(
        &mut exception__prev as *mut *mut jmp_buf,
        (*(&raw mut the_exception_context as *mut exception_context)).penv,
    );
    let ref mut fresh3 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh3 = &raw mut exception__env;
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as c_int {
        loop {
            opng_read_file(infile);
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as c_int;
    }
    let ref mut fresh4 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh4 = exception__prev;
    if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as c_int != 0
    }) {
        if err_msg.is_null() {
            usr_panic.expect("non-null function pointer")(
                b"Mysterious error in opng_read_file\0" as *const u8 as *const c_char,
            );
        }
    }
    fclose(infile);
    if !err_msg.is_null() {
        loop {
            let ref mut fresh5 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(fresh5, err_msg);
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
    if process.status & INPUT_HAS_ERRORS as c_int as c_uint != 0 {
        usr_printf.expect("non-null function pointer")(
            b"Recoverable errors found in input.\0" as *const u8 as *const c_char,
        );
        if options.fix != 0 {
            usr_printf.expect("non-null function pointer")(
                b" Fixing...\n\0" as *const u8 as *const c_char,
            );
            process.status |= OUTPUT_NEEDS_NEW_FILE as c_int as c_uint;
        } else {
            usr_printf.expect("non-null function pointer")(
                b" Rerun OptiPNG with -fix enabled.\n\0" as *const u8 as *const c_char,
            );
            loop {
                let ref mut fresh6 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh6,
                    b"Previous error(s) not fixed\0" as *const u8 as *const c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as c_int,
                );
            }
        }
    }
    if process.status & INPUT_HAS_JUNK as c_int as c_uint != 0 {
        process.status |= OUTPUT_NEEDS_NEW_FILE as c_int as c_uint;
    }
    if process.status & INPUT_HAS_PNG_SIGNATURE as c_int as c_uint == 0 {
        process.status |= OUTPUT_NEEDS_NEW_FILE as c_int as c_uint;
    }
    if process.status & INPUT_HAS_PNG_DATASTREAM as c_int as c_uint != 0 {
        if options.nz != 0
            && process.status & OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint
                != 0
        {
            usr_printf.expect("non-null function pointer")(
                b"IDAT recoding is necessary, but is disabled by the user.\n\0" as *const u8
                    as *const c_char,
            );
            loop {
                let ref mut fresh7 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh7,
                    b"Can't continue\0" as *const u8 as *const c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as c_int,
                );
            }
        }
    } else {
        process.status |= OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint;
    }
    if process.status & INPUT_HAS_DIGITAL_SIGNATURE as c_int as c_uint
        != 0
    {
        usr_printf.expect("non-null function pointer")(
            b"Digital signature found in input.\0" as *const u8 as *const c_char,
        );
        if options.force != 0 {
            usr_printf.expect("non-null function pointer")(
                b" Erasing...\n\0" as *const u8 as *const c_char,
            );
            process.status |= OUTPUT_NEEDS_NEW_FILE as c_int as c_uint;
        } else {
            usr_printf.expect("non-null function pointer")(
                b" Rerun OptiPNG with -force enabled.\n\0" as *const u8
                    as *const c_char,
            );
            loop {
                let ref mut fresh8 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh8,
                    b"Can't optimize digitally-signed files\0" as *const u8
                        as *const c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as c_int,
                );
            }
        }
    }
    if process.status & INPUT_HAS_MULTIPLE_IMAGES as c_int as c_uint != 0
    {
        if options.snip == 0
            && process.status & INPUT_IS_PNG_FILE as c_int as c_uint == 0
        {
            usr_printf.expect("non-null function pointer")(
                b"Conversion to PNG requires snipping. Rerun OptiPNG with -snip enabled.\n\0"
                    as *const u8 as *const c_char,
            );
            loop {
                let ref mut fresh9 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh9,
                    b"Incompatible input format\0" as *const u8 as *const c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as c_int,
                );
            }
        }
    }
    if process.status & INPUT_HAS_APNG as c_int as c_uint != 0
        && options.snip != 0
    {
        process.status |= OUTPUT_NEEDS_NEW_FILE as c_int as c_uint;
    }
    if process.status & INPUT_HAS_STRIPPED_DATA as c_int as c_uint != 0 {
        usr_printf.expect("non-null function pointer")(
            b"Stripping metadata...\n\0" as *const u8 as *const c_char,
        );
    }
    outfile_name = ::core::ptr::null::<c_char>();
    if process.status & INPUT_IS_PNG_FILE as c_int as c_uint == 0 {
        if opng_path_replace_ext(
            &raw mut name_buf as *mut c_char,
            ::core::mem::size_of::<[c_char; 4096]>() as size_t,
            infile_name_local,
            b".png\0" as *const u8 as *const c_char,
        )
        .is_null()
        {
            loop {
                let ref mut fresh10 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh10,
                    b"Can't create the output file (name too long)\0" as *const u8
                        as *const c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as c_int,
                );
            }
        }
        outfile_name = &raw mut name_buf as *mut c_char;
    }
    if !options.out_name.is_null() {
        outfile_name = options.out_name;
    }
    if !options.dir_name.is_null() {
        let mut tmp_name: *const c_char = ::core::ptr::null::<c_char>();
        if !outfile_name.is_null() {
            strcpy(&raw mut tmp_buf as *mut c_char, outfile_name);
            tmp_name = &raw mut tmp_buf as *mut c_char;
        } else {
            tmp_name = infile_name_local;
        }
        if opng_path_replace_dir(
            &raw mut name_buf as *mut c_char,
            ::core::mem::size_of::<[c_char; 4096]>() as size_t,
            tmp_name,
            options.dir_name,
        )
        .is_null()
        {
            loop {
                let ref mut fresh11 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh11,
                    b"Can't create the output file (name too long)\0" as *const u8
                        as *const c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as c_int,
                );
            }
        }
        outfile_name = &raw mut name_buf as *mut c_char;
    }
    if outfile_name.is_null() {
        outfile_name = infile_name_local;
        new_outfile = 0 as c_int;
    } else {
        let mut test_eq: c_int = opng_os_test_eq(infile_name_local, outfile_name);
        if test_eq >= 0 as c_int {
            new_outfile = (test_eq == 0 as c_int) as c_int;
        } else {
            new_outfile = (strcmp(infile_name_local, outfile_name) != 0 as c_int)
                as c_int;
        }
    }
    bakfile_name = &raw mut tmp_buf as *mut c_char;
    if new_outfile != 0 {
        if opng_path_make_backup(
            &raw mut tmp_buf as *mut c_char,
            ::core::mem::size_of::<[c_char; 4096]>() as size_t,
            outfile_name,
        )
        .is_null()
        {
            bakfile_name = ::core::ptr::null::<c_char>();
        }
    } else if opng_path_make_backup(
        &raw mut tmp_buf as *mut c_char,
        ::core::mem::size_of::<[c_char; 4096]>() as size_t,
        infile_name_local,
    )
    .is_null()
    {
        bakfile_name = ::core::ptr::null::<c_char>();
    }
    if bakfile_name.is_null() {
        loop {
            let ref mut fresh12 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(
                fresh12,
                b"Can't create backup file (name too long)\0" as *const u8
                    as *const c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
    if options.simulate == 0
        && opng_os_test(
            outfile_name,
            b"e\0" as *const u8 as *const c_char,
        ) == 0 as c_int
    {
        if new_outfile != 0 && options.backup == 0 && options.clobber == 0 {
            usr_printf.expect("non-null function pointer")(
                b"The output file exists. Rerun OptiPNG with -backup enabled.\n\0" as *const u8
                    as *const c_char,
            );
            loop {
                let ref mut fresh13 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh13,
                    b"Can't overwrite the output file\0" as *const u8 as *const c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as c_int,
                );
            }
        }
        if opng_os_test(
            outfile_name,
            b"fw\0" as *const u8 as *const c_char,
        ) != 0 as c_int
            || options.clobber == 0
                && opng_os_test(
                    bakfile_name,
                    b"e\0" as *const u8 as *const c_char,
                ) == 0 as c_int
        {
            loop {
                let ref mut fresh14 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh14,
                    b"Can't back up the existing output file\0" as *const u8
                        as *const c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as c_int,
                );
            }
        }
    }
    if process.status & INPUT_HAS_PNG_DATASTREAM as c_int as c_uint != 0 {
        usr_printf.expect("non-null function pointer")(
            b"Input IDAT size = %lu bytes\n\0" as *const u8 as *const c_char,
            process.in_idat_size,
        );
    }
    usr_printf.expect("non-null function pointer")(
        b"Input file size = %lu bytes\n\0" as *const u8 as *const c_char,
        process.in_file_size,
    );
    if options.nz == 0
        || process.status & OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint != 0
    {
        opng_init_iterations();
        opng_iterate();
        opng_finish_iterations();
    }
    if process.status & OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint != 0 {
        process.status |= OUTPUT_NEEDS_NEW_FILE as c_int as c_uint;
        opng_check_idat_size(process.best_idat_size);
    }
    if process.status & OUTPUT_NEEDS_NEW_FILE as c_int as c_uint == 0 {
        usr_printf.expect("non-null function pointer")(
            b"\n%s is already optimized.\n\0" as *const u8 as *const c_char,
            infile_name_local,
        );
        if new_outfile == 0 {
            return;
        }
    }
    if options.simulate != 0 {
        usr_printf.expect("non-null function pointer")(
            b"\nNo output: simulation mode.\n\0" as *const u8 as *const c_char,
        );
        return;
    }
    if new_outfile != 0 {
        usr_printf.expect("non-null function pointer")(
            b"\nOutput file: %s\n\0" as *const u8 as *const c_char,
            outfile_name,
        );
        if !options.dir_name.is_null() {
            opng_os_create_dir(options.dir_name);
        }
        has_backup = 0 as c_int;
        if opng_os_test(
            outfile_name,
            b"e\0" as *const u8 as *const c_char,
        ) == 0 as c_int
        {
            if opng_os_rename(outfile_name, bakfile_name, options.clobber)
                != 0 as c_int
            {
                loop {
                    let ref mut fresh15 = (*(&raw mut the_exception_context
                        as *mut exception_context))
                        .v
                        .etmp;
                    ::core::ptr::write_volatile(
                        fresh15,
                        b"Can't back up the output file\0" as *const u8
                            as *const c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as c_int,
                    );
                }
            }
            has_backup = 1 as c_int;
        }
    } else {
        if opng_os_rename(infile_name_local, bakfile_name, options.clobber)
            != 0 as c_int
        {
            loop {
                let ref mut fresh16 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh16,
                    b"Can't back up the input file\0" as *const u8 as *const c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as c_int,
                );
            }
        }
        has_backup = 1 as c_int;
    }
    outfile = fopen(
        outfile_name,
        b"wb\0" as *const u8 as *const c_char,
    );
    let mut exception__prev_0: *mut jmp_buf = ::core::ptr::null_mut::<jmp_buf>();
    let mut exception__env_0: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    ::core::ptr::write_volatile(
        &mut exception__prev_0 as *mut *mut jmp_buf,
        (*(&raw mut the_exception_context as *mut exception_context)).penv,
    );
    let ref mut fresh17 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh17 = &raw mut exception__env_0;
    if _setjmp(&raw mut exception__env_0 as *mut __jmp_buf_tag) == 0 as c_int {
        loop {
            if outfile.is_null() {
                loop {
                    let ref mut fresh18 = (*(&raw mut the_exception_context
                        as *mut exception_context))
                        .v
                        .etmp;
                    ::core::ptr::write_volatile(
                        fresh18,
                        b"Can't open the output file\0" as *const u8 as *const c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as c_int,
                    );
                }
            }
            if process.status & OUTPUT_NEEDS_NEW_IDAT as c_int as c_uint
                != 0
            {
                opng_write_file(
                    outfile,
                    process.best_compr_level,
                    process.best_mem_level,
                    process.best_strategy,
                    process.best_filter,
                );
            } else {
                infile = fopen(
                    if new_outfile != 0 {
                        infile_name_local
                    } else {
                        bakfile_name
                    },
                    b"rb\0" as *const u8 as *const c_char,
                );
                if infile.is_null() {
                    loop {
                        let ref mut fresh19 = (*(&raw mut the_exception_context
                            as *mut exception_context))
                            .v
                            .etmp;
                        ::core::ptr::write_volatile(
                            fresh19,
                            b"Can't reopen the input file\0" as *const u8
                                as *const c_char,
                        );
                        longjmp(
                            &raw mut *(*(&raw mut the_exception_context as *mut exception_context))
                                .penv as *mut __jmp_buf_tag,
                            1 as c_int,
                        );
                    }
                }
                let mut exception__prev_1: *mut jmp_buf = ::core::ptr::null_mut::<jmp_buf>();
                let mut exception__env_1: jmp_buf = [__jmp_buf_tag {
                    __jmpbuf: [0; 8],
                    __mask_was_saved: 0,
                    __saved_mask: __sigset_t { __val: [0; 16] },
                }; 1];
                ::core::ptr::write_volatile(
                    &mut exception__prev_1 as *mut *mut jmp_buf,
                    (*(&raw mut the_exception_context as *mut exception_context)).penv,
                );
                let ref mut fresh20 =
                    (*(&raw mut the_exception_context as *mut exception_context)).penv;
                *fresh20 = &raw mut exception__env_1;
                if _setjmp(&raw mut exception__env_1 as *mut __jmp_buf_tag)
                    == 0 as c_int
                {
                    loop {
                        if process.in_datastream_offset > 0 as c_long
                            && opng_fseeko(infile, process.in_datastream_offset, SEEK_SET)
                                != 0 as c_int
                        {
                            loop {
                                let ref mut fresh21 = (*(&raw mut the_exception_context
                                    as *mut exception_context))
                                    .v
                                    .etmp;
                                ::core::ptr::write_volatile(
                                    fresh21,
                                    b"Can't reposition the input file\0" as *const u8
                                        as *const c_char,
                                );
                                longjmp(
                                    &raw mut *(*(&raw mut the_exception_context
                                        as *mut exception_context))
                                        .penv
                                        as *mut __jmp_buf_tag,
                                    1 as c_int,
                                );
                            }
                        }
                        process.best_idat_size = process.in_idat_size;
                        opng_copy_file(infile, outfile);
                        (*(&raw mut the_exception_context as *mut exception_context)).caught =
                            0 as c_int;
                        if !((*(&raw mut the_exception_context as *mut exception_context)).caught
                            != 0)
                        {
                            break;
                        }
                    }
                } else {
                    (*(&raw mut the_exception_context as *mut exception_context)).caught =
                        1 as c_int;
                }
                let ref mut fresh22 =
                    (*(&raw mut the_exception_context as *mut exception_context)).penv;
                *fresh22 = exception__prev_1;
                if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
                    ::core::ptr::write_volatile(
                        &mut err_msg as *mut *const c_char,
                        (*(&raw mut the_exception_context as *mut exception_context))
                            .v
                            .etmp,
                    );
                    0 as c_int != 0
                }) {
                    if err_msg.is_null() {
                        usr_panic.expect("non-null function pointer")(
                            b"Mysterious error in opng_copy_file\0" as *const u8
                                as *const c_char,
                        );
                    }
                }
                fclose(infile);
                if !err_msg.is_null() {
                    loop {
                        let ref mut fresh23 = (*(&raw mut the_exception_context
                            as *mut exception_context))
                            .v
                            .etmp;
                        ::core::ptr::write_volatile(fresh23, err_msg);
                        longjmp(
                            &raw mut *(*(&raw mut the_exception_context as *mut exception_context))
                                .penv as *mut __jmp_buf_tag,
                            1 as c_int,
                        );
                    }
                }
            }
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as c_int;
    }
    let ref mut fresh24 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh24 = exception__prev_0;
    if (*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as c_int != 0
    } {
    } else {
        if !outfile.is_null() {
            fclose(outfile);
        }
        if has_backup != 0 {
            if opng_os_rename(
                bakfile_name,
                (if new_outfile != 0 {
                    outfile_name
                } else {
                    infile_name_local
                }),
                1 as c_int,
            ) != 0 as c_int
            {
                opng_print_warning(
                    b"Can't recover the original file from backup\0" as *const u8
                        as *const c_char,
                );
            }
        } else {
            if new_outfile == 0 {
                usr_panic.expect("non-null function pointer")(
                    b"Overwrote input with no temporary backup\0" as *const u8
                        as *const c_char,
                );
            }
            if opng_os_unlink(outfile_name) != 0 as c_int {
                opng_print_warning(
                    b"Can't remove the broken output file\0" as *const u8
                        as *const c_char,
                );
            }
        }
        loop {
            let ref mut fresh25 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(fresh25, err_msg);
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as c_int,
            );
        }
    }
    fclose(outfile);
    if options.preserve != 0 {
        opng_os_copy_attr(
            if new_outfile != 0 {
                infile_name_local
            } else {
                bakfile_name
            },
            outfile_name,
        );
    }
    if new_outfile == 0 && options.backup == 0 {
        if opng_os_unlink(bakfile_name) != 0 as c_int {
            opng_print_warning(
                b"Can't remove the backup file\0" as *const u8 as *const c_char,
            );
        }
    }
    usr_printf.expect("non-null function pointer")(
        b"\nOutput IDAT size = %lu bytes\0" as *const u8 as *const c_char,
        process.out_idat_size,
    );
    if process.status & INPUT_HAS_PNG_DATASTREAM as c_int as c_uint != 0 {
        usr_printf.expect("non-null function pointer")(
            b" (\0" as *const u8 as *const c_char,
        );
        opng_print_fsize_difference(
            process.in_idat_size,
            process.out_idat_size,
            0 as c_int,
        );
        usr_printf.expect("non-null function pointer")(
            b")\0" as *const u8 as *const c_char,
        );
    }
    usr_printf.expect("non-null function pointer")(
        b"\nOutput file size = %lu bytes (\0" as *const u8 as *const c_char,
        process.out_file_size,
    );
    opng_print_fsize_difference(
        process.in_file_size,
        process.out_file_size,
        1 as c_int,
    );
    usr_printf.expect("non-null function pointer")(
        b")\n\0" as *const u8 as *const c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn opng_initialize(
    mut init_options: *const opng_options,
    mut init_ui: *const opng_ui,
) -> c_int {
    usr_printf = (*init_ui).printf_fn;
    usr_print_cntrl = (*init_ui).print_cntrl_fn;
    usr_progress = (*init_ui).progress_fn;
    usr_panic = (*init_ui).panic_fn;
    if usr_printf.is_none()
        || usr_print_cntrl.is_none()
        || usr_progress.is_none()
        || usr_panic.is_none()
    {
        return -(1 as c_int);
    }
    options = *init_options;
    if options.optim_level == 0 as c_int {
        options.np = 1 as c_int;
        options.nc = options.np;
        options.nb = options.nc;
        options.nz = 1 as c_int;
    }
    memset(
        &raw mut summary as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<opng_summary_struct>() as size_t,
    );
    engine.started = 1 as c_int;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opng_optimize(
    mut infile_name: *const c_char,
) -> c_int {
    let mut err_msg: *const c_char = ::core::ptr::null::<c_char>();
    let mut result: c_int = 0;
    if engine.started == 0 {
        usr_panic.expect("non-null function pointer")(
            b"The OptiPNG engine is not running\0" as *const u8 as *const c_char,
        );
    }
    usr_printf.expect("non-null function pointer")(
        b"** Processing: %s\n\0" as *const u8 as *const c_char,
        infile_name,
    );
    summary.file_count = summary.file_count.wrapping_add(1);
    opng_clear_image_info();
    let mut exception__prev: *mut jmp_buf = ::core::ptr::null_mut::<jmp_buf>();
    let mut exception__env: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    ::core::ptr::write_volatile(
        &mut exception__prev as *mut *mut jmp_buf,
        (*(&raw mut the_exception_context as *mut exception_context)).penv,
    );
    let ref mut fresh0 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh0 = &raw mut exception__env;
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as c_int {
        loop {
            opng_optimize_impl(infile_name);
            if process.status & INPUT_HAS_ERRORS as c_int as c_uint != 0 {
                summary.err_count = summary.err_count.wrapping_add(1);
                summary.fix_count = summary.fix_count.wrapping_add(1);
            }
            if process.status
                & INPUT_HAS_MULTIPLE_IMAGES as c_int as c_uint
                != 0
            {
                if options.snip != 0 {
                    summary.snip_count = summary.snip_count.wrapping_add(1);
                }
            }
            ::core::ptr::write_volatile(
                &mut result as *mut c_int,
                0 as c_int,
            );
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as c_int;
    }
    let ref mut fresh1 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh1 = exception__prev;
    if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        err_msg = (*(&raw mut the_exception_context as *mut exception_context))
            .v
            .etmp;
        0 as c_int != 0
    }) {
        summary.err_count = summary.err_count.wrapping_add(1);
        opng_print_error(err_msg);
        ::core::ptr::write_volatile(
            &mut result as *mut c_int,
            -(1 as c_int),
        );
    }
    opng_destroy_image_info();
    usr_printf.expect("non-null function pointer")(
        b"\n\0" as *const u8 as *const c_char,
    );
    return result;
}
#[no_mangle]
pub extern "C" fn opng_finalize() -> c_int { unsafe {
    if options.verbose != 0
        || summary.snip_count > 0 as c_uint
        || summary.err_count > 0 as c_uint
    {
        usr_printf.expect("non-null function pointer")(
            b"** Status report\n\0" as *const u8 as *const c_char,
        );
        usr_printf.expect("non-null function pointer")(
            b"%u file(s) have been processed.\n\0" as *const u8 as *const c_char,
            summary.file_count,
        );
        if summary.snip_count > 0 as c_uint {
            usr_printf.expect("non-null function pointer")(
                b"%u multi-image file(s) have been snipped.\n\0" as *const u8
                    as *const c_char,
                summary.snip_count,
            );
        }
        if summary.err_count > 0 as c_uint {
            usr_printf.expect("non-null function pointer")(
                b"%u error(s) have been encountered.\n\0" as *const u8
                    as *const c_char,
                summary.err_count,
            );
            if summary.fix_count > 0 as c_uint {
                usr_printf.expect("non-null function pointer")(
                    b"%u erroneous file(s) have been fixed.\n\0" as *const u8
                        as *const c_char,
                    summary.fix_count,
                );
            }
        }
    }
    engine.started = 0 as c_int;
    return 0 as c_int;
} }
pub const PNG_LIBPNG_VER_STRING: [c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [c_char; 7]>(*b"1.6.34\0") };

pub const PNG_IO_CHUNK_HDR: c_int = 32;
pub const PNG_IO_CHUNK_DATA: c_int = 64;
pub const PNG_IO_CHUNK_CRC: c_int = 128;
pub const PNG_IO_MASK_LOC: c_int = 0xf0 as c_int;

pub const OPNG_REDUCE_PALETTE_TO_RGB: c_int = 0x20 as c_int;

pub const OPNG_REDUCE_PALETTE_SLOW: c_int = 0x100 as c_int;

pub const OPNG_REDUCE_METADATA: c_int = 0x1000 as c_int;
pub const OPNG_REDUCE_BIT_DEPTH: c_int = OPNG_REDUCE_16_TO_8 | OPNG_REDUCE_8_TO_4_2_1;
pub const OPNG_REDUCE_COLOR_TYPE: c_int = OPNG_REDUCE_RGB_TO_GRAY
    | OPNG_REDUCE_STRIP_ALPHA
    | OPNG_REDUCE_RGB_TO_PALETTE
    | OPNG_REDUCE_PALETTE_TO_RGB
    | OPNG_REDUCE_GRAY_TO_PALETTE
    | OPNG_REDUCE_PALETTE_TO_GRAY;
pub const OPNG_REDUCE_PALETTE: c_int =
    OPNG_REDUCE_PALETTE_SLOW | OPNG_REDUCE_PALETTE_FAST;
pub const OPNG_REDUCE_ALL: c_int =
    OPNG_REDUCE_BIT_DEPTH | OPNG_REDUCE_COLOR_TYPE | OPNG_REDUCE_PALETTE | OPNG_REDUCE_METADATA;
pub const PNGX_IO_SIGNATURE: c_int = PNG_IO_SIGNATURE;
pub const PNGX_IO_CHUNK_HDR: c_int = PNG_IO_CHUNK_HDR;
pub const PNGX_IO_CHUNK_DATA: c_int = PNG_IO_CHUNK_DATA;
pub const PNGX_IO_CHUNK_CRC: c_int = PNG_IO_CHUNK_CRC;
pub const PNGX_IO_MASK_LOC: c_int = PNG_IO_MASK_LOC;

