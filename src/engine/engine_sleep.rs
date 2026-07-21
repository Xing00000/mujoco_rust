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
    todo!() // mj_sleepState
}

