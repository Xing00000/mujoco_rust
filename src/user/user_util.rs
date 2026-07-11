//! Port of: user/user_util.cc
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjuu_axisAngle2Quat (user/user_util.cc:564)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, axis : * const f64, angle : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_isValidContentType (user/user_util.cc:973)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_is_valid_content_type(text: string_view) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (text : string_view)
    // Previous return: bool
    todo ! ()
}

/// C: FileToMemory (user/user_util.cc:1196)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn file_to_memory(filename: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (filename : * const i8)
    // Previous return: i32
    todo ! ()
}

/// C: VectorToString (user/user_util.cc:1256)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string(v: *const i32) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (v : * const i32)
    // Previous return: std__string
    todo ! ()
}

/// C: StrToNum (user/user_util.cc:1277)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num(str: *mut i8, c: *mut *mut i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (str : * mut i8, c : * mut * mut i8)
    // Previous return: i32
    todo ! ()
}

/// C: IsNullOrSpace (user/user_util.cc:1301)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_or_space(c: *mut i8) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (c : * mut i8)
    // Previous return: bool
    todo ! ()
}

/// C: SkipSpace (user/user_util.cc:1305)
/// Calls: IsNullOrSpace
#[allow(unused_variables, non_snake_case)]
pub fn skip_space(c: *mut i8) -> *mut i8 {
    // WARNING: signature changed — verify body
    // Previous params: (c : * mut i8)
    // Previous return: * mut i8
    todo ! ()
}

/// C: StringToVector (user/user_util.cc:1315)
/// Calls: SkipSpace
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector(cs: *mut i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (cs : * mut i8)
    // Previous return: i32
    todo ! ()
}

/// C: mjuu_defined (user/user_util.h:35)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_defined(num: f64) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (num : f64)
    // Previous return: bool
    todo ! ()
}

/// C: mjuu_matadr (user/user_util.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_matadr(g1: i32, g2: i32, n: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (g1 : i32, g2 : i32, n : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mjuu_setvec (user/user_util.h:42)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec(dest: *mut f64, x: f64, y: f64, z: f64, w: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut f64, x : f64, y : f64, z : f64, w : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_copyvec (user/user_util.h:54)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec(dest: *mut T1, src: *const T2, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut T1, src : * const T2, n : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_addtovec (user/user_util.h:59)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_addtovec(dest: *mut f64, src: *const f64, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut f64, src : * const f64, n : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_zerovec (user/user_util.h:62)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_zerovec(dest: *mut f64, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut f64, n : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_dot3 (user/user_util.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dot3(a: *const f64, b: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (a : * const f64, b : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: mjuu_dist3 (user/user_util.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dist3(a: *const f64, b: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (a : * const f64, b : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: mjuu_L1 (user/user_util.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_l1(a: *const f64, b: *const f64, n: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (a : * const f64, b : * const f64, n : i32)
    // Previous return: f64
    todo ! ()
}

/// C: mjuu_normvec (user/user_util.h:78)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_normvec(vec: *mut f64, n: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (vec : * mut f64, n : i32)
    // Previous return: f64
    todo ! ()
}

