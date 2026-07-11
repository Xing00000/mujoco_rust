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
    // SAFETY: vec has 4 elements.
    unsafe {
        *vec.add(0) = f0;
        *vec.add(1) = f1;
        *vec.add(2) = f2;
        *vec.add(3) = f3;
    }
}

/// C: mjr_setf3 (render/classic/render_util.h:32)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_setf3(vec: *mut f32, f0: f32, f1: f32, f2: f32) {
    // SAFETY: caller guarantees vec points to at least 3 contiguous f32 elements
    unsafe {
        *vec.add(0) = f0;
        *vec.add(1) = f1;
        *vec.add(2) = f2;
    }
}

/// C: mjr_mulMat44 (render/classic/render_util.h:35)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_mul_mat44(res: *mut f32, A: *const f32, B: *const f32) {
    if res.is_null() { return; }
    // SAFETY: caller guarantees res points to 16 writable f32,
    //         A and B each point to 16 contiguous f32 (4x4 column-major)
    unsafe {
        let mut r: i32 = 0;
        while r < 4 {
            let mut c: i32 = 0;
            while c < 4 {
                *res.add((r + 4 * c) as usize) = 0.0;
                let mut i: i32 = 0;
                while i < 4 {
                    *res.add((r + 4 * c) as usize) += *A.add((r + 4 * i) as usize) * *B.add((i + 4 * c) as usize);
                    i += 1;
                }
                c += 1;
            }
            r += 1;
        }
    }
}

/// C: mjr_getrow4 (render/classic/render_util.h:38)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_getrow4(res: *mut f32, A: *const f32, r: i32) {
    if res.is_null() { return; }
    // SAFETY: caller guarantees res points to 4 writable f32,
    //         A points to 16 contiguous f32 (4x4 column-major)
    unsafe {
        *res.add(0) = *A.add(r as usize);
        *res.add(1) = *A.add((r + 4) as usize);
        *res.add(2) = *A.add((r + 8) as usize);
        *res.add(3) = *A.add((r + 12) as usize);
    }
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
    // SAFETY: caller guarantees a and b point to at least 3 contiguous f32 elements
    unsafe {
        (*a.add(0)) * (*b.add(0)) + (*a.add(1)) * (*b.add(1)) + (*a.add(2)) * (*b.add(2))
    }
}

/// C: mjr_multiply4 (render/classic/render_util.h:53)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_multiply4(res: *mut f32, mat: *const f32, vec: *const f32) {
    // SAFETY: caller guarantees res points to 4 writable f32,
    //         mat points to 16 contiguous f32 (4x4 column-major),
    //         vec points to 4 contiguous f32
    unsafe {
        let mut i: i32 = 0;
        while i < 4 {
            *res.add(i as usize) = 0.0;
            let mut j: i32 = 0;
            while j < 4 {
                *res.add(i as usize) += (*mat.add((i + 4 * j) as usize)) * (*vec.add(j as usize));
                j += 1;
            }
            i += 1;
        }
    }
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
    if eye.is_null() { return; }
    // SAFETY: caller guarantees eye, forward, up each point to 3 contiguous f32 values
    unsafe {
        extern "C" { fn glMultMatrixf(m: *const f32); }

        let mut f: [f32; 3] = [*forward.add(0), *forward.add(1), *forward.add(2)];
        let mut s: [f32; 3] = [0.0; 3];
        let mut u: [f32; 3] = [0.0; 3];
        let mut mat: [f32; 16] = [0.0; 16];

        // prepare vectors
        mjr_normalize_vec(f.as_mut_ptr());
        mjr_cross_vec(s.as_mut_ptr(), f.as_ptr(), up);
        mjr_normalize_vec(s.as_mut_ptr());
        mjr_cross_vec(u.as_mut_ptr(), s.as_ptr(), f.as_ptr());
        mjr_normalize_vec(u.as_mut_ptr());

        // fill in 4x4 matrix (column-major OpenGL format)
        mat[0] = s[0];
        mat[1] = u[0];
        mat[2] = -f[0];
        mat[3] = 0.0;

        mat[4] = s[1];
        mat[5] = u[1];
        mat[6] = -f[1];
        mat[7] = 0.0;

        mat[8] = s[2];
        mat[9] = u[2];
        mat[10] = -f[2];
        mat[11] = 0.0;

        mat[12] = -mjr_dot_vec(s.as_ptr(), eye);
        mat[13] = -mjr_dot_vec(u.as_ptr(), eye);
        mat[14] = mjr_dot_vec(f.as_ptr(), eye);
        mat[15] = 1.0;

        // SAFETY: glMultMatrixf expects pointer to 16 f32 column-major matrix
        glMultMatrixf(mat.as_ptr());
    }
}

