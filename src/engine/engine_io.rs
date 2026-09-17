//! Port of: engine/engine_io.c
//! IR hash: fd5288ad7242f001
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
/// Calls: cxx:_mju_message
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
/// Calls: cxx:_mju_message
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
    todo!() // cxx-internal:engine_io.c.o:_SKIP
}

/// C: mj_setPtrModel (engine/engine_io.c:142)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_SKIP, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_ptr_model(m: *mut mjModel) {
    todo!() // cxx-internal:engine_io.c.o:_mj_setPtrModel
}

/// C: safeAddToBufferSize (engine/engine_io.c:173)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_SKIP
#[allow(unused_variables, non_snake_case)]
pub fn safe_add_to_buffer_size(offset: *mut isize, nbuffer: *mut i64, type_size: usize, nr: i64, nc: i64) -> i64 {
    todo!() // cxx-internal:engine_io.c.o:_safeAddToBufferSize
}

/// C: freeModelBuffers (engine/engine_io.c:221)
/// Calls: cxx:_mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_model_buffers(m: *mut mjModel) {
    todo!() // cxx-internal:engine_io.c.o:_freeModelBuffers
}

/// C: mj_makeModel (engine/engine_io.c:245)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_freeModelBuffers, cxx-internal:engine_io.c.o:_mj_setPtrModel, cxx-internal:engine_io.c.o:_safeAddToBufferSize, cxx:_mj_defaultOption, cxx:_mj_defaultStatistic, cxx:_mj_defaultVisual, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_message, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_model(dest: *mut *mut mjModel, nq: i64, nv: i64, nu: i64, nactuator: i64, nout: i64, na: i64, nbody: i64, nbvh: i64, nbvhstatic: i64, nbvhdynamic: i64, noct: i64, njnt: i64, ntree: i64, nM: i64, nB: i64, nC: i64, nD: i64, ngeom: i64, nsite: i64, ncam: i64, nlight: i64, nflex: i64, nflexnode: i64, nflexvert: i64, nflexedge: i64, nflexelem: i64, nflexelemdata: i64, nflexstiffness: i64, nflexbending: i64, nefm0dof: i64, nefm0L: i64, nflexelemedge: i64, nflexshelldata: i64, nflexevpair: i64, nflextexcoord: i64, nJfe: i64, nJfv: i64, nmesh: i64, nmeshvert: i64, nmeshnormal: i64, nmeshtexcoord: i64, nmeshface: i64, nmeshgraph: i64, nmeshpoly: i64, nmeshpolyvert: i64, nmeshpolymap: i64, nskin: i64, nskinvert: i64, nskintexvert: i64, nskinface: i64, nskinbone: i64, nskinbonevert: i64, nhfield: i64, nhfielddata: i64, ntex: i64, ntexdata: i64, nmat: i64, npair: i64, nexclude: i64, neq: i64, ntendon: i64, nJten: i64, nwrap: i64, nsensor: i64, nnumeric: i64, nnumericdata: i64, ntext: i64, ntextdata: i64, ntuple: i64, ntupledata: i64, nkey: i64, nmocap: i64, nplugin: i64, npluginattr: i64, nuser_body: i64, nuser_jnt: i64, nuser_geom: i64, nuser_site: i64, nuser_cam: i64, nuser_tendon: i64, nuser_actuator: i64, nuser_sensor: i64, nnames: i64, npaths: i64) {
    todo!() // cxx:_mj_makeModel
}

/// C: mj_copyModel (engine/engine_io.c:433)
/// Calls: cxx-internal:engine_io.c.o:_mj_setPtrModel, cxx:_mj_deleteModel, cxx:_mj_makeModel, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_model(dest: *mut mjModel, src: *const mjModel) -> *mut mjModel {
    todo!() // cxx:_mj_copyModel
}

/// C: mjv_copyModel (engine/engine_io.c:483)
/// Calls: cxx-internal:engine_io.c.o:_mj_setPtrModel, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_model(dest: *mut mjModel, src: *const mjModel) {
    todo!() // cxx:_mjv_copyModel
}