/// C: mjuu_scalevec (user/user_util.h:82)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_scalevec(res: *mut f64, vec: *const f64, s: f64, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, s : f64, n : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_quat2mat (user/user_util.h:85)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_quat2mat(res: *mut f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, quat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_mulquat (user/user_util.h:88)
/// Calls: mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulquat(res: *mut f64, qa: *const f64, qb: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, qa : * const f64, qb : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_mulvecmat (user/user_util.h:91)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmat(res: *mut f64, vec: *const f64, mat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, mat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_mulvecmatT (user/user_util.h:94)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmat_t(res: *mut f64, vec: *const f64, mat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, mat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_mulRMRT (user/user_util.h:97)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mul_rmrt(res: *mut f64, R: *const f64, M: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, R : * const f64, M : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_mulmat (user/user_util.h:100)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulmat(res: *mut f64, A: *const f64, B: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, A : * const f64, B : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_transposemat (user/user_util.h:103)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_transposemat(res: *mut f64, mat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_localaxis (user/user_util.h:106)
/// Calls: mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localaxis(al: *mut f64, ag: *const f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (al : * mut f64, ag : * const f64, quat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_localpos (user/user_util.h:109)
/// Calls: mjuu_localaxis
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localpos(pl: *mut f64, pg: *const f64, pos: *const f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (pl : * mut f64, pg : * const f64, pos : * const f64, quat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_localquat (user/user_util.h:112)
/// Calls: mjuu_mulquat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localquat(local: *mut f64, child: *const f64, parent: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (local : * mut f64, child : * const f64, parent : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_crossvec (user/user_util.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_crossvec(a: *mut f64, b: *const f64, c: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (a : * mut f64, b : * const f64, c : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_makenormal (user/user_util.h:118)
/// Calls: mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_makenormal(normal: *mut f64, a: *const type_parameter_0_0, b: *const type_parameter_0_0, c: *const type_parameter_0_0) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (normal : * mut f64, a : * const type_parameter_0_0, b : * const type_parameter_0_0, c : * const type_parameter_0_0)
    // Previous return: f64
    todo ! ()
}

/// C: mjuu_z2quat (user/user_util.h:122)
/// Calls: mjuu_crossvec, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_z2quat(quat: *mut f64, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_frame2quat (user/user_util.h:125)
/// Calls: mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frame2quat(quat: *mut f64, x: *const f64, y: *const f64, z: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, x : * const f64, y : * const f64, z : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_frameinvert (user/user_util.h:128)
/// Calls: mjuu_localaxis
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameinvert(newpos: *mut f64, newquat: *mut f64, oldpos: *const f64, oldquat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (newpos : * mut f64, newquat : * mut f64, oldpos : * const f64, oldquat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_frameaccum (user/user_util.h:132)
/// Calls: mjuu_mulquat, mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccum(pos: *mut f64, quat: *mut f64, childpos: *const f64, childquat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (pos : * mut f64, quat : * mut f64, childpos : * const f64, childquat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_frameaccumChild (user/user_util.h:136)
/// Calls: mjuu_frameaccum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccum_child(pos: *const f64, quat: *const f64, childpos: *mut f64, childquat: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, quat : * const f64, childpos : * mut f64, childquat : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_frameaccuminv (user/user_util.h:140)
/// Calls: mjuu_mulquat, mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccuminv(pos: *mut f64, quat: *mut f64, childpos: *const f64, childquat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (pos : * mut f64, quat : * mut f64, childpos : * const f64, childquat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_globalinertia (user/user_util.h:144)
/// Calls: mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_globalinertia(global: *mut f64, local: *const f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (global : * mut f64, local : * const f64, quat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_offcenter (user/user_util.h:147)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_offcenter(res: *mut f64, mass: f64, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mass : f64, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_visccoef (user/user_util.h:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_visccoef(visccoef: *mut f64, mass: f64, inertia: *const f64, scl: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (visccoef : * mut f64, mass : f64, inertia : * const f64, scl : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_rotVecQuat (user/user_util.h:153)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, quat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_updateFrame (user/user_util.h:156)
/// Calls: mjuu_axisAngle2Quat, mjuu_crossvec, mjuu_dot3, mjuu_frame2quat, mjuu_normvec, mjuu_rotVecQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_update_frame(quat: *mut f64, normal: *mut f64, edge: *const f64, tprv: *const f64, tnxt: *const f64, first: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, normal : * mut f64, edge : * const f64, tprv : * const f64, tnxt : * const f64, first : i32)
    // Previous return: f64
    todo ! ()
}

/// C: mjuu_eig3 (user/user_util.h:160)
/// Calls: mjuu_mulmat, mjuu_mulquat, mjuu_normvec, mjuu_quat2mat, mjuu_transposemat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eig3(eigval: *mut f64, eigvec: *mut f64, quat: *mut f64, mat: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (eigval : * mut f64, eigvec : * mut f64, quat : * mut f64, mat : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjuu_eigendecompose (user/user_util.h:166)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eigendecompose(mat: *mut f64, eigval: *mut f64, eigvec: *mut f64, n: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (mat : * mut f64, eigval : * mut f64, eigvec : * mut f64, n : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mjuu_trnVecPose (user/user_util.h:169)
/// Calls: mjuu_rotVecQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_trn_vec_pose(res: *mut f64, pos: *const f64, quat: *const f64, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, pos : * const f64, quat : * const f64, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjuu_fullInertia (user/user_util.h:172)
/// Calls: mjuu_defined, mjuu_eig3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_full_inertia(quat: *mut f64, inertia: *mut f64, fullinertia: *const f64) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, inertia : * mut f64, fullinertia : * const f64)
    // Previous return: * const i8
    todo ! ()
}

/// C: FilePath::IsAbs (user/user_util.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_abs(self_ptr: *mut FilePath) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: bool
    todo ! ()
}

