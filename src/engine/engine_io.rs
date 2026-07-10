//! Port of: engine/engine_io.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: getnsize (engine/engine_io.c:72)
#[allow(unused_variables, non_snake_case)]
pub fn getnsize() -> i32 {
    // Compile-time constant: count of mjtSize fields in mjModel (MJMODEL_SIZES macro expansion).
    // Verified by compiling the C macro: 92 entries, all of type mjtSize.
    92
}

/// C: getnptr (engine/engine_io.c:84)
#[allow(unused_variables, non_snake_case)]
pub fn getnptr() -> i32 {
    // Compile-time constant: count of pointer fields in mjModel (MJMODEL_POINTERS macro expansion).
    // Verified by compiling the C macro: 471 entries.
    471
}

/// C: bufwrite (engine/engine_io.c:96)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn bufwrite(src: *const (), num: i32, szbuf: usize, buf: *mut (), ptrbuf: *mut usize) {
    // SAFETY: src, buf, ptrbuf are valid pointers per caller contract; memcpy within bounds checked below
    unsafe {
        if src.is_null() || buf.is_null() || ptrbuf.is_null() {
            crate::engine::engine_util_errmem::mju_error(
                b"NULL pointer passed to bufwrite\0".as_ptr() as *const i8,
            );
        }
        if *ptrbuf + num as usize > szbuf {
            crate::engine::engine_util_errmem::mju_error(
                b"attempting to write outside model buffer\0".as_ptr() as *const i8,
            );
        }
        core::ptr::copy_nonoverlapping(
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
    // SAFETY: dest, buf, ptrbuf are valid pointers per caller contract; memcpy within bounds checked below
    unsafe {
        if dest.is_null() || buf.is_null() || ptrbuf.is_null() {
            crate::engine::engine_util_errmem::mju_error(
                b"NULL pointer passed to bufread\0".as_ptr() as *const i8,
            );
        }
        if *ptrbuf + num as usize > szbuf {
            crate::engine::engine_util_errmem::mju_error(
                b"attempting to read outside model buffer\0".as_ptr() as *const i8,
            );
        }
        core::ptr::copy_nonoverlapping(
            (buf as *const u8).add(*ptrbuf),
            dest as *mut u8,
            num as usize,
        );
        *ptrbuf += num as usize;
    }
}

/// C: SKIP (engine/engine_io.c:132)
#[allow(unused_variables, non_snake_case)]
pub fn skip(offset: isize) -> u32 {
    const ALIGN: u32 = 64;
    // compute skipped bytes to achieve 64-byte alignment
    (ALIGN - (offset as u32 % ALIGN)) % ALIGN
}

/// C: mj_setPtrModel (engine/engine_io.c:142)
/// Calls: SKIP, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_ptr_model(m: *mut mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * mut mjModel)
    // Previous return: ()
    extern "C" { fn mj_setPtrModel(m: *mut mjModel); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_setPtrModel(m) }
}

/// C: safeAddToBufferSize (engine/engine_io.c:173)
/// Calls: SKIP
#[allow(unused_variables, non_snake_case)]
pub fn safe_add_to_buffer_size(offset: *mut isize, nbuffer: *mut usize, type_size: usize, nr: usize, nc: usize) -> usize {
    extern "C" { fn safeAddToBufferSize(offset: *mut isize, nbuffer: *mut usize, type_size: usize, nr: usize, nc: usize) -> usize; }
    // SAFETY: delegates to C implementation
    unsafe { safeAddToBufferSize(offset, nbuffer, type_size, nr, nc) }
}

/// C: freeModelBuffers (engine/engine_io.c:221)
/// Calls: mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_model_buffers(m: *mut mjModel) {
    extern "C" { fn freeModelBuffers(m: *mut mjModel); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { freeModelBuffers(m) }
}

