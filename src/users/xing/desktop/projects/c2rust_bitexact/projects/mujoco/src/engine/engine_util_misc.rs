//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: is_intersect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_intersect(p1: *const f64, p2: *const f64, p3: *const f64, p4: *const f64) -> bool {
    todo!() // cxx-internal:engine_util_misc.c.o:_is_intersect
}

/// C: length_circle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:55)
/// Calls: cxx:_mju_dot, cxx:_mju_normalize
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn length_circle(p0: *const f64, p1: *const f64, ind: i32, radius: f64) -> f64 {
    todo!() // cxx-internal:engine_util_misc.c.o:_length_circle
}

/// C: wrap_circle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:78)
/// Calls: cxx-internal:engine_util_misc.c.o:_is_intersect, cxx-internal:engine_util_misc.c.o:_length_circle, cxx:_mju_add, cxx:_mju_dot, cxx:_mju_normalize, cxx:_mju_sub
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_circle(pnt: *mut f64, end: *const f64, side: *const f64, radius: f64) -> f64 {
    todo!() // cxx-internal:engine_util_misc.c.o:_wrap_circle
}

/// C: wrap_inside (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:158)
/// Calls: cxx:_mju_addScl, cxx:_mju_copy, cxx:_mju_max, cxx:_mju_norm, cxx:_mju_normalize, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_inside(pnt: *mut f64, end: *const f64, radius: f64) -> f64 {
    todo!() // cxx-internal:engine_util_misc.c.o:_wrap_inside
}

/// C: flexInterpRotation (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:694)
/// Calls: cxx:_mju_defGradient, cxx:_mju_mat2Rot, cxx:_mju_negQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn flex_interp_rotation(order: i32, xpos_c: *const f64, local: *const f64, quat: *mut f64) {
    todo!() // cxx-internal:engine_util_misc.c.o:_flexInterpRotation
}

/// C: nodeAt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:902)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn node_at(nodexpos: *const f64, ny: i32, nz: i32, i: i32, j: i32, k: i32) -> *const f64 {
    todo!() // cxx-internal:engine_util_misc.c.o:_nodeAt
}

/// C: addWeight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:984)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_weight(nb: *mut i32, body: *mut i32, bweight: *mut f64, b: i32, w: f64) {
    todo!() // cxx-internal:engine_util_misc.c.o:_addWeight
}

/// C: _decode (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:1217)
#[allow(unused_variables, non_snake_case)]
pub fn decode(ch: i8) -> u32 {
    todo!() // cxx-internal:engine_util_misc.c.o:__decode
}

/// C: historyPhysicalIndex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:1359)
#[allow(unused_variables, non_snake_case)]
pub fn history_physical_index(cursor: i32, n: i32, logical: i32) -> i32 {
    todo!() // cxx-internal:engine_util_misc.c.o:_historyPhysicalIndex
}

/// C: historyFindIndex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:1367)
/// Calls: cxx-internal:engine_util_misc.c.o:_historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn history_find_index(times: *const f64, n: i32, cursor: i32, t: f64) -> i32 {
    todo!() // cxx-internal:engine_util_misc.c.o:_historyFindIndex
}

/// C: mju_wrap (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:32)
/// Calls: cxx-internal:engine_util_misc.c.o:_wrap_circle, cxx-internal:engine_util_misc.c.o:_wrap_inside, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_norm3, cxx:_mju_normalize, cxx:_mju_normalize3, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_wrap(wpnt: *mut f64, x0: *const f64, x1: *const f64, xpos: *const f64, xmat: *const f64, radius: f64, r#type: i32, side: *const f64) -> f64 {
    todo!() // cxx:_mju_wrap
}

/// C: mju_muscleGainLength (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain_length(length: f64, lmin: f64, lmax: f64) -> f64 {
    todo!() // cxx:_mju_muscleGainLength
}

/// C: mju_muscleGain (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:39)
/// Calls: cxx:_mju_muscleGainLength
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // cxx:_mju_muscleGain
}

/// C: mju_muscleBias (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:43)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_bias(len: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // cxx:_mju_muscleBias
}

/// C: mju_muscleDynamicsTimescale (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:47)
/// Calls: cxx:_mju_sigmoid
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_dynamics_timescale(dctrl: f64, tau_act: f64, tau_deact: f64, smoothing_width: f64) -> f64 {
    todo!() // cxx:_mju_muscleDynamicsTimescale
}

