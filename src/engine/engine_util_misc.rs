//! Port of: engine/engine_util_misc.c
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: is_intersect (engine/engine_util_misc.c:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_intersect(p1: *const f64, p2: *const f64, p3: *const f64, p4: *const f64) -> mjtBool {
    todo!() // is_intersect
}

/// C: length_circle (engine/engine_util_misc.c:55)
/// Calls: mju_dot, mju_normalize
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn length_circle(p0: *const f64, p1: *const f64, ind: i32, radius: f64) -> f64 {
    todo!() // length_circle
}

/// C: wrap_circle (engine/engine_util_misc.c:78)
/// Calls: is_intersect, length_circle, mju_add, mju_dot, mju_normalize, mju_sub
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_circle(pnt: *mut f64, end: *const f64, side: *const f64, radius: f64) -> f64 {
    todo!() // wrap_circle
}

/// C: wrap_inside (engine/engine_util_misc.c:158)
/// Calls: mju_addScl, mju_copy, mju_max, mju_norm, mju_normalize, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_inside(pnt: *mut f64, end: *const f64, radius: f64) -> f64 {
    todo!() // wrap_inside
}

/// C: flexInterpRotation (engine/engine_util_misc.c:694)
/// Calls: mju_defGradient, mju_mat2Rot, mju_negQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn flex_interp_rotation(order: i32, xpos_c: *const f64, local: *const f64, quat: *mut f64) {
    todo!() // flexInterpRotation
}

/// C: nodeAt (engine/engine_util_misc.c:902)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn node_at(nodexpos: *const f64, ny: i32, nz: i32, i: i32, j: i32, k: i32) -> *const f64 {
    todo!() // nodeAt
}

/// C: addWeight (engine/engine_util_misc.c:984)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_weight(nb: *mut i32, body: *mut i32, bweight: *mut f64, b: i32, w: f64) {
    todo!() // addWeight
}

/// C: _decode (engine/engine_util_misc.c:1217)
#[allow(unused_variables, non_snake_case)]
pub fn decode(ch: i8) -> i32 {
    todo!() // _decode
}

/// C: mju_encodeBase64 (engine/engine_util_misc.c:1244)
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_base64(buf: *mut i8, data: *const i32, ndata: i32) -> i32 {
    todo!() // mju_encodeBase64
}

/// C: mju_isValidBase64 (engine/engine_util_misc.c:1297)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_valid_base64(s: *const i8) -> i32 {
    todo!() // mju_isValidBase64
}

/// C: mju_decodeBase64 (engine/engine_util_misc.c:1327)
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_base64(buf: *mut i32, s: *const i8) -> i32 {
    todo!() // mju_decodeBase64
}

/// C: historyPhysicalIndex (engine/engine_util_misc.c:1359)
#[allow(unused_variables, non_snake_case)]
pub fn history_physical_index(cursor: i32, n: i32, logical: i32) -> i32 {
    todo!() // historyPhysicalIndex
}

/// C: historyFindIndex (engine/engine_util_misc.c:1367)
/// Calls: historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn history_find_index(times: *const f64, n: i32, cursor: i32, t: f64) -> i32 {
    todo!() // historyFindIndex
}

/// C: mju_writeNumBytes (engine/engine_util_misc.c:1972)
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_num_bytes(nbytes: i32) -> *const i8 {
    todo!() // mju_writeNumBytes
}

/// C: mju_warningText (engine/engine_util_misc.c:1992)
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning_text(warning: i32, info: i32) -> *const i8 {
    todo!() // mju_warningText
}

/// C: mju_wrap (engine/engine_util_misc.h:32)
/// Calls: mju_addTo3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_norm3, mju_normalize, mju_normalize3, mju_scl, mju_scl3, mju_sub3, wrap_inside
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_wrap(wpnt: *mut f64, x0: *const f64, x1: *const f64, xpos: *const f64, xmat: *const f64, radius: f64, r#type: i32, side: *const f64) -> f64 {
    todo!() // mju_wrap
}

/// C: mju_muscleGainLength (engine/engine_util_misc.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain_length(length: f64, lmin: f64, lmax: f64) -> f64 {
    todo!() // mju_muscleGainLength
}

/// C: mju_muscleGain (engine/engine_util_misc.h:39)
/// Calls: mju_muscleGainLength
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // mju_muscleGain
}

/// C: mju_muscleBias (engine/engine_util_misc.h:43)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_bias(len: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // mju_muscleBias
}

