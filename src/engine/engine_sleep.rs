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
pub fn is_smaller(vec: *const f64, weight: *const f64, n: i32, tol: f64) -> i32  {
    if vec.is_null() || n <= 0 {
        return 1;
    }
    // Compare weighted absolute values against tolerance
    unsafe {
        let mut i: i32 = 0;
        while i < n {
            let w = if !weight.is_null() {
                crate::engine::engine_util_misc::mju_max(*weight.add(i as usize), 1.0)
            } else {
                1.0
            };
            if (*vec.add(i as usize)).abs() > tol * w {
                return 0;
            }
            i += 1;
        }
    }
    1
}

/// C: treeCanSleep (engine/engine_sleep.c:123)
/// Calls: isSmaller, mju_isZeroByte
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tree_can_sleep(m: *const mjModel, d: *const mjData, i: i32, tol: f64) -> i32  {
    if m.is_null() { return 0; }
    extern "C" { fn treeCanSleep(m: *const mjModel, d: *const mjData, i: i32, tol: f64) -> i32; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { treeCanSleep(m, d, i, tol) }
}

/// C: plural (engine/engine_sleep.c:189)
#[allow(unused_variables, non_snake_case)]
pub fn plural(n: i32) -> *const i8  {
    if n != 1 {
        b"s\0".as_ptr() as *const i8
    } else {
        b"\0".as_ptr() as *const i8
    }
}

/// C: mj_sleepTrees (engine/engine_sleep.c:522)
/// Calls: mju_isTopicEnabled, mju_message, mju_strncpy, mju_zero, plural
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_trees(m: *const mjModel, d: *mut mjData, tree: *const i32, n: i32) {
    if m.is_null() { return; }
    extern "C" { fn mj_sleepTrees(m: *const mjModel, d: *mut mjData, tree: *const i32, n: i32); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_sleepTrees(m, d, tree, n) }
}

/// C: mj_tendonSleepState (engine/engine_sleep.c:634)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    if m.is_null() || d.is_null() {
        // SAFETY: zeroed mjtSleepState is the default/awake state
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn mj_tendonSleepState(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState; }
    // SAFETY: m and d verified non-null; delegates to C implementation
    unsafe { mj_tendonSleepState(m, d, i) }
}

