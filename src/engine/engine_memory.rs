//! Port of: engine/engine_memory.c
//! IR hash: 73393814548a07d1
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: fastmod (engine/engine_memory.c:52)
/// Calls: power
#[allow(unused_variables, non_snake_case)]
pub fn fastmod(a: usize, b: usize) -> usize {
    if (b & (b - 1)) == 0 {
        a & (b - 1)
    } else {
        a % b
    }
}

/// C: get_stack_info_from_data (engine/engine_memory.c:74)
#[allow(unused_variables, non_snake_case)]
pub fn get_stack_info_from_data(d: *const mjData) -> mjStackInfo {
    // SAFETY: d is a valid pointer to mjData (caller contract)
    unsafe {
        mjStackInfo {
            bottom: (*d).arena as usize + (*d).narena as usize,
            top: (*d).arena as usize + (*d).narena as usize - (*d).pstack as usize,
            limit: (*d).arena as usize + (*d).parena as usize,
            stack_base: (*d).pbase,
        }
    }
}

/// C: stackallocinternal (engine/engine_memory.c:144)
/// Calls: fastmod, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn stackallocinternal(d: *mut mjData, stack_info: *mut mjStackInfo, size: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    // mjREDZONE = 0 in non-ASAN builds (no ADDRESS_SANITIZER support in Rust port)
    const MJ_REDZONE: usize = 0;

    // SAFETY: d is a valid mjData pointer, stack_info is a valid mjStackInfo pointer.
    // Pointer arithmetic follows the documented C stack layout.
    unsafe {
        // return NULL if empty
        if size == 0 {
            return std::ptr::null_mut();
        }

        // start of the memory to be allocated to the buffer
        let start_ptr: usize = (*stack_info).top - (size + MJ_REDZONE);

        // align the pointer
        let start_ptr: usize = start_ptr - fastmod(start_ptr, alignment);

        // new top of the stack
        let new_top_ptr: usize = start_ptr - MJ_REDZONE;

        // exclude red zone from stack usage statistics
        let current_alloc_usage: usize = (*stack_info).top - new_top_ptr - 2 * MJ_REDZONE;
        let usage: usize = current_alloc_usage + ((*stack_info).bottom - (*stack_info).top);

        // check size
        let stack_available_bytes: usize = (*stack_info).top - (*stack_info).limit;
        let stack_required_bytes: usize = (*stack_info).top - new_top_ptr;
        if stack_required_bytes > stack_available_bytes {
            crate::engine::engine_util_errmem::mju_error(
                b"mj_stackAlloc: out of memory, stack overflow\0".as_ptr() as *const i8);
        }

        // update max usage statistics
        (*stack_info).top = new_top_ptr;
        if usage as i64 > (*d).maxuse_stack {
            (*d).maxuse_stack = usage as i64;
        }
        let arena_usage = usage as i64 + (*d).parena as i64;
        if arena_usage > (*d).maxuse_arena {
            (*d).maxuse_arena = arena_usage;
        }

        start_ptr as *mut ()
    }
}

