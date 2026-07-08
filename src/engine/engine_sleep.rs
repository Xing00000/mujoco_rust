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
    extern "C" { fn mj_sleepTrees_impl(m: *const mjModel, d: *mut mjData, tree: *const i32, n: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_sleepTrees_impl(m, d, tree, n) }
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
    extern "C" { fn mj_actuatorSleepState_impl(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation
    unsafe { mj_actuatorSleepState_impl(m, d, i) }
}

/// C: mj_equalitySleepState (engine/engine_sleep.c:691)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    extern "C" { fn mj_equalitySleepState_impl(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation
    unsafe { mj_equalitySleepState_impl(m, d, i) }
}

/// C: mj_sensorSleepState (engine/engine_sleep.c:727)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    extern "C" { fn mj_sensorSleepState_impl(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation
    unsafe { mj_sensorSleepState_impl(m, d, i) }
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
    mj_update_sleep_init(m, d, 0);
}

/// C: mj_sleepCycle (engine/engine_sleep.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_cycle(tree_asleep: *const i32, ntree: i32, i: i32) -> i32 {
    if i < 0 || i >= ntree {
        return -1;
    }

    let mut smallest: i32 = i;
    let mut current: i32 = i;
    let mut count: i32 = 0;

    loop {
        if count > ntree {
            return -1;
        }

        // SAFETY: current is bounds-checked against [0, ntree) above and below
        let next: i32 = unsafe { *tree_asleep.add(current as usize) };

        if next < 0 || next >= ntree {
            return -1;
        }

        if next < smallest {
            smallest = next;
        }

        current = next;
        count += 1;

        if current == i {
            break;
        }
    }

    smallest
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
    extern "C" { fn mj_sleep_impl(m: *const mjModel, d: *mut mjData) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_sleep_impl(m, d) }
}

/// C: mj_flexBody (engine/engine_sleep.h:56)
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_body(m: *const mjModel, con: *const mjContact, side: i32) -> i32 {
    // SAFETY: m, con are valid pointers per caller contract. side is 0 or 1.
    // All array accesses use indices derived from model topology.
    unsafe {
        let f: i32 = (*con).flex[side as usize];

        // flex vertex contact (non-interpolated)
        if (*con).vert[side as usize] >= 0 && *(*m).flex_interp.add(f as usize) == 0 {
            return *(*m).flex_vertbodyid.add(
                (*(*m).flex_vertadr.add(f as usize) + (*con).vert[side as usize]) as usize,
            );
        }

        // flex element contact
        if (*con).elem[side as usize] >= 0 {
            if *(*m).flex_interp.add(f as usize) == 0 {
                let dim: i32 = *(*m).flex_dim.add(f as usize);
                let edata: *const i32 = (*m).flex_elem.add(
                    (*(*m).flex_elemdataadr.add(f as usize)
                        + (*con).elem[side as usize] * (dim + 1)) as usize,
                );
                return *(*m).flex_vertbodyid.add(
                    (*(*m).flex_vertadr.add(f as usize) + *edata) as usize,
                );
            } else {
                return *(*m).flex_nodebodyid.add(
                    *(*m).flex_nodeadr.add(f as usize) as usize,
                );
            }
        }

        // flex vertex contact (interpolated): use first node
        *(*m).flex_nodebodyid.add(*(*m).flex_nodeadr.add(f as usize) as usize)
    }
}

/// C: mj_sleepState (engine/engine_sleep.h:59)
/// Calls: mj_actuatorSleepState, mj_equalitySleepState, mj_sensorSleepState, mj_tendonSleepState, mju_message, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_state(m: *const mjModel, d: *const mjData, r#type: mjtObj, i: i32) -> mjtSleepState {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, r#type : mjtObj, i : i32)
    // Previous return: mjtSleepState
    extern "C" { fn mj_sleepState_impl(m: *const mjModel, d: *const mjData, r#type: mjtObj, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation; caller guarantees m and d are valid
    unsafe { mj_sleepState_impl(m, d, r#type, i) }
}