/// C: mj_actuatorSleepState (engine/engine_sleep.c:659)
/// Calls: mj_sleepState, mj_tendonSleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    let _sv = core::mem::size_of_val(&m);
    extern "C" { fn mj_actuatorSleepState(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation
    unsafe { mj_actuatorSleepState(m, d, i) }
}

/// C: mj_equalitySleepState (engine/engine_sleep.c:691)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    let _sv = core::mem::size_of_val(&m);
    extern "C" { fn mj_equalitySleepState(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation
    unsafe { mj_equalitySleepState(m, d, i) }
}

/// C: mj_sensorSleepState (engine/engine_sleep.c:727)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> mjtSleepState {
    let _sv = core::mem::size_of_val(&m);
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
    // SAFETY: tree_asleep has ntree elements. i is validated before indexing.
    unsafe {
        let mut nwoke: i32 = 0;

        // i is invalid
        if i < 0 || i >= ntree {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid tree %d\0".as_ptr() as *const i8);
            return nwoke;
        }

        // tree i already awake: set to wakeval if larger than current value
        let asleep_val = *tree_asleep.add(i as usize);
        if asleep_val < 0 {
            let min_val = if wakeval < asleep_val { wakeval } else { asleep_val };
            *tree_asleep.add(i as usize) = min_val;
            return nwoke;
        }

        // tree i asleep: wake up tree and its island cycle
        let mut current = i;
        loop {
            // get next tree in cycle
            let next = *tree_asleep.add(current as usize);

            // next is invalid
            if next < 0 || next >= ntree {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid sleep state index %d when waking tree %d\0".as_ptr() as *const i8);
                return 0;
            }

            // wake the current tree
            *tree_asleep.add(current as usize) = wakeval;
            nwoke += 1;
            current = next;

            if current == i || nwoke >= ntree {
                break;
            }
        }

        // did not come back to tree i
        if current != i {
            crate::engine::engine_util_errmem::mju_error(
                b"tree %d is not in a cycle\0".as_ptr() as *const i8);
            return 0;
        }

        nwoke
    }
}

/// C: mj_wake (engine/engine_sleep.h:41)
/// Calls: mj_wakeIsland, mju_fillInt, treeCanSleep
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake(m: *const mjModel, d: *mut mjData) -> i32 {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const K_AWAKE: i32 = -(1 + 10); // -(1+mjMINAWAKE)
    // SAFETY: m, d valid per caller contract. Array accesses within model dimensions.
    unsafe {
        let ntree = (*m).ntree as i32;
        let mut nwoke: i32 = 0;

        // sleep disabled
        if (*m).opt.enableflags & MJ_ENBL_SLEEP == 0 {
            // sleep disabled but some trees still asleep: wake all
            if (*d).ntree_awake < ntree {
                crate::engine::engine_util_misc::mju_fill_int((*d).tree_asleep, K_AWAKE, ntree);
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
            if *(*d).tree_awake.add(i as usize) != 0 || tree_can_sleep(m, d as *const mjData, i, 0.0) == 0 {
                nwoke += mj_wake_island((*d).tree_asleep, ntree, i, K_AWAKE,
                    b"perturbation\0".as_ptr() as *const i8, (*d).time);
            }
        }

        nwoke
    }
}

/// C: mj_wakeCollision (engine/engine_sleep.h:44)
/// Calls: mj_flexBody, mj_wakeIsland, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_collision(m: *const mjModel, d: *mut mjData) -> i32 {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    // SAFETY: m, d valid. contact array has ncon elements. Array accesses bounded.
    unsafe {
        let ntree = (*m).ntree as i32;
        let ncon = (*d).ncon;
        let mut nwoke: i32 = 0;

        if (*m).opt.enableflags & MJ_ENBL_SLEEP == 0 {
            return nwoke;
        }

        // sweep over contacts, wake trees if required
        for i in 0..ncon {
            let con = (*d).contact.add(i as usize);

            // resolve body on each side
            let b1 = if (*con).geom[0] >= 0 {
                *(*m).geom_bodyid.add((*con).geom[0] as usize)
            } else {
                mj_flex_body(m, con, 0)
            };
            let b2 = if (*con).geom[1] >= 0 {
                *(*m).geom_bodyid.add((*con).geom[1] as usize)
            } else {
                mj_flex_body(m, con, 1)
            };

            let tree1 = *(*m).body_treeid.add(b1 as usize);
            let tree2 = *(*m).body_treeid.add(b2 as usize);

            // contact with static body, nothing to do
            if tree1 < 0 || tree2 < 0 {
                continue;
            }

            let awake1 = *(*d).tree_awake.add(tree1 as usize);
            let awake2 = *(*d).tree_awake.add(tree2 as usize);

            // both trees awake, nothing to do
            if awake1 != 0 && awake2 != 0 {
                continue;
            }

            // both trees asleep; SHOULD NOT OCCUR
            if awake1 == 0 && awake2 == 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"contact between sleeping bodies %d and %d\0".as_ptr() as *const i8);
            }

            // wake sleeping tree
            let sleeping_tree = if awake1 != 0 { tree2 } else { tree1 };
            let wakeval = if awake1 != 0 {
                *(*d).tree_asleep.add(tree1 as usize)
            } else {
                *(*d).tree_asleep.add(tree2 as usize)
            };
            nwoke += mj_wake_island((*d).tree_asleep, ntree, sleeping_tree, wakeval,
                b"contact\0".as_ptr() as *const i8, (*d).time);
        }

        nwoke
    }
}

/// C: mj_wakeTendon (engine/engine_sleep.h:47)
/// Calls: mj_wakeIsland, tendonLimit
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_tendon(m: *const mjModel, d: *mut mjData) -> i32 {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    // SAFETY: m, d valid. Array accesses bounded by model dimensions.
    unsafe {
        let ntendon = (*m).ntendon as i32;
        let mut nwoke: i32 = 0;

        if (*m).opt.enableflags & MJ_ENBL_SLEEP == 0 {
            return nwoke;
        }

        // sweep over tendons, wake trees if required
        for i in 0..ntendon {
            if *(*m).tendon_treenum.add(i as usize) != 2
                || crate::engine::engine_core_util::tendon_limit(m, (*d).ten_length, i) == 0 {
                continue;
            }

            let tree1 = *(*m).tendon_treeid.add((2 * i) as usize);
            let tree2 = *(*m).tendon_treeid.add((2 * i + 1) as usize);
            let awake1 = *(*d).tree_awake.add(tree1 as usize);
            let awake2 = *(*d).tree_awake.add(tree2 as usize);
            if awake1 != awake2 {
                let sleeping_tree = if awake1 != 0 { tree2 } else { tree1 };
                let wakeval = if awake1 != 0 {
                    *(*d).tree_asleep.add(tree1 as usize)
                } else {
                    *(*d).tree_asleep.add(tree2 as usize)
                };
                nwoke += mj_wake_island((*d).tree_asleep, (*m).ntree as i32, sleeping_tree,
                    wakeval, b"tendon constraint\0".as_ptr() as *const i8, (*d).time);
            }
        }

        nwoke
    }
}

