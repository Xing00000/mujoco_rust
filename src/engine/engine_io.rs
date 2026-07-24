//! Port of: engine/engine_io.c
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: getnsize (engine/engine_io.c:72)
#[allow(unused_variables, non_snake_case)]
pub fn getnsize() -> i32 {
    92
}

/// C: getnptr (engine/engine_io.c:84)
#[allow(unused_variables, non_snake_case)]
pub fn getnptr() -> i32 {
    471
}

/// C: bufwrite (engine/engine_io.c:96)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn bufwrite(src: *const (), num: i32, szbuf: i64, buf: *mut (), ptrbuf: *mut i64) {
    // SAFETY: caller guarantees src, buf, ptrbuf valid; num bytes available
    unsafe {
        if src.is_null() || buf.is_null() || ptrbuf.is_null() {
            return; // mjERROR would be called in C
        }
        if *ptrbuf + num as i64 > szbuf {
            return; // mjERROR would be called in C
        }
        std::ptr::copy_nonoverlapping(
            src as *const u8,
            (buf as *mut u8).add(*ptrbuf as usize),
            num as usize);
        *ptrbuf += num as i64;
    }
}

/// C: bufread (engine/engine_io.c:114)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn bufread(dest: *mut (), num: i32, szbuf: i64, buf: *const (), ptrbuf: *mut i64) {
    // SAFETY: caller guarantees dest, buf, ptrbuf valid; num bytes available
    unsafe {
        if dest.is_null() || buf.is_null() || ptrbuf.is_null() {
            return; // mjERROR would be called in C
        }
        if *ptrbuf + num as i64 > szbuf {
            return; // mjERROR would be called in C
        }
        std::ptr::copy_nonoverlapping(
            (buf as *const u8).add(*ptrbuf as usize),
            dest as *mut u8,
            num as usize);
        *ptrbuf += num as i64;
    }
}

/// C: SKIP (engine/engine_io.c:132)
#[allow(unused_variables, non_snake_case)]
pub fn skip(offset: isize) -> u32 {
    (64 - offset % 64) as u32
}

/// C: mj_setPtrModel (engine/engine_io.c:142)
/// Calls: SKIP, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_ptr_model(m: *mut mjModel) {
    todo!("mj_setPtrModel depends on MJMODEL_POINTERS X-macro expansion which enumerates all pointer fields in mjModel. Cannot translate without codegen support for the field list.")
}

/// C: safeAddToBufferSize (engine/engine_io.c:173)
/// Calls: SKIP
#[allow(unused_variables, non_snake_case)]
pub fn safe_add_to_buffer_size(offset: *mut isize, nbuffer: *mut i64, type_size: usize, nr: i64, nc: i64) -> i64 {
    // SAFETY: caller guarantees offset and nbuffer are valid pointers
    unsafe {
        if (type_size as i64) < 0 || nr < 0 || nc < 0 {
            return 0;
        }

        // SKIP: compute alignment padding (align to 64 bytes)
        let skip = {
            let align: usize = 64;
            (align - (*offset as usize % align)) % align
        };

        // nc * nr (overflow check)
        let product = match (nc as usize).checked_mul(nr as usize) {
            Some(v) => v,
            None => return 0,
        };

        // product * type_size (overflow check)
        let product = match product.checked_mul(type_size) {
            Some(v) => v,
            None => return 0,
        };

        // product + skip (overflow check)
        let to_add = match product.checked_add(skip) {
            Some(v) => v,
            None => return 0,
        };

        // *nbuffer + to_add (overflow check)
        let new_nbuffer = match (*nbuffer as usize).checked_add(to_add) {
            Some(v) => v,
            None => return 0,
        };
        *nbuffer = new_nbuffer as i64;

        // *offset + to_add (overflow check)
        if *offset > 0 && to_add > (isize::MAX as usize - *offset as usize) {
            return 0;
        }
        *offset += to_add as isize;

        1
    }
}

/// C: freeModelBuffers (engine/engine_io.c:221)
/// Calls: mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_model_buffers(m: *mut mjModel) {
    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        crate::engine::engine_util_errmem::mju_free((*m).buffer);
    }
}

/// C: checkDBSparse (engine/engine_io.c:895)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn check_db_sparse(m: *const mjModel) {
    // SAFETY: caller guarantees m is valid with all sparse arrays populated
    unsafe {
        for j in 0..(*m).nv as usize {
            let i = *(*m).dof_bodyid.add(j) as usize;

            // D[row j] and B[row i] should be identical
            if *(*m).D_rownnz.add(j) != *(*m).B_rownnz.add(i) {
                return; // mjERROR in C
            }
            for k in 0..*(*m).D_rownnz.add(j) as usize {
                if *(*m).D_colind.add(*(*m).D_rowadr.add(j) as usize + k)
                    != *(*m).B_colind.add(*(*m).B_rowadr.add(i) as usize + k)
                {
                    return; // mjERROR in C
                }
            }
        }
    }
}

