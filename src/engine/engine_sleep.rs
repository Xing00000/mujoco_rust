//! Port of: engine/engine_sleep.c
//! IR hash: 73a9f665ec0ecfc0
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
    extern "C" {
        fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    }
    // SAFETY: m, d, tree are valid pointers (caller contract)
    unsafe {
        for i in 0..n as usize {
            // create cycle
            let current = *tree.add(i);
            let next = if i == (n as usize - 1) { *tree.add(0) } else { *tree.add(i + 1) };
            if *(*d).tree_asleep.add(current as usize) == -1 {
                *(*d).tree_asleep.add(current as usize) = next;
            } else if *(*d).tree_asleep.add(current as usize) >= 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"trying to sleep tree %d which is already asleep\0".as_ptr() as *const i8);
            } else {
                crate::engine::engine_util_errmem::mju_error(
                    b"trying to sleep tree %d which is not ready to sleep\0".as_ptr() as *const i8);
            }

            // set tree velocity and acceleration to zero
            let adr = *(*m).tree_dofadr.add(current as usize);
            let num = *(*m).tree_dofnum.add(current as usize);
            crate::engine::engine_util_blas::mju_zero((*d).qvel.add(adr as usize), num);
            crate::engine::engine_util_blas::mju_zero((*d).qacc.add(adr as usize), num);
        }

        // debug tracing
        if crate::engine::engine_util_errmem::mju_is_topic_enabled(3) {
            // mjTOPIC_SLEEP = 3 (mjLOG_INFO topics are 1-indexed after compile)
            let mut buf = [0i8; 1024];
            let mut pos = snprintf(buf.as_mut_ptr(), 1024,
                b"t=%6.2g, slept tree%s \0".as_ptr() as *const i8,
                (*d).time, if n == 1 { b"\0".as_ptr() } else { b"s\0".as_ptr() } as *const i8);
            for i in 0..n as usize {
                pos += snprintf(buf.as_mut_ptr().add(pos as usize), (1024 - pos as usize),
                    b"%d%s\0".as_ptr() as *const i8,
                    *tree.add(i),
                    if i == (n as usize - 1) { b"\0".as_ptr() } else { b" \0".as_ptr() } as *const i8);
            }
            let mut msg = mjLogMessage {
                level: 0, // mjLOG_INFO/DEBUG
                topic: 3,
                subject: [0i8; 1024],
                body: std::ptr::null(),
                func: std::ptr::null(),
                file: std::ptr::null(),
                line: 0,
                timestamp: false,
                _pad_0: [0; 3],
            };
            crate::engine::engine_util_misc::mju_strncpy(msg.subject.as_mut_ptr(), buf.as_ptr(), 1024);
            crate::engine::engine_util_errmem::mju_message(&msg);
        }
    }
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
    const MJ_TRN_JOINT: i32 = 0;
    const MJ_TRN_JOINTINPARENT: i32 = 1;
    const MJ_TRN_SLIDERCRANK: i32 = 2;
    const MJ_TRN_TENDON: i32 = 3;
    const MJ_TRN_SITE: i32 = 4;
    const MJ_TRN_BODY: i32 = 5;
    const MJ_OBJ_JOINT: u32 = 3;
    const MJ_OBJ_SITE: u32 = 6;
    const MJ_OBJ_BODY: u32 = 1;
    const MJ_OBJ_TENDON: u32 = 18;
    const MJ_S_AWAKE: i32 = 1;
    const MJ_S_ASLEEP: i32 = 0;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let trnid = *(*m).actuator_trnid.add(i as usize * 2);
        let trntype = *(*m).actuator_trntype.add(i as usize);

        if trntype == MJ_TRN_JOINT || trntype == MJ_TRN_JOINTINPARENT {
            return mj_sleep_state(m, d, MJ_OBJ_JOINT, trnid);
        }
        if trntype == MJ_TRN_SLIDERCRANK {
            let s1 = mj_sleep_state(m, d, MJ_OBJ_SITE, trnid);
            let s2 = mj_sleep_state(m, d, MJ_OBJ_SITE, *(*m).actuator_trnid.add(i as usize * 2 + 1));
            return if s1 == MJ_S_AWAKE || s2 == MJ_S_AWAKE { MJ_S_AWAKE } else { MJ_S_ASLEEP };
        }
        if trntype == MJ_TRN_TENDON {
            return mj_tendon_sleep_state(m, d, trnid);
        }
        if trntype == MJ_TRN_SITE {
            return mj_sleep_state(m, d, MJ_OBJ_SITE, trnid);
        }
        if trntype == MJ_TRN_BODY {
            return mj_sleep_state(m, d, MJ_OBJ_BODY, trnid);
        }

        MJ_S_AWAKE
    }
}

