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
    let _sv = core::mem::size_of_val(&res);
    extern "C" { fn mjuu_axisAngle2Quat(res: [f64; 4], axis: [f64; 3], angle: f64); }
    // SAFETY: no pointer params, arrays passed by value
    unsafe { mjuu_axisAngle2Quat(res, axis, angle) }
}

/// C: mjuu_isValidContentType (user/user_util.cc:973)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_is_valid_content_type(text: string_view) -> bool {
    let _sv = core::mem::size_of_val(&text);
    extern "C" { fn mjuu_isValidContentType(text: string_view) -> bool; }
    // SAFETY: C validates content type string format
    unsafe { mjuu_isValidContentType(text) }
}

/// C: FileToMemory (user/user_util.cc:1196)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn file_to_memory(filename: *const i8) -> i32 {
    if filename.is_null() {
        return -1;
    }
    extern "C" { fn FileToMemory(filename: *const i8) -> i32; }
    unsafe { FileToMemory(filename) }
}

/// C: VectorToString (user/user_util.cc:1256)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string(v: *const i32) -> std__string {
    if v.is_null() {
        // SAFETY: std__string is a zero-sized type; zeroed is trivially valid
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn VectorToString(v: *const i32) -> std__string; }
    // SAFETY: v verified non-null; delegates to C implementation
    unsafe { VectorToString(v) }
}

/// C: StrToNum (user/user_util.cc:1277)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num(str: *mut i8, c: *mut *mut i8) -> i32 {
    if str.is_null() { return 0; }
    extern "C" { fn StrToNum(str: *mut i8, c: *mut *mut i8) -> i32; }
    // SAFETY: str verified non-null
    unsafe { StrToNum(str, c) }
}

/// C: IsNullOrSpace (user/user_util.cc:1301)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_or_space(c: *mut i8) -> bool  {
    if c.is_null() {
        return true;
    }
    // SAFETY: c verified non-null; check if pointed-to char is whitespace or null terminator
    unsafe {
        let ch = *c as u8;
        ch == 0 || ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r'
    }
}

/// C: SkipSpace (user/user_util.cc:1305)
/// Calls: IsNullOrSpace
#[allow(unused_variables, non_snake_case)]
pub fn skip_space(c: *mut i8) -> *mut i8  {
    if c.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn SkipSpace(c: *mut i8) -> *mut i8; }
    // SAFETY: c verified non-null; delegates to C implementation
    unsafe { SkipSpace(c) }
}