/// C: mju_muscleDynamicsTimescale (engine/engine_util_misc.h:47)
/// Calls: mju_sigmoid
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_dynamics_timescale(dctrl: f64, tau_act: f64, tau_deact: f64, smoothing_width: f64) -> f64 {
    todo!() // mju_muscleDynamicsTimescale
}

/// C: mju_muscleDynamics (engine/engine_util_misc.h:51)
/// Calls: mju_clip, mju_muscleDynamicsTimescale
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_dynamics(ctrl: f64, act: f64, prm: *const f64) -> f64 {
    todo!() // mju_muscleDynamics
}

/// C: mj_lugreStribeck (engine/engine_util_misc.h:54)
/// Calls: mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_lugre_stribeck(velocity: f64, F_C: f64, F_S: f64, v_S: f64) -> f64 {
    todo!() // mj_lugreStribeck
}

/// C: mj_dcmotorSlots (engine/engine_util_misc.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_dcmotor_slots(dynprm: *const f64, gainprm: *const f64) -> mjDCMotorSlots {
    todo!() // mj_dcmotorSlots
}

/// C: mju_geomSemiAxes (engine/engine_util_misc.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_geom_semi_axes(semiaxes: *mut f64, size: *const f64, r#type: mjtGeom) {
    todo!() // mju_geomSemiAxes
}

/// C: mju_insideGeom (engine/engine_util_misc.h:74)
/// Calls: mju_clip, mju_dot3, mju_mulMatTVec3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_inside_geom(pos: *const f64, mat: *const f64, size: *const f64, r#type: mjtGeom, point: *const f64) -> i32 {
    todo!() // mju_insideGeom
}

/// C: mju_camPixelRay (engine/engine_util_misc.h:79)
/// Calls: mju_add3, mju_copy3, mju_mulMatVec3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_pixel_ray(origin: *mut f64, direction: *mut f64, cam_xpos: *const f64, cam_xmat: *const f64, col: i32, row: i32, fx: f64, fy: f64, cx: f64, cy: f64, projection: i32, ortho_extent: f64) {
    todo!() // mju_camPixelRay
}

/// C: mju_defGradient (engine/engine_util_misc.h:87)
/// Calls: mju_flexDphi, mju_flexPhi, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_def_gradient(res: *mut f64, p: *const f64, dof: *const f64, order: i32) {
    todo!() // mju_defGradient
}

/// C: mju_evalBasis (engine/engine_util_misc.h:90)
/// Calls: mju_flexPhi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eval_basis(x: *const f64, i: i32, order: i32) -> f64 {
    todo!() // mju_evalBasis
}

/// C: mju_evalBasisArray (engine/engine_util_misc.h:93)
/// Calls: mju_evalBasis, mju_flexPhi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eval_basis_array(basis: *mut f64, x: *const f64, order: i32) {
    todo!() // mju_evalBasisArray
}

/// C: mju_cellLookup (engine/engine_util_misc.h:96)
/// Calls: mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cell_lookup(coord: *const f64, cellnum: *const i32, order: i32, local: *mut f64, nodeindices: *mut i32) -> i32 {
    todo!() // mju_cellLookup
}

/// C: mju_interpolate3D (engine/engine_util_misc.h:100)
/// Calls: mju_addToScl3, mju_evalBasis, mju_evalBasisArray
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_interpolate3d(res: *mut f64, x: *const f64, coeff: *const f64, order: i32, nodeindices: *const i32) {
    todo!() // mju_interpolate3D
}

/// C: mju_flexGatherCellState (engine/engine_util_misc.h:104)
/// Calls: flexInterpRotation, mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_cell_state(order: i32, cy: i32, cz: i32, ci: i32, cj: i32, ck: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_c: *mut f64, vel_c: *mut f64, xpos0_c: *mut f64, nodeindices: *mut i32, quat: *mut f64) {
    todo!() // mju_flexGatherCellState
}

/// C: mju_flexGatherFaceState (engine/engine_util_misc.h:110)
/// Calls: mju_copy3, mju_flexInterpRotation2D
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_face_state(order: i32, cx: i32, cy: i32, cz: i32, face_elem_idx: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_f: *mut f64, vel_f: *mut f64, xpos0_f: *mut f64, nodeindices: *mut i32, quat: *mut f64) {
    todo!() // mju_flexGatherFaceState
}

