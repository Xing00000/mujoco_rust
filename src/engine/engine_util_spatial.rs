//! Port of: engine/engine_util_spatial.h
//! IR hash: 545f394232195ad9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_rotVecQuat (engine/engine_util_spatial.h:27)
/// Calls: mji_copy3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, quat : * const f64)
    // Previous return: ()
    unsafe { if * vec . add (0) == 0.0 && * vec . add (1) == 0.0 && * vec . add (2) == 0.0 { crate :: engine :: engine_util_blas :: mju_zero3 (res) ; } else if * quat . add (0) == 1.0 && * quat . add (1) == 0.0 && * quat . add (2) == 0.0 && * quat . add (3) == 0.0 { crate :: engine :: engine_util_blas :: mju_copy3 (res , vec) ; } else { let mut tmp : [f64 ; 3] = [0.0 ; 3] ; tmp [0] = * quat . add (0) * * vec . add (0) + * quat . add (2) * * vec . add (2) - * quat . add (3) * * vec . add (1) ; tmp [1] = * quat . add (0) * * vec . add (1) + * quat . add (3) * * vec . add (0) - * quat . add (1) * * vec . add (2) ; tmp [2] = * quat . add (0) * * vec . add (2) + * quat . add (1) * * vec . add (1) - * quat . add (2) * * vec . add (0) ; * res . add (0) = * vec . add (0) + 2.0 * (* quat . add (2) * tmp [2] - * quat . add (3) * tmp [1]) ; * res . add (1) = * vec . add (1) + 2.0 * (* quat . add (3) * tmp [0] - * quat . add (1) * tmp [2]) ; * res . add (2) = * vec . add (2) + 2.0 * (* quat . add (1) * tmp [1] - * quat . add (2) * tmp [0]) ; } }
}

/// C: mju_negQuat (engine/engine_util_spatial.h:30)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_neg_quat(res: *mut f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, quat : * const f64)
    // Previous return: ()
    unsafe { * res . add (0) = * quat . add (0) ; * res . add (1) = - * quat . add (1) ; * res . add (2) = - * quat . add (2) ; * res . add (3) = - * quat . add (3) ; }
}

/// C: mju_mulQuat (engine/engine_util_spatial.h:33)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_quat(res: *mut f64, quat1: *const f64, quat2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, quat1 : * const f64, quat2 : * const f64)
    // Previous return: ()
    unsafe { let mut tmp : [f64 ; 4] = [0.0 ; 4] ; tmp [0] = * quat1 . add (0) * * quat2 . add (0) - * quat1 . add (1) * * quat2 . add (1) - * quat1 . add (2) * * quat2 . add (2) - * quat1 . add (3) * * quat2 . add (3) ; tmp [1] = * quat1 . add (0) * * quat2 . add (1) + * quat1 . add (1) * * quat2 . add (0) + * quat1 . add (2) * * quat2 . add (3) - * quat1 . add (3) * * quat2 . add (2) ; tmp [2] = * quat1 . add (0) * * quat2 . add (2) - * quat1 . add (1) * * quat2 . add (3) + * quat1 . add (2) * * quat2 . add (0) + * quat1 . add (3) * * quat2 . add (1) ; tmp [3] = * quat1 . add (0) * * quat2 . add (3) + * quat1 . add (1) * * quat2 . add (2) - * quat1 . add (2) * * quat2 . add (1) + * quat1 . add (3) * * quat2 . add (0) ; * res . add (0) = tmp [0] ; * res . add (1) = tmp [1] ; * res . add (2) = tmp [2] ; * res . add (3) = tmp [3] ; }
}

/// C: mju_mulQuatAxis (engine/engine_util_spatial.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_quat_axis(res: *mut f64, quat: *const f64, axis: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, quat : * const f64, axis : * const f64)
    // Previous return: ()
    unsafe { let mut tmp : [f64 ; 4] = [0.0 ; 4] ; tmp [0] = - * quat . add (1) * * axis . add (0) - * quat . add (2) * * axis . add (1) - * quat . add (3) * * axis . add (2) ; tmp [1] = * quat . add (0) * * axis . add (0) + * quat . add (2) * * axis . add (2) - * quat . add (3) * * axis . add (1) ; tmp [2] = * quat . add (0) * * axis . add (1) + * quat . add (3) * * axis . add (0) - * quat . add (1) * * axis . add (2) ; tmp [3] = * quat . add (0) * * axis . add (2) + * quat . add (1) * * axis . add (1) - * quat . add (2) * * axis . add (0) ; * res . add (0) = tmp [0] ; * res . add (1) = tmp [1] ; * res . add (2) = tmp [2] ; * res . add (3) = tmp [3] ; }
}