/// C: mju_muscleDynamics (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:51)
/// Calls: cxx:_mju_clip, cxx:_mju_muscleDynamicsTimescale
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_dynamics(ctrl: f64, act: f64, prm: *const f64) -> f64 {
    todo!() // cxx:_mju_muscleDynamics
}

/// C: mj_lugreStribeck (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:54)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_lugre_stribeck(velocity: f64, F_C: f64, F_S: f64, v_S: f64) -> f64 {
    todo!() // cxx:_mj_lugreStribeck
}

/// C: mj_dcmotorSlots (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_dcmotor_slots(dynprm: *const f64, gainprm: *const f64) -> mjDCMotorSlots {
    todo!() // cxx:_mj_dcmotorSlots
}

/// C: mju_geomSemiAxes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_geom_semi_axes(semiaxes: *mut f64, size: *const f64, r#type: u32) {
    todo!() // cxx:_mju_geomSemiAxes
}

/// C: mju_insideGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:74)
/// Calls: cxx:_mju_clip, cxx:_mju_dot3, cxx:_mju_mulMatTVec3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_inside_geom(pos: *const f64, mat: *const f64, size: *const f64, r#type: u32, point: *const f64) -> i32 {
    todo!() // cxx:_mju_insideGeom
}

/// C: mju_camPixelRay (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:79)
/// Calls: cxx:_mju_add3, cxx:_mju_copy3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_pixel_ray(origin: *mut f64, direction: *mut f64, cam_xpos: *const f64, cam_xmat: *const f64, col: i32, row: i32, fx: f64, fy: f64, cx: f64, cy: f64, projection: i32, ortho_extent: f64) {
    todo!() // cxx:_mju_camPixelRay
}

/// C: mju_defGradient (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:87)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_def_gradient(res: *mut f64, p: *const f64, dof: *const f64, order: i32) {
    todo!() // cxx:_mju_defGradient
}

/// C: mju_evalBasis (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:90)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eval_basis(x: *const f64, i: i32, order: i32) -> f64 {
    todo!() // cxx:_mju_evalBasis
}

/// C: mju_evalBasisArray (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:93)
/// Calls: cxx:_mju_evalBasis
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eval_basis_array(basis: *mut f64, x: *const f64, order: i32) {
    todo!() // cxx:_mju_evalBasisArray
}

/// C: mju_cellLookup (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:96)
/// Calls: cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cell_lookup(coord: *const f64, cellnum: *const i32, order: i32, local: *mut f64, nodeindices: *mut i32) -> i32 {
    todo!() // cxx:_mju_cellLookup
}

/// C: mju_interpolate3D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:100)
/// Calls: cxx:_mju_addToScl3, cxx:_mju_evalBasis, cxx:_mju_evalBasisArray
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_interpolate3d(res: *mut f64, x: *const f64, coeff: *const f64, order: i32, nodeindices: *const i32) {
    todo!() // cxx:_mju_interpolate3D
}

/// C: mju_flexGatherCellState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:104)
/// Calls: cxx-internal:engine_util_misc.c.o:_flexInterpRotation, cxx:_mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_cell_state(order: i32, cy: i32, cz: i32, ci: i32, cj: i32, ck: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_c: *mut f64, vel_c: *mut f64, xpos0_c: *mut f64, nodeindices: *mut i32, quat: *mut f64) {
    todo!() // cxx:_mju_flexGatherCellState
}

/// C: mju_flexGatherFaceState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:110)
/// Calls: cxx:_mju_copy3, cxx:_mju_flexInterpRotation2D
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_face_state(order: i32, cx: i32, cy: i32, cz: i32, face_elem_idx: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_f: *mut f64, vel_f: *mut f64, xpos0_f: *mut f64, nodeindices: *mut i32, quat: *mut f64) {
    todo!() // cxx:_mju_flexGatherFaceState
}

/// C: mju_flexInterpRotation2D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:118)
/// Calls: cxx:_mju_cross, cxx:_mju_mat2Rot, cxx:_mju_negQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_interp_rotation2d(order: i32, xpos_f: *const f64, npe: i32, axis0: i32, axis1: i32, normal_axis: i32, local: *const f64, quat: *mut f64) {
    todo!() // cxx:_mju_flexInterpRotation2D
}