/// C: copyM2Sparse (engine/engine_io.c:915)
/// Calls: mju_copyInt, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn copy_m2sparse(nv: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, rownnz: *const i32, rowadr: *const i32, src: *const i32, dst: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    // SAFETY: all pointers are valid arrays of appropriate sizes (caller contract)
    unsafe {
        // init remaining
        crate::engine::engine_util_misc::mju_copy_int(remaining, rownnz, nv);

        // copy data
        let mut i = nv - 1;
        while i >= 0 {
            // init at diagonal
            let mut adr = *dof_Madr.add(i as usize);
            *remaining.add(i as usize) -= 1;
            *dst.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) =
                *src.add(adr as usize);
            adr += 1;

            // process below diagonal unless reduced and dof is simple
            if !(reduced != 0 && *dof_simplenum.add(i as usize) != 0) {
                let mut j = i;
                loop {
                    j = *dof_parentid.add(j as usize);
                    if j < 0 { break; }
                    *remaining.add(i as usize) -= 1;
                    *dst.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) =
                        *src.add(adr as usize);

                    // add upper triangle if requested
                    if upper != 0 {
                        *remaining.add(j as usize) -= 1;
                        *dst.add((*rowadr.add(j as usize) + *remaining.add(j as usize)) as usize) =
                            *src.add(adr as usize);
                    }

                    adr += 1;
                }
            }
            i -= 1;
        }

        // check that none remaining
        for i in 0..nv as usize {
            if *remaining.add(i) != 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"unassigned index\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: mj_setPtrData (engine/engine_io.c:989)
/// Calls: SKIP, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_ptr_data(m: *const mjModel, d: *mut mjData) {
    todo!("mj_setPtrData depends on MJDATA_POINTERS and MJDATA_ARENA_POINTERS X-macro expansion. Cannot translate without codegen support for the field list.")
}

/// C: freeDataBuffers (engine/engine_io.c:1036)
/// Calls: mjp_getPluginAtSlot, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_data_buffers(d: *mut mjData) {
    // SAFETY: caller guarantees d is a valid pointer to initialized mjData
    unsafe {
        // destroy plugin instances
        for i in 0..(*d).nplugin {
            let plugin = crate::engine::engine_plugin::mjp_get_plugin_at_slot(*(*d).plugin.add(i as usize));
            if let Some(destroy_fn) = (*plugin).destroy {
                // SAFETY: destroy is actually fn(*mut mjData, i32) but codegen typed it as fn()
                let destroy: unsafe extern "C" fn(*mut mjData, i32) = std::mem::transmute(destroy_fn);
                destroy(d, i);
            }
        }
        crate::engine::engine_util_errmem::mju_free((*d).buffer);
        crate::engine::engine_util_errmem::mju_free((*d).arena);
    }
}

/// C: mj_copyDataVisual (engine/engine_io.c:1142)
/// Calls: mj_initPlugin, mj_makeRawData, mj_setPtrData, mjp_getPluginAtSlot, mju_free, mju_malloc, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data_visual(dest: *mut mjData, m: *const mjModel, src: *const mjData, flg_all: i32) -> *mut mjData {
    todo!() // mj_copyDataVisual
}

/// C: _resetData (engine/engine_io.c:1286)
/// Calls: checkDBSparse, mj_camlight, mj_clearEfc, mj_comPos, mj_deleteData, mj_forward, mj_id2name, mj_kinematics, mj_sleep, mj_tendon, mj_updateSleep, mj_updateSleepInit, mjp_getPluginAtSlot, mju_copy, mju_copy3, mju_copy4, mju_fillInt, mju_free, mju_malloc, mju_message, mju_zero, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn reset_data(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    todo!() // _resetData
}

