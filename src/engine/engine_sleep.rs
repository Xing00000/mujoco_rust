//! Port of: engine/engine_sleep.c
//! IR hash: d2209344472ae336
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
    // SAFETY: vec and weight point to at least n f64 elements (caller contract)
    unsafe {
        let mut max: f64 = 0.0;
        for i in 0..n as usize {
            let val = *weight.add(i) * (*vec.add(i)).abs();
            max = if max > val { max } else { val };
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
    const mjSLEEP_NEVER: i32 = 3;
    const mjSLEEP_AUTO_NEVER: i32 = 1;

    // SAFETY: m, d are valid model/data pointers; i is a valid tree index (caller contract)
    unsafe {
        // check sleep policy
        if *(*m).tree_sleep_policy.add(i as usize) == mjSLEEP_NEVER
            || *(*m).tree_sleep_policy.add(i as usize) == mjSLEEP_AUTO_NEVER
        {
            return 0;
        }

        // check xfrc_applied
        let mut adr = *(*m).tree_bodyadr.add(i as usize);
        let mut num = *(*m).tree_bodynum.add(i as usize);
        if crate::engine::engine_util_misc::mju_is_zero_byte(
            (*d).xfrc_applied.add(6 * adr as usize) as *const u8,
            6 * num * (core::mem::size_of::<f64>() as i32),
        ) == 0
        {
            return 0;
        }

        // check qfrc_applied
        adr = *(*m).tree_dofadr.add(i as usize);
        num = *(*m).tree_dofnum.add(i as usize);
        if crate::engine::engine_util_misc::mju_is_zero_byte(
            (*d).qfrc_applied.add(adr as usize) as *const u8,
            num * (core::mem::size_of::<f64>() as i32),
        ) == 0
        {
            return 0;
        }

        // check qvel
        if tol != 0.0 {
            is_smaller((*d).qvel.add(adr as usize), (*m).dof_length.add(adr as usize), num, tol)
        } else {
            crate::engine::engine_util_misc::mju_is_zero_byte(
                (*d).qvel.add(adr as usize) as *const u8,
                num * (core::mem::size_of::<f64>() as i32),
            )
        }
    }
}

/// C: plural (engine/engine_sleep.c:189)
#[allow(unused_variables, non_snake_case)]
pub fn plural(n: i32) -> *const i8 {
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
    todo!() // mj_sleepTrees
}

/// C: mj_tendonSleepState (engine/engine_sleep.c:634)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    const mjS_STATIC: i32 = -1;
    const mjS_ASLEEP: i32 = 0;
    const mjS_AWAKE: i32 = 1;

    // SAFETY: m and d are valid model/data pointers; i is a valid tendon index
    unsafe {
        let treenum: i32 = *(*m).tendon_treenum.add(i as usize);

        // no trees: tendon is static
        if treenum == 0 {
            return mjS_STATIC;
        }

        // single tree: awake if tree is awake, asleep otherwise
        let id1: i32 = *(*m).tendon_treeid.add((2 * i) as usize);
        if treenum == 1 {
            return if *(*d).tree_awake.add(id1 as usize) != 0 { mjS_AWAKE } else { mjS_ASLEEP };
        }

        // two trees: asleep only if both are asleep
        let id2: i32 = *(*m).tendon_treeid.add((2 * i + 1) as usize);
        if treenum == 2 {
            return if *(*d).tree_awake.add(id1 as usize) != 0 || *(*d).tree_awake.add(id2 as usize) != 0 {
                mjS_AWAKE
            } else {
                mjS_ASLEEP
            };
        }

        mjS_AWAKE
    }
}

/// C: mj_actuatorSleepState (engine/engine_sleep.c:659)
/// Calls: mj_sleepState, mj_tendonSleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // mj_actuatorSleepState
}

/// C: mj_equalitySleepState (engine/engine_sleep.c:691)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // mj_equalitySleepState
}

/// C: mj_sensorSleepState (engine/engine_sleep.c:727)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // mj_sensorSleepState
}

/// C: mj_updateSleepInit (engine/engine_sleep.h:28)
/// Calls: mjCMesh::tree
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep_init(m: *const mjModel, d: *mut mjData, flg_staticawake: i32) {
    todo!() // mj_updateSleepInit
}

/// C: mj_updateSleep (engine/engine_sleep.h:31)
/// Calls: mj_updateSleepInit
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_updateSleep
}

/// C: mj_sleepCycle (engine/engine_sleep.h:34)
/// Calls: GlobalTable::count, next
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_cycle(tree_asleep: *const i32, ntree: i32, i: i32) -> i32 {
    todo!() // mj_sleepCycle
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
    todo!() // mj_wakeIsland
}

/// C: mj_wake (engine/engine_sleep.h:41)
/// Calls: mj_wakeIsland, mju_fillInt, treeCanSleep
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // mj_wake
}

/// C: mj_wakeCollision (engine/engine_sleep.h:44)
/// Calls: mj_flexBody, mj_wakeIsland, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_collision(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // mj_wakeCollision
}

/// C: mj_wakeTendon (engine/engine_sleep.h:47)
/// Calls: mj_wakeIsland, tendonLimit
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_tendon(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // mj_wakeTendon
}

/// C: mj_wakeEquality (engine/engine_sleep.h:50)
/// Calls: mj_sleepCycle, mj_wakeIsland, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_equality(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // mj_wakeEquality
}

/// C: mj_sleep (engine/engine_sleep.h:53)
/// Calls: mj_sleepTrees, mju_message, treeCanSleep
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // mj_sleep
}

/// C: mj_flexBody (engine/engine_sleep.h:56)
/// Calls: mjCActuator::act
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_body(m: *const mjModel, con: *const mjContact, side: i32) -> i32 {
    todo!() // mj_flexBody
}

/// C: mj_sleepState (engine/engine_sleep.h:59)
/// Calls: mj_actuatorSleepState, mj_equalitySleepState, mj_sensorSleepState, mj_tendonSleepState, mju_message, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_state(m: *const mjModel, d: *const mjData, r#type: u32, i: i32) -> i32 {
    todo!() // mj_sleepState
}