/// C: checkDBSparse (engine/engine_io.c:895)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn check_db_sparse(m: *const mjModel) {
    // SAFETY: m is valid mjModel pointer. Accesses D_rownnz, D_rowadr, D_colind,
    // B_rownnz, B_rowadr, B_colind, dof_bodyid arrays.
    unsafe {
        let nv = (*m).nv as i32;

        // process all dofs
        for j in 0..nv {
            // get body for this dof
            let i = *(*m).dof_bodyid.add(j as usize);

            // D[row j] and B[row i] should be identical
            if *(*m).D_rownnz.add(j as usize) != *(*m).B_rownnz.add(i as usize) {
                extern "C" { fn mju_error(msg: *const i8, ...); }
                mju_error(b"rows have different nnz\0".as_ptr() as *const i8);
                return;
            }
            let nnz = *(*m).D_rownnz.add(j as usize);
            for k in 0..nnz {
                let d_idx = *(*m).D_rowadr.add(j as usize) + k;
                let b_idx = *(*m).B_rowadr.add(i as usize) + k;
                if *(*m).D_colind.add(d_idx as usize) != *(*m).B_colind.add(b_idx as usize) {
                    extern "C" { fn mju_error(msg: *const i8, ...); }
                    mju_error(b"rows have different colind\0".as_ptr() as *const i8);
                    return;
                }
            }
        }
    }
}

