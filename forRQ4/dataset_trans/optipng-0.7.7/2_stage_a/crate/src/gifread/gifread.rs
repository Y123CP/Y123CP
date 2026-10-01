use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;

pub const GIF_EXTENSION: c_int = 33;
pub const GIF_IMAGE: c_int = 44;
pub const GIF_TERMINATOR: c_int = 59;

pub const LZW_FALSE: c_int = 0 as c_int;
pub const LZW_TRUE: c_int = 1 as c_int;
pub const LZW_BITS_MAX: c_int = 12 as c_int;
pub const LZW_CODE_MAX: c_int =
    ((1 as c_int) << LZW_BITS_MAX) - 1 as c_int;
#[inline]
pub unsafe fn GIFReadScreen(mut screen: *mut GIFScreen, mut stream: *mut FILE) {
    let mut buffer: [c_uchar; 7] = [0; 7];
    ReadBytes(
        &raw mut buffer as *mut c_uchar,
        6 as c_uint,
        stream,
    );
    if memcmp(
        &raw mut buffer as *mut c_uchar as *const c_void,
        b"GIF\0" as *const u8 as *const c_char as *const c_void,
        3 as size_t,
    ) != 0 as c_int
    {
        GIFError.expect("non-null function pointer")(
            b"Not a GIF file\0" as *const u8 as *const c_char,
        );
    }
    if memcmp(
        (&raw mut buffer as *mut c_uchar).offset(3 as c_int as isize)
            as *const c_void,
        b"87a\0" as *const u8 as *const c_char as *const c_void,
        3 as size_t,
    ) != 0 as c_int
        && memcmp(
            (&raw mut buffer as *mut c_uchar).offset(3 as c_int as isize)
                as *const c_void,
            b"89a\0" as *const u8 as *const c_char as *const c_void,
            3 as size_t,
        ) != 0 as c_int
    {
        GIFWarning.expect("non-null function pointer")(
            b"Invalid GIF version number, not \"87a\" or \"89a\"\0" as *const u8
                as *const c_char,
        );
    }
    ReadBytes(
        &raw mut buffer as *mut c_uchar,
        7 as c_uint,
        stream,
    );
    (*screen).Width = (*(&raw mut buffer as *mut c_uchar)
        .offset(0 as c_int as isize)
        .offset(0 as c_int as isize) as c_int
        + ((*(&raw mut buffer as *mut c_uchar)
            .offset(0 as c_int as isize)
            .offset(1 as c_int as isize) as c_int)
            << 8 as c_int)) as c_uint;
    (*screen).Height = (*(&raw mut buffer as *mut c_uchar)
        .offset(2 as c_int as isize)
        .offset(0 as c_int as isize) as c_int
        + ((*(&raw mut buffer as *mut c_uchar)
            .offset(2 as c_int as isize)
            .offset(1 as c_int as isize) as c_int)
            << 8 as c_int)) as c_uint;
    (*screen).GlobalColorFlag = (if buffer[4 as c_int as usize] as c_int
        & 0x80 as c_int
        != 0
    {
        1 as c_int
    } else {
        0 as c_int
    }) as c_uint;
    (*screen).ColorResolution = (((buffer[4 as c_int as usize] as c_int
        & 0x70 as c_int)
        >> 3 as c_int)
        + 1 as c_int) as c_uint;
    (*screen).SortFlag = (if buffer[4 as c_int as usize] as c_int
        & 0x8 as c_int
        != 0
    {
        1 as c_int
    } else {
        0 as c_int
    }) as c_uint;
    (*screen).GlobalNumColors = ((2 as c_int)
        << (buffer[4 as c_int as usize] as c_int
            & 0x7 as c_int)) as c_uint;
    (*screen).Background = buffer[5 as c_int as usize] as c_uint;
    (*screen).PixelAspectRatio = buffer[6 as c_int as usize] as c_uint;
    if (*screen).GlobalColorFlag != 0 {
        ReadBytes(
            &raw mut (*screen).GlobalColorTable as *mut c_uchar,
            (3 as c_uint).wrapping_mul((*screen).GlobalNumColors),
            stream,
        );
    }
    if (*screen).Width == 0 as c_uint || (*screen).Height == 0 as c_uint {
        GIFError.expect("non-null function pointer")(
            b"Invalid dimensions in GIF image\0" as *const u8 as *const c_char,
        );
    }
    if (*screen).Background > 0 as c_uint {
        if (*screen).GlobalColorFlag != 0 && (*screen).Background >= (*screen).GlobalNumColors
            || (*screen).GlobalColorFlag == 0
        {
            (*screen).Background = 0 as c_uint;
        }
    }
}
#[inline]
pub unsafe fn GIFInitImage(
    mut image: *mut GIFImage,
    mut screen: *mut GIFScreen,
    mut rows: *mut *mut c_uchar,
) {
    (*image).Screen = screen;
    (*image).Rows = rows;
}
#[inline]
pub unsafe fn GIFDestroyImage(mut image: *mut GIFImage) {}
#[inline]
pub unsafe fn GIFReadNextBlock(
    mut image: *mut GIFImage,
    mut ext: *mut GIFExtension,
    mut stream: *mut FILE,
) -> c_int {
    let mut ch: c_int = 0;
    let mut foundBogus: c_int = 0;
    foundBogus = 0 as c_int;
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
                        b"Bogus data in GIF file\0" as *const u8 as *const c_char,
                    );
                }
                foundBogus = 1 as c_int;
            }
        }
    }
}
unsafe fn GIFReadNextImage(mut image: *mut GIFImage, mut stream: *mut FILE) {
    let mut screen: *mut GIFScreen = ::core::ptr::null_mut::<GIFScreen>();
    let mut buffer: [c_uchar; 9] = [0; 9];
    ReadBytes(
        &raw mut buffer as *mut c_uchar,
        9 as c_uint,
        stream,
    );
    if image.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    (*image).LeftPos = (*(&raw mut buffer as *mut c_uchar)
        .offset(0 as c_int as isize)
        .offset(0 as c_int as isize) as c_int
        + ((*(&raw mut buffer as *mut c_uchar)
            .offset(0 as c_int as isize)
            .offset(1 as c_int as isize) as c_int)
            << 8 as c_int)) as c_uint;
    (*image).TopPos = (*(&raw mut buffer as *mut c_uchar)
        .offset(2 as c_int as isize)
        .offset(0 as c_int as isize) as c_int
        + ((*(&raw mut buffer as *mut c_uchar)
            .offset(2 as c_int as isize)
            .offset(1 as c_int as isize) as c_int)
            << 8 as c_int)) as c_uint;
    (*image).Width = (*(&raw mut buffer as *mut c_uchar)
        .offset(4 as c_int as isize)
        .offset(0 as c_int as isize) as c_int
        + ((*(&raw mut buffer as *mut c_uchar)
            .offset(4 as c_int as isize)
            .offset(1 as c_int as isize) as c_int)
            << 8 as c_int)) as c_uint;
    (*image).Height = (*(&raw mut buffer as *mut c_uchar)
        .offset(6 as c_int as isize)
        .offset(0 as c_int as isize) as c_int
        + ((*(&raw mut buffer as *mut c_uchar)
            .offset(6 as c_int as isize)
            .offset(1 as c_int as isize) as c_int)
            << 8 as c_int)) as c_uint;
    (*image).LocalColorFlag = (if buffer[8 as c_int as usize] as c_int
        & 0x80 as c_int
        != 0
    {
        1 as c_int
    } else {
        0 as c_int
    }) as c_uint;
    (*image).InterlaceFlag = (if buffer[8 as c_int as usize] as c_int
        & 0x40 as c_int
        != 0
    {
        1 as c_int
    } else {
        0 as c_int
    }) as c_uint;
    (*image).SortFlag = (if buffer[8 as c_int as usize] as c_int
        & 0x20 as c_int
        != 0
    {
        1 as c_int
    } else {
        0 as c_int
    }) as c_uint;
    (*image).LocalNumColors = (if (*image).LocalColorFlag != 0 {
        (2 as c_int)
            << (buffer[8 as c_int as usize] as c_int
                & 0x7 as c_int)
    } else {
        0 as c_int
    }) as c_uint;
    if (*image).LocalColorFlag != 0 {
        ReadBytes(
            &raw mut (*image).LocalColorTable as *mut c_uchar,
            (3 as c_uint).wrapping_mul((*image).LocalNumColors),
            stream,
        );
    }
    screen = (*image).Screen;
    if (*image).Width == 0 as c_uint
        || (*image).Height == 0 as c_uint
        || (*image).LeftPos.wrapping_add((*image).Width) > (*screen).Width
        || (*image).TopPos.wrapping_add((*image).Height) > (*screen).Height
    {
        GIFError.expect("non-null function pointer")(
            b"Invalid dimensions in GIF image\0" as *const u8 as *const c_char,
        );
    }
    GIFReadImageData(image, stream);
}
unsafe fn GIFReadImageData(mut image: *mut GIFImage, mut stream: *mut FILE) {
    let image_view: &GIFImage = unsafe { &*image };
    let mut minCodeSize: c_int = 0;
    let mut rows: *mut *mut c_uchar =
        ::core::ptr::null_mut::<*mut c_uchar>();
    let mut width: c_uint = 0;
    let mut height: c_uint = 0;
    let mut interlaced: c_uint = 0;
    let mut colors: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut numColors: c_uint = 0;
    let mut xpos: c_uint = 0;
    let mut ypos: c_uint = 0;
    let mut pass: c_int = 0;
    let mut val: c_int = 0;
    minCodeSize = GetByte(stream);
    if minCodeSize >= LZW_BITS_MAX {
        GIFError.expect("non-null function pointer")(
            b"Invalid LZW code size\0" as *const u8 as *const c_char,
        );
    }
    if LZWDecodeByte(LZW_TRUE, minCodeSize, stream) < 0 as c_int {
        GIFError.expect("non-null function pointer")(
            b"Error decoding GIF image\0" as *const u8 as *const c_char,
        );
    }
    rows = image_view.Rows;
    if rows.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    width = image_view.Width;
    height = image_view.Height;
    interlaced = image_view.InterlaceFlag;
    GIFGetColorTable(&raw mut colors, &raw mut numColors, image);
    ypos = 0 as c_uint;
    xpos = ypos;
    pass = 0 as c_int;
    loop {
        val = LZWDecodeByte(LZW_FALSE, minCodeSize, stream);
        if !(val >= 0 as c_int) {
            break;
        }
        if val as c_uint >= numColors {
            GIFWarning.expect("non-null function pointer")(
                b"Pixel value out of range in GIF image\0" as *const u8
                    as *const c_char,
            );
            val = numColors.wrapping_sub(1 as c_uint) as c_int;
        }
        *(*rows.offset(ypos as isize)).offset(xpos as isize) = val as c_uchar;
        xpos = xpos.wrapping_add(1);
        if xpos == width {
            xpos = 0 as c_uint;
            if interlaced != 0 {
                match pass {
                    0 | 1 => {
                        ypos = ypos.wrapping_add(8 as c_uint);
                    }
                    2 => {
                        ypos = ypos.wrapping_add(4 as c_uint);
                    }
                    3 => {
                        ypos = ypos.wrapping_add(2 as c_uint);
                    }
                    _ => {}
                }
                if ypos >= height {
                    pass += 1;
                    match pass {
                        1 => {
                            ypos = 4 as c_uint;
                        }
                        2 => {
                            ypos = 2 as c_uint;
                        }
                        3 => {
                            ypos = 1 as c_uint;
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
    while LZWDecodeByte(LZW_FALSE, minCodeSize, stream) >= 0 as c_int {}
}
static mut DataBlockSize: c_int = 0 as c_int;
unsafe fn GIFReadDataBlock(
    mut buffer: *mut c_uchar,
    mut stream: *mut FILE,
) -> c_int {
    let mut count: c_int = 0;
    count = GetByte(stream);
    DataBlockSize = count;
    if count > 0 as c_int {
        ReadBytes(buffer, count as c_uint, stream);
    }
    return count;
}
unsafe fn GIFSkipDataBlocks(mut stream: *mut FILE) {
    let mut count: c_int = 0;
    let mut buffer: [c_uchar; 256] = [0; 256];
    loop {
        count = GetByte(stream);
        if count > 0 as c_int {
            ReadBytes(
                &raw mut buffer as *mut c_uchar,
                count as c_uint,
                stream,
            );
        } else {
            return;
        }
    }
}
unsafe fn LZWGetCode(
    mut code_size: c_int,
    mut init_flag: c_int,
    mut stream: *mut FILE,
) -> c_int {
    static mut buffer: [c_uchar; 280] = [0; 280];
    static mut curbit: c_int = 0;
    static mut lastbit: c_int = 0;
    static mut done: c_int = 0;
    static mut last_byte: c_int = 0;
    let mut count: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut ret: c_int = 0;
    if init_flag != 0 {
        curbit = 0 as c_int;
        lastbit = 0 as c_int;
        last_byte = 2 as c_int;
        done = LZW_FALSE;
        return 0 as c_int;
    }
    if curbit + code_size >= lastbit {
        if done != 0 {
            if curbit >= lastbit {
                GIFError.expect("non-null function pointer")(
                    b"Ran off the end of input bits in LZW decoding\0" as *const u8
                        as *const c_char,
                );
            }
            return -(1 as c_int);
        }
        buffer[0 as c_int as usize] =
            buffer[(last_byte - 2 as c_int) as usize];
        buffer[1 as c_int as usize] =
            buffer[(last_byte - 1 as c_int) as usize];
        count = GIFReadDataBlock(
            (&raw mut buffer as *mut c_uchar).offset(2 as c_int as isize)
                as *mut c_uchar,
            stream,
        );
        if count == 0 as c_int {
            done = LZW_TRUE;
        }
        last_byte = 2 as c_int + count;
        curbit = curbit - lastbit + 16 as c_int;
        lastbit = (2 as c_int + count) * 8 as c_int;
    }
    ret = 0 as c_int;
    i = curbit;
    j = 0 as c_int;
    while j < code_size {
        ret |= ((buffer[(i / 8 as c_int) as usize] as c_int
            & (1 as c_int) << i % 8 as c_int
            != 0 as c_int) as c_int)
            << j;
        i += 1;
        j += 1;
    }
    curbit += code_size;
    return ret;
}
unsafe fn LZWDecodeByte(
    mut init_flag: c_int,
    mut input_code_size: c_int,
    mut stream: *mut FILE,
) -> c_int {
    static mut fresh: c_int = LZW_FALSE;
    static mut code_size: c_int = 0;
    static mut set_code_size: c_int = 0;
    static mut max_code: c_int = 0;
    static mut max_code_size: c_int = 0;
    static mut firstcode: c_int = 0;
    static mut oldcode: c_int = 0;
    static mut clear_code: c_int = 0;
    static mut end_code: c_int = 0;
    static mut table: [[c_int; 4096]; 2] = [[0; 4096]; 2];
    static mut stack: [c_int; 8192] = [0; 8192];
    static mut sp: *mut c_int =
        ::core::ptr::null::<c_int>() as *mut c_int;
    let mut code: c_int = 0;
    let mut incode: c_int = 0;
    let mut i: c_int = 0;
    if init_flag != 0 {
        fresh = LZW_TRUE;
        set_code_size = input_code_size;
        code_size = set_code_size + 1 as c_int;
        clear_code = (1 as c_int) << set_code_size;
        end_code = clear_code + 1 as c_int;
        max_code_size = 2 as c_int * clear_code;
        max_code = clear_code + 2 as c_int;
        LZWGetCode(0 as c_int, LZW_TRUE, stream);
        i = 0 as c_int;
        while i < clear_code {
            table[0 as c_int as usize][i as usize] = 0 as c_int;
            table[1 as c_int as usize][i as usize] = i;
            i += 1;
        }
        while i <= LZW_CODE_MAX {
            table[0 as c_int as usize][i as usize] = 0 as c_int;
            table[1 as c_int as usize][i as usize] = 0 as c_int;
            i += 1;
        }
        sp = &raw mut stack as *mut c_int;
        return 0 as c_int;
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
    if sp > &raw mut stack as *mut c_int {
        sp = sp.offset(-1);
        return *sp;
    }
    loop {
        code = LZWGetCode(code_size, LZW_FALSE, stream);
        if !(code >= 0 as c_int) {
            break;
        }
        if code == clear_code {
            i = 0 as c_int;
            while i < clear_code {
                table[0 as c_int as usize][i as usize] = 0 as c_int;
                table[1 as c_int as usize][i as usize] = i;
                i += 1;
            }
            while i <= LZW_CODE_MAX {
                table[0 as c_int as usize][i as usize] = 0 as c_int;
                table[1 as c_int as usize][i as usize] = 0 as c_int;
                i += 1;
            }
            code_size = set_code_size + 1 as c_int;
            max_code_size = 2 as c_int * clear_code;
            max_code = clear_code + 2 as c_int;
            sp = &raw mut stack as *mut c_int;
            oldcode = LZWGetCode(code_size, LZW_FALSE, stream);
            firstcode = oldcode;
            return firstcode;
        } else if code == end_code {
            let mut count: c_int = 0;
            let mut buffer: [c_uchar; 260] = [0; 260];
            if DataBlockSize == 0 as c_int {
                return -(2 as c_int);
            }
            loop {
                count = GIFReadDataBlock(&raw mut buffer as *mut c_uchar, stream);
                if !(count > 0 as c_int) {
                    break;
                }
            }
            return -(2 as c_int);
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
            *fresh1 = table[1 as c_int as usize][code as usize];
            if code == table[0 as c_int as usize][code as usize]
                || sp.offset_from(&raw mut stack as *mut c_int) as c_long
                    as size_t
                    >= (::core::mem::size_of::<[c_int; 8192]>() as usize)
                        .wrapping_div(::core::mem::size_of::<c_int>() as usize)
            {
                GIFError.expect("non-null function pointer")(
                    b"Circular dependency found in LZW table\0" as *const u8
                        as *const c_char,
                );
            }
            code = table[0 as c_int as usize][code as usize];
        }
        firstcode = table[1 as c_int as usize][code as usize];
        let fresh2 = sp;
        sp = sp.offset(1);
        *fresh2 = firstcode;
        code = max_code;
        if code <= LZW_CODE_MAX {
            table[0 as c_int as usize][code as usize] = oldcode;
            table[1 as c_int as usize][code as usize] = firstcode;
            max_code += 1;
            if max_code >= max_code_size && max_code_size <= LZW_CODE_MAX {
                max_code_size *= 2 as c_int;
                code_size += 1;
            }
        }
        oldcode = incode;
        if sp > &raw mut stack as *mut c_int {
            sp = sp.offset(-1);
            return *sp;
        }
    }
    return code;
}
static mut DefaultColorTable: [c_uchar; 24] = [
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    255 as c_int as c_uchar,
    255 as c_int as c_uchar,
    255 as c_int as c_uchar,
    255 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    255 as c_int as c_uchar,
    255 as c_int as c_uchar,
    0 as c_int as c_uchar,
    255 as c_int as c_uchar,
    0 as c_int as c_uchar,
    255 as c_int as c_uchar,
    0 as c_int as c_uchar,
    255 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    255 as c_int as c_uchar,
    255 as c_int as c_uchar,
    255 as c_int as c_uchar,
    0 as c_int as c_uchar,
];
#[inline]
pub unsafe fn GIFGetColorTable(
    mut colors: *mut *mut c_uchar,
    mut numColors: *mut c_uint,
    mut image: *mut GIFImage,
) {
    let numColors_view: &mut c_uint = unsafe { &mut *numColors };
    let mut screen: *mut GIFScreen = ::core::ptr::null_mut::<GIFScreen>();
    if (*image).LocalColorFlag != 0 {
        *colors = &raw mut (*image).LocalColorTable as *mut c_uchar;
        *numColors_view = (*image).LocalNumColors;
        return;
    }
    screen = (*image).Screen;
    if (*screen).GlobalColorFlag != 0 {
        *colors = &raw mut (*screen).GlobalColorTable as *mut c_uchar;
        *numColors_view = (*screen).GlobalNumColors;
        return;
    }
    *colors = &raw mut DefaultColorTable as *mut c_uchar;
    *numColors_view = (::core::mem::size_of::<[c_uchar; 24]>() as usize)
        .wrapping_div(3 as usize) as c_uint;
}
#[inline]
pub unsafe fn GIFInitExtension(
    mut ext: *mut GIFExtension,
    mut screen: *mut GIFScreen,
    mut initBufferSize: c_uint,
) {
    let mut newBuffer: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    (*ext).Screen = screen;
    if initBufferSize > 0 as c_uint {
        newBuffer = malloc(initBufferSize as size_t) as *mut c_uchar;
        if newBuffer.is_null() {
            ErrorAlloc();
        }
        (*ext).Buffer = newBuffer;
        (*ext).BufferSize = initBufferSize;
    } else {
        (*ext).Buffer = ::core::ptr::null_mut::<c_uchar>();
        (*ext).BufferSize = 0 as c_uint;
    };
}
#[inline]
pub unsafe fn GIFDestroyExtension(mut ext: *mut GIFExtension) {
    let ext_view: &GIFExtension = unsafe { &*ext };
    free(ext_view.Buffer as *mut c_void);
}
unsafe fn GIFReadNextExtension(mut ext: *mut GIFExtension, mut stream: *mut FILE) {
    let mut newBuffer: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut newBufferSize: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut len: c_uint = 0;
    let mut count: c_int = 0;
    let mut label: c_int = 0;
    label = GetByte(stream);
    if ext.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    (*ext).Label = label as c_uchar;
    offset = 0 as c_uint;
    len = (*ext).BufferSize;
    loop {
        if len < 255 as c_uint {
            newBufferSize = (*ext).BufferSize.wrapping_add(1024 as c_uint);
            newBuffer = realloc(
                (*ext).Buffer as *mut c_void,
                newBufferSize as size_t,
            ) as *mut c_uchar;
            if newBuffer.is_null() {
                ErrorAlloc();
            }
            (*ext).BufferSize = newBufferSize;
            (*ext).Buffer = newBuffer;
            len = len.wrapping_add(1024 as c_uint);
        }
        count = GIFReadDataBlock((*ext).Buffer.offset(offset as isize), stream);
        if count == 0 as c_int {
            break;
        }
        offset = offset.wrapping_add(count as c_uint);
        len = len.wrapping_sub(count as c_uint);
    }
}
#[inline]
pub unsafe fn GIFGetGraphicCtl(
    mut graphicExt: *mut GIFGraphicCtlExt,
    mut ext: *mut GIFExtension,
) {
    let ext_view: &GIFExtension = unsafe { &*ext };
    let mut buffer: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    if ext_view.Label as c_int != GIF_GRAPHICCTL {
        GIFWarning.expect("non-null function pointer")(
            b"Not a graphic control extension in GIF file\0" as *const u8
                as *const c_char,
        );
        return;
    }
    if ext_view.BufferSize < 4 as c_uint {
        GIFWarning.expect("non-null function pointer")(
            b"Broken graphic control extension in GIF file\0" as *const u8
                as *const c_char,
        );
        return;
    }
    buffer = ext_view.Buffer;
    (*graphicExt).DisposalMethod = (*buffer.offset(0 as c_int as isize)
        as c_int
        >> 2 as c_int
        & 0x7 as c_int) as c_uint;
    (*graphicExt).InputFlag = (*buffer.offset(0 as c_int as isize)
        as c_int
        >> 1 as c_int
        & 0x1 as c_int) as c_uint;
    (*graphicExt).TransparentFlag = (*buffer.offset(0 as c_int as isize)
        as c_int
        & 0x1 as c_int) as c_uint;
    (*graphicExt).DelayTime = (*buffer
        .offset(1 as c_int as isize)
        .offset(0 as c_int as isize)
        as c_int
        + ((*buffer
            .offset(1 as c_int as isize)
            .offset(1 as c_int as isize) as c_int)
            << 8 as c_int)) as c_uint;
    (*graphicExt).Transparent =
        *buffer.offset(3 as c_int as isize) as c_uint;
}
unsafe fn GetByte(mut stream: *mut FILE) -> c_int {
    let mut ch: c_int = 0;
    ch = getc(stream);
    if ch == EOF {
        ErrorRead(stream);
    }
    return ch;
}
unsafe fn ReadBytes(
    mut buffer: *mut c_uchar,
    mut count: c_uint,
    mut stream: *mut FILE,
) {
    if fread(
        buffer as *mut c_void,
        count as size_t,
        1 as size_t,
        stream,
    ) != 1 as c_ulong
    {
        ErrorRead(stream);
    }
}
fn ErrorAlloc() { unsafe {
    GIFError.expect("non-null function pointer")(
        b"Out of memory in GIF decoder\0" as *const u8 as *const c_char,
    );
} }
unsafe fn ErrorRead(mut stream: *mut FILE) {
    if ferror(stream) != 0 {
        GIFError.expect("non-null function pointer")(
            b"Error reading GIF file\0" as *const u8 as *const c_char,
        );
    } else {
        GIFError.expect("non-null function pointer")(
            b"Unexpected end of GIF file\0" as *const u8 as *const c_char,
        );
    };
}
unsafe extern "C" fn DefaultError(mut message: *const c_char) {
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const c_char,
        message,
    );
    exit(EXIT_FAILURE);
}
unsafe extern "C" fn DefaultWarning(mut message: *const c_char) {
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const c_char,
        message,
    );
}
#[no_mangle]
pub static mut GIFError: Option<unsafe extern "C" fn(*const c_char) -> ()> =
    Some(DefaultError as unsafe extern "C" fn(*const c_char) -> ());
#[no_mangle]
pub static mut GIFWarning: Option<unsafe extern "C" fn(*const c_char) -> ()> =
    Some(DefaultWarning as unsafe extern "C" fn(*const c_char) -> ());
