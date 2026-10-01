use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    fn read(__fd: c_int, __buf: *mut c_void, __nbytes: size_t)
        -> ssize_t;
    fn pselect(
        __nfds: c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> c_int;
    fn tcgetattr(__fd: c_int, __termios_p: *mut termios) -> c_int;
    fn tcsetattr(
        __fd: c_int,
        __optional_actions: c_int,
        __termios_p: *const termios,
    ) -> c_int;
    fn ioctl(__fd: c_int, __request: c_ulong, ...) -> c_int;
    fn signal(__sig: c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: c_int) -> c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const c_char,
        __arg: ::core::ffi::VaList,
    ) -> c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}

pub type __time_t = c_long;
pub type __ssize_t = c_long;
pub type __syscall_slong_t = c_long;
pub type ssize_t = __ssize_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type __fd_mask = c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type va_list = __builtin_va_list;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(c_int) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_t {
    pub fdin: c_int,
    pub fout: *mut FILE,
    pub original_termios: termios,
    pub fgcolor: c_int,
    pub maxwidth: size_t,
    pub maxheight: size_t,
}

pub const O_RDONLY: c_int = 0 as c_int;

pub const ICRNL: c_int = 0o400 as c_int;
pub const __NFDBITS: c_int =
    8 as c_int * ::core::mem::size_of::<__fd_mask>() as c_int;
