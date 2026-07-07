//! Port of: user/user_util.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjuu_axisAngle2Quat (user/user_util.cc:564)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_axis_angle2quat(res: [f64; 4], axis: [f64; 3], angle: f64) {
    // NOTE: In C ABI, array params are pointers. res is the output.
    let res_ptr = res.as_ptr() as *mut f64;

    // zero angle: identity quat
    if angle == 0.0 {
        // SAFETY: res_ptr points to caller-owned memory (C ABI array param = pointer)
        unsafe {
            *res_ptr = 1.0;
            *res_ptr.add(1) = 0.0;
            *res_ptr.add(2) = 0.0;
            *res_ptr.add(3) = 0.0;
        }
    } else {
        // regular processing
        let s = (angle * 0.5).sin();
        // SAFETY: res_ptr points to caller-owned memory (C ABI array param = pointer)
        unsafe {
            *res_ptr = (angle * 0.5).cos();
            *res_ptr.add(1) = axis[0] * s;
            *res_ptr.add(2) = axis[1] * s;
            *res_ptr.add(3) = axis[2] * s;
        }
    }
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
    extern "C" { fn FileToMemory_impl (filename : * const i8) -> i32 ; } unsafe { FileToMemory_impl (filename) }
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
    unsafe { let ch = * c as u8 ; ch == 0 || ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r' || ch == 0x0B || ch == 0x0C }
}

/// C: SkipSpace (user/user_util.cc:1305)
/// Calls: IsNullOrSpace
#[allow(unused_variables, non_snake_case)]
pub fn skip_space(c: *mut i8) -> *mut i8 {
    // WARNING: signature changed — verify body
    // Previous params: (c : * mut i8)
    // Previous return: * mut i8
    unsafe { let mut p = c ; while * p != 0 { if ! is_null_or_space (p) { break ; } p = p . add (1) ; } p }
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
    ! num . is_nan ()
}