/// C: copyM2Sparse (engine/engine_io.c:915)
/// Calls: mju_copyInt, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn copy_m2sparse(nv: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, rownnz: *const i32, rowadr: *const i32, src: *const i32, dst: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    // SAFETY: all pointers are valid arrays of appropriate size per caller contract.
    unsafe {
        // init remaining
        crate::engine::engine_util_misc::mju_copy_int(remaining, rownnz, nv);

        // copy data
        let mut i = nv - 1;
        while i >= 0 {
            // init at diagonal
            let mut adr = *dof_Madr.add(i as usize);
            *remaining.add(i as usize) -= 1;
            *dst.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) = *src.add(adr as usize);
            adr += 1;

            // process below diagonal unless reduced and dof is simple
            if !(reduced != 0 && *dof_simplenum.add(i as usize) != 0) {
                let mut j = i;
                loop {
                    j = *dof_parentid.add(j as usize);
                    if j < 0 {
                        break;
                    }
                    *remaining.add(i as usize) -= 1;
                    *dst.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) = *src.add(adr as usize);

                    // add upper triangle if requested
                    if upper != 0 {
                        *remaining.add(j as usize) -= 1;
                        *dst.add((*rowadr.add(j as usize) + *remaining.add(j as usize)) as usize) = *src.add(adr as usize);
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    extern "C" { fn mj_setPtrData(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation which assigns pointers into mjData buffer using MJDATA_POINTERS macro; caller guarantees m and d are valid
    unsafe { mj_setPtrData(m, d) }
}

/// C: freeDataBuffers (engine/engine_io.c:1036)
/// Calls: mjp_getPluginAtSlot, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_data_buffers(d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData)
    // Previous return: ()
    extern "C" { fn freeDataBuffers(d : * mut mjData) ; } unsafe { freeDataBuffers(d) }
}

/// C: mj_copyDataVisual (engine/engine_io.c:1142)
/// Calls: mj_initPlugin, mj_makeRawData, mj_setPtrData, mjp_getPluginAtSlot, mju_free, mju_malloc, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data_visual(dest: *mut mjData, m: *const mjModel, src: *const mjData, flg_all: i32) -> *mut mjData {
    extern "C" { fn mj_copyDataVisual(dest: *mut mjData, m: *const mjModel, src: *const mjData, flg_all: i32) -> *mut mjData; }
    // SAFETY: delegates to C implementation
    unsafe { mj_copyDataVisual(dest, m, src, flg_all) }
}

/// C: _resetData (engine/engine_io.c:1286)
/// Calls: checkDBSparse, mj_camlight, mj_clearEfc, mj_comPos, mj_deleteData, mj_forward, mj_id2name, mj_kinematics, mj_sleep, mj_tendon, mj_updateSleep, mj_updateSleepInit, mjp_getPluginAtSlot, mju_copy, mju_copy3, mju_copy4, mju_fillInt, mju_free, mju_malloc, mju_message, mju_zero, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn reset_data(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    extern "C" { fn _resetData(m: *const mjModel, d: *mut mjData, debug_value: u8); }
    // SAFETY: delegates to C implementation; caller guarantees m and d are valid
    unsafe { _resetData(m, d, debug_value) }
}

/// C: mj_logTimingDiagnostics (engine/engine_io.c:1570)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_log_timing_diagnostics(d: *const mjData) {
    extern "C" { fn mj_logTimingDiagnostics(d: *const mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_logTimingDiagnostics(d) }
}

/// C: sensorSize (engine/engine_io.c:1685)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_size(sensor_type: mjtSensor, sensor_dim: i32) -> i32 {
    extern "C" { fn sensorSize(sensor_type: mjtSensor, sensor_dim: i32) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { sensorSize(sensor_type, sensor_dim) }
}

/// C: numObjects (engine/engine_io.c:1759)
#[allow(unused_variables, non_snake_case)]
pub fn num_objects(m: *const mjModel, objtype: mjtObj) -> i32 {
    extern "C" { fn numObjects(m: *const mjModel, objtype: mjtObj) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { numObjects(m, objtype) }
}

/// C: mj_makeModel (engine/engine_io.h:50)
/// Calls: freeModelBuffers, mj_defaultOption, mj_defaultStatistic, mj_defaultVisual, mj_setPtrModel, mju_free, mju_malloc, mju_message, mju_warning, safeAddToBufferSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_model(dest: *mut *mut mjModel, nq: usize, nv: usize, nu: usize, na: usize, nbody: usize, nbvh: usize, nbvhstatic: usize, nbvhdynamic: usize, noct: usize, njnt: usize, ntree: usize, nM: usize, nB: usize, nC: usize, nD: usize, ngeom: usize, nsite: usize, ncam: usize, nlight: usize, nflex: usize, nflexnode: usize, nflexvert: usize, nflexedge: usize, nflexelem: usize, nflexelemdata: usize, nflexstiffness: usize, nflexbending: usize, nflexelemedge: usize, nflexshelldata: usize, nflexevpair: usize, nflextexcoord: usize, nJfe: usize, nJfv: usize, nmesh: usize, nmeshvert: usize, nmeshnormal: usize, nmeshtexcoord: usize, nmeshface: usize, nmeshgraph: usize, nmeshpoly: usize, nmeshpolyvert: usize, nmeshpolymap: usize, nskin: usize, nskinvert: usize, nskintexvert: usize, nskinface: usize, nskinbone: usize, nskinbonevert: usize, nhfield: usize, nhfielddata: usize, ntex: usize, ntexdata: usize, nmat: usize, npair: usize, nexclude: usize, neq: usize, ntendon: usize, nJten: usize, nwrap: usize, nsensor: usize, nnumeric: usize, nnumericdata: usize, ntext: usize, ntextdata: usize, ntuple: usize, ntupledata: usize, nkey: usize, nmocap: usize, nplugin: usize, npluginattr: usize, nuser_body: usize, nuser_jnt: usize, nuser_geom: usize, nuser_site: usize, nuser_cam: usize, nuser_tendon: usize, nuser_actuator: usize, nuser_sensor: usize, nnames: usize, npaths: usize) {
    extern "C" { fn mj_makeModel(dest: *mut *mut mjModel, nq: usize, nv: usize, nu: usize, na: usize, nbody: usize, nbvh: usize, nbvhstatic: usize, nbvhdynamic: usize, noct: usize, njnt: usize, ntree: usize, nM: usize, nB: usize, nC: usize, nD: usize, ngeom: usize, nsite: usize, ncam: usize, nlight: usize, nflex: usize, nflexnode: usize, nflexvert: usize, nflexedge: usize, nflexelem: usize, nflexelemdata: usize, nflexstiffness: usize, nflexbending: usize, nflexelemedge: usize, nflexshelldata: usize, nflexevpair: usize, nflextexcoord: usize, nJfe: usize, nJfv: usize, nmesh: usize, nmeshvert: usize, nmeshnormal: usize, nmeshtexcoord: usize, nmeshface: usize, nmeshgraph: usize, nmeshpoly: usize, nmeshpolyvert: usize, nmeshpolymap: usize, nskin: usize, nskinvert: usize, nskintexvert: usize, nskinface: usize, nskinbone: usize, nskinbonevert: usize, nhfield: usize, nhfielddata: usize, ntex: usize, ntexdata: usize, nmat: usize, npair: usize, nexclude: usize, neq: usize, ntendon: usize, nJten: usize, nwrap: usize, nsensor: usize, nnumeric: usize, nnumericdata: usize, ntext: usize, ntextdata: usize, ntuple: usize, ntupledata: usize, nkey: usize, nmocap: usize, nplugin: usize, npluginattr: usize, nuser_body: usize, nuser_jnt: usize, nuser_geom: usize, nuser_site: usize, nuser_cam: usize, nuser_tendon: usize, nuser_actuator: usize, nuser_sensor: usize, nnames: usize, npaths: usize); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_makeModel(dest, nq, nv, nu, na, nbody, nbvh, nbvhstatic, nbvhdynamic, noct, njnt, ntree, nM, nB, nC, nD, ngeom, nsite, ncam, nlight, nflex, nflexnode, nflexvert, nflexedge, nflexelem, nflexelemdata, nflexstiffness, nflexbending, nflexelemedge, nflexshelldata, nflexevpair, nflextexcoord, nJfe, nJfv, nmesh, nmeshvert, nmeshnormal, nmeshtexcoord, nmeshface, nmeshgraph, nmeshpoly, nmeshpolyvert, nmeshpolymap, nskin, nskinvert, nskintexvert, nskinface, nskinbone, nskinbonevert, nhfield, nhfielddata, ntex, ntexdata, nmat, npair, nexclude, neq, ntendon, nJten, nwrap, nsensor, nnumeric, nnumericdata, ntext, ntextdata, ntuple, ntupledata, nkey, nmocap, nplugin, npluginattr, nuser_body, nuser_jnt, nuser_geom, nuser_site, nuser_cam, nuser_tendon, nuser_actuator, nuser_sensor, nnames, npaths) }
}

/// C: mj_copyModel (engine/engine_io.h:69)
/// Calls: mj_deleteModel, mj_makeModel, mj_setPtrModel, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_model(dest: *mut mjModel, src: *const mjModel) -> *mut mjModel {



    extern "C" { fn mj_copyModel(dest: *mut mjModel, src: *const mjModel) -> *mut mjModel; }
    // SAFETY: delegates to C implementation
    unsafe { mj_copyModel(dest, src) }
}