pub const ISIG: c_int = 0o1 as c_int;
pub const ICANON: c_int = 0o2 as c_int;
pub const ECHO: c_int = 0o10 as c_int;
pub const TCSANOW: c_int = 0 as c_int;
pub const TIOCGWINSZ: c_int = 0x5413 as c_int;
pub const SIGWINCH: c_int = 28 as c_int;
pub const EINTR: c_int = 4 as c_int;
pub const _IOFBF: c_int = 0 as c_int;
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
unsafe extern "C" fn handle_sigwinch(mut sig: c_int) {}
#[no_mangle]
pub unsafe extern "C" fn tty_init(
    mut tty: *mut tty_t,
    mut tty_filename: *const c_char,
) {
    (*tty).fdin = open(tty_filename, O_RDONLY);
    if (*tty).fdin < 0 as c_int {
        perror(b"Failed to open tty\0" as *const u8 as *const c_char);
        exit(EXIT_FAILURE);
    }
    (*tty).fout = fopen(
        tty_filename,
        b"w\0" as *const u8 as *const c_char,
    ) as *mut FILE;
    if (*tty).fout.is_null() {
        perror(b"Failed to open tty\0" as *const u8 as *const c_char);
        exit(EXIT_FAILURE);
    }
    if setvbuf(
        (*tty).fout,
        ::core::ptr::null_mut::<c_char>(),
        _IOFBF,
        16384 as size_t,
    ) != 0
    {
        perror(b"setvbuf\0" as *const u8 as *const c_char);
        exit(EXIT_FAILURE);
    }
    if tcgetattr((*tty).fdin, &raw mut (*tty).original_termios) != 0 {
        perror(b"tcgetattr\0" as *const u8 as *const c_char);
        exit(EXIT_FAILURE);
    }
    let mut new_termios: termios = (*tty).original_termios;
    new_termios.c_iflag &= !(0o400 as c_int) as tcflag_t;
    new_termios.c_lflag &= !(ICANON | ECHO | ISIG) as tcflag_t;
    if tcsetattr((*tty).fdin, TCSANOW, &raw mut new_termios) != 0 {
        perror(b"tcsetattr\0" as *const u8 as *const c_char);
    }
    tty_getwinsz(tty);
    tty_setnormal(tty);
    signal(
        SIGWINCH,
        Some(handle_sigwinch as unsafe extern "C" fn(c_int) -> ()),
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
        TIOCGWINSZ as c_ulong,
        &raw mut ws,
    ) == -(1 as c_int)
    {
        (*tty).maxwidth = 80 as size_t;
        (*tty).maxheight = 25 as size_t;
    } else {
        (*tty).maxwidth = ws.ws_col as size_t;
        (*tty).maxheight = ws.ws_row as size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_getchar(mut tty: *mut tty_t) -> c_char {
    let mut ch: c_char = 0;
    let mut size: c_int = read(
        (*tty).fdin,
        &raw mut ch as *mut c_void,
        1 as size_t,
    ) as c_int;
    if size < 0 as c_int {
        perror(b"error reading from tty\0" as *const u8 as *const c_char);
        exit(EXIT_FAILURE);
    } else if size == 0 as c_int {
        exit(EXIT_FAILURE);
    } else {
        return ch;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_input_ready(
    mut tty: *mut tty_t,
    mut timeout: c_long,
    mut return_on_signal: c_int,
) -> c_int {
    let mut readfs: fd_set = fd_set { fds_bits: [0; 16] };
    let mut __d0: c_int = 0;
    let mut __d1: c_int = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = (::core::mem::size_of::<fd_set>() as usize)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize);
    let fresh3 = &mut __d1;
    let fresh4;
    let fresh5 = (&raw mut readfs.fds_bits as *mut __fd_mask)
        .offset(0 as c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq\n", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
        fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, inlateout("ax") 0 as c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    readfs.fds_bits[((*tty).fdin / __NFDBITS) as usize] |=
        ((1 as c_ulong) << (*tty).fdin % __NFDBITS) as __fd_mask;
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
    let mut err: c_int = pselect(
        (*tty).fdin + 1 as c_int,
        &raw mut readfs,
        ::core::ptr::null_mut::<fd_set>(),
        ::core::ptr::null_mut::<fd_set>(),
        if timeout < 0 as c_long {
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
    if err < 0 as c_int {
        if *__errno_location() == EINTR {
            return 0 as c_int;
        } else {
            perror(b"select\0" as *const u8 as *const c_char);
            exit(EXIT_FAILURE);
        }
    } else {
        return (readfs.fds_bits[((*tty).fdin / __NFDBITS) as usize]
            & ((1 as c_ulong) << (*tty).fdin % __NFDBITS) as __fd_mask
            != 0 as __fd_mask) as c_int;
    };
}
unsafe extern "C" fn tty_sgr(mut tty: *mut tty_t, mut code: c_int) {
    tty_printf(
        tty,
        b"%c%c%im\0" as *const u8 as *const c_char,
        0x1b as c_int,
        '[' as i32,
        code,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_setfg(mut tty: *mut tty_t, mut fg: c_int) {
    if (*tty).fgcolor != fg {
        tty_sgr(tty, 30 as c_int + fg);
        (*tty).fgcolor = fg;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tty_setinvert(mut tty: *mut tty_t) {
    tty_sgr(tty, 7 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tty_setunderline(mut tty: *mut tty_t) {
    tty_sgr(tty, 4 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tty_setnormal(mut tty: *mut tty_t) {
    tty_sgr(tty, 0 as c_int);
    (*tty).fgcolor = 9 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tty_setnowrap(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%c?7l\0" as *const u8 as *const c_char,
        0x1b as c_int,
        '[' as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_setwrap(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%c?7h\0" as *const u8 as *const c_char,
        0x1b as c_int,
        '[' as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_newline(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%cK\n\0" as *const u8 as *const c_char,
        0x1b as c_int,
        '[' as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_clearline(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%cK\0" as *const u8 as *const c_char,
        0x1b as c_int,
        '[' as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_setcol(mut tty: *mut tty_t, mut col: c_int) {
    tty_printf(
        tty,
        b"%c%c%iG\0" as *const u8 as *const c_char,
        0x1b as c_int,
        '[' as i32,
        col + 1 as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_moveup(mut tty: *mut tty_t, mut i: c_int) {
    tty_printf(
        tty,
        b"%c%c%iA\0" as *const u8 as *const c_char,
        0x1b as c_int,
        '[' as i32,
        i,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_printf(
    mut tty: *mut tty_t,
    mut fmt: *const c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf((*tty).fout, fmt, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn tty_putc(mut tty: *mut tty_t, mut c: c_char) {
    fputc(c as c_int, (*tty).fout);
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