/// C: mj_saveModel (engine/engine_io.c:514)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_getnptr, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_getnsize, cxx-internal:engine_io.c.o:_bufwrite, cxx:_mj_sizeModel, cxx:_mj_version, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_warning, cxx:_mju_writeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_model(m: *const mjModel, filename: *const i8, buffer: *mut (), buffer_sz: i32) {
    todo!() // cxx:_mj_saveModel
}

/// C: mj_loadModelBuffer (engine/engine_io.c:559)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_getnptr, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_getnsize, cxx-internal:engine_io.c.o:_bufread, cxx:_mj_deleteModel, cxx:_mj_makeModel, cxx:_mj_validateReferences, cxx:_mj_version, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_model_buffer(buffer: *const (), buffer_sz: i32) -> *mut mjModel {
    todo!() // cxx:_mj_loadModelBuffer
}

/// C: mj_deleteModel (engine/engine_io.c:693)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_freeModelBuffers, cxx:_mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_model(m: *mut mjModel) {
    todo!() // cxx:_mj_deleteModel
}

/// C: mj_sizeModel (engine/engine_io.c:702)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_getnsize
#[allow(unused_variables, non_snake_case)]
pub fn mj_size_model(m: *const mjModel) -> i64 {
    todo!() // cxx:_mj_sizeModel
}

/// C: mj_makeDofDofSparse (engine/engine_io.c:728)
/// Calls: cxx:_mju_copyInt, cxx:_mju_message, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_sparse(nv: i32, nC: i32, nD: i32, nM: i32, dof_parentid: *const i32, dof_simplenum: *const i32, rownnz: *mut i32, rowadr: *mut i32, diag: *mut i32, colind: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    todo!() // cxx:_mj_makeDofDofSparse
}

/// C: mj_makeBSparse (engine/engine_io.c:815)
/// Calls: cxx:_mju_insertionSortInt, cxx:_mju_message, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_b_sparse(nv: i32, nbody: i32, nB: i32, body_dofnum: *const i32, body_parentid: *const i32, body_dofadr: *const i32, B_rownnz: *mut i32, B_rowadr: *mut i32, B_colind: *mut i32, count: *mut i32) {
    todo!() // cxx:_mj_makeBSparse
}

/// C: checkDBSparse (engine/engine_io.c:898)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn check_db_sparse(m: *const mjModel) {
    todo!() // cxx-internal:engine_io.c.o:_checkDBSparse
}

/// C: copyM2Sparse (engine/engine_io.c:918)
/// Calls: cxx:_mju_copyInt, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn copy_m2sparse(nv: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, rownnz: *const i32, rowadr: *const i32, src: *const i32, dst: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    todo!() // cxx-internal:engine_io.c.o:_copyM2Sparse
}

/// C: mj_makeDofDofMaps (engine/engine_io.c:967)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_copyM2Sparse, cxx:_mju_fillInt, cxx:_mju_lower2SymMap, cxx:_mju_message, cxx:_mju_sparseMap
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_maps(nv: i32, nM: i32, nC: i32, nD: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, D_rownnz: *const i32, D_rowadr: *const i32, D_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, mapM2D: *mut i32, mapD2M: *mut i32, mapM2M: *mut i32, M: *mut i32, scratch: *mut i32) {
    todo!() // cxx:_mj_makeDofDofMaps
}

/// C: mj_setPtrData (engine/engine_io.c:992)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_SKIP, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_ptr_data(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_io.c.o:_mj_setPtrData
}

/// C: mj_initPlugin (engine/engine_io.c:1023)
/// Calls: cxx:_mjp_getPluginAtSlot, cxx:_mju_free, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_plugin(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_initPlugin
}

/// C: freeDataBuffers (engine/engine_io.c:1039)
/// Calls: cxx:_mjp_getPluginAtSlot, cxx:_mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_data_buffers(d: *mut mjData) {
    todo!() // cxx-internal:engine_io.c.o:_freeDataBuffers
}

