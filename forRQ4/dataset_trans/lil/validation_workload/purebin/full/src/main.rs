#![feature(extern_types)]
                                                        
                                                     
                                                               
                                             
                                                                    
                                                                 
                                          
mod libcall {
    use core::ffi::{c_char, c_int, c_void};
    use lillib::src::lil as l;

    pub type Lil = l::lil_t;
    pub type Value = l::lil_value_t;
    pub type List = l::lil_list_t;
    pub type Env = l::lil_env_t;

    const LIL_CALLBACK_WRITE: c_int = 1;

    #[inline]
    pub unsafe fn lil_new() -> Lil {
        l::lil_new()
    }
    #[inline]
    pub unsafe fn lil_free(lil: Lil) {
        l::lil_free(lil)
    }
    #[inline]
    pub unsafe fn lil_set_data(lil: Lil, data: *mut c_void) {
        l::lil_set_data(lil, data)
    }
    #[inline]
    pub unsafe fn lil_get_data(lil: Lil) -> *mut c_void {
        l::lil_get_data(lil)
    }
    #[inline]
    pub unsafe fn lil_callback_write(lil: Lil, f: unsafe extern "C" fn(Lil, *const c_char)) {
        l::lil_callback(
            lil,
            LIL_CALLBACK_WRITE,
            Some(core::mem::transmute::<
                unsafe extern "C" fn(Lil, *const c_char),
                unsafe extern "C" fn() -> (),
            >(f)),
        )
    }
    #[inline]
    pub unsafe fn lil_parse(lil: Lil, code: *const c_char, codelen: usize, funclevel: i32) -> Value {
        l::lil_parse(lil, code, codelen, funclevel as c_int)
    }
    #[inline]
    pub unsafe fn lil_parse_value(lil: Lil, val: Value, funclevel: i32) -> Value {
        l::lil_parse_value(lil, val, funclevel as c_int)
    }
    #[inline]
    pub unsafe fn lil_to_string(val: Value) -> *const c_char {
        l::lil_to_string(val)
    }
    #[inline]
    pub unsafe fn lil_to_integer(val: Value) -> i64 {
        l::lil_to_integer(val) as i64
    }
    #[inline]
    pub unsafe fn lil_to_boolean(val: Value) -> i32 {
        l::lil_to_boolean(val) as i32
    }
    #[inline]
    pub unsafe fn lil_to_double(val: Value) -> f64 {
        l::lil_to_double(val)
    }
    #[inline]
    pub unsafe fn lil_alloc_string(s: *const c_char) -> Value {
        l::lil_alloc_string(s)
    }
    #[inline]
    pub unsafe fn lil_alloc_integer(n: i64) -> Value {
        l::lil_alloc_integer(n)
    }
    #[inline]
    pub unsafe fn lil_alloc_double(n: f64) -> Value {
        l::lil_alloc_double(n)
    }
    #[inline]
    pub unsafe fn lil_clone_value(v: Value) -> Value {
        l::lil_clone_value(v)
    }
    #[inline]
    pub unsafe fn lil_append_char(v: Value, ch: c_char) {
        l::lil_append_char(v, ch);
    }
    #[inline]
    pub unsafe fn lil_append_string(v: Value, s: *const c_char) {
        l::lil_append_string(v, s);
    }
    #[inline]
    pub unsafe fn lil_append_val(v: Value, other: Value) {
        l::lil_append_val(v, other);
    }
    #[inline]
    pub unsafe fn lil_free_value(v: Value) {
        l::lil_free_value(v)
    }
    #[inline]
    pub unsafe fn lil_alloc_list() -> List {
        l::lil_alloc_list()
    }
    #[inline]
    pub unsafe fn lil_free_list(list: List) {
        l::lil_free_list(list)
    }
    #[inline]
    pub unsafe fn lil_list_append(list: List, v: Value) {
        l::lil_list_append(list, v)
    }
    #[inline]
    pub unsafe fn lil_list_size(list: List) -> usize {
        l::lil_list_size(list)
    }
    #[inline]
    pub unsafe fn lil_list_get(list: List, index: usize) -> Value {
        l::lil_list_get(list, index)
    }
    #[inline]
    pub unsafe fn lil_list_to_value(list: List, do_escape: i32) -> Value {
        l::lil_list_to_value(list, do_escape as c_int)
    }
    #[inline]
    pub unsafe fn lil_set_var(lil: Lil, name: *const c_char, val: Value, local: i32) {
        l::lil_set_var(lil, name, val, local as c_int);
    }
    #[inline]
    pub unsafe fn lil_get_var(lil: Lil, name: *const c_char) -> Value {
        l::lil_get_var(lil, name)
    }
    #[inline]
    pub unsafe fn lil_get_var_or(lil: Lil, name: *const c_char, defvalue: Value) -> Value {
        l::lil_get_var_or(lil, name, defvalue)
    }
    #[inline]
    pub unsafe fn lil_push_env(lil: Lil) -> Env {
        l::lil_push_env(lil)
    }
    #[inline]
    pub unsafe fn lil_pop_env(lil: Lil) {
        l::lil_pop_env(lil)
    }
    #[inline]
    pub unsafe fn lil_alloc_env(parent: Env) -> Env {
        l::lil_alloc_env(parent)
    }
    #[inline]
    pub unsafe fn lil_free_env(env: Env) {
        l::lil_free_env(env)
    }
    #[inline]
    pub unsafe fn lil_register(
        lil: Lil,
        name: *const c_char,
        proc_: unsafe extern "C" fn(Lil, usize, *mut Value) -> Value,
    ) {
        l::lil_register(lil, name, Some(proc_));
    }
    #[inline]
    pub unsafe fn lil_call(lil: Lil, name: *const c_char, argc: usize, argv: *mut Value) -> Value {
        l::lil_call(lil, name, argc, argv)
    }
    #[inline]
    pub unsafe fn lil_arg(argv: *mut Value, index: usize) -> Value {
        l::lil_arg(argv, index)
    }
    #[inline]
    pub unsafe fn lil_eval_expr(lil: Lil, code: Value) -> Value {
        l::lil_eval_expr(lil, code)
    }
    #[inline]
    pub unsafe fn lil_subst_to_value(lil: Lil, code: Value) -> Value {
        l::lil_subst_to_value(lil, code)
    }
    #[inline]
    pub unsafe fn lil_subst_to_list(lil: Lil, code: Value) -> List {
        l::lil_subst_to_list(lil, code)
    }
    #[inline]
    pub unsafe fn lil_set_error(lil: Lil, msg: *const c_char) {
        l::lil_set_error(lil, msg)
    }
    #[inline]
    pub unsafe fn lil_set_error_at(lil: Lil, pos: usize, msg: *const c_char) {
        l::lil_set_error_at(lil, pos, msg)
    }
    #[inline]
    pub unsafe fn lil_error(lil: Lil, msg: *mut *const c_char, pos: *mut usize) -> i32 {
        l::lil_error(lil, msg, pos) as i32
    }
    #[inline]
    pub unsafe fn lil_unused_name(lil: Lil, part: *const c_char) -> Value {
        l::lil_unused_name(lil, part)
    }
    #[inline]
    pub unsafe fn lil_write(lil: Lil, msg: *const c_char) {
        l::lil_write(lil, msg)
    }
    #[inline]
    pub unsafe fn lil_freemem(ptr: *mut c_void) {
        l::lil_freemem(ptr)
    }

                                                                      
    extern "C" {
        pub fn strdup(s: *const c_char) -> *mut c_char;
    }
}

include!("../../driver.rs");
