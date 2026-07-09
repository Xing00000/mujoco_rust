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
        fn treeCanSleep(m: *const mjModel, d: *const mjData, i: i32, tol: f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { treeCanSleep(m, d, i, tol) }
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
    extern "C" { fn mj_sleepTrees(m: *const mjModel, d: *mut mjData, tree: *const i32, n: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_sleepTrees(m, d, tree, n) }
}

/// C: mj_tendonSleepState (engine/engine_sleep.c:634)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32)
    // Previous return: mjtSleepState
    extern "C" { fn mj_tendonSleepState(m : * const mjModel , d : * const mjData , i : i32) -> mjtSleepState ; } unsafe { mj_tendonSleepState(m , d , i) }
}

/// C: mj_actuatorSleepState (engine/engine_sleep.c:659)
/// Calls: mj_sleepState, mj_tendonSleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    extern "C" { fn mj_actuatorSleepState(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation
    unsafe { mj_actuatorSleepState(m, d, i) }
}

/// C: mj_equalitySleepState (engine/engine_sleep.c:691)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    extern "C" { fn mj_equalitySleepState(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation
    unsafe { mj_equalitySleepState(m, d, i) }
}

/// C: mj_sensorSleepState (engine/engine_sleep.c:727)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    extern "C" { fn mj_sensorSleepState(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation
    unsafe { mj_sensorSleepState(m, d, i) }
}

/// C: mj_updateSleepInit (engine/engine_sleep.h:28)
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep_init(m: *const mjModel, d: *mut mjData, flg_staticawake: i32) {
    // SAFETY: m and d are valid pointers per caller contract.
    // All array accesses are within bounds defined by model dimensions.
    unsafe {
        let ntree = (*m).ntree as i32;
        let nbody = (*m).nbody as i32;
        let nv = (*m).nv as i32;

        // input arrays
        let tree_asleep = (*d).tree_asleep;
        let body_treeid = (*m).body_treeid;
        let body_parentid = (*m).body_parentid;
        let body_rootid = (*m).body_rootid;
        let body_mocapid = (*m).body_mocapid;
        let dof_bodyid = (*m).dof_bodyid;

        // output arrays
        let tree_awake = (*d).tree_awake;
        let body_awake = (*d).body_awake;
        let dof_awake_ind = (*d).dof_awake_ind;
        let body_awake_ind = (*d).body_awake_ind;
        let parent_awake_ind = (*d).parent_awake_ind;

        // tree_awake
        let mut ntree_awake: i32 = 0;
        for i in 0..ntree {
            let val = if *tree_asleep.add(i as usize) < 0 { 1 } else { 0 };
            *tree_awake.add(i as usize) = val;
            ntree_awake += val;
        }
        (*d).ntree_awake = ntree_awake;

        // {body,parent}_awake_ind
        let mut nbody_awake: i32 = 0;
        let mut nparent_awake: i32 = 0;
        for i in 0..nbody {
            // static body
            if *body_treeid.add(i as usize) < 0 {
                if *body_mocapid.add(*body_rootid.add(i as usize) as usize) >= 0 {
                    // mocap body are always awake
                    *body_awake.add(i as usize) = 1; // mjS_AWAKE
                } else {
                    // mark static body unless flg_staticawake is set
                    *body_awake.add(i as usize) = if flg_staticawake != 0 { 1 } else { -1 }; // mjS_AWAKE : mjS_STATIC
                }
            }
            // dynamic body
            else {
                *body_awake.add(i as usize) = if *tree_awake.add(*body_treeid.add(i as usize) as usize) != 0 { 1 } else { 0 }; // mjS_AWAKE : mjS_ASLEEP
            }

            // body_awake_ind: list of awake and static bodies
            if *body_awake.add(i as usize) != 0 { // != mjS_ASLEEP
                *body_awake_ind.add(nbody_awake as usize) = i;
                nbody_awake += 1;
            }

            // parent_awake_ind: list of bodies with awake or static parents
            if i != 0 && *body_awake.add(*body_parentid.add(i as usize) as usize) != 0 { // != mjS_ASLEEP
                *parent_awake_ind.add(nparent_awake as usize) = i;
                nparent_awake += 1;
            }
        }
        (*d).nbody_awake = nbody_awake;
        (*d).nparent_awake = nparent_awake;

        // dof_awake_ind: list of awake degrees of freedom
        let mut nv_awake: i32 = 0;
        for i in 0..nv {
            let bodyid = *dof_bodyid.add(i as usize);
            if *body_treeid.add(bodyid as usize) >= 0 && *body_awake.add(bodyid as usize) == 1 { // mjS_AWAKE
                *dof_awake_ind.add(nv_awake as usize) = i;
                nv_awake += 1;
            }
        }
        (*d).nv_awake = nv_awake;
    }
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
        fn mj_wakeIsland(tree_asleep: *mut i32, ntree: i32, i: i32, wakeval: i32, reason: *const i8, time: f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wakeIsland(tree_asleep, ntree, i, wakeval, reason, time) }
}

/// C: mj_wake (engine/engine_sleep.h:41)
/// Calls: mj_wakeIsland, mju_fillInt, treeCanSleep
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn mj_wake(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wake(m, d) }
}

/// C: mj_wakeCollision (engine/engine_sleep.h:44)
/// Calls: mj_flexBody, mj_wakeIsland, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_collision(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn mj_wakeCollision(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wakeCollision(m, d) }
}

/// C: mj_wakeTendon (engine/engine_sleep.h:47)
/// Calls: mj_wakeIsland, tendonLimit
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_tendon(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn mj_wakeTendon(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wakeTendon(m, d) }
}

/// C: mj_wakeEquality (engine/engine_sleep.h:50)
/// Calls: mj_sleepCycle, mj_wakeIsland, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_equality(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn mj_wakeEquality(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_wakeEquality(m, d) }
}

/// C: mj_sleep (engine/engine_sleep.h:53)
/// Calls: mj_sleepTrees, mju_message, treeCanSleep
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" { fn mj_sleep(m: *const mjModel, d: *mut mjData) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_sleep(m, d) }
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
    extern "C" { fn mj_sleepState(m: *const mjModel, d: *const mjData, r#type: mjtObj, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation; caller guarantees m and d are valid
    unsafe { mj_sleepState(m, d, r#type, i) }
}