/// C: mj_makeRawData (engine/engine_io.c:1058)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_freeDataBuffers, cxx-internal:engine_io.c.o:_mj_setPtrData, cxx-internal:engine_io.c.o:_safeAddToBufferSize, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_message, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_raw_data(dest: *mut *mut mjData, m: *const mjModel) {
    todo!() // cxx:_mj_makeRawData
}

/// C: mj_makeData (engine/engine_io.c:1132)
/// Calls: cxx:_mj_initPlugin, cxx:_mj_makeRawData, cxx:_mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_data(m: *const mjModel) -> *mut mjData {
    todo!() // cxx:_mj_makeData
}

/// C: mj_copyDataVisual (engine/engine_io.c:1145)
/// Calls: cxx-internal:engine_io.c.o:_mj_setPtrData, cxx:_mj_initPlugin, cxx:_mj_makeRawData, cxx:_mjp_getPluginAtSlot, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data_visual(dest: *mut mjData, m: *const mjModel, src: *const mjData, flg_all: i32) -> *mut mjData {
    todo!() // cxx:_mj_copyDataVisual
}

/// C: mj_copyData (engine/engine_io.c:1278)
/// Calls: cxx:_mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    todo!() // cxx:_mj_copyData
}

/// C: mjv_copyData (engine/engine_io.c:1283)
/// Calls: cxx:_mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    todo!() // cxx:_mjv_copyData
}

/// C: _resetData (engine/engine_io.c:1289)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_checkDBSparse, cxx:_mj_camlight, cxx:_mj_comPos, cxx:_mj_deleteData, cxx:_mj_forward, cxx:_mj_id2name, cxx:_mj_kinematics, cxx:_mj_resetCtrl, cxx:_mj_sleep, cxx:_mj_tendon, cxx:_mj_updateSleep, cxx:_mj_updateSleepInit, cxx:_mjp_getPluginAtSlot, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_fillInt, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_message, cxx:_mju_zero, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn reset_data(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    todo!() // cxx-internal:engine_io.c.o:__resetData
}

/// C: mj_logTimingDiagnostics (engine/engine_io.c:1577)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_log_timing_diagnostics(d: *const mjData) {
    todo!() // cxx-internal:engine_io.c.o:_mj_logTimingDiagnostics
}

/// C: mj_resetCtrl (engine/engine_io.c:1646)
/// Calls: cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_ctrl(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_resetCtrl
}

/// C: mj_resetData (engine/engine_io.c:1657)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_mj_logTimingDiagnostics, cxx-internal:engine_io.c.o:__resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_resetData
}

/// C: mj_resetDataDebug (engine/engine_io.c:1667)
/// Calls: cxx-internal:engine_io.c.o:__resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_debug(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    todo!() // cxx:_mj_resetDataDebug
}

/// C: mj_resetDataKeyframe (engine/engine_io.c:1673)
/// Calls: cxx-internal:engine_io.c.o:__resetData, cxx:_mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_keyframe(m: *const mjModel, d: *mut mjData, key: i32) {
    todo!() // cxx:_mj_resetDataKeyframe
}

/// C: mj_deleteData (engine/engine_io.c:1690)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_io.c:_freeDataBuffers, cxx:_mju_free, cxx:_mju_threadpool
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_data(d: *mut mjData) {
    todo!() // cxx:_mj_deleteData
}

/// C: sensorSize (engine/engine_io.c:1703)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_size(sensor_type: u32, sensor_dim: i32) -> i32 {
    todo!() // cxx-internal:engine_io.c.o:_sensorSize
}

/// C: numObjects (engine/engine_io.c:1777)
#[allow(unused_variables, non_snake_case)]
pub fn num_objects(m: *const mjModel, objtype: u32) -> i32 {
    todo!() // cxx-internal:engine_io.c.o:_numObjects
}

/// C: mj_validateReferences (engine/engine_io.c:1841)
/// Calls: cxx-internal:engine_io.c.o:_numObjects, cxx-internal:engine_io.c.o:_sensorSize, cxx:_mjp_getPluginAtSlot, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_validate_references(m: *const mjModel) -> *const i8 {
    todo!() // cxx:_mj_validateReferences
}

