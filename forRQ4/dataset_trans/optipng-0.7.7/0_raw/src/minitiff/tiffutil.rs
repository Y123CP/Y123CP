extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
pub struct minitiff_info {
    pub error_handler: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>,
    pub warning_handler: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>,
    pub byte_order: ::core::ffi::c_int,
    pub width: size_t,
    pub height: size_t,
    pub bits_per_sample: ::core::ffi::c_uint,
    pub compression: ::core::ffi::c_uint,
    pub photometric: ::core::ffi::c_uint,
    pub strip_offsets_count: size_t,
    pub strip_offsets: *mut ::core::ffi::c_ulong,
    pub orientation: ::core::ffi::c_uint,
    pub samples_per_pixel: ::core::ffi::c_uint,
    pub rows_per_strip: size_t,
}
pub const MINITIFF_PHOTOMETRIC_PALETTE: C2RustUnnamed_0 = 3;
pub const MINITIFF_COMPRESSION_NONE: C2RustUnnamed = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MINITIFF_COMPRESSION_LZMA2: C2RustUnnamed = 34925;
pub const MINITIFF_COMPRESSION_JPEG2000: C2RustUnnamed = 34712;
pub const MINITIFF_COMPRESSION_SGI_LOGLUV24: C2RustUnnamed = 34677;
pub const MINITIFF_COMPRESSION_SGI_LOGLUV: C2RustUnnamed = 34676;
pub const MINITIFF_COMPRESSION_JBIG: C2RustUnnamed = 34661;
pub const MINITIFF_COMPRESSION_KODAK_DCS: C2RustUnnamed = 32947;
pub const MINITIFF_COMPRESSION_DEFLATE: C2RustUnnamed = 32946;
pub const MINITIFF_COMPRESSION_PIXARLOG: C2RustUnnamed = 32909;
pub const MINITIFF_COMPRESSION_PIXARFILM: C2RustUnnamed = 32908;
pub const MINITIFF_COMPRESSION_IT8_BL: C2RustUnnamed = 32898;
pub const MINITIFF_COMPRESSION_IT8_HC: C2RustUnnamed = 32897;
pub const MINITIFF_COMPRESSION_IT8_LW: C2RustUnnamed = 32896;
pub const MINITIFF_COMPRESSION_IT8_CT_MP: C2RustUnnamed = 32895;
pub const MINITIFF_COMPRESSION_THUNDERSCAN: C2RustUnnamed = 32809;
pub const MINITIFF_COMPRESSION_PACKBITS: C2RustUnnamed = 32773;
pub const MINITIFF_COMPRESSION_CCITT_RLEW: C2RustUnnamed = 32771;
pub const MINITIFF_COMPRESSION_NEXT_RLE: C2RustUnnamed = 32766;
pub const MINITIFF_COMPRESSION_ITU_T43: C2RustUnnamed = 10;
pub const MINITIFF_COMPRESSION_ITU_T85: C2RustUnnamed = 9;
pub const MINITIFF_COMPRESSION_ADOBE_DEFLATE: C2RustUnnamed = 8;
pub const MINITIFF_COMPRESSION_JPEG: C2RustUnnamed = 7;
pub const MINITIFF_COMPRESSION_OLD_JPEG: C2RustUnnamed = 6;
pub const MINITIFF_COMPRESSION_LZW: C2RustUnnamed = 5;
pub const MINITIFF_COMPRESSION_CCITT_FAX4: C2RustUnnamed = 4;
pub const MINITIFF_COMPRESSION_CCITT_T6: C2RustUnnamed = 4;
pub const MINITIFF_COMPRESSION_CCITT_FAX3: C2RustUnnamed = 3;
pub const MINITIFF_COMPRESSION_CCITT_T4: C2RustUnnamed = 3;
pub const MINITIFF_COMPRESSION_CCITT_RLE: C2RustUnnamed = 2;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const MINITIFF_PHOTOMETRIC_LOGLUV: C2RustUnnamed_0 = 32845;
pub const MINITIFF_PHOTOMETRIC_LOGL: C2RustUnnamed_0 = 32844;
pub const MINITIFF_PHOTOMETRIC_CFA: C2RustUnnamed_0 = 32803;
pub const MINITIFF_PHOTOMETRIC_ITULAB: C2RustUnnamed_0 = 10;
pub const MINITIFF_PHOTOMETRIC_ICCLAB: C2RustUnnamed_0 = 9;
pub const MINITIFF_PHOTOMETRIC_CIELAB: C2RustUnnamed_0 = 8;
pub const MINITIFF_PHOTOMETRIC_YCBCR: C2RustUnnamed_0 = 6;
pub const MINITIFF_PHOTOMETRIC_SEPARATED: C2RustUnnamed_0 = 5;
pub const MINITIFF_PHOTOMETRIC_MASK: C2RustUnnamed_0 = 4;
pub const MINITIFF_PHOTOMETRIC_RGB: C2RustUnnamed_0 = 2;
pub const MINITIFF_PHOTOMETRIC_MINBLACK: C2RustUnnamed_0 = 1;
pub const MINITIFF_PHOTOMETRIC_MINWHITE: C2RustUnnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn minitiff_init_info(mut info_ptr: *mut minitiff_info) {
    memset(
        info_ptr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<minitiff_info>() as size_t,
    );
    (*info_ptr).photometric = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn minitiff_validate_info(mut info_ptr: *const minitiff_info) {
    if (*info_ptr).width == 0 as size_t || (*info_ptr).height == 0 as size_t {
        minitiff_error(
            info_ptr,
            b"Invalid image dimensions in TIFF file\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*info_ptr).bits_per_sample == 0 as ::core::ffi::c_uint
        || (*info_ptr).samples_per_pixel == 0 as ::core::ffi::c_uint
    {
        minitiff_error(
            info_ptr,
            b"Invalid pixel info in TIFF file\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*info_ptr).strip_offsets.is_null() || (*info_ptr).rows_per_strip == 0 as size_t {
        minitiff_error(
            info_ptr,
            b"Invalid strip info in TIFF file\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*info_ptr).compression
        != MINITIFF_COMPRESSION_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        minitiff_error(
            info_ptr,
            b"Unsupported compression method in TIFF file\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*info_ptr).photometric
        >= MINITIFF_PHOTOMETRIC_PALETTE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        minitiff_error(
            info_ptr,
            b"Unsupported photometric interpretation in TIFF file\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn minitiff_destroy_info(mut info_ptr: *mut minitiff_info) {
    if !(*info_ptr).strip_offsets.is_null() {
        free((*info_ptr).strip_offsets as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn default_error_handler(mut msg: *const ::core::ffi::c_char) {
    fprintf(
        stderr,
        b"minitiff: error: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        msg,
    );
    exit(EXIT_FAILURE);
}
#[no_mangle]
pub unsafe extern "C" fn minitiff_error(
    mut info_ptr: *const minitiff_info,
    mut msg: *const ::core::ffi::c_char,
) {
    if (*info_ptr).error_handler.is_some() {
        (*info_ptr)
            .error_handler
            .expect("non-null function pointer")(msg);
    } else {
        default_error_handler(msg);
    }
    abort();
}
unsafe extern "C" fn default_warning_handler(mut msg: *const ::core::ffi::c_char) {
    fprintf(
        stderr,
        b"minitiff: warning: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        msg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn minitiff_warning(
    mut info_ptr: *const minitiff_info,
    mut msg: *const ::core::ffi::c_char,
) {
    if (*info_ptr).warning_handler.is_some() {
        (*info_ptr)
            .warning_handler
            .expect("non-null function pointer")(msg);
    } else {
        default_warning_handler(msg);
    };
}
#[no_mangle]
pub static mut minitiff_sig_m: [::core::ffi::c_char; 4] = [
    0x4d as ::core::ffi::c_int as ::core::ffi::c_char,
    0x4d as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x2a as ::core::ffi::c_int as ::core::ffi::c_char,
];
#[no_mangle]
pub static mut minitiff_sig_i: [::core::ffi::c_char; 4] = [
    0x49 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x49 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x2a as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
];
#[no_mangle]
pub static mut minitiff_sig_bigm: [::core::ffi::c_char; 4] = [
    0x4d as ::core::ffi::c_int as ::core::ffi::c_char,
    0x4d as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x2b as ::core::ffi::c_int as ::core::ffi::c_char,
];
#[no_mangle]
pub static mut minitiff_sig_bigi: [::core::ffi::c_char; 4] = [
    0x49 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x49 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x2b as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
];
