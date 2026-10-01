use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    fn minitiff_error(info_ptr: *const minitiff_info, msg: *const c_char);
    fn minitiff_warning(info_ptr: *const minitiff_info, msg: *const c_char);
    static minitiff_sig_bigm: [c_char; 4];
    static minitiff_sig_bigi: [c_char; 4];
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct minitiff_getter {
    pub get_ushort:
        Option<unsafe extern "C" fn(*const c_uchar) -> c_uint>,
    pub get_ulong:
        Option<unsafe extern "C" fn(*const c_uchar) -> c_ulong>,
}
pub const MINITIFF_TYPE_LONG: C2RustUnnamed = 4;
pub const MINITIFF_TYPE_SHORT: C2RustUnnamed = 3;
pub const MINITIFF_TYPE_BYTE: C2RustUnnamed = 1;
pub const MINITIFF_TAG_PRINT_IM: C2RustUnnamed_0 = 50341;
pub const MINITIFF_TAG_INTEROPERABILITY_IFD: C2RustUnnamed_0 = 40965;
pub const MINITIFF_TAG_GPS_IFD: C2RustUnnamed_0 = 34853;
pub const MINITIFF_TAG_ICC_PROFILE: C2RustUnnamed_0 = 34675;
pub const MINITIFF_TAG_EXIF_IFD: C2RustUnnamed_0 = 34665;
pub const MINITIFF_TAG_IPTC: C2RustUnnamed_0 = 33723;
pub const MINITIFF_TAG_XMP: C2RustUnnamed_0 = 700;
pub const MINITIFF_TAG_PREDICTOR: C2RustUnnamed_0 = 317;
pub const MINITIFF_TAG_PLANAR_CONFIGURATION: C2RustUnnamed_0 = 284;
pub const MINITIFF_TAG_STRIP_BYTE_COUNTS: C2RustUnnamed_0 = 279;
pub const MINITIFF_TAG_ROWS_PER_STRIP: C2RustUnnamed_0 = 278;
pub const MINITIFF_TAG_SAMPLES_PER_PIXEL: C2RustUnnamed_0 = 277;
pub const MINITIFF_TAG_ORIENTATION: C2RustUnnamed_0 = 274;
pub const MINITIFF_TAG_STRIP_OFFSETS: C2RustUnnamed_0 = 273;
pub const MINITIFF_TAG_PHOTOMETRIC: C2RustUnnamed_0 = 262;
pub const MINITIFF_TAG_COMPRESSION: C2RustUnnamed_0 = 259;
pub const MINITIFF_TAG_BITS_PER_SAMPLE: C2RustUnnamed_0 = 258;
pub const MINITIFF_TAG_HEIGHT: C2RustUnnamed_0 = 257;
pub const MINITIFF_TAG_WIDTH: C2RustUnnamed_0 = 256;
pub const MINITIFF_TAG_SUBFILE_TYPE: C2RustUnnamed_0 = 254;
pub type C2RustUnnamed = c_uint;
pub const MINITIFF_TYPE_IFD64: C2RustUnnamed = 18;
pub const MINITIFF_TYPE_SLONG64: C2RustUnnamed = 17;
pub const MINITIFF_TYPE_LONG64: C2RustUnnamed = 16;
pub const MINITIFF_TYPE_IFD: C2RustUnnamed = 13;
pub const MINITIFF_TYPE_DOUBLE: C2RustUnnamed = 12;
pub const MINITIFF_TYPE_FLOAT: C2RustUnnamed = 11;
pub const MINITIFF_TYPE_SRATIONAL: C2RustUnnamed = 10;
pub const MINITIFF_TYPE_SLONG: C2RustUnnamed = 9;
pub const MINITIFF_TYPE_SSHORT: C2RustUnnamed = 8;
pub const MINITIFF_TYPE_UNDEFINED: C2RustUnnamed = 7;
pub const MINITIFF_TYPE_SBYTE: C2RustUnnamed = 6;
pub const MINITIFF_TYPE_RATIONAL: C2RustUnnamed = 5;
pub const MINITIFF_TYPE_ASCII: C2RustUnnamed = 2;
pub const MINITIFF_TYPE_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = c_uint;
pub const MINITIFF_TAG_COPYRIGHT: C2RustUnnamed_0 = 33432;
pub const MINITIFF_TAG_BYTE_COUNTS: C2RustUnnamed_0 = 325;
pub const MINITIFF_TAG_TILE_OFFSETS: C2RustUnnamed_0 = 324;
pub const MINITIFF_TAG_TILE_LENGTH: C2RustUnnamed_0 = 323;
pub const MINITIFF_TAG_TILE_WIDTH: C2RustUnnamed_0 = 322;
pub const MINITIFF_TAG_HALFTONE_HINTS: C2RustUnnamed_0 = 321;
pub const MINITIFF_TAG_COLOR_MAP: C2RustUnnamed_0 = 320;
pub const MINITIFF_TAG_PRIMARY_CHROMATICITIES: C2RustUnnamed_0 = 319;
pub const MINITIFF_TAG_WHITE_POINT: C2RustUnnamed_0 = 318;
pub const MINITIFF_TAG_HOST_COMPUTER: C2RustUnnamed_0 = 316;
pub const MINITIFF_TAG_ARTIST: C2RustUnnamed_0 = 315;
pub const MINITIFF_TAG_DATE_TIME: C2RustUnnamed_0 = 306;
pub const MINITIFF_TAG_SOFTWARE: C2RustUnnamed_0 = 305;
pub const MINITIFF_TAG_TRANSFER_FUNCTION: C2RustUnnamed_0 = 301;
pub const MINITIFF_TAG_PAGE_NUMBER: C2RustUnnamed_0 = 297;
pub const MINITIFF_TAG_RESOLUTION_UNIT: C2RustUnnamed_0 = 296;
pub const MINITIFF_TAG_Y_POSITION: C2RustUnnamed_0 = 287;
pub const MINITIFF_TAG_X_POSITION: C2RustUnnamed_0 = 286;
pub const MINITIFF_TAG_PAGE_NAME: C2RustUnnamed_0 = 285;
pub const MINITIFF_TAG_Y_RESOLUTION: C2RustUnnamed_0 = 283;
pub const MINITIFF_TAG_X_RESOLUTION: C2RustUnnamed_0 = 282;
pub const MINITIFF_TAG_MAX_SAMPLE_VALUE: C2RustUnnamed_0 = 281;
pub const MINITIFF_TAG_MIN_SAMPLE_VALUE: C2RustUnnamed_0 = 280;
pub const MINITIFF_TAG_MODEL: C2RustUnnamed_0 = 272;
pub const MINITIFF_TAG_MAKE: C2RustUnnamed_0 = 271;
pub const MINITIFF_TAG_IMAGE_DESCRIPTION: C2RustUnnamed_0 = 270;
pub const MINITIFF_TAG_DOCUMENT_NAME: C2RustUnnamed_0 = 269;
pub const MINITIFF_TAG_FILL_ORDER: C2RustUnnamed_0 = 266;
pub const MINITIFF_TAG_CELL_LENGTH: C2RustUnnamed_0 = 265;
pub const MINITIFF_TAG_CELL_WIDTH: C2RustUnnamed_0 = 264;
pub const MINITIFF_TAG_THRESHOLDING: C2RustUnnamed_0 = 263;
pub const MINITIFF_TAG_OLD_SUBFILE_TYPE: C2RustUnnamed_0 = 255;