/// C: mjuu_matadr (user/user_util.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_matadr(g1: i32, g2: i32, n: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (g1 : i32, g2 : i32, n : i32)
    // Previous return: i32
    let mut g1 = g1 ; let mut g2 = g2 ; if g1 < 0 || g2 < 0 || g1 >= n || g2 >= n { return - 1 ; } if g1 > g2 { let tmp = g1 ; g1 = g2 ; g2 = tmp ; } g1 * n + g2
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
    unsafe { * dest . add (0) = x ; * dest . add (1) = y ; * dest . add (2) = z ; * dest . add (3) = w ; }
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
    unsafe { for i in 0 .. n as usize { * dest . add (i) += * src . add (i) ; } }
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
    unsafe { for i in 0 .. n as usize { * dest . add (i) = 0.0 ; } }
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
    unsafe { * a . add (0) * * b . add (0) + * a . add (1) * * b . add (1) + * a . add (2) * * b . add (2) }
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
    unsafe { let d0 = * a . add (0) - * b . add (0) ; let d1 = * a . add (1) - * b . add (1) ; let d2 = * a . add (2) - * b . add (2) ; (d0 * d0 + d1 * d1 + d2 * d2) . sqrt () }
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
    unsafe { let mut res : f64 = 0.0 ; for i in 0 .. n as usize { res += (* a . add (i) - * b . add (i)) . abs () ; } res }
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
    unsafe { const MJ_EPS : f64 = 1e-14 ; let mut nrm : f64 = 0.0 ; for i in 0 .. n as usize { nrm += * vec . add (i) * * vec . add (i) ; } if nrm < MJ_EPS { return 0.0 ; } nrm = nrm . sqrt () ; if (nrm - 1.0) . abs () > MJ_EPS { for i in 0 .. n as usize { * vec . add (i) /= nrm ; } } nrm }
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
    unsafe { for i in 0 .. n as usize { * res . add (i) = s * * vec . add (i) ; } }
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
    unsafe { if * quat . add (0) == 1.0 && * quat . add (1) == 0.0 && * quat . add (2) == 0.0 && * quat . add (3) == 0.0 { * res . add (0) = 1.0 ; * res . add (1) = 0.0 ; * res . add (2) = 0.0 ; * res . add (3) = 0.0 ; * res . add (4) = 1.0 ; * res . add (5) = 0.0 ; * res . add (6) = 0.0 ; * res . add (7) = 0.0 ; * res . add (8) = 1.0 ; return ; } let q00 : f64 = * quat . add (0) * * quat . add (0) ; let q01 : f64 = * quat . add (0) * * quat . add (1) ; let q02 : f64 = * quat . add (0) * * quat . add (2) ; let q03 : f64 = * quat . add (0) * * quat . add (3) ; let q11 : f64 = * quat . add (1) * * quat . add (1) ; let q12 : f64 = * quat . add (1) * * quat . add (2) ; let q13 : f64 = * quat . add (1) * * quat . add (3) ; let q22 : f64 = * quat . add (2) * * quat . add (2) ; let q23 : f64 = * quat . add (2) * * quat . add (3) ; let q33 : f64 = * quat . add (3) * * quat . add (3) ; * res . add (0) = q00 + q11 - q22 - q33 ; * res . add (4) = q00 - q11 + q22 - q33 ; * res . add (8) = q00 - q11 - q22 + q33 ; * res . add (1) = 2.0 * (q12 - q03) ; * res . add (2) = 2.0 * (q13 + q02) ; * res . add (3) = 2.0 * (q12 + q03) ; * res . add (5) = 2.0 * (q23 - q01) ; * res . add (6) = 2.0 * (q13 - q02) ; * res . add (7) = 2.0 * (q23 + q01) ; }
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
    unsafe { let mut tmp : [f64 ; 4] = [0.0 ; 4] ; tmp [0] = * qa . add (0) * * qb . add (0) - * qa . add (1) * * qb . add (1) - * qa . add (2) * * qb . add (2) - * qa . add (3) * * qb . add (3) ; tmp [1] = * qa . add (0) * * qb . add (1) + * qa . add (1) * * qb . add (0) + * qa . add (2) * * qb . add (3) - * qa . add (3) * * qb . add (2) ; tmp [2] = * qa . add (0) * * qb . add (2) - * qa . add (1) * * qb . add (3) + * qa . add (2) * * qb . add (0) + * qa . add (3) * * qb . add (1) ; tmp [3] = * qa . add (0) * * qb . add (3) + * qa . add (1) * * qb . add (2) - * qa . add (2) * * qb . add (1) + * qa . add (3) * * qb . add (0) ; mjuu_normvec (tmp . as_mut_ptr () , 4) ; for i in 0 .. 4usize { * res . add (i) = tmp [i] ; } }
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
    unsafe { let tmp0 : f64 = * mat . add (0) * * vec . add (0) + * mat . add (1) * * vec . add (1) + * mat . add (2) * * vec . add (2) ; let tmp1 : f64 = * mat . add (3) * * vec . add (0) + * mat . add (4) * * vec . add (1) + * mat . add (5) * * vec . add (2) ; let tmp2 : f64 = * mat . add (6) * * vec . add (0) + * mat . add (7) * * vec . add (1) + * mat . add (8) * * vec . add (2) ; * res . add (0) = tmp0 ; * res . add (1) = tmp1 ; * res . add (2) = tmp2 ; }
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
    unsafe { let tmp0 : f64 = * mat . add (0) * * vec . add (0) + * mat . add (3) * * vec . add (1) + * mat . add (6) * * vec . add (2) ; let tmp1 : f64 = * mat . add (1) * * vec . add (0) + * mat . add (4) * * vec . add (1) + * mat . add (7) * * vec . add (2) ; let tmp2 : f64 = * mat . add (2) * * vec . add (0) + * mat . add (5) * * vec . add (1) + * mat . add (8) * * vec . add (2) ; * res . add (0) = tmp0 ; * res . add (1) = tmp1 ; * res . add (2) = tmp2 ; }
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
    unsafe { let mut tmp : [f64 ; 9] = [0.0 ; 9] ; tmp [0] = * R . add (0) * * M . add (0) + * R . add (1) * * M . add (3) + * R . add (2) * * M . add (6) ; tmp [1] = * R . add (0) * * M . add (1) + * R . add (1) * * M . add (4) + * R . add (2) * * M . add (7) ; tmp [2] = * R . add (0) * * M . add (2) + * R . add (1) * * M . add (5) + * R . add (2) * * M . add (8) ; tmp [3] = * R . add (3) * * M . add (0) + * R . add (4) * * M . add (3) + * R . add (5) * * M . add (6) ; tmp [4] = * R . add (3) * * M . add (1) + * R . add (4) * * M . add (4) + * R . add (5) * * M . add (7) ; tmp [5] = * R . add (3) * * M . add (2) + * R . add (4) * * M . add (5) + * R . add (5) * * M . add (8) ; tmp [6] = * R . add (6) * * M . add (0) + * R . add (7) * * M . add (3) + * R . add (8) * * M . add (6) ; tmp [7] = * R . add (6) * * M . add (1) + * R . add (7) * * M . add (4) + * R . add (8) * * M . add (7) ; tmp [8] = * R . add (6) * * M . add (2) + * R . add (7) * * M . add (5) + * R . add (8) * * M . add (8) ; * res . add (0) = tmp [0] * * R . add (0) + tmp [1] * * R . add (1) + tmp [2] * * R . add (2) ; * res . add (1) = tmp [0] * * R . add (3) + tmp [1] * * R . add (4) + tmp [2] * * R . add (5) ; * res . add (2) = tmp [0] * * R . add (6) + tmp [1] * * R . add (7) + tmp [2] * * R . add (8) ; * res . add (3) = tmp [3] * * R . add (0) + tmp [4] * * R . add (1) + tmp [5] * * R . add (2) ; * res . add (4) = tmp [3] * * R . add (3) + tmp [4] * * R . add (4) + tmp [5] * * R . add (5) ; * res . add (5) = tmp [3] * * R . add (6) + tmp [4] * * R . add (7) + tmp [5] * * R . add (8) ; * res . add (6) = tmp [6] * * R . add (0) + tmp [7] * * R . add (1) + tmp [8] * * R . add (2) ; * res . add (7) = tmp [6] * * R . add (3) + tmp [7] * * R . add (4) + tmp [8] * * R . add (5) ; * res . add (8) = tmp [6] * * R . add (6) + tmp [7] * * R . add (7) + tmp [8] * * R . add (8) ; }
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
    unsafe { let mut tmp : [f64 ; 9] = [0.0 ; 9] ; tmp [0] = * A . add (0) * * B . add (0) + * A . add (1) * * B . add (3) + * A . add (2) * * B . add (6) ; tmp [1] = * A . add (0) * * B . add (1) + * A . add (1) * * B . add (4) + * A . add (2) * * B . add (7) ; tmp [2] = * A . add (0) * * B . add (2) + * A . add (1) * * B . add (5) + * A . add (2) * * B . add (8) ; tmp [3] = * A . add (3) * * B . add (0) + * A . add (4) * * B . add (3) + * A . add (5) * * B . add (6) ; tmp [4] = * A . add (3) * * B . add (1) + * A . add (4) * * B . add (4) + * A . add (5) * * B . add (7) ; tmp [5] = * A . add (3) * * B . add (2) + * A . add (4) * * B . add (5) + * A . add (5) * * B . add (8) ; tmp [6] = * A . add (6) * * B . add (0) + * A . add (7) * * B . add (3) + * A . add (8) * * B . add (6) ; tmp [7] = * A . add (6) * * B . add (1) + * A . add (7) * * B . add (4) + * A . add (8) * * B . add (7) ; tmp [8] = * A . add (6) * * B . add (2) + * A . add (7) * * B . add (5) + * A . add (8) * * B . add (8) ; for i in 0 .. 9usize { * res . add (i) = tmp [i] ; } }
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
    unsafe { let tmp : [f64 ; 9] = [* mat . add (0) , * mat . add (3) , * mat . add (6) , * mat . add (1) , * mat . add (4) , * mat . add (7) , * mat . add (2) , * mat . add (5) , * mat . add (8) ,] ; for i in 0 .. 9usize { * res . add (i) = tmp [i] ; } }
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
    unsafe { let mut mat : [f64 ; 9] = [0.0 ; 9] ; let qneg : [f64 ; 4] = [* quat . add (0) , - * quat . add (1) , - * quat . add (2) , - * quat . add (3)] ; mjuu_quat2mat (mat . as_mut_ptr () , qneg . as_ptr ()) ; mjuu_mulvecmat (al , ag , mat . as_ptr ()) ; }
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
    unsafe { let a : [f64 ; 3] = [* pg . add (0) - * pos . add (0) , * pg . add (1) - * pos . add (1) , * pg . add (2) - * pos . add (2)] ; mjuu_localaxis (pl , a . as_ptr () , quat) ; }
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
    unsafe { let pneg : [f64 ; 4] = [* parent . add (0) , - * parent . add (1) , - * parent . add (2) , - * parent . add (3)] ; mjuu_mulquat (local , pneg . as_ptr () , child) ; }
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
    unsafe { * a . add (0) = * b . add (1) * * c . add (2) - * b . add (2) * * c . add (1) ; * a . add (1) = * b . add (2) * * c . add (0) - * b . add (0) * * c . add (2) ; * a . add (2) = * b . add (0) * * c . add (1) - * b . add (1) * * c . add (0) ; }
}