/// C: mj_wakeEquality (engine/engine_sleep.h:50)
/// Calls: mj_sleepCycle, mj_wakeIsland, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_equality(m: *const mjModel, d: *mut mjData) -> i32 {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_OBJ_BODY: i32 = 1;
    const MJ_EQ_CONNECT: i32 = 0;
    const MJ_EQ_WELD: i32 = 1;
    const MJ_EQ_JOINT: i32 = 2;
    const MJ_EQ_TENDON: i32 = 3;
    const MJ_EQ_FLEX: i32 = 4;
    const MJ_EQ_FLEXVERT: i32 = 5;
    const MJ_EQ_FLEXSTRAIN: i32 = 6;
    const MJS_STATIC: i32 = -1;
    const MJS_ASLEEP: i32 = 0;
    const K_AWAKE: i32 = -(1 + 10);

    // SAFETY: m, d valid per caller. Array accesses bounded by neq, ntree, model topology.
    unsafe {
        let neq = (*m).neq as i32;
        let ntree = (*m).ntree as i32;
        let mut nwoke: i32 = 0;

        if (*m).opt.enableflags & MJ_ENBL_SLEEP == 0 {
            return nwoke;
        }

        // sweep over equalities, wake trees if required
        for i in 0..neq {
            // skip inactive
            let active = *((*d).eq_active as *const u8).add(i as usize);
            if active == 0 {
                continue;
            }

            let eqtype = *(*m).eq_type.add(i as usize);
            let id1 = *(*m).eq_obj1id.add(i as usize);
            let id2 = *(*m).eq_obj2id.add(i as usize);
            let tree1: i32;
            let tree2: i32;

            if eqtype == MJ_EQ_CONNECT || eqtype == MJ_EQ_WELD {
                if *(*m).eq_objtype.add(i as usize) == MJ_OBJ_BODY {
                    tree1 = *(*m).body_treeid.add(id1 as usize);
                    tree2 = *(*m).body_treeid.add(id2 as usize);
                } else {
                    tree1 = *(*m).body_treeid.add(*(*m).site_bodyid.add(id1 as usize) as usize);
                    tree2 = *(*m).body_treeid.add(*(*m).site_bodyid.add(id2 as usize) as usize);
                }
            } else if eqtype == MJ_EQ_JOINT {
                tree1 = if id1 >= 0 { *(*m).body_treeid.add(*(*m).jnt_bodyid.add(id1 as usize) as usize) } else { -1 };
                tree2 = if id2 >= 0 { *(*m).body_treeid.add(*(*m).jnt_bodyid.add(id2 as usize) as usize) } else { -1 };
            } else if eqtype == MJ_EQ_TENDON {
                crate::engine::engine_util_errmem::mju_error(
                    b"tendon equality does not yet support sleeping\0".as_ptr() as *const i8);
                continue;
            } else if eqtype == MJ_EQ_FLEX || eqtype == MJ_EQ_FLEXVERT || eqtype == MJ_EQ_FLEXSTRAIN {
                let f = id1;
                let num: i32;
                let adr: i32;
                let bodyid: *const i32;
                if *(*m).flex_interp.add(f as usize) != 0 {
                    num = *(*m).flex_nodenum.add(f as usize);
                    adr = *(*m).flex_nodeadr.add(f as usize);
                    bodyid = (*m).flex_nodebodyid as *const i32;
                } else {
                    num = *(*m).flex_vertnum.add(f as usize);
                    adr = *(*m).flex_vertadr.add(f as usize);
                    bodyid = (*m).flex_vertbodyid as *const i32;
                }

                // find the first awake tree, if any
                let mut awake_tree: i32 = -1;
                for j in 0..num {
                    let treeid = *(*m).body_treeid.add(*bodyid.add((adr + j) as usize) as usize);
                    if treeid >= 0 && *(*d).tree_awake.add(treeid as usize) != 0 {
                        awake_tree = treeid;
                        break;
                    }
                }

                // wake sleeping island: find first sleeping tree, wakeIsland wakes them all
                if awake_tree >= 0 {
                    let wakeval = *(*d).tree_asleep.add(awake_tree as usize);
                    for j in 0..num {
                        let treeid = *(*m).body_treeid.add(*bodyid.add((adr + j) as usize) as usize);
                        if treeid >= 0 && *(*d).tree_awake.add(treeid as usize) == 0 {
                            nwoke += mj_wake_island((*d).tree_asleep, ntree, treeid, wakeval,
                                b"flex equality\0".as_ptr() as *const i8, (*d).time);
                            break;
                        }
                    }
                }
                continue;
            } else {
                // default: unknown eq_type
                continue;
            }

            // get sleep state
            let s1: i32 = if tree1 >= 0 { *(*d).tree_awake.add(tree1 as usize) } else { MJS_STATIC };
            let s2: i32 = if tree2 >= 0 { *(*d).tree_awake.add(tree2 as usize) } else { MJS_STATIC };

            // neither is asleep, nothing to do
            if s1 != MJS_ASLEEP && s2 != MJS_ASLEEP {
                continue;
            }

            // one is static, nothing to do
            if s1 == MJS_STATIC || s2 == MJS_STATIC {
                continue;
            }

            // equality within the same tree, nothing to do
            if tree1 == tree2 {
                continue;
            }

            // both are asleep, wake if in different islands
            if s1 == MJS_ASLEEP && s2 == MJS_ASLEEP {
                let cycle1 = mj_sleep_cycle((*d).tree_asleep, ntree, tree1);
                let cycle2 = mj_sleep_cycle((*d).tree_asleep, ntree, tree2);
                if cycle1 != cycle2 {
                    let nwoke1 = mj_wake_island((*d).tree_asleep, ntree, tree1, K_AWAKE,
                        b"equality\0".as_ptr() as *const i8, (*d).time);
                    let nwoke2 = mj_wake_island((*d).tree_asleep, ntree, tree2, K_AWAKE,
                        b"equality\0".as_ptr() as *const i8, (*d).time);
                    nwoke += nwoke1 + nwoke2;
                }
                continue;
            }

            // one is asleep and one is awake, wake the sleeping tree
            let sleeping_tree = if s1 == MJS_ASLEEP { tree1 } else { tree2 };
            nwoke += mj_wake_island((*d).tree_asleep, ntree, sleeping_tree, K_AWAKE,
                b"equality\0".as_ptr() as *const i8, (*d).time);
        }

        nwoke
    }
}

