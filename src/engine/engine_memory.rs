//! Port of: engine/engine_memory.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: fastmod (engine/engine_memory.c:52)
#[allow(unused_variables, non_snake_case)]
pub fn fastmod(a: usize, b: usize) -> usize {
    // (b & (b - 1)) == 0 implies b is a power of 2
    if (b & (b - 1)) == 0 {
        a & (b - 1)
    } else {
        a % b
    }
}

/// C: get_stack_info_from_data (engine/engine_memory.c:74)
#[allow(unused_variables, non_snake_case)]
pub fn get_stack_info_from_data(d: *const mjData) -> mjStackInfo {
    // WARNING: signature changed — verify body
    // Previous params: (d : * const mjData)
    // Previous return: mjStackInfo
    extern "C" { fn get_stack_info_from_data(d : * const mjData) -> mjStackInfo ; } unsafe { get_stack_info_from_data(d) }
}

/// C: stackallocinternal (engine/engine_memory.c:144)
/// Calls: fastmod
#[allow(unused_variables, non_snake_case)]
pub fn stackallocinternal(d: *mut mjData, stack_info: *mut mjStackInfo, size: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    extern "C" { fn stackallocinternal(d: *mut mjData, stack_info: *mut mjStackInfo, size: usize, alignment: usize, caller: *const i8, line: i32) -> *mut (); }
    // SAFETY: delegates to C implementation
    unsafe { stackallocinternal(d, stack_info, size, alignment, caller, line) }
}

/// C: stackalloc (engine/engine_memory.c:208)
/// Calls: fastmod, get_stack_info_from_data, stackallocinternal
#[allow(unused_variables, non_snake_case)]
pub fn stackalloc(d: *mut mjData, size: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    extern "C" { fn stackalloc(d: *mut mjData, size: usize, alignment: usize, caller: *const i8, line: i32) -> *mut (); }
    // SAFETY: delegates to C implementation
    unsafe { stackalloc(d, size, alignment, caller, line) }
}

/// C: markstackinternal (engine/engine_memory.c:256)
/// Calls: stackallocinternal
#[allow(unused_variables, non_snake_case)]
pub fn markstackinternal(d: *mut mjData, stack_info: *mut mjStackInfo) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, stack_info : * mut mjStackInfo)
    // Previous return: ()
    extern "C" { fn markstackinternal(d : * mut mjData , stack_info : * mut mjStackInfo) ; } unsafe { markstackinternal(d , stack_info) }
}

/// C: freestackinternal (engine/engine_memory.c:292)
#[allow(unused_variables, non_snake_case)]
pub fn freestackinternal(stack_info: *mut mjStackInfo) {
    // WARNING: signature changed — verify body
    // Previous params: (stack_info : * mut mjStackInfo)
    // Previous return: ()
    extern "C" { fn freestackinternal(stack_info : * mut mjStackInfo) ; } unsafe { freestackinternal(stack_info) }
}

/// C: mj_arenaAllocByte (engine/engine_memory.h:35)
/// Calls: fastmod
#[allow(unused_variables, non_snake_case)]
pub fn mj_arena_alloc_byte(d: *mut mjData, bytes: usize, alignment: usize) -> *mut () {
    // SAFETY: d valid. Allocates from arena, returns null if insufficient space.
    unsafe {
        let misalignment = fastmod((*d).parena, alignment);
        let padding = if misalignment != 0 { alignment - misalignment } else { 0 };

        // check size
        let bytes_available = (*d).narena - (*d).pstack;
        if (*d).parena + padding + bytes > bytes_available {
            return core::ptr::null_mut();
        }

        let stack_usage = (*d).pstack;

        // allocate, update max, return pointer to buffer
        let result = ((*d).arena as *mut u8).add((*d).parena + padding) as *mut ();
        (*d).parena += padding + bytes;
        if stack_usage + (*d).parena > (*d).maxuse_arena {
            (*d).maxuse_arena = stack_usage + (*d).parena;
        }

        result
    }
}

/// C: mj_markStack (engine/engine_memory.h:40)
/// Calls: get_stack_info_from_data, markstackinternal
#[allow(unused_variables, non_snake_case)]
pub fn mj_mark_stack(d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData)
    // Previous return: ()
    extern "C" { fn mj_markStack(d : * mut mjData) ; } unsafe { mj_markStack(d) }
}

/// C: mj_freeStack (engine/engine_memory.h:43)
/// Calls: freestackinternal, get_stack_info_from_data
#[allow(unused_variables, non_snake_case)]
pub fn mj_free_stack(d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData)
    // Previous return: ()
    extern "C" { fn mj_freeStack(d : * mut mjData) ; } unsafe { mj_freeStack(d) }
}

/// C: mj_stackAllocByte (engine/engine_memory.h:53)
/// Calls: stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_byte(d: *mut mjData, bytes: usize, alignment: usize) -> *mut () {
    // SAFETY: d is valid mjData pointer. Delegates to stackalloc with NULL caller.
    unsafe {
        crate::engine::engine_memory::stackalloc(d, bytes, alignment, core::ptr::null(), 0)
    }
}

/// C: mj_stackAllocInfo (engine/engine_memory.h:56)
/// Calls: stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_info(d: *mut mjData, bytes: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    // SAFETY: d is valid mjData pointer, delegates to stackalloc which manages arena
    unsafe {
        crate::engine::engine_memory::stackalloc(d, bytes, alignment, caller, line)
    }
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

    if size >= usize::MAX / core::mem::size_of::<f64>() {
        // SAFETY: calls error handler which aborts — matches mjERROR behavior
        unsafe {
            crate::engine::engine_util_errmem::mju_error(
                b"requested size is too large (more than 2^64 bytes).\0".as_ptr() as *const i8,
            );
        }
    }
    // SAFETY: d is valid mjData pointer; alignment matches _Alignof(mjtNum) == 8
    unsafe {
        crate::engine::engine_memory::stackalloc(
            d,
            size * core::mem::size_of::<f64>(),
            core::mem::align_of::<f64>(),
            core::ptr::null(),
            0,
        ) as *mut f64
    }
}

/// C: mj_stackAllocInt (engine/engine_memory.h:67)
/// Calls: mju_message, stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_int(d: *mut mjData, size: usize) -> *mut i32 {

    if size >= usize::MAX / core::mem::size_of::<i32>() {
        // SAFETY: calls error handler which aborts — matches mjERROR behavior
        unsafe {
            crate::engine::engine_util_errmem::mju_error(
                b"requested size is too large (more than 2^64 bytes).\0".as_ptr() as *const i8,
            );
        }
    }
    // SAFETY: d is valid mjData pointer; alignment matches _Alignof(int) == 4
    unsafe {
        crate::engine::engine_memory::stackalloc(
            d,
            size * core::mem::size_of::<i32>(),
            core::mem::align_of::<i32>(),
            core::ptr::null(),
            0,
        ) as *mut i32
    }
}

/// C: mj_clearEfc (engine/engine_memory.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_efc(d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData)
    // Previous return: ()
    extern "C" { fn mj_clearEfc(d : * mut mjData) ; } unsafe { mj_clearEfc(d) }
}