/// C: mjuu_makenormal (user/user_util.h:118)
/// Calls: mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_makenormal(normal: *mut f64, a: [T; 3], b: [T; 3], c: [T; 3]) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (normal : * mut f64, a : [T ; 3], b : [T ; 3], c : [T ; 3])
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
    extern "C" { fn mjuu_z2quat_impl(quat: *mut f64, vec: *const f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjuu_z2quat_impl(quat, vec) }
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
    extern "C" { fn mjuu_frame2quat_impl(quat: *mut f64, x: *const f64, y: *const f64, z: *const f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjuu_frame2quat_impl(quat, x, y, z) }
}

/// C: mjuu_frameinvert (user/user_util.h:128)
/// Calls: mjuu_localaxis
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameinvert(newpos: [f64; 3], newquat: [f64; 4], oldpos: [f64; 3], oldquat: [f64; 4]) {
    extern "C" { fn mjuu_frameinvert_impl(newpos: [f64; 3], newquat: [f64; 4], oldpos: [f64; 3], oldquat: [f64; 4]); }
    // SAFETY: delegates to C implementation
    unsafe { mjuu_frameinvert_impl(newpos, newquat, oldpos, oldquat) }
}

/// C: mjuu_frameaccum (user/user_util.h:132)
/// Calls: mjuu_mulquat, mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccum(pos: [f64; 3], quat: [f64; 4], childpos: [f64; 3], childquat: [f64; 4]) {
    extern "C" { fn mjuu_frameaccum_impl(pos: [f64; 3], quat: [f64; 4], childpos: [f64; 3], childquat: [f64; 4]); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjuu_frameaccum_impl(pos, quat, childpos, childquat) }
}

