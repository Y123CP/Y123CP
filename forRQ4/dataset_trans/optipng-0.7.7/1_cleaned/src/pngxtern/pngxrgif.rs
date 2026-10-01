use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
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
    fn png_set_PLTE(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        palette: png_const_colorp,
        num_palette: c_int,
    );
    fn png_set_tRNS(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        trans_alpha: png_const_bytep,
        num_trans: c_int,
        trans_color: png_const_color_16p,
    );
    fn pngx_set_interlace_type(
        png_ptr: png_structp,
        info_ptr: png_infop,
        interlace_type: c_int,
    );
    fn pngx_malloc_rows(
        png_ptr: png_structp,
        info_ptr: png_infop,
        filler: c_int,
    ) -> png_bytepp;
    fn GIFReadScreen(screen: *mut GIFScreen, stream: *mut FILE);
    fn GIFInitImage(
        image: *mut GIFImage,
        screen: *mut GIFScreen,
        rows: *mut *mut c_uchar,
    );
    fn GIFDestroyImage(image: *mut GIFImage);
    fn GIFReadNextBlock(
        image: *mut GIFImage,
        ext: *mut GIFExtension,
        stream: *mut FILE,
    ) -> c_int;
    fn GIFGetColorTable(
        colors: *mut *mut c_uchar,
        numColors: *mut c_uint,
        image: *mut GIFImage,
    );
    fn GIFInitExtension(
        ext: *mut GIFExtension,
        screen: *mut GIFScreen,
        initBufferSize: c_uint,
    );
    fn GIFDestroyExtension(ext: *mut GIFExtension);
    fn GIFGetGraphicCtl(graphicExt: *mut GIFGraphicCtlExt, ext: *mut GIFExtension);
    static mut GIFError: Option<unsafe extern "C" fn(*const c_char) -> ()>;
    static mut GIFWarning: Option<unsafe extern "C" fn(*const c_char) -> ()>;
}

pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;

pub const PNG_COLOR_TYPE_PALETTE: c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;

pub const GIF_EXTENSION: c_int = 0x21 as c_int;
pub const GIF_IMAGE: c_int = 0x2c as c_int;
pub const GIF_TERMINATOR: c_int = 0x3b as c_int;

