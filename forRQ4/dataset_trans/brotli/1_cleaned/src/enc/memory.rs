use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[no_mangle]
pub unsafe extern "C" fn BrotliInitMemoryManager(
    mut m: *mut MemoryManager,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut c_void,
) {
    if alloc_func.is_none() {
        (*m).alloc_func = Some(
            BrotliDefaultAllocFunc
                as unsafe extern "C" fn(
                    *mut c_void,
                    size_t,
                ) -> *mut c_void,
        ) as brotli_alloc_func;
        (*m).free_func = Some(
            BrotliDefaultFreeFunc
                as unsafe extern "C" fn(*mut c_void, *mut c_void) -> (),
        ) as brotli_free_func;
        (*m).opaque = ::core::ptr::null_mut::<c_void>();
    } else {
        (*m).alloc_func = alloc_func;
        (*m).free_func = free_func;
        (*m).opaque = opaque;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliAllocate(
    mut m: *mut MemoryManager,
    mut n: size_t,
) -> *mut c_void {
    let mut result: *mut c_void =
        (*m).alloc_func.expect("non-null function pointer")((*m).opaque, n);
    if result.is_null() {
        exit(EXIT_FAILURE);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliFree(mut m: *mut MemoryManager, mut p: *mut c_void) {
    (*m).free_func.expect("non-null function pointer")((*m).opaque, p);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliWipeOutMemoryManager(mut m: *mut MemoryManager) {}
#[no_mangle]
pub unsafe extern "C" fn BrotliBootstrapAlloc(
    mut size: size_t,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut c_void,
) -> *mut c_void {
    if alloc_func.is_none() && free_func.is_none() {
        return malloc(size);
    } else if alloc_func.is_some() && free_func.is_some() {
        return alloc_func.expect("non-null function pointer")(opaque, size);
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBootstrapFree(
    mut address: *mut c_void,
    mut m: *mut MemoryManager,
) {
    if address.is_null() {
        return;
    } else {
        let mut free_func: brotli_free_func = (*m).free_func;
        let mut opaque: *mut c_void = (*m).opaque;
        free_func.expect("non-null function pointer")(opaque, address);
    };
}