/// C: mju_axisAngle2Quat (engine/engine_util_spatial.h:39)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, axis : * const f64, angle : f64)
    // Previous return: ()
    unsafe { if angle == 0.0 { * res . add (0) = 1.0 ; * res . add (1) = 0.0 ; * res . add (2) = 0.0 ; * res . add (3) = 0.0 ; } else { let s : f64 = (angle * 0.5) . sin () ; * res . add (0) = (angle * 0.5) . cos () ; * res . add (1) = * axis . add (0) * s ; * res . add (2) = * axis . add (1) * s ; * res . add (3) = * axis . add (2) * s ; } }
}

/// C: mju_quat2Vel (engine/engine_util_spatial.h:42)
/// Calls: mji_scl3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat2vel(res: *mut f64, quat: *const f64, dt: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, quat : * const f64, dt : f64)
    // Previous return: ()
    unsafe { let mut axis : [f64 ; 3] = [* quat . add (1) , * quat . add (2) , * quat . add (3)] ; let sin_a_2 : f64 = crate :: engine :: engine_util_blas :: mju_normalize3 (axis . as_mut_ptr ()) ; let mut speed : f64 = 2.0 * sin_a_2 . atan2 (* quat . add (0)) ; if speed > std :: f64 :: consts :: PI { speed -= 2.0 * std :: f64 :: consts :: PI ; } speed /= dt ; crate :: engine :: engine_util_blas :: mju_scl3 (res , axis . as_ptr () , speed) ; }
}

/// C: mju_subQuat (engine/engine_util_spatial.h:45)
/// Calls: mji_mulQuat, mji_negQuat, mji_quat2Vel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_quat(res: *mut f64, qa: *const f64, qb: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, qa : * const f64, qb : * const f64)
    // Previous return: ()
    unsafe { let mut qneg : [f64 ; 4] = [0.0 ; 4] ; let mut qdif : [f64 ; 4] = [0.0 ; 4] ; mju_neg_quat (qneg . as_mut_ptr () , qb) ; mju_mul_quat (qdif . as_mut_ptr () , qneg . as_ptr () , qa) ; mju_quat2vel (res , qdif . as_ptr () , 1.0) ; }
}

/// C: mju_quat2Mat (engine/engine_util_spatial.h:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat2mat(res: *mut f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, quat : * const f64)
    // Previous return: ()
    unsafe { if * quat . add (0) == 1.0 && * quat . add (1) == 0.0 && * quat . add (2) == 0.0 && * quat . add (3) == 0.0 { * res . add (0) = 1.0 ; * res . add (1) = 0.0 ; * res . add (2) = 0.0 ; * res . add (3) = 0.0 ; * res . add (4) = 1.0 ; * res . add (5) = 0.0 ; * res . add (6) = 0.0 ; * res . add (7) = 0.0 ; * res . add (8) = 1.0 ; } else { let q00 : f64 = * quat . add (0) * * quat . add (0) ; let q01 : f64 = * quat . add (0) * * quat . add (1) ; let q02 : f64 = * quat . add (0) * * quat . add (2) ; let q03 : f64 = * quat . add (0) * * quat . add (3) ; let q11 : f64 = * quat . add (1) * * quat . add (1) ; let q12 : f64 = * quat . add (1) * * quat . add (2) ; let q13 : f64 = * quat . add (1) * * quat . add (3) ; let q22 : f64 = * quat . add (2) * * quat . add (2) ; let q23 : f64 = * quat . add (2) * * quat . add (3) ; let q33 : f64 = * quat . add (3) * * quat . add (3) ; * res . add (0) = q00 + q11 - q22 - q33 ; * res . add (4) = q00 - q11 + q22 - q33 ; * res . add (8) = q00 - q11 - q22 + q33 ; * res . add (1) = 2.0 * (q12 - q03) ; * res . add (2) = 2.0 * (q13 + q02) ; * res . add (3) = 2.0 * (q12 + q03) ; * res . add (5) = 2.0 * (q23 - q01) ; * res . add (6) = 2.0 * (q13 - q02) ; * res . add (7) = 2.0 * (q23 + q01) ; } }
}