/// C: StringToVector (user/user_util.cc:1315)
/// Calls: SkipSpace
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector(cs: *mut i8) -> i32 {
    extern "C" { fn StringToVector(cs: *mut i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { StringToVector(cs) }
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
    if dest.is_null() { return; }
    extern "C" { fn mjuu_copyvec(dest: *mut T1, src: *const T2, n: i32); }
    // SAFETY: dest verified non-null; C copies n elements from src to dest
    unsafe { mjuu_copyvec(dest, src, n) }
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
    if normal.is_null() { return 0.0; }
    extern "C" { fn mjuu_makenormal(normal: *mut f64, a: [T; 3], b: [T; 3], c: [T; 3]) -> f64; }
    // SAFETY: normal verified non-null; C computes triangle normal from vertices
    unsafe { mjuu_makenormal(normal, a, b, c) }
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
    // SAFETY: quat has 4 elements, vec has 3 elements.
    unsafe {
        let z: [f64; 3] = [0.0, 0.0, 1.0];
        mjuu_crossvec(quat.add(1), z.as_ptr(), vec);
        let s = mjuu_normvec(quat.add(1), 3);
        if s < 1e-10 {
            *quat.add(1) = 1.0;
            *quat.add(2) = 0.0;
            *quat.add(3) = 0.0;
        }
        let ang = s.atan2(*vec.add(2));
        *quat.add(0) = (ang / 2.0).cos();
        *quat.add(1) *= (ang / 2.0).sin();
        *quat.add(2) *= (ang / 2.0).sin();
        *quat.add(3) *= (ang / 2.0).sin();
    }
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
    // SAFETY: quat has 4 elements, x/y/z each have 3 elements.
    // mat[c][r] indexing: mat[0]=x, mat[1]=y, mat[2]=z
    unsafe {
        let mat: [*const f64; 3] = [x, y, z];

        // q0 largest
        if *mat[0].add(0) + *mat[1].add(1) + *mat[2].add(2) > 0.0 {
            *quat.add(0) = 0.5 * (1.0 + *mat[0].add(0) + *mat[1].add(1) + *mat[2].add(2)).sqrt();
            *quat.add(1) = 0.25 * (*mat[1].add(2) - *mat[2].add(1)) / *quat.add(0);
            *quat.add(2) = 0.25 * (*mat[2].add(0) - *mat[0].add(2)) / *quat.add(0);
            *quat.add(3) = 0.25 * (*mat[0].add(1) - *mat[1].add(0)) / *quat.add(0);
        }
        // q1 largest
        else if *mat[0].add(0) > *mat[1].add(1) && *mat[0].add(0) > *mat[2].add(2) {
            *quat.add(1) = 0.5 * (1.0 + *mat[0].add(0) - *mat[1].add(1) - *mat[2].add(2)).sqrt();
            *quat.add(0) = 0.25 * (*mat[1].add(2) - *mat[2].add(1)) / *quat.add(1);
            *quat.add(2) = 0.25 * (*mat[1].add(0) + *mat[0].add(1)) / *quat.add(1);
            *quat.add(3) = 0.25 * (*mat[2].add(0) + *mat[0].add(2)) / *quat.add(1);
        }
        // q2 largest
        else if *mat[1].add(1) > *mat[2].add(2) {
            *quat.add(2) = 0.5 * (1.0 - *mat[0].add(0) + *mat[1].add(1) - *mat[2].add(2)).sqrt();
            *quat.add(0) = 0.25 * (*mat[2].add(0) - *mat[0].add(2)) / *quat.add(2);
            *quat.add(1) = 0.25 * (*mat[1].add(0) + *mat[0].add(1)) / *quat.add(2);
            *quat.add(3) = 0.25 * (*mat[2].add(1) + *mat[1].add(2)) / *quat.add(2);
        }
        // q3 largest
        else {
            *quat.add(3) = 0.5 * (1.0 - *mat[0].add(0) - *mat[1].add(1) + *mat[2].add(2)).sqrt();
            *quat.add(0) = 0.25 * (*mat[0].add(1) - *mat[1].add(0)) / *quat.add(3);
            *quat.add(1) = 0.25 * (*mat[2].add(0) + *mat[0].add(2)) / *quat.add(3);
            *quat.add(2) = 0.25 * (*mat[2].add(1) + *mat[1].add(2)) / *quat.add(3);
        }

        mjuu_normvec(quat, 4);
    }
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
    // NOTE: In C ABI, array params are pointers. newpos, newquat are output.
    let newpos_ptr = newpos.as_ptr() as *mut f64;
    let newquat_ptr = newquat.as_ptr() as *mut f64;

    // SAFETY: all pointers valid per C ABI (array param = pointer to caller's memory)
    unsafe {
        // position: newpos = localaxis(oldpos, oldquat), then negate
        mjuu_localaxis(newpos_ptr, oldpos.as_ptr(), oldquat.as_ptr());
        *newpos_ptr = -*newpos_ptr;
        *newpos_ptr.add(1) = -*newpos_ptr.add(1);
        *newpos_ptr.add(2) = -*newpos_ptr.add(2);

        // orientation: conjugate of oldquat
        *newquat_ptr = *oldquat.as_ptr();
        *newquat_ptr.add(1) = -*oldquat.as_ptr().add(1);
        *newquat_ptr.add(2) = -*oldquat.as_ptr().add(2);
        *newquat_ptr.add(3) = -*oldquat.as_ptr().add(3);
    }
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
    // NOTE: In C ABI, array params are pointers. pos, quat are output/inout.
    let pos_ptr = pos.as_ptr() as *mut f64;
    let quat_ptr = quat.as_ptr() as *mut f64;

    let mut mat: [f64; 9] = [0.0; 9];
    let mut vec: [f64; 3] = [0.0; 3];
    let mut qtmp: [f64; 4] = [0.0; 4];

    // SAFETY: all pointers valid per caller contract (C ABI array param = pointer)
    unsafe {
        mjuu_quat2mat(mat.as_mut_ptr(), quat_ptr);
        mjuu_mulvecmat(vec.as_mut_ptr(), childpos.as_ptr(), mat.as_ptr());
        *pos_ptr += vec[0];
        *pos_ptr.add(1) += vec[1];
        *pos_ptr.add(2) += vec[2];
        mjuu_mulquat(qtmp.as_mut_ptr(), quat_ptr, childquat.as_ptr());
        mjuu_copyvec(quat_ptr as *mut T1, qtmp.as_ptr() as *const T2, 4);
    }
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
    // NOTE: In C ABI, array params are pointers. childpos, childquat are output/inout.
    let childpos_ptr = childpos.as_ptr() as *mut f64;
    let childquat_ptr = childquat.as_ptr() as *mut f64;

    // SAFETY: all pointers valid per C ABI (array param = pointer to caller's memory)
    unsafe {
        // copy pos, quat into local arrays
        let mut p: [f64; 3] = [*pos.as_ptr(), *pos.as_ptr().add(1), *pos.as_ptr().add(2)];
        let mut q: [f64; 4] = [
            *quat.as_ptr(), *quat.as_ptr().add(1),
            *quat.as_ptr().add(2), *quat.as_ptr().add(3),
        ];

        // accumulate: p, q = frameaccum(p, q, childpos, childquat)
        mjuu_frameaccum(p, q, childpos, childquat);

        // copy results back to childpos, childquat
        mjuu_copyvec(childpos_ptr as *mut T1, p.as_ptr() as *const T2, 3);
        mjuu_copyvec(childquat_ptr as *mut T1, q.as_ptr() as *const T2, 4);
    }
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
    // NOTE: In C ABI, array params are pointers. pos, quat are output/inout.
    let pos_ptr = pos.as_ptr() as *mut f64;
    let quat_ptr = quat.as_ptr() as *mut f64;

    let mut mat: [f64; 9] = [0.0; 9];
    let mut vec: [f64; 3] = [0.0; 3];
    let mut qtmp: [f64; 4] = [0.0; 4];
    let qneg: [f64; 4] = [childquat[0], -childquat[1], -childquat[2], -childquat[3]];

    // SAFETY: all pointers valid per caller contract (C ABI array param = pointer)
    unsafe {
        mjuu_mulquat(qtmp.as_mut_ptr(), quat_ptr, qneg.as_ptr());
        mjuu_copyvec(quat_ptr as *mut T1, qtmp.as_ptr() as *const T2, 4);
        mjuu_quat2mat(mat.as_mut_ptr(), quat_ptr);
        mjuu_mulvecmat(vec.as_mut_ptr(), childpos.as_ptr(), mat.as_ptr());
        *pos_ptr -= vec[0];
        *pos_ptr.add(1) -= vec[1];
        *pos_ptr.add(2) -= vec[2];
    }
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
    // SAFETY: global has 6 elements, local has 3, quat has 4. All valid.
    unsafe {
        let mut mat: [f64; 9] = [0.0; 9];
        mjuu_quat2mat(mat.as_mut_ptr(), quat);

        let tmp: [f64; 9] = [
            mat[0] * *local.add(0), mat[3] * *local.add(0), mat[6] * *local.add(0),
            mat[1] * *local.add(1), mat[4] * *local.add(1), mat[7] * *local.add(1),
            mat[2] * *local.add(2), mat[5] * *local.add(2), mat[8] * *local.add(2),
        ];

        *global.add(0) = mat[0] * tmp[0] + mat[1] * tmp[3] + mat[2] * tmp[6];
        *global.add(1) = mat[3] * tmp[1] + mat[4] * tmp[4] + mat[5] * tmp[7];
        *global.add(2) = mat[6] * tmp[2] + mat[7] * tmp[5] + mat[8] * tmp[8];
        *global.add(3) = mat[0] * tmp[1] + mat[1] * tmp[4] + mat[2] * tmp[7];
        *global.add(4) = mat[0] * tmp[2] + mat[1] * tmp[5] + mat[2] * tmp[8];
        *global.add(5) = mat[3] * tmp[2] + mat[4] * tmp[5] + mat[5] * tmp[8];
    }
}

/// C: mjuu_offcenter (user/user_util.h:147)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_offcenter(res: *mut f64, mass: f64, vec: *const f64) {
    // SAFETY: res has 6 elements, vec has 3 elements.
    unsafe {
        *res.add(0) = mass * (*vec.add(1) * *vec.add(1) + *vec.add(2) * *vec.add(2));
        *res.add(1) = mass * (*vec.add(0) * *vec.add(0) + *vec.add(2) * *vec.add(2));
        *res.add(2) = mass * (*vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1));
        *res.add(3) = -mass * *vec.add(0) * *vec.add(1);
        *res.add(4) = -mass * *vec.add(0) * *vec.add(2);
        *res.add(5) = -mass * *vec.add(1) * *vec.add(2);
    }
}