/// C: mjuu_frameaccumChild (user/user_util.h:136)
/// Calls: mjuu_frameaccum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccum_child(pos: [f64; 3], quat: [f64; 4], childpos: [f64; 3], childquat: [f64; 4]) {
    extern "C" { fn mjuu_frameaccumChild_impl(pos: [f64; 3], quat: [f64; 4], childpos: [f64; 3], childquat: [f64; 4]); }
    // SAFETY: delegates to C implementation
    unsafe { mjuu_frameaccumChild_impl(pos, quat, childpos, childquat) }
}

/// C: mjuu_frameaccuminv (user/user_util.h:140)
/// Calls: mjuu_mulquat, mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccuminv(pos: [f64; 3], quat: [f64; 4], childpos: [f64; 3], childquat: [f64; 4]) {
    extern "C" { fn mjuu_frameaccuminv_impl(pos: [f64; 3], quat: [f64; 4], childpos: [f64; 3], childquat: [f64; 4]); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjuu_frameaccuminv_impl(pos, quat, childpos, childquat) }
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
    extern "C" { fn mjuu_globalinertia_impl(global: *mut f64, local: *const f64, quat: *const f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjuu_globalinertia_impl(global, local, quat) }
}

/// C: mjuu_offcenter (user/user_util.h:147)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_offcenter(res: *mut f64, mass: f64, vec: *const f64) {
    extern "C" { fn mjuu_offcenter_impl(res: *mut f64, mass: f64, vec: *const f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjuu_offcenter_impl(res, mass, vec) }
}

