//! Port of: engine/engine_sleep.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: isSmaller (engine/engine_sleep.c:110)
/// Calls: mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_smaller(vec: *const f64, weight: *const f64, n: i32, tol: f64) -> i32 {
    // SAFETY: caller guarantees valid buffers of at least n elements
    unsafe {
        let mut max: f64 = 0.0;
        for i in 0..n as usize {
            max = crate::engine::engine_util_misc::mju_max(max, *weight.add(i) * (*vec.add(i)).abs());
            if max >= tol {
                return 0;
            }
        }
        1
    }
}

/// C: treeCanSleep (engine/engine_sleep.c:123)
/// Calls: isSmaller, mju_isZeroByte
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tree_can_sleep(m: *const mjModel, d: *const mjData, i: i32, tol: f64) -> i32 {
    extern "C" {
        fn treeCanSleep_impl(m: *const mjModel, d: *const mjData, i: i32, tol: f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { treeCanSleep_impl(m, d, i, tol) }
}

/// C: plural (engine/engine_sleep.c:189)
#[allow(unused_variables, non_snake_case)]
pub fn plural(n: i32) -> *const i8 {
    // SAFETY: returns pointer to static string literal
    if n > 1 {
        b"s\0".as_ptr() as *const i8
    } else {
        b"\0".as_ptr() as *const i8
    }
}

/// C: mj_sleepTrees (engine/engine_sleep.c:522)
/// Calls: mju_isTopicEnabled, mju_message, mju_strncpy, mju_zero, plural
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_trees(m: *const mjModel, d: *mut mjData, tree: *const i32, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, tree : * const i32, n : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_tendonSleepState (engine/engine_sleep.c:634)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32)
    // Previous return: mjtSleepState
    extern "C" { fn mj_tendonSleepState_impl (m : * const mjModel , d : * const mjData , i : i32) -> mjtSleepState ; } unsafe { mj_tendonSleepState_impl (m , d , i) }
}

/// C: mj_actuatorSleepState (engine/engine_sleep.c:659)
/// Calls: mj_sleepState, mj_tendonSleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32)
    // Previous return: mjtSleepState
    todo ! ()
}

/// C: mj_equalitySleepState (engine/engine_sleep.c:691)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32)
    // Previous return: mjtSleepState
    todo ! ()
}

/// C: mj_sensorSleepState (engine/engine_sleep.c:727)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32)
    // Previous return: mjtSleepState
    todo ! ()
}

/// C: mj_updateSleepInit (engine/engine_sleep.h:28)
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep_init(m: *const mjModel, d: *mut mjData, flg_staticawake: i32) {
    // SAFETY: complex struct access pattern, delegating to C implementation
    extern "C" { fn mj_updateSleepInit(m: *const mjModel, d: *mut mjData, flg_staticawake: i32); }
    unsafe { mj_updateSleepInit(m, d, flg_staticawake); }
}

/// C: mj_updateSleep (engine/engine_sleep.h:31)
/// Calls: mj_updateSleepInit
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_updateSleep_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_updateSleep_impl(m, d) }
}

/// C: mj_sleepCycle (engine/engine_sleep.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_cycle(tree_asleep: *const i32, ntree: i32, i: i32) -> i32 {
    extern "C" {
        fn mj_sleepCycle_impl(tree_asleep: *const i32, ntree: i32, i: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_sleepCycle_impl(tree_asleep, ntree, i) }
}

/// C: mj_wakeIsland (engine/engine_sleep.h:37)
/// Calls: mju_isTopicEnabled, mju_message, mju_strncpy, plural
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_island(tree_asleep: *mut i32, ntree: i32, i: i32, wakeval: i32, reason: *const i8, time: f64) -> i32 {
    extern "C" {
        fn mj_wakeIsland_impl(tree_asleep: *mut i32, ntree: i32, i: i32, wakeval: i32, reason: *const i8, time: f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wakeIsland_impl(tree_asleep, ntree, i, wakeval, reason, time) }
}

/// C: mj_wake (engine/engine_sleep.h:41)
/// Calls: mj_wakeIsland, mju_fillInt, treeCanSleep
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn mj_wake_impl(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wake_impl(m, d) }
}

/// C: mj_wakeCollision (engine/engine_sleep.h:44)
/// Calls: mj_flexBody, mj_wakeIsland, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_collision(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn mj_wakeCollision_impl(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wakeCollision_impl(m, d) }
}

/// C: mj_wakeTendon (engine/engine_sleep.h:47)
/// Calls: mj_wakeIsland, tendonLimit
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_tendon(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn mj_wakeTendon_impl(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wakeTendon_impl(m, d) }
}

/// C: mj_wakeEquality (engine/engine_sleep.h:50)
/// Calls: mj_sleepCycle, mj_wakeIsland, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_equality(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn mj_wakeEquality_impl(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wakeEquality_impl(m, d) }
}

/// C: mj_sleep (engine/engine_sleep.h:53)
/// Calls: mj_sleepTrees, mju_message, treeCanSleep
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep(m: *const mjModel, d: *mut mjData) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: i32
    todo ! ()
}

/// C: mj_flexBody (engine/engine_sleep.h:56)
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_body(m: *const mjModel, con: *const mjContact, side: i32) -> i32 {
    extern "C" {
        fn mj_flexBody_impl(m: *const mjModel, con: *const mjContact, side: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_flexBody_impl(m, con, side) }
}

/// C: mj_sleepState (engine/engine_sleep.h:59)
/// Calls: mj_actuatorSleepState, mj_equalitySleepState, mj_sensorSleepState, mj_tendonSleepState, mju_message, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_state(m: *const mjModel, d: *const mjData, r#type: mjtObj, i: i32) -> mjtSleepState {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, r#type : mjtObj, i : i32)
    // Previous return: mjtSleepState
    todo ! ()
}