/// C: stackalloc (engine/engine_memory.c:208)
/// Calls: fastmod, get_stack_info_from_data, mju_error, stackallocinternal
#[allow(unused_variables, non_snake_case)]
pub fn stackalloc(d: *mut mjData, size: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    // mjREDZONE = 0 in non-ASAN builds
    const MJ_REDZONE: usize = 0;

    // SAFETY: d is a valid mjData pointer (caller contract).
    unsafe {
        // size zero: no-op
        if size == 0 {
            return std::ptr::null_mut();
        }

        // call in mju_dispatch: atomically reserve space on the stack
        if (*d).threadlock {
            let alloc_size = size + alignment - 1 + 2 * MJ_REDZONE;
            // atomic fetch_add on d->pstack (usize field)
            let old_pstack = {
                let ptr = &(*d).pstack as *const usize as *const std::sync::atomic::AtomicUsize;
                (*ptr).fetch_add(alloc_size, std::sync::atomic::Ordering::Relaxed)
            };

            // check for stack overflow
            let stack_available_bytes = (*d).narena as usize - (*d).parena as usize;
            if old_pstack + alloc_size > stack_available_bytes {
                crate::engine::engine_util_errmem::mju_error(
                    b"mj_stackAlloc: out of memory, stack overflow (threadlock)\0".as_ptr() as *const i8);
            }

            let bottom = (*d).arena as usize + (*d).narena as usize;
            let mut start_ptr = bottom - old_pstack - size - MJ_REDZONE;
            start_ptr -= fastmod(start_ptr, alignment);
            return start_ptr as *mut ();
        }

        // non-threaded case
        let mut stack_info = get_stack_info_from_data(d as *const mjData);
        let result = stackallocinternal(d, &mut stack_info, size, alignment, caller, line);
        (*d).pstack = stack_info.bottom - stack_info.top;
        result
    }
}

/// C: markstackinternal (engine/engine_memory.c:256)
/// Calls: stackallocinternal
#[allow(unused_variables, non_snake_case)]
pub fn markstackinternal(d: *mut mjData, stack_info: *mut mjStackInfo) {
    // mjStackFrame layout (24 bytes, align 8): { pbase: usize, pstack: usize, pc: *mut () }
    // Non-ASAN build: pc is unused.
    const SIZEOF_MJSTACKFRAME: usize = 24;
    const ALIGNOF_MJSTACKFRAME: usize = 8;

    // SAFETY: d is valid mjData pointer, stack_info is valid mjStackInfo pointer.
    // stackallocinternal returns a valid pointer sized for mjStackFrame.
    unsafe {
        let top_old: usize = (*stack_info).top;
        let s = stackallocinternal(
            d,
            stack_info,
            SIZEOF_MJSTACKFRAME,
            ALIGNOF_MJSTACKFRAME,
            std::ptr::null(),
            0,
        ) as *mut usize;

        // s->pbase = stack_info->stack_base
        *s.add(0) = (*stack_info).stack_base;
        // s->pstack = top_old (= old top before alloc)
        *s.add(1) = top_old;
        // s->pc is not set (non-ASAN build)

        // update stack_base to point to this frame
        (*stack_info).stack_base = s as usize;
    }
}

/// C: freestackinternal (engine/engine_memory.c:292)
/// Calls: mj_freeStack
#[allow(unused_variables, non_snake_case)]
pub fn freestackinternal(stack_info: *mut mjStackInfo) {
    // SAFETY: stack_info is a valid pointer to mjStackInfo (caller contract).
    // stack_base holds a pointer-as-usize to an mjStackFrame allocated on the stack.
    unsafe {
        if (*stack_info).stack_base == 0 {
            return;
        }

        // mjStackFrame layout: { pbase: usize, pstack: usize, pc: *mut () }
        // We only need pbase and pstack (first two fields).
        let s = (*stack_info).stack_base as *const usize;

        // restore pbase and pstack
        (*stack_info).stack_base = *s;           // s->pbase
        (*stack_info).top = *s.add(1);           // s->pstack
    }
}

/// C: mj_arenaAllocByte (engine/engine_memory.h:35)
/// Calls: fastmod
#[allow(unused_variables, non_snake_case)]
pub fn mj_arena_alloc_byte(d: *mut mjData, bytes: usize, alignment: usize) -> *mut () {
    // SAFETY: d is a valid mjData pointer with arena memory (caller contract)
    unsafe {
        let misalignment = fastmod((*d).parena as usize, alignment);
        let padding = if misalignment != 0 { alignment - misalignment } else { 0 };

        // check size
        let bytes_available = (*d).narena as usize - (*d).pstack as usize;
        if (*d).parena as usize + padding + bytes > bytes_available {
            return std::ptr::null_mut();
        }

        let stack_usage = (*d).pstack as usize;

        // allocate, update max, return pointer to buffer
        let result = ((*d).arena as *mut u8).add((*d).parena as usize + padding) as *mut ();
        (*d).parena = ((*d).parena as usize + padding + bytes) as _;
        let new_max = stack_usage + (*d).parena as usize;
        if new_max > (*d).maxuse_arena as usize {
            (*d).maxuse_arena = new_max as _;
        }

        result
    }
}