/// C: mju_mat2Quat (engine/engine_util_spatial.h:51)
/// Calls: mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mat2quat(quat: *mut f64, mat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, mat : * const f64)
    // Previous return: ()
    unsafe { if * mat . add (0) + * mat . add (4) + * mat . add (8) > 0.0 { * quat . add (0) = 0.5 * (1.0 + * mat . add (0) + * mat . add (4) + * mat . add (8)) . sqrt () ; * quat . add (1) = 0.25 * (* mat . add (7) - * mat . add (5)) / * quat . add (0) ; * quat . add (2) = 0.25 * (* mat . add (2) - * mat . add (6)) / * quat . add (0) ; * quat . add (3) = 0.25 * (* mat . add (3) - * mat . add (1)) / * quat . add (0) ; } else if * mat . add (0) > * mat . add (4) && * mat . add (0) > * mat . add (8) { * quat . add (1) = 0.5 * (1.0 + * mat . add (0) - * mat . add (4) - * mat . add (8)) . sqrt () ; * quat . add (0) = 0.25 * (* mat . add (7) - * mat . add (5)) / * quat . add (1) ; * quat . add (2) = 0.25 * (* mat . add (1) + * mat . add (3)) / * quat . add (1) ; * quat . add (3) = 0.25 * (* mat . add (2) + * mat . add (6)) / * quat . add (1) ; } else if * mat . add (4) > * mat . add (8) { * quat . add (2) = 0.5 * (1.0 - * mat . add (0) + * mat . add (4) - * mat . add (8)) . sqrt () ; * quat . add (0) = 0.25 * (* mat . add (2) - * mat . add (6)) / * quat . add (2) ; * quat . add (1) = 0.25 * (* mat . add (1) + * mat . add (3)) / * quat . add (2) ; * quat . add (3) = 0.25 * (* mat . add (5) + * mat . add (7)) / * quat . add (2) ; } else { * quat . add (3) = 0.5 * (1.0 - * mat . add (0) - * mat . add (4) + * mat . add (8)) . sqrt () ; * quat . add (0) = 0.25 * (* mat . add (3) - * mat . add (1)) / * quat . add (3) ; * quat . add (1) = 0.25 * (* mat . add (2) + * mat . add (6)) / * quat . add (3) ; * quat . add (2) = 0.25 * (* mat . add (5) + * mat . add (7)) / * quat . add (3) ; } crate :: engine :: engine_util_blas :: mju_normalize4 (quat) ; }
}

/// C: mju_derivQuat (engine/engine_util_spatial.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_deriv_quat(res: *mut f64, quat: *const f64, vel: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, quat : * const f64, vel : * const f64)
    // Previous return: ()
    unsafe { * res . add (0) = 0.5 * (- * vel . add (0) * * quat . add (1) - * vel . add (1) * * quat . add (2) - * vel . add (2) * * quat . add (3)) ; * res . add (1) = 0.5 * (* vel . add (0) * * quat . add (0) + * vel . add (1) * * quat . add (3) - * vel . add (2) * * quat . add (2)) ; * res . add (2) = 0.5 * (- * vel . add (0) * * quat . add (3) + * vel . add (1) * * quat . add (0) + * vel . add (2) * * quat . add (1)) ; * res . add (3) = 0.5 * (* vel . add (0) * * quat . add (2) - * vel . add (1) * * quat . add (1) + * vel . add (2) * * quat . add (0)) ; }
}

/// C: mju_quatIntegrate (engine/engine_util_spatial.h:57)
/// Calls: mji_axisAngle2Quat, mji_copy3, mju_mulQuat, mju_normalize3, mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat_integrate(quat: *mut f64, vel: *const f64, scale: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, vel : * const f64, scale : f64)
    // Previous return: ()
    unsafe { let mut tmp : [f64 ; 4] = [0.0 ; 4] ; let mut qrot : [f64 ; 4] = [0.0 ; 4] ; crate :: engine :: engine_util_blas :: mju_copy3 (tmp . as_mut_ptr () , vel) ; let angle : f64 = scale * crate :: engine :: engine_util_blas :: mju_normalize3 (tmp . as_mut_ptr ()) ; mju_axis_angle2quat (qrot . as_mut_ptr () , tmp . as_ptr () , angle) ; crate :: engine :: engine_util_blas :: mju_normalize4 (quat) ; let quat_copy : [f64 ; 4] = [* quat . add (0) , * quat . add (1) , * quat . add (2) , * quat . add (3)] ; mju_mul_quat (quat , quat_copy . as_ptr () , qrot . as_ptr ()) ; }
}