static mut msg_err_alloc: *const c_char =
    b"Error allocating memory for TIFF file\0" as *const u8 as *const c_char;
static mut msg_err_read: *const c_char =
    b"Error reading TIFF file\0" as *const u8 as *const c_char;
static mut msg_err_seek: *const c_char =
    b"Error repositioning TIFF file\0" as *const u8 as *const c_char;
static mut msg_err_notiff: *const c_char =
    b"Not a TIFF file\0" as *const u8 as *const c_char;
static mut msg_err_bigtiff: *const c_char =
    b"Unsupported BigTIFF file\0" as *const u8 as *const c_char;
static mut msg_err_invalid: *const c_char =
    b"Invalid TIFF file\0" as *const u8 as *const c_char;
static mut msg_err_range: *const c_char =
    b"Value out of the supported range in TIFF file\0" as *const u8 as *const c_char;
static mut msg_err_unsupported: *const c_char =
    b"Unsupported data in TIFF file\0" as *const u8 as *const c_char;
static mut msg_warn_metadata: *const c_char =
    b"Unrecognized metadata in TIFF file\0" as *const u8 as *const c_char;
static mut msg_warn_multiple: *const c_char =
    b"Selected first image from multi-image TIFF file\0" as *const u8 as *const c_char;
unsafe extern "C" fn get_ushort_m(mut buf_ptr: *const c_uchar) -> c_uint {
    return ((*buf_ptr.offset(0 as c_int as isize) as c_uint)
        << 8 as c_int)
        .wrapping_add(*buf_ptr.offset(1 as c_int as isize) as c_uint);
}
unsafe extern "C" fn get_ushort_i(mut buf_ptr: *const c_uchar) -> c_uint {
    return (*buf_ptr.offset(0 as c_int as isize) as c_uint)
        .wrapping_add(
            (*buf_ptr.offset(1 as c_int as isize) as c_uint)
                << 8 as c_int,
        );
}
unsafe extern "C" fn get_ulong_m(mut buf_ptr: *const c_uchar) -> c_ulong {
    return ((*buf_ptr.offset(0 as c_int as isize) as c_ulong)
        << 24 as c_int)
        .wrapping_add(
            (*buf_ptr.offset(1 as c_int as isize) as c_ulong)
                << 16 as c_int,
        )
        .wrapping_add(
            (*buf_ptr.offset(2 as c_int as isize) as c_ulong)
                << 8 as c_int,
        )
        .wrapping_add(*buf_ptr.offset(3 as c_int as isize) as c_ulong);
}
unsafe extern "C" fn get_ulong_i(mut buf_ptr: *const c_uchar) -> c_ulong {
    return (*buf_ptr.offset(0 as c_int as isize) as c_ulong)
        .wrapping_add(
            (*buf_ptr.offset(1 as c_int as isize) as c_ulong)
                << 8 as c_int,
        )
        .wrapping_add(
            (*buf_ptr.offset(2 as c_int as isize) as c_ulong)
                << 16 as c_int,
        )
        .wrapping_add(
            (*buf_ptr.offset(3 as c_int as isize) as c_ulong)
                << 24 as c_int,
        );
}
unsafe extern "C" fn get_ulong_value(
    mut getter_ptr: *const minitiff_getter,
    mut tag_type: c_int,
    mut buf_ptr: *const c_uchar,
) -> c_ulong {
    match tag_type {
        1 => {
            return *buf_ptr.offset(0 as c_int as isize) as c_ulong;
        }
        3 => {
            return (*getter_ptr).get_ushort.expect("non-null function pointer")(buf_ptr)
                as c_ulong;
        }
        4 => return (*getter_ptr).get_ulong.expect("non-null function pointer")(buf_ptr),
        _ => return -(1 as c_long) as c_ulong,
    };
}
unsafe extern "C" fn alloc_ulong_array(
    mut info_ptr: *mut minitiff_info,
    mut count: size_t,
) -> *mut c_ulong {
    let mut result: *mut c_ulong = ::core::ptr::null_mut::<c_ulong>();
    if count
        > (-(1 as c_int) as size_t)
            .wrapping_div(::core::mem::size_of::<c_ulong>() as size_t)
    {
        minitiff_error(info_ptr, msg_err_range);
    }
    result = malloc(count.wrapping_mul(::core::mem::size_of::<c_ulong>() as size_t))
        as *mut c_ulong;
    if result.is_null() {
        minitiff_error(info_ptr, msg_err_alloc);
    }
    return result;
}
unsafe extern "C" fn cast_ulong_to_uint(
    mut info_ptr: *mut minitiff_info,
    mut value: c_ulong,
) -> c_uint {
    let mut result: c_uint = value as c_uint;
    if result as c_ulong != value {
        minitiff_error(info_ptr, msg_err_range);
    }
    return result;
}
unsafe extern "C" fn read_ulong_values(
    mut getter_ptr: *const minitiff_getter,
    mut tag_type: c_int,
    mut values: *mut c_ulong,
    mut count: size_t,
    mut stream: *mut FILE,
) -> size_t {
    let mut buf: [c_uchar; 4] = [0; 4];
    let mut value_size: size_t = 0;
    let mut i: size_t = 0;
    match tag_type {
        1 => {
            value_size = 1 as size_t;
        }
        3 => {
            value_size = 2 as size_t;
        }
        4 => {
            value_size = 4 as size_t;
        }
        _ => return 0 as size_t,
    }
    i = 0 as size_t;
    while i < count {
        if fread(
            &raw mut buf as *mut c_uchar as *mut c_void,
            value_size,
            1 as size_t,
            stream,
        ) != 1 as c_ulong
        {
            break;
        }
        *values.offset(i as isize) = get_ulong_value(
            getter_ptr,
            tag_type,
            &raw mut buf as *mut c_uchar,
        );
        i = i.wrapping_add(1);
    }
    return i;
}
unsafe extern "C" fn seek_to_offset(
    mut info_ptr: *mut minitiff_info,
    mut offset: c_long,
    mut stream: *mut FILE,
) {
    if offset < 0 as c_long {
        minitiff_error(info_ptr, msg_err_range);
    }
    if ftell(stream) == offset {
        return;
    }
    if fseek(stream, offset, SEEK_SET) != 0 as c_int {
        minitiff_error(info_ptr, msg_err_seek);
    }
}
#[no_mangle]
pub unsafe extern "C" fn minitiff_read_info(
    mut info_ptr: *mut minitiff_info,
    mut stream: *mut FILE,
) {
    let mut current_block: u64;
    let mut getter: minitiff_getter = minitiff_getter {
        get_ushort: None,
        get_ulong: None,
    };
    let mut buf: [c_uchar; 12] = [0; 12];
    let mut vbuf: *mut c_uchar =
        (&raw mut buf as *mut c_uchar).offset(8 as c_int as isize);
    let mut ulvals: [c_ulong; 4] = [0; 4];
    let mut ulval: c_ulong = 0;
    let mut dir_offset: c_long = 0;
    let mut dir_size: c_uint = 0;
    let mut i: c_uint = 0;
    let mut tag_id: c_uint = 0;
    let mut tag_type: c_uint = 0;
    let mut count: size_t = 0;
    let mut bits_per_sample_count: size_t = 0;
    let mut bits_per_sample_tag_type: c_uint = 0;
    let mut strip_offsets_tag_type: c_uint = 0;
    let mut bits_per_sample_offset: c_long = 0;
    let mut strip_offsets_offset: c_long = 0;
    let mut unknown_metadata_found: c_int = 0;
    if fread(
        &raw mut buf as *mut c_uchar as *mut c_void,
        8 as size_t,
        1 as size_t,
        stream,
    ) != 1 as c_ulong
    {
        current_block = 4547171438666572104;
    } else {
        if memcmp(
            &raw mut buf as *mut c_uchar as *const c_void,
            &raw const minitiff_sig_m as *const c_char as *const c_void,
            4 as size_t,
        ) == 0 as c_int
        {
            (*info_ptr).byte_order = 'M' as i32;
            getter.get_ushort = Some(
                get_ushort_m
                    as unsafe extern "C" fn(*const c_uchar) -> c_uint,
            )
                as Option<unsafe extern "C" fn(*const c_uchar) -> c_uint>;
            getter.get_ulong = Some(
                get_ulong_m
                    as unsafe extern "C" fn(*const c_uchar) -> c_ulong,
            )
                as Option<
                    unsafe extern "C" fn(*const c_uchar) -> c_ulong,
                >;
        } else if memcmp(
            &raw mut buf as *mut c_uchar as *const c_void,
            &raw const minitiff_sig_i as *const c_char as *const c_void,
            4 as size_t,
        ) == 0 as c_int
        {
            (*info_ptr).byte_order = 'I' as i32;
            getter.get_ushort = Some(
                get_ushort_i
                    as unsafe extern "C" fn(*const c_uchar) -> c_uint,
            )
                as Option<unsafe extern "C" fn(*const c_uchar) -> c_uint>;
            getter.get_ulong = Some(
                get_ulong_i
                    as unsafe extern "C" fn(*const c_uchar) -> c_ulong,
            )
                as Option<
                    unsafe extern "C" fn(*const c_uchar) -> c_ulong,
                >;
        } else if memcmp(
            &raw mut buf as *mut c_uchar as *const c_void,
            &raw const minitiff_sig_bigm as *const c_char
                as *const c_void,
            4 as size_t,
        ) == 0 as c_int
            || memcmp(
                &raw mut buf as *mut c_uchar as *const c_void,
                &raw const minitiff_sig_bigi as *const c_char
                    as *const c_void,
                4 as size_t,
            ) == 0 as c_int
        {
            minitiff_error(info_ptr, msg_err_bigtiff);
            return;
        } else {
            minitiff_error(info_ptr, msg_err_notiff);
            return;
        }
        bits_per_sample_count = 0 as size_t;
        strip_offsets_tag_type = 0 as c_uint;
        bits_per_sample_tag_type = strip_offsets_tag_type;
        strip_offsets_offset = 0 as c_long;
        bits_per_sample_offset = strip_offsets_offset;
        dir_offset = getter.get_ulong.expect("non-null function pointer")(
            (&raw mut buf as *mut c_uchar).offset(4 as c_int as isize),
        ) as c_long;
        if dir_offset >= 0 as c_long && dir_offset < 8 as c_long {
            current_block = 4776593793993497620;
        } else {
            seek_to_offset(info_ptr, dir_offset, stream);
            if fread(
                &raw mut buf as *mut c_uchar as *mut c_void,
                2 as size_t,
                1 as size_t,
                stream,
            ) != 1 as c_ulong
            {
                current_block = 4547171438666572104;
            } else {
                dir_size = getter.get_ushort.expect("non-null function pointer")(
                    &raw mut buf as *mut c_uchar,
                );
                unknown_metadata_found = 0 as c_int;
                i = 0 as c_uint;
                loop {
                    if !(i < dir_size) {
                        current_block = 479107131381816815;
                        break;
                    }
                    if fread(
                        &raw mut buf as *mut c_uchar as *mut c_void,
                        12 as size_t,
                        1 as size_t,
                        stream,
                    ) != 1 as c_ulong
                    {
                        current_block = 4547171438666572104;
                        break;
                    }
                    tag_id = getter.get_ushort.expect("non-null function pointer")(
                        &raw mut buf as *mut c_uchar,
                    );
                    tag_type = getter.get_ushort.expect("non-null function pointer")(
                        (&raw mut buf as *mut c_uchar)
                            .offset(2 as c_int as isize),
                    );
                    count = cast_ulong_to_uint(
                        info_ptr,
                        getter.get_ulong.expect("non-null function pointer")(
                            (&raw mut buf as *mut c_uchar)
                                .offset(4 as c_int as isize),
                        ),
                    ) as size_t;
                    if count == 0 as size_t {
                        current_block = 3084335786941569002;
                        break;
                    }
                    match tag_id {
                        254 => {
                            if count != 1 as size_t {
                                current_block = 3084335786941569002;
                                break;
                            }
                            ulval = get_ulong_value(
                                &raw mut getter,
                                tag_type as c_int,
                                vbuf,
                            );
                            if ulval != 0 as c_ulong
                                && ulval != 1 as c_ulong
                            {
                                current_block = 3084335786941569002;
                                break;
                            }
                        }
                        256 => {
                            if count != 1 as size_t {
                                current_block = 3084335786941569002;
                                break;
                            }
                            ulval = get_ulong_value(
                                &raw mut getter,
                                tag_type as c_int,
                                vbuf,
                            );
                            (*info_ptr).width = cast_ulong_to_uint(info_ptr, ulval) as size_t;
                        }
                        257 => {
                            if count != 1 as size_t {
                                current_block = 3084335786941569002;
                                break;
                            }
                            ulval = get_ulong_value(
                                &raw mut getter,
                                tag_type as c_int,
                                vbuf,
                            );
                            (*info_ptr).height = cast_ulong_to_uint(info_ptr, ulval) as size_t;
                        }
                        258 => {
                            if count == 1 as size_t {
                                ulval = get_ulong_value(
                                    &raw mut getter,
                                    tag_type as c_int,
                                    vbuf,
                                );
                                (*info_ptr).bits_per_sample = cast_ulong_to_uint(info_ptr, ulval);
                            } else {
                                bits_per_sample_count = count;
                                bits_per_sample_tag_type = tag_type;
                                bits_per_sample_offset =
                                    getter.get_ulong.expect("non-null function pointer")(vbuf)
                                        as c_long;
                            }
                        }
                        259 => {
                            if count != 1 as size_t {
                                current_block = 3084335786941569002;
                                break;
                            }
                            ulval = get_ulong_value(
                                &raw mut getter,
                                tag_type as c_int,
                                vbuf,
                            );
                            (*info_ptr).compression = cast_ulong_to_uint(info_ptr, ulval);
                        }
                        262 => {
                            if count != 1 as size_t {
                                current_block = 3084335786941569002;
                                break;
                            }
                            ulval = get_ulong_value(
                                &raw mut getter,
                                tag_type as c_int,
                                vbuf,
                            );
                            (*info_ptr).photometric = cast_ulong_to_uint(info_ptr, ulval);
                        }
                        273 => {
                            (*info_ptr).strip_offsets_count = count;
                            if count == 1 as size_t {
                                if !(*info_ptr).strip_offsets.is_null() {
                                    current_block = 4776593793993497620;
                                    break;
                                }
                                (*info_ptr).strip_offsets =
                                    alloc_ulong_array(info_ptr, 1 as size_t);
                                *(*info_ptr)
                                    .strip_offsets
                                    .offset(0 as c_int as isize) = get_ulong_value(
                                    &raw mut getter,
                                    tag_type as c_int,
                                    vbuf,
                                );
                            } else {
                                strip_offsets_tag_type = tag_type;
                                strip_offsets_offset =
                                    getter.get_ulong.expect("non-null function pointer")(vbuf)
                                        as c_long;
                            }
                        }
                        274 => {
                            if count != 1 as size_t {
                                current_block = 3084335786941569002;
                                break;
                            }
                            ulval = get_ulong_value(
                                &raw mut getter,
                                tag_type as c_int,
                                vbuf,
                            );
                            (*info_ptr).orientation = cast_ulong_to_uint(info_ptr, ulval);
                        }
                        277 => {
                            if count != 1 as size_t {
                                current_block = 3084335786941569002;
                                break;
                            }
                            ulval = get_ulong_value(
                                &raw mut getter,
                                tag_type as c_int,
                                vbuf,
                            );
                            (*info_ptr).samples_per_pixel = cast_ulong_to_uint(info_ptr, ulval);
                        }
                        278 => {
                            if count != 1 as size_t {
                                current_block = 3084335786941569002;
                                break;
                            }
                            ulval = get_ulong_value(
                                &raw mut getter,
                                tag_type as c_int,
                                vbuf,
                            );
                            (*info_ptr).rows_per_strip =
                                cast_ulong_to_uint(info_ptr, ulval) as size_t;
                        }
                        284 | 317 => {
                            if count != 1 as size_t
                                || get_ulong_value(
                                    &raw mut getter,
                                    tag_type as c_int,
                                    vbuf,
                                ) != 1 as c_ulong
                            {
                                current_block = 3084335786941569002;
                                break;
                            }
                        }
                        700 | 33723 | 34665 | 34675 | 34853 | 40965 | 50341 => {
                            if unknown_metadata_found == 0 {
                                unknown_metadata_found = 1 as c_int;
                                minitiff_warning(info_ptr, msg_warn_metadata);
                            }
                        }
                        279 | _ => {}
                    }
                    i = i.wrapping_add(1);
                }
                match current_block {
                    4547171438666572104 => {}
                    3084335786941569002 => {}
                    4776593793993497620 => {}
                    _ => {
                        if fread(
                            &raw mut buf as *mut c_uchar as *mut c_void,
                            4 as size_t,
                            1 as size_t,
                            stream,
                        ) != 1 as c_ulong
                        {
                            current_block = 4547171438666572104;
                        } else {
                            if getter.get_ulong.expect("non-null function pointer")(
                                &raw mut buf as *mut c_uchar,
                            ) != 0 as c_ulong
                            {
                                minitiff_warning(info_ptr, msg_warn_multiple);
                            }
                            if bits_per_sample_offset != 0 as c_long {
                                count = bits_per_sample_count;
                                if count != (*info_ptr).samples_per_pixel as size_t {
                                    current_block = 4776593793993497620;
                                } else if count > 4 as size_t {
                                    current_block = 3084335786941569002;
                                } else {
                                    seek_to_offset(info_ptr, bits_per_sample_offset, stream);
                                    if read_ulong_values(
                                        &raw mut getter,
                                        bits_per_sample_tag_type as c_int,
                                        &raw mut ulvals as *mut c_ulong,
                                        count,
                                        stream,
                                    ) != count
                                    {
                                        current_block = 4547171438666572104;
                                    } else {
                                        loop {
                                            count = count.wrapping_sub(1);
                                            if !(count > 0 as size_t) {
                                                current_block = 5005389895767293342;
                                                break;
                                            }
                                            if ulvals[0 as c_int as usize]
                                                != ulvals[count as usize]
                                            {
                                                current_block = 3084335786941569002;
                                                break;
                                            }
                                        }
                                        match current_block {
                                            3084335786941569002 => {}
                                            _ => {
                                                (*info_ptr).bits_per_sample = cast_ulong_to_uint(
                                                    info_ptr,
                                                    ulvals[0 as c_int as usize],
                                                );
                                                current_block = 16791665189521845338;
                                            }
                                        }
                                    }
                                }
                            } else {
                                current_block = 16791665189521845338;
                            }
                            match current_block {
                                3084335786941569002 => {}
                                4547171438666572104 => {}
                                4776593793993497620 => {}
                                _ => {
                                    if strip_offsets_offset != 0 as c_long {
                                        count = (*info_ptr).strip_offsets_count;
                                        if count == 0 as size_t || count > (*info_ptr).height {
                                            current_block = 4776593793993497620;
                                        } else if !(*info_ptr).strip_offsets.is_null() {
                                            current_block = 4776593793993497620;
                                        } else {
                                            (*info_ptr).strip_offsets =
                                                alloc_ulong_array(info_ptr, count);
                                            seek_to_offset(info_ptr, strip_offsets_offset, stream);
                                            if read_ulong_values(
                                                &raw mut getter,
                                                strip_offsets_tag_type as c_int,
                                                (*info_ptr).strip_offsets
                                                    as *mut c_ulong,
                                                count,
                                                stream,
                                            ) != count
                                            {
                                                current_block = 4547171438666572104;
                                            } else {
                                                current_block = 17395932908762866334;
                                            }
                                        }
                                    } else {
                                        current_block = 17395932908762866334;
                                    }
                                    match current_block {
                                        4547171438666572104 => {}
                                        4776593793993497620 => {}
                                        _ => return,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        4547171438666572104 => {
            minitiff_error(info_ptr, msg_err_read);
            current_block = 4776593793993497620;
        }
        _ => {}
    }
    match current_block {
        4776593793993497620 => {
            minitiff_error(info_ptr, msg_err_invalid);
        }
        _ => {}
    }
    minitiff_error(info_ptr, msg_err_unsupported);
}
#[no_mangle]
pub unsafe extern "C" fn minitiff_read_row(
    mut info_ptr: *mut minitiff_info,
    mut row_ptr: *mut c_uchar,
    mut row_index: size_t,
    mut stream: *mut FILE,
) {
    let mut current_block: u64;
    let mut row_size: size_t = 0;
    let mut strip_index: size_t = 0;
    let mut bytes_per_sample: c_uint = 0;
    let mut sample_max: c_uint = 0;
    let mut offset: c_long = 0;
    let mut i: size_t = 0;
    bytes_per_sample = (*info_ptr)
        .bits_per_sample
        .wrapping_add(7 as c_uint)
        .wrapping_div(8 as c_uint);
    row_size = (*info_ptr)
        .width
        .wrapping_mul((*info_ptr).samples_per_pixel as size_t)
        .wrapping_mul(bytes_per_sample as size_t);
    strip_index = row_index.wrapping_div((*info_ptr).rows_per_strip);
    if strip_index >= (*info_ptr).strip_offsets_count {
        current_block = 7518011848042542155;
    } else if (*(*info_ptr).strip_offsets.offset(strip_index as isize) as c_long)
        < 0 as c_long
    {
        current_block = 14872618380919038190;
    } else {
        offset = (*(*info_ptr).strip_offsets.offset(strip_index as isize) as size_t)
            .wrapping_add(row_size.wrapping_mul(row_index.wrapping_rem((*info_ptr).rows_per_strip)))
            as c_long;
        seek_to_offset(info_ptr, offset, stream);
        if fread(
            row_ptr as *mut c_void,
            row_size,
            1 as size_t,
            stream,
        ) != 1 as c_ulong
        {
            minitiff_error(info_ptr, msg_err_read);
            current_block = 7518011848042542155;
        } else {
            if (*info_ptr).photometric == 0 as c_uint {
                if bytes_per_sample > 1 as c_uint {
                    current_block = 18059164448232926512;
                } else {
                    sample_max = (((1 as c_int) << (*info_ptr).bits_per_sample)
                        - 1 as c_int)
                        as c_uint;
                    i = 0 as size_t;
                    while i < row_size {
                        *row_ptr.offset(i as isize) = sample_max
                            .wrapping_sub(*row_ptr.offset(i as isize) as c_uint)
                            as c_uchar;
                        i = i.wrapping_add(1);
                    }
                    current_block = 13586036798005543211;
                }
            } else {
                current_block = 13586036798005543211;
            }
            match current_block {
                18059164448232926512 => {}
                _ => return,
            }
        }
    }
    match current_block {
        7518011848042542155 => {
            minitiff_error(info_ptr, msg_err_invalid);
            current_block = 14872618380919038190;
        }
        _ => {}
    }
    match current_block {
        14872618380919038190 => {
            minitiff_error(info_ptr, msg_err_range);
        }
        _ => {}
    }
    minitiff_error(info_ptr, msg_err_unsupported);
}