/// C: mj_markStack (engine/engine_memory.h:40)
/// Calls: get_stack_info_from_data, markstackinternal
#[allow(unused_variables, non_snake_case)]
pub fn mj_mark_stack(d: *mut mjData) {
    // SAFETY: d is a valid pointer (caller contract).
    unsafe {
        // no-op if called from mju_dispatch
        if (*d).threadlock {
            return;
        }

        // get stack info from data
        let bottom = (*d).arena as usize + (*d).narena as usize;
        let top = bottom - (*d).pstack;

        // allocate mjStackFrame on the stack (16 bytes = 2 x usize, aligned to 8)
        let frame_size = 2 * std::mem::size_of::<usize>();
        let alignment = std::mem::align_of::<usize>();
        let start_ptr = (top - frame_size) & !(alignment - 1);
        let new_top = start_ptr;

        // check size
        let limit = (*d).arena as usize + (*d).parena;
        if new_top < limit {
            crate::engine::engine_util_errmem::mju_error(
                b"mj_stackAlloc: stack overflow in mj_markStack\0".as_ptr() as *const i8);
        }

        // write frame: pbase and pstack
        let frame = start_ptr as *mut usize;
        *frame.add(0) = (*d).pbase;   // save current pbase
        *frame.add(1) = top;          // save current top (= bottom - pstack before alloc)

        // update pbase and pstack
        (*d).pbase = start_ptr;
        (*d).pstack = bottom - new_top;

        // update max usage
        let usage = ((*d).pstack as i64) - (frame_size as i64);
        if usage > (*d).maxuse_stack {
            (*d).maxuse_stack = usage;
        }
        let arena_usage = usage + (*d).parena as i64;
        if arena_usage > (*d).maxuse_arena {
            (*d).maxuse_arena = arena_usage;
        }
    }
}

/// C: mj_freeStack (engine/engine_memory.h:43)
/// Calls: freestackinternal, get_stack_info_from_data
#[allow(unused_variables, non_snake_case)]
pub fn mj_free_stack(d: *mut mjData) {
    // SAFETY: d is a valid pointer (caller contract).
    unsafe {
        // no-op if called from mju_dispatch
        if (*d).threadlock {
            return;
        }

        // get stack_base
        let stack_base = (*d).pbase;
        if stack_base == 0 {
            return;
        }

        // read saved frame
        let frame = stack_base as *const usize;
        let saved_pbase = *frame.add(0);
        let saved_top = *frame.add(1);

        // restore pbase and pstack
        let bottom = (*d).arena as usize + (*d).narena as usize;
        (*d).pbase = saved_pbase;
        (*d).pstack = bottom - saved_top;
    }
}

/// C: mj_stackAllocByte (engine/engine_memory.h:53)
/// Calls: stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_byte(d: *mut mjData, bytes: usize, alignment: usize) -> *mut () {
    // SAFETY: d is a valid pointer (caller contract).
    unsafe {
        if bytes == 0 {
            return std::ptr::null_mut();
        }

        // get stack info
        let bottom = (*d).arena as usize + (*d).narena as usize;
        let top = bottom - (*d).pstack;
        let limit = (*d).arena as usize + (*d).parena;

        // allocate: stack grows downward
        let start_ptr = (top - bytes) & !(alignment - 1);
        let new_top = start_ptr;

        // check for stack overflow
        if new_top < limit {
            crate::engine::engine_util_errmem::mju_error(
                b"mj_stackAlloc: stack overflow\0".as_ptr() as *const i8);
        }

        // update pstack and max usage
        (*d).pstack = bottom - new_top;
        let usage = (*d).pstack as i64;
        if usage > (*d).maxuse_stack {
            (*d).maxuse_stack = usage;
        }
        let arena_usage = usage + (*d).parena as i64;
        if arena_usage > (*d).maxuse_arena {
            (*d).maxuse_arena = arena_usage;
        }

        start_ptr as *mut ()
    }
}