/// C: mjuu_visccoef (user/user_util.h:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_visccoef(visccoef: *mut f64, mass: f64, inertia: *const f64, scl: f64) {
    extern "C" { fn mjuu_visccoef_impl(visccoef: *mut f64, mass: f64, inertia: *const f64, scl: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjuu_visccoef_impl(visccoef, mass, inertia, scl) }
}

/// C: mjuu_rotVecQuat (user/user_util.h:153)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_rot_vec_quat(res: [f64; 3], vec: [f64; 3], quat: [f64; 4]) {
    extern "C" { fn mjuu_rotVecQuat_impl(res: [f64; 3], vec: [f64; 3], quat: [f64; 4]); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjuu_rotVecQuat_impl(res, vec, quat) }
}

/// C: mjuu_updateFrame (user/user_util.h:156)
/// Calls: mjuu_axisAngle2Quat, mjuu_crossvec, mjuu_dot3, mjuu_frame2quat, mjuu_normvec, mjuu_rotVecQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_update_frame(quat: [f64; 4], normal: [f64; 3], edge: [f64; 3], tprv: [f64; 3], tnxt: [f64; 3], first: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (quat : [f64 ; 4], normal : [f64 ; 3], edge : [f64 ; 3], tprv : [f64 ; 3], tnxt : [f64 ; 3], first : i32)
    // Previous return: f64
    extern "C" { fn mjuu_updateFrame_impl(quat: [f64; 4], normal: [f64; 3], edge: [f64; 3], tprv: [f64; 3], tnxt: [f64; 3], first: i32) -> f64; }
    // SAFETY: delegates to C implementation; array params are pointers in C ABI
    unsafe { mjuu_updateFrame_impl(quat, normal, edge, tprv, tnxt, first) }
}

/// C: mjuu_eig3 (user/user_util.h:160)
/// Calls: mjuu_mulmat, mjuu_mulquat, mjuu_normvec, mjuu_quat2mat, mjuu_transposemat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eig3(eigval: [f64; 3], eigvec: [f64; 9], quat: [f64; 4], mat: [f64; 9]) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (eigval : [f64 ; 3], eigvec : [f64 ; 9], quat : [f64 ; 4], mat : [f64 ; 9])
    // Previous return: i32
    const K_EIG_EPS : f64 = 1e-12 ; let mut eigval = eigval ; let mut eigvec = eigvec ; let mut quat = quat ; let mut D : [f64 ; 9] = [0.0 ; 9] ; let mut tmp : [f64 ; 9] = [0.0 ; 9] ; let mut tmp2 : [f64 ; 9] = [0.0 ; 9] ; let mut tau : f64 ; let mut t : f64 ; let mut c : f64 ; let mut rk : usize ; let mut ck : usize ; let mut rotk : usize ; quat [0] = 1.0 ; quat [1] = 0.0 ; quat [2] = 0.0 ; quat [3] = 0.0 ; let mut iter : i32 = 0 ; while iter < 500 { unsafe { mjuu_quat2mat (eigvec . as_mut_ptr () , quat . as_ptr ()) ; mjuu_transposemat (tmp2 . as_mut_ptr () , eigvec . as_ptr ()) ; mjuu_mulmat (tmp . as_mut_ptr () , tmp2 . as_ptr () , mat . as_ptr ()) ; mjuu_mulmat (D . as_mut_ptr () , tmp . as_ptr () , eigvec . as_ptr ()) ; } eigval [0] = D [0] ; eigval [1] = D [4] ; eigval [2] = D [8] ; if D [1] . abs () > D [2] . abs () && D [1] . abs () > D [5] . abs () { rk = 0 ; ck = 1 ; rotk = 2 ; } else if D [2] . abs () > D [5] . abs () { rk = 0 ; ck = 2 ; rotk = 1 ; } else { rk = 1 ; ck = 2 ; rotk = 0 ; } if D [3 * rk + ck] . abs () < K_EIG_EPS { break ; } tau = (D [4 * ck] - D [4 * rk]) / (2.0 * D [3 * rk + ck]) ; if tau >= 0.0 { t = 1.0 / (tau + (1.0 + tau * tau) . sqrt ()) ; } else { t = - 1.0 / (- tau + (1.0 + tau * tau) . sqrt ()) ; } c = 1.0 / (1.0 + t * t) . sqrt () ; if c > 1.0 - K_EIG_EPS { break ; } tmp [1] = 0.0 ; tmp [2] = 0.0 ; tmp [3] = 0.0 ; tmp [rotk + 1] = if tau >= 0.0 { - (0.5 - 0.5 * c) . sqrt () } else { (0.5 - 0.5 * c) . sqrt () } ; if rotk == 1 { tmp [rotk + 1] = - tmp [rotk + 1] ; } tmp [0] = (1.0 - tmp [rotk + 1] * tmp [rotk + 1]) . sqrt () ; unsafe { mjuu_normvec (tmp . as_mut_ptr () , 4) ; } unsafe { mjuu_mulquat (quat . as_mut_ptr () , quat . as_ptr () , tmp . as_ptr ()) ; mjuu_normvec (quat . as_mut_ptr () , 4) ; } iter += 1 ; } for j in 0 .. 3i32 { let j1 = (j % 2) as usize ; if eigval [j1] + K_EIG_EPS < eigval [j1 + 1] { t = eigval [j1] ; eigval [j1] = eigval [j1 + 1] ; eigval [j1 + 1] = t ; tmp [0] = 0.707106781186548 ; tmp [1] = 0.0 ; tmp [2] = 0.0 ; tmp [3] = 0.0 ; tmp [(j1 + 2) % 3 + 1] = tmp [0] ; unsafe { mjuu_mulquat (quat . as_mut_ptr () , quat . as_ptr () , tmp . as_ptr ()) ; mjuu_normvec (quat . as_mut_ptr () , 4) ; } } } unsafe { mjuu_quat2mat (eigvec . as_mut_ptr () , quat . as_ptr ()) ; } iter
}

