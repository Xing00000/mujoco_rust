//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: fastmod (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:52)
/// Calls: cxx-internal:engine_core_constraint.c.o:_power
#[allow(unused_variables, non_snake_case)]
pub fn fastmod(a: usize, b: usize) -> usize {
    todo!() // cxx-internal:engine_memory.c.o:_fastmod
}

/// C: get_stack_info_from_data (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:74)
#[allow(unused_variables, non_snake_case)]
pub fn get_stack_info_from_data(d: *const mjData) -> mjStackInfo {
    todo!() // cxx-internal:engine_memory.c.o:_get_stack_info_from_data
}

/// C: stackallocinternal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:144)
/// Calls: cxx-internal:engine_memory.c.o:_fastmod, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn stackallocinternal(d: *mut mjData, stack_info: *mut mjStackInfo, size: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    todo!() // cxx-internal:engine_memory.c.o:_stackallocinternal
}

/// C: stackalloc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:208)
/// Calls: cxx-internal:engine_memory.c.o:_fastmod, cxx-internal:engine_memory.c.o:_get_stack_info_from_data, cxx-internal:engine_memory.c.o:_stackallocinternal, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn stackalloc(d: *mut mjData, size: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    todo!() // cxx-internal:engine_memory.c.o:_stackalloc
}

/// C: markstackinternal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:256)
/// Calls: cxx-internal:engine_memory.c.o:_stackallocinternal
#[allow(unused_variables, non_snake_case)]
pub fn markstackinternal(d: *mut mjData, stack_info: *mut mjStackInfo) {
    todo!() // cxx-internal:engine_memory.c.o:_markstackinternal
}

/// C: freestackinternal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.c:292)
/// Calls: cxx:_mj_freeStack
#[allow(unused_variables, non_snake_case)]
pub fn freestackinternal(stack_info: *mut mjStackInfo) {
    todo!() // cxx-internal:engine_memory.c.o:_freestackinternal
}

/// C: mj_arenaAllocByte (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:35)
/// Calls: cxx-internal:engine_memory.c.o:_fastmod
#[allow(unused_variables, non_snake_case)]
pub fn mj_arena_alloc_byte(d: *mut mjData, bytes: usize, alignment: usize) -> *mut () {
    todo!() // cxx:_mj_arenaAllocByte
}

/// C: mj_markStack (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:40)
/// Calls: cxx-internal:engine_memory.c.o:_get_stack_info_from_data, cxx-internal:engine_memory.c.o:_markstackinternal
#[allow(unused_variables, non_snake_case)]
pub fn mj_mark_stack(d: *mut mjData) {
    todo!() // cxx:_mj_markStack
}

/// C: mj_freeStack (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:43)
/// Calls: cxx-internal:engine_memory.c.o:_freestackinternal, cxx-internal:engine_memory.c.o:_get_stack_info_from_data
#[allow(unused_variables, non_snake_case)]
pub fn mj_free_stack(d: *mut mjData) {
    todo!() // cxx:_mj_freeStack
}

/// C: mj_stackAllocByte (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:53)
/// Calls: cxx-internal:engine_memory.c.o:_stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_byte(d: *mut mjData, bytes: usize, alignment: usize) -> *mut () {
    todo!() // cxx:_mj_stackAllocByte
}

/// C: mj_stackAllocInfo (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:56)
/// Calls: cxx-internal:engine_memory.c.o:_stackalloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_info(d: *mut mjData, bytes: usize, alignment: usize, caller: *const i8, line: i32) -> *mut () {
    todo!() // cxx:_mj_stackAllocInfo
}

/// C: mj_stackAllocNum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:64)
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

/// C: mj_stackAllocInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:67)
/// Calls: cxx-internal:engine_memory.c.o:_stackalloc, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_stack_alloc_int(d: *mut mjData, size: usize) -> *mut i32 {
    todo!() // cxx:_mj_stackAllocInt
}

/// C: mj_clearEfc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:70)
/// Calls: cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_clear_efc(d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_memory.h:__ZL11mj_clearEfcP7mjData_
}

