extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn match_positions(
        needle: *const ::core::ffi::c_char,
        haystack: *const ::core::ffi::c_char,
        positions: *mut size_t,
    ) -> score_t;
    fn choices_available(c: *mut choices_t) -> size_t;
    fn choices_search(c: *mut choices_t, search: *const ::core::ffi::c_char);
    fn choices_get(c: *mut choices_t, n: size_t) -> *const ::core::ffi::c_char;
    fn choices_prev(c: *mut choices_t);
    fn choices_next(c: *mut choices_t);
    fn tty_close(tty: *mut tty_t);
    fn tty_getchar(tty: *mut tty_t) -> ::core::ffi::c_char;
    fn tty_input_ready(
        tty: *mut tty_t,
        timeout: ::core::ffi::c_long,
        return_on_signal: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tty_setfg(tty: *mut tty_t, fg: ::core::ffi::c_int);
    fn tty_setinvert(tty: *mut tty_t);
    fn tty_setnormal(tty: *mut tty_t);
    fn tty_setnowrap(tty: *mut tty_t);
    fn tty_setwrap(tty: *mut tty_t);
    fn tty_newline(tty: *mut tty_t);
    fn tty_clearline(tty: *mut tty_t);
    fn tty_moveup(tty: *mut tty_t, i: ::core::ffi::c_int);
    fn tty_setcol(tty: *mut tty_t, col: ::core::ffi::c_int);
    fn tty_printf(tty: *mut tty_t, fmt: *const ::core::ffi::c_char, ...);
    fn tty_putc(tty: *mut tty_t, c: ::core::ffi::c_char);
    fn tty_flush(tty: *mut tty_t);
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = usize;
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
pub type score_t = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_t {
    pub benchmark: ::core::ffi::c_int,
    pub filter: *const ::core::ffi::c_char,
    pub init_search: *const ::core::ffi::c_char,
    pub tty_filename: *const ::core::ffi::c_char,
    pub show_scores: ::core::ffi::c_int,
    pub num_lines: ::core::ffi::c_uint,
    pub scrolloff: ::core::ffi::c_uint,
    pub prompt: *const ::core::ffi::c_char,
    pub workers: ::core::ffi::c_uint,
    pub input_delimiter: ::core::ffi::c_char,
    pub show_info: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scored_result {
    pub score: score_t,
    pub str_0: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct choices_t {
    pub buffer: *mut ::core::ffi::c_char,
    pub buffer_size: size_t,
    pub capacity: size_t,
    pub size: size_t,
    pub strings: *mut *const ::core::ffi::c_char,
    pub results: *mut scored_result,
    pub available: size_t,
    pub selection: size_t,
    pub worker_count: ::core::ffi::c_uint,
}
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
pub struct tty_t {
    pub fdin: ::core::ffi::c_int,
    pub fout: *mut FILE,
    pub original_termios: termios,
    pub fgcolor: ::core::ffi::c_int,
    pub maxwidth: size_t,
    pub maxheight: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_interface_t {
    pub tty: *mut tty_t,
    pub choices: *mut choices_t,
    pub options: *mut options_t,
    pub search: [::core::ffi::c_char; 4097],
    pub last_search: [::core::ffi::c_char; 4097],
    pub cursor: size_t,
    pub ambiguous_key_pending: ::core::ffi::c_int,
    pub input: [::core::ffi::c_char; 32],
    pub exit: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keybinding_t {
    pub key: *const ::core::ffi::c_char,
    pub action: Option<unsafe extern "C" fn(*mut tty_interface_t) -> ()>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MATCH_MAX_LEN: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TTY_COLOR_YELLOW: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TTY_COLOR_NORMAL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SEARCH_SIZE_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C" fn isprint_unicode(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (*(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISprint as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
        || c as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn is_boundary(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (!(c as ::core::ffi::c_int) & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
        || c as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn clear(mut state: *mut tty_interface_t) {
    let mut tty: *mut tty_t = (*state).tty;
    tty_setcol(tty, 0 as ::core::ffi::c_int);
    let mut line: size_t = 0 as size_t;
    loop {
        let fresh0 = line;
        line = line.wrapping_add(1);
        if !(fresh0
            < (*(*state).options).num_lines.wrapping_add(
                (if (*(*state).options).show_info != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            ) as size_t)
        {
            break;
        }
        tty_newline(tty);
    }
    tty_clearline(tty);
    if (*(*state).options).num_lines > 0 as ::core::ffi::c_uint {
        tty_moveup(tty, line.wrapping_sub(1 as size_t) as ::core::ffi::c_int);
    }
    tty_flush(tty);
}
unsafe extern "C" fn draw_match(
    mut state: *mut tty_interface_t,
    mut choice: *const ::core::ffi::c_char,
    mut selected: ::core::ffi::c_int,
) {
    let mut tty: *mut tty_t = (*state).tty;
    let mut options: *mut options_t = (*state).options;
    let mut search: *mut ::core::ffi::c_char =
        &raw mut (*state).last_search as *mut ::core::ffi::c_char;
    let mut n: ::core::ffi::c_int = strlen(search) as ::core::ffi::c_int;
    let mut positions: [size_t; 1024] = [0; 1024];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < n + 1 as ::core::ffi::c_int && i < MATCH_MAX_LEN {
        positions[i as usize] = -(1 as ::core::ffi::c_int) as size_t;
        i += 1;
    }
    let mut score: score_t = match_positions(
        search,
        choice,
        (&raw mut positions as *mut size_t).offset(0 as ::core::ffi::c_int as isize) as *mut size_t,
    );
    if (*options).show_scores != 0 {
        if score == -::core::f32::INFINITY as score_t {
            tty_printf(
                tty,
                b"(     ) \0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            tty_printf(
                tty,
                b"(%5.2f) \0" as *const u8 as *const ::core::ffi::c_char,
                score,
            );
        }
    }
    if selected != 0 {
        tty_setinvert(tty);
    }
    tty_setnowrap(tty);
    let mut i_0: size_t = 0 as size_t;
    let mut p: size_t = 0 as size_t;
    while *choice.offset(i_0 as isize) as ::core::ffi::c_int != '\0' as i32 {
        if positions[p as usize] == i_0 {
            tty_setfg(tty, TTY_COLOR_HIGHLIGHT);
            p = p.wrapping_add(1);
        } else {
            tty_setfg(tty, TTY_COLOR_NORMAL);
        }
        if *choice.offset(i_0 as isize) as ::core::ffi::c_int == '\n' as i32 {
            tty_putc(tty, ' ' as i32 as ::core::ffi::c_char);
        } else {
            tty_printf(
                tty,
                b"%c\0" as *const u8 as *const ::core::ffi::c_char,
                *choice.offset(i_0 as isize) as ::core::ffi::c_int,
            );
        }
        i_0 = i_0.wrapping_add(1);
    }
    tty_setwrap(tty);
    tty_setnormal(tty);
}
unsafe extern "C" fn draw(mut state: *mut tty_interface_t) {
    let mut tty: *mut tty_t = (*state).tty;
    let mut choices: *mut choices_t = (*state).choices;
    let mut options: *mut options_t = (*state).options;
    let mut num_lines: ::core::ffi::c_uint = (*options).num_lines;
    let mut start: size_t = 0 as size_t;
    let mut current_selection: size_t = (*choices).selection;
    if current_selection.wrapping_add((*options).scrolloff as size_t) >= num_lines as size_t {
        start = current_selection
            .wrapping_add((*options).scrolloff as size_t)
            .wrapping_sub(num_lines as size_t)
            .wrapping_add(1 as size_t);
        let mut available: size_t = choices_available(choices);
        if start.wrapping_add(num_lines as size_t) >= available && available > 0 as size_t {
            start = available.wrapping_sub(num_lines as size_t);
        }
    }
    tty_setcol(tty, 0 as ::core::ffi::c_int);
    tty_printf(
        tty,
        b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        (*options).prompt,
        &raw mut (*state).search as *mut ::core::ffi::c_char,
    );
    tty_clearline(tty);
    if (*options).show_info != 0 {
        tty_printf(
            tty,
            b"\n[%lu/%lu]\0" as *const u8 as *const ::core::ffi::c_char,
            (*choices).available,
            (*choices).size,
        );
        tty_clearline(tty);
    }
    let mut i: size_t = start;
    while i < start.wrapping_add(num_lines as size_t) {
        tty_printf(tty, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        tty_clearline(tty);
        let mut choice: *const ::core::ffi::c_char = choices_get(choices, i);
        if !choice.is_null() {
            draw_match(
                state,
                choice,
                (i == (*choices).selection) as ::core::ffi::c_int,
            );
        }
        i = i.wrapping_add(1);
    }
    if num_lines.wrapping_add((*options).show_info as ::core::ffi::c_uint) != 0 {
        tty_moveup(
            tty,
            num_lines.wrapping_add((*options).show_info as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
        );
    }
    tty_setcol(tty, 0 as ::core::ffi::c_int);
    fputs((*options).prompt, (*tty).fout);
    let mut i_0: size_t = 0 as size_t;
    while i_0 < (*state).cursor {
        fputc(
            (*state).search[i_0 as usize] as ::core::ffi::c_int,
            (*tty).fout,
        );
        i_0 = i_0.wrapping_add(1);
    }
    tty_flush(tty);
}
unsafe extern "C" fn update_search(mut state: *mut tty_interface_t) {
    choices_search(
        (*state).choices,
        &raw mut (*state).search as *mut ::core::ffi::c_char,
    );
    strcpy(
        &raw mut (*state).last_search as *mut ::core::ffi::c_char,
        &raw mut (*state).search as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn update_state(mut state: *mut tty_interface_t) {
    if strcmp(
        &raw mut (*state).last_search as *mut ::core::ffi::c_char,
        &raw mut (*state).search as *mut ::core::ffi::c_char,
    ) != 0
    {
        update_search(state);
        draw(state);
    }
}
unsafe extern "C" fn action_emit(mut state: *mut tty_interface_t) {
    update_state(state);
    clear(state);
    tty_close((*state).tty);
    let mut selection: *const ::core::ffi::c_char =
        choices_get((*state).choices, (*(*state).choices).selection);
    if !selection.is_null() {
        printf(
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            selection,
        );
    } else {
        printf(
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*state).search as *mut ::core::ffi::c_char,
        );
    }
    (*state).exit = EXIT_SUCCESS;
}
unsafe extern "C" fn action_del_char(mut state: *mut tty_interface_t) {
    let mut length: size_t = strlen(&raw mut (*state).search as *mut ::core::ffi::c_char);
    if (*state).cursor == 0 as size_t {
        return;
    }
    let mut original_cursor: size_t = (*state).cursor;
    loop {
        (*state).cursor = (*state).cursor.wrapping_sub(1);
        if !(is_boundary((*state).search[(*state).cursor as usize]) == 0 && (*state).cursor != 0) {
            break;
        }
    }
    memmove(
        (&raw mut (*state).search as *mut ::core::ffi::c_char).offset((*state).cursor as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (&raw mut (*state).search as *mut ::core::ffi::c_char).offset(original_cursor as isize)
            as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        length
            .wrapping_sub(original_cursor)
            .wrapping_add(1 as size_t),
    );
}
unsafe extern "C" fn action_del_word(mut state: *mut tty_interface_t) {
    let mut original_cursor: size_t = (*state).cursor;
    let mut cursor: size_t = (*state).cursor;
    while cursor != 0
        && *(*__ctype_b_loc()).offset(
            (*state).search[cursor.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
                as isize,
        ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
    {
        cursor = cursor.wrapping_sub(1);
    }
    while cursor != 0
        && *(*__ctype_b_loc()).offset(
            (*state).search[cursor.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
                as isize,
        ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
    {
        cursor = cursor.wrapping_sub(1);
    }
    memmove(
        (&raw mut (*state).search as *mut ::core::ffi::c_char).offset(cursor as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (&raw mut (*state).search as *mut ::core::ffi::c_char).offset(original_cursor as isize)
            as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut (*state).search as *mut ::core::ffi::c_char)
            .wrapping_sub(original_cursor)
            .wrapping_add(1 as size_t),
    );
    (*state).cursor = cursor;
}
unsafe extern "C" fn action_del_all(mut state: *mut tty_interface_t) {
    memmove(
        &raw mut (*state).search as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (&raw mut (*state).search as *mut ::core::ffi::c_char).offset((*state).cursor as isize)
            as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut (*state).search as *mut ::core::ffi::c_char)
            .wrapping_sub((*state).cursor)
            .wrapping_add(1 as size_t),
    );
    (*state).cursor = 0 as size_t;
}
unsafe extern "C" fn action_prev(mut state: *mut tty_interface_t) {
    update_state(state);
    choices_prev((*state).choices);
}
unsafe extern "C" fn action_ignore(mut state: *mut tty_interface_t) {}
unsafe extern "C" fn action_next(mut state: *mut tty_interface_t) {
    update_state(state);
    choices_next((*state).choices);
}
unsafe extern "C" fn action_left(mut state: *mut tty_interface_t) {
    if (*state).cursor > 0 as size_t {
        (*state).cursor = (*state).cursor.wrapping_sub(1);
        while is_boundary((*state).search[(*state).cursor as usize]) == 0 && (*state).cursor != 0 {
            (*state).cursor = (*state).cursor.wrapping_sub(1);
        }
    }
}
unsafe extern "C" fn action_right(mut state: *mut tty_interface_t) {
    if (*state).cursor < strlen(&raw mut (*state).search as *mut ::core::ffi::c_char) {
        (*state).cursor = (*state).cursor.wrapping_add(1);
        while is_boundary((*state).search[(*state).cursor as usize]) == 0 {
            (*state).cursor = (*state).cursor.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn action_beginning(mut state: *mut tty_interface_t) {
    (*state).cursor = 0 as size_t;
}
unsafe extern "C" fn action_end(mut state: *mut tty_interface_t) {
    (*state).cursor = strlen(&raw mut (*state).search as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn action_pageup(mut state: *mut tty_interface_t) {
    update_state(state);
    let mut i: size_t = 0 as size_t;
    while i < (*(*state).options).num_lines as size_t && (*(*state).choices).selection > 0 as size_t
    {
        choices_prev((*state).choices);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn action_pagedown(mut state: *mut tty_interface_t) {
    update_state(state);
    let mut i: size_t = 0 as size_t;
    while i < (*(*state).options).num_lines as size_t
        && (*(*state).choices).selection < (*(*state).choices).available.wrapping_sub(1 as size_t)
    {
        choices_next((*state).choices);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn action_autocomplete(mut state: *mut tty_interface_t) {
    update_state(state);
    let mut current_selection: *const ::core::ffi::c_char =
        choices_get((*state).choices, (*(*state).choices).selection);
    if !current_selection.is_null() {
        strncpy(
            &raw mut (*state).search as *mut ::core::ffi::c_char,
            choices_get((*state).choices, (*(*state).choices).selection),
            SEARCH_SIZE_MAX as size_t,
        );
        (*state).cursor = strlen(&raw mut (*state).search as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn action_exit(mut state: *mut tty_interface_t) {
    clear(state);
    tty_close((*state).tty);
    (*state).exit = EXIT_FAILURE;
}
unsafe extern "C" fn append_search(mut state: *mut tty_interface_t, mut ch: ::core::ffi::c_char) {
    let mut search: *mut ::core::ffi::c_char = &raw mut (*state).search as *mut ::core::ffi::c_char;
    let mut search_size: size_t = strlen(search);
    if search_size < SEARCH_SIZE_MAX as size_t {
        memmove(
            search.offset((*state).cursor.wrapping_add(1 as size_t) as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            search.offset((*state).cursor as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            search_size
                .wrapping_sub((*state).cursor)
                .wrapping_add(1 as size_t),
        );
        *search.offset((*state).cursor as isize) = ch;
        (*state).cursor = (*state).cursor.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tty_interface_init(
    mut state: *mut tty_interface_t,
    mut tty: *mut tty_t,
    mut choices: *mut choices_t,
    mut options: *mut options_t,
) {
    (*state).tty = tty;
    (*state).choices = choices;
    (*state).options = options;
    (*state).ambiguous_key_pending = 0 as ::core::ffi::c_int;
    strcpy(
        &raw mut (*state).input as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    strcpy(
        &raw mut (*state).search as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    strcpy(
        &raw mut (*state).last_search as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*state).exit = -(1 as ::core::ffi::c_int);
    if !(*options).init_search.is_null() {
        strncpy(
            &raw mut (*state).search as *mut ::core::ffi::c_char,
            (*options).init_search,
            SEARCH_SIZE_MAX as size_t,
        );
    }
    (*state).cursor = strlen(&raw mut (*state).search as *mut ::core::ffi::c_char);
    update_search(state);
}
static mut keybindings: [keybinding_t; 33] = unsafe {
    [
        keybinding_t {
            key: b"\x1B\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_exit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x7F\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_del_char as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [8, 0].as_ptr(),
            action: Some(action_del_char as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [23, 0].as_ptr(),
            action: Some(action_del_word as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [21, 0].as_ptr(),
            action: Some(action_del_all as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [9, 0].as_ptr(),
            action: Some(action_autocomplete as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [3, 0].as_ptr(),
            action: Some(action_exit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [4, 0].as_ptr(),
            action: Some(action_exit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [7, 0].as_ptr(),
            action: Some(action_exit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [13, 0].as_ptr(),
            action: Some(action_emit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [16, 0].as_ptr(),
            action: Some(action_prev as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [14, 0].as_ptr(),
            action: Some(action_next as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [11, 0].as_ptr(),
            action: Some(action_prev as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [10, 0].as_ptr(),
            action: Some(action_next as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [1, 0].as_ptr(),
            action: Some(action_beginning as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: [5, 0].as_ptr(),
            action: Some(action_end as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1BOD\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_left as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[D\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_left as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1BOC\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_right as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[C\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_right as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[1~\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_beginning as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[H\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_beginning as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[4~\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_end as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[F\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_end as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[A\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_prev as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1BOA\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_prev as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[B\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_next as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1BOB\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_next as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[5~\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_pageup as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[6~\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_pagedown as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[200~\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_ignore as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: b"\x1B[201~\0" as *const u8 as *const ::core::ffi::c_char,
            action: Some(action_ignore as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
        },
        keybinding_t {
            key: ::core::ptr::null::<::core::ffi::c_char>(),
            action: None,
        },
    ]
};
unsafe extern "C" fn handle_input(
    mut state: *mut tty_interface_t,
    mut s: *const ::core::ffi::c_char,
    mut handle_ambiguous_key: ::core::ffi::c_int,
) {
    (*state).ambiguous_key_pending = 0 as ::core::ffi::c_int;
    let mut input: *mut ::core::ffi::c_char = &raw mut (*state).input as *mut ::core::ffi::c_char;
    strcat(&raw mut (*state).input as *mut ::core::ffi::c_char, s);
    let mut found_keybinding: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut in_middle: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !keybindings[i as usize].key.is_null() {
        if strcmp(input, keybindings[i as usize].key) == 0 {
            found_keybinding = i;
        } else if strncmp(
            input,
            keybindings[i as usize].key,
            strlen(&raw mut (*state).input as *mut ::core::ffi::c_char),
        ) == 0
        {
            in_middle = 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if found_keybinding != -(1 as ::core::ffi::c_int)
        && (in_middle == 0 || handle_ambiguous_key != 0)
    {
        keybindings[found_keybinding as usize]
            .action
            .expect("non-null function pointer")(state);
        strcpy(input, b"\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    if found_keybinding != -(1 as ::core::ffi::c_int) && in_middle != 0 {
        (*state).ambiguous_key_pending = 1 as ::core::ffi::c_int;
        return;
    }
    if in_middle != 0 {
        return;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *input.offset(i_0 as isize) != 0 {
        if isprint_unicode(*input.offset(i_0 as isize)) != 0 {
            append_search(state, *input.offset(i_0 as isize));
        }
        i_0 += 1;
    }
    strcpy(input, b"\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn tty_interface_run(mut state: *mut tty_interface_t) -> ::core::ffi::c_int {
    draw(state);
    loop {
        loop {
            while tty_input_ready(
                (*state).tty,
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
                1 as ::core::ffi::c_int,
            ) == 0
            {
                draw(state);
            }
            let mut s: [::core::ffi::c_char; 2] = [
                tty_getchar((*state).tty),
                '\0' as i32 as ::core::ffi::c_char,
            ];
            handle_input(
                state,
                &raw mut s as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            if (*state).exit >= 0 as ::core::ffi::c_int {
                return (*state).exit;
            }
            draw(state);
            if !(tty_input_ready(
                (*state).tty,
                (if (*state).ambiguous_key_pending != 0 {
                    KEYTIMEOUT
                } else {
                    0 as ::core::ffi::c_int
                }) as ::core::ffi::c_long,
                0 as ::core::ffi::c_int,
            ) != 0)
            {
                break;
            }
        }
        if (*state).ambiguous_key_pending != 0 {
            let mut s_0: [::core::ffi::c_char; 1] =
                ::core::mem::transmute::<[u8; 1], [::core::ffi::c_char; 1]>(*b"\0");
            handle_input(
                state,
                &raw mut s_0 as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            if (*state).exit >= 0 as ::core::ffi::c_int {
                return (*state).exit;
            }
        }
        update_state(state);
    }
}
pub const TTY_COLOR_HIGHLIGHT: ::core::ffi::c_int = TTY_COLOR_YELLOW;
pub const KEYTIMEOUT: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
