use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[inline]
pub unsafe fn BrotliInitMemoryManager(
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
#[inline]
pub unsafe fn BrotliAllocate(
    mut m: *mut MemoryManager,
    mut n: size_t,
) -> *mut c_void {
    let m_view: &MemoryManager = unsafe { &*m };
    let mut result: *mut c_void =
        m_view.alloc_func.expect("non-null function pointer")(m_view.opaque, n);
    if result.is_null() {
        exit(EXIT_FAILURE);
    }
    return result;
}
#[inline]
pub unsafe fn BrotliFree(mut m: *mut MemoryManager, mut p: *mut c_void) {
    let m_view: &MemoryManager = unsafe { &*m };
    m_view.free_func.expect("non-null function pointer")(m_view.opaque, p);
}
#[inline]
pub unsafe fn BrotliWipeOutMemoryManager(mut m: *mut MemoryManager) {}
#[inline]
pub unsafe fn BrotliBootstrapAlloc(
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
#[inline]
pub unsafe fn BrotliBootstrapFree(
    mut address: *mut c_void,
    mut m: *mut MemoryManager,
) {
    let m_view: &MemoryManager = unsafe { &*m };
    if address.is_null() {
        return;
    } else {
        let mut free_func: brotli_free_func = m_view.free_func;
        let mut opaque: *mut c_void = m_view.opaque;
        free_func.expect("non-null function pointer")(opaque, address);
    };
}