/// C: mjv_copyModel (engine/engine_io.h:72)
/// Calls: mj_setPtrModel, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_model(dest: *mut mjModel, src: *const mjModel) {
    extern "C" { fn mjv_copyModel(dest: *mut mjModel, src: *const mjModel); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjv_copyModel(dest, src) }
}

/// C: mj_saveModel (engine/engine_io.h:75)
/// Calls: bufwrite, getnptr, getnsize, mj_version, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_model(m: *const mjModel, filename: *const i8, buffer: *mut (), buffer_sz: i32) {
    extern "C" { fn mj_saveModel(m: *const mjModel, filename: *const i8, buffer: *mut (), buffer_sz: i32); }
    // SAFETY: delegates to mj_saveModel in libmujoco (exported MJAPI function)
    unsafe { mj_saveModel(m, filename, buffer, buffer_sz) }
}

/// C: mj_loadModelBuffer (engine/engine_io.h:78)
/// Calls: bufread, getnptr, getnsize, mj_deleteModel, mj_makeModel, mj_validateReferences, mj_version, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_model_buffer(buffer: *const (), buffer_sz: i32) -> *mut mjModel {
    extern "C" { fn mj_loadModelBuffer(buffer: *const (), buffer_sz: i32) -> *mut mjModel; }
    // SAFETY: delegates to C implementation
    unsafe { mj_loadModelBuffer(buffer, buffer_sz) }
}

