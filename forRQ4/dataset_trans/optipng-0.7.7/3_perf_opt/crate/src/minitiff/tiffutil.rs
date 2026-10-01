use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;

pub const MINITIFF_PHOTOMETRIC_PALETTE: C2RustUnnamed_htdd24ee73 = 3;
pub const MINITIFF_COMPRESSION_NONE: C2RustUnnamed_htdd24ee73 = 1;
pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const MINITIFF_COMPRESSION_LZMA2: C2RustUnnamed_htdd24ee73 = 34925;
pub const MINITIFF_COMPRESSION_JPEG2000: C2RustUnnamed_htdd24ee73 = 34712;
pub const MINITIFF_COMPRESSION_SGI_LOGLUV24: C2RustUnnamed_htdd24ee73 = 34677;
pub const MINITIFF_COMPRESSION_SGI_LOGLUV: C2RustUnnamed_htdd24ee73 = 34676;
pub const MINITIFF_COMPRESSION_JBIG: C2RustUnnamed_htdd24ee73 = 34661;
pub const MINITIFF_COMPRESSION_KODAK_DCS: C2RustUnnamed_htdd24ee73 = 32947;
pub const MINITIFF_COMPRESSION_DEFLATE: C2RustUnnamed_htdd24ee73 = 32946;
pub const MINITIFF_COMPRESSION_PIXARLOG: C2RustUnnamed_htdd24ee73 = 32909;
pub const MINITIFF_COMPRESSION_PIXARFILM: C2RustUnnamed_htdd24ee73 = 32908;
pub const MINITIFF_COMPRESSION_IT8_BL: C2RustUnnamed_htdd24ee73 = 32898;
pub const MINITIFF_COMPRESSION_IT8_HC: C2RustUnnamed_htdd24ee73 = 32897;
pub const MINITIFF_COMPRESSION_IT8_LW: C2RustUnnamed_htdd24ee73 = 32896;
pub const MINITIFF_COMPRESSION_IT8_CT_MP: C2RustUnnamed_htdd24ee73 = 32895;
pub const MINITIFF_COMPRESSION_THUNDERSCAN: C2RustUnnamed_htdd24ee73 = 32809;
pub const MINITIFF_COMPRESSION_PACKBITS: C2RustUnnamed_htdd24ee73 = 32773;
pub const MINITIFF_COMPRESSION_CCITT_RLEW: C2RustUnnamed_htdd24ee73 = 32771;
pub const MINITIFF_COMPRESSION_NEXT_RLE: C2RustUnnamed_htdd24ee73 = 32766;
pub const MINITIFF_COMPRESSION_ITU_T43: C2RustUnnamed_htdd24ee73 = 10;
pub const MINITIFF_COMPRESSION_ITU_T85: C2RustUnnamed_htdd24ee73 = 9;
pub const MINITIFF_COMPRESSION_ADOBE_DEFLATE: C2RustUnnamed_htdd24ee73 = 8;
pub const MINITIFF_COMPRESSION_JPEG: C2RustUnnamed_htdd24ee73 = 7;
pub const MINITIFF_COMPRESSION_OLD_JPEG: C2RustUnnamed_htdd24ee73 = 6;
pub const MINITIFF_COMPRESSION_LZW: C2RustUnnamed_htdd24ee73 = 5;
pub const MINITIFF_COMPRESSION_CCITT_FAX4: C2RustUnnamed_htdd24ee73 = 4;
pub const MINITIFF_COMPRESSION_CCITT_T6: C2RustUnnamed_htdd24ee73 = 4;
pub const MINITIFF_COMPRESSION_CCITT_FAX3: C2RustUnnamed_htdd24ee73 = 3;
pub const MINITIFF_COMPRESSION_CCITT_T4: C2RustUnnamed_htdd24ee73 = 3;
pub const MINITIFF_COMPRESSION_CCITT_RLE: C2RustUnnamed_htdd24ee73 = 2;
pub const MINITIFF_PHOTOMETRIC_LOGLUV: C2RustUnnamed_htdd24ee73 = 32845;
pub const MINITIFF_PHOTOMETRIC_LOGL: C2RustUnnamed_htdd24ee73 = 32844;
pub const MINITIFF_PHOTOMETRIC_CFA: C2RustUnnamed_htdd24ee73 = 32803;
pub const MINITIFF_PHOTOMETRIC_ITULAB: C2RustUnnamed_htdd24ee73 = 10;
pub const MINITIFF_PHOTOMETRIC_ICCLAB: C2RustUnnamed_htdd24ee73 = 9;
pub const MINITIFF_PHOTOMETRIC_CIELAB: C2RustUnnamed_htdd24ee73 = 8;
pub const MINITIFF_PHOTOMETRIC_YCBCR: C2RustUnnamed_htdd24ee73 = 6;
pub const MINITIFF_PHOTOMETRIC_SEPARATED: C2RustUnnamed_htdd24ee73 = 5;
pub const MINITIFF_PHOTOMETRIC_MASK: C2RustUnnamed_htdd24ee73 = 4;
pub const MINITIFF_PHOTOMETRIC_RGB: C2RustUnnamed_htdd24ee73 = 2;
pub const MINITIFF_PHOTOMETRIC_MINBLACK: C2RustUnnamed_htdd24ee73 = 1;
pub const MINITIFF_PHOTOMETRIC_MINWHITE: C2RustUnnamed_htdd24ee73 = 0;