/// C: mjuu_visccoef (user/user_util.h:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_visccoef(visccoef: *mut f64, mass: f64, inertia: *const f64, scl: f64) {
    const MJEPS: f64 = 1e-14;
    // SAFETY: visccoef points to 6 f64; inertia points to 3 f64. Caller guarantees.
    unsafe {
        // compute equivalent box
        let mut ebox: [f64; 3] = [0.0; 3];
        let v0 = *inertia.add(1) + *inertia.add(2) - *inertia.add(0);
        let v1 = *inertia.add(0) + *inertia.add(2) - *inertia.add(1);
        let v2 = *inertia.add(0) + *inertia.add(1) - *inertia.add(2);
        ebox[0] = (if v0 > MJEPS { v0 } else { MJEPS } / mass * 6.0).sqrt();
        ebox[1] = (if v1 > MJEPS { v1 } else { MJEPS } / mass * 6.0).sqrt();
        ebox[2] = (if v2 > MJEPS { v2 } else { MJEPS } / mass * 6.0).sqrt();

        // apply formula for box (or rather cross) viscosity
        // torque components
        *visccoef.add(0) = scl * 4.0 / 3.0 * ebox[0] * (ebox[1] * ebox[1] * ebox[1] + ebox[2] * ebox[2] * ebox[2]);
        *visccoef.add(1) = scl * 4.0 / 3.0 * ebox[1] * (ebox[0] * ebox[0] * ebox[0] + ebox[2] * ebox[2] * ebox[2]);
        *visccoef.add(2) = scl * 4.0 / 3.0 * ebox[2] * (ebox[0] * ebox[0] * ebox[0] + ebox[1] * ebox[1] * ebox[1]);

        // force components
        *visccoef.add(3) = scl * 4.0 * ebox[1] * ebox[2];
        *visccoef.add(4) = scl * 4.0 * ebox[0] * ebox[2];
        *visccoef.add(5) = scl * 4.0 * ebox[0] * ebox[1];
    }
}