/// C: mj_equalitySleepState (engine/engine_sleep.c:691)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    const MJ_EQ_CONNECT: i32 = 0;
    const MJ_EQ_WELD: i32 = 1;
    const MJ_EQ_JOINT: i32 = 2;
    const MJ_EQ_TENDON: i32 = 3;
    const MJ_EQ_FLEX: i32 = 4;
    const MJ_EQ_FLEXVERT: i32 = 5;
    const MJ_EQ_FLEXSTRAIN: i32 = 6;
    const MJ_OBJ_JOINT: u32 = 3;
    const MJ_OBJ_TENDON: u32 = 18;
    const MJ_OBJ_FLEX: u32 = 9;
    const MJ_S_AWAKE: i32 = 1;
    const MJ_S_ASLEEP: i32 = 0;
    const MJ_S_STATIC: i32 = -1;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let eqtype = *(*m).eq_type.add(i as usize);

        let objtype: u32 = if eqtype == MJ_EQ_CONNECT || eqtype == MJ_EQ_WELD {
            *(*m).eq_objtype.add(i as usize) as u32
        } else if eqtype == MJ_EQ_JOINT {
            MJ_OBJ_JOINT
        } else if eqtype == MJ_EQ_TENDON {
            MJ_OBJ_TENDON
        } else if eqtype == MJ_EQ_FLEX || eqtype == MJ_EQ_FLEXVERT || eqtype == MJ_EQ_FLEXSTRAIN {
            MJ_OBJ_FLEX
        } else {
            return MJ_S_AWAKE;
        };

        let id1 = *(*m).eq_obj1id.add(i as usize);
        let id2 = *(*m).eq_obj2id.add(i as usize);
        let s1 = if id1 >= 0 { mj_sleep_state(m, d, objtype, id1) } else { MJ_S_STATIC };
        let s2 = if id2 >= 0 { mj_sleep_state(m, d, objtype, id2) } else { MJ_S_STATIC };

        // return ASLEEP if both objects are asleep or static, AWAKE otherwise
        let neither_awake = s1 != MJ_S_AWAKE && s2 != MJ_S_AWAKE;
        if neither_awake { MJ_S_ASLEEP } else { MJ_S_AWAKE }
    }
}