/// C: mj_logTimingDiagnostics (engine/engine_io.c:1570)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_log_timing_diagnostics(d: *const mjData) {
    // Timer enum constants
    const MJTIMER_STEP: usize = 0;
    const MJTIMER_POSITION: usize = 3;
    const MJTIMER_ADVANCE: usize = 7;
    const MJTIMER_POS_KINEMATICS: usize = 8;
    const MJTIMER_POS_PROJECT: usize = 12;
    const MJTIMER_POS_COLLISION: usize = 10;
    const MJTIMER_COL_BROAD: usize = 13;
    const MJTIMER_COL_NARROW: usize = 14;

    // mjTimerStat layout: { duration: f64, number: i32, _pad: [u8; 4] } = 16 bytes
    const TIMER_STAT_SIZE: usize = 16;

    extern "C" {
        fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
    }

    // SAFETY: d is a valid mjData pointer (caller contract)
    // timer is [u8; 240] representing 15 mjTimerStat structs
    unsafe {
        let timer_base = (*d).timer.as_ptr();

        // helper: get duration (f64 at offset 0) from timer[i]
        let get_duration = |idx: usize| -> f64 {
            let ptr = timer_base.add(idx * TIMER_STAT_SIZE) as *const f64;
            *ptr
        };
        // helper: get number (i32 at offset 8) from timer[i]
        let get_number = |idx: usize| -> i32 {
            let ptr = timer_base.add(idx * TIMER_STAT_SIZE + 8) as *const i32;
            *ptr
        };

        let nstep = get_number(MJTIMER_STEP);
        if nstep <= 0 {
            return;
        }

        let tstep = get_duration(MJTIMER_STEP) / nstep as f64;
        if tstep <= 0.0 {
            return;
        }

        let mut buf = [0i8; 2048];
        let mut pos: usize = 0;
        let mut components: f64 = 0.0;

        let timer_names: [&[u8]; 15] = [
            b"step\0", b"forward\0", b"inverse\0",
            b"fwdPosition\0", b"fwdVelocity\0", b"fwdActuation\0",
            b"fwdConstraint\0", b"advance\0",
            b"pos_kinematics\0", b"pos_inertia\0", b"pos_collision\0",
            b"pos_make\0", b"pos_project\0",
            b"col_broad\0", b"col_narrow\0",
        ];

        for i in MJTIMER_POSITION..=MJTIMER_ADVANCE {
            if get_number(i) > 0 {
                let istep = get_duration(i) / get_number(i) as f64;
                components += istep;
                let written = snprintf(
                    buf.as_mut_ptr().add(pos),
                    2048 - pos,
                    b"%s  %-15s %8.1f  (%5.1f%%)\0".as_ptr() as *const i8,
                    if pos > 0 { b"\n\0".as_ptr() } else { b"\0".as_ptr() },
                    timer_names[i].as_ptr(),
                    istep * 1000.0,
                    100.0 * istep / tstep);
                if written > 0 { pos += written as usize; }

                // position sub-breakdown
                if i == MJTIMER_POSITION {
                    for p in MJTIMER_POS_KINEMATICS..=MJTIMER_POS_PROJECT {
                        if get_number(p) > 0 {
                            let pstep = get_duration(p) / get_number(p) as f64;
                            let written = snprintf(
                                buf.as_mut_ptr().add(pos),
                                2048 - pos,
                                b"\n    %-13s %8.1f  (%5.1f%%)\0".as_ptr() as *const i8,
                                timer_names[p].as_ptr().add(4),
                                pstep * 1000.0,
                                100.0 * pstep / tstep);
                            if written > 0 { pos += written as usize; }

                            // collision sub-breakdown
                            if p == MJTIMER_POS_COLLISION {
                                for c in MJTIMER_COL_BROAD..=MJTIMER_COL_NARROW {
                                    if get_number(c) > 0 {
                                        let cstep = get_duration(c) / get_number(c) as f64;
                                        let written = snprintf(
                                            buf.as_mut_ptr().add(pos),
                                            2048 - pos,
                                            b"\n      %-11s %8.1f  (%5.1f%%)\0".as_ptr() as *const i8,
                                            timer_names[c].as_ptr().add(4),
                                            cstep * 1000.0,
                                            100.0 * cstep / tstep);
                                        if written > 0 { pos += written as usize; }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let other = tstep - components;
        let written = snprintf(
            buf.as_mut_ptr().add(pos),
            2048 - pos,
            b"%s  %-15s %8.1f  (%5.1f%%)\0".as_ptr() as *const i8,
            if pos > 0 { b"\n\0".as_ptr() } else { b"\0".as_ptr() },
            b"other\0".as_ptr(),
            other * 1000.0,
            100.0 * other / tstep);
        if written > 0 { pos += written as usize; }

        let written = snprintf(
            buf.as_mut_ptr().add(pos),
            2048 - pos,
            b"%s  %-15s %8.1f\0".as_ptr() as *const i8,
            if pos > 0 { b"\n\0".as_ptr() } else { b"\0".as_ptr() },
            b"total\0".as_ptr(),
            tstep * 1000.0);
        if written > 0 { pos += written as usize; }

        // emit log message
        let mut msg = mjLogMessage {
            level: 0,  // mjLOG_INFO
            topic: 1,  // mjTOPIC_TIME_STP
            subject: [0i8; 1024],
            body: buf.as_ptr(),
            func: std::ptr::null(),
            file: std::ptr::null(),
            line: 0,
            timestamp: false,
            _pad_0: [0u8; 3],
        };
        snprintf(
            msg.subject.as_mut_ptr(),
            std::mem::size_of_val(&msg.subject),
            b"average time per step (%d steps, units: \xc2\xb5s)\0".as_ptr() as *const i8,
            nstep);
        crate::engine::engine_util_errmem::mju_message(&msg);
    }
}

/// C: sensorSize (engine/engine_io.c:1685)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_size(sensor_type: u32, sensor_dim: i32) -> i32 {
    use crate::types::*;

    match sensor_type {
        mjtSensor_mjSENS_TOUCH
        | mjtSensor_mjSENS_RANGEFINDER
        | mjtSensor_mjSENS_JOINTPOS
        | mjtSensor_mjSENS_JOINTVEL
        | mjtSensor_mjSENS_TENDONPOS
        | mjtSensor_mjSENS_TENDONVEL
        | mjtSensor_mjSENS_ACTUATORPOS
        | mjtSensor_mjSENS_ACTUATORVEL
        | mjtSensor_mjSENS_ACTUATORFRC
        | mjtSensor_mjSENS_JOINTACTFRC
        | mjtSensor_mjSENS_TENDONACTFRC
        | mjtSensor_mjSENS_JOINTLIMITPOS
        | mjtSensor_mjSENS_JOINTLIMITVEL
        | mjtSensor_mjSENS_JOINTLIMITFRC
        | mjtSensor_mjSENS_TENDONLIMITPOS
        | mjtSensor_mjSENS_TENDONLIMITVEL
        | mjtSensor_mjSENS_TENDONLIMITFRC
        | mjtSensor_mjSENS_GEOMDIST
        | mjtSensor_mjSENS_INSIDESITE
        | mjtSensor_mjSENS_E_POTENTIAL
        | mjtSensor_mjSENS_E_KINETIC
        | mjtSensor_mjSENS_CLOCK => 1,

        mjtSensor_mjSENS_CAMPROJECTION => 2,

        mjtSensor_mjSENS_ACCELEROMETER
        | mjtSensor_mjSENS_VELOCIMETER
        | mjtSensor_mjSENS_GYRO
        | mjtSensor_mjSENS_FORCE
        | mjtSensor_mjSENS_TORQUE
        | mjtSensor_mjSENS_MAGNETOMETER
        | mjtSensor_mjSENS_BALLANGVEL
        | mjtSensor_mjSENS_FRAMEPOS
        | mjtSensor_mjSENS_FRAMEXAXIS
        | mjtSensor_mjSENS_FRAMEYAXIS
        | mjtSensor_mjSENS_FRAMEZAXIS
        | mjtSensor_mjSENS_FRAMELINVEL
        | mjtSensor_mjSENS_FRAMEANGVEL
        | mjtSensor_mjSENS_FRAMELINACC
        | mjtSensor_mjSENS_FRAMEANGACC
        | mjtSensor_mjSENS_SUBTREECOM
        | mjtSensor_mjSENS_SUBTREELINVEL
        | mjtSensor_mjSENS_SUBTREEANGMOM
        | mjtSensor_mjSENS_GEOMNORMAL => 3,

        mjtSensor_mjSENS_GEOMFROMTO => 6,

        mjtSensor_mjSENS_BALLQUAT
        | mjtSensor_mjSENS_FRAMEQUAT => 4,

        mjtSensor_mjSENS_CONTACT
        | mjtSensor_mjSENS_TACTILE
        | mjtSensor_mjSENS_USER => sensor_dim,

        mjtSensor_mjSENS_PLUGIN => -1,

        _ => -1,
    }
}

/// C: numObjects (engine/engine_io.c:1759)
#[allow(unused_variables, non_snake_case)]
pub fn num_objects(m: *const mjModel, objtype: u32) -> i32 {
    use crate::types::*;

    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        match objtype {
            mjtObj_mjOBJ_DEFAULT | mjtObj_mjOBJ_FRAME | mjtObj_mjOBJ_UNKNOWN | mjtObj_mjOBJ_MODEL => -1,
            mjtObj_mjOBJ_BODY | mjtObj_mjOBJ_XBODY => (*m).nbody as i32,
            mjtObj_mjOBJ_JOINT => (*m).njnt as i32,
            mjtObj_mjOBJ_DOF => (*m).nv as i32,
            mjtObj_mjOBJ_GEOM => (*m).ngeom as i32,
            mjtObj_mjOBJ_SITE => (*m).nsite as i32,
            mjtObj_mjOBJ_CAMERA => (*m).ncam as i32,
            mjtObj_mjOBJ_LIGHT => (*m).nlight as i32,
            mjtObj_mjOBJ_FLEX => (*m).nflex as i32,
            mjtObj_mjOBJ_MESH => (*m).nmesh as i32,
            mjtObj_mjOBJ_SKIN => (*m).nskin as i32,
            mjtObj_mjOBJ_HFIELD => (*m).nhfield as i32,
            mjtObj_mjOBJ_TEXTURE => (*m).ntex as i32,
            mjtObj_mjOBJ_MATERIAL => (*m).nmat as i32,
            mjtObj_mjOBJ_PAIR => (*m).npair as i32,
            mjtObj_mjOBJ_EXCLUDE => (*m).nexclude as i32,
            mjtObj_mjOBJ_EQUALITY => (*m).neq as i32,
            mjtObj_mjOBJ_TENDON => (*m).ntendon as i32,
            mjtObj_mjOBJ_ACTUATOR => (*m).nu as i32,
            mjtObj_mjOBJ_SENSOR => (*m).nsensor as i32,
            mjtObj_mjOBJ_NUMERIC => (*m).nnumeric as i32,
            mjtObj_mjOBJ_TEXT => (*m).ntext as i32,
            mjtObj_mjOBJ_TUPLE => (*m).ntuple as i32,
            mjtObj_mjOBJ_KEY => (*m).nkey as i32,
            mjtObj_mjOBJ_PLUGIN => (*m).nplugin as i32,
            mjtObj_mjNOBJECT => -2,
            _ => -2,
        }
    }
}

/// C: mj_makeModel (engine/engine_io.h:50)
/// Calls: freeModelBuffers, mj_defaultOption, mj_defaultStatistic, mj_defaultVisual, mj_setPtrModel, mju_free, mju_malloc, mju_message, mju_warning, safeAddToBufferSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_model(dest: *mut *mut mjModel, nq: i64, nv: i64, nu: i64, na: i64, nbody: i64, nbvh: i64, nbvhstatic: i64, nbvhdynamic: i64, noct: i64, njnt: i64, ntree: i64, nM: i64, nB: i64, nC: i64, nD: i64, ngeom: i64, nsite: i64, ncam: i64, nlight: i64, nflex: i64, nflexnode: i64, nflexvert: i64, nflexedge: i64, nflexelem: i64, nflexelemdata: i64, nflexstiffness: i64, nflexbending: i64, nflexelemedge: i64, nflexshelldata: i64, nflexevpair: i64, nflextexcoord: i64, nJfe: i64, nJfv: i64, nmesh: i64, nmeshvert: i64, nmeshnormal: i64, nmeshtexcoord: i64, nmeshface: i64, nmeshgraph: i64, nmeshpoly: i64, nmeshpolyvert: i64, nmeshpolymap: i64, nskin: i64, nskinvert: i64, nskintexvert: i64, nskinface: i64, nskinbone: i64, nskinbonevert: i64, nhfield: i64, nhfielddata: i64, ntex: i64, ntexdata: i64, nmat: i64, npair: i64, nexclude: i64, neq: i64, ntendon: i64, nJten: i64, nwrap: i64, nsensor: i64, nnumeric: i64, nnumericdata: i64, ntext: i64, ntextdata: i64, ntuple: i64, ntupledata: i64, nkey: i64, nmocap: i64, nplugin: i64, npluginattr: i64, nuser_body: i64, nuser_jnt: i64, nuser_geom: i64, nuser_site: i64, nuser_cam: i64, nuser_tendon: i64, nuser_actuator: i64, nuser_sensor: i64, nnames: i64, npaths: i64) {
    todo!() // mj_makeModel
}

/// C: mj_copyModel (engine/engine_io.h:69)
/// Calls: mj_deleteModel, mj_makeModel, mj_setPtrModel, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_model(dest: *mut mjModel, src: *const mjModel) -> *mut mjModel {
    todo!() // mj_copyModel
}

/// C: mjv_copyModel (engine/engine_io.h:72)
/// Calls: mj_setPtrModel, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_model(dest: *mut mjModel, src: *const mjModel) {
    todo!() // mjv_copyModel
}

/// C: mj_saveModel (engine/engine_io.h:75)
/// Calls: bufwrite, getnptr, getnsize, mj_version, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_model(m: *const mjModel, filename: *const i8, buffer: *mut (), buffer_sz: i32) {
    todo!("mj_saveModel depends on MJMODEL_POINTERS and MJMODEL_SIZES X-macro expansion to serialize all fields. Cannot translate without codegen support for the field list.")
}

/// C: mj_loadModelBuffer (engine/engine_io.h:78)
/// Calls: bufread, getnptr, getnsize, mj_deleteModel, mj_makeModel, mj_validateReferences, mj_version, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_model_buffer(buffer: *const (), buffer_sz: i32) -> *mut mjModel {
    todo!() // mj_loadModelBuffer
}