/// C: mjuu_rotVecQuat (user/user_util.h:153)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_rot_vec_quat(res: [f64; 3], vec: [f64; 3], quat: [f64; 4]) {
    // NOTE: In C ABI, array params are pointers. res is the output.
    let res_ptr = res.as_ptr() as *mut f64;

    if vec[0] == 0.0 && vec[1] == 0.0 && vec[2] == 0.0 {
        // SAFETY: res_ptr points to caller-owned memory (C ABI array param = pointer)
        unsafe {
            *res_ptr = 0.0;
            *res_ptr.add(1) = 0.0;
            *res_ptr.add(2) = 0.0;
        }
    } else if quat[0] == 1.0 && quat[1] == 0.0 && quat[2] == 0.0 && quat[3] == 0.0 {
        // SAFETY: res_ptr points to caller-owned memory, vec is valid input array
        unsafe {
            mjuu_copyvec(res_ptr as *mut T1, vec.as_ptr() as *const T2, 3);
        }
    } else {
        let tmp: [f64; 3] = [
            quat[0] * vec[0] + quat[2] * vec[2] - quat[3] * vec[1],
            quat[0] * vec[1] + quat[3] * vec[0] - quat[1] * vec[2],
            quat[0] * vec[2] + quat[1] * vec[1] - quat[2] * vec[0],
        ];
        // SAFETY: res_ptr points to caller-owned memory (C ABI array param = pointer)
        unsafe {
            *res_ptr = vec[0] + 2.0 * (quat[2] * tmp[2] - quat[3] * tmp[1]);
            *res_ptr.add(1) = vec[1] + 2.0 * (quat[3] * tmp[0] - quat[1] * tmp[2]);
            *res_ptr.add(2) = vec[2] + 2.0 * (quat[1] * tmp[1] - quat[2] * tmp[0]);
        }
    }
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
    // NOTE: In C ABI, array params are pointers. quat and normal are outputs.
    let normal_ptr = normal.as_ptr() as *mut f64;
    let edge_ptr = edge.as_ptr();
    let tprv_ptr = tprv.as_ptr();
    let tnxt_ptr = tnxt.as_ptr();

    let mut tangent: [f64; 3] = [0.0; 3];
    let mut binormal: [f64; 3] = [0.0; 3];

    // SAFETY: all array pointers valid for their respective sizes per caller contract
    unsafe {
        // normalize tangent
        mjuu_copyvec(tangent.as_mut_ptr() as *mut T1, edge_ptr as *const T2, 3);
        mjuu_normvec(tangent.as_mut_ptr(), 3);

        // compute moving frame
        if first != 0 {
            // use the first vertex binormal for the first edge
            mjuu_crossvec(binormal.as_mut_ptr(), tangent.as_ptr(), tnxt_ptr);
            mjuu_normvec(binormal.as_mut_ptr(), 3);

            // compute edge normal given tangent and binormal
            mjuu_crossvec(normal_ptr, binormal.as_ptr(), tangent.as_ptr());
            mjuu_normvec(normal_ptr, 3);
        } else {
            let darboux: [f64; 4] = [0.0; 4];

            // rotate edge normal about the vertex binormal
            mjuu_crossvec(binormal.as_mut_ptr(), tprv_ptr, tangent.as_ptr());
            let angle = f64::atan2(
                mjuu_normvec(binormal.as_mut_ptr(), 3),
                mjuu_dot3(tprv_ptr, tangent.as_ptr()),
            );
            mjuu_axis_angle2quat(darboux, [binormal[0], binormal[1], binormal[2]], angle);
            mjuu_rot_vec_quat(normal, normal, darboux);
            mjuu_normvec(normal_ptr, 3);

            // compute edge binormal given tangent and normal
            mjuu_crossvec(binormal.as_mut_ptr(), tangent.as_ptr(), normal_ptr);
            mjuu_normvec(binormal.as_mut_ptr(), 3);
        }

        // global orientation of the frame
        mjuu_frame2quat(quat.as_ptr() as *mut f64, tangent.as_ptr(), normal_ptr, binormal.as_ptr());

        // return edge length
        (mjuu_dot3(edge_ptr, edge_ptr)).sqrt()
    }
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
    // SAFETY: mat is n*n f64; eigval is n f64; eigvec is n*n f64. Caller guarantees.
    unsafe {
        let nu = n as usize;

        // initialize eigvec to identity
        for i in 0..nu * nu {
            *eigvec.add(i) = 0.0;
        }
        for i in 0..nu {
            *eigvec.add(i * nu + i) = 1.0;
        }

        let max_sweeps: i32 = 200;
        let tol: f64 = 1e-12;

        let mut sweep: i32 = 0;
        while sweep < max_sweeps {
            // check convergence: sum of squared off-diagonal elements
            let mut off_diag: f64 = 0.0;
            for i in 0..nu {
                for j in (i + 1)..nu {
                    off_diag += *mat.add(i * nu + j) * *mat.add(i * nu + j);
                }
            }
            if off_diag < tol * tol {
                break;
            }

            // sweep over all off-diagonal pairs
            for p in 0..nu {
                for q in (p + 1)..nu {
                    let apq = *mat.add(p * nu + q);
                    if apq.abs() < tol * 1e-3 {
                        continue;
                    }

                    // compute rotation angle
                    let app = *mat.add(p * nu + p);
                    let aqq = *mat.add(q * nu + q);
                    let tau = (aqq - app) / (2.0 * apq);
                    let t = (if tau >= 0.0 { 1.0 } else { -1.0 }) /
                            (tau.abs() + (1.0 + tau * tau).sqrt());
                    let c = 1.0 / (1.0 + t * t).sqrt();
                    let s = t * c;

                    // update matrix (Jacobi rotation)
                    *mat.add(p * nu + p) -= t * apq;
                    *mat.add(q * nu + q) += t * apq;
                    *mat.add(p * nu + q) = 0.0;
                    *mat.add(q * nu + p) = 0.0;

                    for r in 0..nu {
                        if r == p || r == q {
                            continue;
                        }
                        let mrp = *mat.add(r * nu + p);
                        let mrq = *mat.add(r * nu + q);
                        *mat.add(r * nu + p) = c * mrp - s * mrq;
                        *mat.add(p * nu + r) = c * mrp - s * mrq;
                        *mat.add(r * nu + q) = s * mrp + c * mrq;
                        *mat.add(q * nu + r) = s * mrp + c * mrq;
                    }

                    // accumulate eigenvectors
                    for r in 0..nu {
                        let vrp = *eigvec.add(r * nu + p);
                        let vrq = *eigvec.add(r * nu + q);
                        *eigvec.add(r * nu + p) = c * vrp - s * vrq;
                        *eigvec.add(r * nu + q) = s * vrp + c * vrq;
                    }
                }
            }

            sweep += 1;
        }

        // extract eigenvalues from diagonal
        for i in 0..nu {
            *eigval.add(i) = *mat.add(i * nu + i);
        }

        sweep
    }
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
    // res = quat*vec + pos
    mjuu_rot_vec_quat(res, vec, quat);
    // SAFETY: res is a C ABI array param (pointer to caller-owned f64[3])
    unsafe {
        let res_ptr = res.as_ptr() as *mut f64;
        *res_ptr += pos[0];
        *res_ptr.add(1) += pos[1];
        *res_ptr.add(2) += pos[2];
    }
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
    // NOTE: In C ABI, array params are pointers. quat and inertia are outputs.
    let quat_ptr = quat.as_ptr() as *mut f64;
    let inertia_ptr = inertia.as_ptr() as *mut f64;
    let fullinertia_ptr = fullinertia.as_ptr();

    // SAFETY: fullinertia_ptr points to caller-owned memory (C ABI array param = pointer)
    if !mjuu_defined(unsafe { *fullinertia_ptr }) {
        return core::ptr::null();
    }

    let eigval: [f64; 3] = [0.0; 3];
    let eigvec: [f64; 9] = [0.0; 9];
    let quattmp: [f64; 4] = [0.0; 4];
    // SAFETY: fullinertia_ptr valid for 6 elements per caller contract
    let full: [f64; 9] = unsafe {
        [
            *fullinertia_ptr.add(0), *fullinertia_ptr.add(3), *fullinertia_ptr.add(4),
            *fullinertia_ptr.add(3), *fullinertia_ptr.add(1), *fullinertia_ptr.add(5),
            *fullinertia_ptr.add(4), *fullinertia_ptr.add(5), *fullinertia_ptr.add(2),
        ]
    };

    mjuu_eig3(eigval, eigvec, quattmp, full);

    const MJEPS: f64 = 1e-14;
    // SAFETY: eigval is a local array, indexing is in bounds
    if eigval[2] < MJEPS {
        return b"inertia must have positive eigenvalues\0".as_ptr() as *const i8;
    }

    // SAFETY: quat_ptr points to caller-owned memory (C ABI array param = pointer)
    if !quat_ptr.is_null() {
        unsafe {
            *quat_ptr.add(0) = quattmp[0];
            *quat_ptr.add(1) = quattmp[1];
            *quat_ptr.add(2) = quattmp[2];
            *quat_ptr.add(3) = quattmp[3];
        }
    }

    // SAFETY: inertia_ptr points to caller-owned memory (C ABI array param = pointer)
    if !inertia_ptr.is_null() {
        unsafe {
            *inertia_ptr.add(0) = eigval[0];
            *inertia_ptr.add(1) = eigval[1];
            *inertia_ptr.add(2) = eigval[2];
        }
    }

    core::ptr::null()
}