/// C: mju_quatZ2Vec (engine/engine_util_spatial.h:60)
/// Calls: mji_axisAngle2Quat, mji_cross, mju_dot3, mju_normalize3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat_z2vec(quat: *mut f64, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, vec : * const f64)
    // Previous return: ()
    unsafe { let mut axis : [f64 ; 3] = [0.0 ; 3] ; let mut vn : [f64 ; 3] = [* vec . add (0) , * vec . add (1) , * vec . add (2)] ; let z : [f64 ; 3] = [0.0 , 0.0 , 1.0] ; * quat . add (0) = 1.0 ; crate :: engine :: engine_util_blas :: mju_zero3 (quat . add (1)) ; if crate :: engine :: engine_util_blas :: mju_normalize3 (vn . as_mut_ptr ()) < 1e-15_f64 { return ; } mju_cross (axis . as_mut_ptr () , z . as_ptr () , vn . as_ptr ()) ; let mut a : f64 = crate :: engine :: engine_util_blas :: mju_normalize3 (axis . as_mut_ptr ()) ; if a . abs () < 1e-15_f64 { if crate :: engine :: engine_util_blas :: mju_dot3 (vn . as_ptr () , z . as_ptr ()) < 0.0 { * quat . add (0) = 0.0 ; * quat . add (1) = 1.0 ; } return ; } a = a . atan2 (crate :: engine :: engine_util_blas :: mju_dot3 (vn . as_ptr () , z . as_ptr ())) ; mju_axis_angle2quat (quat , axis . as_ptr () , a) ; }
}

/// C: mju_mat2Rot (engine/engine_util_spatial.h:64)
/// Calls: mji_add3, mji_addTo3, mji_axisAngle2Quat, mji_cross, mju_dot3, mju_mulQuat, mju_normalize3, mju_normalize4, mju_quat2Mat, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mat2rot(quat: *mut f64, mat: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, mat : * const f64)
    // Previous return: i32
    unsafe { const ROT_EPS : f64 = 1e-9 ; let mut iter : i32 = 0 ; let col1_mat : [f64 ; 3] = [* mat . add (0) , * mat . add (3) , * mat . add (6)] ; let col2_mat : [f64 ; 3] = [* mat . add (1) , * mat . add (4) , * mat . add (7)] ; let col3_mat : [f64 ; 3] = [* mat . add (2) , * mat . add (5) , * mat . add (8)] ; while iter < 500 { let mut rot : [f64 ; 9] = [0.0 ; 9] ; mju_quat2mat (rot . as_mut_ptr () , quat as * const f64) ; let col1_rot : [f64 ; 3] = [rot [0] , rot [3] , rot [6]] ; let col2_rot : [f64 ; 3] = [rot [1] , rot [4] , rot [7]] ; let col3_rot : [f64 ; 3] = [rot [2] , rot [5] , rot [8]] ; let mut omega : [f64 ; 3] = [0.0 ; 3] ; let mut vec1 : [f64 ; 3] = [0.0 ; 3] ; let mut vec2 : [f64 ; 3] = [0.0 ; 3] ; let mut vec3 : [f64 ; 3] = [0.0 ; 3] ; mju_cross (vec1 . as_mut_ptr () , col1_rot . as_ptr () , col1_mat . as_ptr ()) ; mju_cross (vec2 . as_mut_ptr () , col2_rot . as_ptr () , col2_mat . as_ptr ()) ; mju_cross (vec3 . as_mut_ptr () , col3_rot . as_ptr () , col3_mat . as_ptr ()) ; omega [0] = vec1 [0] + vec2 [0] ; omega [1] = vec1 [1] + vec2 [1] ; omega [2] = vec1 [2] + vec2 [2] ; omega [0] += vec3 [0] ; omega [1] += vec3 [1] ; omega [2] += vec3 [2] ; let denom : f64 = (crate :: engine :: engine_util_blas :: mju_dot3 (col1_rot . as_ptr () , col1_mat . as_ptr ()) + crate :: engine :: engine_util_blas :: mju_dot3 (col2_rot . as_ptr () , col2_mat . as_ptr ()) + crate :: engine :: engine_util_blas :: mju_dot3 (col3_rot . as_ptr () , col3_mat . as_ptr ())) . abs () + 1e-15_f64 ; crate :: engine :: engine_util_blas :: mju_scl3 (omega . as_mut_ptr () , omega . as_ptr () , 1.0 / denom) ; let w : f64 = crate :: engine :: engine_util_blas :: mju_normalize3 (omega . as_mut_ptr ()) ; if w < ROT_EPS { break ; } let mut qrot : [f64 ; 4] = [0.0 ; 4] ; mju_axis_angle2quat (qrot . as_mut_ptr () , omega . as_ptr () , w) ; let quat_copy : [f64 ; 4] = [* quat . add (0) , * quat . add (1) , * quat . add (2) , * quat . add (3)] ; mju_mul_quat (quat , qrot . as_ptr () , quat_copy . as_ptr ()) ; crate :: engine :: engine_util_blas :: mju_normalize4 (quat) ; iter += 1 ; } iter }
}