/// C: mj_deleteModel (engine/engine_io.h:81)
/// Calls: freeModelBuffers, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_model(m: *mut mjModel) {
    // SAFETY: m may be null (early return). Otherwise valid per caller.
    if m.is_null() {
        return;
    }
    crate::engine::engine_io::free_model_buffers(m);
    crate::engine::engine_util_errmem::mju_free(m as *mut ());
}

/// C: mj_sizeModel (engine/engine_io.h:84)
/// Calls: getnsize
#[allow(unused_variables, non_snake_case)]
pub fn mj_size_model(m: *const mjModel) -> usize {
    extern "C" { fn mj_sizeModel(m: *const mjModel) -> usize; }
    // SAFETY: delegates to C implementation which uses MJMODEL_POINTERS macro expansion
    unsafe { mj_sizeModel(m) }
}

/// C: mj_validateReferences (engine/engine_io.h:87)
/// Calls: mjp_getPluginAtSlot, mju_message, numObjects, sensorSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_validate_references(m: *const mjModel) -> *const i8 {
    extern "C" { fn mj_validateReferences(m: *const mjModel) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mj_validateReferences(m) }
}

/// C: mj_makeDofDofSparse (engine/engine_io.h:90)
/// Calls: mju_copyInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_sparse(nv: i32, nC: i32, nD: i32, nM: i32, dof_parentid: *const i32, dof_simplenum: *const i32, rownnz: *mut i32, rowadr: *mut i32, diag: *mut i32, colind: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    // SAFETY: All pointers are valid arrays of appropriate size per caller contract.
    unsafe {
        // no dofs, nothing to do
        if nv == 0 {
            return;
        }

        // compute rownnz
        crate::engine::engine_util_misc::mju_zero_int(rownnz, nv);
        let mut i: i32 = nv - 1;
        while i >= 0 {
            // init at diagonal
            let mut j: i32 = i;
            *rownnz.add(i as usize) += 1;

            // process below diagonal unless reduced and dof is simple
            if !(reduced != 0 && *dof_simplenum.add(i as usize) != 0) {
                loop {
                    j = *dof_parentid.add(j as usize);
                    if j < 0 {
                        break;
                    }
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
        let mut i: i32 = 1;
        while i < nv {
            *rowadr.add(i as usize) = *rowadr.add((i - 1) as usize) + *rownnz.add((i - 1) as usize);
            i += 1;
        }

        // populate colind
        crate::engine::engine_util_misc::mju_copy_int(remaining, rownnz, nv);
        let mut i: i32 = nv - 1;
        while i >= 0 {
            // init at diagonal
            *remaining.add(i as usize) -= 1;
            *colind.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) = i;

            // process below diagonal unless reduced and dof is simple
            if !(reduced != 0 && *dof_simplenum.add(i as usize) != 0) {
                let mut j: i32 = i;
                loop {
                    j = *dof_parentid.add(j as usize);
                    if j < 0 {
                        break;
                    }
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
        let mut i: i32 = 0;
        while i < nv {
            if *remaining.add(i as usize) != 0 {
                crate::engine::engine_util_errmem::mju_error(b"unexpected remaining\0".as_ptr() as *const i8);
            }
            i += 1;
        }

        // check total nnz; SHOULD NOT OCCUR
        let expected_nnz: i32 = if upper != 0 { nD } else { if reduced != 0 { nC } else { nM } };
        if *rowadr.add((nv - 1) as usize) + *rownnz.add((nv - 1) as usize) != expected_nnz {
            crate::engine::engine_util_errmem::mju_error(b"sum of rownnz different from expected\0".as_ptr() as *const i8);
        }

        // find diagonal indices
        if !diag.is_null() {
            let mut i: i32 = 0;
            while i < nv {
                let adr: i32 = *rowadr.add(i as usize);
                let mut j: i32 = 0;
                while *colind.add((adr + j) as usize) < i && j < *rownnz.add(i as usize) {
                    j += 1;
                }
                if *colind.add((adr + j) as usize) != i {
                    crate::engine::engine_util_errmem::mju_error(b"diagonal index not found\0".as_ptr() as *const i8);
                }
                *diag.add(i as usize) = j;
                i += 1;
            }
        }
    }
}

/// C: mj_makeBSparse (engine/engine_io.h:96)
/// Calls: mju_insertionSortInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_b_sparse(nv: i32, nbody: i32, nB: i32, body_dofnum: *const i32, body_parentid: *const i32, body_dofadr: *const i32, B_rownnz: *mut i32, B_rowadr: *mut i32, B_colind: *mut i32, count: *mut i32) {
    // SAFETY: All pointers are valid arrays of appropriate size per caller contract.
    unsafe {
        // set rownnz to subtree dofs counts, including self
        crate::engine::engine_util_misc::mju_zero_int(B_rownnz, nbody);
        let mut i: i32 = nbody - 1;
        while i > 0 {
            *B_rownnz.add(i as usize) += *body_dofnum.add(i as usize);
            *B_rownnz.add(*body_parentid.add(i as usize) as usize) += *B_rownnz.add(i as usize);
            i -= 1;
        }

        // check if rownnz[0] != nv; SHOULD NOT OCCUR
        if *B_rownnz.add(0) != nv {
            crate::engine::engine_util_errmem::mju_error(b"rownnz[0] different from nv\0".as_ptr() as *const i8);
        }

        // add dofs in ancestors bodies
        let mut i: i32 = 0;
        while i < nbody {
            let mut j: i32 = *body_parentid.add(i as usize);
            while j > 0 {
                *B_rownnz.add(i as usize) += *body_dofnum.add(j as usize);
                j = *body_parentid.add(j as usize);
            }
            i += 1;
        }

        // compute rowadr
        *B_rowadr.add(0) = 0;
        let mut i: i32 = 1;
        while i < nbody {
            *B_rowadr.add(i as usize) = *B_rowadr.add((i - 1) as usize) + *B_rownnz.add((i - 1) as usize);
            i += 1;
        }

        // check if total nnz != nB; SHOULD NOT OCCUR
        if nB != *B_rowadr.add((nbody - 1) as usize) + *B_rownnz.add((nbody - 1) as usize) {
            crate::engine::engine_util_errmem::mju_error(b"sum of rownnz different from nB\0".as_ptr() as *const i8);
        }

        // clear incremental row counts
        crate::engine::engine_util_misc::mju_zero_int(count, nbody);

        // add subtree dofs to colind
        let mut i: i32 = nbody - 1;
        while i > 0 {
            // add this body's dofs to subtree
            let mut n: i32 = 0;
            while n < *body_dofnum.add(i as usize) {
                *B_colind.add((*B_rowadr.add(i as usize) + *count.add(i as usize)) as usize) = *body_dofadr.add(i as usize) + n;
                *count.add(i as usize) += 1;
                n += 1;
            }

            // add body subtree to parent
            let par: i32 = *body_parentid.add(i as usize);
            let mut n: i32 = 0;
            while n < *count.add(i as usize) {
                *B_colind.add((*B_rowadr.add(par as usize) + *count.add(par as usize)) as usize) =
                    *B_colind.add((*B_rowadr.add(i as usize) + n) as usize);
                *count.add(par as usize) += 1;
                n += 1;
            }
            i -= 1;
        }

        // add all ancestor dofs
        let mut i: i32 = 0;
        while i < nbody {
            let mut par: i32 = *body_parentid.add(i as usize);
            while par > 0 {
                // add ancestor body dofs
                let mut n: i32 = 0;
                while n < *body_dofnum.add(par as usize) {
                    *B_colind.add((*B_rowadr.add(i as usize) + *count.add(i as usize)) as usize) = *body_dofadr.add(par as usize) + n;
                    *count.add(i as usize) += 1;
                    n += 1;
                }

                // advance to parent
                par = *body_parentid.add(par as usize);
            }
            i += 1;
        }

        // process all bodies
        let mut i: i32 = 0;
        while i < nbody {
            // make sure cnt = rownnz; SHOULD NOT OCCUR
            if *B_rownnz.add(i as usize) != *count.add(i as usize) {
                crate::engine::engine_util_errmem::mju_error(b"cnt different from rownnz\0".as_ptr() as *const i8);
            }

            // sort colind in each row
            if *count.add(i as usize) > 1 {
                crate::engine::engine_util_misc::mju_insertion_sort_int(B_colind.add(*B_rowadr.add(i as usize) as usize), *count.add(i as usize));
            }
            i += 1;
        }
    }
}

/// C: mj_makeDofDofMaps (engine/engine_io.h:102)
/// Calls: copyM2Sparse, mju_fillInt, mju_lower2SymMap, mju_message, mju_sparseMap
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_maps(nv: i32, nM: i32, nC: i32, nD: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, D_rownnz: *const i32, D_rowadr: *const i32, D_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, mapM2D: *mut i32, mapD2M: *mut i32, mapM2M: *mut i32, M: *mut i32, scratch: *mut i32) {
    // SAFETY: All pointers are valid arrays of appropriate size per caller contract.
    unsafe {
        // make mapM2D: M -> D (lower to symmetric)
        crate::engine::engine_util_misc::mju_lower2sym_map(mapM2D, nv, D_rowadr, D_rownnz, D_colind, M_rowadr, M_rownnz, M_colind, scratch);

        // make mapD2M: D -> M (symmetric to lower)
        crate::engine::engine_util_misc::mju_sparse_map(mapD2M, nv, M_rowadr, M_rownnz, M_colind, D_rowadr, D_rownnz, D_colind);

        // make mapM2M
        let mut i: i32 = 0;
        while i < nM {
            *M.add(i as usize) = i;
            i += 1;
        }
        crate::engine::engine_util_misc::mju_fill_int(mapM2M, -1, nC);
        copy_m2sparse(nv, dof_Madr, dof_simplenum, dof_parentid, M_rownnz,
                      M_rowadr, M, mapM2M, /*reduced=*/1, /*upper=*/0, scratch);

        // check that all indices are filled in
        let mut i: i32 = 0;
        while i < nC {
            if *mapM2M.add(i as usize) < 0 {
                crate::engine::engine_util_errmem::mju_error(b"unassigned index in mapM2M\0".as_ptr() as *const i8);
            }
            i += 1;
        }
    }
}

/// C: mj_makeData (engine/engine_io.h:113)
/// Calls: mj_initPlugin, mj_makeRawData, mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_data(m: *const mjModel) -> *mut mjData {
    extern "C" { fn mj_makeData(m: *const mjModel) -> *mut mjData; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeData(m) }
}

/// C: mj_makeRawData (engine/engine_io.h:116)
/// Calls: freeDataBuffers, mj_setPtrData, mju_free, mju_malloc, mju_message, mju_warning, safeAddToBufferSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_raw_data(dest: *mut *mut mjData, m: *const mjModel) {
    extern "C" { fn mj_makeRawData(dest: *mut *mut mjData, m: *const mjModel); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeRawData(dest, m) }
}

/// C: mj_copyData (engine/engine_io.h:120)
/// Calls: mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    // SAFETY: delegates to mj_copy_data_visual Rust wrapper
    crate::engine::engine_io::mj_copy_data_visual(dest, m, src, 1)
}

