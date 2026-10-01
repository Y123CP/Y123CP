extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn heman_image_texel(
        _: *mut heman_image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_float;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heman_image_s {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub nbands: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_float,
}
pub type heman_image = heman_image_s;
pub type heman_byte = ::core::ffi::c_uchar;
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
pub type size_t = usize;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn heman_export_ply(
    mut img: *mut heman_image,
    mut filename: *const ::core::ffi::c_char,
) {
    let mut fout: *mut FILE = fopen(filename, b"wb\0" as *const u8 as *const ::core::ffi::c_char);
    let mut ncols: ::core::ffi::c_int = (*img).width - 1 as ::core::ffi::c_int;
    let mut nrows: ::core::ffi::c_int = (*img).height - 1 as ::core::ffi::c_int;
    let mut ncells: ::core::ffi::c_int = ncols * nrows;
    let mut nverts: ::core::ffi::c_int = (*img).width * (*img).height;
    fprintf(
        fout,
        b"ply\nformat binary_little_endian 1.0\ncomment heman\nelement vertex %d\nproperty float32 x\nproperty float32 y\nproperty float32 z\nelement face %d\nproperty list int32 int32 vertex_indices\nend_header\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        nverts,
        ncells,
    );
    let mut invw: ::core::ffi::c_float = 2.0f32 / (*img).width as ::core::ffi::c_float;
    let mut invh: ::core::ffi::c_float = 2.0f32 / (*img).height as ::core::ffi::c_float;
    let mut vert: [::core::ffi::c_float; 3] = [0.; 3];
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j < (*img).height {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*img).width {
            vert[0 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int)
                as ::core::ffi::c_float
                + i as ::core::ffi::c_float * invw;
            vert[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int)
                as ::core::ffi::c_float
                + j as ::core::ffi::c_float * invh;
            vert[2 as ::core::ffi::c_int as usize] = *heman_image_texel(img, i, j);
            fwrite(
                &raw mut vert as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_float; 3]>() as size_t,
                1 as size_t,
                fout,
            );
            i += 1;
        }
        j += 1;
    }
    let mut face: [::core::ffi::c_int; 5] = [0; 5];
    face[0 as ::core::ffi::c_int as usize] = 4 as ::core::ffi::c_int;
    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j_0 < nrows {
        let mut p: ::core::ffi::c_int = j_0 * (*img).width;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < ncols {
            face[1 as ::core::ffi::c_int as usize] = p;
            face[2 as ::core::ffi::c_int as usize] = p + 1 as ::core::ffi::c_int;
            face[3 as ::core::ffi::c_int as usize] = p + (*img).width + 1 as ::core::ffi::c_int;
            face[4 as ::core::ffi::c_int as usize] = p + (*img).width;
            fwrite(
                &raw mut face as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_int; 5]>() as size_t,
                1 as size_t,
                fout,
            );
            i_0 += 1;
            p += 1;
        }
        j_0 += 1;
    }
    fclose(fout);
}
#[no_mangle]
pub unsafe extern "C" fn heman_export_with_colors_ply(
    mut hmap: *mut heman_image,
    mut colors: *mut heman_image,
    mut filename: *const ::core::ffi::c_char,
) {
    let mut width: ::core::ffi::c_int = (*hmap).width;
    let mut height: ::core::ffi::c_int = (*hmap).height;
    let mut fout: *mut FILE = fopen(filename, b"wb\0" as *const u8 as *const ::core::ffi::c_char);
    let mut ncols: ::core::ffi::c_int = (*hmap).width - 1 as ::core::ffi::c_int;
    let mut nrows: ::core::ffi::c_int = (*hmap).height - 1 as ::core::ffi::c_int;
    let mut ncells: ::core::ffi::c_int = ncols * nrows;
    let mut nverts: ::core::ffi::c_int = (*hmap).width * (*hmap).height;
    let mut colordata: *mut ::core::ffi::c_uchar =
        malloc((width * height * 3 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_uchar;
    heman_export_u8(colors, 0.0f32, 1.0f32, colordata as *mut heman_byte);
    fprintf(
        fout,
        b"ply\nformat binary_little_endian 1.0\ncomment heman\nelement vertex %d\nproperty float32 x\nproperty float32 y\nproperty float32 z\nproperty uchar red\nproperty uchar green\nproperty uchar blue\nproperty uchar alpha\nelement face %d\nproperty list int32 int32 vertex_indices\nend_header\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        nverts,
        ncells,
    );
    let mut invw: ::core::ffi::c_float = 2.0f32 / width as ::core::ffi::c_float;
    let mut invh: ::core::ffi::c_float = 2.0f32 / height as ::core::ffi::c_float;
    let mut pcolor: *mut heman_byte = colordata as *mut heman_byte;
    let mut vert: [::core::ffi::c_float; 3] = [0.; 3];
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j < height {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < width {
            vert[0 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int)
                as ::core::ffi::c_float
                + i as ::core::ffi::c_float * invw;
            vert[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int)
                as ::core::ffi::c_float
                + j as ::core::ffi::c_float * invh;
            vert[2 as ::core::ffi::c_int as usize] = *heman_image_texel(hmap, i, j);
            fwrite(
                &raw mut vert as *mut ::core::ffi::c_float as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_float; 3]>() as size_t,
                1 as size_t,
                fout,
            );
            fwrite(
                pcolor as *const ::core::ffi::c_void,
                3 as size_t,
                1 as size_t,
                fout,
            );
            pcolor = pcolor.offset(3 as ::core::ffi::c_int as isize);
            fputc(255 as ::core::ffi::c_int, fout);
            i += 1;
        }
        j += 1;
    }
    let mut face: [::core::ffi::c_int; 5] = [0; 5];
    face[0 as ::core::ffi::c_int as usize] = 4 as ::core::ffi::c_int;
    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j_0 < nrows {
        let mut p: ::core::ffi::c_int = j_0 * width;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < ncols {
            face[1 as ::core::ffi::c_int as usize] = p;
            face[2 as ::core::ffi::c_int as usize] = p + 1 as ::core::ffi::c_int;
            face[3 as ::core::ffi::c_int as usize] = p + (*hmap).width + 1 as ::core::ffi::c_int;
            face[4 as ::core::ffi::c_int as usize] = p + (*hmap).width;
            fwrite(
                &raw mut face as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_int; 5]>() as size_t,
                1 as size_t,
                fout,
            );
            i_0 += 1;
            p += 1;
        }
        j_0 += 1;
    }
    fclose(fout);
    free(colordata as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_export_u8(
    mut source: *mut heman_image,
    mut minv: ::core::ffi::c_float,
    mut maxv: ::core::ffi::c_float,
    mut outp: *mut heman_byte,
) {
    let mut inp: *const ::core::ffi::c_float = (*source).data;
    let mut scale: ::core::ffi::c_float = 1.0f32 / (maxv - minv);
    let mut size: ::core::ffi::c_int = (*source).height * (*source).width * (*source).nbands;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size {
        let fresh0 = inp;
        inp = inp.offset(1);
        let mut v: ::core::ffi::c_float =
            255 as ::core::ffi::c_int as ::core::ffi::c_float * (*fresh0 - minv) * scale;
        let fresh1 = outp;
        outp = outp.offset(1);
        *fresh1 = (if 0 as ::core::ffi::c_int as ::core::ffi::c_float
            > (if 255 as ::core::ffi::c_int as ::core::ffi::c_float > v {
                v
            } else {
                255 as ::core::ffi::c_int as ::core::ffi::c_float
            }) {
            0 as ::core::ffi::c_int as ::core::ffi::c_float
        } else if 255 as ::core::ffi::c_int as ::core::ffi::c_float > v {
            v
        } else {
            255 as ::core::ffi::c_int as ::core::ffi::c_float
        }) as heman_byte;
        i += 1;
    }
}
