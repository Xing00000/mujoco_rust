//! Port of: engine/engine_memory.c
//! IR hash: 73a9f665ec0ecfc0
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
    todo!() // stackallocinternal
}

/// C: stackalloc (engine/engine_memory.c:208)
/// Calls: fastmod, get_stack_info_from_data, mju_error, stackallocinternal
#[allow(unused_variables, non_snake_case)]
pub fn stackalloc(d: *mut mjData, size: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    todo!() // stackalloc
}

/// C: markstackinternal (engine/engine_memory.c:256)
/// Calls: stackallocinternal
#[allow(unused_variables, non_snake_case)]
pub fn markstackinternal(d: *mut mjData, stack_info: *mut mjStackInfo) {
    todo!() // markstackinternal
}

/// C: freestackinternal (engine/engine_memory.c:292)
/// Calls: mj_freeStack
#[allow(unused_variables, non_snake_case)]
pub fn freestackinternal(stack_info: *mut mjStackInfo) {
    todo!() // freestackinternal
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
    todo!() // mj_clearEfc
}

