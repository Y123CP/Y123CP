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
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
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
pub struct GIFScreen {
    pub Width: ::core::ffi::c_uint,
    pub Height: ::core::ffi::c_uint,
    pub GlobalColorFlag: ::core::ffi::c_uint,
    pub ColorResolution: ::core::ffi::c_uint,
    pub SortFlag: ::core::ffi::c_uint,
    pub GlobalNumColors: ::core::ffi::c_uint,
    pub Background: ::core::ffi::c_uint,
    pub PixelAspectRatio: ::core::ffi::c_uint,
    pub GlobalColorTable: [::core::ffi::c_uchar; 768],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFImage {
    pub Screen: *mut GIFScreen,
    pub LeftPos: ::core::ffi::c_uint,
    pub TopPos: ::core::ffi::c_uint,
    pub Width: ::core::ffi::c_uint,
    pub Height: ::core::ffi::c_uint,
    pub LocalColorFlag: ::core::ffi::c_uint,
    pub InterlaceFlag: ::core::ffi::c_uint,
    pub SortFlag: ::core::ffi::c_uint,
    pub LocalNumColors: ::core::ffi::c_uint,
    pub LocalColorTable: [::core::ffi::c_uchar; 768],
    pub Rows: *mut *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFExtension {
    pub Screen: *mut GIFScreen,
    pub Buffer: *mut ::core::ffi::c_uchar,
    pub BufferSize: ::core::ffi::c_uint,
    pub Label: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFGraphicCtlExt {
    pub DisposalMethod: ::core::ffi::c_uint,
    pub InputFlag: ::core::ffi::c_uint,
    pub TransparentFlag: ::core::ffi::c_uint,
    pub DelayTime: ::core::ffi::c_uint,
    pub Transparent: ::core::ffi::c_uint,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const GIF_EXTENSION: ::core::ffi::c_int = 33;
pub const GIF_IMAGE: ::core::ffi::c_int = 44;
pub const GIF_TERMINATOR: ::core::ffi::c_int = 59;
pub const GIF_GRAPHICCTL: ::core::ffi::c_int = 0xf9 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LZW_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZW_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LZW_BITS_MAX: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LZW_CODE_MAX: ::core::ffi::c_int =
    ((1 as ::core::ffi::c_int) << LZW_BITS_MAX) - 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn GIFReadScreen(mut screen: *mut GIFScreen, mut stream: *mut FILE) {
    let mut buffer: [::core::ffi::c_uchar; 7] = [0; 7];
    ReadBytes(
        &raw mut buffer as *mut ::core::ffi::c_uchar,
        6 as ::core::ffi::c_uint,
        stream,
    );
    if memcmp(
        &raw mut buffer as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        b"GIF\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        3 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        GIFError.expect("non-null function pointer")(
            b"Not a GIF file\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if memcmp(
        (&raw mut buffer as *mut ::core::ffi::c_uchar).offset(3 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        b"87a\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        3 as size_t,
    ) != 0 as ::core::ffi::c_int
        && memcmp(
            (&raw mut buffer as *mut ::core::ffi::c_uchar).offset(3 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            b"89a\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            3 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        GIFWarning.expect("non-null function pointer")(
            b"Invalid GIF version number, not \"87a\" or \"89a\"\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    ReadBytes(
        &raw mut buffer as *mut ::core::ffi::c_uchar,
        7 as ::core::ffi::c_uint,
        stream,
    );
    (*screen).Width = (*(&raw mut buffer as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*(&raw mut buffer as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    (*screen).Height = (*(&raw mut buffer as *mut ::core::ffi::c_uchar)
        .offset(2 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*(&raw mut buffer as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    (*screen).GlobalColorFlag = (if buffer[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        & 0x80 as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    (*screen).ColorResolution = (((buffer[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        & 0x70 as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    (*screen).SortFlag = (if buffer[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        & 0x8 as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    (*screen).GlobalNumColors = ((2 as ::core::ffi::c_int)
        << (buffer[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            & 0x7 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    (*screen).Background = buffer[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
    (*screen).PixelAspectRatio = buffer[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
    if (*screen).GlobalColorFlag != 0 {
        ReadBytes(
            &raw mut (*screen).GlobalColorTable as *mut ::core::ffi::c_uchar,
            (3 as ::core::ffi::c_uint).wrapping_mul((*screen).GlobalNumColors),
            stream,
        );
    }
    if (*screen).Width == 0 as ::core::ffi::c_uint || (*screen).Height == 0 as ::core::ffi::c_uint {
        GIFError.expect("non-null function pointer")(
            b"Invalid dimensions in GIF image\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*screen).Background > 0 as ::core::ffi::c_uint {
        if (*screen).GlobalColorFlag != 0 && (*screen).Background >= (*screen).GlobalNumColors
            || (*screen).GlobalColorFlag == 0
        {
            (*screen).Background = 0 as ::core::ffi::c_uint;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn GIFInitImage(
    mut image: *mut GIFImage,
    mut screen: *mut GIFScreen,
    mut rows: *mut *mut ::core::ffi::c_uchar,
) {
    (*image).Screen = screen;
    (*image).Rows = rows;
}
#[no_mangle]
pub unsafe extern "C" fn GIFDestroyImage(mut image: *mut GIFImage) {}
#[no_mangle]
pub unsafe extern "C" fn GIFReadNextBlock(
    mut image: *mut GIFImage,
    mut ext: *mut GIFExtension,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    let mut foundBogus: ::core::ffi::c_int = 0;
    foundBogus = 0 as ::core::ffi::c_int;
    loop {
        ch = GetByte(stream);
        match ch {
            GIF_IMAGE => {
                GIFReadNextImage(image, stream);
                return ch;
            }
            GIF_EXTENSION => {
                GIFReadNextExtension(ext, stream);
                return ch;
            }
            GIF_TERMINATOR => return ch,
            _ => {
                if foundBogus == 0 {
                    GIFWarning.expect("non-null function pointer")(
                        b"Bogus data in GIF file\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                foundBogus = 1 as ::core::ffi::c_int;
            }
        }
    }
}
unsafe extern "C" fn GIFReadNextImage(mut image: *mut GIFImage, mut stream: *mut FILE) {
    let mut screen: *mut GIFScreen = ::core::ptr::null_mut::<GIFScreen>();
    let mut buffer: [::core::ffi::c_uchar; 9] = [0; 9];
    ReadBytes(
        &raw mut buffer as *mut ::core::ffi::c_uchar,
        9 as ::core::ffi::c_uint,
        stream,
    );
    if image.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    (*image).LeftPos = (*(&raw mut buffer as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*(&raw mut buffer as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    (*image).TopPos = (*(&raw mut buffer as *mut ::core::ffi::c_uchar)
        .offset(2 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*(&raw mut buffer as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    (*image).Width = (*(&raw mut buffer as *mut ::core::ffi::c_uchar)
        .offset(4 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*(&raw mut buffer as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    (*image).Height = (*(&raw mut buffer as *mut ::core::ffi::c_uchar)
        .offset(6 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*(&raw mut buffer as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    (*image).LocalColorFlag = (if buffer[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        & 0x80 as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    (*image).InterlaceFlag = (if buffer[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        & 0x40 as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    (*image).SortFlag = (if buffer[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        & 0x20 as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    (*image).LocalNumColors = (if (*image).LocalColorFlag != 0 {
        (2 as ::core::ffi::c_int)
            << (buffer[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 0x7 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    if (*image).LocalColorFlag != 0 {
        ReadBytes(
            &raw mut (*image).LocalColorTable as *mut ::core::ffi::c_uchar,
            (3 as ::core::ffi::c_uint).wrapping_mul((*image).LocalNumColors),
            stream,
        );
    }
    screen = (*image).Screen;
    if (*image).Width == 0 as ::core::ffi::c_uint
        || (*image).Height == 0 as ::core::ffi::c_uint
        || (*image).LeftPos.wrapping_add((*image).Width) > (*screen).Width
        || (*image).TopPos.wrapping_add((*image).Height) > (*screen).Height
    {
        GIFError.expect("non-null function pointer")(
            b"Invalid dimensions in GIF image\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    GIFReadImageData(image, stream);
}
unsafe extern "C" fn GIFReadImageData(mut image: *mut GIFImage, mut stream: *mut FILE) {
    let mut minCodeSize: ::core::ffi::c_int = 0;
    let mut rows: *mut *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut interlaced: ::core::ffi::c_uint = 0;
    let mut colors: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut numColors: ::core::ffi::c_uint = 0;
    let mut xpos: ::core::ffi::c_uint = 0;
    let mut ypos: ::core::ffi::c_uint = 0;
    let mut pass: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_int = 0;
    minCodeSize = GetByte(stream);
    if minCodeSize >= LZW_BITS_MAX {
        GIFError.expect("non-null function pointer")(
            b"Invalid LZW code size\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if LZWDecodeByte(LZW_TRUE, minCodeSize, stream) < 0 as ::core::ffi::c_int {
        GIFError.expect("non-null function pointer")(
            b"Error decoding GIF image\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    rows = (*image).Rows;
    if rows.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    width = (*image).Width;
    height = (*image).Height;
    interlaced = (*image).InterlaceFlag;
    GIFGetColorTable(&raw mut colors, &raw mut numColors, image);
    ypos = 0 as ::core::ffi::c_uint;
    xpos = ypos;
    pass = 0 as ::core::ffi::c_int;
    loop {
        val = LZWDecodeByte(LZW_FALSE, minCodeSize, stream);
        if !(val >= 0 as ::core::ffi::c_int) {
            break;
        }
        if val as ::core::ffi::c_uint >= numColors {
            GIFWarning.expect("non-null function pointer")(
                b"Pixel value out of range in GIF image\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            val = numColors.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        }
        *(*rows.offset(ypos as isize)).offset(xpos as isize) = val as ::core::ffi::c_uchar;
        xpos = xpos.wrapping_add(1);
        if xpos == width {
            xpos = 0 as ::core::ffi::c_uint;
            if interlaced != 0 {
                match pass {
                    0 | 1 => {
                        ypos = ypos.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    2 => {
                        ypos = ypos.wrapping_add(4 as ::core::ffi::c_uint);
                    }
                    3 => {
                        ypos = ypos.wrapping_add(2 as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                if ypos >= height {
                    pass += 1;
                    match pass {
                        1 => {
                            ypos = 4 as ::core::ffi::c_uint;
                        }
                        2 => {
                            ypos = 2 as ::core::ffi::c_uint;
                        }
                        3 => {
                            ypos = 1 as ::core::ffi::c_uint;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            } else {
                ypos = ypos.wrapping_add(1);
            }
        }
        if ypos >= height {
            break;
        }
    }
    while LZWDecodeByte(LZW_FALSE, minCodeSize, stream) >= 0 as ::core::ffi::c_int {}
}
static mut DataBlockSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn GIFReadDataBlock(
    mut buffer: *mut ::core::ffi::c_uchar,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0;
    count = GetByte(stream);
    DataBlockSize = count;
    if count > 0 as ::core::ffi::c_int {
        ReadBytes(buffer, count as ::core::ffi::c_uint, stream);
    }
    return count;
}
unsafe extern "C" fn GIFSkipDataBlocks(mut stream: *mut FILE) {
    let mut count: ::core::ffi::c_int = 0;
    let mut buffer: [::core::ffi::c_uchar; 256] = [0; 256];
    loop {
        count = GetByte(stream);
        if count > 0 as ::core::ffi::c_int {
            ReadBytes(
                &raw mut buffer as *mut ::core::ffi::c_uchar,
                count as ::core::ffi::c_uint,
                stream,
            );
        } else {
            return;
        }
    }
}
unsafe extern "C" fn LZWGetCode(
    mut code_size: ::core::ffi::c_int,
    mut init_flag: ::core::ffi::c_int,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    static mut buffer: [::core::ffi::c_uchar; 280] = [0; 280];
    static mut curbit: ::core::ffi::c_int = 0;
    static mut lastbit: ::core::ffi::c_int = 0;
    static mut done: ::core::ffi::c_int = 0;
    static mut last_byte: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if init_flag != 0 {
        curbit = 0 as ::core::ffi::c_int;
        lastbit = 0 as ::core::ffi::c_int;
        last_byte = 2 as ::core::ffi::c_int;
        done = LZW_FALSE;
        return 0 as ::core::ffi::c_int;
    }
    if curbit + code_size >= lastbit {
        if done != 0 {
            if curbit >= lastbit {
                GIFError.expect("non-null function pointer")(
                    b"Ran off the end of input bits in LZW decoding\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            return -(1 as ::core::ffi::c_int);
        }
        buffer[0 as ::core::ffi::c_int as usize] =
            buffer[(last_byte - 2 as ::core::ffi::c_int) as usize];
        buffer[1 as ::core::ffi::c_int as usize] =
            buffer[(last_byte - 1 as ::core::ffi::c_int) as usize];
        count = GIFReadDataBlock(
            (&raw mut buffer as *mut ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_uchar,
            stream,
        );
        if count == 0 as ::core::ffi::c_int {
            done = LZW_TRUE;
        }
        last_byte = 2 as ::core::ffi::c_int + count;
        curbit = curbit - lastbit + 16 as ::core::ffi::c_int;
        lastbit = (2 as ::core::ffi::c_int + count) * 8 as ::core::ffi::c_int;
    }
    ret = 0 as ::core::ffi::c_int;
    i = curbit;
    j = 0 as ::core::ffi::c_int;
    while j < code_size {
        ret |= ((buffer[(i / 8 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << i % 8 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
            << j;
        i += 1;
        j += 1;
    }
    curbit += code_size;
    return ret;
}
unsafe extern "C" fn LZWDecodeByte(
    mut init_flag: ::core::ffi::c_int,
    mut input_code_size: ::core::ffi::c_int,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    static mut fresh: ::core::ffi::c_int = LZW_FALSE;
    static mut code_size: ::core::ffi::c_int = 0;
    static mut set_code_size: ::core::ffi::c_int = 0;
    static mut max_code: ::core::ffi::c_int = 0;
    static mut max_code_size: ::core::ffi::c_int = 0;
    static mut firstcode: ::core::ffi::c_int = 0;
    static mut oldcode: ::core::ffi::c_int = 0;
    static mut clear_code: ::core::ffi::c_int = 0;
    static mut end_code: ::core::ffi::c_int = 0;
    static mut table: [[::core::ffi::c_int; 4096]; 2] = [[0; 4096]; 2];
    static mut stack: [::core::ffi::c_int; 8192] = [0; 8192];
    static mut sp: *mut ::core::ffi::c_int =
        ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int;
    let mut code: ::core::ffi::c_int = 0;
    let mut incode: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if init_flag != 0 {
        fresh = LZW_TRUE;
        set_code_size = input_code_size;
        code_size = set_code_size + 1 as ::core::ffi::c_int;
        clear_code = (1 as ::core::ffi::c_int) << set_code_size;
        end_code = clear_code + 1 as ::core::ffi::c_int;
        max_code_size = 2 as ::core::ffi::c_int * clear_code;
        max_code = clear_code + 2 as ::core::ffi::c_int;
        LZWGetCode(0 as ::core::ffi::c_int, LZW_TRUE, stream);
        i = 0 as ::core::ffi::c_int;
        while i < clear_code {
            table[0 as ::core::ffi::c_int as usize][i as usize] = 0 as ::core::ffi::c_int;
            table[1 as ::core::ffi::c_int as usize][i as usize] = i;
            i += 1;
        }
        while i <= LZW_CODE_MAX {
            table[0 as ::core::ffi::c_int as usize][i as usize] = 0 as ::core::ffi::c_int;
            table[1 as ::core::ffi::c_int as usize][i as usize] = 0 as ::core::ffi::c_int;
            i += 1;
        }
        sp = &raw mut stack as *mut ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    } else if fresh != 0 {
        fresh = LZW_FALSE;
        loop {
            oldcode = LZWGetCode(code_size, LZW_FALSE, stream);
            firstcode = oldcode;
            if !(firstcode == clear_code) {
                break;
            }
        }
        return firstcode;
    }
    if sp > &raw mut stack as *mut ::core::ffi::c_int {
        sp = sp.offset(-1);
        return *sp;
    }
    loop {
        code = LZWGetCode(code_size, LZW_FALSE, stream);
        if !(code >= 0 as ::core::ffi::c_int) {
            break;
        }
        if code == clear_code {
            i = 0 as ::core::ffi::c_int;
            while i < clear_code {
                table[0 as ::core::ffi::c_int as usize][i as usize] = 0 as ::core::ffi::c_int;
                table[1 as ::core::ffi::c_int as usize][i as usize] = i;
                i += 1;
            }
            while i <= LZW_CODE_MAX {
                table[0 as ::core::ffi::c_int as usize][i as usize] = 0 as ::core::ffi::c_int;
                table[1 as ::core::ffi::c_int as usize][i as usize] = 0 as ::core::ffi::c_int;
                i += 1;
            }
            code_size = set_code_size + 1 as ::core::ffi::c_int;
            max_code_size = 2 as ::core::ffi::c_int * clear_code;
            max_code = clear_code + 2 as ::core::ffi::c_int;
            sp = &raw mut stack as *mut ::core::ffi::c_int;
            oldcode = LZWGetCode(code_size, LZW_FALSE, stream);
            firstcode = oldcode;
            return firstcode;
        } else if code == end_code {
            let mut count: ::core::ffi::c_int = 0;
            let mut buffer: [::core::ffi::c_uchar; 260] = [0; 260];
            if DataBlockSize == 0 as ::core::ffi::c_int {
                return -(2 as ::core::ffi::c_int);
            }
            loop {
                count = GIFReadDataBlock(&raw mut buffer as *mut ::core::ffi::c_uchar, stream);
                if !(count > 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            return -(2 as ::core::ffi::c_int);
        }
        incode = code;
        if code >= max_code {
            let fresh0 = sp;
            sp = sp.offset(1);
            *fresh0 = firstcode;
            code = oldcode;
        }
        while code >= clear_code {
            let fresh1 = sp;
            sp = sp.offset(1);
            *fresh1 = table[1 as ::core::ffi::c_int as usize][code as usize];
            if code == table[0 as ::core::ffi::c_int as usize][code as usize]
                || sp.offset_from(&raw mut stack as *mut ::core::ffi::c_int) as ::core::ffi::c_long
                    as size_t
                    >= (::core::mem::size_of::<[::core::ffi::c_int; 8192]>() as usize)
                        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
            {
                GIFError.expect("non-null function pointer")(
                    b"Circular dependency found in LZW table\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            code = table[0 as ::core::ffi::c_int as usize][code as usize];
        }
        firstcode = table[1 as ::core::ffi::c_int as usize][code as usize];
        let fresh2 = sp;
        sp = sp.offset(1);
        *fresh2 = firstcode;
        code = max_code;
        if code <= LZW_CODE_MAX {
            table[0 as ::core::ffi::c_int as usize][code as usize] = oldcode;
            table[1 as ::core::ffi::c_int as usize][code as usize] = firstcode;
            max_code += 1;
            if max_code >= max_code_size && max_code_size <= LZW_CODE_MAX {
                max_code_size *= 2 as ::core::ffi::c_int;
                code_size += 1;
            }
        }
        oldcode = incode;
        if sp > &raw mut stack as *mut ::core::ffi::c_int {
            sp = sp.offset(-1);
            return *sp;
        }
    }
    return code;
}
static mut DefaultColorTable: [::core::ffi::c_uchar; 24] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn GIFGetColorTable(
    mut colors: *mut *mut ::core::ffi::c_uchar,
    mut numColors: *mut ::core::ffi::c_uint,
    mut image: *mut GIFImage,
) {
    let mut screen: *mut GIFScreen = ::core::ptr::null_mut::<GIFScreen>();
    if (*image).LocalColorFlag != 0 {
        *colors = &raw mut (*image).LocalColorTable as *mut ::core::ffi::c_uchar;
        *numColors = (*image).LocalNumColors;
        return;
    }
    screen = (*image).Screen;
    if (*screen).GlobalColorFlag != 0 {
        *colors = &raw mut (*screen).GlobalColorTable as *mut ::core::ffi::c_uchar;
        *numColors = (*screen).GlobalNumColors;
        return;
    }
    *colors = &raw mut DefaultColorTable as *mut ::core::ffi::c_uchar;
    *numColors = (::core::mem::size_of::<[::core::ffi::c_uchar; 24]>() as usize)
        .wrapping_div(3 as usize) as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn GIFInitExtension(
    mut ext: *mut GIFExtension,
    mut screen: *mut GIFScreen,
    mut initBufferSize: ::core::ffi::c_uint,
) {
    let mut newBuffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*ext).Screen = screen;
    if initBufferSize > 0 as ::core::ffi::c_uint {
        newBuffer = malloc(initBufferSize as size_t) as *mut ::core::ffi::c_uchar;
        if newBuffer.is_null() {
            ErrorAlloc();
        }
        (*ext).Buffer = newBuffer;
        (*ext).BufferSize = initBufferSize;
    } else {
        (*ext).Buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        (*ext).BufferSize = 0 as ::core::ffi::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn GIFDestroyExtension(mut ext: *mut GIFExtension) {
    free((*ext).Buffer as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn GIFReadNextExtension(mut ext: *mut GIFExtension, mut stream: *mut FILE) {
    let mut newBuffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut newBufferSize: ::core::ffi::c_uint = 0;
    let mut offset: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_uint = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut label: ::core::ffi::c_int = 0;
    label = GetByte(stream);
    if ext.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    (*ext).Label = label as ::core::ffi::c_uchar;
    offset = 0 as ::core::ffi::c_uint;
    len = (*ext).BufferSize;
    loop {
        if len < 255 as ::core::ffi::c_uint {
            newBufferSize = (*ext).BufferSize.wrapping_add(1024 as ::core::ffi::c_uint);
            newBuffer = realloc(
                (*ext).Buffer as *mut ::core::ffi::c_void,
                newBufferSize as size_t,
            ) as *mut ::core::ffi::c_uchar;
            if newBuffer.is_null() {
                ErrorAlloc();
            }
            (*ext).BufferSize = newBufferSize;
            (*ext).Buffer = newBuffer;
            len = len.wrapping_add(1024 as ::core::ffi::c_uint);
        }
        count = GIFReadDataBlock((*ext).Buffer.offset(offset as isize), stream);
        if count == 0 as ::core::ffi::c_int {
            break;
        }
        offset = offset.wrapping_add(count as ::core::ffi::c_uint);
        len = len.wrapping_sub(count as ::core::ffi::c_uint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GIFGetGraphicCtl(
    mut graphicExt: *mut GIFGraphicCtlExt,
    mut ext: *mut GIFExtension,
) {
    let mut buffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if (*ext).Label as ::core::ffi::c_int != GIF_GRAPHICCTL {
        GIFWarning.expect("non-null function pointer")(
            b"Not a graphic control extension in GIF file\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*ext).BufferSize < 4 as ::core::ffi::c_uint {
        GIFWarning.expect("non-null function pointer")(
            b"Broken graphic control extension in GIF file\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    buffer = (*ext).Buffer;
    (*graphicExt).DisposalMethod = (*buffer.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int
        & 0x7 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    (*graphicExt).InputFlag = (*buffer.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int
        & 0x1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    (*graphicExt).TransparentFlag = (*buffer.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 0x1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    (*graphicExt).DelayTime = (*buffer
        .offset(1 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        + ((*buffer
            .offset(1 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    (*graphicExt).Transparent =
        *buffer.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
}
unsafe extern "C" fn GetByte(mut stream: *mut FILE) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    ch = getc(stream);
    if ch == EOF {
        ErrorRead(stream);
    }
    return ch;
}
unsafe extern "C" fn ReadBytes(
    mut buffer: *mut ::core::ffi::c_uchar,
    mut count: ::core::ffi::c_uint,
    mut stream: *mut FILE,
) {
    if fread(
        buffer as *mut ::core::ffi::c_void,
        count as size_t,
        1 as size_t,
        stream,
    ) != 1 as ::core::ffi::c_ulong
    {
        ErrorRead(stream);
    }
}
unsafe extern "C" fn ErrorAlloc() {
    GIFError.expect("non-null function pointer")(
        b"Out of memory in GIF decoder\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn ErrorRead(mut stream: *mut FILE) {
    if ferror(stream) != 0 {
        GIFError.expect("non-null function pointer")(
            b"Error reading GIF file\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        GIFError.expect("non-null function pointer")(
            b"Unexpected end of GIF file\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn DefaultError(mut message: *const ::core::ffi::c_char) {
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        message,
    );
    exit(EXIT_FAILURE);
}
unsafe extern "C" fn DefaultWarning(mut message: *const ::core::ffi::c_char) {
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        message,
    );
}
#[no_mangle]
pub static mut GIFError: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()> =
    unsafe { Some(DefaultError as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()) };
#[no_mangle]
pub static mut GIFWarning: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()> =
    unsafe { Some(DefaultWarning as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()) };