/// C: mju_flexFaceNormal2D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:124)
/// Calls: cxx:_mju_cross, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_face_normal2d(normal: *mut f64, t1: *mut f64, t2: *mut f64, order: i32, xpos_f: *const f64, local: *const f64) {
    todo!() // cxx:_mju_flexFaceNormal2D
}

/// C: mju_flexPhi (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:130)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_phi(s: f64, i: i32, order: i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:__ZL11mju_flexPhidii
}

/// C: mju_flexDphi (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:141)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_dphi(s: f64, i: i32, order: i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:__ZL12mju_flexDphidii
}

/// C: mju_shellTrackInterior (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:151)
/// Calls: cxx-internal:engine_util_misc.c.o:_nodeAt, cxx:_mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_shell_track_interior(nodexpos: *mut f64, nx: i32, ny: i32, nz: i32) {
    todo!() // cxx:_mju_shellTrackInterior
}

/// C: mju_shellTFIWeights (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:154)
/// Calls: cxx-internal:engine_util_misc.c.o:_addWeight
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_shell_tfi_weights(nx: i32, ny: i32, nz: i32, i: i32, j: i32, k: i32, w: f64, nb: *mut i32, body: *mut i32, bweight: *mut f64, nodebodyid: *const i32, nstart: i32) {
    todo!() // cxx:_mju_shellTFIWeights
}

/// C: mju_encodeBase64 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:163)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_next
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_base64(buf: *mut i8, data: *const u8, ndata: usize) -> usize {
    todo!() // cxx:_mju_encodeBase64
}

/// C: mju_isValidBase64 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:167)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_valid_base64(s: *const i8) -> usize {
    todo!() // cxx:_mju_isValidBase64
}

/// C: mju_decodeBase64 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:171)
/// Calls: cxx-internal:engine_util_misc.c.o:__decode
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_base64(buf: *mut u8, s: *const i8) -> usize {
    todo!() // cxx:_mju_decodeBase64
}

/// C: mju_historyInit (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:184)
/// Calls: cxx:_mju_copy, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_init(buf: *mut f64, n: i32, dim: i32, times: *const f64, values: *const f64, user: f64) {
    todo!() // cxx:_mju_historyInit
}

/// C: mju_historyInsert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:189)
/// Calls: cxx-internal:engine_util_misc.c.o:_historyFindIndex, cxx-internal:engine_util_misc.c.o:_historyPhysicalIndex, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_insert(buf: *mut f64, n: i32, dim: i32, t: f64) -> *mut f64 {
    todo!() // cxx:_mju_historyInsert
}

/// C: mju_historyRead (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:194)
/// Calls: cxx-internal:engine_util_misc.c.o:_historyFindIndex, cxx-internal:engine_util_misc.c.o:_historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_read(buf: *const f64, n: i32, dim: i32, res: *mut f64, t: f64, interp: i32) -> *const f64 {
    todo!() // cxx:_mju_historyRead
}

/// C: mju_encodePyramid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:200)
/// Calls: cxx:_mju_min
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_pyramid(pyramid: *mut f64, force: *const f64, mu: *const f64, dim: i32) {
    todo!() // cxx:_mju_encodePyramid
}

/// C: mju_decodePyramid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_pyramid(force: *mut f64, pyramid: *const f64, mu: *const f64, dim: i32) {
    todo!() // cxx:_mju_decodePyramid
}

/// C: mju_springDamper (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:208)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_spring_damper(pos0: f64, vel0: f64, Kp: f64, Kv: f64, dt: f64) -> f64 {
    todo!() // cxx:_mju_springDamper
}

/// C: mju_outsideBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:213)
/// Calls: cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_outside_box(point: *const f64, pos: *const f64, mat: *const f64, size: *const f64, inflate: f64) -> i32 {
    todo!() // cxx:_mju_outsideBox
}

/// C: mju_printMat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:217)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat(mat: *const f64, nr: i32, nc: i32) {
    todo!() // cxx:_mju_printMat
}

/// C: mju_printMatSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:220)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat_sparse(mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // cxx:_mju_printMatSparse
}

/// C: mju_min (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_min(a: f64, b: f64) -> f64 {
    todo!() // cxx:_mju_min
}

/// C: mju_max (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:228)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_max(a: f64, b: f64) -> f64 {
    todo!() // cxx:_mju_max
}

