//! Port of: engine/engine_io.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: getnsize (engine/engine_io.c:72)
#[allow(unused_variables, non_snake_case)]
pub fn getnsize() -> i32 {
    todo ! ()
}

/// C: getnptr (engine/engine_io.c:84)
#[allow(unused_variables, non_snake_case)]
pub fn getnptr() -> i32 {
    todo ! ()
}

/// C: bufwrite (engine/engine_io.c:96)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn bufwrite(src: *const (), num: i32, szbuf: usize, buf: *mut (), ptrbuf: *mut usize) {
    // WARNING: signature changed — verify body
    // Previous params: (src : * const (), num : i32, szbuf : usize, buf : * mut (), ptrbuf : * mut usize)
    // Previous return: ()
    todo ! ()
}

/// C: bufread (engine/engine_io.c:114)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn bufread(dest: *mut (), num: i32, szbuf: usize, buf: *const (), ptrbuf: *mut usize) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut (), num : i32, szbuf : usize, buf : * const (), ptrbuf : * mut usize)
    // Previous return: ()
    todo ! ()
}

/// C: SKIP (engine/engine_io.c:132)
#[allow(unused_variables, non_snake_case)]
pub fn skip(offset: isize) -> u32 {
    // WARNING: signature changed — verify body
    // Previous params: (offset : isize)
    // Previous return: u32
    todo ! ()
}

/// C: mj_setPtrModel (engine/engine_io.c:142)
/// Calls: SKIP, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_ptr_model(m: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: safeAddToBufferSize (engine/engine_io.c:173)
/// Calls: SKIP
#[allow(unused_variables, non_snake_case)]
pub fn safe_add_to_buffer_size(offset: *mut isize, nbuffer: *mut usize, type_size: usize, nr: usize, nc: usize) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (offset : * mut isize, nbuffer : * mut usize, type_size : usize, nr : usize, nc : usize)
    // Previous return: usize
    todo ! ()
}

/// C: freeModelBuffers (engine/engine_io.c:221)
/// Calls: mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_model_buffers(m: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: checkDBSparse (engine/engine_io.c:895)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn check_db_sparse(m: *const mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: copyM2Sparse (engine/engine_io.c:915)
/// Calls: mju_copyInt, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn copy_m2sparse(nv: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, rownnz: *const i32, rowadr: *const i32, src: *const i32, dst: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (nv : i32, dof_Madr : * const i32, dof_simplenum : * const i32, dof_parentid : * const i32, rownnz : * const i32, rowadr : * const i32, src : * const i32, dst : * mut i32, reduced : i32, upper : i32, remaining : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_setPtrData (engine/engine_io.c:989)
/// Calls: SKIP, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_ptr_data(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: freeDataBuffers (engine/engine_io.c:1036)
/// Calls: mjp_getPluginAtSlot, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_data_buffers(d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_copyDataVisual (engine/engine_io.c:1142)
/// Calls: mj_initPlugin, mj_makeRawData, mj_setPtrData, mjp_getPluginAtSlot, mju_free, mju_malloc, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data_visual(dest: *mut mjData, m: *const mjModel, src: *const mjData, flg_all: i32) -> *mut mjData {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut mjData, m : * const mjModel, src : * const mjData, flg_all : i32)
    // Previous return: * mut mjData
    todo ! ()
}

/// C: _resetData (engine/engine_io.c:1286)
/// Calls: checkDBSparse, mj_camlight, mj_clearEfc, mj_comPos, mj_deleteData, mj_forward, mj_id2name, mj_kinematics, mj_sleep, mj_tendon, mj_updateSleep, mj_updateSleepInit, mjp_getPluginAtSlot, mju_copy, mju_copy3, mju_copy4, mju_fillInt, mju_free, mju_malloc, mju_message, mju_zero, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn reset_data(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, debug_value : u8)
    // Previous return: ()
    todo ! ()
}

/// C: mj_logTimingDiagnostics (engine/engine_io.c:1570)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_log_timing_diagnostics(d: *const mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * const mjData)
    // Previous return: ()
    todo ! ()
}

/// C: sensorSize (engine/engine_io.c:1685)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_size(sensor_type: u32, sensor_dim: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (sensor_type : u32, sensor_dim : i32)
    // Previous return: i32
    todo ! ()
}