/// C: mjr_perspective (render/classic/render_util.h:59)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_perspective(fovy: f32, aspect: f32, znear: f32, zfar: f32) {
    let _sv = core::mem::size_of_val(&fovy);
    // SAFETY: implements gluPerspective replacement using glFrustum
    unsafe {
        extern "C" { fn glFrustum(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64); }

        let h: f64 = ((fovy as f64) / 360.0 * core::f64::consts::PI).tan() * (znear as f64);
        let w: f64 = h * (aspect as f64);

        // SAFETY: glFrustum sets up perspective projection matrix
        glFrustum(-w, w, -h, h, znear as f64, zfar as f64);
    }
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
    if pos.is_null() { return; }
    // SAFETY: caller guarantees pos points to 3 f32, mat points to 9 f32 (3x3 row-major)
    unsafe {
        extern "C" { fn glMultMatrixf(m: *const f32); }

        let mut reflect: [f32; 16] = [0.0; 16];
        let mut v: [f32; 3] = [0.0; 3];
        let mut vvt: [f32; 9] = [0.0; 9];

        // copy axis
        v[0] = *mat.add(2);
        v[1] = *mat.add(5);
        v[2] = *mat.add(8);

        // compute outer product v*vT
        let mut i: i32 = 0;
        while i < 3 {
            let mut j: i32 = 0;
            while j < 3 {
                vvt[(3 * i + j) as usize] = v[i as usize] * v[j as usize];
                j += 1;
            }
            i += 1;
        }

        // construct reflection matrix
        reflect[0]  = 1.0 - 2.0 * vvt[0];
        reflect[1]  =      -2.0 * vvt[1];
        reflect[2]  =      -2.0 * vvt[2];
        reflect[3]  = 0.0;
        reflect[4]  =      -2.0 * vvt[3];
        reflect[5]  = 1.0 - 2.0 * vvt[4];
        reflect[6]  =      -2.0 * vvt[5];
        reflect[7]  = 0.0;
        reflect[8]  =      -2.0 * vvt[6];
        reflect[9]  =      -2.0 * vvt[7];
        reflect[10] = 1.0 - 2.0 * vvt[8];
        reflect[11] = 0.0;
        reflect[12] = 2.0 * mjr_dot_vec(vvt.as_ptr(), pos);
        reflect[13] = 2.0 * mjr_dot_vec(vvt.as_ptr().add(3), pos);
        reflect[14] = 2.0 * mjr_dot_vec(vvt.as_ptr().add(6), pos);
        reflect[15] = 1.0;

        // SAFETY: glMultMatrixf expects pointer to 16 f32 column-major matrix
        glMultMatrixf(reflect.as_ptr());
    }
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
    if translate.is_null() { return; }
    // SAFETY: caller guarantees translate points to 3 f32, rotate points to 4 f32 (quaternion)
    unsafe {
        extern "C" {
            fn mju_f2n(res: *mut f64, vec: *const f32, n: i32);
            fn mju_quat2Mat(res: *mut f64, quat: *const f64);
            fn glMultMatrixf(m: *const f32);
        }

        let mut quat: [f64; 4] = [0.0; 4];
        let mut mat: [f64; 9] = [0.0; 9];
        let mut glmat: [f32; 16] = [0.0; 16];

        // construct matrix rotation
        // SAFETY: mju_f2n converts 4 floats to doubles
        mju_f2n(quat.as_mut_ptr(), rotate, 4);
        // SAFETY: mju_quat2Mat converts quaternion to 3x3 rotation matrix
        mju_quat2Mat(mat.as_mut_ptr(), quat.as_ptr());

        // prepare transformation matrix
        glmat[0]  = scale * (mat[0] as f32);
        glmat[1]  = scale * (mat[3] as f32);
        glmat[2]  = scale * (mat[6] as f32);
        glmat[3]  = 0.0;
        glmat[4]  = scale * (mat[1] as f32);
        glmat[5]  = scale * (mat[4] as f32);
        glmat[6]  = scale * (mat[7] as f32);
        glmat[7]  = 0.0;
        glmat[8]  = scale * (mat[2] as f32);
        glmat[9]  = scale * (mat[5] as f32);
        glmat[10] = scale * (mat[8] as f32);
        glmat[11] = 0.0;
        glmat[12] = *translate.add(0);
        glmat[13] = *translate.add(1);
        glmat[14] = *translate.add(2);
        glmat[15] = 1.0;

        // SAFETY: glMultMatrixf expects pointer to 16 f32 column-major matrix
        glMultMatrixf(glmat.as_ptr());
    }
}

/// C: mjr_findRect (render/classic/render_util.h:68)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_find_rect(x: i32, y: i32, nrect: i32, rect: *const mjrRect) -> i32  {
    if rect.is_null() { return -1; }
    // SAFETY: caller guarantees rect points to nrect contiguous mjrRect values
    unsafe {
        let mut i: i32 = 0;
        while i < nrect {
            if x >= (*rect.add(i as usize)).left &&
               x < (*rect.add(i as usize)).left + (*rect.add(i as usize)).width &&
               y >= (*rect.add(i as usize)).bottom &&
               y < (*rect.add(i as usize)).bottom + (*rect.add(i as usize)).height {
                return i;
            }
            i += 1;
        }
    }
    -1
}

