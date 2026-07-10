//! Port of: engine/engine_derivative.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjd_cross (engine/engine_derivative.c:38)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross(a: *const f64, b: *const f64, Da: *mut f64, Db: *mut f64) {
    if a.is_null() || b.is_null() { return; }
    extern "C" { fn mjd_cross(a: *const f64, b: *const f64, Da: *mut f64, Db: *mut f64); }
    // SAFETY: a, b verified non-null
    unsafe { mjd_cross(a, b, Da, Db) }
}

/// C: mjd_crossMotion_vel (engine/engine_derivative.c:65)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_motion_vel(D: *mut f64, v: *const f64) {
    if D.is_null() || v.is_null() { return; }
    extern "C" { fn mjd_crossMotion_vel(D: *mut f64, v: *const f64); }
    // SAFETY: D, v verified non-null
    unsafe { mjd_crossMotion_vel(D, v) }
}

/// C: mjd_crossForce_vel (engine/engine_derivative.c:101)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_force_vel(D: *mut f64, f: *const f64) {
    if D.is_null() || f.is_null() { return; }
    extern "C" { fn mjd_crossForce_vel(D: *mut f64, f: *const f64); }
    // SAFETY: D, f verified non-null
    unsafe { mjd_crossForce_vel(D, f) }
}

/// C: mjd_crossForce_frc (engine/engine_derivative.c:137)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_force_frc(D: *mut f64, vel: *const f64) {
    if D.is_null() || vel.is_null() { return; }
    extern "C" { fn mjd_crossForce_frc(D: *mut f64, vel: *const f64); }
    // SAFETY: D, vel verified non-null
    unsafe { mjd_crossForce_frc(D, vel) }
}

/// C: mjd_mulInertVec_vel (engine/engine_derivative.c:173)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_mul_inert_vec_vel(D: *mut f64, i: *const f64) {
    if D.is_null() || i.is_null() { return; }
    extern "C" { fn mjd_mulInertVec_vel(D: *mut f64, i: *const f64); }
    // SAFETY: D, i verified non-null
    unsafe { mjd_mulInertVec_vel(D, i) }
}

/// C: mjd_comVel_vel_dense (engine/engine_derivative.c:321)
/// Calls: mjd_crossMotion_vel, mju_copy, mju_mulMatMat, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_com_vel_vel_dense(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64) {
    extern "C" { fn mjd_comVel_vel_dense(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjd_comVel_vel_dense(m, d, Dcvel, Dcdofdot) }
}

/// C: copyFromParent (engine/engine_derivative.c:468)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_from_parent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32) {
    if m.is_null() || d.is_null() { return; }
    extern "C" { fn copyFromParent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32); }
    // SAFETY: m, d verified non-null
    unsafe { copyFromParent(m, d, mat, n) }
}

/// C: addToParent (engine/engine_derivative.c:491)
/// Calls: mju_addTo, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_to_parent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32) {
    if m.is_null() || d.is_null() { return; }
    extern "C" { fn addToParent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32); }
    // SAFETY: m, d verified non-null
    unsafe { addToParent(m, d, mat, n) }
}

/// C: mjd_comVel_vel (engine/engine_derivative.c:524)
/// Calls: copyFromParent, mjd_crossMotion_vel, mju_addTo, mju_message, mju_mulMatMat, mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_com_vel_vel(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64) {
    extern "C" { fn mjd_comVel_vel(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjd_comVel_vel(m, d, Dcvel, Dcdofdot) }
}

/// C: mjd_rne_vel (engine/engine_derivative.c:596)
/// Calls: addToParent, copyFromParent, mj_freeStack, mj_markStack, mj_stackAllocInfo, mjd_comVel_vel, mjd_crossForce_frc, mjd_crossForce_vel, mjd_mulInertVec_vel, mju_addTo, mju_addToScl, mju_mulInertVec, mju_mulMatMat, mju_mulMatVec, mju_subFrom, mju_transpose, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mjd_rne_vel(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjd_rne_vel(m, d) }
}

/// C: addJTBJ (engine/engine_derivative.c:711)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32) {
    extern "C" { fn addJTBJ(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32); }
    // SAFETY: delegates to C implementation
    unsafe { addJTBJ(m, d, J, B, n) }
}

/// C: addJTBJSparse (engine/engine_derivative.c:746)
/// Calls: mju_addToSclSparseInc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj_sparse(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32, offset: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32) {
    if m.is_null() || d.is_null() { return; }
    extern "C" { fn addJTBJSparse(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32, offset: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32); }
    // SAFETY: m, d verified non-null
    unsafe { addJTBJSparse(m, d, J, B, n, offset, J_rownnz, J_rowadr, J_colind) }
}