/// C: mj_sensorSleepState (engine/engine_sleep.c:727)
/// Calls: mj_sleepState
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_sleep_state(m: *const mjModel, d: *const mjData, i: i32) -> i32 {
    const MJ_SENS_USER: i32 = 48;
    const MJ_SENS_PLUGIN: i32 = 47;
    const MJ_SENS_CONTACT: i32 = 44;
    const MJ_SENS_RANGEFINDER: i32 = 8;
    const MJ_OBJ_SITE: u32 = 6;
    const MJ_OBJ_UNKNOWN: u32 = 0;
    const MJ_S_AWAKE: i32 = 1;
    const MJ_S_ASLEEP: i32 = 0;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let sensor_type = *(*m).sensor_type.add(i as usize);
        let objtype = *(*m).sensor_objtype.add(i as usize) as u32;
        let reftype = *(*m).sensor_reftype.add(i as usize) as u32;

        // special handling for specific sensor types
        if sensor_type == MJ_SENS_USER || sensor_type == MJ_SENS_PLUGIN {
            return MJ_S_AWAKE;
        }
        if sensor_type == MJ_SENS_CONTACT {
            if objtype == MJ_OBJ_SITE || reftype == MJ_OBJ_SITE {
                return MJ_S_AWAKE;
            }
        }
        if sensor_type == MJ_SENS_RANGEFINDER {
            return MJ_S_AWAKE;
        }

        // get sleep state of the primary and reference objects
        let s_obj = mj_sleep_state(m, d, objtype, *(*m).sensor_objid.add(i as usize));
        let s_ref = mj_sleep_state(m, d, reftype, *(*m).sensor_refid.add(i as usize));

        // if both are UNKNOWN, return AWAKE
        if objtype == MJ_OBJ_UNKNOWN && reftype == MJ_OBJ_UNKNOWN {
            return MJ_S_AWAKE;
        }

        // if one is UNKNOWN, return the other's sleep state
        if objtype == MJ_OBJ_UNKNOWN {
            return if s_ref == MJ_S_ASLEEP { MJ_S_ASLEEP } else { MJ_S_AWAKE };
        }
        if reftype == MJ_OBJ_UNKNOWN {
            return if s_obj == MJ_S_ASLEEP { MJ_S_ASLEEP } else { MJ_S_AWAKE };
        }

        // if either object is awake, return AWAKE
        if s_obj == MJ_S_AWAKE || s_ref == MJ_S_AWAKE {
            return MJ_S_AWAKE;
        }

        // otherwise return ASLEEP
        MJ_S_ASLEEP
    }
}

