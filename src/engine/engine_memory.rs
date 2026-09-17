//! Port of: engine/engine_memory.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: fastmod (engine/engine_memory.c:52)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_power
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

/// C: mj_arenaAllocByte (engine/engine_memory.c:107)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_fastmod
#[allow(unused_variables, non_snake_case)]
pub fn mj_arena_alloc_byte(d: *mut mjData, bytes: usize, alignment: usize) -> *mut () {
    todo!() // cxx:_mj_arenaAllocByte
}

/// C: stackallocinternal (engine/engine_memory.c:144)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_fastmod, cxx:_mju_error
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_fastmod, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_get_stack_info_from_data, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_stackallocinternal, cxx:_mju_error
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_stackallocinternal
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

/// C: mj_markStack (engine/engine_memory.c:276)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_get_stack_info_from_data, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_markstackinternal
#[allow(unused_variables, non_snake_case)]
pub fn mj_mark_stack(d: *mut mjData) {
    todo!() // cxx:_mj_markStack
}

/// C: freestackinternal (engine/engine_memory.c:292)
/// Calls: cxx:_mj_freeStack
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

/// C: mj_freeStack (engine/engine_memory.c:325)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_freestackinternal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:_get_stack_info_from_data
#[allow(unused_variables, non_snake_case)]
pub fn mj_free_stack(d: *mut mjData) {
    todo!() // cxx:_mj_freeStack
}

/// C: mj_stackAllocByte (engine/engine_memory.c:338)
/// Calls: cxx-internal:engine_memory.c.o:_stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_byte(d: *mut mjData, bytes: usize, alignment: usize) -> *mut () {
    todo!() // cxx:_mj_stackAllocByte
}

/// C: mj_stackAllocInfo (engine/engine_memory.c:345)
/// Calls: cxx-internal:engine_memory.c.o:_stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_info(d: *mut mjData, bytes: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    todo!() // cxx:_mj_stackAllocInfo
}

/// C: mj_stackAllocNum (engine/engine_memory.c:351)
/// Calls: cxx-internal:engine_memory.c.o:_stackalloc, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_num(d: *mut mjData, size: usize) -> *mut f64 {
    todo!() // cxx:_mj_stackAllocNum
}

/// C: mj_stackAllocInt (engine/engine_memory.c:360)
/// Calls: cxx-internal:engine_memory.c.o:_stackalloc, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_int(d: *mut mjData, size: usize) -> *mut i32 {
    todo!() // cxx:_mj_stackAllocInt
}

/// C: mj_clearEfc (engine/engine_memory.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_efc__8b00d6e22377bbeb(d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:__ZL11mj_clearEfcP7mjData_
}

/// C: mj_clearEfc (engine/engine_memory.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_efc__b700390f8226d7d1(d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:_mj_clearEfc
}