/// C: FilePath::IsAbs (user/user_util.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_abs(self_ptr: *mut FilePath) -> bool {
    if self_ptr.is_null() {
        return false;
    }
    extern "C" { fn FilePath_IsAbs(self_ptr: *mut FilePath) -> bool; }
    unsafe { FilePath_IsAbs(self_ptr) }
}

/// C: FilePath::AbsPrefix (user/user_util.h:195)
/// Returns the byte-length of the absolute prefix of the path stored in this FilePath.
/// The path is stored as a libc++ std::string (24 bytes with SSO on macOS).
#[allow(unused_variables, non_snake_case)]
pub fn file_path_abs_prefix(self_ptr: *mut FilePath) -> i32 {
    if self_ptr.is_null() {
        return 0;
    }
    extern "C" { #[link_name = "_ZN6mujoco4user8FilePath9AbsPrefixEv"] fn FilePath_AbsPrefix(self_ptr: *mut FilePath) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ FilePath::AbsPrefix
    unsafe { FilePath_AbsPrefix(self_ptr) }
}

/// C: FilePath::Str (user/user_util.h:198)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str(self_ptr: *mut FilePath) -> *const i32 {
    if self_ptr.is_null() {
        return core::ptr::null();
    }
    extern "C" { fn FilePath_Str(self_ptr: *mut FilePath) -> *const i32; }
    unsafe { FilePath_Str(self_ptr) }
}

