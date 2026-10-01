use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn pselect(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::core::ffi::c_int;
    fn tcgetattr(__fd: ::core::ffi::c_int, __termios_p: *mut termios) -> ::core::ffi::c_int;
    fn tcsetattr(
        __fd: ::core::ffi::c_int,
        __optional_actions: ::core::ffi::c_int,
        __termios_p: *const termios,
    ) -> ::core::ffi::c_int;
    fn ioctl(__fd: ::core::ffi::c_int, __request: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_int;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::core::ffi::c_char,
        __modes: ::core::ffi::c_int,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type __fd_mask = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type va_list = __builtin_va_list;
pub type cc_t = ::core::ffi::c_uchar;
pub type speed_t = ::core::ffi::c_uint;
pub type tcflag_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: ::core::ffi::c_ushort,
    pub ws_col: ::core::ffi::c_ushort,
    pub ws_xpixel: ::core::ffi::c_ushort,
    pub ws_ypixel: ::core::ffi::c_ushort,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
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
pub struct tty_t {
    pub fdin: ::core::ffi::c_int,
    pub fout: *mut FILE,
    pub original_termios: termios,
    pub fgcolor: ::core::ffi::c_int,
    pub maxwidth: size_t,
    pub maxheight: size_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICRNL: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const ISIG: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const ICANON: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const ECHO: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const TCSANOW: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TIOCGWINSZ: ::core::ffi::c_int = 0x5413 as ::core::ffi::c_int;
pub const SIGWINCH: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const _IOFBF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn tty_reset(mut tty: *mut tty_t) {
    tcsetattr((*tty).fdin, TCSANOW, &raw mut (*tty).original_termios);
}
#[no_mangle]
pub unsafe extern "C" fn tty_close(mut tty: *mut tty_t) {
    tty_reset(tty);
    fclose((*tty).fout);
    close((*tty).fdin);
}
unsafe extern "C" fn handle_sigwinch(mut sig: ::core::ffi::c_int) {}
#[no_mangle]
pub unsafe extern "C" fn tty_init(
    mut tty: *mut tty_t,
    mut tty_filename: *const ::core::ffi::c_char,
) {
    (*tty).fdin = open(tty_filename, O_RDONLY);
    if (*tty).fdin < 0 as ::core::ffi::c_int {
        perror(b"Failed to open tty\0" as *const u8 as *const ::core::ffi::c_char);
        exit(EXIT_FAILURE);
    }
    (*tty).fout = fopen(
        tty_filename,
        b"w\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if (*tty).fout.is_null() {
        perror(b"Failed to open tty\0" as *const u8 as *const ::core::ffi::c_char);
        exit(EXIT_FAILURE);
    }
    if setvbuf(
        (*tty).fout,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        _IOFBF,
        16384 as size_t,
    ) != 0
    {
        perror(b"setvbuf\0" as *const u8 as *const ::core::ffi::c_char);
        exit(EXIT_FAILURE);
    }
    if tcgetattr((*tty).fdin, &raw mut (*tty).original_termios) != 0 {
        perror(b"tcgetattr\0" as *const u8 as *const ::core::ffi::c_char);
        exit(EXIT_FAILURE);
    }
    let mut new_termios: termios = (*tty).original_termios;
    new_termios.c_iflag &= !(0o400 as ::core::ffi::c_int) as tcflag_t;
    new_termios.c_lflag &= !(ICANON | ECHO | ISIG) as tcflag_t;
    if tcsetattr((*tty).fdin, TCSANOW, &raw mut new_termios) != 0 {
        perror(b"tcsetattr\0" as *const u8 as *const ::core::ffi::c_char);
    }
    tty_getwinsz(tty);
    tty_setnormal(tty);
    signal(
        SIGWINCH,
        Some(handle_sigwinch as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_getwinsz(mut tty: *mut tty_t) {
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if ioctl(
        fileno((*tty).fout),
        TIOCGWINSZ as ::core::ffi::c_ulong,
        &raw mut ws,
    ) == -(1 as ::core::ffi::c_int)
    {
        (*tty).maxwidth = 80 as size_t;
        (*tty).maxheight = 25 as size_t;
    } else {
        (*tty).maxwidth = ws.ws_col as size_t;
        (*tty).maxheight = ws.ws_row as size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_getchar(mut tty: *mut tty_t) -> ::core::ffi::c_char {
    let mut ch: ::core::ffi::c_char = 0;
    let mut size: ::core::ffi::c_int = read(
        (*tty).fdin,
        &raw mut ch as *mut ::core::ffi::c_void,
        1 as size_t,
    ) as ::core::ffi::c_int;
    if size < 0 as ::core::ffi::c_int {
        perror(b"error reading from tty\0" as *const u8 as *const ::core::ffi::c_char);
        exit(EXIT_FAILURE);
    } else if size == 0 as ::core::ffi::c_int {
        exit(EXIT_FAILURE);
    } else {
        return ch;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_input_ready(
    mut tty: *mut tty_t,
    mut timeout: ::core::ffi::c_long,
    mut return_on_signal: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut readfs: fd_set = fd_set { fds_bits: [0; 16] };
    let mut __d0: ::core::ffi::c_int = 0;
    let mut __d1: ::core::ffi::c_int = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = (::core::mem::size_of::<fd_set>() as usize)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize);
    let fresh3 = &mut __d1;
    let fresh4;
    let fresh5 = (&raw mut readfs.fds_bits as *mut __fd_mask)
        .offset(0 as ::core::ffi::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq\n", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
        fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, inlateout("ax") 0 as ::core::ffi::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    readfs.fds_bits[((*tty).fdin / __NFDBITS) as usize] |=
        ((1 as ::core::ffi::c_ulong) << (*tty).fdin % __NFDBITS) as __fd_mask;
    let mut ts: timespec = timespec {
        tv_sec: timeout as __time_t / 1000 as __time_t,
        tv_nsec: timeout as __syscall_slong_t % 1000 as __syscall_slong_t
            * 1000000 as __syscall_slong_t,
    };
    let mut mask: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&raw mut mask);
    if return_on_signal == 0 {
        sigaddset(&raw mut mask, SIGWINCH);
    }
    let mut err: ::core::ffi::c_int = pselect(
        (*tty).fdin + 1 as ::core::ffi::c_int,
        &raw mut readfs,
        ::core::ptr::null_mut::<fd_set>(),
        ::core::ptr::null_mut::<fd_set>(),
        if timeout < 0 as ::core::ffi::c_long {
            ::core::ptr::null_mut::<timespec>()
        } else {
            &raw mut ts
        },
        if return_on_signal != 0 {
            ::core::ptr::null_mut::<sigset_t>()
        } else {
            &raw mut mask
        },
    );
    if err < 0 as ::core::ffi::c_int {
        if *__errno_location() == EINTR {
            return 0 as ::core::ffi::c_int;
        } else {
            perror(b"select\0" as *const u8 as *const ::core::ffi::c_char);
            exit(EXIT_FAILURE);
        }
    } else {
        return (readfs.fds_bits[((*tty).fdin / __NFDBITS) as usize]
            & ((1 as ::core::ffi::c_ulong) << (*tty).fdin % __NFDBITS) as __fd_mask
            != 0 as __fd_mask) as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn tty_sgr(mut tty: *mut tty_t, mut code: ::core::ffi::c_int) {
    tty_printf(
        tty,
        b"%c%c%im\0" as *const u8 as *const ::core::ffi::c_char,
        0x1b as ::core::ffi::c_int,
        '[' as i32,
        code,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_setfg(mut tty: *mut tty_t, mut fg: ::core::ffi::c_int) {
    if (*tty).fgcolor != fg {
        tty_sgr(tty, 30 as ::core::ffi::c_int + fg);
        (*tty).fgcolor = fg;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tty_setinvert(mut tty: *mut tty_t) {
    tty_sgr(tty, 7 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tty_setunderline(mut tty: *mut tty_t) {
    tty_sgr(tty, 4 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tty_setnormal(mut tty: *mut tty_t) {
    tty_sgr(tty, 0 as ::core::ffi::c_int);
    (*tty).fgcolor = 9 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tty_setnowrap(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%c?7l\0" as *const u8 as *const ::core::ffi::c_char,
        0x1b as ::core::ffi::c_int,
        '[' as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_setwrap(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%c?7h\0" as *const u8 as *const ::core::ffi::c_char,
        0x1b as ::core::ffi::c_int,
        '[' as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_newline(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%cK\n\0" as *const u8 as *const ::core::ffi::c_char,
        0x1b as ::core::ffi::c_int,
        '[' as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_clearline(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%cK\0" as *const u8 as *const ::core::ffi::c_char,
        0x1b as ::core::ffi::c_int,
        '[' as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_setcol(mut tty: *mut tty_t, mut col: ::core::ffi::c_int) {
    tty_printf(
        tty,
        b"%c%c%iG\0" as *const u8 as *const ::core::ffi::c_char,
        0x1b as ::core::ffi::c_int,
        '[' as i32,
        col + 1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_moveup(mut tty: *mut tty_t, mut i: ::core::ffi::c_int) {
    tty_printf(
        tty,
        b"%c%c%iA\0" as *const u8 as *const ::core::ffi::c_char,
        0x1b as ::core::ffi::c_int,
        '[' as i32,
        i,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_printf(
    mut tty: *mut tty_t,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf((*tty).fout, fmt, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn tty_putc(mut tty: *mut tty_t, mut c: ::core::ffi::c_char) {
    fputc(c as ::core::ffi::c_int, (*tty).fout);
}
#[no_mangle]
pub unsafe extern "C" fn tty_flush(mut tty: *mut tty_t) {
    fflush((*tty).fout);
}
#[no_mangle]
pub unsafe extern "C" fn tty_getwidth(mut tty: *mut tty_t) -> size_t {
    return (*tty).maxwidth;
}
#[no_mangle]
pub unsafe extern "C" fn tty_getheight(mut tty: *mut tty_t) -> size_t {
    return (*tty).maxheight;
}
