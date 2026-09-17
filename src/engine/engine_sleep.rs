//! Port of: engine/engine_sleep.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_updateSleepInit (engine/engine_sleep.c:31)
/// Calls: cxx:__ZN7mjCMesh4treeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep_init(m: *const mjModel, d: *mut mjData, flg_staticawake: i32) {
    todo!() // cxx:_mj_updateSleepInit
}

/// C: mj_updateSleep (engine/engine_sleep.c:102)
/// Calls: cxx:_mj_updateSleepInit
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_updateSleep
}

/// C: isSmaller (engine/engine_sleep.c:110)
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

/// C: treeCanSleep (engine/engine_sleep.c:123)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:_isSmaller, cxx:_mju_isZeroByte
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tree_can_sleep(m: *const mjModel, d: *const mjData, i: i32, tol: f64) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_treeCanSleep
}

/// C: mj_sleepCycle (engine/engine_sleep.c:154)
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_cycle(tree_asleep: *const i32, ntree: i32, i: i32) -> i32 {
    todo!() // cxx:_mj_sleepCycle
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

/// C: mj_wakeIsland (engine/engine_sleep.c:195)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:_plural, cxx:_mju_isTopicEnabled, cxx:_mju_message, cxx:_mju_strncpy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_island(tree_asleep: *mut i32, ntree: i32, i: i32, wakeval: i32, reason: *const i8, time: f64) -> i32 {
    todo!() // cxx:_mj_wakeIsland
}

/// C: mj_wake (engine/engine_sleep.c:261)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:_treeCanSleep, cxx:_mj_wakeIsland, cxx:_mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake(m: *const mjModel, d: *mut mjData) -> i32 {
    const K_AWAKE: i32 = -(1 + 10);  // -(1+mjMINAWAKE)
    const MJENBL_SLEEP: i32 = 1 << 4;

    // SAFETY: m, d are valid model/data pointers (caller contract)
    unsafe {
        let ntree = (*m).ntree as i32;
        let mut nwoke: i32 = 0;

        // sleep disabled
        if (*m).opt.enableflags & MJENBL_SLEEP == 0 {
            // sleep disabled but some trees still asleep: wake all
            if (*d).ntree_awake < ntree {
                crate::engine::engine_util_misc::mju_fillInt((*d).tree_asleep, K_AWAKE, ntree);
            }
            return ntree - (*d).ntree_awake;
        }

        // sweep over trees, wake if required
        for i in 0..ntree {
            let asleep = *(*d).tree_asleep.add(i as usize) >= 0;

            // awake: nothing to do
            if !asleep {
                continue;
            }

            // if qpos mismatch or cannot sleep: wake up
            if *(*d).tree_awake.add(i as usize) != 0 || treeCanSleep(m, d as *const mjData, i, 0.0) == 0 {
                nwoke += mj_wakeIsland((*d).tree_asleep, ntree, i, K_AWAKE,
                    b"perturbation\0".as_ptr() as *const i8, (*d).time);
            }
        }

        nwoke
    }
}

/// C: mj_flexBody (engine/engine_sleep.c:293)
/// Calls: cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_body(m: *const mjModel, con: *const mjContact, side: i32) -> i32 {
    todo!() // cxx:_mj_flexBody
}

/// C: mj_wakeCollision (engine/engine_sleep.c:318)
/// Calls: cxx:_mj_flexBody, cxx:_mj_wakeIsland, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_collision(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_wakeCollision
}

/// C: mj_wakeTendon (engine/engine_sleep.c:371)
/// Calls: cxx:_mj_effTendonPossible, cxx:_mj_isMetric, cxx:_mj_wakeIsland, cxx:_tendonLimit
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_tendon(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_wakeTendon
}

/// C: mj_wakeEquality (engine/engine_sleep.c:436)
/// Calls: cxx:_mj_sleepCycle, cxx:_mj_wakeIsland, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_equality(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_wakeEquality
}

/// C: mj_sleepTrees (engine/engine_sleep.c:570)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:_plural, cxx:_mju_isTopicEnabled, cxx:_mju_message, cxx:_mju_strncpy, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_trees(m: *const mjModel, d: *mut mjData, tree: *const i32, n: i32) {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_sleepTrees
}