/// C: mju_mulPose (engine/engine_util_spatial.h:70)
/// Calls: mji_addTo3, mji_mulQuat, mji_rotVecQuat, mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_pose(posres: *mut f64, quatres: *mut f64, pos1: *const f64, quat1: *const f64, pos2: *const f64, quat2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (posres : * mut f64, quatres : * mut f64, pos1 : * const f64, quat1 : * const f64, pos2 : * const f64, quat2 : * const f64)
    // Previous return: ()
    unsafe { mju_mul_quat (quatres , quat1 , quat2) ; crate :: engine :: engine_util_blas :: mju_normalize4 (quatres) ; mju_rot_vec_quat (posres , pos2 , quat1) ; * posres . add (0) += * pos1 . add (0) ; * posres . add (1) += * pos1 . add (1) ; * posres . add (2) += * pos1 . add (2) ; }
}

/// C: mju_negPose (engine/engine_util_spatial.h:75)
/// Calls: mji_negQuat, mji_rotVecQuat, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_neg_pose(posres: *mut f64, quatres: *mut f64, pos: *const f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (posres : * mut f64, quatres : * mut f64, pos : * const f64, quat : * const f64)
    // Previous return: ()
    unsafe { mju_neg_quat (quatres , quat) ; mju_rot_vec_quat (posres , pos , quatres as * const f64) ; crate :: engine :: engine_util_blas :: mju_scl3 (posres , posres as * const f64 , - 1.0) ; }
}

/// C: mju_trnVecPose (engine/engine_util_spatial.h:79)
/// Calls: mji_addTo3, mji_rotVecQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_trn_vec_pose(res: *mut f64, pos: *const f64, quat: *const f64, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, pos : * const f64, quat : * const f64, vec : * const f64)
    // Previous return: ()
    unsafe { mju_rot_vec_quat (res , vec , quat) ; * res . add (0) += * pos . add (0) ; * res . add (1) += * pos . add (1) ; * res . add (2) += * pos . add (2) ; }
}

/// C: mju_euler2Quat (engine/engine_util_spatial.h:84)
/// Calls: mji_copy4, mju_message, mju_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_euler2quat(quat: *mut f64, euler: *const f64, seq: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, euler : * const f64, seq : * const i8)
    // Previous return: ()
    unsafe { let mut len : usize = 0 ; while len < 4 && * seq . add (len) != 0 { len += 1 ; } if len != 3 { crate :: engine :: engine_util_errmem :: mju_error (b"seq must contain exactly 3 characters\0" . as_ptr () as * const i8 ,) ; } let mut tmp : [f64 ; 4] = [1.0 , 0.0 , 0.0 , 0.0] ; for i in 0 .. 3_usize { let mut rot : [f64 ; 4] = [(* euler . add (i) / 2.0) . cos () , 0.0 , 0.0 , 0.0] ; let sa : f64 = (* euler . add (i) / 2.0) . sin () ; let c : i8 = * seq . add (i) ; if c == b'x' as i8 || c == b'X' as i8 { rot [1] = sa ; } else if c == b'y' as i8 || c == b'Y' as i8 { rot [2] = sa ; } else if c == b'z' as i8 || c == b'Z' as i8 { rot [3] = sa ; } else { crate :: engine :: engine_util_errmem :: mju_error (b"seq element should be one of x, y, z, X, Y, Z\0" . as_ptr () as * const i8 ,) ; } if c == b'x' as i8 || c == b'y' as i8 || c == b'z' as i8 { let tmp_copy : [f64 ; 4] = [tmp [0] , tmp [1] , tmp [2] , tmp [3]] ; mju_mul_quat (tmp . as_mut_ptr () , tmp_copy . as_ptr () , rot . as_ptr ()) ; } else { let tmp_copy : [f64 ; 4] = [tmp [0] , tmp [1] , tmp [2] , tmp [3]] ; mju_mul_quat (tmp . as_mut_ptr () , rot . as_ptr () , tmp_copy . as_ptr ()) ; } } * quat . add (0) = tmp [0] ; * quat . add (1) = tmp [1] ; * quat . add (2) = tmp [2] ; * quat . add (3) = tmp [3] ; }
}

