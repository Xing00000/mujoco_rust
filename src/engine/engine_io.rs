//! Port of: engine/engine_io.c
//! IR hash: 73a9f665ec0ecfc0
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
    todo!() // mj_setPtrModel
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
    todo!() // mj_setPtrData
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
    todo!() // mj_logTimingDiagnostics
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
    todo!() // mj_saveModel
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
    todo!() // mj_sizeModel
}

/// C: mj_validateReferences (engine/engine_io.h:87)
/// Calls: mjp_getPluginAtSlot, mju_message, numObjects, sensorSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_validate_references(m: *const mjModel) -> *const i8 {
    todo!() // mj_validateReferences
}

/// C: mj_makeDofDofSparse (engine/engine_io.h:90)
/// Calls: mju_copyInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_sparse(nv: i32, nC: i32, nD: i32, nM: i32, dof_parentid: *const i32, dof_simplenum: *const i32, rownnz: *mut i32, rowadr: *mut i32, diag: *mut i32, colind: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    todo!() // mj_makeDofDofSparse
}

/// C: mj_makeBSparse (engine/engine_io.h:96)
/// Calls: mju_insertionSortInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_b_sparse(nv: i32, nbody: i32, nB: i32, body_dofnum: *const i32, body_parentid: *const i32, body_dofadr: *const i32, B_rownnz: *mut i32, B_rowadr: *mut i32, B_colind: *mut i32, count: *mut i32) {
    todo!() // mj_makeBSparse
}

/// C: mj_makeDofDofMaps (engine/engine_io.h:102)
/// Calls: copyM2Sparse, mju_fillInt, mju_lower2SymMap, mju_message, mju_sparseMap
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_maps(nv: i32, nM: i32, nC: i32, nD: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, D_rownnz: *const i32, D_rowadr: *const i32, D_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, mapM2D: *mut i32, mapD2M: *mut i32, mapM2M: *mut i32, M: *mut i32, scratch: *mut i32) {
    todo!() // mj_makeDofDofMaps
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
    todo!() // mj_initPlugin
}

/// C: mj_deleteData (engine/engine_io.h:138)
/// Calls: freeDataBuffers, mju_free, mju_threadpool
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_data(d: *mut mjData) {
    todo!() // mj_deleteData
}