/// C: mju_flexInterpRotation2D (engine/engine_util_misc.h:118)
/// Calls: mju_cross, mju_flexDphi, mju_flexPhi, mju_mat2Rot, mju_negQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_interp_rotation2d(order: i32, xpos_f: *const f64, npe: i32, axis0: i32, axis1: i32, normal_axis: i32, local: *const f64, quat: *mut f64) {
    todo!() // mju_flexInterpRotation2D
}

/// C: mju_flexFaceNormal2D (engine/engine_util_misc.h:124)
/// Calls: mju_cross, mju_flexDphi, mju_flexPhi, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_face_normal2d(normal: *mut f64, t1: *mut f64, t2: *mut f64, order: i32, xpos_f: *const f64, local: *const f64) {
    todo!() // mju_flexFaceNormal2D
}

/// C: mju_flexPhi (engine/engine_util_misc.h:130)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_phi(s: f64, i: i32, order: i32) -> f64 {
    todo!() // mju_flexPhi
}

/// C: mju_flexDphi (engine/engine_util_misc.h:141)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_dphi(s: f64, i: i32, order: i32) -> f64 {
    todo!() // mju_flexDphi
}

/// C: mju_shellTrackInterior (engine/engine_util_misc.h:151)
/// Calls: mju_copy3, nodeAt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_shell_track_interior(nodexpos: *mut f64, nx: i32, ny: i32, nz: i32) {
    todo!() // mju_shellTrackInterior
}

/// C: mju_shellTFIWeights (engine/engine_util_misc.h:154)
/// Calls: addWeight
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_shell_tfi_weights(nx: i32, ny: i32, nz: i32, i: i32, j: i32, k: i32, w: f64, nb: *mut i32, body: *mut i32, bweight: *mut f64, nodebodyid: *const i32, nstart: i32) {
    todo!() // mju_shellTFIWeights
}

/// C: mju_historyInit (engine/engine_util_misc.h:184)
/// Calls: mju_copy, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_init(buf: *mut f64, n: i32, dim: i32, times: *const f64, values: *const f64, user: f64) {
    todo!() // mju_historyInit
}

/// C: mju_historyInsert (engine/engine_util_misc.h:189)
/// Calls: historyFindIndex, historyPhysicalIndex, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_insert(buf: *mut f64, n: i32, dim: i32, t: f64) -> *mut f64 {
    todo!() // mju_historyInsert
}

/// C: mju_historyRead (engine/engine_util_misc.h:194)
/// Calls: historyFindIndex, historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_read(buf: *const f64, n: i32, dim: i32, res: *mut f64, t: f64, interp: i32) -> *const f64 {
    todo!() // mju_historyRead
}

/// C: mju_encodePyramid (engine/engine_util_misc.h:200)
/// Calls: mju_min
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_pyramid(pyramid: *mut f64, force: *const f64, mu: *const f64, dim: i32) {
    todo!() // mju_encodePyramid
}

/// C: mju_decodePyramid (engine/engine_util_misc.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_pyramid(force: *mut f64, pyramid: *const f64, mu: *const f64, dim: i32) {
    todo!() // mju_decodePyramid
}

/// C: mju_springDamper (engine/engine_util_misc.h:208)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_spring_damper(pos0: f64, vel0: f64, Kp: f64, Kv: f64, dt: f64) -> f64 {
    todo!() // mju_springDamper
}

/// C: mju_outsideBox (engine/engine_util_misc.h:213)
/// Calls: mju_message, mju_mulMatTVec3, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_outside_box(point: *const f64, pos: *const f64, mat: *const f64, size: *const f64, inflate: f64) -> i32 {
    todo!() // mju_outsideBox
}

/// C: mju_printMat (engine/engine_util_misc.h:217)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat(mat: *const f64, nr: i32, nc: i32) {
    todo!() // mju_printMat
}

/// C: mju_printMatSparse (engine/engine_util_misc.h:220)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat_sparse(mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // mju_printMatSparse
}

/// C: mju_min (engine/engine_util_misc.h:225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_min(a: f64, b: f64) -> f64 {
    todo!() // mju_min
}

/// C: mju_max (engine/engine_util_misc.h:228)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_max(a: f64, b: f64) -> f64 {
    todo!() // mju_max
}

/// C: mju_clip (engine/engine_util_misc.h:231)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_clip(x: f64, min: f64, max: f64) -> f64 {
    todo!() // mju_clip
}

/// C: mju_sign (engine/engine_util_misc.h:234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sign(x: f64) -> f64 {
    todo!() // mju_sign
}

/// C: mju_round (engine/engine_util_misc.h:237)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_round(x: f64) -> i32 {
    todo!() // mju_round
}