/// C: FilePath::StrLower (user/user_util.h:202)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str_lower(self_ptr: *mut FilePath) -> std__string {
    if self_ptr.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn FilePath_StrLower(self_ptr: *mut FilePath) -> std__string; }
    unsafe { FilePath_StrLower(self_ptr) }
}

/// C: FilePath::Ext (user/user_util.h:205)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_ext(self_ptr: *mut FilePath) -> std__string {
    if self_ptr.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn FilePath_Ext(self_ptr: *mut FilePath) -> std__string; }
    // SAFETY: self_ptr verified non-null; delegates to C++ FilePath::Ext
    unsafe { FilePath_Ext(self_ptr) }
}

/// C: FilePath::StripExt (user/user_util.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_ext(self_ptr: *mut FilePath) -> FilePath {
    if self_ptr.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn FilePath_StripExt(self_ptr: *mut FilePath) -> FilePath; }
    unsafe { FilePath_StripExt(self_ptr) }
}

/// C: FilePath::StripPath (user/user_util.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_path(self_ptr: *mut FilePath) -> FilePath {
    if self_ptr.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn FilePath_StripPath(self_ptr: *mut FilePath) -> FilePath; }
    unsafe { FilePath_StripPath(self_ptr) }
}

/// C: FilePath::Lower (user/user_util.h:217)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_lower(self_ptr: *mut FilePath) -> FilePath {
    if self_ptr.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn FilePath_Lower(self_ptr: *mut FilePath) -> FilePath; }
    unsafe { FilePath_Lower(self_ptr) }
}