/// C: mjd_muscleGain_vel (engine/engine_derivative.c:781)
/// Calls: mju_max, mju_muscleGainLength
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_muscle_gain_vel(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64  {
    if lengthrange.is_null() || prm.is_null() { return 0.0; }
    extern "C" { fn mjd_muscleGain_vel(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64; }
    // SAFETY: lengthrange, prm verified non-null
    unsafe { mjd_muscleGain_vel(len, vel, lengthrange, acc0, prm) }
}

/// C: addJTBJ_mulSparse (engine/engine_derivative.c:832)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_mulMatVec, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj_mul_sparse(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, J: *const f64, B: *const f64, n: i32) {
    extern "C" { fn addJTBJ_mulSparse(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, J: *const f64, B: *const f64, n: i32); }
    // SAFETY: delegates to C implementation
    unsafe { addJTBJ_mulSparse(m, d, res, vec, J_rownnz, J_rowadr, J_colind, J, B, n) }
}

/// C: mjd_flexInterp_kernel (engine/engine_derivative.c:872)
/// Calls: addJTBJ_mulSparse, mj_bodyChain, mj_freeStack, mj_jacSparse, mj_markStack, mj_stackAllocInfo, mju_flexGatherCellState, mju_flexGatherFaceState, mju_flexGatherState, mju_mulMatMat3, mju_quat2Mat, mju_transpose, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_kernel(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64, K_rot_out: *mut f64) {
    extern "C" { fn mjd_flexInterp_kernel(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64, K_rot_out: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_flexInterp_kernel(m, d, res, vec, s1, s2, K_rot_cache, K_rot_out) }
}

/// C: pow2 (engine/engine_derivative.c:1339)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn pow2(val: f64) -> f64  {
    val * val
}

/// C: ellipsoid_max_moment (engine/engine_derivative.c:1344)
/// Calls: mju_max, pow2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ellipsoid_max_moment(size: *const f64, dir: i32) -> f64  {
    extern "C" { fn ellipsoid_max_moment(size: *const f64, dir: i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { ellipsoid_max_moment(size, dir) }
}

/// C: addToQuadrant (engine/engine_derivative.c:1354)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_to_quadrant(B: *mut f64, D: *const f64, col_quad: i32, row_quad: i32) {
    if B.is_null() || D.is_null() { return; }
    extern "C" { fn addToQuadrant(B: *mut f64, D: *const f64, col_quad: i32, row_quad: i32); }
    // SAFETY: B, D verified non-null
    unsafe { addToQuadrant(B, D, col_quad, row_quad) }
}

/// C: mjd_addedMassForces (engine/engine_derivative.c:1371)
/// Calls: addToQuadrant, mjd_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_added_mass_forces(B: *mut f64, local_vels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64) {
    extern "C" { fn mjd_addedMassForces(B: *mut f64, local_vels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_addedMassForces(B, local_vels, fluid_density, virtual_mass, virtual_inertia) }
}

/// C: mjd_viscous_torque (engine/engine_derivative.c:1416)
/// Calls: ellipsoid_max_moment, mju_addToScl3, mju_max, mju_min, mju_norm3, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_viscous_torque(D: *mut f64, lvel: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, slender_drag_coef: f64, ang_drag_coef: f64) {
    extern "C" { fn mjd_viscous_torque(D: *mut f64, lvel: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, slender_drag_coef: f64, ang_drag_coef: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_viscous_torque(D, lvel, fluid_density, fluid_viscosity, size, slender_drag_coef, ang_drag_coef) }
}

/// C: mjd_viscous_drag (engine/engine_derivative.c:1469)
/// Calls: mju_addToScl3, mju_max, mju_min, mju_scl, pow2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_viscous_drag(D: *mut f64, lvel: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, blunt_drag_coef: f64, slender_drag_coef: f64) {
    extern "C" { fn mjd_viscous_drag(D: *mut f64, lvel: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, blunt_drag_coef: f64, slender_drag_coef: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_viscous_drag(D, lvel, fluid_density, fluid_viscosity, size, blunt_drag_coef, slender_drag_coef) }
}