/// C: mju_type2Str (engine/engine_util_misc.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn mju_type2str(r#type: i32) -> *const i8 {
    todo!() // mju_type2Str
}

/// C: mju_str2Type (engine/engine_util_misc.h:243)
#[allow(unused_variables, non_snake_case)]
pub fn mju_str2type(str: *const i8) -> i32 {
    todo!() // mju_str2Type
}

/// C: mju_isBad (engine/engine_util_misc.h:252)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_bad(x: f64) -> i32 {
    todo!() // mju_isBad
}

/// C: mju_isZero (engine/engine_util_misc.h:255)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero(vec: *const f64, n: i32) -> i32 {
    todo!() // mju_isZero
}

/// C: mju_isZeroByte (engine/engine_util_misc.h:258)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero_byte(vec: *const u8, n: i32) -> i32 {
    todo!() // mju_isZeroByte
}

/// C: mju_zeroInt (engine/engine_util_misc.h:261)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_int(res: *mut i32, n: i32) {
    todo!() // mju_zeroInt
}

/// C: mju_copyInt (engine/engine_util_misc.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_int(res: *mut i32, vec: *const i32, n: i32) {
    todo!() // mju_copyInt
}

/// C: mju_fillInt (engine/engine_util_misc.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fill_int(res: *mut i32, val: i32, n: i32) {
    todo!() // mju_fillInt
}

/// C: mju_standardNormal (engine/engine_util_misc.h:270)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_standard_normal(num2: *mut f64) -> f64 {
    todo!() // mju_standardNormal
}

/// C: mju_f2n (engine/engine_util_misc.h:273)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_f2n(res: *mut f64, vec: *const f32, n: i32) {
    todo!() // mju_f2n
}

/// C: mju_n2f (engine/engine_util_misc.h:276)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_n2f(res: *mut f32, vec: *const f64, n: i32) {
    todo!() // mju_n2f
}

/// C: mju_d2n (engine/engine_util_misc.h:279)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_d2n(res: *mut f64, vec: *const f64, n: i32) {
    todo!() // mju_d2n
}

/// C: mju_n2d (engine/engine_util_misc.h:282)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_n2d(res: *mut f64, vec: *const f64, n: i32) {
    todo!() // mju_n2d
}

/// C: mju_gather (engine/engine_util_misc.h:285)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // mju_gather
}

/// C: mju_gatherMasked (engine/engine_util_misc.h:288)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_masked(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // mju_gatherMasked
}

/// C: mju_scatter (engine/engine_util_misc.h:291)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scatter(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // mju_scatter
}

/// C: mju_gatherInt (engine/engine_util_misc.h:294)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    todo!() // mju_gatherInt
}

/// C: mju_scatterInt (engine/engine_util_misc.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scatter_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    todo!() // mju_scatterInt
}

/// C: mju_sparseMap (engine/engine_util_misc.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sparse_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32) {
    todo!() // mju_sparseMap
}

/// C: mju_lower2SymMap (engine/engine_util_misc.h:306)
/// Calls: mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_lower2sym_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32, cursor: *mut i32) {
    todo!() // mju_lower2SymMap
}

/// C: mju_insertionSort (engine/engine_util_misc.h:312)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort(list: *mut f64, n: i32) {
    todo!() // mju_insertionSort
}

/// C: mju_insertionSortInt (engine/engine_util_misc.h:315)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort_int(list: *mut i32, n: i32) {
    todo!() // mju_insertionSortInt
}

/// C: mju_Halton (engine/engine_util_misc.h:318)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_halton(index: i32, base: i32) -> f64 {
    todo!() // mju_Halton
}

/// C: mju_strncpy (engine/engine_util_misc.h:321)
#[allow(unused_variables, non_snake_case)]
pub fn mju_strncpy(dst: *mut i8, src: *const i8, n: i32) -> *mut i8 {
    todo!() // mju_strncpy
}

/// C: mju_polyForce (engine/engine_util_misc.h:326)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    todo!() // mju_polyForce
}

/// C: mjd_xPolyForce (engine/engine_util_misc.h:329)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_x_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    todo!() // mjd_xPolyForce
}

/// C: mju_polyPotential (engine/engine_util_misc.h:332)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_potential(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    todo!() // mju_polyPotential
}

/// C: mju_sigmoid (engine/engine_util_misc.h:335)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sigmoid(x: f64) -> f64 {
    todo!() // mju_sigmoid
}