/// C: FilePath::size (user/user_util.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_size(self_ptr: *mut FilePath) -> std__size_t {
    if self_ptr.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn FilePath_size(self_ptr: *mut FilePath) -> std__size_t; }
    unsafe { FilePath_size(self_ptr) }
}

/// C: FilePath::c_str (user/user_util.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_c_str(self_ptr: *mut FilePath) -> *const i8 {
    if self_ptr.is_null() {
        return core::ptr::null();
    }
    extern "C" { fn FilePath_c_str(self_ptr: *mut FilePath) -> *const i8; }
    unsafe { FilePath_c_str(self_ptr) }
}

/// C: FilePath::empty (user/user_util.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_empty(self_ptr: *mut FilePath) -> bool {
    if self_ptr.is_null() {
        return true;
    }
    extern "C" { fn FilePath_empty(self_ptr: *mut FilePath) -> bool; }
    unsafe { FilePath_empty(self_ptr) }
}

/// C: FilePath::PathReduce (user/user_util.h:227)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_path_reduce(str: *const std__string) -> std__string {
    if str.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn FilePath_PathReduce(str: *const std__string) -> std__string; }
    unsafe { FilePath_PathReduce(str) }
}

/// C: FilePath::IsSeparator (user/user_util.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_separator(c: i8) -> bool {
    // C++ IsSeparator: return c == '/' || c == '\\';
    c == b'/' as i8 || c == b'\\' as i8
}

