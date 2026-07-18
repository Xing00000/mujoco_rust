//! Port of: engine/engine_io.c
//! IR hash: d2209344472ae336
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
pub fn bufwrite(src: *const (), num: i32, szbuf: usize, buf: *mut (), ptrbuf: *mut usize) {
    extern "C" {
        fn mju_error(msg: *const i8);
    }

    // SAFETY: caller guarantees src, buf, ptrbuf are valid pointers when non-null
    unsafe {
        // check pointers
        if src.is_null() || buf.is_null() || ptrbuf.is_null() {
            mju_error(b"NULL pointer passed to bufwrite\0".as_ptr() as *const i8);
            return;
        }

        // check size
        if *ptrbuf + num as usize > szbuf {
            mju_error(b"attempting to write outside model buffer\0".as_ptr() as *const i8);
            return;
        }

        // write, advance pointer
        std::ptr::copy_nonoverlapping(
            src as *const u8,
            (buf as *mut u8).add(*ptrbuf),
            num as usize,
        );
        *ptrbuf += num as usize;
    }
}

/// C: bufread (engine/engine_io.c:114)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn bufread(dest: *mut (), num: i32, szbuf: usize, buf: *const (), ptrbuf: *mut usize) {
    todo!() // bufread
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
pub fn safe_add_to_buffer_size(offset: *mut isize, nbuffer: *mut usize, type_size: usize, nr: usize, nc: usize) -> usize {
    // SAFETY: offset and nbuffer are valid pointers (caller contract)
    unsafe {
        // nc * nr
        let product = match nc.checked_mul(nr) {
            Some(v) => v,
            None => return 0,
        };

        // product * type_size
        let product = match product.checked_mul(type_size) {
            Some(v) => v,
            None => return 0,
        };

        // product + SKIP(*offset)
        let skip_val = skip(*offset) as usize;
        let to_add = match product.checked_add(skip_val) {
            Some(v) => v,
            None => return 0,
        };

        // *nbuffer + to_add
        let new_nbuffer = match (*nbuffer).checked_add(to_add) {
            Some(v) => v,
            None => return 0,
        };
        *nbuffer = new_nbuffer;

        // *offset + to_add
        let new_offset = match (*offset).checked_add(to_add as isize) {
            Some(v) => v,
            None => return 0,
        };
        *offset = new_offset;

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
    todo!() // checkDBSparse
}

/// C: copyM2Sparse (engine/engine_io.c:915)
/// Calls: mju_copyInt, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn copy_m2sparse(nv: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, rownnz: *const i32, rowadr: *const i32, src: *const i32, dst: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    todo!() // copyM2Sparse
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
    todo!() // freeDataBuffers
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
pub fn mj_make_model(dest: *mut *mut mjModel, nq: usize, nv: usize, nu: usize, na: usize, nbody: usize, nbvh: usize, nbvhstatic: usize, nbvhdynamic: usize, noct: usize, njnt: usize, ntree: usize, nM: usize, nB: usize, nC: usize, nD: usize, ngeom: usize, nsite: usize, ncam: usize, nlight: usize, nflex: usize, nflexnode: usize, nflexvert: usize, nflexedge: usize, nflexelem: usize, nflexelemdata: usize, nflexstiffness: usize, nflexbending: usize, nflexelemedge: usize, nflexshelldata: usize, nflexevpair: usize, nflextexcoord: usize, nJfe: usize, nJfv: usize, nmesh: usize, nmeshvert: usize, nmeshnormal: usize, nmeshtexcoord: usize, nmeshface: usize, nmeshgraph: usize, nmeshpoly: usize, nmeshpolyvert: usize, nmeshpolymap: usize, nskin: usize, nskinvert: usize, nskintexvert: usize, nskinface: usize, nskinbone: usize, nskinbonevert: usize, nhfield: usize, nhfielddata: usize, ntex: usize, ntexdata: usize, nmat: usize, npair: usize, nexclude: usize, neq: usize, ntendon: usize, nJten: usize, nwrap: usize, nsensor: usize, nnumeric: usize, nnumericdata: usize, ntext: usize, ntextdata: usize, ntuple: usize, ntupledata: usize, nkey: usize, nmocap: usize, nplugin: usize, npluginattr: usize, nuser_body: usize, nuser_jnt: usize, nuser_geom: usize, nuser_site: usize, nuser_cam: usize, nuser_tendon: usize, nuser_actuator: usize, nuser_sensor: usize, nnames: usize, npaths: usize) {
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
pub fn mj_size_model(m: *const mjModel) -> usize {
    // SAFETY: m is a valid mjModel pointer (caller contract).
    // Direct expansion of MJMODEL_POINTERS X-macro from mjxmacro.h.
    unsafe {
        // MJMODEL_POINTERS_PREAMBLE(m) — used as nc multipliers
        let nuser_body: usize = (*m).nuser_body;
        let nuser_jnt: usize = (*m).nuser_jnt;
        let nuser_geom: usize = (*m).nuser_geom;
        let nuser_site: usize = (*m).nuser_site;
        let nuser_cam: usize = (*m).nuser_cam;
        let nuser_tendon: usize = (*m).nuser_tendon;
        let nuser_actuator: usize = (*m).nuser_actuator;
        let nuser_sensor: usize = (*m).nuser_sensor;
        let nq: usize = (*m).nq;
        let nv: usize = (*m).nv;
        let na: usize = (*m).na;
        let nu: usize = (*m).nu;
        let nmocap: usize = (*m).nmocap;

        // Base: sizeof(int)*NHEADER + sizeof(mjtSize)*getnsize() + option + visual + stat
        let mut size: usize = 4 * 5
            + 8 * (getnsize() as usize)
            + std::mem::size_of::<mjOption>()
            + std::mem::size_of::<mjVisual>()
            + std::mem::size_of::<mjStatistic>();

        // MJMODEL_POINTERS: size += sizeof(type) * m->nr * nc
        size += 8 * (*m).nq;
        size += 8 * (*m).nq;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 1 * (*m).nbody;
        size += 1 * (*m).nbody;
        size += 8 * (*m).nbody * 3;
        size += 8 * (*m).nbody * 4;
        size += 8 * (*m).nbody * 3;
        size += 8 * (*m).nbody * 4;
        size += 8 * (*m).nbody;
        size += 8 * (*m).nbody;
        size += 8 * (*m).nbody * 3;
        size += 8 * (*m).nbody * 2;
        size += 8 * (*m).nbody;
        size += 8 * (*m).nbody;
        size += 8 * (*m).nbody * nuser_body;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbvh;
        size += 4 * (*m).nbvh * 2;
        size += 4 * (*m).nbvh;
        size += 8 * (*m).nbvhstatic * 6;
        size += 4 * (*m).noct;
        size += 4 * (*m).noct * 8;
        size += 8 * (*m).noct * 6;
        size += 8 * (*m).noct * 8;
        size += 4 * (*m).njnt;
        size += 4 * (*m).njnt;
        size += 4 * (*m).njnt;
        size += 4 * (*m).njnt;
        size += 4 * (*m).njnt;
        size += 4 * (*m).njnt;
        size += 1 * (*m).njnt;
        size += 1 * (*m).njnt;
        size += 1 * (*m).njnt;
        size += 8 * (*m).njnt * 2;
        size += 8 * (*m).njnt * 5;
        size += 8 * (*m).njnt * 3;
        size += 8 * (*m).njnt * 3;
        size += 8 * (*m).njnt;
        size += 8 * (*m).njnt * 2;
        size += 8 * (*m).njnt * 2;
        size += 8 * (*m).njnt * 2;
        size += 8 * (*m).njnt;
        size += 8 * (*m).njnt * nuser_jnt;
        size += 4 * (*m).nv;
        size += 4 * (*m).nv;
        size += 4 * (*m).nv;
        size += 4 * (*m).nv;
        size += 4 * (*m).nv;
        size += 4 * (*m).nv;
        size += 8 * (*m).nv * 2;
        size += 8 * (*m).nv * 5;
        size += 8 * (*m).nv;
        size += 8 * (*m).nv;
        size += 8 * (*m).nv;
        size += 8 * (*m).nv * 2;
        size += 8 * (*m).nv;
        size += 8 * (*m).nv;
        size += 8 * (*m).nv;
        size += 4 * (*m).ntree;
        size += 4 * (*m).ntree;
        size += 4 * (*m).ntree;
        size += 4 * (*m).ntree;
        size += 4 * (*m).ntree;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).ngeom;
        size += 1 * (*m).ngeom;
        size += 8 * (*m).ngeom;
        size += 8 * (*m).ngeom * 2;
        size += 8 * (*m).ngeom * 5;
        size += 8 * (*m).ngeom * 3;
        size += 8 * (*m).ngeom * 6;
        size += 8 * (*m).ngeom;
        size += 8 * (*m).ngeom * 3;
        size += 8 * (*m).ngeom * 4;
        size += 8 * (*m).ngeom * 3;
        size += 8 * (*m).ngeom;
        size += 8 * (*m).ngeom;
        size += 8 * (*m).ngeom * 12;
        size += 8 * (*m).ngeom * nuser_geom;
        size += 4 * (*m).ngeom * 4;
        size += 4 * (*m).nsite;
        size += 4 * (*m).nsite;
        size += 4 * (*m).nsite;
        size += 4 * (*m).nsite;
        size += 1 * (*m).nsite;
        size += 8 * (*m).nsite * 3;
        size += 8 * (*m).nsite * 3;
        size += 8 * (*m).nsite * 4;
        size += 8 * (*m).nsite * nuser_site;
        size += 4 * (*m).nsite * 4;
        size += 4 * (*m).ncam;
        size += 4 * (*m).ncam;
        size += 4 * (*m).ncam;
        size += 8 * (*m).ncam * 3;
        size += 8 * (*m).ncam * 4;
        size += 8 * (*m).ncam * 3;
        size += 8 * (*m).ncam * 3;
        size += 8 * (*m).ncam * 9;
        size += 4 * (*m).ncam;
        size += 8 * (*m).ncam;
        size += 8 * (*m).ncam;
        size += 4 * (*m).ncam * 2;
        size += 4 * (*m).ncam;
        size += 4 * (*m).ncam * 2;
        size += 4 * (*m).ncam * 4;
        size += 8 * (*m).ncam * nuser_cam;
        size += 4 * (*m).nlight;
        size += 4 * (*m).nlight;
        size += 4 * (*m).nlight;
        size += 4 * (*m).nlight;
        size += 4 * (*m).nlight;
        size += 1 * (*m).nlight;
        size += 4 * (*m).nlight;
        size += 4 * (*m).nlight;
        size += 4 * (*m).nlight;
        size += 1 * (*m).nlight;
        size += 8 * (*m).nlight * 3;
        size += 8 * (*m).nlight * 3;
        size += 8 * (*m).nlight * 3;
        size += 8 * (*m).nlight * 3;
        size += 8 * (*m).nlight * 3;
        size += 4 * (*m).nlight * 3;
        size += 4 * (*m).nlight;
        size += 4 * (*m).nlight;
        size += 4 * (*m).nlight * 3;
        size += 4 * (*m).nlight * 3;
        size += 4 * (*m).nlight * 3;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 8 * (*m).nflex;
        size += 8 * (*m).nflex * 2;
        size += 8 * (*m).nflex * 5;
        size += 8 * (*m).nflex * 3;
        size += 8 * (*m).nflex;
        size += 8 * (*m).nflex;
        size += 1 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex * 3;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflexnode;
        size += 4 * (*m).nflexvert;
        size += 4 * (*m).nflexvert;
        size += 4 * (*m).nflexvert;
        size += 4 * (*m).nflexedge * 2;
        size += 4 * (*m).nflexedge * 2;
        size += 4 * (*m).nflexedge * 2;
        size += 4 * (*m).nflexelemdata;
        size += 4 * (*m).nflexelemdata;
        size += 4 * (*m).nflexelemedge;
        size += 4 * (*m).nflexelem;
        size += 4 * (*m).nflexshelldata;
        size += 4 * (*m).nflexevpair * 2;
        size += 8 * (*m).nflexvert * 3;
        size += 8 * (*m).nflexvert * 3;
        size += 8 * (*m).nflexvert * 4;
        size += 8 * (*m).nflexnode * 3;
        size += 8 * (*m).nflexnode * 3;
        size += 8 * (*m).nflexedge;
        size += 8 * (*m).nflexedge;
        size += 8 * (*m).nflex;
        size += 8 * (*m).nflex * 3;
        size += 8 * (*m).nflexstiffness;
        size += 8 * (*m).nflexbending;
        size += 8 * (*m).nflex;
        size += 8 * (*m).nflex;
        size += 8 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 1 * (*m).nflex;
        size += 1 * (*m).nflexedge;
        size += 1 * (*m).nflex;
        size += 1 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nflexedge;
        size += 4 * (*m).nflexedge;
        size += 4 * (*m).nJfe;
        size += 4 * (*m).nflexvert * 2;
        size += 4 * (*m).nflexvert * 2;
        size += 4 * (*m).nJfv * 2;
        size += 4 * (*m).nflex * 4;
        size += 4 * (*m).nflextexcoord * 2;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmeshvert * 3;
        size += 4 * (*m).nmeshnormal * 3;
        size += 4 * (*m).nmeshtexcoord * 2;
        size += 4 * (*m).nmeshface * 3;
        size += 4 * (*m).nmeshface * 3;
        size += 4 * (*m).nmeshface * 3;
        size += 4 * (*m).nmeshgraph;
        size += 8 * (*m).nmesh * 3;
        size += 8 * (*m).nmesh * 3;
        size += 8 * (*m).nmesh * 4;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nmesh;
        size += 8 * (*m).nmeshpoly * 3;
        size += 4 * (*m).nmeshpoly;
        size += 4 * (*m).nmeshpoly;
        size += 4 * (*m).nmeshpolyvert;
        size += 4 * (*m).nmeshvert;
        size += 4 * (*m).nmeshvert;
        size += 4 * (*m).nmeshpolymap;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskin * 4;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nskinvert * 3;
        size += 4 * (*m).nskintexvert * 2;
        size += 4 * (*m).nskinface * 3;
        size += 4 * (*m).nskinbone;
        size += 4 * (*m).nskinbone;
        size += 4 * (*m).nskinbone * 3;
        size += 4 * (*m).nskinbone * 4;
        size += 4 * (*m).nskinbone;
        size += 4 * (*m).nskinbonevert;
        size += 4 * (*m).nskinbonevert;
        size += 4 * (*m).nskin;
        size += 8 * (*m).nhfield * 4;
        size += 4 * (*m).nhfield;
        size += 4 * (*m).nhfield;
        size += 4 * (*m).nhfield;
        size += 4 * (*m).nhfielddata;
        size += 4 * (*m).nhfield;
        size += 4 * (*m).ntex;
        size += 4 * (*m).ntex;
        size += 4 * (*m).ntex;
        size += 4 * (*m).ntex;
        size += 4 * (*m).ntex;
        size += 8 * (*m).ntex;
        size += 1 * (*m).ntexdata;
        size += 4 * (*m).ntex;
        size += 4 * (*m).nmat * 10;
        size += 1 * (*m).nmat;
        size += 4 * (*m).nmat * 2;
        size += 4 * (*m).nmat;
        size += 4 * (*m).nmat;
        size += 4 * (*m).nmat;
        size += 4 * (*m).nmat;
        size += 4 * (*m).nmat;
        size += 4 * (*m).nmat;
        size += 4 * (*m).nmat * 4;
        size += 4 * (*m).npair;
        size += 4 * (*m).npair;
        size += 4 * (*m).npair;
        size += 4 * (*m).npair;
        size += 8 * (*m).npair * 2;
        size += 8 * (*m).npair * 2;
        size += 8 * (*m).npair * 5;
        size += 8 * (*m).npair;
        size += 8 * (*m).npair;
        size += 8 * (*m).npair * 5;
        size += 4 * (*m).nexclude;
        size += 4 * (*m).neq;
        size += 4 * (*m).neq;
        size += 4 * (*m).neq;
        size += 4 * (*m).neq;
        size += 1 * (*m).neq;
        size += 8 * (*m).neq * 2;
        size += 8 * (*m).neq * 5;
        size += 8 * (*m).neq * 11;
        size += 4 * (*m).ntendon;
        size += 4 * (*m).ntendon;
        size += 4 * (*m).ntendon;
        size += 4 * (*m).ntendon;
        size += 4 * (*m).ntendon;
        size += 4 * (*m).ntendon;
        size += 4 * (*m).ntendon * 2;
        size += 4 * (*m).ntendon;
        size += 4 * (*m).ntendon;
        size += 4 * (*m).nJten;
        size += 1 * (*m).ntendon;
        size += 1 * (*m).ntendon;
        size += 8 * (*m).ntendon;
        size += 8 * (*m).ntendon * 2;
        size += 8 * (*m).ntendon * 5;
        size += 8 * (*m).ntendon * 2;
        size += 8 * (*m).ntendon * 5;
        size += 8 * (*m).ntendon * 2;
        size += 8 * (*m).ntendon * 2;
        size += 8 * (*m).ntendon;
        size += 8 * (*m).ntendon;
        size += 8 * (*m).ntendon * 2;
        size += 8 * (*m).ntendon;
        size += 8 * (*m).ntendon * 2;
        size += 8 * (*m).ntendon;
        size += 8 * (*m).ntendon;
        size += 8 * (*m).ntendon * 2;
        size += 8 * (*m).ntendon;
        size += 8 * (*m).ntendon;
        size += 8 * (*m).ntendon * nuser_tendon;
        size += 4 * (*m).ntendon * 4;
        size += 4 * (*m).nwrap;
        size += 4 * (*m).nwrap;
        size += 8 * (*m).nwrap;
        size += 4 * (*m).nu;
        size += 4 * (*m).nu;
        size += 4 * (*m).nu;
        size += 4 * (*m).nu;
        size += 4 * (*m).nu * 2;
        size += 8 * (*m).nu;
        size += 8 * (*m).nu * 2;
        size += 8 * (*m).nu;
        size += 4 * (*m).nu;
        size += 4 * (*m).nu;
        size += 4 * (*m).nu;
        size += 4 * (*m).nu * 2;
        size += 4 * (*m).nu;
        size += 8 * (*m).nu;
        size += 1 * (*m).nu;
        size += 1 * (*m).nu;
        size += 1 * (*m).nu;
        size += 8 * (*m).nu * 10;
        size += 8 * (*m).nu * 10;
        size += 8 * (*m).nu * 10;
        size += 1 * (*m).nu;
        size += 8 * (*m).nu * 2;
        size += 8 * (*m).nu * 2;
        size += 8 * (*m).nu * 2;
        size += 8 * (*m).nu * 6;
        size += 8 * (*m).nu;
        size += 8 * (*m).nu;
        size += 8 * (*m).nu;
        size += 8 * (*m).nu * 2;
        size += 8 * (*m).nu * nuser_actuator;
        size += 4 * (*m).nu;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nsensor * 3;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nsensor;
        size += 8 * (*m).nsensor;
        size += 8 * (*m).nsensor;
        size += 4 * (*m).nsensor * 2;
        size += 4 * (*m).nsensor;
        size += 8 * (*m).nsensor;
        size += 8 * (*m).nsensor * 2;
        size += 8 * (*m).nsensor * nuser_sensor;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nplugin;
        size += 4 * (*m).nplugin;
        size += 4 * (*m).nplugin;
        size += 1 * (*m).npluginattr;
        size += 4 * (*m).nplugin;
        size += 4 * (*m).nnumeric;
        size += 4 * (*m).nnumeric;
        size += 8 * (*m).nnumericdata;
        size += 4 * (*m).ntext;
        size += 4 * (*m).ntext;
        size += 1 * (*m).ntextdata;
        size += 4 * (*m).ntuple;
        size += 4 * (*m).ntuple;
        size += 4 * (*m).ntupledata;
        size += 4 * (*m).ntupledata;
        size += 8 * (*m).ntupledata;
        size += 8 * (*m).nkey;
        size += 8 * (*m).nkey * nq;
        size += 8 * (*m).nkey * nv;
        size += 8 * (*m).nkey * na;
        size += 8 * (*m).nkey * nmocap * 3;
        size += 8 * (*m).nkey * nmocap * 4;
        size += 8 * (*m).nkey * nu;
        size += 4 * (*m).nbody;
        size += 4 * (*m).njnt;
        size += 4 * (*m).ngeom;
        size += 4 * (*m).nsite;
        size += 4 * (*m).ncam;
        size += 4 * (*m).nlight;
        size += 4 * (*m).nflex;
        size += 4 * (*m).nmesh;
        size += 4 * (*m).nskin;
        size += 4 * (*m).nhfield;
        size += 4 * (*m).ntex;
        size += 4 * (*m).nmat;
        size += 4 * (*m).npair;
        size += 4 * (*m).nexclude;
        size += 4 * (*m).neq;
        size += 4 * (*m).ntendon;
        size += 4 * (*m).nu;
        size += 4 * (*m).nsensor;
        size += 4 * (*m).nnumeric;
        size += 4 * (*m).ntext;
        size += 4 * (*m).ntuple;
        size += 4 * (*m).nkey;
        size += 4 * (*m).nplugin;
        size += 1 * (*m).nnames;
        size += 4 * (*m).nnames_map;
        size += 1 * (*m).npaths;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nbody;
        size += 4 * (*m).nB;
        size += 4 * (*m).nv;
        size += 4 * (*m).nv;
        size += 4 * (*m).nC;
        size += 4 * (*m).nC;
        size += 4 * (*m).nv;
        size += 4 * (*m).nv;
        size += 4 * (*m).nv;
        size += 4 * (*m).nD;
        size += 4 * (*m).nD;
        size += 4 * (*m).nC;

        size
    }
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