/// C: numObjects (engine/engine_io.c:1759)
#[allow(unused_variables, non_snake_case)]
pub fn num_objects(m: *const mjModel, objtype: u32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, objtype : u32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_makeModel (engine/engine_io.h:50)
/// Calls: freeModelBuffers, mj_defaultOption, mj_defaultStatistic, mj_defaultVisual, mj_setPtrModel, mju_free, mju_malloc, mju_message, mju_warning, safeAddToBufferSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_model(dest: *mut *mut mjModel, nq: usize, nv: usize, nu: usize, na: usize, nbody: usize, nbvh: usize, nbvhstatic: usize, nbvhdynamic: usize, noct: usize, njnt: usize, ntree: usize, nM: usize, nB: usize, nC: usize, nD: usize, ngeom: usize, nsite: usize, ncam: usize, nlight: usize, nflex: usize, nflexnode: usize, nflexvert: usize, nflexedge: usize, nflexelem: usize, nflexelemdata: usize, nflexstiffness: usize, nflexbending: usize, nflexelemedge: usize, nflexshelldata: usize, nflexevpair: usize, nflextexcoord: usize, nJfe: usize, nJfv: usize, nmesh: usize, nmeshvert: usize, nmeshnormal: usize, nmeshtexcoord: usize, nmeshface: usize, nmeshgraph: usize, nmeshpoly: usize, nmeshpolyvert: usize, nmeshpolymap: usize, nskin: usize, nskinvert: usize, nskintexvert: usize, nskinface: usize, nskinbone: usize, nskinbonevert: usize, nhfield: usize, nhfielddata: usize, ntex: usize, ntexdata: usize, nmat: usize, npair: usize, nexclude: usize, neq: usize, ntendon: usize, nJten: usize, nwrap: usize, nsensor: usize, nnumeric: usize, nnumericdata: usize, ntext: usize, ntextdata: usize, ntuple: usize, ntupledata: usize, nkey: usize, nmocap: usize, nplugin: usize, npluginattr: usize, nuser_body: usize, nuser_jnt: usize, nuser_geom: usize, nuser_site: usize, nuser_cam: usize, nuser_tendon: usize, nuser_actuator: usize, nuser_sensor: usize, nnames: usize, npaths: usize) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut * mut mjModel, nq : usize, nv : usize, nu : usize, na : usize, nbody : usize, nbvh : usize, nbvhstatic : usize, nbvhdynamic : usize, noct : usize, njnt : usize, ntree : usize, nM : usize, nB : usize, nC : usize, nD : usize, ngeom : usize, nsite : usize, ncam : usize, nlight : usize, nflex : usize, nflexnode : usize, nflexvert : usize, nflexedge : usize, nflexelem : usize, nflexelemdata : usize, nflexstiffness : usize, nflexbending : usize, nflexelemedge : usize, nflexshelldata : usize, nflexevpair : usize, nflextexcoord : usize, nJfe : usize, nJfv : usize, nmesh : usize, nmeshvert : usize, nmeshnormal : usize, nmeshtexcoord : usize, nmeshface : usize, nmeshgraph : usize, nmeshpoly : usize, nmeshpolyvert : usize, nmeshpolymap : usize, nskin : usize, nskinvert : usize, nskintexvert : usize, nskinface : usize, nskinbone : usize, nskinbonevert : usize, nhfield : usize, nhfielddata : usize, ntex : usize, ntexdata : usize, nmat : usize, npair : usize, nexclude : usize, neq : usize, ntendon : usize, nJten : usize, nwrap : usize, nsensor : usize, nnumeric : usize, nnumericdata : usize, ntext : usize, ntextdata : usize, ntuple : usize, ntupledata : usize, nkey : usize, nmocap : usize, nplugin : usize, npluginattr : usize, nuser_body : usize, nuser_jnt : usize, nuser_geom : usize, nuser_site : usize, nuser_cam : usize, nuser_tendon : usize, nuser_actuator : usize, nuser_sensor : usize, nnames : usize, npaths : usize)
    // Previous return: ()
    todo ! ()
}

/// C: mj_copyModel (engine/engine_io.h:69)
/// Calls: mj_deleteModel, mj_makeModel, mj_setPtrModel, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_model(dest: *mut mjModel, src: *const mjModel) -> *mut mjModel {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut mjModel, src : * const mjModel)
    // Previous return: * mut mjModel
    todo ! ()
}

/// C: mjv_copyModel (engine/engine_io.h:72)
/// Calls: mj_setPtrModel, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_model(dest: *mut mjModel, src: *const mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut mjModel, src : * const mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mj_saveModel (engine/engine_io.h:75)
/// Calls: bufwrite, getnptr, getnsize, mj_version, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_model(m: *const mjModel, filename: *const i8, buffer: *mut (), buffer_sz: i32) {
    extern "C" {
        fn mj_saveModel(m: *const mjModel, filename: *const i8, buffer: *mut (), buffer_sz: i32);
    }
    // SAFETY: caller guarantees m is valid, filename is a valid C string or null,
    // buffer (if non-null) has at least buffer_sz bytes available.
    unsafe { mj_saveModel(m, filename, buffer, buffer_sz) }
}