/// C: FilePath::Combine (user/user_util.h:231)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_combine(s1: *const std__string, s2: *const std__string) -> std__string {
    if s1.is_null() || s2.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn FilePath_Combine(s1: *const std__string, s2: *const std__string) -> std__string; }
    // SAFETY: s1 and s2 verified non-null; delegates to C++ implementation
    unsafe { FilePath_Combine(s1, s2) }
}

/// C: FilePath::FilePathFast (user/user_util.h:234)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_file_path_fast(str: *const i32) -> FilePath {
    if str.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn FilePath_FilePathFast(str: *const i32) -> FilePath; }
    unsafe { FilePath_FilePathFast(str) }
}

/// C: mjuu_strippath (user/user_util.h:273)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_strippath(filename: string) -> std__string {
    let _sv = core::mem::size_of_val(&filename);
    extern "C" { fn mjuu_strippath(filename: string) -> std__string; }
    // SAFETY: C strips directory path from filename
    unsafe { mjuu_strippath(filename) }
}

/// C: mjuu_stripext (user/user_util.h:276)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_stripext(filename: string) -> std__string {
    let _sv = core::mem::size_of_val(&filename);
    extern "C" { fn mjuu_stripext(filename: string) -> std__string; }
    // SAFETY: C strips file extension from filename
    unsafe { mjuu_stripext(filename) }
}

/// C: mjuu_getext (user/user_util.h:279)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_getext(filename: string_view) -> std__string {
    let _sv = core::mem::size_of_val(&filename);
    extern "C" { fn mjuu_getext(filename: string_view) -> std__string; }
    // SAFETY: C extracts file extension from filename
    unsafe { mjuu_getext(filename) }
}

/// C: mjuu_isabspath (user/user_util.h:282)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_isabspath(path: string) -> bool {
    let _sv = core::mem::size_of_val(&path);
    extern "C" { fn mjuu_isabspath(path: string) -> bool; }
    // SAFETY: C checks if path is absolute
    unsafe { mjuu_isabspath(path) }
}

/// C: mjuu_combinePaths (user/user_util.h:285)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_combine_paths(path1: *const std__string, path2: *const std__string) -> std__string {
    let _sv = core::mem::size_of_val(&path1);
    extern "C" { fn mjuu_combinePaths(path1: *const std__string, path2: *const std__string) -> std__string; }
    // SAFETY: C combines two path strings
    unsafe { mjuu_combinePaths(path1, path2) }
}

/// C: mjuu_parseContentTypeAttrType (user/user_util.h:290)
/// Calls: mjuu_isValidContentType
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_type(text: string_view) -> *const () {
    extern "C" { fn mjuu_parseContentTypeAttrType(text: string_view) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjuu_parseContentTypeAttrType(text) }
}

/// C: mjuu_parseContentTypeAttrSubtype (user/user_util.h:293)
/// Calls: mjuu_isValidContentType
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_subtype(text: string_view) -> *const () {
    extern "C" { fn mjuu_parseContentTypeAttrSubtype(text: string_view) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjuu_parseContentTypeAttrSubtype(text) }
}

/// C: mjuu_extToContentType (user/user_util.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_ext_to_content_type(filename: string_view) -> std__string {
    let _sv = core::mem::size_of_val(&filename);
    extern "C" { fn mjuu_extToContentType(filename: string_view) -> std__string; }
    // SAFETY: C maps file extension to MIME content type
    unsafe { mjuu_extToContentType(filename) }
}

/// C: mjuu_dirnamelen (user/user_util.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dirnamelen(path: *const i8) -> i32 {
    // SAFETY: path is a valid null-terminated C string, or null.
    unsafe {
        if path.is_null() {
            return 0;
        }

        let mut pos: i32 = -1;
        let mut i: i32 = 0;
        while *path.add(i as usize) != 0 {
            if *path.add(i as usize) == b'/' as i8 || *path.add(i as usize) == b'\\' as i8 {
                pos = i;
            }
            i += 1;
        }

        pos + 1
    }
}