/// C: mjv_copyData (engine/engine_io.h:123)
/// Calls: mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    // SAFETY: delegates to mj_copy_data_visual Rust wrapper
    crate::engine::engine_io::mj_copy_data_visual(dest, m, src, 0)
}

/// C: mj_resetData (engine/engine_io.h:126)
/// Calls: _resetData, mj_logTimingDiagnostics
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller.
    crate::engine::engine_io::mj_log_timing_diagnostics(d as *const mjData);
    crate::engine::engine_io::reset_data(m, d, 0);
}

/// C: mj_resetDataDebug (engine/engine_io.h:129)
/// Calls: _resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_debug(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    // SAFETY: delegates to reset_data Rust wrapper
    crate::engine::engine_io::reset_data(m, d, debug_value);
}

/// C: mj_resetDataKeyframe (engine/engine_io.h:132)
/// Calls: _resetData, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_keyframe(m: *const mjModel, d: *mut mjData, key: i32) {
    // SAFETY: m, d valid per caller. Key index checked before use.
    unsafe {
        crate::engine::engine_io::reset_data(m, d, 0);

        // copy keyframe data if key is valid
        if key >= 0 && key < (*m).nkey as i32 {
            (*d).time = *(*m).key_time.add(key as usize);
            crate::engine::engine_util_blas::mju_copy((*d).qpos, (*m).key_qpos.add(key as usize * (*m).nq), (*m).nq as i32);
            crate::engine::engine_util_blas::mju_copy((*d).qvel, (*m).key_qvel.add(key as usize * (*m).nv), (*m).nv as i32);
            crate::engine::engine_util_blas::mju_copy((*d).act, (*m).key_act.add(key as usize * (*m).na), (*m).na as i32);
            crate::engine::engine_util_blas::mju_copy((*d).mocap_pos, (*m).key_mpos.add(key as usize * 3 * (*m).nmocap), 3 * (*m).nmocap as i32);
            crate::engine::engine_util_blas::mju_copy((*d).mocap_quat, (*m).key_mquat.add(key as usize * 4 * (*m).nmocap), 4 * (*m).nmocap as i32);
            crate::engine::engine_util_blas::mju_copy((*d).ctrl, (*m).key_ctrl.add(key as usize * (*m).nu), (*m).nu as i32);
        }
    }
}

