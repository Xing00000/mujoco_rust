//! Port of: render/classic/render_util.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjr_makeNormal (render/classic/render_util.h:26)
/// Calls: mjr_crossVec, mjr_normalizeVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_normal(normal: *mut f32, p1: *const f32, p2: *const f32, p3: *const f32) {
    // SAFETY: caller guarantees normal, p1, p2, p3 each point to at least 3 contiguous f32 values
    unsafe {
        let v1: [f32; 3] = [
            *p2.add(0) - *p1.add(0),
            *p2.add(1) - *p1.add(1),
            *p2.add(2) - *p1.add(2),
        ];
        let v2: [f32; 3] = [
            *p3.add(0) - *p1.add(0),
            *p3.add(1) - *p1.add(1),
            *p3.add(2) - *p1.add(2),
        ];

        mjr_cross_vec(normal, v1.as_ptr(), v2.as_ptr());
        mjr_normalize_vec(normal);
    }
}

/// C: mjr_setf4 (render/classic/render_util.h:29)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_setf4(vec: *mut f32, f0: f32, f1: f32, f2: f32, f3: f32) {
    extern "C" { fn mjr_setf4_impl(vec: *mut f32, f0: f32, f1: f32, f2: f32, f3: f32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjr_setf4_impl(vec, f0, f1, f2, f3) }
}

/// C: mjr_setf3 (render/classic/render_util.h:32)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_setf3(vec: *mut f32, f0: f32, f1: f32, f2: f32) {
    extern "C" { fn mjr_setf3_impl(vec: *mut f32, f0: f32, f1: f32, f2: f32); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_setf3_impl(vec, f0, f1, f2) }
}

/// C: mjr_mulMat44 (render/classic/render_util.h:35)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_mul_mat44(res: *mut f32, A: *const f32, B: *const f32) {
    extern "C" { fn mjr_mulMat44_impl(res: *mut f32, A: *const f32, B: *const f32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjr_mulMat44_impl(res, A, B) }
}

/// C: mjr_getrow4 (render/classic/render_util.h:38)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_getrow4(res: *mut f32, A: *const f32, r: i32) {
    extern "C" { fn mjr_getrow4_impl(res: *mut f32, A: *const f32, r: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjr_getrow4_impl(res, A, r) }
}

/// C: mjr_crossVec (render/classic/render_util.h:41)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_cross_vec(a: *mut f32, b: *const f32, c: *const f32) {
    // SAFETY: caller guarantees a, b, c each point to at least 3 contiguous f32 values
    unsafe {
        *a.add(0) = *b.add(1) * *c.add(2) - *b.add(2) * *c.add(1);
        *a.add(1) = *b.add(2) * *c.add(0) - *b.add(0) * *c.add(2);
        *a.add(2) = *b.add(0) * *c.add(1) - *b.add(1) * *c.add(0);
    }
}

/// C: mjr_normalizeVec (render/classic/render_util.h:44)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_normalize_vec(v: *mut f32) {
    // SAFETY: caller guarantees v points to at least 3 contiguous f32 values
    unsafe {
        let len: f32 = ((*v.add(0))*(*v.add(0)) + (*v.add(1))*(*v.add(1)) + (*v.add(2))*(*v.add(2))).sqrt();

        if len < 1E-10f32 {
            *v.add(0) = 0.0f32;
            *v.add(1) = 0.0f32;
            *v.add(2) = 1.0f32;
        } else {
            let scl: f32 = 1.0f32 / len;
            *v.add(0) *= scl;
            *v.add(1) *= scl;
            *v.add(2) *= scl;
        }
    }
}

/// C: mjr_orthoVec (render/classic/render_util.h:47)
/// Calls: mjr_crossVec, mjr_normalizeVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_ortho_vec(res: *mut f32, v: *const f32) {
    // SAFETY: caller guarantees res points to at least 3 f32, v points to at least 3 f32
    unsafe {
        let mut other: [f32; 3] = [-1.0, 0.0, 0.0];

        // try cross with negative X-axis
        mjr_cross_vec(res, v, other.as_ptr());

        // success
        if *res.add(0) * *res.add(0) + *res.add(1) * *res.add(1) + *res.add(2) * *res.add(2) > 0.01 {
            mjr_normalize_vec(res);
            return;
        }

        // otherwise use positive Y-axis
        other[0] = 0.0;
        other[1] = 1.0;
        mjr_cross_vec(res, v, other.as_ptr());
        mjr_normalize_vec(res);
    }
}

/// C: mjr_dotVec (render/classic/render_util.h:50)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_dot_vec(a: *const f32, b: *const f32) -> f32 {
    extern "C" { fn mjr_dotVec_impl(a: *const f32, b: *const f32) -> f32; }
    // SAFETY: delegates to C implementation
    unsafe { mjr_dotVec_impl(a, b) }
}

/// C: mjr_multiply4 (render/classic/render_util.h:53)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_multiply4(res: *mut f32, mat: *const f32, vec: *const f32) {
    extern "C" { fn mjr_multiply4_impl(res: *mut f32, mat: *const f32, vec: *const f32); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_multiply4_impl(res, mat, vec) }
}

/// C: mjr_lookAt (render/classic/render_util.h:56)
/// Calls: mjr_crossVec, mjr_dotVec, mjr_normalizeVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_look_at(eye: *const f32, forward: *const f32, up: *const f32) {
    extern "C" { fn mjr_lookAt_impl(eye: *const f32, forward: *const f32, up: *const f32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjr_lookAt_impl(eye, forward, up) }
}

/// C: mjr_perspective (render/classic/render_util.h:59)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_perspective(fovy: f32, aspect: f32, znear: f32, zfar: f32) {
    extern "C" { fn mjr_perspective_impl(fovy: f32, aspect: f32, znear: f32, zfar: f32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjr_perspective_impl(fovy, aspect, znear, zfar) }
}

/// C: mjr_reflect (render/classic/render_util.h:62)
/// Calls: mjr_dotVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_reflect(pos: *const f32, mat: *const f32) {
    extern "C" { fn mjr_reflect_impl(pos: *const f32, mat: *const f32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjr_reflect_impl(pos, mat) }
}

/// C: mjr_transform (render/classic/render_util.h:65)
/// Calls: mju_f2n, mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_transform(translate: *const f32, rotate: *const f32, scale: f32) {
    // WARNING: signature changed — verify body
    // Previous params: (translate : * const f32, rotate : * const f32, scale : f32)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_findRect (render/classic/render_util.h:68)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_find_rect(x: i32, y: i32, nrect: i32, rect: *const mjrRect) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (x : i32, y : i32, nrect : i32, rect : * const mjrRect)
    // Previous return: i32
    todo ! ()
}