/// C: mjd_kutta_lift (engine/engine_derivative.c:1536)
/// Calls: mju_addToScl3, mju_max, mju_scl, pow2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_kutta_lift(D: *mut f64, lvel: *const f64, fluid_density: f64, size: *const f64, kutta_lift_coef: f64) {
    extern "C" { fn mjd_kutta_lift(D: *mut f64, lvel: *const f64, fluid_density: f64, size: *const f64, kutta_lift_coef: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_kutta_lift(D, lvel, fluid_density, size, kutta_lift_coef) }
}

/// C: mjd_magnus_force (engine/engine_derivative.c:1589)
/// Calls: addToQuadrant, mjd_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_magnus_force(B: *mut f64, lvel: *const f64, fluid_density: f64, size: *const f64, magnus_lift_coef: f64) {
    extern "C" { fn mjd_magnus_force(B: *mut f64, lvel: *const f64, fluid_density: f64, size: *const f64, magnus_lift_coef: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_magnus_force(B, lvel, fluid_density, size, magnus_lift_coef) }
}

/// C: mjd_ellipsoidFluid (engine/engine_derivative.c:1618)
/// Calls: addJTBJ, addJTBJSparse, addToQuadrant, mj_bodyChain, mj_freeStack, mj_isSparse, mj_jacGeom, mj_jacSparse, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mjd_addedMassForces, mjd_kutta_lift, mjd_magnus_force, mjd_viscous_drag, mjd_viscous_torque, mju_copy, mju_copy3, mju_geomSemiAxes, mju_mulMatTMat, mju_subFrom3, mju_symmetrize, mju_transformSpatial, mju_zero, readFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mjd_ellipsoid_fluid(m: *const mjModel, d: *mut mjData, bodyid: i32) {
    extern "C" { fn mjd_ellipsoidFluid(m: *const mjModel, d: *mut mjData, bodyid: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjd_ellipsoidFluid(m, d, bodyid) }
}

/// C: mjd_inertiaBoxFluid (engine/engine_derivative.c:1724)
/// Calls: addJTBJ, addJTBJSparse, mj_bodyChain, mj_freeStack, mj_isSparse, mj_jacBodyCom, mj_jacSparse, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mju_copy, mju_copy3, mju_max, mju_mulMatTMat, mju_subFrom3, mju_transformSpatial, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_inertia_box_fluid(m: *const mjModel, d: *mut mjData, i: i32) {
    extern "C" { fn mjd_inertiaBoxFluid(m: *const mjModel, d: *mut mjData, i: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjd_inertiaBoxFluid(m, d, i) }
}

/// C: mjd_subQuat (engine/engine_derivative.h:27)
/// Calls: mju_addToScl, mju_copy9, mju_mulMatMat3, mju_normalize3, mju_scl, mju_subQuat, mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case, unused_unsafe)]
pub fn mjd_sub_quat(qa: *const f64, qb: *const f64, Da: *mut f64, Db: *mut f64) {
    // SAFETY: raw pointer arithmetic on caller-guaranteed valid buffers.
    // qa, qb: 4 f64; Da, Db: 9 f64 (or null)
    unsafe {
        // no outputs, quick return
        if Da.is_null() && Db.is_null() {
            return;
        }

        // compute axis-angle quaternion difference
        let mut axis: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_sub_quat(axis.as_mut_ptr(), qa, qb);

        // normalize axis, get half-angle
        let half_angle: f64 = 0.5 * crate::engine::engine_util_blas::mju_normalize3(axis.as_mut_ptr());

        // identity
        let mut Da_tmp: [f64; 9] = [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];

        // add term linear in cross product matrix K
        let K: [f64; 9] = [
            0.0, -axis[2], axis[1],
            axis[2], 0.0, -axis[0],
            -axis[1], axis[0], 0.0,
        ];
        crate::engine::engine_util_blas::mju_add_to_scl(Da_tmp.as_mut_ptr(), K.as_ptr(), half_angle, 9);

        // add term linear in K * K
        let mut KK: [f64; 9] = [0.0; 9];
        crate::engine::engine_util_blas::mju_mul_mat_mat3(KK.as_mut_ptr(), K.as_ptr(), K.as_ptr());
        let coef: f64 = 1.0 - (if half_angle < 6e-8 { 1.0 } else { half_angle / half_angle.tan() });
        crate::engine::engine_util_blas::mju_add_to_scl(Da_tmp.as_mut_ptr(), KK.as_ptr(), coef, 9);

        if !Da.is_null() {
            crate::engine::engine_util_blas::mju_copy9(Da, Da_tmp.as_ptr());
        }

        if !Db.is_null() {
            // Db = -Da^T
            crate::engine::engine_util_blas::mju_transpose(Db, Da_tmp.as_ptr(), 3, 3);
            crate::engine::engine_util_blas::mju_scl(Db, Db, -1.0, 9);
        }
    }
}