/// C: mj_updateSleepInit (engine/engine_sleep.h:28)
/// Calls: mjCMesh::tree
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_sleep_init(m: *const mjModel, d: *mut mjData, flg_staticawake: i32) {
    // SAFETY: m and d are valid pointers to mjModel and mjData provided by the caller.
    // All array accesses are within bounds guaranteed by the model structure.
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
            let awake = (*tree_asleep.add(i as usize) < 0) as i32;
            *tree_awake.add(i as usize) = awake;
            ntree_awake += awake;
        }
        (*d).ntree_awake = ntree_awake;

        // {body,parent}_awake_ind
        let mut nbody_awake: i32 = 0;
        let mut nparent_awake: i32 = 0;
        for i in 0..nbody {
            let tid = *body_treeid.add(i as usize);

            // static body
            if tid < 0 {
                if *body_mocapid.add(*body_rootid.add(i as usize) as usize) >= 0 {
                    // mocap body are always awake
                    *body_awake.add(i as usize) = mjtSleepState_mjS_AWAKE;
                } else {
                    // mark static body unless flg_staticawake is set
                    *body_awake.add(i as usize) = if flg_staticawake != 0 {
                        mjtSleepState_mjS_AWAKE
                    } else {
                        mjtSleepState_mjS_STATIC
                    };
                }
            }
            // dynamic body
            else {
                *body_awake.add(i as usize) = if *tree_awake.add(tid as usize) != 0 {
                    mjtSleepState_mjS_AWAKE
                } else {
                    mjtSleepState_mjS_ASLEEP
                };
            }

            // body_awake_ind: list of awake and static bodies
            if *body_awake.add(i as usize) != mjtSleepState_mjS_ASLEEP {
                *body_awake_ind.add(nbody_awake as usize) = i;
                nbody_awake += 1;
            }

            // parent_awake_ind: list of bodies with awake or static parents
            if i != 0 && *body_awake.add(*body_parentid.add(i as usize) as usize) != mjtSleepState_mjS_ASLEEP {
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
            if *body_treeid.add(bodyid as usize) >= 0
                && *body_awake.add(bodyid as usize) == mjtSleepState_mjS_AWAKE
            {
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
/// Calls: GlobalTable::count, next
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_cycle(tree_asleep: *const i32, ntree: i32, i: i32) -> i32 {
    // SAFETY: caller guarantees tree_asleep[ntree] is valid
    unsafe {
        if i < 0 || i >= ntree {
            return -1;
        }

        let mut smallest = i;
        let mut current = i;
        let mut count = 0;

        loop {
            if count > ntree {
                return -1;
            }

            let next = *tree_asleep.add(current as usize);

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
    // SAFETY: tree_asleep is a valid array of ntree elements (caller contract)
    unsafe {
        let mut nwoke: i32 = 0;

        // i is invalid; SHOULD NOT OCCUR
        if i < 0 || i >= ntree {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid tree %d\0".as_ptr() as *const i8);
            return nwoke;
        }

        // tree i already awake: set to wakeval if larger than current value
        let asleep_val = *tree_asleep.add(i as usize);
        if asleep_val < 0 {
            if wakeval < asleep_val {
                *tree_asleep.add(i as usize) = wakeval;
            }
            return nwoke;
        }

        // tree i asleep: wake up tree and its island cycle
        let mut current = i;
        loop {
            // get the index of the next tree in the cycle
            let next = *tree_asleep.add(current as usize);

            // next is invalid; SHOULD NOT OCCUR
            if next < 0 || next >= ntree {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid sleep state index %d when waking tree %d\0".as_ptr() as *const i8);
                return 0;
            }

            // wake the current tree, increment and advance to next
            *tree_asleep.add(current as usize) = wakeval;
            nwoke += 1;
            current = next;

            if current == i || nwoke >= ntree {
                break;
            }
        }

        // did not come back to tree i, not a cycle; SHOULD NOT OCCUR
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
    const MJENBL_SLEEP: i32 = 1 << 4;

    // SAFETY: m, d are valid model/data pointers (caller contract)
    unsafe {
        let ntree = (*m).ntree as i32;
        let ncon = (*d).ncon;
        let mut nwoke: i32 = 0;

        if (*m).opt.enableflags & MJENBL_SLEEP == 0 {
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
    const MJENBL_SLEEP: i32 = 1 << 4;

    // SAFETY: m, d are valid model/data pointers (caller contract)
    unsafe {
        let ntendon = (*m).ntendon as i32;
        let mut nwoke: i32 = 0;

        if (*m).opt.enableflags & MJENBL_SLEEP == 0 {
            return nwoke;
        }

        // sweep over tendons, wake trees if required
        for i in 0..ntendon {
            if *(*m).tendon_treenum.add(i as usize) != 2 ||
               crate::engine::engine_core_util::tendon_limit(m, (*d).ten_length, i) == 0 {
                continue;
            }

            let tree1 = *(*m).tendon_treeid.add(2 * i as usize);
            let tree2 = *(*m).tendon_treeid.add(2 * i as usize + 1);
            let awake1 = *(*d).tree_awake.add(tree1 as usize);
            let awake2 = *(*d).tree_awake.add(tree2 as usize);

            if awake1 != awake2 {
                let sleeping_tree = if awake1 != 0 { tree2 } else { tree1 };
                let wakeval = if awake1 != 0 {
                    *(*d).tree_asleep.add(tree1 as usize)
                } else {
                    *(*d).tree_asleep.add(tree2 as usize)
                };
                nwoke += mj_wake_island((*d).tree_asleep, (*m).ntree as i32, sleeping_tree, wakeval,
                    b"tendon constraint\0".as_ptr() as *const i8, (*d).time);
            }
        }

        nwoke
    }
}

/// C: mj_wakeEquality (engine/engine_sleep.h:50)
/// Calls: mj_sleepCycle, mj_wakeIsland, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_wake_equality(m: *const mjModel, d: *mut mjData) -> i32 {
    const MJENBL_SLEEP: i32 = 1 << 4;
    const K_AWAKE: i32 = -(1 + 10);  // -(1+mjMINAWAKE)
    // mjEQ: CONNECT=0, WELD=1, JOINT=2, TENDON=3, FLEX=4, FLEXVERT=5, FLEXSTRAIN=6
    // mjOBJ_BODY=1, mjOBJ_SITE=6
    // mjS_STATIC=-1, mjS_ASLEEP=0, mjS_AWAKE=1

    // SAFETY: m, d are valid model/data pointers (caller contract)
    unsafe {
        let neq = (*m).neq as i32;
        let mut nwoke: i32 = 0;

        if (*m).opt.enableflags & MJENBL_SLEEP == 0 {
            return nwoke;
        }

        // sweep over equalities
        for i in 0..neq {
            // skip inactive
            if !*(*d).eq_active.add(i as usize) {
                continue;
            }

            let eqtype = *(*m).eq_type.add(i as usize);
            let id1 = *(*m).eq_obj1id.add(i as usize);
            let id2 = *(*m).eq_obj2id.add(i as usize);
            let mut tree1: i32;
            let mut tree2: i32;

            match eqtype {
                0 | 1 => {  // mjEQ_CONNECT, mjEQ_WELD
                    if *(*m).eq_objtype.add(i as usize) == 6 {  // mjOBJ_SITE
                        tree1 = *(*m).body_treeid.add(*(*m).site_bodyid.add(id1 as usize) as usize);
                        tree2 = *(*m).body_treeid.add(*(*m).site_bodyid.add(id2 as usize) as usize);
                    } else {
                        tree1 = *(*m).body_treeid.add(id1 as usize);
                        tree2 = *(*m).body_treeid.add(id2 as usize);
                    }
                }
                2 => {  // mjEQ_JOINT
                    tree1 = if id1 >= 0 { *(*m).body_treeid.add(*(*m).jnt_bodyid.add(id1 as usize) as usize) } else { -1 };
                    tree2 = if id2 >= 0 { *(*m).body_treeid.add(*(*m).jnt_bodyid.add(id2 as usize) as usize) } else { -1 };
                }
                3 => {  // mjEQ_TENDON
                    crate::engine::engine_util_errmem::mju_error(
                        b"tendon equality does not yet support sleeping\0".as_ptr() as *const i8);
                    continue;
                }
                4 | 5 | 6 => {  // mjEQ_FLEX, FLEXVERT, FLEXSTRAIN
                    let f = id1;
                    let (num, adr, bodyid);
                    if *(*m).flex_interp.add(f as usize) != 0 {
                        num = *(*m).flex_nodenum.add(f as usize);
                        adr = *(*m).flex_nodeadr.add(f as usize);
                        bodyid = (*m).flex_nodebodyid;
                    } else {
                        num = *(*m).flex_vertnum.add(f as usize);
                        adr = *(*m).flex_vertadr.add(f as usize);
                        bodyid = (*m).flex_vertbodyid;
                    }

                    // find first awake tree
                    let mut awake_tree: i32 = -1;
                    for j in 0..num {
                        let treeid = *(*m).body_treeid.add(*bodyid.add((adr + j) as usize) as usize);
                        if treeid >= 0 && *(*d).tree_awake.add(treeid as usize) != 0 {
                            awake_tree = treeid;
                            break;
                        }
                    }

                    // wake sleeping island
                    if awake_tree >= 0 {
                        let wakeval = *(*d).tree_asleep.add(awake_tree as usize);
                        for j in 0..num {
                            let treeid = *(*m).body_treeid.add(*bodyid.add((adr + j) as usize) as usize);
                            if treeid >= 0 && *(*d).tree_awake.add(treeid as usize) == 0 {
                                nwoke += mj_wake_island((*d).tree_asleep, (*m).ntree as i32, treeid, wakeval,
                                    b"flex equality\0".as_ptr() as *const i8, (*d).time);
                                break;
                            }
                        }
                    }
                    continue;
                }
                _ => { continue; }
            }

            // get sleep state
            let s1 = if tree1 >= 0 { *(*d).tree_awake.add(tree1 as usize) as i32 } else { -1 };  // mjS_STATIC=-1
            let s2 = if tree2 >= 0 { *(*d).tree_awake.add(tree2 as usize) as i32 } else { -1 };

            // neither is asleep, nothing to do
            if s1 != 0 && s2 != 0 {
                continue;
            }

            // one is static, nothing to do
            if s1 == -1 || s2 == -1 {
                continue;
            }

            // equality within same tree, nothing to do
            if tree1 == tree2 {
                continue;
            }

            // both asleep, wake if in different islands
            if s1 == 0 && s2 == 0 {
                let cycle1 = mj_sleep_cycle((*d).tree_asleep, (*m).ntree as i32, tree1);
                let cycle2 = mj_sleep_cycle((*d).tree_asleep, (*m).ntree as i32, tree2);
                if cycle1 != cycle2 {
                    let nwoke1 = mj_wake_island((*d).tree_asleep, (*m).ntree as i32, tree1, K_AWAKE,
                        b"equality\0".as_ptr() as *const i8, (*d).time);
                    let nwoke2 = mj_wake_island((*d).tree_asleep, (*m).ntree as i32, tree2, K_AWAKE,
                        b"equality\0".as_ptr() as *const i8, (*d).time);
                    nwoke += nwoke1 + nwoke2;
                }
                continue;
            }

            // one asleep and one awake, wake sleeping tree
            let sleeping_tree = if s1 == 0 { tree1 } else { tree2 };
            nwoke += mj_wake_island((*d).tree_asleep, (*m).ntree as i32, sleeping_tree, K_AWAKE,
                b"equality\0".as_ptr() as *const i8, (*d).time);
        }

        nwoke
    }
}

/// C: mj_sleep (engine/engine_sleep.h:53)
/// Calls: mj_sleepTrees, mju_message, treeCanSleep
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
            if tree_can_sleep(m, d as *const mjData, i, (*m).opt.sleep_tolerance) != 0 {
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
                mj_sleep_trees(m, d, tree, n);
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
                mj_sleep_trees(m, d, &i as *const i32, 1);
                nslept += 1;
            }
        }

        nslept
    }
}

/// C: mj_flexBody (engine/engine_sleep.h:56)
/// Calls: mjCActuator::act
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_body(m: *const mjModel, con: *const mjContact, side: i32) -> i32 {
    // SAFETY: caller guarantees m, con are valid; side is 0 or 1
    unsafe {
        let f = (*con).flex[side as usize];

        // flex vertex contact (non-interpolated)
        if (*con).vert[side as usize] >= 0 && *(*m).flex_interp.add(f as usize) == 0 {
            return *(*m).flex_vertbodyid.add(
                (*(*m).flex_vertadr.add(f as usize) + (*con).vert[side as usize]) as usize
            );
        }

        // flex element contact
        if (*con).elem[side as usize] >= 0 {
            if *(*m).flex_interp.add(f as usize) == 0 {
                let dim = *(*m).flex_dim.add(f as usize);
                let edata = (*m).flex_elem.add(
                    (*(*m).flex_elemdataadr.add(f as usize)
                     + (*con).elem[side as usize] * (dim + 1)) as usize
                );
                return *(*m).flex_vertbodyid.add(
                    (*(*m).flex_vertadr.add(f as usize) + *edata) as usize
                );
            } else {
                return *(*m).flex_nodebodyid.add(*(*m).flex_nodeadr.add(f as usize) as usize);
            }
        }

        // flex vertex contact (interpolated): use first node
        *(*m).flex_nodebodyid.add(*(*m).flex_nodeadr.add(f as usize) as usize)
    }
}

/// C: mj_sleepState (engine/engine_sleep.h:59)
/// Calls: mj_actuatorSleepState, mj_equalitySleepState, mj_sensorSleepState, mj_tendonSleepState, mju_message, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_state(m: *const mjModel, d: *const mjData, r#type: u32, i: i32) -> i32 {
    const MJ_OBJ_UNKNOWN: u32 = 0;
    const MJ_OBJ_BODY: u32 = 1;
    const MJ_OBJ_XBODY: u32 = 2;
    const MJ_OBJ_JOINT: u32 = 3;
    const MJ_OBJ_DOF: u32 = 4;
    const MJ_OBJ_GEOM: u32 = 5;
    const MJ_OBJ_SITE: u32 = 6;
    const MJ_OBJ_CAMERA: u32 = 7;
    const MJ_OBJ_LIGHT: u32 = 8;
    const MJ_OBJ_FLEX: u32 = 9;
    const MJ_OBJ_EQUALITY: u32 = 17;
    const MJ_OBJ_TENDON: u32 = 18;
    const MJ_OBJ_ACTUATOR: u32 = 19;
    const MJ_OBJ_SENSOR: u32 = 20;
    const MJ_S_STATIC: i32 = -1;
    const MJ_S_AWAKE: i32 = 1;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        if r#type == MJ_OBJ_BODY || r#type == MJ_OBJ_XBODY {
            return *(*d).body_awake.add(i as usize);
        }
        if r#type == MJ_OBJ_JOINT {
            return *(*d).body_awake.add(*(*m).jnt_bodyid.add(i as usize) as usize);
        }
        if r#type == MJ_OBJ_SITE {
            return *(*d).body_awake.add(*(*m).site_bodyid.add(i as usize) as usize);
        }
        if r#type == MJ_OBJ_DOF {
            return *(*d).body_awake.add(*(*m).dof_bodyid.add(i as usize) as usize);
        }
        if r#type == MJ_OBJ_GEOM {
            return *(*d).body_awake.add(*(*m).geom_bodyid.add(i as usize) as usize);
        }
        if r#type == MJ_OBJ_CAMERA {
            return *(*d).body_awake.add(*(*m).cam_bodyid.add(i as usize) as usize);
        }
        if r#type == MJ_OBJ_LIGHT {
            return *(*d).body_awake.add(*(*m).light_bodyid.add(i as usize) as usize);
        }
        if r#type == MJ_OBJ_EQUALITY {
            return mj_equality_sleep_state(m, d, i);
        }
        if r#type == MJ_OBJ_TENDON {
            return mj_tendon_sleep_state(m, d, i);
        }
        if r#type == MJ_OBJ_ACTUATOR {
            return mj_actuator_sleep_state(m, d, i);
        }
        if r#type == MJ_OBJ_SENSOR {
            return mj_sensor_sleep_state(m, d, i);
        }
        if r#type == MJ_OBJ_FLEX {
            // all dynamic bodies share sleep state: find and check the first one
            let num: i32;
            let adr: i32;
            let bodyid: *const i32;
            if *(*m).flex_interp.add(i as usize) != 0 {
                num = *(*m).flex_nodenum.add(i as usize);
                adr = *(*m).flex_nodeadr.add(i as usize);
                bodyid = (*m).flex_nodebodyid;
            } else {
                num = *(*m).flex_vertnum.add(i as usize);
                adr = *(*m).flex_vertadr.add(i as usize);
                bodyid = (*m).flex_vertbodyid;
            }
            for j in 0..num {
                let b = *bodyid.add((adr + j) as usize);
                if *(*m).body_treeid.add(b as usize) >= 0 {
                    return *(*d).body_awake.add(b as usize);
                }
            }
            return MJ_S_STATIC;
        }
        if r#type == MJ_OBJ_UNKNOWN {
            return MJ_S_AWAKE;
        }

        // unsupported type
        crate::engine::engine_util_errmem::mju_error(
            b"unsupported object type in mj_sleepState\0".as_ptr() as *const i8);
        MJ_S_AWAKE
    }
}