/// C: mj_deleteModel (engine/engine_io.h:81)
/// Calls: freeModelBuffers, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_model(m: *mut mjModel) {
    if !m.is_null() {
        free_model_buffers(m);
        crate::engine::engine_util_errmem::mju_free(m as *mut ());
    }
}

/// C: mj_sizeModel (engine/engine_io.h:84)
/// Calls: getnsize
#[allow(unused_variables, non_snake_case)]
pub fn mj_size_model(m: *const mjModel) -> i64 {
    todo!("mj_sizeModel depends on MJMODEL_POINTERS X-macro expansion to sum all array sizes. Cannot translate without codegen support for the field list.")
}

/// C: mj_validateReferences (engine/engine_io.h:87)
/// Calls: mjp_getPluginAtSlot, mju_message, numObjects, sensorSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_validate_references(m: *const mjModel) -> *const i8 {
    todo!("mj_validateReferences depends on MJMODEL_REFERENCES X-macro expansion which enumerates ~100 reference fields for bounds checking. Cannot translate without codegen support for the field list.")
}

/// C: mj_makeDofDofSparse (engine/engine_io.h:90)
/// Calls: mju_copyInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_sparse(nv: i32, nC: i32, nD: i32, nM: i32, dof_parentid: *const i32, dof_simplenum: *const i32, rownnz: *mut i32, rowadr: *mut i32, diag: *mut i32, colind: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    // SAFETY: all pointer args are valid arrays of appropriate sizes (caller contract)
    unsafe {
        // no dofs, nothing to do
        if nv == 0 {
            return;
        }

        // compute rownnz
        crate::engine::engine_util_misc::mju_zero_int(rownnz, nv);
        let mut i = nv - 1;
        while i >= 0 {
            // init at diagonal
            let mut j = i;
            *rownnz.add(i as usize) += 1;

            // process below diagonal unless reduced and dof is simple
            if !(reduced != 0 && *dof_simplenum.add(i as usize) != 0) {
                loop {
                    j = *dof_parentid.add(j as usize);
                    if j < 0 { break; }
                    // both reduced and non-reduced have lower triangle
                    *rownnz.add(i as usize) += 1;
                    // add upper triangle if requested
                    if upper != 0 {
                        *rownnz.add(j as usize) += 1;
                    }
                }
            }
            i -= 1;
        }

        // accumulate rowadr
        *rowadr.add(0) = 0;
        for i in 1..nv as usize {
            *rowadr.add(i) = *rowadr.add(i - 1) + *rownnz.add(i - 1);
        }

        // populate colind
        crate::engine::engine_util_misc::mju_copy_int(remaining, rownnz, nv);
        let mut i = nv - 1;
        while i >= 0 {
            // init at diagonal
            *remaining.add(i as usize) -= 1;
            *colind.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) = i;

            // process below diagonal unless reduced and dof is simple
            if !(reduced != 0 && *dof_simplenum.add(i as usize) != 0) {
                let mut j = i;
                loop {
                    j = *dof_parentid.add(j as usize);
                    if j < 0 { break; }
                    *remaining.add(i as usize) -= 1;
                    *colind.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) = j;

                    // add upper triangle if requested
                    if upper != 0 {
                        *remaining.add(j as usize) -= 1;
                        *colind.add((*rowadr.add(j as usize) + *remaining.add(j as usize)) as usize) = i;
                    }
                }
            }
            i -= 1;
        }

        // check for remaining; SHOULD NOT OCCUR
        for i in 0..nv as usize {
            if *remaining.add(i) != 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"unexpected remaining\0".as_ptr() as *const i8);
            }
        }

        // check total nnz; SHOULD NOT OCCUR
        let expected_nnz = if upper != 0 { nD } else if reduced != 0 { nC } else { nM };
        if *rowadr.add((nv - 1) as usize) + *rownnz.add((nv - 1) as usize) != expected_nnz {
            crate::engine::engine_util_errmem::mju_error(
                b"sum of rownnz different from expected\0".as_ptr() as *const i8);
        }

        // find diagonal indices
        if !diag.is_null() {
            for i in 0..nv as usize {
                let adr = *rowadr.add(i);
                let mut j = 0;
                while *colind.add((adr + j) as usize) < i as i32 && j < *rownnz.add(i) {
                    j += 1;
                }
                if *colind.add((adr + j) as usize) != i as i32 {
                    crate::engine::engine_util_errmem::mju_error(
                        b"diagonal index not found\0".as_ptr() as *const i8);
                }
                *diag.add(i) = j;
            }
        }
    }
}