/// C: mjuu_eigendecompose (user/user_util.h:166)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eigendecompose(mat: *mut f64, eigval: *mut f64, eigvec: *mut f64, n: i32) -> i32 {

    extern "C" { fn mjuu_eigendecompose_impl(mat: *mut f64, eigval: *mut f64, eigvec: *mut f64, n: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjuu_eigendecompose_impl(mat, eigval, eigvec, n) }
}

/// C: mjuu_trnVecPose (user/user_util.h:169)
/// Calls: mjuu_rotVecQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_trn_vec_pose(res: [f64; 3], pos: [f64; 3], quat: [f64; 4], vec: [f64; 3]) {
    extern "C" { fn mjuu_trnVecPose_impl(res: [f64; 3], pos: [f64; 3], quat: [f64; 4], vec: [f64; 3]); }
    // SAFETY: delegates to C implementation
    unsafe { mjuu_trnVecPose_impl(res, pos, quat, vec) }
}

/// C: mjuu_fullInertia (user/user_util.h:172)
/// Calls: mjuu_defined, mjuu_eig3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_full_inertia(quat: [f64; 4], inertia: [f64; 3], fullinertia: [f64; 6]) -> *const i8 {
    extern "C" { fn mjuu_fullInertia_impl(quat: [f64; 4], inertia: [f64; 3], fullinertia: [f64; 6]) -> *const i8; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjuu_fullInertia_impl(quat, inertia, fullinertia) }
}

/// C: FilePath::IsAbs (user/user_util.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_abs(self_ptr: *mut FilePath) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: bool
    extern "C" { fn FilePath_IsAbs_impl (self_ptr : * mut FilePath) -> bool ; } unsafe { FilePath_IsAbs_impl (self_ptr) }
}

/// C: FilePath::AbsPrefix (user/user_util.h:195)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_abs_prefix(self_ptr: *mut FilePath) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: i32
    extern "C" { fn FilePath_AbsPrefix_impl (self_ptr : * mut FilePath) -> i32 ; } unsafe { FilePath_AbsPrefix_impl (self_ptr) }
}