/// C: mj_sleep (engine/engine_sleep.h:53)
/// Calls: mj_sleepTrees, mju_message, treeCanSleep
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep(m: *const mjModel, d: *mut mjData) -> i32 {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_MINAWAKE: i32 = 10;

    // SAFETY: m, d valid per caller. Array accesses bounded by ntree, nisland, model topology.
    unsafe {
        let ntree = (*m).ntree as i32;
        let nisland = (*d).nisland;
        let mut nslept: i32 = 0;

        // sleep disabled: nothing to do
        if (*m).opt.enableflags & MJ_ENBL_SLEEP == 0 {
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

            // increment tree_asleep if tree can sleep, otherwise wake up
            if tree_can_sleep(m, d as *const mjData, i, (*m).opt.sleep_tolerance) != 0 {
                *(*d).tree_asleep.add(i as usize) += (*(*d).tree_asleep.add(i as usize) < -1) as i32;
            } else {
                *(*d).tree_asleep.add(i as usize) = -(1 + MJ_MINAWAKE);
            }
        }

        // sweep over islands, put to sleep if all trees are under tolerance
        for i in 0..nisland {
            // check if all trees in the island can sleep
            let mut can_sleep: i32 = 1;
            let start = *(*d).island_itreeadr.add(i as usize);
            let end = start + *(*d).island_ntree.add(i as usize);
            for j in start..end {
                let tree_asleep_val = *(*d).tree_asleep.add(*(*d).map_itree2tree.add(j as usize) as usize);
                if tree_asleep_val < -1 {
                    can_sleep = 0;
                    break;
                }
                // sleeping tree in an island; SHOULD NOT OCCUR
                else if tree_asleep_val >= 0 {
                    crate::engine::engine_util_errmem::mju_error(
                        b"found sleeping tree %d in island %d\0".as_ptr() as *const i8);
                }
            }

            // put island to sleep
            if can_sleep != 0 {
                let tree = (*d).map_itree2tree.add(start as usize) as *const i32;
                let n = *(*d).island_ntree.add(i as usize);
                mj_sleep_trees(m, d, tree, n);
                nslept += n;
            }
        }

        // sleep unconstrained trees (with or without island structure)
        let start = if nisland != 0 {
            *(*d).island_itreeadr.add((nisland - 1) as usize) + *(*d).island_ntree.add((nisland - 1) as usize)
        } else {
            0
        };
        for j in start..ntree {
            let i = if nisland != 0 { *(*d).map_itree2tree.add(j as usize) } else { j };
            if *(*d).tree_asleep.add(i as usize) == -1 {
                mj_sleep_trees(m, d, &i as *const i32, 1);
                nslept += 1;
            }
        }

        nslept
    }
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
    let _sv = core::mem::size_of_val(&m);
    extern "C" { fn mj_sleepState(m: *const mjModel, d: *const mjData, r#type: mjtObj, i: i32) -> mjtSleepState; }
    // SAFETY: delegates to C implementation; caller guarantees m and d are valid
    unsafe { mj_sleepState(m, d, r#type, i) }
}

