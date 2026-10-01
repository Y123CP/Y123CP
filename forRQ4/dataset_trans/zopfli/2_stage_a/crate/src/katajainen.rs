use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_types::*;
extern "C" {
    fn qsort(
        __base: *mut c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub weight: size_t,
    pub tail: *mut Node,
    pub count: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodePool {
    pub next: *mut Node,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const c_void,
        *const c_void,
    ) -> c_int,
>;
pub const CHAR_BIT: c_int = __CHAR_BIT__;
unsafe fn InitNode(
    mut weight: size_t,
    mut count: c_int,
    mut tail: *mut Node,
    mut node: *mut Node,
) {
    (*node).weight = weight;
    (*node).count = count;
    (*node).tail = tail;
}
unsafe fn BoundaryPM(
    mut lists: *mut [*mut Node; 2],
    mut leaves: *mut Node,
    mut numsymbols: c_int,
    mut pool: *mut NodePool,
    mut index: c_int,
) {
    let mut newchain: *mut Node = ::core::ptr::null_mut::<Node>();
    let mut oldchain: *mut Node = ::core::ptr::null_mut::<Node>();
    let mut lastcount: c_int =
        (*(*lists.offset(index as isize))[1 as c_int as usize]).count;
    if index == 0 as c_int && lastcount >= numsymbols {
        return;
    }
    let fresh4 = (*pool).next;
    (*pool).next = (*pool).next.offset(1);
    newchain = fresh4;
    oldchain = (*lists.offset(index as isize))[1 as c_int as usize];
    let ref mut fresh5 = (*lists.offset(index as isize))[0 as c_int as usize];
    *fresh5 = oldchain;
    let ref mut fresh6 = (*lists.offset(index as isize))[1 as c_int as usize];
    *fresh6 = newchain;
    if index == 0 as c_int {
        InitNode(
            (*leaves.offset(lastcount as isize)).weight,
            lastcount + 1 as c_int,
            ::core::ptr::null_mut::<Node>(),
            newchain,
        );
    } else {
        let mut sum: size_t = (*(*lists.offset((index - 1 as c_int) as isize))
            [0 as c_int as usize])
            .weight
            .wrapping_add(
                (*(*lists.offset((index - 1 as c_int) as isize))
                    [1 as c_int as usize])
                    .weight,
            );
        if lastcount < numsymbols && sum > (*leaves.offset(lastcount as isize)).weight {
            InitNode(
                (*leaves.offset(lastcount as isize)).weight,
                lastcount + 1 as c_int,
                (*oldchain).tail,
                newchain,
            );
        } else {
            InitNode(
                sum,
                lastcount,
                (*lists.offset((index - 1 as c_int) as isize))
                    [1 as c_int as usize],
                newchain,
            );
            BoundaryPM(
                lists,
                leaves,
                numsymbols,
                pool,
                index - 1 as c_int,
            );
            BoundaryPM(
                lists,
                leaves,
                numsymbols,
                pool,
                index - 1 as c_int,
            );
        }
    };
}
unsafe fn BoundaryPMFinal(
    mut lists: *mut [*mut Node; 2],
    mut leaves: *mut Node,
    mut numsymbols: c_int,
    mut pool: *mut NodePool,
    mut index: c_int,
) {
    let pool_view: &NodePool = unsafe { &*pool };
    let mut lastcount: c_int =
        (*(*lists.offset(index as isize))[1 as c_int as usize]).count;
    let mut sum: size_t = (*(*lists.offset((index - 1 as c_int) as isize))
        [0 as c_int as usize])
        .weight
        .wrapping_add(
            (*(*lists.offset((index - 1 as c_int) as isize))
                [1 as c_int as usize])
                .weight,
        );
    if lastcount < numsymbols && sum > (*leaves.offset(lastcount as isize)).weight {
        let mut newchain: *mut Node = pool_view.next;
        let mut oldchain: *mut Node =
            (*(*lists.offset(index as isize))[1 as c_int as usize]).tail;
        let ref mut fresh2 = (*lists.offset(index as isize))[1 as c_int as usize];
        *fresh2 = newchain;
        (*newchain).count = lastcount + 1 as c_int;
        (*newchain).tail = oldchain;
    } else {
        let ref mut fresh3 =
            (*(*lists.offset(index as isize))[1 as c_int as usize]).tail;
        *fresh3 = (*lists.offset((index - 1 as c_int) as isize))
            [1 as c_int as usize];
    };
}
unsafe fn InitLists(
    mut pool: *mut NodePool,
    mut leaves: *const Node,
    mut maxbits: c_int,
    mut lists: *mut [*mut Node; 2],
) {
    let pool_view: &mut NodePool = unsafe { &mut *pool };
    let mut i: c_int = 0;
    let fresh7 = pool_view.next;
    pool_view.next = pool_view.next.offset(1);
    let mut node0: *mut Node = fresh7;
    let fresh8 = pool_view.next;
    pool_view.next = pool_view.next.offset(1);
    let mut node1: *mut Node = fresh8;
    InitNode(
        (*leaves.offset(0 as c_int as isize)).weight,
        1 as c_int,
        ::core::ptr::null_mut::<Node>(),
        node0,
    );
    InitNode(
        (*leaves.offset(1 as c_int as isize)).weight,
        2 as c_int,
        ::core::ptr::null_mut::<Node>(),
        node1,
    );
    i = 0 as c_int;
    while i < maxbits {
        let ref mut fresh9 = (*lists.offset(i as isize))[0 as c_int as usize];
        *fresh9 = node0;
        let ref mut fresh10 = (*lists.offset(i as isize))[1 as c_int as usize];
        *fresh10 = node1;
        i += 1;
    }
}
unsafe fn ExtractBitLengths(
    mut chain: *mut Node,
    mut leaves: *mut Node,
    mut bitlengths: *mut c_uint,
) {
    let mut counts: [c_int; 16] = [0 as c_int; 16];
    let mut end: c_uint = 16 as c_uint;
    let mut ptr: c_uint = 15 as c_uint;
    let mut value: c_uint = 1 as c_uint;
    let mut node: *mut Node = ::core::ptr::null_mut::<Node>();
    let mut val: c_int = 0;
    node = chain;
    while !node.is_null() {
        end = end.wrapping_sub(1);
        counts[end as usize] = (*node).count;
        node = (*node).tail;
    }
    val = counts[15 as c_int as usize];
    while ptr >= end {
        while val > counts[ptr.wrapping_sub(1 as c_uint) as usize] {
            *bitlengths.offset(
                (*leaves.offset((val - 1 as c_int) as isize)).count as isize,
            ) = value;
            val -= 1;
        }
        ptr = ptr.wrapping_sub(1);
        value = value.wrapping_add(1);
    }
}
unsafe extern "C" fn LeafComparator(
    mut a: *const c_void,
    mut b: *const c_void,
) -> c_int {
    return (*(a as *const Node))
        .weight
        .wrapping_sub((*(b as *const Node)).weight) as c_int;
}
#[inline]
pub unsafe fn ZopfliLengthLimitedCodeLengths(
    mut frequencies: *const size_t,
    mut n: c_int,
    mut maxbits: c_int,
    mut bitlengths: *mut c_uint,
) -> c_int {
    let frequencies_view: &[size_t] = unsafe { core::slice::from_raw_parts(frequencies, (n) as usize) };
    let mut pool: NodePool = NodePool {
        next: ::core::ptr::null_mut::<Node>(),
    };
    let mut i: c_int = 0;
    let mut numsymbols: c_int = 0 as c_int;
    let mut numBoundaryPMRuns: c_int = 0;
    let mut nodes: *mut Node = ::core::ptr::null_mut::<Node>();
    let mut lists: *mut [*mut Node; 2] = ::core::ptr::null_mut::<[*mut Node; 2]>();
    let mut leaves: *mut Node =
        malloc((n as size_t).wrapping_mul(::core::mem::size_of::<Node>() as size_t)) as *mut Node;
    i = 0 as c_int;
    while i < n {
        *bitlengths.offset(i as isize) = 0 as c_uint;
        i += 1;
    }
    i = 0 as c_int;
    while i < n {
        if frequencies_view[(i) as usize] != 0 {
            (*leaves.offset(numsymbols as isize)).weight = frequencies_view[(i) as usize];
            (*leaves.offset(numsymbols as isize)).count = i;
            numsymbols += 1;
        }
        i += 1;
    }
    if (1 as c_int) << maxbits < numsymbols {
        free(leaves as *mut c_void);
        return 1 as c_int;
    }
    if numsymbols == 0 as c_int {
        free(leaves as *mut c_void);
        return 0 as c_int;
    }
    if numsymbols == 1 as c_int {
        *bitlengths.offset((*leaves.offset(0 as c_int as isize)).count as isize) =
            1 as c_uint;
        free(leaves as *mut c_void);
        return 0 as c_int;
    }
    if numsymbols == 2 as c_int {
        let ref mut fresh0 =
            *bitlengths.offset((*leaves.offset(0 as c_int as isize)).count as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        let ref mut fresh1 =
            *bitlengths.offset((*leaves.offset(1 as c_int as isize)).count as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        free(leaves as *mut c_void);
        return 0 as c_int;
    }
    i = 0 as c_int;
    while i < numsymbols {
        if (*leaves.offset(i as isize)).weight
            >= (1 as c_int as size_t)
                << (::core::mem::size_of::<size_t>() as usize)
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(9 as usize)
        {
            free(leaves as *mut c_void);
            return 1 as c_int;
        }
        (*leaves.offset(i as isize)).weight = (*leaves.offset(i as isize)).weight
            << 9 as c_int
            | (*leaves.offset(i as isize)).count as size_t;
        i += 1;
    }
    qsort(
        leaves as *mut c_void,
        numsymbols as size_t,
        ::core::mem::size_of::<Node>() as size_t,
        Some(
            LeafComparator
                as unsafe extern "C" fn(
                    *const c_void,
                    *const c_void,
                ) -> c_int,
        ),
    );
    i = 0 as c_int;
    while i < numsymbols {
        (*leaves.offset(i as isize)).weight >>= 9 as c_int;
        i += 1;
    }
    if (numsymbols - 1 as c_int) < maxbits {
        maxbits = numsymbols - 1 as c_int;
    }
    nodes = malloc(
        ((maxbits * 2 as c_int * numsymbols) as size_t)
            .wrapping_mul(::core::mem::size_of::<Node>() as size_t),
    ) as *mut Node;
    pool.next = nodes;
    lists = malloc(
        (maxbits as size_t).wrapping_mul(::core::mem::size_of::<[*mut Node; 2]>() as size_t),
    ) as *mut [*mut Node; 2];
    InitLists(&raw mut pool, leaves, maxbits, lists);
    numBoundaryPMRuns = 2 as c_int * numsymbols - 4 as c_int;
    i = 0 as c_int;
    while i < numBoundaryPMRuns - 1 as c_int {
        BoundaryPM(
            lists,
            leaves,
            numsymbols,
            &raw mut pool,
            maxbits - 1 as c_int,
        );
        i += 1;
    }
    BoundaryPMFinal(
        lists,
        leaves,
        numsymbols,
        &raw mut pool,
        maxbits - 1 as c_int,
    );
    ExtractBitLengths(
        (*lists.offset((maxbits - 1 as c_int) as isize))
            [1 as c_int as usize],
        leaves,
        bitlengths,
    );
    free(lists as *mut c_void);
    free(leaves as *mut c_void);
    free(nodes as *mut c_void);
    return 0 as c_int;
}
pub const __CHAR_BIT__: c_int = 8 as c_int;