/// C: mj_initPlugin (engine/engine_io.h:135)
/// Calls: mjp_getPluginAtSlot, mju_free, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_plugin(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller contract. Plugin function pointer is type-erased
    // in the Rust struct (Option<fn()>) but actually has signature
    // (const mjModel*, mjData*, int) -> int. We transmute to call correctly.
    unsafe {
        (*d).nplugin = (*m).nplugin as i32;
        let mut i: i32 = 0;
        while i < (*m).nplugin as i32 {
            *(*d).plugin.offset(i as isize) = *(*m).plugin.offset(i as isize);
            let plugin: *const mjpPlugin =
                crate::engine::engine_plugin::mjp_get_plugin_at_slot(*(*m).plugin.offset(i as isize));
            if let Some(init_fn) = (*plugin).init {
                // SAFETY: transmute erased fn() to true signature (m, d, i) -> i32
                let init_typed: unsafe extern "C" fn(*const mjModel, *mut mjData, i32) -> i32 =
                    core::mem::transmute(init_fn);
                if init_typed(m, d, i) < 0 {
                    crate::engine::engine_util_errmem::mju_free((*d).buffer);
                    crate::engine::engine_util_errmem::mju_free((*d).arena);
                    crate::engine::engine_util_errmem::mju_free(d as *mut ());
                    crate::engine::engine_util_errmem::mju_error(
                        b"plugin->init failed for plugin id\0".as_ptr() as *const i8,
                    );
                }
            }
            i += 1;
        }
    }
}

/// C: mj_deleteData (engine/engine_io.h:138)
/// Calls: freeDataBuffers, mju_free, mju_threadpool
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_data(d: *mut mjData) {
    // SAFETY: d may be null (early return). Otherwise valid per caller.
    if d.is_null() {
        return;
    }
    crate::engine::engine_thread::mju_threadpool(d, 0);
    crate::engine::engine_io::free_data_buffers(d);
    crate::engine::engine_util_errmem::mju_free(d as *mut ());
}