static mut gif_fmt_name: [c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"GIF\0") };
static mut gif_fmt_long_name: [c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [c_char; 28]>(*b"Graphics Interchange Format\0")
};
static mut gif_sig_gif87a: [png_byte; 6] = [
    0x47 as c_int as png_byte,
    0x49 as c_int as png_byte,
    0x46 as c_int as png_byte,
    0x38 as c_int as png_byte,
    0x37 as c_int as png_byte,
    0x61 as c_int as png_byte,
];
static mut gif_sig_gif89a: [png_byte; 6] = [
    0x47 as c_int as png_byte,
    0x49 as c_int as png_byte,
    0x46 as c_int as png_byte,
    0x38 as c_int as png_byte,
    0x39 as c_int as png_byte,
    0x61 as c_int as png_byte,
];
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_gif(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> c_int {
    if sig_size < (6 as c_int + 7 as c_int) as size_t {
        return -(1 as c_int);
    }
    if memcmp(
        sig as *const c_void,
        &raw const gif_sig_gif87a as *const png_byte as *const c_void,
        6 as size_t,
    ) != 0 as c_int
        && memcmp(
            sig as *const c_void,
            &raw const gif_sig_gif89a as *const png_byte as *const c_void,
            6 as size_t,
        ) != 0 as c_int
    {
        return 0 as c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = &raw const gif_fmt_name as *const c_char;
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = &raw const gif_fmt_long_name as *const c_char;
    }
    return 1 as c_int;
}
static mut err_png_ptr: png_structp = ::core::ptr::null::<png_struct>() as *mut png_struct;
static mut err_gif_image_ptr: *mut GIFImage = ::core::ptr::null::<GIFImage>() as *mut GIFImage;
static mut err_gif_ext_ptr: *mut GIFExtension =
    ::core::ptr::null::<GIFExtension>() as *mut GIFExtension;
unsafe extern "C" fn pngx_gif_error(mut msg: *const c_char) {
    if !err_gif_image_ptr.is_null() {
        GIFDestroyImage(err_gif_image_ptr);
    }
    if !err_gif_ext_ptr.is_null() {
        GIFDestroyExtension(err_gif_ext_ptr);
    }
    png_error(err_png_ptr as png_const_structrp, msg as png_const_charp);
}
unsafe extern "C" fn pngx_gif_warning(mut msg: *const c_char) {
    png_warning(err_png_ptr as png_const_structrp, msg as png_const_charp);
}
unsafe extern "C" fn pngx_set_gif_palette(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut color_table: *mut c_uchar,
    mut num_colors: c_uint,
) {
    let mut palette: [png_color; 256] = [png_color {
        red: 0,
        green: 0,
        blue: 0,
    }; 256];
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < num_colors {
        palette[i as usize].red =
            *color_table.offset((3 as c_uint).wrapping_mul(i) as isize) as png_byte;
        palette[i as usize].green = *color_table.offset(
            (3 as c_uint)
                .wrapping_mul(i)
                .wrapping_add(1 as c_uint) as isize,
        ) as png_byte;
        palette[i as usize].blue = *color_table.offset(
            (3 as c_uint)
                .wrapping_mul(i)
                .wrapping_add(2 as c_uint) as isize,
        ) as png_byte;
        i = i.wrapping_add(1);
    }
    png_set_PLTE(
        png_ptr as png_structrp,
        info_ptr as png_inforp,
        &raw mut palette as *mut png_color as png_const_colorp,
        num_colors as c_int,
    );
}
unsafe extern "C" fn pngx_set_gif_transparent(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut transparent: c_uint,
) {
    let mut trans: [png_byte; 256] = [0; 256];
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < transparent {
        trans[i as usize] = 255 as png_byte;
        i = i.wrapping_add(1);
    }
    trans[transparent as usize] = 0 as png_byte;
    png_set_tRNS(
        png_ptr as png_structrp,
        info_ptr as png_inforp,
        &raw mut trans as *mut png_byte as png_const_bytep,
        transparent as c_int + 1 as c_int,
        ::core::ptr::null::<png_color_16>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_gif(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> c_int {
    let mut screen: GIFScreen = GIFScreen {
        Width: 0,
        Height: 0,
        GlobalColorFlag: 0,
        ColorResolution: 0,
        SortFlag: 0,
        GlobalNumColors: 0,
        Background: 0,
        PixelAspectRatio: 0,
        GlobalColorTable: [0; 768],
    };
    let mut image: GIFImage = GIFImage {
        Screen: ::core::ptr::null_mut::<GIFScreen>(),
        LeftPos: 0,
        TopPos: 0,
        Width: 0,
        Height: 0,
        LocalColorFlag: 0,
        InterlaceFlag: 0,
        SortFlag: 0,
        LocalNumColors: 0,
        LocalColorTable: [0; 768],
        Rows: ::core::ptr::null_mut::<*mut c_uchar>(),
    };
    let mut ext: GIFExtension = GIFExtension {
        Screen: ::core::ptr::null_mut::<GIFScreen>(),
        Buffer: ::core::ptr::null_mut::<c_uchar>(),
        BufferSize: 0,
        Label: 0,
    };
    let mut graphicExt: GIFGraphicCtlExt = GIFGraphicCtlExt {
        DisposalMethod: 0,
        InputFlag: 0,
        TransparentFlag: 0,
        DelayTime: 0,
        Transparent: 0,
    };
    let mut blockCode: c_int = 0;
    let mut colorTable: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut numColors: c_uint = 0;
    let mut transparent: c_uint = 0;
    let mut numImages: c_uint = 0;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut row_pointers: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    GIFError = Some(pngx_gif_error as unsafe extern "C" fn(*const c_char) -> ())
        as Option<unsafe extern "C" fn(*const c_char) -> ()>;
    GIFWarning = Some(pngx_gif_warning as unsafe extern "C" fn(*const c_char) -> ())
        as Option<unsafe extern "C" fn(*const c_char) -> ()>;
    err_png_ptr = png_ptr;
    err_gif_image_ptr = ::core::ptr::null_mut::<GIFImage>();
    err_gif_ext_ptr = ::core::ptr::null_mut::<GIFExtension>();
    GIFReadScreen(&raw mut screen, stream);
    width = screen.Width as png_uint_32;
    height = screen.Height as png_uint_32;
    png_set_IHDR(
        png_ptr as png_const_structrp,
        info_ptr as png_inforp,
        width,
        height,
        8 as c_int,
        PNG_COLOR_TYPE_PALETTE,
        PNG_INTERLACE_NONE,
        PNG_COMPRESSION_TYPE_BASE,
        PNG_FILTER_TYPE_BASE,
    );
    row_pointers = pngx_malloc_rows(png_ptr, info_ptr, screen.Background as c_int);
    GIFInitImage(
        &raw mut image,
        &raw mut screen,
        row_pointers as *mut *mut c_uchar,
    );
    err_gif_image_ptr = &raw mut image;
    GIFInitExtension(&raw mut ext, &raw mut screen, 256 as c_uint);
    err_gif_ext_ptr = &raw mut ext;
    numImages = 0 as c_uint;
    transparent = -(1 as c_int) as c_uint;
    loop {
        blockCode = GIFReadNextBlock(&raw mut image, &raw mut ext, stream);
        if blockCode == GIF_IMAGE {
            if !image.Rows.is_null() {
                if image.InterlaceFlag != 0 {
                    pngx_set_interlace_type(png_ptr, info_ptr, PNG_INTERLACE_ADAM7);
                }
                GIFGetColorTable(&raw mut colorTable, &raw mut numColors, &raw mut image);
                pngx_set_gif_palette(png_ptr, info_ptr, colorTable, numColors);
                if transparent < 256 as c_uint {
                    pngx_set_gif_transparent(png_ptr, info_ptr, transparent);
                }
                image.Rows = ::core::ptr::null_mut::<*mut c_uchar>();
            }
            numImages = numImages.wrapping_add(1);
        } else if blockCode == GIF_EXTENSION {
            if ext.Label as c_int == GIF_GRAPHICCTL {
                GIFGetGraphicCtl(&raw mut graphicExt, &raw mut ext);
                if !image.Rows.is_null() && graphicExt.TransparentFlag != 0 {
                    if transparent >= 256 as c_uint {
                        transparent = graphicExt.Transparent;
                    }
                }
            }
        } else if blockCode == GIF_TERMINATOR {
            break;
        }
    }
    if !image.Rows.is_null() {
        png_error(
            png_ptr as png_const_structrp,
            b"No image in GIF file\0" as *const u8 as png_const_charp,
        );
    }
    GIFDestroyImage(&raw mut image);
    GIFDestroyExtension(&raw mut ext);
    return numImages as c_int;
}