/// C: mj_stackAllocInfo (engine/engine_memory.h:56)
/// Calls: stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_info(d: *mut mjData, bytes: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    // SAFETY: d is a valid pointer (caller contract). caller may be null.
    // This is just stackalloc with debug info — same behavior without ASAN.
    mj_stack_alloc_byte(d, bytes, alignment)
}

/// C: mj_stackAllocNum (engine/engine_memory.h:64)
/// Calls: mju_message, stackalloc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_num(d: *mut mjData, size: usize) -> *mut f64 {
    // SAFETY: d is a valid pointer (caller contract).
    unsafe {
        if size == 0 {
            return std::ptr::null_mut();
        }

        let bytes = size * std::mem::size_of::<f64>();
        let alignment = std::mem::align_of::<f64>();

        // get stack info
        let bottom = (*d).arena as usize + (*d).narena as usize;
        let top = bottom - (*d).pstack;
        let limit = (*d).arena as usize + (*d).parena;

        // allocate: stack grows downward
        let start_ptr = (top - bytes) & !(alignment - 1);
        let new_top = start_ptr;

        // check for stack overflow
        if new_top < limit {
            crate::engine::engine_util_errmem::mju_error(
                b"mj_stackAlloc: stack overflow\0".as_ptr() as *const i8);
        }

        // update pstack and max usage
        (*d).pstack = bottom - new_top;
        let usage = (*d).pstack as i64;
        if usage > (*d).maxuse_stack {
            (*d).maxuse_stack = usage;
        }
        let arena_usage = usage + (*d).parena as i64;
        if arena_usage > (*d).maxuse_arena {
            (*d).maxuse_arena = arena_usage;
        }

        start_ptr as *mut f64
    }
}

/// C: mj_stackAllocInt (engine/engine_memory.h:67)
/// Calls: mju_message, stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_int(d: *mut mjData, size: usize) -> *mut i32 {
    // SAFETY: d is a valid pointer (caller contract).
    unsafe {
        if size == 0 {
            return std::ptr::null_mut();
        }

        let bytes = size * std::mem::size_of::<i32>();
        let alignment = std::mem::align_of::<i32>();

        // get stack info
        let bottom = (*d).arena as usize + (*d).narena as usize;
        let top = bottom - (*d).pstack;
        let limit = (*d).arena as usize + (*d).parena;

        // allocate: stack grows downward
        let start_ptr = (top - bytes) & !(alignment - 1);
        let new_top = start_ptr;

        // check for stack overflow
        if new_top < limit {
            crate::engine::engine_util_errmem::mju_error(
                b"mj_stackAlloc: stack overflow\0".as_ptr() as *const i8);
        }

        // update pstack and max usage
        (*d).pstack = bottom - new_top;
        let usage = (*d).pstack as i64;
        if usage > (*d).maxuse_stack {
            (*d).maxuse_stack = usage;
        }
        let arena_usage = usage + (*d).parena as i64;
        if arena_usage > (*d).maxuse_arena {
            (*d).maxuse_arena = arena_usage;
        }

        start_ptr as *mut i32
    }
}