/// C: mjd_quatIntegrate (engine/engine_derivative.h:30)
/// Calls: mju_copy9, mju_dot3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_quat_integrate(vel: *const f64, scale: f64, Dquat: *mut f64, Dvel: *mut f64, Dscale: *mut f64) {
    // SAFETY: raw pointer arithmetic on caller-guaranteed valid buffers.
    // vel: 3 f64; Dquat, Dvel: 9 f64 (or null); Dscale: 3 f64 (or null)
    unsafe {
        // scaled velocity
        let s: [f64; 3] = [scale * *vel.add(0), scale * *vel.add(1), scale * *vel.add(2)];

        // 3 basis matrices
        let eye: [f64; 9] = [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];
        let cross: [f64; 9] = [
             0.0,    s[2], -s[1],
            -s[2],   0.0,   s[0],
             s[1],  -s[0],  0.0,
        ];
        let outer: [f64; 9] = [
            s[0]*s[0], s[0]*s[1], s[0]*s[2],
            s[1]*s[0], s[1]*s[1], s[1]*s[2],
            s[2]*s[0], s[2]*s[1], s[2]*s[2],
        ];

        // squared norm, norm of s
        let xx: f64 = crate::engine::engine_util_blas::mju_dot3(s.as_ptr(), s.as_ptr());
        let x: f64 = xx.sqrt();

        // 4 coefficients: a=cos(x), b=sin(x)/x, c=(1-cos(x))/x^2, d=(x-sin(x))/x^3
        let a: f64 = x.cos();
        let b: f64;
        let c: f64;
        let d: f64;

        // x is not small: use full expressions
        if x.abs() > 1.0 / 32.0 {
            b = x.sin() / x;
            c = (1.0 - a) / xx;
            d = (1.0 - b) / xx;
        }
        // |x| <= 1/32: use 6th order Taylor expansion (Horner form)
        else {
            b = 1.0 + xx / 6.0 * (xx / 20.0 * (1.0 - xx / 42.0) - 1.0);
            c = (1.0 + xx / 12.0 * (xx / 30.0 * (1.0 - xx / 56.0) - 1.0)) / 2.0;
            d = (1.0 + xx / 20.0 * (xx / 42.0 * (1.0 - xx / 72.0) - 1.0)) / 6.0;
        }

        // derivatives
        let mut Dvel_: [f64; 9] = [0.0; 9];
        for i in 0..9 {
            if !Dquat.is_null() {
                *Dquat.add(i) = a * eye[i] + b * cross[i] + c * outer[i];
            }
            if !Dvel.is_null() || !Dscale.is_null() {
                Dvel_[i] = b * eye[i] + c * cross[i] + d * outer[i];
            }
        }
        if !Dvel.is_null() {
            crate::engine::engine_util_blas::mju_copy9(Dvel, Dvel_.as_ptr());
        }
        if !Dscale.is_null() {
            crate::engine::engine_util_blas::mju_mul_mat_vec3(Dscale, Dvel_.as_ptr(), vel);
        }
    }
}

/// C: mjd_smooth_vel (engine/engine_derivative.h:35)
/// Calls: mjd_actuator_vel, mjd_passive_vel, mjd_rne_vel, mju_zero, mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mjd_smooth_vel(m: *const mjModel, d: *mut mjData, flg_bias: i32) {
    unsafe {
        // mjENABLED(mjENBL_SLEEP) => (m->opt.enableflags & (1<<2))
        let sleep_filter = ((*m).opt.enableflags & (1 << 2)) != 0
            && ((*d).nv_awake as usize) < (*m).nv;

        // clear qDeriv
        if !sleep_filter {
            crate::engine::engine_util_blas::mju_zero((*d).qDeriv, (*m).nD as i32);
        } else {
            crate::engine::engine_util_sparse::mju_zero_sparse(
                (*d).qDeriv, (*m).D_rownnz, (*m).D_rowadr,
                (*d).dof_awake_ind, (*d).nv_awake,
            );
        }

        // qDeriv += d qfrc_actuator / d qvel
        mjd_actuator_vel(m, d);

        // qDeriv += d qfrc_passive / d qvel
        mjd_passive_vel(m, d);

        // qDeriv -= d qfrc_bias / d qvel; optional
        if flg_bias != 0 {
            mjd_rne_vel(m, d);
        }
    }
}

