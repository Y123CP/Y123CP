extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_info_def;
    pub type png_struct_def;
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
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn opng_bitset_count(set: opng_bitset_t) -> ::core::ffi::c_uint;
    fn opng_bitset_find_first(set: opng_bitset_t) -> ::core::ffi::c_int;
    fn opng_strparse_rangeset_to_bitset(
        out_set: *mut opng_bitset_t,
        rangeset_str: *const ::core::ffi::c_char,
        mask_set: opng_bitset_t,
    ) -> ::core::ffi::c_int;
    fn opng_ftello(stream: *mut FILE) -> opng_foffset_t;
    fn opng_fseeko(
        stream: *mut FILE,
        offset: opng_foffset_t,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn opng_fwriteo(
        stream: *mut FILE,
        offset: opng_foffset_t,
        whence: ::core::ffi::c_int,
        block: *const ::core::ffi::c_void,
        blocksize: size_t,
    ) -> size_t;
    fn opng_fgetsize(stream: *mut FILE, size: *mut opng_fsize_t) -> ::core::ffi::c_int;
    fn opng_path_replace_dir(
        buffer: *mut ::core::ffi::c_char,
        bufsize: size_t,
        old_path: *const ::core::ffi::c_char,
        new_dirname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn opng_path_replace_ext(
        buffer: *mut ::core::ffi::c_char,
        bufsize: size_t,
        old_path: *const ::core::ffi::c_char,
        new_extname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn opng_path_make_backup(
        buffer: *mut ::core::ffi::c_char,
        bufsize: size_t,
        path: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn opng_os_rename(
        src_path: *const ::core::ffi::c_char,
        dest_path: *const ::core::ffi::c_char,
        clobber: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn opng_os_copy_attr(
        src_path: *const ::core::ffi::c_char,
        dest_path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn opng_os_create_dir(dirname: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn opng_os_test(
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn opng_os_test_eq(
        path1: *const ::core::ffi::c_char,
        path2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn opng_os_unlink(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn png_create_read_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
    ) -> png_structp;
    fn png_create_write_struct(
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
    fn png_destroy_write_struct(png_ptr_ptr: png_structpp, info_ptr_ptr: png_infopp);
    fn png_set_filter(
        png_ptr: png_structrp,
        method: ::core::ffi::c_int,
        filters: ::core::ffi::c_int,
    );
    fn png_set_compression_level(png_ptr: png_structrp, level: ::core::ffi::c_int);
    fn png_set_compression_mem_level(png_ptr: png_structrp, mem_level: ::core::ffi::c_int);
    fn png_set_compression_strategy(png_ptr: png_structrp, strategy: ::core::ffi::c_int);
    fn png_set_compression_window_bits(png_ptr: png_structrp, window_bits: ::core::ffi::c_int);
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
        freer: ::core::ffi::c_int,
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
        bit_depth: *mut ::core::ffi::c_int,
        color_type: *mut ::core::ffi::c_int,
        interlace_method: *mut ::core::ffi::c_int,
        compression_method: *mut ::core::ffi::c_int,
        filter_method: *mut ::core::ffi::c_int,
    ) -> png_uint_32;
    fn png_set_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        width: png_uint_32,
        height: png_uint_32,
        bit_depth: ::core::ffi::c_int,
        color_type: ::core::ffi::c_int,
        interlace_method: ::core::ffi::c_int,
        compression_method: ::core::ffi::c_int,
        filter_method: ::core::ffi::c_int,
    );
    fn png_get_PLTE(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        palette: *mut png_colorp,
        num_palette: *mut ::core::ffi::c_int,
    ) -> png_uint_32;
    fn png_set_PLTE(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        palette: png_const_colorp,
        num_palette: ::core::ffi::c_int,
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
        num_trans: *mut ::core::ffi::c_int,
        trans_color: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_set_tRNS(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        trans_alpha: png_const_bytep,
        num_trans: ::core::ffi::c_int,
        trans_color: png_const_color_16p,
    );
    fn png_set_keep_unknown_chunks(
        png_ptr: png_structrp,
        keep: ::core::ffi::c_int,
        chunk_list: png_const_bytep,
        num_chunks: ::core::ffi::c_int,
    );
    fn png_handle_as_unknown(
        png_ptr: png_const_structrp,
        chunk_name: png_const_bytep,
    ) -> ::core::ffi::c_int;
    fn png_set_unknown_chunks(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        unknowns: png_const_unknown_chunkp,
        num_unknowns: ::core::ffi::c_int,
    );
    fn png_set_unknown_chunk_location(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        chunk: ::core::ffi::c_int,
        location: ::core::ffi::c_int,
    );
    fn png_get_unknown_chunks(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        entries: png_unknown_chunkpp,
    ) -> ::core::ffi::c_int;
    fn png_write_png(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        transforms: ::core::ffi::c_int,
        params: png_voidp,
    );
    fn png_set_user_limits(
        png_ptr: png_structrp,
        user_width_max: png_uint_32,
        user_height_max: png_uint_32,
    );
    fn png_get_io_state(png_ptr: png_const_structrp) -> png_uint_32;
    fn png_save_uint_32(buf: png_bytep, i: png_uint_32);
    fn opng_validate_image(png_ptr: png_structp, info_ptr: png_infop) -> ::core::ffi::c_int;
    fn opng_reduce_image(
        png_ptr: png_structp,
        info_ptr: png_infop,
        reductions: png_uint_32,
    ) -> png_uint_32;
    fn pngx_read_image(
        png_ptr: png_structp,
        info_ptr: png_infop,
        fmt_name_ptr: png_const_charpp,
        fmt_long_name_ptr: png_const_charpp,
    ) -> ::core::ffi::c_int;
    fn pngx_malloc_rows(
        png_ptr: png_structp,
        info_ptr: png_infop,
        filler: ::core::ffi::c_int,
    ) -> png_bytepp;
    fn opng_ulratio_to_factor_string(
        buffer: *mut ::core::ffi::c_char,
        buffer_size: size_t,
        ratio: *const opng_ulratio,
    ) -> ::core::ffi::c_int;
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn _setjmp(__env: *mut __jmp_buf_tag) -> ::core::ffi::c_int;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type opng_bitset_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_options {
    pub backup: ::core::ffi::c_int,
    pub clobber: ::core::ffi::c_int,
    pub debug: ::core::ffi::c_int,
    pub fix: ::core::ffi::c_int,
    pub force: ::core::ffi::c_int,
    pub full: ::core::ffi::c_int,
    pub preserve: ::core::ffi::c_int,
    pub quiet: ::core::ffi::c_int,
    pub simulate: ::core::ffi::c_int,
    pub verbose: ::core::ffi::c_int,
    pub out_name: *const ::core::ffi::c_char,
    pub dir_name: *const ::core::ffi::c_char,
    pub log_name: *const ::core::ffi::c_char,
    pub interlace: ::core::ffi::c_int,
    pub nb: ::core::ffi::c_int,
    pub nc: ::core::ffi::c_int,
    pub np: ::core::ffi::c_int,
    pub nz: ::core::ffi::c_int,
    pub optim_level: ::core::ffi::c_int,
    pub compr_level_set: opng_bitset_t,
    pub mem_level_set: opng_bitset_t,
    pub strategy_set: opng_bitset_t,
    pub filter_set: opng_bitset_t,
    pub window_bits: ::core::ffi::c_int,
    pub snip: ::core::ffi::c_int,
    pub strip_all: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ui {
    pub printf_fn: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>,
    pub print_cntrl_fn: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    pub progress_fn: Option<unsafe extern "C" fn(::core::ffi::c_ulong, ::core::ffi::c_ulong) -> ()>,
    pub panic_fn: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_engine_struct {
    pub started: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_summary_struct {
    pub file_count: ::core::ffi::c_uint,
    pub err_count: ::core::ffi::c_uint,
    pub fix_count: ::core::ffi::c_uint,
    pub snip_count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_image_struct {
    pub width: png_uint_32,
    pub height: png_uint_32,
    pub bit_depth: ::core::ffi::c_int,
    pub color_type: ::core::ffi::c_int,
    pub compression_type: ::core::ffi::c_int,
    pub filter_type: ::core::ffi::c_int,
    pub interlace_type: ::core::ffi::c_int,
    pub row_pointers: png_bytepp,
    pub palette: png_colorp,
    pub num_palette: ::core::ffi::c_int,
    pub background_ptr: png_color_16p,
    pub background: png_color_16,
    pub hist: png_uint_16p,
    pub sig_bit_ptr: png_color_8p,
    pub sig_bit: png_color_8,
    pub trans_alpha: png_bytep,
    pub num_trans: ::core::ffi::c_int,
    pub trans_color_ptr: png_color_16p,
    pub trans_color: png_color_16,
    pub unknowns: png_unknown_chunkp,
    pub num_unknowns: ::core::ffi::c_int,
}
pub type png_unknown_chunkp = *mut png_unknown_chunk;
pub type png_unknown_chunk = png_unknown_chunk_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_unknown_chunk_t {
    pub name: [png_byte; 5],
    pub data: *mut png_byte,
    pub size: png_size_t,
    pub location: png_byte,
}
pub type png_byte = ::core::ffi::c_uchar;
pub type png_size_t = size_t;
pub type png_color_16 = png_color_16_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_uint_16 = ::core::ffi::c_ushort;
pub type png_color_16p = *mut png_color_16;
pub type png_bytep = *mut png_byte;
pub type png_color_8 = png_color_8_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_8_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
    pub gray: png_byte,
    pub alpha: png_byte,
}
pub type png_color_8p = *mut png_color_8;
pub type png_uint_16p = *mut png_uint_16;
pub type png_colorp = *mut png_color;
pub type png_color = png_color_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_bytepp = *mut *mut png_byte;
pub type png_uint_32 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub etmp: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exception_context {
    pub penv: *mut jmp_buf,
    pub caught: ::core::ffi::c_int,
    pub v: C2RustUnnamed,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [::core::ffi::c_long; 8];
pub const INPUT_HAS_MULTIPLE_IMAGES: C2RustUnnamed_0 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_process_struct {
    pub status: ::core::ffi::c_uint,
    pub num_iterations: ::core::ffi::c_int,
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
    pub best_compr_level: ::core::ffi::c_int,
    pub best_mem_level: ::core::ffi::c_int,
    pub best_strategy: ::core::ffi::c_int,
    pub best_filter: ::core::ffi::c_int,
}
pub type opng_fsize_t = ::core::ffi::c_ulong;
pub type opng_foffset_t = ::core::ffi::c_long;
pub const INPUT_HAS_ERRORS: C2RustUnnamed_0 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ulratio {
    pub num: ::core::ffi::c_ulong,
    pub denom: ::core::ffi::c_ulong,
}
pub const INPUT_HAS_PNG_DATASTREAM: C2RustUnnamed_0 = 2;
pub type png_infopp = *mut *mut png_info;
pub type png_info = png_info_def;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
pub type png_structpp = *mut *mut png_struct;
pub type png_voidp = *mut ::core::ffi::c_void;
pub type png_const_structrp = *const png_struct;
pub type png_const_bytep = *const png_byte;
pub type png_structrp = *mut png_struct;
pub type png_alloc_size_t = png_size_t;
pub type png_flush_ptr = Option<unsafe extern "C" fn(png_structp) -> ()>;
pub type png_const_charp = *const ::core::ffi::c_char;
pub type uLong = ::core::ffi::c_ulong;
pub type uInt = ::core::ffi::c_uint;
pub type Bytef = Byte;
pub type Byte = ::core::ffi::c_uchar;
pub type png_rw_ptr = Option<unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> ()>;
pub const OUTPUT_NEEDS_NEW_IDAT: C2RustUnnamed_0 = 8192;
pub type png_error_ptr = Option<unsafe extern "C" fn(png_structp, png_const_charp) -> ()>;
pub type png_infop = *mut png_info;
pub type png_inforp = *mut png_info;
pub type png_const_unknown_chunkp = *const png_unknown_chunk;
pub type png_const_color_8p = *const png_color_8;
pub type png_const_uint_16p = *const png_uint_16;
pub type png_const_color_16p = *const png_color_16;
pub type png_const_colorp = *const png_color;
pub const OUTPUT_NEEDS_NEW_FILE: C2RustUnnamed_0 = 4096;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_preset {
    pub compr_level: *const ::core::ffi::c_char,
    pub mem_level: *const ::core::ffi::c_char,
    pub strategy: *const ::core::ffi::c_char,
    pub filter: *const ::core::ffi::c_char,
}
pub const INPUT_IS_PNG_FILE: C2RustUnnamed_0 = 1;
pub const INPUT_HAS_STRIPPED_DATA: C2RustUnnamed_0 = 64;
pub const INPUT_HAS_APNG: C2RustUnnamed_0 = 32;
pub const INPUT_HAS_DIGITAL_SIGNATURE: C2RustUnnamed_0 = 8;
pub const INPUT_HAS_PNG_SIGNATURE: C2RustUnnamed_0 = 4;
pub const INPUT_HAS_JUNK: C2RustUnnamed_0 = 128;
pub type png_unknown_chunkpp = *mut *mut png_unknown_chunk;
pub type png_const_inforp = *const png_info;
pub type png_const_charpp = *mut *const ::core::ffi::c_char;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const OUTPUT_HAS_ERRORS: C2RustUnnamed_0 = 16384;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPNG_OPTIM_LEVEL_DEFAULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OPNG_OPTIM_LEVEL_MAX: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OPNG_COMPR_LEVEL_MIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OPNG_COMPR_LEVEL_MAX: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OPNG_COMPR_LEVEL_SET_MASK: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
    << 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
    - ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
pub const OPNG_MEM_LEVEL_MIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OPNG_MEM_LEVEL_MAX: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OPNG_MEM_LEVEL_SET_MASK: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
    << 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
    - ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
pub const OPNG_STRATEGY_MIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPNG_STRATEGY_MAX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OPNG_STRATEGY_SET_MASK: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
    << 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
    - ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
pub const OPNG_FILTER_MIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPNG_FILTER_MAX: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OPNG_FILTER_SET_MASK: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
    << 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
    - ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
#[no_mangle]
pub static mut the_exception_context: [exception_context; 1] = [exception_context {
    penv: ::core::ptr::null::<jmp_buf>() as *mut jmp_buf,
    caught: 0,
    v: C2RustUnnamed {
        etmp: ::core::ptr::null::<::core::ffi::c_char>(),
    },
}; 1];
static mut presets: [opng_preset; 8] = [
    opng_preset {
        compr_level: b"\0" as *const u8 as *const ::core::ffi::c_char,
        mem_level: b"\0" as *const u8 as *const ::core::ffi::c_char,
        strategy: b"\0" as *const u8 as *const ::core::ffi::c_char,
        filter: b"\0" as *const u8 as *const ::core::ffi::c_char,
    },
    opng_preset {
        compr_level: b"\0" as *const u8 as *const ::core::ffi::c_char,
        mem_level: b"\0" as *const u8 as *const ::core::ffi::c_char,
        strategy: b"\0" as *const u8 as *const ::core::ffi::c_char,
        filter: b"\0" as *const u8 as *const ::core::ffi::c_char,
    },
    opng_preset {
        compr_level: b"9\0" as *const u8 as *const ::core::ffi::c_char,
        mem_level: b"8\0" as *const u8 as *const ::core::ffi::c_char,
        strategy: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
        filter: b"0,5\0" as *const u8 as *const ::core::ffi::c_char,
    },
    opng_preset {
        compr_level: b"9\0" as *const u8 as *const ::core::ffi::c_char,
        mem_level: b"8-9\0" as *const u8 as *const ::core::ffi::c_char,
        strategy: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
        filter: b"0,5\0" as *const u8 as *const ::core::ffi::c_char,
    },
    opng_preset {
        compr_level: b"9\0" as *const u8 as *const ::core::ffi::c_char,
        mem_level: b"8\0" as *const u8 as *const ::core::ffi::c_char,
        strategy: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
        filter: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
    },
    opng_preset {
        compr_level: b"9\0" as *const u8 as *const ::core::ffi::c_char,
        mem_level: b"8-9\0" as *const u8 as *const ::core::ffi::c_char,
        strategy: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
        filter: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
    },
    opng_preset {
        compr_level: b"1-9\0" as *const u8 as *const ::core::ffi::c_char,
        mem_level: b"8\0" as *const u8 as *const ::core::ffi::c_char,
        strategy: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
        filter: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
    },
    opng_preset {
        compr_level: b"1-9\0" as *const u8 as *const ::core::ffi::c_char,
        mem_level: b"8-9\0" as *const u8 as *const ::core::ffi::c_char,
        strategy: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
        filter: b"0-\0" as *const u8 as *const ::core::ffi::c_char,
    },
];
static mut filter_table: [::core::ffi::c_int; 6] = [
    PNG_FILTER_NONE,
    PNG_FILTER_SUB,
    PNG_FILTER_UP,
    PNG_FILTER_AVG,
    PNG_FILTER_PAETH,
    PNG_ALL_FILTERS,
];
static mut sig_PLTE: [png_byte; 4] = [
    0x50 as ::core::ffi::c_int as png_byte,
    0x4c as ::core::ffi::c_int as png_byte,
    0x54 as ::core::ffi::c_int as png_byte,
    0x45 as ::core::ffi::c_int as png_byte,
];
static mut sig_tRNS: [png_byte; 4] = [
    0x74 as ::core::ffi::c_int as png_byte,
    0x52 as ::core::ffi::c_int as png_byte,
    0x4e as ::core::ffi::c_int as png_byte,
    0x53 as ::core::ffi::c_int as png_byte,
];
static mut sig_IDAT: [png_byte; 4] = [
    0x49 as ::core::ffi::c_int as png_byte,
    0x44 as ::core::ffi::c_int as png_byte,
    0x41 as ::core::ffi::c_int as png_byte,
    0x54 as ::core::ffi::c_int as png_byte,
];
static mut sig_IEND: [png_byte; 4] = [
    0x49 as ::core::ffi::c_int as png_byte,
    0x45 as ::core::ffi::c_int as png_byte,
    0x4e as ::core::ffi::c_int as png_byte,
    0x44 as ::core::ffi::c_int as png_byte,
];
static mut sig_bKGD: [png_byte; 4] = [
    0x62 as ::core::ffi::c_int as png_byte,
    0x4b as ::core::ffi::c_int as png_byte,
    0x47 as ::core::ffi::c_int as png_byte,
    0x44 as ::core::ffi::c_int as png_byte,
];
static mut sig_hIST: [png_byte; 4] = [
    0x68 as ::core::ffi::c_int as png_byte,
    0x49 as ::core::ffi::c_int as png_byte,
    0x53 as ::core::ffi::c_int as png_byte,
    0x54 as ::core::ffi::c_int as png_byte,
];
static mut sig_sBIT: [png_byte; 4] = [
    0x73 as ::core::ffi::c_int as png_byte,
    0x42 as ::core::ffi::c_int as png_byte,
    0x49 as ::core::ffi::c_int as png_byte,
    0x54 as ::core::ffi::c_int as png_byte,
];
static mut sig_dSIG: [png_byte; 4] = [
    0x64 as ::core::ffi::c_int as png_byte,
    0x53 as ::core::ffi::c_int as png_byte,
    0x49 as ::core::ffi::c_int as png_byte,
    0x47 as ::core::ffi::c_int as png_byte,
];
static mut sig_acTL: [png_byte; 4] = [
    0x61 as ::core::ffi::c_int as png_byte,
    0x63 as ::core::ffi::c_int as png_byte,
    0x54 as ::core::ffi::c_int as png_byte,
    0x4c as ::core::ffi::c_int as png_byte,
];
static mut sig_fcTL: [png_byte; 4] = [
    0x66 as ::core::ffi::c_int as png_byte,
    0x63 as ::core::ffi::c_int as png_byte,
    0x54 as ::core::ffi::c_int as png_byte,
    0x4c as ::core::ffi::c_int as png_byte,
];
static mut sig_fdAT: [png_byte; 4] = [
    0x66 as ::core::ffi::c_int as png_byte,
    0x64 as ::core::ffi::c_int as png_byte,
    0x41 as ::core::ffi::c_int as png_byte,
    0x54 as ::core::ffi::c_int as png_byte,
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
static mut idat_size_max_string: *const ::core::ffi::c_char =
    b"2GB\0" as *const u8 as *const ::core::ffi::c_char;
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
    out_name: ::core::ptr::null::<::core::ffi::c_char>(),
    dir_name: ::core::ptr::null::<::core::ffi::c_char>(),
    log_name: ::core::ptr::null::<::core::ffi::c_char>(),
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
static mut usr_printf: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()> = None;
static mut usr_print_cntrl: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()> = None;
static mut usr_progress: Option<
    unsafe extern "C" fn(::core::ffi::c_ulong, ::core::ffi::c_ulong) -> (),
> = None;
static mut usr_panic: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()> = None;
static mut read_ptr: png_structp = ::core::ptr::null::<png_struct>() as *mut png_struct;
static mut read_info_ptr: png_infop = ::core::ptr::null::<png_info>() as *mut png_info;
static mut write_ptr: png_structp = ::core::ptr::null::<png_struct>() as *mut png_struct;
static mut write_info_ptr: png_infop = ::core::ptr::null::<png_info>() as *mut png_info;
unsafe extern "C" fn opng_print_fsize_ratio(mut num: opng_fsize_t, mut denom: opng_fsize_t) {
    let mut buffer: [::core::ffi::c_char; 32] = [0; 32];
    let mut ratio: opng_ulratio = opng_ulratio { num: 0, denom: 0 };
    let mut result: ::core::ffi::c_int = 0;
    ratio.num = num as ::core::ffi::c_ulong;
    ratio.denom = denom as ::core::ffi::c_ulong;
    result = opng_ulratio_to_factor_string(
        &raw mut buffer as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
        &raw mut ratio,
    );
    usr_printf.expect("non-null function pointer")(
        b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut buffer as *mut ::core::ffi::c_char,
        if result > 0 as ::core::ffi::c_int {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"...\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
}
unsafe extern "C" fn opng_print_fsize_difference(
    mut init_size: opng_fsize_t,
    mut final_size: opng_fsize_t,
    mut show_ratio: ::core::ffi::c_int,
) {
    let mut difference: opng_fsize_t = 0;
    let mut sign: ::core::ffi::c_int = 0;
    if init_size <= final_size {
        sign = 0 as ::core::ffi::c_int;
        difference = final_size.wrapping_sub(init_size);
    } else {
        sign = 1 as ::core::ffi::c_int;
        difference = init_size.wrapping_sub(final_size);
    }
    if difference == 0 as ::core::ffi::c_ulong {
        usr_printf.expect("non-null function pointer")(
            b"no change\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if difference == 1 as ::core::ffi::c_ulong {
        usr_printf.expect("non-null function pointer")(
            b"1 byte\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        usr_printf.expect("non-null function pointer")(
            b"%lu bytes\0" as *const u8 as *const ::core::ffi::c_char,
            difference,
        );
    }
    if show_ratio != 0 && init_size > 0 as ::core::ffi::c_ulong {
        usr_printf.expect("non-null function pointer")(
            b" = \0" as *const u8 as *const ::core::ffi::c_char,
        );
        opng_print_fsize_ratio(difference, init_size);
    }
    usr_printf.expect("non-null function pointer")(if sign == 0 as ::core::ffi::c_int {
        b" increase\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b" decrease\0" as *const u8 as *const ::core::ffi::c_char
    });
}
unsafe extern "C" fn opng_print_image_info(
    mut show_dim: ::core::ffi::c_int,
    mut show_depth: ::core::ffi::c_int,
    mut show_type: ::core::ffi::c_int,
    mut show_interlaced: ::core::ffi::c_int,
) {
    static mut type_channels: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut channels: ::core::ffi::c_int = 0;
    let mut printed: ::core::ffi::c_int = 0;
    printed = 0 as ::core::ffi::c_int;
    if show_dim != 0 {
        printed = 1 as ::core::ffi::c_int;
        usr_printf.expect("non-null function pointer")(
            b"%lux%lu pixels\0" as *const u8 as *const ::core::ffi::c_char,
            image.width as ::core::ffi::c_ulong,
            image.height as ::core::ffi::c_ulong,
        );
    }
    if show_depth != 0 {
        if printed != 0 {
            usr_printf.expect("non-null function pointer")(
                b", \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        printed = 1 as ::core::ffi::c_int;
        channels = type_channels[(image.color_type & 7 as ::core::ffi::c_int) as usize];
        if channels != 1 as ::core::ffi::c_int {
            usr_printf.expect("non-null function pointer")(
                b"%dx%d bits/pixel\0" as *const u8 as *const ::core::ffi::c_char,
                channels,
                image.bit_depth,
            );
        } else if image.bit_depth != 1 as ::core::ffi::c_int {
            usr_printf.expect("non-null function pointer")(
                b"%d bits/pixel\0" as *const u8 as *const ::core::ffi::c_char,
                image.bit_depth,
            );
        } else {
            usr_printf.expect("non-null function pointer")(
                b"1 bit/pixel\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if show_type != 0 {
        if printed != 0 {
            usr_printf.expect("non-null function pointer")(
                b", \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        printed = 1 as ::core::ffi::c_int;
        if image.color_type & PNG_COLOR_MASK_PALETTE != 0 {
            if image.num_palette == 1 as ::core::ffi::c_int {
                usr_printf.expect("non-null function pointer")(
                    b"1 color\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                usr_printf.expect("non-null function pointer")(
                    b"%d colors\0" as *const u8 as *const ::core::ffi::c_char,
                    image.num_palette,
                );
            }
            if image.num_trans > 0 as ::core::ffi::c_int {
                usr_printf.expect("non-null function pointer")(
                    b" (%d transparent)\0" as *const u8 as *const ::core::ffi::c_char,
                    image.num_trans,
                );
            }
            usr_printf.expect("non-null function pointer")(
                b" in palette\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            usr_printf.expect("non-null function pointer")(
                if image.color_type & PNG_COLOR_MASK_COLOR != 0 {
                    b"RGB\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"grayscale\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            if image.color_type & PNG_COLOR_MASK_ALPHA != 0 {
                usr_printf.expect("non-null function pointer")(
                    b"+alpha\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if !image.trans_color_ptr.is_null() {
                usr_printf.expect("non-null function pointer")(
                    b"+transparency\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
    }
    if show_interlaced != 0 {
        if image.interlace_type != PNG_INTERLACE_NONE {
            if printed != 0 {
                usr_printf.expect("non-null function pointer")(
                    b", \0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            usr_printf.expect("non-null function pointer")(
                b"interlaced\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn opng_print_warning(mut msg: *const ::core::ffi::c_char) {
    usr_print_cntrl.expect("non-null function pointer")('\u{b}' as i32);
    usr_printf.expect("non-null function pointer")(
        b"Warning: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        msg,
    );
}
unsafe extern "C" fn opng_print_error(mut msg: *const ::core::ffi::c_char) {
    usr_print_cntrl.expect("non-null function pointer")('\u{b}' as i32);
    usr_printf.expect("non-null function pointer")(
        b"Error: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        msg,
    );
}
unsafe extern "C" fn opng_warning(mut png_ptr: png_structp, mut msg: png_const_charp) {
    if png_ptr == read_ptr {
        process.status |= (INPUT_HAS_ERRORS as ::core::ffi::c_int
            | OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
    }
    opng_print_warning(msg as *const ::core::ffi::c_char);
}
unsafe extern "C" fn opng_error(mut png_ptr: png_structp, mut msg: png_const_charp) {
    if png_ptr == read_ptr {
        process.status |= (INPUT_HAS_ERRORS as ::core::ffi::c_int
            | OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
    }
    loop {
        let ref mut fresh35 = (*(&raw mut the_exception_context as *mut exception_context))
            .v
            .etmp;
        ::core::ptr::write_volatile(fresh35, msg as *const ::core::ffi::c_char);
        longjmp(
            &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                as *mut __jmp_buf_tag,
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn opng_free(mut ptr: *mut ::core::ffi::c_void) {
    free(ptr);
}
unsafe extern "C" fn opng_check_idat_size(mut size: opng_fsize_t) {
    if size > idat_size_max {
        loop {
            let ref mut fresh34 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(
                fresh34,
                b"IDAT sizes larger than the maximum chunk size are currently unsupported\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn opng_set_keep_unknown_chunk(
    mut png_ptr: png_structp,
    mut keep: ::core::ffi::c_int,
    mut chunk_type: png_bytep,
) {
    let mut chunk_name: [png_byte; 5] = [0; 5];
    memcpy(
        &raw mut chunk_name as *mut png_byte as *mut ::core::ffi::c_void,
        chunk_type as *const ::core::ffi::c_void,
        4 as size_t,
    );
    chunk_name[4 as ::core::ffi::c_int as usize] = 0 as png_byte;
    if png_handle_as_unknown(
        png_ptr as png_const_structrp,
        &raw mut chunk_name as *mut png_byte as png_const_bytep,
    ) == 0
    {
        png_set_keep_unknown_chunks(
            png_ptr as png_structrp,
            keep,
            &raw mut chunk_name as *mut png_byte as png_const_bytep,
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn opng_is_image_chunk(mut chunk_type: png_bytep) -> ::core::ffi::c_int {
    if *chunk_type.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0x20 as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if memcmp(
        chunk_type as *const ::core::ffi::c_void,
        &raw const sig_tRNS as *const png_byte as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn opng_is_apng_chunk(mut chunk_type: png_bytep) -> ::core::ffi::c_int {
    if memcmp(
        chunk_type as *const ::core::ffi::c_void,
        &raw const sig_acTL as *const png_byte as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0 as ::core::ffi::c_int
        || memcmp(
            chunk_type as *const ::core::ffi::c_void,
            &raw const sig_fcTL as *const png_byte as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        || memcmp(
            chunk_type as *const ::core::ffi::c_void,
            &raw const sig_fdAT as *const png_byte as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn opng_allow_chunk(mut chunk_type: png_bytep) -> ::core::ffi::c_int {
    if opng_is_image_chunk(chunk_type) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if options.strip_all != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if memcmp(
        chunk_type as *const ::core::ffi::c_void,
        &raw const sig_dSIG as *const png_byte as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if options.snip != 0 && opng_is_apng_chunk(chunk_type) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn opng_handle_chunk(mut png_ptr: png_structp, mut chunk_type: png_bytep) {
    let mut keep: ::core::ffi::c_int = 0;
    if opng_is_image_chunk(chunk_type) != 0 {
        return;
    }
    if options.strip_all != 0 {
        process.status |= (INPUT_HAS_STRIPPED_DATA as ::core::ffi::c_int
            | INPUT_HAS_JUNK as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        opng_set_keep_unknown_chunk(png_ptr, PNG_HANDLE_CHUNK_NEVER, chunk_type);
        return;
    }
    if memcmp(
        chunk_type as *const ::core::ffi::c_void,
        &raw const sig_bKGD as *const png_byte as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0 as ::core::ffi::c_int
        || memcmp(
            chunk_type as *const ::core::ffi::c_void,
            &raw const sig_hIST as *const png_byte as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        || memcmp(
            chunk_type as *const ::core::ffi::c_void,
            &raw const sig_sBIT as *const png_byte as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        return;
    }
    keep = PNG_HANDLE_CHUNK_ALWAYS;
    if memcmp(
        chunk_type as *const ::core::ffi::c_void,
        &raw const sig_dSIG as *const png_byte as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        process.status |= INPUT_HAS_DIGITAL_SIGNATURE as ::core::ffi::c_int as ::core::ffi::c_uint;
    } else if opng_is_apng_chunk(chunk_type) != 0 {
        process.status |= INPUT_HAS_APNG as ::core::ffi::c_int as ::core::ffi::c_uint;
        if memcmp(
            chunk_type as *const ::core::ffi::c_void,
            &raw const sig_fdAT as *const png_byte as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            process.status |=
                INPUT_HAS_MULTIPLE_IMAGES as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
        if options.snip != 0 {
            process.status |= INPUT_HAS_JUNK as ::core::ffi::c_int as ::core::ffi::c_uint;
            keep = PNG_HANDLE_CHUNK_NEVER;
        }
    }
    opng_set_keep_unknown_chunk(png_ptr, keep, chunk_type);
}
unsafe extern "C" fn opng_init_read_data() {}
unsafe extern "C" fn opng_init_write_data() {
    process.out_file_size = 0 as opng_fsize_t;
    process.out_plte_trns_size = 0 as png_uint_32;
    process.out_idat_size = 0 as opng_fsize_t;
}
unsafe extern "C" fn opng_read_data(
    mut png_ptr: png_structp,
    mut data: png_bytep,
    mut length: size_t,
) {
    let mut stream: *mut FILE = png_get_io_ptr(png_ptr as png_const_structrp) as *mut FILE;
    let mut io_state: ::core::ffi::c_int =
        png_get_io_state(png_ptr as png_const_structrp) as ::core::ffi::c_int;
    let mut io_state_loc: ::core::ffi::c_int = io_state & PNGX_IO_MASK_LOC;
    let mut chunk_sig: png_bytep = ::core::ptr::null_mut::<png_byte>();
    if fread(
        data as *mut ::core::ffi::c_void,
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
    if process.in_file_size == 0 as ::core::ffi::c_ulong {
        if !(length == 8 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"PNG I/O must start with the first 8 bytes\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        process.in_datastream_offset = (opng_ftello(stream) as ::core::ffi::c_long
            - 8 as ::core::ffi::c_long) as opng_foffset_t;
        process.status |= INPUT_HAS_PNG_DATASTREAM as ::core::ffi::c_int as ::core::ffi::c_uint;
        if io_state_loc == PNGX_IO_SIGNATURE {
            process.status |= INPUT_HAS_PNG_SIGNATURE as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
        if process.in_datastream_offset == 0 as ::core::ffi::c_long {
            process.status |= INPUT_IS_PNG_FILE as ::core::ffi::c_int as ::core::ffi::c_uint;
        } else if process.in_datastream_offset < 0 as ::core::ffi::c_long {
            png_error(
                png_ptr as png_const_structrp,
                b"Can't get the file-position indicator in input file\0" as *const u8
                    as png_const_charp,
            );
        }
        process.in_file_size = process.in_datastream_offset as opng_fsize_t;
    }
    process.in_file_size = (process.in_file_size as ::core::ffi::c_ulong)
        .wrapping_add(length as ::core::ffi::c_ulong) as opng_fsize_t
        as opng_fsize_t;
    if !(io_state & 0x1 as ::core::ffi::c_int != 0 && io_state_loc != 0 as ::core::ffi::c_int) {
        usr_panic.expect("non-null function pointer")(
            b"Incorrect info in png_ptr->io_state\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if io_state_loc == PNGX_IO_CHUNK_HDR {
        if !(length == 8 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"Reading chunk header, expecting 8 bytes\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        chunk_sig = data.offset(4 as ::core::ffi::c_int as isize);
        if memcmp(
            chunk_sig as *const ::core::ffi::c_void,
            &raw const sig_IDAT as *const png_byte as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            if !(png_ptr == read_ptr) {
                usr_panic.expect("non-null function pointer")(
                    b"Incorrect I/O handler setup\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if png_get_rows(
                read_ptr as png_const_structrp,
                read_info_ptr as png_const_inforp,
            )
            .is_null()
            {
                if !(process.in_idat_size == 0 as ::core::ffi::c_ulong) {
                    usr_panic.expect("non-null function pointer")(
                        b"Found IDAT with no rows\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                if png_get_image_height(
                    read_ptr as png_const_structrp,
                    read_info_ptr as png_const_inforp,
                ) == 0 as ::core::ffi::c_uint
                {
                    return;
                }
                if pngx_malloc_rows(read_ptr, read_info_ptr, 0 as ::core::ffi::c_int).is_null() {
                    usr_panic.expect("non-null function pointer")(
                        b"Failed allocation of image rows; unsafe libpng allocator\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                png_data_freer(
                    read_ptr as png_const_structrp,
                    read_info_ptr as png_inforp,
                    PNG_USER_WILL_FREE_DATA,
                    PNG_FREE_ROWS,
                );
            } else {
                process.status |= INPUT_HAS_JUNK as ::core::ffi::c_int as ::core::ffi::c_uint;
            }
            process.in_idat_size = (process.in_idat_size as ::core::ffi::c_ulong).wrapping_add(
                ((*data as png_uint_32) << 24 as ::core::ffi::c_int)
                    .wrapping_add(
                        (*data.offset(1 as ::core::ffi::c_int as isize) as png_uint_32)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*data.offset(2 as ::core::ffi::c_int as isize) as png_uint_32)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize) as png_uint_32)
                    as ::core::ffi::c_ulong,
            ) as opng_fsize_t as opng_fsize_t;
        } else if memcmp(
            chunk_sig as *const ::core::ffi::c_void,
            &raw const sig_PLTE as *const png_byte as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
            || memcmp(
                chunk_sig as *const ::core::ffi::c_void,
                &raw const sig_tRNS as *const png_byte as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            process.in_plte_trns_size = (process.in_plte_trns_size as ::core::ffi::c_uint)
                .wrapping_add(
                    ((*data as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                        .wrapping_add(
                            (*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                        )
                        .wrapping_add(12 as ::core::ffi::c_uint),
                ) as png_uint_32 as png_uint_32;
        } else {
            opng_handle_chunk(png_ptr, chunk_sig);
        }
    } else if io_state_loc == PNGX_IO_CHUNK_CRC {
        if !(length == 4 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"Reading chunk CRC, expecting 4 bytes\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn opng_write_data(
    mut png_ptr: png_structp,
    mut data: png_bytep,
    mut length: size_t,
) {
    static mut allow_crt_chunk: ::core::ffi::c_int = 0;
    static mut crt_chunk_is_idat: ::core::ffi::c_int = 0;
    static mut crt_idat_offset: opng_foffset_t = 0;
    static mut crt_idat_size: opng_fsize_t = 0;
    static mut crt_idat_crc: png_uint_32 = 0;
    let mut stream: *mut FILE = png_get_io_ptr(png_ptr as png_const_structrp) as *mut FILE;
    let mut io_state: ::core::ffi::c_int =
        png_get_io_state(png_ptr as png_const_structrp) as ::core::ffi::c_int;
    let mut io_state_loc: ::core::ffi::c_int = io_state & PNGX_IO_MASK_LOC;
    let mut chunk_sig: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut buf: [png_byte; 4] = [0; 4];
    if !(io_state & 0x2 as ::core::ffi::c_int != 0 && io_state_loc != 0 as ::core::ffi::c_int) {
        usr_panic.expect("non-null function pointer")(
            b"Incorrect info in png_ptr->io_state\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if io_state_loc == PNGX_IO_CHUNK_HDR {
        if !(length == 8 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"Writing chunk header, expecting 8 bytes\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        chunk_sig = data.offset(4 as ::core::ffi::c_int as isize);
        allow_crt_chunk = opng_allow_chunk(chunk_sig);
        if memcmp(
            chunk_sig as *const ::core::ffi::c_void,
            &raw const sig_IDAT as *const png_byte as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            crt_chunk_is_idat = 1 as ::core::ffi::c_int;
            process.out_idat_size = (process.out_idat_size as ::core::ffi::c_ulong).wrapping_add(
                ((*data as png_uint_32) << 24 as ::core::ffi::c_int)
                    .wrapping_add(
                        (*data.offset(1 as ::core::ffi::c_int as isize) as png_uint_32)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*data.offset(2 as ::core::ffi::c_int as isize) as png_uint_32)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize) as png_uint_32)
                    as ::core::ffi::c_ulong,
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
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                        longjmp(
                            &raw mut *(*(&raw mut the_exception_context as *mut exception_context))
                                .penv as *mut __jmp_buf_tag,
                            1 as ::core::ffi::c_int,
                        );
                    }
                }
            }
        } else {
            crt_chunk_is_idat = 0 as ::core::ffi::c_int;
            if memcmp(
                chunk_sig as *const ::core::ffi::c_void,
                &raw const sig_PLTE as *const png_byte as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
                || memcmp(
                    chunk_sig as *const ::core::ffi::c_void,
                    &raw const sig_tRNS as *const png_byte as *const ::core::ffi::c_void,
                    4 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                process.out_plte_trns_size = (process.out_plte_trns_size as ::core::ffi::c_uint)
                    .wrapping_add(
                        ((*data as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                            .wrapping_add(
                                (*data.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*data.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(*data.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                            .wrapping_add(12 as ::core::ffi::c_uint),
                    ) as png_uint_32 as png_uint_32;
            }
        }
    } else if io_state_loc == PNGX_IO_CHUNK_CRC {
        if !(length == 4 as size_t) {
            usr_panic.expect("non-null function pointer")(
                b"Writing chunk CRC, expecting 4 bytes\0" as *const u8
                    as *const ::core::ffi::c_char,
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
                if crt_idat_offset == 0 as ::core::ffi::c_long {
                    crt_idat_offset = opng_ftello(stream);
                    if process.best_idat_size > 0 as ::core::ffi::c_ulong {
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
            } else if crt_idat_offset != 0 as ::core::ffi::c_long {
                png_save_uint_32(&raw mut buf as png_bytep, crt_idat_crc);
                if fwrite(
                    &raw mut buf as *mut png_byte as *const ::core::ffi::c_void,
                    1 as size_t,
                    4 as size_t,
                    stream,
                ) != 4 as ::core::ffi::c_ulong
                {
                    io_state = 0 as ::core::ffi::c_int;
                }
                process.out_file_size = (process.out_file_size as ::core::ffi::c_ulong)
                    .wrapping_add(4 as ::core::ffi::c_ulong)
                    as opng_fsize_t as opng_fsize_t;
                if process.out_idat_size != crt_idat_size {
                    if !(process.best_idat_size == 0 as ::core::ffi::c_ulong) {
                        usr_panic.expect("non-null function pointer")(
                            b"Wrong guess of the output IDAT size\0" as *const u8
                                as *const ::core::ffi::c_char,
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
                        &raw mut buf as *mut png_byte as *const ::core::ffi::c_void,
                        4 as size_t,
                    ) != 4 as size_t
                    {
                        io_state = 0 as ::core::ffi::c_int;
                    }
                }
                if io_state == 0 as ::core::ffi::c_int {
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
        data as *const ::core::ffi::c_void,
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
    process.out_file_size = (process.out_file_size as ::core::ffi::c_ulong)
        .wrapping_add(length as ::core::ffi::c_ulong) as opng_fsize_t
        as opng_fsize_t;
}
unsafe extern "C" fn opng_clear_image_info() {
    memset(
        &raw mut image as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<opng_image_struct>() as size_t,
    );
}
unsafe extern "C" fn opng_load_image_info(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut load_meta: ::core::ffi::c_int,
) {
    memset(
        &raw mut image as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
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
}
unsafe extern "C" fn opng_store_image_info(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut store_meta: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    if image.row_pointers.is_null() {
        usr_panic.expect("non-null function pointer")(
            b"No info in image\0" as *const u8 as *const ::core::ffi::c_char,
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
    if image.num_unknowns != 0 as ::core::ffi::c_int {
        png_set_unknown_chunks(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            image.unknowns as png_const_unknown_chunkp,
            image.num_unknowns,
        );
        i = 0 as ::core::ffi::c_int;
        while i < image.num_unknowns {
            png_set_unknown_chunk_location(
                png_ptr as png_const_structrp,
                info_ptr as png_inforp,
                i,
                (*image.unknowns.offset(i as isize)).location as ::core::ffi::c_int,
            );
            i += 1;
        }
    }
}
unsafe extern "C" fn opng_destroy_image_info() {
    let mut i: png_uint_32 = 0;
    let mut j: ::core::ffi::c_int = 0;
    if image.row_pointers.is_null() {
        return;
    }
    i = 0 as png_uint_32;
    while i < image.height {
        opng_free(*image.row_pointers.offset(i as isize) as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    opng_free(image.row_pointers as *mut ::core::ffi::c_void);
    opng_free(image.palette as *mut ::core::ffi::c_void);
    opng_free(image.trans_alpha as *mut ::core::ffi::c_void);
    opng_free(image.hist as *mut ::core::ffi::c_void);
    j = 0 as ::core::ffi::c_int;
    while j < image.num_unknowns {
        opng_free((*image.unknowns.offset(j as isize)).data as *mut ::core::ffi::c_void);
        j += 1;
    }
    opng_free(image.unknowns as *mut ::core::ffi::c_void);
    memset(
        &raw mut image as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<opng_image_struct>() as size_t,
    );
}
unsafe extern "C" fn opng_read_file(mut infile: *mut FILE) {
    let mut fmt_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut num_img: ::core::ffi::c_int = 0;
    let mut reductions: png_uint_32 = 0;
    let mut err_msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
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
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as ::core::ffi::c_int {
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
                        b"Out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as ::core::ffi::c_int,
                    );
                }
            }
            png_set_keep_unknown_chunks(
                read_ptr as png_structrp,
                PNG_HANDLE_CHUNK_ALWAYS,
                ::core::ptr::null::<png_byte>(),
                0 as ::core::ffi::c_int,
            );
            png_set_user_limits(read_ptr as png_structrp, PNG_UINT_31_MAX, PNG_UINT_31_MAX);
            opng_init_read_data();
            png_set_read_fn(
                read_ptr as png_structrp,
                infile as png_voidp,
                Some(opng_read_data as unsafe extern "C" fn(png_structp, png_bytep, size_t) -> ()),
            );
            fmt_name = ::core::ptr::null::<::core::ffi::c_char>();
            num_img = pngx_read_image(
                read_ptr,
                read_info_ptr,
                &raw mut fmt_name,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if num_img <= 0 as ::core::ffi::c_int {
                loop {
                    let ref mut fresh43 = (*(&raw mut the_exception_context
                        as *mut exception_context))
                        .v
                        .etmp;
                    ::core::ptr::write_volatile(
                        fresh43,
                        b"Unrecognized image file format\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as ::core::ffi::c_int,
                    );
                }
            }
            if num_img > 1 as ::core::ffi::c_int {
                process.status |=
                    INPUT_HAS_MULTIPLE_IMAGES as ::core::ffi::c_int as ::core::ffi::c_uint;
            }
            if process.status & INPUT_IS_PNG_FILE as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                && process.status
                    & INPUT_HAS_MULTIPLE_IMAGES as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
            {
                fmt_name = if process.status
                    & INPUT_HAS_PNG_SIGNATURE as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    b"APNG\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"APNG datastream\0" as *const u8 as *const ::core::ffi::c_char
                };
            }
            if fmt_name.is_null() {
                usr_panic.expect("non-null function pointer")(
                    b"No format name from pngxtern\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if process.in_file_size == 0 as ::core::ffi::c_ulong {
                if opng_fgetsize(infile, &raw mut process.in_file_size) < 0 as ::core::ffi::c_int {
                    opng_print_warning(
                        b"Can't get the correct file size\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    process.in_file_size = 0 as opng_fsize_t;
                }
            }
            ::core::ptr::write_volatile(
                &mut err_msg as *mut *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as ::core::ffi::c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as ::core::ffi::c_int;
    }
    let ref mut fresh44 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh44 = exception__prev;
    if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const ::core::ffi::c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as ::core::ffi::c_int != 0
    }) {
        if opng_validate_image(read_ptr, read_info_ptr) != 0 {
            png_warning(read_ptr as png_const_structrp, err_msg as png_const_charp);
            ::core::ptr::write_volatile(
                &mut err_msg as *mut *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
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
    if _setjmp(&raw mut exception__env_0 as *mut __jmp_buf_tag) == 0 as ::core::ffi::c_int {
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
                        1 as ::core::ffi::c_int,
                    );
                }
            }
            if strcmp(
                fmt_name,
                b"PNG\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                usr_printf.expect("non-null function pointer")(
                    b"Importing %s\0" as *const u8 as *const ::core::ffi::c_char,
                    fmt_name,
                );
                if process.status
                    & INPUT_HAS_MULTIPLE_IMAGES as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    if process.status
                        & INPUT_IS_PNG_FILE as ::core::ffi::c_int as ::core::ffi::c_uint
                        == 0
                    {
                        usr_printf.expect("non-null function pointer")(
                            b" (multi-image or animation)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if options.snip != 0 {
                        usr_printf.expect("non-null function pointer")(
                            b"; snipping...\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                }
                usr_printf.expect("non-null function pointer")(
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            opng_load_image_info(read_ptr, read_info_ptr, 1 as ::core::ffi::c_int);
            opng_print_image_info(
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            usr_printf.expect("non-null function pointer")(
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            reductions = (OPNG_REDUCE_ALL & !OPNG_REDUCE_METADATA) as png_uint_32;
            if options.nb != 0 {
                reductions &= !OPNG_REDUCE_BIT_DEPTH as ::core::ffi::c_uint;
            }
            if options.nc != 0 {
                reductions &= !OPNG_REDUCE_COLOR_TYPE as ::core::ffi::c_uint;
            }
            if options.np != 0 {
                reductions &= !OPNG_REDUCE_PALETTE as ::core::ffi::c_uint;
            }
            if options.nz != 0
                && process.status
                    & INPUT_HAS_PNG_DATASTREAM as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
            {
                reductions = OPNG_REDUCE_NONE as png_uint_32;
            }
            if process.status
                & INPUT_HAS_DIGITAL_SIGNATURE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                reductions = OPNG_REDUCE_NONE as png_uint_32;
            }
            if process.status & INPUT_IS_PNG_FILE as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                && process.status
                    & INPUT_HAS_MULTIPLE_IMAGES as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                && reductions != OPNG_REDUCE_NONE as ::core::ffi::c_uint
                && options.snip == 0
            {
                usr_printf
                    .expect(
                        "non-null function pointer",
                    )(
                    b"Can't reliably reduce APNG file; disabling reductions.\n(Did you want to -snip and optimize the first frame?)\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                reductions = OPNG_REDUCE_NONE as png_uint_32;
            }
            process.reductions = opng_reduce_image(read_ptr, read_info_ptr, reductions);
            if process.reductions != OPNG_REDUCE_NONE as ::core::ffi::c_uint {
                opng_load_image_info(read_ptr, read_info_ptr, 1 as ::core::ffi::c_int);
                usr_printf.expect("non-null function pointer")(
                    b"Reducing image to \0" as *const u8 as *const ::core::ffi::c_char,
                );
                opng_print_image_info(
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                usr_printf.expect("non-null function pointer")(
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if options.interlace >= 0 as ::core::ffi::c_int
                && image.interlace_type != options.interlace
            {
                image.interlace_type = options.interlace;
                process.status |=
                    OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint;
            }
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as ::core::ffi::c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as ::core::ffi::c_int;
    }
    let ref mut fresh47 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh47 = exception__prev_0;
    if (*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const ::core::ffi::c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as ::core::ffi::c_int != 0
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
                1 as ::core::ffi::c_int,
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
unsafe extern "C" fn opng_write_file(
    mut outfile: *mut FILE,
    mut compression_level: ::core::ffi::c_int,
    mut memory_level: ::core::ffi::c_int,
    mut compression_strategy: ::core::ffi::c_int,
    mut filter: ::core::ffi::c_int,
) {
    let mut err_msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !(compression_level >= 1 as ::core::ffi::c_int
        && compression_level <= 9 as ::core::ffi::c_int
        && memory_level >= 1 as ::core::ffi::c_int
        && memory_level <= 9 as ::core::ffi::c_int
        && compression_strategy >= 0 as ::core::ffi::c_int
        && compression_strategy <= 3 as ::core::ffi::c_int
        && filter >= 0 as ::core::ffi::c_int
        && filter <= 5 as ::core::ffi::c_int)
    {
        usr_panic.expect("non-null function pointer")(
            b"Invalid encoding parameters\0" as *const u8 as *const ::core::ffi::c_char,
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
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as ::core::ffi::c_int {
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
                        b"Out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as ::core::ffi::c_int,
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
                if options.window_bits > 0 as ::core::ffi::c_int {
                    png_set_compression_window_bits(write_ptr as png_structrp, options.window_bits);
                }
            } else {
                png_set_compression_window_bits(write_ptr as png_structrp, 9 as ::core::ffi::c_int);
            }
            png_set_keep_unknown_chunks(
                write_ptr as png_structrp,
                PNG_HANDLE_CHUNK_ALWAYS,
                ::core::ptr::null::<png_byte>(),
                0 as ::core::ffi::c_int,
            );
            png_set_user_limits(write_ptr as png_structrp, PNG_UINT_31_MAX, PNG_UINT_31_MAX);
            opng_store_image_info(
                write_ptr,
                write_info_ptr,
                (outfile != NULL as *mut FILE) as ::core::ffi::c_int,
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
                0 as ::core::ffi::c_int,
                NULL,
            );
            ::core::ptr::write_volatile(
                &mut err_msg as *mut *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as ::core::ffi::c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as ::core::ffi::c_int;
    }
    let ref mut fresh38 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh38 = exception__prev;
    if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const ::core::ffi::c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as ::core::ffi::c_int != 0
    }) {
        process.out_idat_size = (idat_size_max as ::core::ffi::c_ulong)
            .wrapping_add(1 as ::core::ffi::c_ulong)
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
                1 as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn opng_copy_file(mut infile: *mut FILE, mut outfile: *mut FILE) {
    let mut buf: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let buf_size_incr: png_uint_32 = 0x1000 as png_uint_32;
    let mut buf_size: png_uint_32 = 0;
    let mut length: png_uint_32 = 0;
    let mut chunk_hdr: [png_byte; 8] = [0; 8];
    let mut err_msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
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
                b"Out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as ::core::ffi::c_int,
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
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as ::core::ffi::c_int {
        loop {
            ::core::ptr::write_volatile(
                &mut buf as *mut png_bytep,
                ::core::ptr::null_mut::<png_byte>(),
            );
            buf_size = 0 as png_uint_32;
            png_write_sig(write_ptr as png_structrp);
            loop {
                if fread(
                    &raw mut chunk_hdr as *mut png_byte as *mut ::core::ffi::c_void,
                    8 as size_t,
                    1 as size_t,
                    infile,
                ) != 1 as ::core::ffi::c_ulong
                {
                    loop {
                        let ref mut fresh28 = (*(&raw mut the_exception_context
                            as *mut exception_context))
                            .v
                            .etmp;
                        ::core::ptr::write_volatile(
                            fresh28,
                            b"Read error\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        longjmp(
                            &raw mut *(*(&raw mut the_exception_context as *mut exception_context))
                                .penv as *mut __jmp_buf_tag,
                            1 as ::core::ffi::c_int,
                        );
                    }
                }
                length = ((*(&raw mut chunk_hdr as *mut png_byte) as png_uint_32)
                    << 24 as ::core::ffi::c_int)
                    .wrapping_add(
                        (*(&raw mut chunk_hdr as *mut png_byte)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as png_uint_32)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*(&raw mut chunk_hdr as *mut png_byte)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as png_uint_32)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        *(&raw mut chunk_hdr as *mut png_byte)
                            .offset(3 as ::core::ffi::c_int as isize)
                            as png_uint_32,
                    );
                if length > PNG_UINT_31_MAX {
                    if !(buf.is_null()
                        && length as ::core::ffi::c_ulong == 0x89504e47 as ::core::ffi::c_ulong)
                    {
                        loop {
                            let ref mut fresh29 = (*(&raw mut the_exception_context
                                as *mut exception_context))
                                .v
                                .etmp;
                            ::core::ptr::write_volatile(
                                fresh29,
                                b"Data error\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            longjmp(
                                &raw mut *(*(&raw mut the_exception_context
                                    as *mut exception_context))
                                    .penv as *mut __jmp_buf_tag,
                                1 as ::core::ffi::c_int,
                            );
                        }
                    }
                } else {
                    if (length as ::core::ffi::c_uint).wrapping_add(4 as ::core::ffi::c_uint)
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
                        buf as *mut ::core::ffi::c_void,
                        (length as ::core::ffi::c_uint).wrapping_add(4 as ::core::ffi::c_uint)
                            as size_t,
                        1 as size_t,
                        infile,
                    ) != 1 as ::core::ffi::c_ulong
                    {
                        loop {
                            let ref mut fresh30 = (*(&raw mut the_exception_context
                                as *mut exception_context))
                                .v
                                .etmp;
                            ::core::ptr::write_volatile(
                                fresh30,
                                b"Read error\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            longjmp(
                                &raw mut *(*(&raw mut the_exception_context
                                    as *mut exception_context))
                                    .penv as *mut __jmp_buf_tag,
                                1 as ::core::ffi::c_int,
                            );
                        }
                    }
                    png_write_chunk(
                        write_ptr as png_structrp,
                        (&raw mut chunk_hdr as *mut png_byte)
                            .offset(4 as ::core::ffi::c_int as isize)
                            as png_const_bytep,
                        buf as png_const_bytep,
                        length as png_size_t,
                    );
                }
                if !(memcmp(
                    (&raw mut chunk_hdr as *mut png_byte).offset(4 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    &raw const sig_IEND as *const png_byte as *const ::core::ffi::c_void,
                    4 as size_t,
                ) != 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            ::core::ptr::write_volatile(
                &mut err_msg as *mut *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as ::core::ffi::c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as ::core::ffi::c_int;
    }
    let ref mut fresh31 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh31 = exception__prev;
    (*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const ::core::ffi::c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as ::core::ffi::c_int != 0
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
                1 as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn opng_init_iteration(
    mut cmdline_set: opng_bitset_t,
    mut mask_set: opng_bitset_t,
    mut preset: *const ::core::ffi::c_char,
    mut output_set: *mut opng_bitset_t,
) {
    let mut preset_set: opng_bitset_t = 0;
    let mut check: ::core::ffi::c_int = 0;
    *output_set = cmdline_set & mask_set;
    if *output_set == 0 as ::core::ffi::c_uint && cmdline_set != 0 as ::core::ffi::c_uint {
        loop {
            let ref mut fresh40 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(
                fresh40,
                b"Iteration parameter(s) out of range\0" as *const u8 as *const ::core::ffi::c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as ::core::ffi::c_int,
            );
        }
    }
    if *output_set == 0 as ::core::ffi::c_uint || options.optim_level >= 0 as ::core::ffi::c_int {
        check = opng_strparse_rangeset_to_bitset(&raw mut preset_set, preset, mask_set);
        if !(check == 0 as ::core::ffi::c_int) {
            usr_panic.expect("non-null function pointer")(
                b"[internal] Invalid preset\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        *output_set |= (preset_set & mask_set) as ::core::ffi::c_uint;
    }
}
unsafe extern "C" fn opng_init_iterations() {
    let mut compr_level_set: opng_bitset_t = 0;
    let mut mem_level_set: opng_bitset_t = 0;
    let mut strategy_set: opng_bitset_t = 0;
    let mut filter_set: opng_bitset_t = 0;
    let mut strategy_singles_set: opng_bitset_t = 0;
    let mut preset_index: ::core::ffi::c_int = 0;
    let mut t1: ::core::ffi::c_int = 0;
    let mut t2: ::core::ffi::c_int = 0;
    if process.status & OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        || options.full != 0
    {
        process.max_idat_size = idat_size_max;
    } else {
        if !(process.in_idat_size > 0 as ::core::ffi::c_ulong) {
            usr_panic.expect("non-null function pointer")(
                b"No IDAT in input\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        process.max_idat_size = (process.in_idat_size as ::core::ffi::c_ulong)
            .wrapping_add(process.in_plte_trns_size as ::core::ffi::c_ulong)
            as opng_fsize_t;
    }
    preset_index = options.optim_level;
    if preset_index < 0 as ::core::ffi::c_int {
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
    if compr_level_set == 0 as ::core::ffi::c_uint {
        compr_level_set |= (1 as ::core::ffi::c_uint) << 9 as ::core::ffi::c_int;
    }
    if mem_level_set == 0 as ::core::ffi::c_uint {
        mem_level_set |= (1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int;
    }
    if image.bit_depth < 8 as ::core::ffi::c_int || !image.palette.is_null() {
        if strategy_set == 0 as ::core::ffi::c_uint {
            strategy_set |= (1 as ::core::ffi::c_uint) << 0 as ::core::ffi::c_int;
        }
        if filter_set == 0 as ::core::ffi::c_uint {
            filter_set |= (1 as ::core::ffi::c_uint) << 0 as ::core::ffi::c_int;
        }
    } else {
        if strategy_set == 0 as ::core::ffi::c_uint {
            strategy_set |= (1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int;
        }
        if filter_set == 0 as ::core::ffi::c_uint {
            filter_set |= (1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int;
        }
    }
    process.compr_level_set = compr_level_set;
    process.mem_level_set = mem_level_set;
    process.strategy_set = strategy_set;
    process.filter_set = filter_set;
    strategy_singles_set = ((1 as ::core::ffi::c_int) << Z_HUFFMAN_ONLY
        | (1 as ::core::ffi::c_int) << Z_RLE) as opng_bitset_t;
    t1 = opng_bitset_count(compr_level_set)
        .wrapping_mul(opng_bitset_count(strategy_set & !strategy_singles_set))
        as ::core::ffi::c_int;
    t2 = opng_bitset_count(strategy_set & strategy_singles_set) as ::core::ffi::c_int;
    process.num_iterations = ((t1 + t2) as ::core::ffi::c_uint)
        .wrapping_mul(opng_bitset_count(mem_level_set))
        .wrapping_mul(opng_bitset_count(filter_set))
        as ::core::ffi::c_int;
    if !(process.num_iterations > 0 as ::core::ffi::c_int) {
        usr_panic.expect("non-null function pointer")(
            b"Invalid iteration parameters\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn opng_iterate() {
    let mut compr_level_set: opng_bitset_t = 0;
    let mut mem_level_set: opng_bitset_t = 0;
    let mut strategy_set: opng_bitset_t = 0;
    let mut filter_set: opng_bitset_t = 0;
    let mut compr_level: ::core::ffi::c_int = 0;
    let mut mem_level: ::core::ffi::c_int = 0;
    let mut strategy: ::core::ffi::c_int = 0;
    let mut filter: ::core::ffi::c_int = 0;
    let mut counter: ::core::ffi::c_int = 0;
    let mut line_reused: ::core::ffi::c_int = 0;
    if !(process.num_iterations > 0 as ::core::ffi::c_int) {
        usr_panic.expect("non-null function pointer")(
            b"Iterations not initialized\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    compr_level_set = process.compr_level_set;
    mem_level_set = process.mem_level_set;
    strategy_set = process.strategy_set;
    filter_set = process.filter_set;
    if process.num_iterations == 1 as ::core::ffi::c_int
        && process.status & OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        process.best_idat_size = 0 as opng_fsize_t;
        process.best_compr_level = opng_bitset_find_first(compr_level_set);
        process.best_mem_level = opng_bitset_find_first(mem_level_set);
        process.best_strategy = opng_bitset_find_first(strategy_set);
        process.best_filter = opng_bitset_find_first(filter_set);
        return;
    }
    process.best_idat_size = (idat_size_max as ::core::ffi::c_ulong)
        .wrapping_add(1 as ::core::ffi::c_ulong) as opng_fsize_t;
    process.best_compr_level = -(1 as ::core::ffi::c_int);
    process.best_mem_level = -(1 as ::core::ffi::c_int);
    process.best_strategy = -(1 as ::core::ffi::c_int);
    process.best_filter = -(1 as ::core::ffi::c_int);
    usr_printf.expect("non-null function pointer")(
        b"\nTrying:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    line_reused = 0 as ::core::ffi::c_int;
    counter = 0 as ::core::ffi::c_int;
    filter = OPNG_FILTER_MIN;
    while filter <= OPNG_FILTER_MAX {
        if filter_set as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << filter
            != 0 as ::core::ffi::c_uint
        {
            strategy = OPNG_STRATEGY_MIN;
            while strategy <= OPNG_STRATEGY_MAX {
                if strategy_set as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << strategy
                    != 0 as ::core::ffi::c_uint
                {
                    if strategy == Z_HUFFMAN_ONLY {
                        compr_level_set = 0 as opng_bitset_t;
                        compr_level_set |= (1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int;
                    } else if strategy == Z_RLE {
                        compr_level_set = 0 as opng_bitset_t;
                        compr_level_set |= (1 as ::core::ffi::c_uint) << 9 as ::core::ffi::c_int;
                    } else {
                        compr_level_set = process.compr_level_set;
                    }
                    compr_level = OPNG_COMPR_LEVEL_MAX;
                    while compr_level >= OPNG_COMPR_LEVEL_MIN {
                        if compr_level_set as ::core::ffi::c_uint
                            & (1 as ::core::ffi::c_uint) << compr_level
                            != 0 as ::core::ffi::c_uint
                        {
                            mem_level = OPNG_MEM_LEVEL_MAX;
                            while mem_level >= OPNG_MEM_LEVEL_MIN {
                                if mem_level_set as ::core::ffi::c_uint
                                    & (1 as ::core::ffi::c_uint) << mem_level
                                    != 0 as ::core::ffi::c_uint
                                {
                                    usr_printf.expect("non-null function pointer")(
                                        b"  zc = %d  zm = %d  zs = %d  f = %d\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        compr_level,
                                        mem_level,
                                        strategy,
                                        filter,
                                    );
                                    usr_progress.expect("non-null function pointer")(
                                        counter as ::core::ffi::c_ulong,
                                        process.num_iterations as ::core::ffi::c_ulong,
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
                                                    as *const ::core::ffi::c_char,
                                            );
                                            line_reused = 0 as ::core::ffi::c_int;
                                        } else {
                                            usr_print_cntrl.expect("non-null function pointer")(
                                                '\r' as i32,
                                            );
                                            line_reused = 1 as ::core::ffi::c_int;
                                        }
                                    } else {
                                        usr_printf.expect("non-null function pointer")(
                                            b"\t\tIDAT size = %lu\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            process.out_idat_size,
                                        );
                                        line_reused = 0 as ::core::ffi::c_int;
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
        usr_print_cntrl.expect("non-null function pointer")(-(31 as ::core::ffi::c_int));
    }
    if !(counter == process.num_iterations) {
        usr_panic.expect("non-null function pointer")(
            b"Inconsistent iteration counter\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    usr_progress.expect("non-null function pointer")(
        counter as ::core::ffi::c_ulong,
        process.num_iterations as ::core::ffi::c_ulong,
    );
}
unsafe extern "C" fn opng_finish_iterations() {
    if (process.best_idat_size as ::core::ffi::c_ulong)
        .wrapping_add(process.out_plte_trns_size as ::core::ffi::c_ulong)
        < (process.in_idat_size as ::core::ffi::c_ulong)
            .wrapping_add(process.in_plte_trns_size as ::core::ffi::c_ulong)
    {
        process.status |= OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if process.status & OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        if process.best_idat_size <= idat_size_max {
            usr_printf.expect("non-null function pointer")(
                b"\nSelecting parameters:\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            usr_printf.expect("non-null function pointer")(
                b"  zc = %d  zm = %d  zs = %d  f = %d\0" as *const u8 as *const ::core::ffi::c_char,
                process.best_compr_level,
                process.best_mem_level,
                process.best_strategy,
                process.best_filter,
            );
            if process.best_idat_size > 0 as ::core::ffi::c_ulong {
                usr_printf.expect("non-null function pointer")(
                    b"\t\tIDAT size = %lu\0" as *const u8 as *const ::core::ffi::c_char,
                    process.best_idat_size,
                );
            }
            usr_printf.expect("non-null function pointer")(
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            usr_printf.expect("non-null function pointer")(
                b"  zc = *  zm = *  zs = *  f = *\t\tIDAT size > %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                idat_size_max_string,
            );
        }
    }
}
unsafe extern "C" fn opng_optimize_impl(mut infile_name: *const ::core::ffi::c_char) {
    static mut infile: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
    static mut outfile: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
    static mut infile_name_local: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    static mut outfile_name: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    static mut bakfile_name: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    static mut new_outfile: ::core::ffi::c_int = 0;
    static mut has_backup: ::core::ffi::c_int = 0;
    let mut name_buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut tmp_buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut err_msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    memset(
        &raw mut process as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<opng_process_struct>() as size_t,
    );
    if options.force != 0 {
        process.status |= OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    ::core::ptr::write_volatile(
        &mut err_msg as *mut *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    infile_name_local = infile_name;
    infile = fopen(
        infile_name_local,
        b"rb\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if infile.is_null() {
        loop {
            let ref mut fresh2 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(
                fresh2,
                b"Can't open the input file\0" as *const u8 as *const ::core::ffi::c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as ::core::ffi::c_int,
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
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as ::core::ffi::c_int {
        loop {
            opng_read_file(infile);
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as ::core::ffi::c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as ::core::ffi::c_int;
    }
    let ref mut fresh4 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh4 = exception__prev;
    if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const ::core::ffi::c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as ::core::ffi::c_int != 0
    }) {
        if err_msg.is_null() {
            usr_panic.expect("non-null function pointer")(
                b"Mysterious error in opng_read_file\0" as *const u8 as *const ::core::ffi::c_char,
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
                1 as ::core::ffi::c_int,
            );
        }
    }
    if process.status & INPUT_HAS_ERRORS as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        usr_printf.expect("non-null function pointer")(
            b"Recoverable errors found in input.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if options.fix != 0 {
            usr_printf.expect("non-null function pointer")(
                b" Fixing...\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            process.status |= OUTPUT_NEEDS_NEW_FILE as ::core::ffi::c_int as ::core::ffi::c_uint;
        } else {
            usr_printf.expect("non-null function pointer")(
                b" Rerun OptiPNG with -fix enabled.\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            loop {
                let ref mut fresh6 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh6,
                    b"Previous error(s) not fixed\0" as *const u8 as *const ::core::ffi::c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as ::core::ffi::c_int,
                );
            }
        }
    }
    if process.status & INPUT_HAS_JUNK as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        process.status |= OUTPUT_NEEDS_NEW_FILE as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if process.status & INPUT_HAS_PNG_SIGNATURE as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
        process.status |= OUTPUT_NEEDS_NEW_FILE as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if process.status & INPUT_HAS_PNG_DATASTREAM as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        if options.nz != 0
            && process.status & OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
        {
            usr_printf.expect("non-null function pointer")(
                b"IDAT recoding is necessary, but is disabled by the user.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            loop {
                let ref mut fresh7 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh7,
                    b"Can't continue\0" as *const u8 as *const ::core::ffi::c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as ::core::ffi::c_int,
                );
            }
        }
    } else {
        process.status |= OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if process.status & INPUT_HAS_DIGITAL_SIGNATURE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        usr_printf.expect("non-null function pointer")(
            b"Digital signature found in input.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if options.force != 0 {
            usr_printf.expect("non-null function pointer")(
                b" Erasing...\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            process.status |= OUTPUT_NEEDS_NEW_FILE as ::core::ffi::c_int as ::core::ffi::c_uint;
        } else {
            usr_printf.expect("non-null function pointer")(
                b" Rerun OptiPNG with -force enabled.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            loop {
                let ref mut fresh8 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh8,
                    b"Can't optimize digitally-signed files\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as ::core::ffi::c_int,
                );
            }
        }
    }
    if process.status & INPUT_HAS_MULTIPLE_IMAGES as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        if options.snip == 0
            && process.status & INPUT_IS_PNG_FILE as ::core::ffi::c_int as ::core::ffi::c_uint == 0
        {
            usr_printf.expect("non-null function pointer")(
                b"Conversion to PNG requires snipping. Rerun OptiPNG with -snip enabled.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            loop {
                let ref mut fresh9 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh9,
                    b"Incompatible input format\0" as *const u8 as *const ::core::ffi::c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as ::core::ffi::c_int,
                );
            }
        }
    }
    if process.status & INPUT_HAS_APNG as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && options.snip != 0
    {
        process.status |= OUTPUT_NEEDS_NEW_FILE as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if process.status & INPUT_HAS_STRIPPED_DATA as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        usr_printf.expect("non-null function pointer")(
            b"Stripping metadata...\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    outfile_name = ::core::ptr::null::<::core::ffi::c_char>();
    if process.status & INPUT_IS_PNG_FILE as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
        if opng_path_replace_ext(
            &raw mut name_buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            infile_name_local,
            b".png\0" as *const u8 as *const ::core::ffi::c_char,
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
                        as *const ::core::ffi::c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        outfile_name = &raw mut name_buf as *mut ::core::ffi::c_char;
    }
    if !options.out_name.is_null() {
        outfile_name = options.out_name;
    }
    if !options.dir_name.is_null() {
        let mut tmp_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if !outfile_name.is_null() {
            strcpy(&raw mut tmp_buf as *mut ::core::ffi::c_char, outfile_name);
            tmp_name = &raw mut tmp_buf as *mut ::core::ffi::c_char;
        } else {
            tmp_name = infile_name_local;
        }
        if opng_path_replace_dir(
            &raw mut name_buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
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
                        as *const ::core::ffi::c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        outfile_name = &raw mut name_buf as *mut ::core::ffi::c_char;
    }
    if outfile_name.is_null() {
        outfile_name = infile_name_local;
        new_outfile = 0 as ::core::ffi::c_int;
    } else {
        let mut test_eq: ::core::ffi::c_int = opng_os_test_eq(infile_name_local, outfile_name);
        if test_eq >= 0 as ::core::ffi::c_int {
            new_outfile = (test_eq == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        } else {
            new_outfile = (strcmp(infile_name_local, outfile_name) != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        }
    }
    bakfile_name = &raw mut tmp_buf as *mut ::core::ffi::c_char;
    if new_outfile != 0 {
        if opng_path_make_backup(
            &raw mut tmp_buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            outfile_name,
        )
        .is_null()
        {
            bakfile_name = ::core::ptr::null::<::core::ffi::c_char>();
        }
    } else if opng_path_make_backup(
        &raw mut tmp_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        infile_name_local,
    )
    .is_null()
    {
        bakfile_name = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if bakfile_name.is_null() {
        loop {
            let ref mut fresh12 = (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp;
            ::core::ptr::write_volatile(
                fresh12,
                b"Can't create backup file (name too long)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            longjmp(
                &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                    as *mut __jmp_buf_tag,
                1 as ::core::ffi::c_int,
            );
        }
    }
    if options.simulate == 0
        && opng_os_test(
            outfile_name,
            b"e\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        if new_outfile != 0 && options.backup == 0 && options.clobber == 0 {
            usr_printf.expect("non-null function pointer")(
                b"The output file exists. Rerun OptiPNG with -backup enabled.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            loop {
                let ref mut fresh13 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh13,
                    b"Can't overwrite the output file\0" as *const u8 as *const ::core::ffi::c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        if opng_os_test(
            outfile_name,
            b"fw\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            || options.clobber == 0
                && opng_os_test(
                    bakfile_name,
                    b"e\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
        {
            loop {
                let ref mut fresh14 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh14,
                    b"Can't back up the existing output file\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as ::core::ffi::c_int,
                );
            }
        }
    }
    if process.status & INPUT_HAS_PNG_DATASTREAM as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        usr_printf.expect("non-null function pointer")(
            b"Input IDAT size = %lu bytes\n\0" as *const u8 as *const ::core::ffi::c_char,
            process.in_idat_size,
        );
    }
    usr_printf.expect("non-null function pointer")(
        b"Input file size = %lu bytes\n\0" as *const u8 as *const ::core::ffi::c_char,
        process.in_file_size,
    );
    if options.nz == 0
        || process.status & OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        opng_init_iterations();
        opng_iterate();
        opng_finish_iterations();
    }
    if process.status & OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        process.status |= OUTPUT_NEEDS_NEW_FILE as ::core::ffi::c_int as ::core::ffi::c_uint;
        opng_check_idat_size(process.best_idat_size);
    }
    if process.status & OUTPUT_NEEDS_NEW_FILE as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
        usr_printf.expect("non-null function pointer")(
            b"\n%s is already optimized.\n\0" as *const u8 as *const ::core::ffi::c_char,
            infile_name_local,
        );
        if new_outfile == 0 {
            return;
        }
    }
    if options.simulate != 0 {
        usr_printf.expect("non-null function pointer")(
            b"\nNo output: simulation mode.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if new_outfile != 0 {
        usr_printf.expect("non-null function pointer")(
            b"\nOutput file: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            outfile_name,
        );
        if !options.dir_name.is_null() {
            opng_os_create_dir(options.dir_name);
        }
        has_backup = 0 as ::core::ffi::c_int;
        if opng_os_test(
            outfile_name,
            b"e\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if opng_os_rename(outfile_name, bakfile_name, options.clobber)
                != 0 as ::core::ffi::c_int
            {
                loop {
                    let ref mut fresh15 = (*(&raw mut the_exception_context
                        as *mut exception_context))
                        .v
                        .etmp;
                    ::core::ptr::write_volatile(
                        fresh15,
                        b"Can't back up the output file\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as ::core::ffi::c_int,
                    );
                }
            }
            has_backup = 1 as ::core::ffi::c_int;
        }
    } else {
        if opng_os_rename(infile_name_local, bakfile_name, options.clobber)
            != 0 as ::core::ffi::c_int
        {
            loop {
                let ref mut fresh16 = (*(&raw mut the_exception_context as *mut exception_context))
                    .v
                    .etmp;
                ::core::ptr::write_volatile(
                    fresh16,
                    b"Can't back up the input file\0" as *const u8 as *const ::core::ffi::c_char,
                );
                longjmp(
                    &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                        as *mut __jmp_buf_tag,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        has_backup = 1 as ::core::ffi::c_int;
    }
    outfile = fopen(
        outfile_name,
        b"wb\0" as *const u8 as *const ::core::ffi::c_char,
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
    if _setjmp(&raw mut exception__env_0 as *mut __jmp_buf_tag) == 0 as ::core::ffi::c_int {
        loop {
            if outfile.is_null() {
                loop {
                    let ref mut fresh18 = (*(&raw mut the_exception_context
                        as *mut exception_context))
                        .v
                        .etmp;
                    ::core::ptr::write_volatile(
                        fresh18,
                        b"Can't open the output file\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    longjmp(
                        &raw mut *(*(&raw mut the_exception_context as *mut exception_context)).penv
                            as *mut __jmp_buf_tag,
                        1 as ::core::ffi::c_int,
                    );
                }
            }
            if process.status & OUTPUT_NEEDS_NEW_IDAT as ::core::ffi::c_int as ::core::ffi::c_uint
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
                    b"rb\0" as *const u8 as *const ::core::ffi::c_char,
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
                                as *const ::core::ffi::c_char,
                        );
                        longjmp(
                            &raw mut *(*(&raw mut the_exception_context as *mut exception_context))
                                .penv as *mut __jmp_buf_tag,
                            1 as ::core::ffi::c_int,
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
                    == 0 as ::core::ffi::c_int
                {
                    loop {
                        if process.in_datastream_offset > 0 as ::core::ffi::c_long
                            && opng_fseeko(infile, process.in_datastream_offset, SEEK_SET)
                                != 0 as ::core::ffi::c_int
                        {
                            loop {
                                let ref mut fresh21 = (*(&raw mut the_exception_context
                                    as *mut exception_context))
                                    .v
                                    .etmp;
                                ::core::ptr::write_volatile(
                                    fresh21,
                                    b"Can't reposition the input file\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                longjmp(
                                    &raw mut *(*(&raw mut the_exception_context
                                        as *mut exception_context))
                                        .penv
                                        as *mut __jmp_buf_tag,
                                    1 as ::core::ffi::c_int,
                                );
                            }
                        }
                        process.best_idat_size = process.in_idat_size;
                        opng_copy_file(infile, outfile);
                        (*(&raw mut the_exception_context as *mut exception_context)).caught =
                            0 as ::core::ffi::c_int;
                        if !((*(&raw mut the_exception_context as *mut exception_context)).caught
                            != 0)
                        {
                            break;
                        }
                    }
                } else {
                    (*(&raw mut the_exception_context as *mut exception_context)).caught =
                        1 as ::core::ffi::c_int;
                }
                let ref mut fresh22 =
                    (*(&raw mut the_exception_context as *mut exception_context)).penv;
                *fresh22 = exception__prev_1;
                if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
                    ::core::ptr::write_volatile(
                        &mut err_msg as *mut *const ::core::ffi::c_char,
                        (*(&raw mut the_exception_context as *mut exception_context))
                            .v
                            .etmp,
                    );
                    0 as ::core::ffi::c_int != 0
                }) {
                    if err_msg.is_null() {
                        usr_panic.expect("non-null function pointer")(
                            b"Mysterious error in opng_copy_file\0" as *const u8
                                as *const ::core::ffi::c_char,
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
                            1 as ::core::ffi::c_int,
                        );
                    }
                }
            }
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as ::core::ffi::c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as ::core::ffi::c_int;
    }
    let ref mut fresh24 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh24 = exception__prev_0;
    if (*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        ::core::ptr::write_volatile(
            &mut err_msg as *mut *const ::core::ffi::c_char,
            (*(&raw mut the_exception_context as *mut exception_context))
                .v
                .etmp,
        );
        0 as ::core::ffi::c_int != 0
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
                1 as ::core::ffi::c_int,
            ) != 0 as ::core::ffi::c_int
            {
                opng_print_warning(
                    b"Can't recover the original file from backup\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        } else {
            if new_outfile == 0 {
                usr_panic.expect("non-null function pointer")(
                    b"Overwrote input with no temporary backup\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if opng_os_unlink(outfile_name) != 0 as ::core::ffi::c_int {
                opng_print_warning(
                    b"Can't remove the broken output file\0" as *const u8
                        as *const ::core::ffi::c_char,
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
                1 as ::core::ffi::c_int,
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
        if opng_os_unlink(bakfile_name) != 0 as ::core::ffi::c_int {
            opng_print_warning(
                b"Can't remove the backup file\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    usr_printf.expect("non-null function pointer")(
        b"\nOutput IDAT size = %lu bytes\0" as *const u8 as *const ::core::ffi::c_char,
        process.out_idat_size,
    );
    if process.status & INPUT_HAS_PNG_DATASTREAM as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        usr_printf.expect("non-null function pointer")(
            b" (\0" as *const u8 as *const ::core::ffi::c_char,
        );
        opng_print_fsize_difference(
            process.in_idat_size,
            process.out_idat_size,
            0 as ::core::ffi::c_int,
        );
        usr_printf.expect("non-null function pointer")(
            b")\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    usr_printf.expect("non-null function pointer")(
        b"\nOutput file size = %lu bytes (\0" as *const u8 as *const ::core::ffi::c_char,
        process.out_file_size,
    );
    opng_print_fsize_difference(
        process.in_file_size,
        process.out_file_size,
        1 as ::core::ffi::c_int,
    );
    usr_printf.expect("non-null function pointer")(
        b")\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn opng_initialize(
    mut init_options: *const opng_options,
    mut init_ui: *const opng_ui,
) -> ::core::ffi::c_int {
    usr_printf = (*init_ui).printf_fn;
    usr_print_cntrl = (*init_ui).print_cntrl_fn;
    usr_progress = (*init_ui).progress_fn;
    usr_panic = (*init_ui).panic_fn;
    if usr_printf.is_none()
        || usr_print_cntrl.is_none()
        || usr_progress.is_none()
        || usr_panic.is_none()
    {
        return -(1 as ::core::ffi::c_int);
    }
    options = *init_options;
    if options.optim_level == 0 as ::core::ffi::c_int {
        options.np = 1 as ::core::ffi::c_int;
        options.nc = options.np;
        options.nb = options.nc;
        options.nz = 1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut summary as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<opng_summary_struct>() as size_t,
    );
    engine.started = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opng_optimize(
    mut infile_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut err_msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut result: ::core::ffi::c_int = 0;
    if engine.started == 0 {
        usr_panic.expect("non-null function pointer")(
            b"The OptiPNG engine is not running\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    usr_printf.expect("non-null function pointer")(
        b"** Processing: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
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
    if _setjmp(&raw mut exception__env as *mut __jmp_buf_tag) == 0 as ::core::ffi::c_int {
        loop {
            opng_optimize_impl(infile_name);
            if process.status & INPUT_HAS_ERRORS as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
                summary.err_count = summary.err_count.wrapping_add(1);
                summary.fix_count = summary.fix_count.wrapping_add(1);
            }
            if process.status
                & INPUT_HAS_MULTIPLE_IMAGES as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                if options.snip != 0 {
                    summary.snip_count = summary.snip_count.wrapping_add(1);
                }
            }
            ::core::ptr::write_volatile(
                &mut result as *mut ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            (*(&raw mut the_exception_context as *mut exception_context)).caught =
                0 as ::core::ffi::c_int;
            if !((*(&raw mut the_exception_context as *mut exception_context)).caught != 0) {
                break;
            }
        }
    } else {
        (*(&raw mut the_exception_context as *mut exception_context)).caught =
            1 as ::core::ffi::c_int;
    }
    let ref mut fresh1 = (*(&raw mut the_exception_context as *mut exception_context)).penv;
    *fresh1 = exception__prev;
    if !((*(&raw mut the_exception_context as *mut exception_context)).caught == 0 || {
        err_msg = (*(&raw mut the_exception_context as *mut exception_context))
            .v
            .etmp;
        0 as ::core::ffi::c_int != 0
    }) {
        summary.err_count = summary.err_count.wrapping_add(1);
        opng_print_error(err_msg);
        ::core::ptr::write_volatile(
            &mut result as *mut ::core::ffi::c_int,
            -(1 as ::core::ffi::c_int),
        );
    }
    opng_destroy_image_info();
    usr_printf.expect("non-null function pointer")(
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_finalize() -> ::core::ffi::c_int {
    if options.verbose != 0
        || summary.snip_count > 0 as ::core::ffi::c_uint
        || summary.err_count > 0 as ::core::ffi::c_uint
    {
        usr_printf.expect("non-null function pointer")(
            b"** Status report\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        usr_printf.expect("non-null function pointer")(
            b"%u file(s) have been processed.\n\0" as *const u8 as *const ::core::ffi::c_char,
            summary.file_count,
        );
        if summary.snip_count > 0 as ::core::ffi::c_uint {
            usr_printf.expect("non-null function pointer")(
                b"%u multi-image file(s) have been snipped.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                summary.snip_count,
            );
        }
        if summary.err_count > 0 as ::core::ffi::c_uint {
            usr_printf.expect("non-null function pointer")(
                b"%u error(s) have been encountered.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                summary.err_count,
            );
            if summary.fix_count > 0 as ::core::ffi::c_uint {
                usr_printf.expect("non-null function pointer")(
                    b"%u erroneous file(s) have been fixed.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    summary.fix_count,
                );
            }
        }
    }
    engine.started = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
pub const PNG_LIBPNG_VER_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"1.6.34\0") };
pub const PNG_UINT_31_MAX: png_uint_32 = 0x7fffffff as ::core::ffi::c_long as png_uint_32;
pub const PNG_COLOR_MASK_PALETTE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_COLOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_FILTER_TYPE_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_INTERLACE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_FILTER_NONE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const PNG_FILTER_SUB: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const PNG_FILTER_UP: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const PNG_FILTER_AVG: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const PNG_FILTER_PAETH: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const PNG_FAST_FILTERS: ::core::ffi::c_int = PNG_FILTER_NONE | PNG_FILTER_SUB | PNG_FILTER_UP;
pub const PNG_ALL_FILTERS: ::core::ffi::c_int =
    PNG_FAST_FILTERS | PNG_FILTER_AVG | PNG_FILTER_PAETH;
pub const PNG_DESTROY_WILL_FREE_DATA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_USER_WILL_FREE_DATA: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_FREE_ROWS: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const PNG_FREE_ALL: ::core::ffi::c_uint = 0xffff as ::core::ffi::c_uint;
pub const PNG_HANDLE_CHUNK_NEVER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_HANDLE_CHUNK_ALWAYS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PNG_IO_SIGNATURE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const PNG_IO_CHUNK_HDR: ::core::ffi::c_int = 32;
pub const PNG_IO_CHUNK_DATA: ::core::ffi::c_int = 64;
pub const PNG_IO_CHUNK_CRC: ::core::ffi::c_int = 128;
pub const PNG_IO_MASK_LOC: ::core::ffi::c_int = 0xf0 as ::core::ffi::c_int;
pub const OPNG_REDUCE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPNG_REDUCE_16_TO_8: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPNG_REDUCE_8_TO_4_2_1: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPNG_REDUCE_RGB_TO_GRAY: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OPNG_REDUCE_STRIP_ALPHA: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPNG_REDUCE_RGB_TO_PALETTE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPNG_REDUCE_PALETTE_TO_RGB: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const OPNG_REDUCE_GRAY_TO_PALETTE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const OPNG_REDUCE_PALETTE_TO_GRAY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const OPNG_REDUCE_PALETTE_SLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const OPNG_REDUCE_PALETTE_FAST: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const OPNG_REDUCE_METADATA: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const OPNG_REDUCE_BIT_DEPTH: ::core::ffi::c_int = OPNG_REDUCE_16_TO_8 | OPNG_REDUCE_8_TO_4_2_1;
pub const OPNG_REDUCE_COLOR_TYPE: ::core::ffi::c_int = OPNG_REDUCE_RGB_TO_GRAY
    | OPNG_REDUCE_STRIP_ALPHA
    | OPNG_REDUCE_RGB_TO_PALETTE
    | OPNG_REDUCE_PALETTE_TO_RGB
    | OPNG_REDUCE_GRAY_TO_PALETTE
    | OPNG_REDUCE_PALETTE_TO_GRAY;
pub const OPNG_REDUCE_PALETTE: ::core::ffi::c_int =
    OPNG_REDUCE_PALETTE_SLOW | OPNG_REDUCE_PALETTE_FAST;
pub const OPNG_REDUCE_ALL: ::core::ffi::c_int =
    OPNG_REDUCE_BIT_DEPTH | OPNG_REDUCE_COLOR_TYPE | OPNG_REDUCE_PALETTE | OPNG_REDUCE_METADATA;
pub const PNGX_IO_SIGNATURE: ::core::ffi::c_int = PNG_IO_SIGNATURE;
pub const PNGX_IO_CHUNK_HDR: ::core::ffi::c_int = PNG_IO_CHUNK_HDR;
pub const PNGX_IO_CHUNK_DATA: ::core::ffi::c_int = PNG_IO_CHUNK_DATA;
pub const PNGX_IO_CHUNK_CRC: ::core::ffi::c_int = PNG_IO_CHUNK_CRC;
pub const PNGX_IO_MASK_LOC: ::core::ffi::c_int = PNG_IO_MASK_LOC;
pub const Z_HUFFMAN_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const Z_RLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