/// C: mj_clearEfc (engine/engine_memory.h:70)
/// Calls: mjCActuator::act
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_efc(d: *mut mjData) {
    // mj_clearEfc: set all arena pointer fields to NULL, reset counts, set contact = arena
    // Implements the MJDATA_ARENA_POINTERS X-macro expansion.
    // SAFETY: d is a valid mjData pointer with all fields accessible.
    unsafe {
        // MJDATA_ARENA_POINTERS_CONTACT
        (*d).contact = (*d).arena as *mut crate::types::mjContact;
        // MJDATA_ARENA_POINTERS_SOLVER
        (*d).efc_type = std::ptr::null_mut();
        (*d).efc_id = std::ptr::null_mut();
        (*d).efc_J_rownnz = std::ptr::null_mut();
        (*d).efc_J_rowadr = std::ptr::null_mut();
        (*d).efc_J_rowsuper = std::ptr::null_mut();
        (*d).efc_J_colind = std::ptr::null_mut();
        (*d).efc_J = std::ptr::null_mut();
        (*d).efc_pos = std::ptr::null_mut();
        (*d).efc_margin = std::ptr::null_mut();
        (*d).efc_frictionloss = std::ptr::null_mut();
        (*d).efc_diagA = std::ptr::null_mut();
        (*d).efc_KBIP = std::ptr::null_mut();
        (*d).efc_D = std::ptr::null_mut();
        (*d).efc_R = std::ptr::null_mut();
        (*d).tendon_efcadr = std::ptr::null_mut();
        (*d).efc_vel = std::ptr::null_mut();
        (*d).efc_aref = std::ptr::null_mut();
        (*d).efc_b = std::ptr::null_mut();
        (*d).efc_state = std::ptr::null_mut();
        (*d).efc_force = std::ptr::null_mut();
        // MJDATA_ARENA_POINTERS_DUAL
        (*d).efc_Y_rownnz = std::ptr::null_mut();
        (*d).efc_Y_rowadr = std::ptr::null_mut();
        (*d).efc_Y_colind = std::ptr::null_mut();
        (*d).efc_Y = std::ptr::null_mut();
        (*d).efc_AR_rownnz = std::ptr::null_mut();
        (*d).efc_AR_rowadr = std::ptr::null_mut();
        (*d).efc_AR_colind = std::ptr::null_mut();
        (*d).efc_AR = std::ptr::null_mut();
        // MJDATA_ARENA_POINTERS_ISLAND
        (*d).tree_island = std::ptr::null_mut();
        (*d).island_ntree = std::ptr::null_mut();
        (*d).island_itreeadr = std::ptr::null_mut();
        (*d).map_itree2tree = std::ptr::null_mut();
        (*d).dof_island = std::ptr::null_mut();
        (*d).island_nv = std::ptr::null_mut();
        (*d).island_idofadr = std::ptr::null_mut();
        (*d).island_dofadr = std::ptr::null_mut();
        (*d).map_dof2idof = std::ptr::null_mut();
        (*d).map_idof2dof = std::ptr::null_mut();
        (*d).ifrc_smooth = std::ptr::null_mut();
        (*d).iacc_smooth = std::ptr::null_mut();
        (*d).iacc = std::ptr::null_mut();
        (*d).efc_island = std::ptr::null_mut();
        (*d).island_ne = std::ptr::null_mut();
        (*d).island_nf = std::ptr::null_mut();
        (*d).island_nefc = std::ptr::null_mut();
        (*d).island_iefcadr = std::ptr::null_mut();
        (*d).map_efc2iefc = std::ptr::null_mut();
        (*d).map_iefc2efc = std::ptr::null_mut();
        (*d).iefc_type = std::ptr::null_mut();
        (*d).iefc_id = std::ptr::null_mut();
        (*d).iefc_frictionloss = std::ptr::null_mut();
        (*d).iefc_D = std::ptr::null_mut();
        (*d).iefc_R = std::ptr::null_mut();
        (*d).iefc_aref = std::ptr::null_mut();
        (*d).iefc_state = std::ptr::null_mut();
        (*d).iefc_force = std::ptr::null_mut();
        (*d).ifrc_constraint = std::ptr::null_mut();
        // Scalars
        (*d).nefc = 0;
        (*d).nisland = 0;
        (*d).nJ = 0;
        (*d).nY = 0;
        (*d).nA = 0;
        // contact efc_address
        for i in 0..(*d).ncon as usize {
            (*(*d).contact.add(i)).efc_address = -1;
        }
    }
}