/// C: mju_cross (engine/engine_util_spatial.h:89)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross(res: *mut f64, a: *const f64, b: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, a : * const f64, b : * const f64)
    // Previous return: ()
    unsafe { let mut tmp : [f64 ; 3] = [0.0 ; 3] ; tmp [0] = * a . add (1) * * b . add (2) - * a . add (2) * * b . add (1) ; tmp [1] = * a . add (2) * * b . add (0) - * a . add (0) * * b . add (2) ; tmp [2] = * a . add (0) * * b . add (1) - * a . add (1) * * b . add (0) ; * res . add (0) = tmp [0] ; * res . add (1) = tmp [1] ; * res . add (2) = tmp [2] ; }
}

/// C: mju_crossMotion (engine/engine_util_spatial.h:92)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross_motion(res: *mut f64, vel: *const f64, v: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vel : * const f64, v : * const f64)
    // Previous return: ()
    unsafe { * res . add (0) = - * vel . add (2) * * v . add (1) + * vel . add (1) * * v . add (2) ; * res . add (1) = * vel . add (2) * * v . add (0) - * vel . add (0) * * v . add (2) ; * res . add (2) = - * vel . add (1) * * v . add (0) + * vel . add (0) * * v . add (1) ; * res . add (3) = - * vel . add (2) * * v . add (4) + * vel . add (1) * * v . add (5) ; * res . add (4) = * vel . add (2) * * v . add (3) - * vel . add (0) * * v . add (5) ; * res . add (5) = - * vel . add (1) * * v . add (3) + * vel . add (0) * * v . add (4) ; * res . add (3) += - * vel . add (5) * * v . add (1) + * vel . add (4) * * v . add (2) ; * res . add (4) += * vel . add (5) * * v . add (0) - * vel . add (3) * * v . add (2) ; * res . add (5) += - * vel . add (4) * * v . add (0) + * vel . add (3) * * v . add (1) ; }
}

/// C: mju_crossForce (engine/engine_util_spatial.h:95)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross_force(res: *mut f64, vel: *const f64, f: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vel : * const f64, f : * const f64)
    // Previous return: ()
    unsafe { * res . add (0) = - * vel . add (2) * * f . add (1) + * vel . add (1) * * f . add (2) ; * res . add (1) = * vel . add (2) * * f . add (0) - * vel . add (0) * * f . add (2) ; * res . add (2) = - * vel . add (1) * * f . add (0) + * vel . add (0) * * f . add (1) ; * res . add (3) = - * vel . add (2) * * f . add (4) + * vel . add (1) * * f . add (5) ; * res . add (4) = * vel . add (2) * * f . add (3) - * vel . add (0) * * f . add (5) ; * res . add (5) = - * vel . add (1) * * f . add (3) + * vel . add (0) * * f . add (4) ; * res . add (0) += - * vel . add (5) * * f . add (4) + * vel . add (4) * * f . add (5) ; * res . add (1) += * vel . add (5) * * f . add (3) - * vel . add (3) * * f . add (5) ; * res . add (2) += - * vel . add (4) * * f . add (3) + * vel . add (3) * * f . add (4) ; }
}

