//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: isSmaller (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:110)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_smaller(vec: *const f64, weight: *const f64, n: i32, tol: f64) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_isSmaller
}

/// C: treeCanSleep (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:123)
/// Calls: cxx-internal:engine_sleep.c.o:_isSmaller, cxx:_mju_isZeroByte
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tree_can_sleep(m: *const mjModel, d: *const mjData, i: i32, tol: f64) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_treeCanSleep
}

/// C: plural (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:189)
#[allow(unused_variables, non_snake_case)]
pub fn plural(n: i32) -> *const i8 {
    todo!() // cxx-internal:engine_sleep.c.o:_plural
}

/// C: mj_sleepTrees (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:522)
/// Calls: cxx-internal:engine_sleep.c.o:_plural, cxx:_mju_isTopicEnabled, cxx:_mju_message, cxx:_mju_strncpy, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_trees(m: *const mjModel, d: *mut mjData, tree: *const i32, n: i32) {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_sleepTrees
}

/// C: mj_tendonSleepState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:634)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_tendonSleepState
}

/// C: mj_actuatorSleepState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:659)
/// Calls: cxx-internal:engine_sleep.c.o:_mj_tendonSleepState, cxx:_mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_actuatorSleepState
}

/// C: mj_equalitySleepState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:691)
/// Calls: cxx:_mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_equalitySleepState
}

/// C: mj_sensorSleepState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:727)
/// Calls: cxx:_mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_sensorSleepState
}

/// C: mj_updateSleepInit (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:28)
/// Calls: cxx:__ZN7mjCMesh4treeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep_init(m: *const mjModel, d: *mut mjData, flg_staticawake: i32) {
    todo!() // cxx:_mj_updateSleepInit
}

/// C: mj_updateSleep (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:31)
/// Calls: cxx:_mj_updateSleepInit
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_updateSleep
}

/// C: mj_sleepCycle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:34)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_next
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_cycle(tree_asleep: *const i32, ntree: i32, i: i32) -> i32 {
    todo!() // cxx:_mj_sleepCycle
}

/// C: mj_wakeIsland (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:37)
/// Calls: cxx-internal:engine_sleep.c.o:_plural, cxx:_mju_isTopicEnabled, cxx:_mju_message, cxx:_mju_strncpy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_island(tree_asleep: *mut i32, ntree: i32, i: i32, wakeval: i32, reason: *const i8, time: f64) -> i32 {
    todo!() // cxx:_mj_wakeIsland
}

/// C: mj_wake (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:41)
/// Calls: cxx-internal:engine_sleep.c.o:_treeCanSleep, cxx:_mj_wakeIsland, cxx:_mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_wake
}

/// C: mj_wakeCollision (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:44)
/// Calls: cxx:_mj_flexBody, cxx:_mj_wakeIsland, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_collision(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_wakeCollision
}

/// C: mj_wakeTendon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:47)
/// Calls: cxx:_mj_wakeIsland, cxx:_tendonLimit
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_tendon(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_wakeTendon
}

/// C: mj_wakeEquality (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:50)
/// Calls: cxx:_mj_sleepCycle, cxx:_mj_wakeIsland, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_equality(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_wakeEquality
}

/// C: mj_sleep (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:53)
/// Calls: cxx-internal:engine_sleep.c.o:_mj_sleepTrees, cxx-internal:engine_sleep.c.o:_treeCanSleep, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_sleep
}

/// C: mj_flexBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:56)
/// Calls: cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_body(m: *const mjModel, con: *const mjContact, side: i32) -> i32 {
    todo!() // cxx:_mj_flexBody
}

/// C: mj_sleepState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.h:59)
/// Calls: cxx-internal:engine_sleep.c.o:_mj_actuatorSleepState, cxx-internal:engine_sleep.c.o:_mj_equalitySleepState, cxx-internal:engine_sleep.c.o:_mj_sensorSleepState, cxx-internal:engine_sleep.c.o:_mj_tendonSleepState, cxx:_mju_message, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_state(m: *const mjModel, d: *const mjData, r#type: u32, i: i32) -> i32 {
    todo!() // cxx:_mj_sleepState
}