/// C: FilePath::AbsPrefix (user/user_util.h:195)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_abs_prefix(self_ptr: *mut FilePath) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: i32
    todo ! ()
}

/// C: FilePath::Str (user/user_util.h:198)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str(self_ptr: *mut FilePath) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: * const i32
    todo ! ()
}

/// C: FilePath::StrLower (user/user_util.h:202)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str_lower(self_ptr: *mut FilePath) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: std__string
    todo ! ()
}

/// C: FilePath::Ext (user/user_util.h:205)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_ext(self_ptr: *mut FilePath) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: std__string
    todo ! ()
}

/// C: FilePath::StripExt (user/user_util.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_ext(self_ptr: *mut FilePath) -> FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: FilePath
    todo ! ()
}

/// C: FilePath::StripPath (user/user_util.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_path(self_ptr: *mut FilePath) -> FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: FilePath
    todo ! ()
}

/// C: FilePath::Lower (user/user_util.h:217)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_lower(self_ptr: *mut FilePath) -> FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: FilePath
    todo ! ()
}

/// C: FilePath::size (user/user_util.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_size(self_ptr: *mut FilePath) -> std__size_t {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: std__size_t
    todo ! ()
}

/// C: FilePath::c_str (user/user_util.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_c_str(self_ptr: *mut FilePath) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: * const i8
    todo ! ()
}

/// C: FilePath::empty (user/user_util.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_empty(self_ptr: *mut FilePath) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: bool
    todo ! ()
}

/// C: FilePath::PathReduce (user/user_util.h:227)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_path_reduce(str: *const std__string) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const std__string)
    // Previous return: std__string
    todo ! ()
}

/// C: FilePath::IsSeparator (user/user_util.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_separator(c: i8) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (c : i8)
    // Previous return: bool
    todo ! ()
}

/// C: FilePath::Combine (user/user_util.h:231)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_combine(s1: *const std__string, s2: *const std__string) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (s1 : * const std__string, s2 : * const std__string)
    // Previous return: std__string
    todo ! ()
}

/// C: FilePath::FilePathFast (user/user_util.h:234)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_file_path_fast(str: *const i32) -> FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i32)
    // Previous return: FilePath
    todo ! ()
}

/// C: mjuu_strippath (user/user_util.h:273)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_strippath(filename: string) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (filename : string)
    // Previous return: std__string
    todo ! ()
}

/// C: mjuu_stripext (user/user_util.h:276)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_stripext(filename: string) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (filename : string)
    // Previous return: std__string
    todo ! ()
}

/// C: mjuu_getext (user/user_util.h:279)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_getext(filename: string_view) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (filename : string_view)
    // Previous return: std__string
    todo ! ()
}

/// C: mjuu_isabspath (user/user_util.h:282)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_isabspath(path: string) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (path : string)
    // Previous return: bool
    todo ! ()
}

/// C: mjuu_combinePaths (user/user_util.h:285)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_combine_paths(path1: *const std__string, path2: *const std__string) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (path1 : * const std__string, path2 : * const std__string)
    // Previous return: std__string
    todo ! ()
}

/// C: mjuu_parseContentTypeAttrType (user/user_util.h:290)
/// Calls: mjuu_isValidContentType
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_type(text: string_view) -> *const () {
    // WARNING: signature changed — verify body
    // Previous params: (text : string_view)
    // Previous return: * const ()
    todo ! ()
}

/// C: mjuu_parseContentTypeAttrSubtype (user/user_util.h:293)
/// Calls: mjuu_isValidContentType
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_subtype(text: string_view) -> *const () {
    // WARNING: signature changed — verify body
    // Previous params: (text : string_view)
    // Previous return: * const ()
    todo ! ()
}

/// C: mjuu_extToContentType (user/user_util.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_ext_to_content_type(filename: string_view) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (filename : string_view)
    // Previous return: std__string
    todo ! ()
}

/// C: mjuu_dirnamelen (user/user_util.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dirnamelen(path: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (path : * const i8)
    // Previous return: i32
    todo ! ()
}