/// C: mju_inertCom (engine/engine_util_spatial.h:98)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_inert_com(res: *mut f64, inert: *const f64, mat: *const f64, dif: *const f64, mass: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, inert : * const f64, mat : * const f64, dif : * const f64, mass : f64)
    // Previous return: ()
    unsafe { let mut tmp : [f64 ; 9] = [0.0 ; 9] ; tmp [0] = * mat . add (0) * * inert . add (0) ; tmp [1] = * mat . add (3) * * inert . add (0) ; tmp [2] = * mat . add (6) * * inert . add (0) ; tmp [3] = * mat . add (1) * * inert . add (1) ; tmp [4] = * mat . add (4) * * inert . add (1) ; tmp [5] = * mat . add (7) * * inert . add (1) ; tmp [6] = * mat . add (2) * * inert . add (2) ; tmp [7] = * mat . add (5) * * inert . add (2) ; tmp [8] = * mat . add (8) * * inert . add (2) ; * res . add (0) = * mat . add (0) * tmp [0] + * mat . add (1) * tmp [3] + * mat . add (2) * tmp [6] ; * res . add (1) = * mat . add (3) * tmp [1] + * mat . add (4) * tmp [4] + * mat . add (5) * tmp [7] ; * res . add (2) = * mat . add (6) * tmp [2] + * mat . add (7) * tmp [5] + * mat . add (8) * tmp [8] ; * res . add (3) = * mat . add (0) * tmp [1] + * mat . add (1) * tmp [4] + * mat . add (2) * tmp [7] ; * res . add (4) = * mat . add (0) * tmp [2] + * mat . add (1) * tmp [5] + * mat . add (2) * tmp [8] ; * res . add (5) = * mat . add (3) * tmp [2] + * mat . add (4) * tmp [5] + * mat . add (5) * tmp [8] ; * res . add (0) += mass * (* dif . add (1) * * dif . add (1) + * dif . add (2) * * dif . add (2)) ; * res . add (1) += mass * (* dif . add (0) * * dif . add (0) + * dif . add (2) * * dif . add (2)) ; * res . add (2) += mass * (* dif . add (0) * * dif . add (0) + * dif . add (1) * * dif . add (1)) ; * res . add (3) -= mass * * dif . add (0) * * dif . add (1) ; * res . add (4) -= mass * * dif . add (0) * * dif . add (2) ; * res . add (5) -= mass * * dif . add (1) * * dif . add (2) ; * res . add (6) = mass * * dif . add (0) ; * res . add (7) = mass * * dif . add (1) ; * res . add (8) = mass * * dif . add (2) ; * res . add (9) = mass ; }
}

/// C: mju_dofCom (engine/engine_util_spatial.h:102)
/// Calls: mji_copy3, mji_cross, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dof_com(res: *mut f64, axis: *const f64, offset: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, axis : * const f64, offset : * const f64)
    // Previous return: ()
    unsafe { if ! offset . is_null () { crate :: engine :: engine_util_blas :: mju_copy3 (res , axis) ; mju_cross (res . add (3) , axis , offset) ; } else { crate :: engine :: engine_util_blas :: mju_zero3 (res) ; crate :: engine :: engine_util_blas :: mju_copy3 (res . add (3) , axis) ; } }
}

/// C: mju_mulInertVec (engine/engine_util_spatial.h:105)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_inert_vec(res: *mut f64, inert: *const f64, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, inert : * const f64, vec : * const f64)
    // Previous return: ()
    unsafe { * res . add (0) = * inert . add (0) * * vec . add (0) + * inert . add (3) * * vec . add (1) + * inert . add (4) * * vec . add (2) - * inert . add (8) * * vec . add (4) + * inert . add (7) * * vec . add (5) ; * res . add (1) = * inert . add (3) * * vec . add (0) + * inert . add (1) * * vec . add (1) + * inert . add (5) * * vec . add (2) + * inert . add (8) * * vec . add (3) - * inert . add (6) * * vec . add (5) ; * res . add (2) = * inert . add (4) * * vec . add (0) + * inert . add (5) * * vec . add (1) + * inert . add (2) * * vec . add (2) - * inert . add (7) * * vec . add (3) + * inert . add (6) * * vec . add (4) ; * res . add (3) = * inert . add (8) * * vec . add (1) - * inert . add (7) * * vec . add (2) + * inert . add (9) * * vec . add (3) ; * res . add (4) = * inert . add (6) * * vec . add (2) - * inert . add (8) * * vec . add (0) + * inert . add (9) * * vec . add (4) ; * res . add (5) = * inert . add (7) * * vec . add (0) - * inert . add (6) * * vec . add (1) + * inert . add (9) * * vec . add (5) ; }
}