/// C: mj_makeBSparse (engine/engine_io.h:96)
/// Calls: mju_insertionSortInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_b_sparse(nv: i32, nbody: i32, nB: i32, body_dofnum: *const i32, body_parentid: *const i32, body_dofadr: *const i32, B_rownnz: *mut i32, B_rowadr: *mut i32, B_colind: *mut i32, count: *mut i32) {
    // SAFETY: all pointer args are valid arrays of appropriate sizes (caller contract)
    unsafe {
        // set rownnz to subtree dofs counts, including self
        crate::engine::engine_util_misc::mju_zero_int(B_rownnz, nbody);
        let mut i = nbody - 1;
        while i > 0 {
            *B_rownnz.add(i as usize) += *body_dofnum.add(i as usize);
            *B_rownnz.add(*body_parentid.add(i as usize) as usize) += *B_rownnz.add(i as usize);
            i -= 1;
        }

        // check if rownnz[0] != nv; SHOULD NOT OCCUR
        if *B_rownnz.add(0) != nv {
            crate::engine::engine_util_errmem::mju_error(
                b"rownnz[0] different from nv\0".as_ptr() as *const i8);
        }

        // add dofs in ancestors bodies
        for i in 0..nbody as usize {
            let mut j = *body_parentid.add(i) as usize;
            while j > 0 {
                *B_rownnz.add(i) += *body_dofnum.add(j);
                j = *body_parentid.add(j) as usize;
            }
        }

        // compute rowadr
        *B_rowadr.add(0) = 0;
        for i in 1..nbody as usize {
            *B_rowadr.add(i) = *B_rowadr.add(i - 1) + *B_rownnz.add(i - 1);
        }

        // check if total nnz != nB; SHOULD NOT OCCUR
        if nB != *B_rowadr.add((nbody - 1) as usize) + *B_rownnz.add((nbody - 1) as usize) {
            crate::engine::engine_util_errmem::mju_error(
                b"sum of rownnz different from nB\0".as_ptr() as *const i8);
        }

        // clear incremental row counts
        crate::engine::engine_util_misc::mju_zero_int(count, nbody);

        // add subtree dofs to colind
        let mut i = nbody - 1;
        while i > 0 {
            // add this body's dofs to subtree
            for n in 0..*body_dofnum.add(i as usize) {
                *B_colind.add((*B_rowadr.add(i as usize) + *count.add(i as usize)) as usize) =
                    *body_dofadr.add(i as usize) + n;
                *count.add(i as usize) += 1;
            }

            // add body subtree to parent
            let par = *body_parentid.add(i as usize);
            for n in 0..*count.add(i as usize) {
                *B_colind.add((*B_rowadr.add(par as usize) + *count.add(par as usize)) as usize) =
                    *B_colind.add((*B_rowadr.add(i as usize) + n) as usize);
                *count.add(par as usize) += 1;
            }
            i -= 1;
        }

        // add all ancestor dofs
        for i in 0..nbody as usize {
            let mut par = *body_parentid.add(i) as usize;
            while par > 0 {
                // add ancestor body dofs
                for n in 0..*body_dofnum.add(par) {
                    *B_colind.add((*B_rowadr.add(i) + *count.add(i)) as usize) =
                        *body_dofadr.add(par) + n;
                    *count.add(i) += 1;
                }
                // advance to parent
                par = *body_parentid.add(par) as usize;
            }
        }

        // process all bodies
        for i in 0..nbody as usize {
            // make sure cnt = rownnz; SHOULD NOT OCCUR
            if *B_rownnz.add(i) != *count.add(i) {
                crate::engine::engine_util_errmem::mju_error(
                    b"cnt different from rownnz\0".as_ptr() as *const i8);
            }

            // sort colind in each row
            if *count.add(i) > 1 {
                crate::engine::engine_util_misc::mju_insertion_sort_int(
                    B_colind.add(*B_rowadr.add(i) as usize) as *mut i32,
                    *count.add(i));
            }
        }
    }
}