/// C: mju_clip (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:231)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_clip(x: f64, min: f64, max: f64) -> f64 {
    // clip x to the range [min, max]
    // C: return x < min ? min : (x > max ? max : x);
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// C: mju_sign (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sign(x: f64) -> f64 {
    todo!() // cxx:_mju_sign
}

/// C: mju_round (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:237)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_round(x: f64) -> i32 {
    todo!() // cxx:_mju_round
}

/// C: mju_type2Str (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn mju_type2str(r#type: i32) -> *const i8 {
    todo!() // cxx:_mju_type2Str
}

/// C: mju_str2Type (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:243)
#[allow(unused_variables, non_snake_case)]
pub fn mju_str2type(str: *const i8) -> i32 {
    todo!() // cxx:_mju_str2Type
}

/// C: mju_writeNumBytes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_num_bytes(nbytes: usize) -> *const i8 {
    todo!() // cxx:_mju_writeNumBytes
}

/// C: mju_warningText (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:249)
/// Calls: cxx:_mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning_text(warning: i32, info: usize) -> *const i8 {
    todo!() // cxx:_mju_warningText
}

/// C: mju_isBad (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:252)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_bad(x: f64) -> i32 {
    todo!() // cxx:_mju_isBad
}

/// C: mju_isZero (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:255)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero(vec: *const f64, n: i32) -> i32 {
    todo!() // cxx:_mju_isZero
}

/// C: mju_isZeroByte (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:258)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero_byte(vec: *const u8, n: i32) -> i32 {
    todo!() // cxx:_mju_isZeroByte
}

/// C: mju_zeroInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:261)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_int(res: *mut i32, n: i32) {
    todo!() // cxx:_mju_zeroInt
}

/// C: mju_copyInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_int(res: *mut i32, vec: *const i32, n: i32) {
    todo!() // cxx:_mju_copyInt
}

/// C: mju_fillInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fill_int(res: *mut i32, val: i32, n: i32) {
    todo!() // cxx:_mju_fillInt
}

/// C: mju_standardNormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:270)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_standard_normal(num2: *mut f64) -> f64 {
    todo!() // cxx:_mju_standardNormal
}

/// C: mju_f2n (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:273)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_f2n(res: *mut f64, vec: *const f32, n: i32) {
    todo!() // cxx:_mju_f2n
}

/// C: mju_n2f (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:276)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_n2f(res: *mut f32, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_n2f
}

/// C: mju_d2n (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:279)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_d2n(res: *mut f64, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_d2n
}

/// C: mju_n2d (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:282)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_n2d(res: *mut f64, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_n2d
}

/// C: mju_gather (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:285)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_gather
}

/// C: mju_gatherMasked (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:288)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_masked(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_gatherMasked
}

/// C: mju_scatter (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:291)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scatter(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_scatter
}

/// C: mju_gatherInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:294)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_gatherInt
}

/// C: mju_scatterInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scatter_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_scatterInt
}

/// C: mju_sparseMap (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sparse_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32) {
    todo!() // cxx:_mju_sparseMap
}

/// C: mju_lower2SymMap (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:306)
/// Calls: cxx:_mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_lower2sym_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32, cursor: *mut i32) {
    todo!() // cxx:_mju_lower2SymMap
}

/// C: mju_insertionSort (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:312)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort(list: *mut f64, n: i32) {
    todo!() // cxx:_mju_insertionSort
}

/// C: mju_insertionSortInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:315)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort_int(list: *mut i32, n: i32) {
    todo!() // cxx:_mju_insertionSortInt
}

/// C: mju_Halton (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:318)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_halton(index: i32, base: i32) -> f64 {
    todo!() // cxx:_mju_Halton
}

/// C: mju_strncpy (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:321)
#[allow(unused_variables, non_snake_case)]
pub fn mju_strncpy(dst: *mut i8, src: *const i8, n: i32) -> *mut i8 {
    todo!() // cxx:_mju_strncpy
}

/// C: mju_polyForce (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:326)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    todo!() // cxx:_mju_polyForce
}

/// C: mjd_xPolyForce (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:329)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_x_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    todo!() // cxx:_mjd_xPolyForce
}

/// C: mju_polyPotential (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:332)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_potential(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    todo!() // cxx:_mju_polyPotential
}

/// C: mju_sigmoid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:335)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sigmoid(x: f64) -> f64 {
    todo!() // cxx:_mju_sigmoid
}

