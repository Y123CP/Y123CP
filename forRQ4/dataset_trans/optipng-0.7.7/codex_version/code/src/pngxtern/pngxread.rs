extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::core::ffi::c_int;
    fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::core::ffi::c_int;
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_read_png(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        transforms: ::core::ffi::c_int,
        params: png_voidp,
    );
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn pngx_sig_is_bmp(
        sig: png_bytep,
        sig_size: size_t,
        fmt_name_ptr: png_const_charpp,
        fmt_long_name_ptr: png_const_charpp,
    ) -> ::core::ffi::c_int;
    fn pngx_read_bmp(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn pngx_sig_is_gif(
        sig: png_bytep,
        sig_size: size_t,
        fmt_name_ptr: png_const_charpp,
        fmt_long_name_ptr: png_const_charpp,
    ) -> ::core::ffi::c_int;
    fn pngx_read_gif(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn pngx_sig_is_jpeg(
        sig: png_bytep,
        sig_size: size_t,
        fmt_name_ptr: png_const_charpp,
        fmt_long_name_ptr: png_const_charpp,
    ) -> ::core::ffi::c_int;
    fn pngx_read_jpeg(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn pngx_sig_is_pnm(
        sig: png_bytep,
        sig_size: size_t,
        fmt_name_ptr: png_const_charpp,
        fmt_long_name_ptr: png_const_charpp,
    ) -> ::core::ffi::c_int;
    fn pngx_read_pnm(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn pngx_sig_is_tiff(
        sig: png_bytep,
        sig_size: size_t,
        fmt_name_ptr: png_const_charpp,
        fmt_long_name_ptr: png_const_charpp,
    ) -> ::core::ffi::c_int;
    fn pngx_read_tiff(
        png_ptr: png_structp,
        info_ptr: png_infop,
        stream: *mut FILE,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: ::core::ffi::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: ::core::ffi::c_uint,
    pub __wchb: [::core::ffi::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type __fpos_t = _G_fpos_t;
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
pub type fpos_t = __fpos_t;
pub type png_byte = ::core::ffi::c_uchar;
pub type png_voidp = *mut ::core::ffi::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const ::core::ffi::c_char;
pub type png_const_charpp = *mut *const ::core::ffi::c_char;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn pngx_sig_is_png(
    mut png_ptr: png_structp,
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> ::core::ffi::c_int {
    static mut pngx_png_standalone_fmt_name: [::core::ffi::c_char; 4] =
        unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"PNG\0") };
    static mut pngx_png_datastream_fmt_name: [::core::ffi::c_char; 15] = unsafe {
        ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"PNG datastream\0")
    };
    static mut pngx_png_standalone_fmt_long_name: [::core::ffi::c_char; 26] = unsafe {
        ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
            *b"Portable Network Graphics\0",
        )
    };
    static mut pngx_png_datastream_fmt_long_name: [::core::ffi::c_char; 46] = unsafe {
        ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
            *b"Portable Network Graphics embedded datastream\0",
        )
    };
    static mut png_file_sig: [png_byte; 8] = [
        137 as ::core::ffi::c_int as png_byte,
        80 as ::core::ffi::c_int as png_byte,
        78 as ::core::ffi::c_int as png_byte,
        71 as ::core::ffi::c_int as png_byte,
        13 as ::core::ffi::c_int as png_byte,
        10 as ::core::ffi::c_int as png_byte,
        26 as ::core::ffi::c_int as png_byte,
        10 as ::core::ffi::c_int as png_byte,
    ];
    static mut mng_file_sig: [png_byte; 8] = [
        138 as ::core::ffi::c_int as png_byte,
        77 as ::core::ffi::c_int as png_byte,
        78 as ::core::ffi::c_int as png_byte,
        71 as ::core::ffi::c_int as png_byte,
        13 as ::core::ffi::c_int as png_byte,
        10 as ::core::ffi::c_int as png_byte,
        26 as ::core::ffi::c_int as png_byte,
        10 as ::core::ffi::c_int as png_byte,
    ];
    static mut png_ihdr_sig: [png_byte; 8] = [
        0 as ::core::ffi::c_int as png_byte,
        0 as ::core::ffi::c_int as png_byte,
        0 as ::core::ffi::c_int as png_byte,
        13 as ::core::ffi::c_int as png_byte,
        73 as ::core::ffi::c_int as png_byte,
        72 as ::core::ffi::c_int as png_byte,
        68 as ::core::ffi::c_int as png_byte,
        82 as ::core::ffi::c_int as png_byte,
    ];
    let mut has_png_sig: ::core::ffi::c_int = 0;
    if sig_size <= (25 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    has_png_sig = (memcmp(
        sig as *const ::core::ffi::c_void,
        &raw const png_file_sig as *const png_byte as *const ::core::ffi::c_void,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if memcmp(
        sig.offset(
            (if has_png_sig != 0 {
                8 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as isize,
        ) as *const ::core::ffi::c_void,
        &raw const png_ihdr_sig as *const png_byte as *const ::core::ffi::c_void,
        8 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        if memcmp(
            sig as *const ::core::ffi::c_void,
            &raw const png_file_sig as *const png_byte as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
            && (*sig.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 10 as ::core::ffi::c_int
                || *sig.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 13 as ::core::ffi::c_int)
        {
            png_error(
                png_ptr as png_const_structrp,
                b"PNG file appears to be corrupted by text file conversions\0" as *const u8
                    as png_const_charp,
            );
        } else if memcmp(
            sig as *const ::core::ffi::c_void,
            &raw const mng_file_sig as *const png_byte as *const ::core::ffi::c_void,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            png_error(
                png_ptr as png_const_structrp,
                b"MNG decoding is not supported\0" as *const u8 as png_const_charp,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = if has_png_sig != 0 {
            &raw const pngx_png_standalone_fmt_name as *const ::core::ffi::c_char
        } else {
            &raw const pngx_png_datastream_fmt_name as *const ::core::ffi::c_char
        };
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = if has_png_sig != 0 {
            &raw const pngx_png_standalone_fmt_long_name as *const ::core::ffi::c_char
        } else {
            &raw const pngx_png_datastream_fmt_long_name as *const ::core::ffi::c_char
        };
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_image(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> ::core::ffi::c_int {
    let mut sig: [png_byte; 128] = [0; 128];
    let mut num: size_t = 0;
    let mut read_fn: Option<
        unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
    > = None;
    let mut stream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut fpos: fpos_t = fpos_t {
        __pos: 0,
        __state: __mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        },
    };
    let mut result: ::core::ffi::c_int = 0;
    stream = png_get_io_ptr(png_ptr as png_const_structrp) as *mut FILE;
    if fgetpos(stream, &raw mut fpos) != 0 as ::core::ffi::c_int {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't ftell in input file stream\0" as *const u8 as png_const_charp,
        );
    }
    num = fread(
        &raw mut sig as *mut png_byte as *mut ::core::ffi::c_void,
        1 as size_t,
        ::core::mem::size_of::<[png_byte; 128]>() as size_t,
        stream,
    ) as size_t;
    if fsetpos(stream, &raw mut fpos) != 0 as ::core::ffi::c_int {
        png_error(
            png_ptr as png_const_structrp,
            b"Can't fseek in input file stream\0" as *const u8 as png_const_charp,
        );
    }
    if pngx_sig_is_png(
        png_ptr,
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as ::core::ffi::c_int
    {
        png_read_png(
            png_ptr as png_structrp,
            info_ptr as png_inforp,
            0 as ::core::ffi::c_int,
            NULL,
        );
        if getc(stream) != EOF {
            png_warning(
                png_ptr as png_const_structrp,
                b"Extraneous data found after IEND\0" as *const u8 as png_const_charp,
            );
            fseek(stream, 0 as ::core::ffi::c_long, SEEK_END);
        }
        return 1 as ::core::ffi::c_int;
    }
    if pngx_sig_is_bmp(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as ::core::ffi::c_int
    {
        read_fn = Some(
            pngx_read_bmp
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
            >;
    } else if pngx_sig_is_gif(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as ::core::ffi::c_int
    {
        read_fn = Some(
            pngx_read_gif
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
            >;
    } else if pngx_sig_is_jpeg(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as ::core::ffi::c_int
    {
        read_fn = Some(
            pngx_read_jpeg
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
            >;
    } else if pngx_sig_is_pnm(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as ::core::ffi::c_int
    {
        read_fn = Some(
            pngx_read_pnm
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
            >;
    } else if pngx_sig_is_tiff(
        &raw mut sig as png_bytep,
        num,
        fmt_name_ptr,
        fmt_long_name_ptr,
    ) > 0 as ::core::ffi::c_int
    {
        read_fn = Some(
            pngx_read_tiff
                as unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(png_structp, png_infop, *mut FILE) -> ::core::ffi::c_int,
            >;
    } else {
        return 0 as ::core::ffi::c_int;
    }
    result = read_fn.expect("non-null function pointer")(png_ptr, info_ptr, stream);
    if result <= 0 as ::core::ffi::c_int {
        if fsetpos(stream, &raw mut fpos) != 0 as ::core::ffi::c_int {
            png_error(
                png_ptr as png_const_structrp,
                b"Can't fseek in input file stream\0" as *const u8 as png_const_charp,
            );
        }
    }
    return result;
}