/// C: mjd_actuator_vel (engine/engine_derivative.h:38)
/// Calls: addJTBJSparse, mj_actuatorDisabled, mj_nextActivation, mj_sleepState, mjd_muscleGain_vel, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mjd_actuator_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mjd_actuator_vel(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjd_actuator_vel(m, d) }
}

/// C: mjd_passive_vel (engine/engine_derivative.h:41)
/// Calls: addJTBJSparse, mj_actuatorDamping, mjd_ellipsoidFluid, mjd_inertiaBoxFluid, mjd_xPolyForce, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mjd_passive_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mjd_passive_vel(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjd_passive_vel(m, d) }
}

/// C: mjd_rne_vel_dense (engine/engine_derivative.h:44)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mjd_comVel_vel_dense, mjd_crossForce_frc, mjd_crossForce_vel, mjd_mulInertVec_vel, mju_addTo, mju_addToScl, mju_copy, mju_mulInertVec, mju_mulMatMat, mju_scl, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel_dense(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mjd_rne_vel_dense(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjd_rne_vel_dense(m, d) }
}

/// C: mjd_flexInterp_mul (engine/engine_derivative.h:48)
/// Calls: mjd_flexInterp_kernel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64) {
    // SAFETY: delegates to mjd_flex_interp_kernel with K_rot_out=NULL.
    unsafe {
        mjd_flex_interp_kernel(m, d, res, vec, s1, s2, K_rot_cache, core::ptr::null_mut());
    }
}

/// C: mjd_flexInterp_cacheKrot (engine/engine_derivative.h:52)
/// Calls: mjd_flexInterp_kernel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_cache_krot(m: *const mjModel, d: *mut mjData, K_rot_out: *mut f64) {
    // SAFETY: uses s1=1, s2=0 so scale=1 and K_rot_out gets unscaled values.
    unsafe {
        mjd_flex_interp_kernel(m, d, core::ptr::null_mut(), core::ptr::null(), 1.0, 0.0, core::ptr::null(), K_rot_out);
    }
}

/// C: mjd_flexBend_mul (engine/engine_derivative.h:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_bend_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64) {
    // SAFETY: m, d valid. res/vec have nv elements. All flex model arrays valid.
    unsafe {
        let mut f: i32 = 0;
        while f < (*m).nflex as i32 {
            // skip interp, rigid, or non-2D
            if *(*m).flex_interp.add(f as usize) != 0
                || *((*m).flex_rigid as *const u8).add(f as usize) != 0
                || *(*m).flex_dim.add(f as usize) != 2 {
                f += 1;
                continue;
            }

            let bendingadr = *(*m).flex_bendingadr.add(f as usize);
            if bendingadr < 0 {
                f += 1;
                continue;
            }

            let scale = s1 + s2 * *(*m).flex_damping.add(f as usize);
            if scale == 0.0 {
                f += 1;
                continue;
            }

            let b = (*m).flex_bending.add(bendingadr as usize);
            let bodyid = (*m).flex_vertbodyid.add(*(*m).flex_vertadr.add(f as usize) as usize);
            let edgenum = *(*m).flex_edgenum.add(f as usize);
            let edgeadr = *(*m).flex_edgeadr.add(f as usize);

            let mut e: i32 = 0;
            while e < edgenum {
                let edge = (*m).flex_edge.add(2 * (e + edgeadr) as usize);
                let flap = (*m).flex_edgeflap.add(2 * (e + edgeadr) as usize);
                let v: [i32; 4] = [*edge.add(0), *edge.add(1), *flap.add(0), *flap.add(1)];

                // skip boundary edges (no second flap vertex)
                if v[3] == -1 {
                    e += 1;
                    continue;
                }

                // apply 4x4 bending stencil, coordinate-wise
                let mut i: i32 = 0;
                while i < 4 {
                    let dof_i = *(*m).body_dofadr.add(*bodyid.add(v[i as usize] as usize) as usize);
                    let mut x: i32 = 0;
                    while x < 3 {
                        let mut val: f64 = 0.0;
                        let mut j: i32 = 0;
                        while j < 4 {
                            let dof_j = *(*m).body_dofadr.add(*bodyid.add(v[j as usize] as usize) as usize);
                            val += *b.add((17 * e + 4 * i + j) as usize) * *vec.add((dof_j + x) as usize);
                            j += 1;
                        }
                        *res.add((dof_i + x) as usize) += scale * val;
                        x += 1;
                    }
                    i += 1;
                }
                e += 1;
            }
            f += 1;
        }
    }
}