/// C: FilePath::Str (user/user_util.h:198)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str(self_ptr: *mut FilePath) -> *const i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: * const i32
    extern "C" { fn FilePath_Str_impl (self_ptr : * mut FilePath) -> * const i32 ; } unsafe { FilePath_Str_impl (self_ptr) }
}

/// C: FilePath::StrLower (user/user_util.h:202)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str_lower(self_ptr: *mut FilePath) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: std__string
    extern "C" { fn FilePath_StrLower_impl (self_ptr : * mut FilePath) -> std__string ; } unsafe { FilePath_StrLower_impl (self_ptr) }
}

/// C: FilePath::Ext (user/user_util.h:205)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_ext(self_ptr: *mut FilePath) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: std__string
    extern "C" { fn FilePath_Ext_impl (self_ptr : * mut FilePath) -> std__string ; } unsafe { FilePath_Ext_impl (self_ptr) }
}

/// C: FilePath::StripExt (user/user_util.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_ext(self_ptr: *mut FilePath) -> FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: FilePath
    extern "C" { fn FilePath_StripExt_impl (self_ptr : * mut FilePath) -> FilePath ; } unsafe { FilePath_StripExt_impl (self_ptr) }
}

/// C: FilePath::StripPath (user/user_util.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_path(self_ptr: *mut FilePath) -> FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: FilePath
    extern "C" { fn FilePath_StripPath_impl (self_ptr : * mut FilePath) -> FilePath ; } unsafe { FilePath_StripPath_impl (self_ptr) }
}

/// C: FilePath::Lower (user/user_util.h:217)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_lower(self_ptr: *mut FilePath) -> FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: FilePath
    extern "C" { fn FilePath_Lower_impl (self_ptr : * mut FilePath) -> FilePath ; } unsafe { FilePath_Lower_impl (self_ptr) }
}

/// C: FilePath::size (user/user_util.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_size(self_ptr: *mut FilePath) -> std__size_t {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: std__size_t
    extern "C" { fn FilePath_size_impl (self_ptr : * mut FilePath) -> std__size_t ; } unsafe { FilePath_size_impl (self_ptr) }
}

/// C: FilePath::c_str (user/user_util.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_c_str(self_ptr: *mut FilePath) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: * const i8
    extern "C" { fn FilePath_c_str_impl (self_ptr : * mut FilePath) -> * const i8 ; } unsafe { FilePath_c_str_impl (self_ptr) }
}

/// C: FilePath::empty (user/user_util.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_empty(self_ptr: *mut FilePath) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: bool
    extern "C" { fn FilePath_empty_impl (self_ptr : * mut FilePath) -> bool ; } unsafe { FilePath_empty_impl (self_ptr) }
}

/// C: FilePath::PathReduce (user/user_util.h:227)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_path_reduce(str: *const std__string) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const std__string)
    // Previous return: std__string
    extern "C" { fn FilePath_PathReduce_impl (str : * const std__string) -> std__string ; } unsafe { FilePath_PathReduce_impl (str) }
}

/// C: FilePath::IsSeparator (user/user_util.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_separator(c: i8) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (c : i8)
    // Previous return: bool
    extern "C" { fn FilePath_IsSeparator_impl (c : i8) -> bool ; } unsafe { FilePath_IsSeparator_impl (c) }
}

/// C: FilePath::Combine (user/user_util.h:231)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_combine(s1: *const std__string, s2: *const std__string) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (s1 : * const std__string, s2 : * const std__string)
    // Previous return: std__string
    extern "C" { fn FilePath_Combine_impl (s1 : * const std__string , s2 : * const std__string) -> std__string ; } unsafe { FilePath_Combine_impl (s1 , s2) }
}

/// C: FilePath::FilePathFast (user/user_util.h:234)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_file_path_fast(str: *const i32) -> FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i32)
    // Previous return: FilePath
    extern "C" { fn FilePath_FilePathFast_impl (str : * const i32) -> FilePath ; } unsafe { FilePath_FilePathFast_impl (str) }
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
    extern "C" { fn mjuu_dirnamelen_impl(path: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjuu_dirnamelen_impl(path) }
}