/// C: mj_loadModelBuffer (engine/engine_io.h:78)
/// Calls: bufread, getnptr, getnsize, mj_deleteModel, mj_makeModel, mj_validateReferences, mj_version, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_model_buffer(buffer: *const (), buffer_sz: i32) -> *mut mjModel {
    // WARNING: signature changed — verify body
    // Previous params: (buffer : * const (), buffer_sz : i32)
    // Previous return: * mut mjModel
    todo ! ()
}

/// C: mj_deleteModel (engine/engine_io.h:81)
/// Calls: freeModelBuffers, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_model(m: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * mut mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mj_sizeModel (engine/engine_io.h:84)
/// Calls: getnsize
#[allow(unused_variables, non_snake_case)]
pub fn mj_size_model(m: *const mjModel) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: usize
    todo ! ()
}

/// C: mj_validateReferences (engine/engine_io.h:87)
/// Calls: mjp_getPluginAtSlot, mju_message, numObjects, sensorSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_validate_references(m: *const mjModel) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: * const i8
    todo ! ()
}

/// C: mj_makeDofDofSparse (engine/engine_io.h:90)
/// Calls: mju_copyInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_sparse(nv: i32, nC: i32, nD: i32, nM: i32, dof_parentid: *const i32, dof_simplenum: *const i32, rownnz: *mut i32, rowadr: *mut i32, diag: *mut i32, colind: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (nv : i32, nC : i32, nD : i32, nM : i32, dof_parentid : * const i32, dof_simplenum : * const i32, rownnz : * mut i32, rowadr : * mut i32, diag : * mut i32, colind : * mut i32, reduced : i32, upper : i32, remaining : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_makeBSparse (engine/engine_io.h:96)
/// Calls: mju_insertionSortInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_b_sparse(nv: i32, nbody: i32, nB: i32, body_dofnum: *const i32, body_parentid: *const i32, body_dofadr: *const i32, B_rownnz: *mut i32, B_rowadr: *mut i32, B_colind: *mut i32, count: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (nv : i32, nbody : i32, nB : i32, body_dofnum : * const i32, body_parentid : * const i32, body_dofadr : * const i32, B_rownnz : * mut i32, B_rowadr : * mut i32, B_colind : * mut i32, count : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_makeDofDofMaps (engine/engine_io.h:102)
/// Calls: copyM2Sparse, mju_fillInt, mju_lower2SymMap, mju_message, mju_sparseMap
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_maps(nv: i32, nM: i32, nC: i32, nD: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, D_rownnz: *const i32, D_rowadr: *const i32, D_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, mapM2D: *mut i32, mapD2M: *mut i32, mapM2M: *mut i32, M: *mut i32, scratch: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (nv : i32, nM : i32, nC : i32, nD : i32, dof_Madr : * const i32, dof_simplenum : * const i32, dof_parentid : * const i32, D_rownnz : * const i32, D_rowadr : * const i32, D_colind : * const i32, M_rownnz : * const i32, M_rowadr : * const i32, M_colind : * const i32, mapM2D : * mut i32, mapD2M : * mut i32, mapM2M : * mut i32, M : * mut i32, scratch : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_makeData (engine/engine_io.h:113)
/// Calls: mj_initPlugin, mj_makeRawData, mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_data(m: *const mjModel) -> *mut mjData {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: * mut mjData
    todo ! ()
}

/// C: mj_makeRawData (engine/engine_io.h:116)
/// Calls: freeDataBuffers, mj_setPtrData, mju_free, mju_malloc, mju_message, mju_warning, safeAddToBufferSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_raw_data(dest: *mut *mut mjData, m: *const mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut * mut mjData, m : * const mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mj_copyData (engine/engine_io.h:120)
/// Calls: mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut mjData, m : * const mjModel, src : * const mjData)
    // Previous return: * mut mjData
    todo ! ()
}

/// C: mjv_copyData (engine/engine_io.h:123)
/// Calls: mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut mjData, m : * const mjModel, src : * const mjData)
    // Previous return: * mut mjData
    todo ! ()
}

/// C: mj_resetData (engine/engine_io.h:126)
/// Calls: _resetData, mj_logTimingDiagnostics
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_resetDataDebug (engine/engine_io.h:129)
/// Calls: _resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_debug(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, debug_value : u8)
    // Previous return: ()
    todo ! ()
}

/// C: mj_resetDataKeyframe (engine/engine_io.h:132)
/// Calls: _resetData, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_keyframe(m: *const mjModel, d: *mut mjData, key: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, key : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_initPlugin (engine/engine_io.h:135)
/// Calls: mjp_getPluginAtSlot, mju_free, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_plugin(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_deleteData (engine/engine_io.h:138)
/// Calls: freeDataBuffers, mju_free, mju_threadpool
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_data(d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