/// C: mju_mulDofVec (engine/engine_util_spatial.h:108)
/// Calls: mju_mulMatTVec, mju_scl, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_dof_vec(res: *mut f64, mat: *const f64, vec: *const f64, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, n : i32)
    // Previous return: ()
    unsafe { if n == 1 { crate :: engine :: engine_util_blas :: mju_scl (res , mat , * vec . add (0) , 6) ; } else if n <= 0 { crate :: engine :: engine_util_blas :: mju_zero (res , 6) ; } else { crate :: engine :: engine_util_blas :: mju_mul_mat_t_vec (res , mat , vec , n , 6) ; } }
}

/// C: mju_transformSpatial (engine/engine_util_spatial.h:112)
/// Calls: mji_copy6, mji_cross, mji_mulMatTVec3, mji_sub3, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_transform_spatial(res: *mut f64, vec: *const f64, flg_force: i32, newpos: *const f64, oldpos: *const f64, rotnew2old: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, flg_force : i32, newpos : * const f64, oldpos : * const f64, rotnew2old : * const f64)
    // Previous return: ()
    unsafe { let mut cros : [f64 ; 3] = [0.0 ; 3] ; let mut dif : [f64 ; 3] = [0.0 ; 3] ; let mut tran : [f64 ; 6] = [0.0 ; 6] ; crate :: engine :: engine_util_blas :: mju_copy (tran . as_mut_ptr () , vec , 6) ; crate :: engine :: engine_util_blas :: mju_sub3 (dif . as_mut_ptr () , newpos , oldpos) ; if flg_force != 0 { mju_cross (cros . as_mut_ptr () , dif . as_ptr () , vec . add (3)) ; crate :: engine :: engine_util_blas :: mju_sub3 (tran . as_mut_ptr () , vec , cros . as_ptr ()) ; } else { mju_cross (cros . as_mut_ptr () , dif . as_ptr () , vec) ; crate :: engine :: engine_util_blas :: mju_sub3 (tran . as_mut_ptr () . add (3) , vec . add (3) , cros . as_ptr ()) ; } if ! rotnew2old . is_null () { crate :: engine :: engine_util_blas :: mju_mul_mat_t_vec3 (res , rotnew2old , tran . as_ptr ()) ; crate :: engine :: engine_util_blas :: mju_mul_mat_t_vec3 (res . add (3) , rotnew2old , tran . as_ptr () . add (3)) ; } else { * res . add (0) = tran [0] ; * res . add (1) = tran [1] ; * res . add (2) = tran [2] ; * res . add (3) = tran [3] ; * res . add (4) = tran [4] ; * res . add (5) = tran [5] ; } }
}

/// C: mju_makeFrame (engine/engine_util_spatial.h:117)
/// Calls: mji_cross, mji_scl3, mji_subFrom3, mju_dot3, mju_message, mju_normalize3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_make_frame(frame: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (frame : * mut f64)
    // Previous return: ()
    unsafe { let mut tmp : [f64 ; 3] = [0.0 ; 3] ; if crate :: engine :: engine_util_blas :: mju_normalize3 (frame) < 0.5 { crate :: engine :: engine_util_errmem :: mju_error (b"xaxis of contact frame undefined\0" . as_ptr () as * const i8 ,) ; } if crate :: engine :: engine_util_blas :: mju_dot3 (frame . add (3) as * const f64 , frame . add (3) as * const f64) < 0.25 { crate :: engine :: engine_util_blas :: mju_zero3 (frame . add (3)) ; if * frame . add (1) < 0.5 && * frame . add (1) > - 0.5 { * frame . add (4) = 1.0 ; } else { * frame . add (5) = 1.0 ; } } let dot : f64 = crate :: engine :: engine_util_blas :: mju_dot3 (frame as * const f64 , frame . add (3) as * const f64) ; crate :: engine :: engine_util_blas :: mju_scl3 (tmp . as_mut_ptr () , frame as * const f64 , dot) ; * frame . add (3) -= tmp [0] ; * frame . add (4) -= tmp [1] ; * frame . add (5) -= tmp [2] ; crate :: engine :: engine_util_blas :: mju_normalize3 (frame . add (3)) ; mju_cross (frame . add (6) , frame as * const f64 , frame . add (3) as * const f64) ; }
}