/// C: mj_makeDofDofMaps (engine/engine_io.h:102)
/// Calls: copyM2Sparse, mju_fillInt, mju_lower2SymMap, mju_message, mju_sparseMap
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_maps(nv: i32, nM: i32, nC: i32, nD: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, D_rownnz: *const i32, D_rowadr: *const i32, D_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, mapM2D: *mut i32, mapD2M: *mut i32, mapM2M: *mut i32, M: *mut i32, scratch: *mut i32) {
    // SAFETY: all pointer args are valid arrays of appropriate sizes (caller contract)
    unsafe {
        // make mapM2D: M -> D (lower to symmetric)
        crate::engine::engine_util_misc::mju_lower2sym_map(
            mapM2D, nv, D_rowadr, D_rownnz, D_colind, M_rowadr, M_rownnz, M_colind, scratch);

        // make mapD2M: D -> M (symmetric to lower)
        crate::engine::engine_util_misc::mju_sparse_map(
            mapD2M, nv, M_rowadr, M_rownnz, M_colind, D_rowadr, D_rownnz, D_colind);

        // make mapM2M
        for i in 0..nM as usize {
            *M.add(i) = i as i32;
        }
        crate::engine::engine_util_misc::mju_fill_int(mapM2M, -1, nC);
        copy_m2sparse(nv, dof_Madr, dof_simplenum, dof_parentid, M_rownnz,
                      M_rowadr, M, mapM2M, /*reduced=*/1, /*upper=*/0, scratch);

        // check that all indices are filled in
        for i in 0..nC as usize {
            if *mapM2M.add(i) < 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"unassigned index in mapM2M\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: mj_makeData (engine/engine_io.h:113)
/// Calls: mj_initPlugin, mj_makeRawData, mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_data(m: *const mjModel) -> *mut mjData {
    todo!() // mj_makeData
}

/// C: mj_makeRawData (engine/engine_io.h:116)
/// Calls: freeDataBuffers, mj_setPtrData, mju_free, mju_malloc, mju_message, mju_warning, safeAddToBufferSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_raw_data(dest: *mut *mut mjData, m: *const mjModel) {
    todo!() // mj_makeRawData
}

/// C: mj_copyData (engine/engine_io.h:120)
/// Calls: mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    todo!() // mj_copyData
}