#[inline]
pub unsafe fn minitiff_init_info(mut info_ptr: *mut minitiff_info) {
    memset(
        info_ptr as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<minitiff_info>() as size_t,
    );
    (*info_ptr).photometric = -(1 as c_int) as c_uint;
}
#[inline]
pub unsafe fn minitiff_validate_info(mut info_ptr: *const minitiff_info) {
    let info_ptr_view: &minitiff_info = unsafe { &*info_ptr };
    if info_ptr_view.width == 0 as size_t || info_ptr_view.height == 0 as size_t {
        minitiff_error(
            info_ptr,
            b"Invalid image dimensions in TIFF file\0" as *const u8 as *const c_char,
        );
    }
    if info_ptr_view.bits_per_sample == 0 as c_uint
        || info_ptr_view.samples_per_pixel == 0 as c_uint
    {
        minitiff_error(
            info_ptr,
            b"Invalid pixel info in TIFF file\0" as *const u8 as *const c_char,
        );
    }
    if info_ptr_view.strip_offsets.is_null() || info_ptr_view.rows_per_strip == 0 as size_t {
        minitiff_error(
            info_ptr,
            b"Invalid strip info in TIFF file\0" as *const u8 as *const c_char,
        );
    }
    if info_ptr_view.compression
        != MINITIFF_COMPRESSION_NONE as c_int as c_uint
    {
        minitiff_error(
            info_ptr,
            b"Unsupported compression method in TIFF file\0" as *const u8
                as *const c_char,
        );
    }
    if info_ptr_view.photometric
        >= MINITIFF_PHOTOMETRIC_PALETTE as c_int as c_uint
    {
        minitiff_error(
            info_ptr,
            b"Unsupported photometric interpretation in TIFF file\0" as *const u8
                as *const c_char,
        );
    }
}
#[inline]
pub unsafe fn minitiff_destroy_info(mut info_ptr: *mut minitiff_info) {
    let info_ptr_view: &minitiff_info = unsafe { &*info_ptr };
    if !info_ptr_view.strip_offsets.is_null() {
        free(info_ptr_view.strip_offsets as *mut c_void);
    }
}
unsafe fn default_error_handler(mut msg: *const c_char) {
    fprintf(
        stderr,
        b"minitiff: error: %s\n\0" as *const u8 as *const c_char,
        msg,
    );
    exit(EXIT_FAILURE);
}
#[no_mangle]
pub unsafe extern "C" fn minitiff_error(
    mut info_ptr: *const minitiff_info,
    mut msg: *const c_char,
) {
    let info_ptr_view: &minitiff_info = unsafe { &*info_ptr };
    if info_ptr_view.error_handler.is_some() {
        info_ptr_view
            .error_handler
            .expect("non-null function pointer")(msg);
    } else {
        default_error_handler(msg);
    }
    abort();
}
unsafe fn default_warning_handler(mut msg: *const c_char) {
    fprintf(
        stderr,
        b"minitiff: warning: %s\n\0" as *const u8 as *const c_char,
        msg,
    );
}
#[inline]
pub unsafe fn minitiff_warning(
    mut info_ptr: *const minitiff_info,
    mut msg: *const c_char,
) {
    let info_ptr_view: &minitiff_info = unsafe { &*info_ptr };
    if info_ptr_view.warning_handler.is_some() {
        info_ptr_view
            .warning_handler
            .expect("non-null function pointer")(msg);
    } else {
        default_warning_handler(msg);
    };
}
#[no_mangle]
pub static mut minitiff_sig_m: [c_char; 4] = [
    0x4d as c_int as c_char,
    0x4d as c_int as c_char,
    0 as c_int as c_char,
    0x2a as c_int as c_char,
];
#[no_mangle]
pub static mut minitiff_sig_i: [c_char; 4] = [
    0x49 as c_int as c_char,
    0x49 as c_int as c_char,
    0x2a as c_int as c_char,
    0 as c_int as c_char,
];
#[no_mangle]
pub static mut minitiff_sig_bigm: [c_char; 4] = [
    0x4d as c_int as c_char,
    0x4d as c_int as c_char,
    0 as c_int as c_char,
    0x2b as c_int as c_char,
];
#[no_mangle]
pub static mut minitiff_sig_bigi: [c_char; 4] = [
    0x49 as c_int as c_char,
    0x49 as c_int as c_char,
    0x2b as c_int as c_char,
    0 as c_int as c_char,
];