/// C: mj_sleep (engine/engine_sleep.c:609)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:_treeCanSleep, cxx-internal:engine_sleep.c.o:_mj_sleepTrees, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep(m: *const mjModel, d: *mut mjData) -> i32 {
    const MJENBL_SLEEP: i32 = 1 << 4;
    const MJ_MINAWAKE: i32 = 10;

    // SAFETY: m, d are valid model/data pointers (caller contract)
    unsafe {
        let ntree = (*m).ntree as i32;
        let nisland = (*d).nisland;
        let mut nslept: i32 = 0;

        // sleep disabled
        if (*m).opt.enableflags & MJENBL_SLEEP == 0 {
            return nslept;
        }

        // have constraints but no island structure: can't sleep
        if (*d).nefc != 0 && nisland == 0 {
            return nslept;
        }

        // sweep over awake trees, increment tree_asleep if under tolerance
        for i in 0..ntree {
            // skip sleeping tree
            if *(*d).tree_asleep.add(i as usize) >= 0 {
                continue;
            }

            // increment if can sleep, otherwise wake up
            if treeCanSleep(m, d as *const mjData, i, (*m).opt.sleep_tolerance) != 0 {
                if *(*d).tree_asleep.add(i as usize) < -1 {
                    *(*d).tree_asleep.add(i as usize) += 1;
                }
            } else {
                *(*d).tree_asleep.add(i as usize) = -(1 + MJ_MINAWAKE);
            }
        }

        // sweep over islands, put to sleep if all trees under tolerance
        for i in 0..nisland {
            let mut can_sleep: i32 = 1;
            let start = *(*d).island_itreeadr.add(i as usize);
            let end = start + *(*d).island_ntree.add(i as usize);
            for j in start..end {
                let ta = *(*d).tree_asleep.add(*(*d).map_itree2tree.add(j as usize) as usize);
                if ta < -1 {
                    can_sleep = 0;
                    break;
                } else if ta >= 0 {
                    crate::engine::engine_util_errmem::mju_error(
                        b"found sleeping tree %d in island %d\0".as_ptr() as *const i8);
                }
            }

            // put island to sleep
            if can_sleep != 0 {
                let tree = (*d).map_itree2tree.add(start as usize);
                let n = *(*d).island_ntree.add(i as usize);
                mj_sleepTrees(m, d, tree, n);
                nslept += n;
            }
        }

        // sleep unconstrained trees
        let start = if nisland > 0 {
            *(*d).island_itreeadr.add((nisland - 1) as usize) + *(*d).island_ntree.add((nisland - 1) as usize)
        } else { 0 };
        for j in start..ntree {
            let i = if nisland > 0 { *(*d).map_itree2tree.add(j as usize) } else { j };
            if *(*d).tree_asleep.add(i as usize) == -1 {
                mj_sleepTrees(m, d, &i as *const i32, 1);
                nslept += 1;
            }
        }

        nslept
    }
}

/// C: mj_tendonSleepState (engine/engine_sleep.c:682)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_tendonSleepState
}

/// C: mj_actuatorSleepState (engine/engine_sleep.c:707)
/// Calls: cxx-internal:engine_sleep.c.o:_mj_tendonSleepState, cxx:_mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_actuatorSleepState
}

/// C: mj_equalitySleepState (engine/engine_sleep.c:748)
/// Calls: cxx:_mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_equalitySleepState
}

/// C: mj_sensorSleepState (engine/engine_sleep.c:784)
/// Calls: cxx:_mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    todo!() // cxx-internal:engine_sleep.c.o:_mj_sensorSleepState
}

/// C: mj_sleepState (engine/engine_sleep.c:842)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:_mj_actuatorSleepState, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:_mj_equalitySleepState, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sleep.c:_mj_sensorSleepState, cxx-internal:engine_sleep.c.o:_mj_tendonSleepState, cxx:_mju_message, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_state(m: *const mjModel, d: *const mjData, r#type: u32, i: i32) -> i32 {
    todo!() // cxx:_mj_sleepState
}