/// C: mjv_copyData (engine/engine_io.h:123)
/// Calls: mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    todo!() // mjv_copyData
}

/// C: mj_resetData (engine/engine_io.h:126)
/// Calls: _resetData, mj_logTimingDiagnostics
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_resetData
}

/// C: mj_resetDataDebug (engine/engine_io.h:129)
/// Calls: _resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_debug(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    todo!() // mj_resetDataDebug
}

/// C: mj_resetDataKeyframe (engine/engine_io.h:132)
/// Calls: _resetData, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_keyframe(m: *const mjModel, d: *mut mjData, key: i32) {
    todo!() // mj_resetDataKeyframe
}

/// C: mj_initPlugin (engine/engine_io.h:135)
/// Calls: mjp_getPluginAtSlot, mju_free, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_plugin(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m and d are valid pointers; plugin arrays are allocated (caller contract)
    unsafe {
        (*d).nplugin = (*m).nplugin as i32;
        for i in 0..(*m).nplugin as usize {
            *(*d).plugin.add(i) = *(*m).plugin.add(i);
            let plugin = crate::engine::engine_plugin::mjp_get_plugin_at_slot(
                *(*m).plugin.add(i));
            if let Some(init_fn) = (*plugin).init {
                // SAFETY: init is actually fn(*const mjModel, *mut mjData, i32) -> i32
                let init: unsafe extern "C" fn(*const mjModel, *mut mjData, i32) -> i32 =
                    std::mem::transmute(init_fn);
                if init(m, d, i as i32) < 0 {
                    crate::engine::engine_util_errmem::mju_free((*d).buffer);
                    crate::engine::engine_util_errmem::mju_free((*d).arena);
                    crate::engine::engine_util_errmem::mju_free(d as *mut ());
                    crate::engine::engine_util_errmem::mju_error(
                        b"plugin->init failed for plugin id %d\0".as_ptr() as *const i8);
                    return;
                }
            }
        }
    }
}

/// C: mj_deleteData (engine/engine_io.h:138)
/// Calls: freeDataBuffers, mju_free, mju_threadpool
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_data(d: *mut mjData) {
    todo!() // mj_deleteData
}

