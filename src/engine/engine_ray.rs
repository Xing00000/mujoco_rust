//! Port of: engine/engine_ray.c
//! IR hash: 545f394232195ad9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ray_map (engine/engine_ray.c:38)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_map(pos: *const f64, mat: *const f64, pnt: *const f64, vec: *const f64, lpnt: *mut f64, lvec: *mut f64) {
    // SAFETY: caller guarantees pos[3], mat[9], pnt[3], vec[3], lpnt[3], lvec[3] are valid
    unsafe {
        let dif0: f64 = *pnt.add(0) - *pos.add(0);
        let dif1: f64 = *pnt.add(1) - *pos.add(1);
        let dif2: f64 = *pnt.add(2) - *pos.add(2);
        *lpnt.add(0) = *mat.add(0) * dif0 + *mat.add(3) * dif1 + *mat.add(6) * dif2;
        *lpnt.add(1) = *mat.add(1) * dif0 + *mat.add(4) * dif1 + *mat.add(7) * dif2;
        *lpnt.add(2) = *mat.add(2) * dif0 + *mat.add(5) * dif1 + *mat.add(8) * dif2;
        *lvec.add(0) = *mat.add(0) * *vec.add(0) + *mat.add(3) * *vec.add(1) + *mat.add(6) * *vec.add(2);
        *lvec.add(1) = *mat.add(1) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(7) * *vec.add(2);
        *lvec.add(2) = *mat.add(2) * *vec.add(0) + *mat.add(5) * *vec.add(1) + *mat.add(8) * *vec.add(2);
    }
}

/// C: longitude (engine/engine_ray.c:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn longitude(vec: *const f64) -> f64 {
    // SAFETY: caller guarantees vec points to array of at least 3 f64
    unsafe {
        (*vec.add(1)).atan2(*vec.add(0))
    }
}

/// C: latitude (engine/engine_ray.c:62)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn latitude(vec: *const f64) -> f64 {
    // SAFETY: caller guarantees vec points to array of at least 3 f64
    unsafe {
        let v0 = *vec.add(0);
        let v1 = *vec.add(1);
        let v2 = *vec.add(2);
        (v0 * v0 + v1 * v1).sqrt().atan2(v2)
    }
}

/// C: ray_eliminate (engine/engine_ray.c:68)
#[allow(unused_variables, non_snake_case)]
pub fn ray_eliminate(m: *const mjModel, d: *const mjData, geomid: i32, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32) -> i32 {
    extern "C" { fn ray_eliminate_impl(m: *const mjModel, d: *const mjData, geomid: i32, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32) -> i32; }
    // SAFETY: delegates to C implementation; caller guarantees all pointers are valid
    unsafe { ray_eliminate_impl(m, d, geomid, geomgroup, flg_static, bodyexclude) }
}

/// C: ray_quad (engine/engine_ray.c:103)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_quad(a: f64, b: f64, c: f64, x: *mut f64) -> f64 {
    const MJMINVAL: f64 = 1e-15;
    let mut det: f64 = b * b - a * c;
    if det < 0.0 || a < MJMINVAL {
        // SAFETY: caller guarantees x points to array of at least 2 f64
        unsafe {
            *x.add(0) = -1.0;
            *x.add(1) = -1.0;
        }
        return -1.0;
    }
    det = det.sqrt();
    // SAFETY: caller guarantees x points to array of at least 2 f64
    unsafe {
        *x.add(0) = (-b - det) / a;
        *x.add(1) = (-b + det) / a;
        if *x.add(0) >= 0.0 {
            return *x.add(0);
        } else if *x.add(1) >= 0.0 {
            return *x.add(1);
        }
    }
    -1.0
}

/// C: ray_plane (engine/engine_ray.c:204)
/// Calls: mju_zero3, ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_plane(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    const MJMINVAL: f64 = 1e-15;
    // SAFETY: caller guarantees pos[3], mat[9], size[3], pnt[3], vec[3] are valid; normal may be null
    unsafe {
        // clear normal if given
        if !normal.is_null() {
            crate::engine::engine_util_blas::mju_zero3(normal);
        }

        // map to local frame
        let mut lpnt = [0.0f64; 3];
        let mut lvec = [0.0f64; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // z-vec not pointing towards front face: reject
        if lvec[2] > -MJMINVAL {
            return -1.0;
        }

        // intersection with plane
        let x: f64 = -lpnt[2] / lvec[2];
        if x < 0.0 {
            return -1.0;
        }
        let p0: f64 = lpnt[0] + x * lvec[0];
        let p1: f64 = lpnt[1] + x * lvec[1];

        // accept only within rendered rectangle
        if (*size.add(0) <= 0.0 || p0.abs() <= *size.add(0)) &&
           (*size.add(1) <= 0.0 || p1.abs() <= *size.add(1)) {
            if !normal.is_null() {
                *normal.add(0) = *mat.add(2);
                *normal.add(1) = *mat.add(5);
                *normal.add(2) = *mat.add(8);
            }
            return x;
        } else {
            return -1.0;
        }
    }
}

/// C: ray_sphere (engine/engine_ray.c:242)
/// Calls: mju_addScl3, mju_normalize3, mju_sub3, mju_zero3, ray_quad
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_sphere(pos: *const f64, mat: *const f64, dist_sqr: f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: caller guarantees pos[3], pnt[3], vec[3] valid; mat and normal may be null
    unsafe {
        // (x*vec+pnt-pos)'*(x*vec+pnt-pos) = dist_sqr
        let dif: [f64; 3] = [
            *pnt.add(0) - *pos.add(0),
            *pnt.add(1) - *pos.add(1),
            *pnt.add(2) - *pos.add(2),
        ];
        let a: f64 = *vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1) + *vec.add(2) * *vec.add(2);
        let b: f64 = *vec.add(0) * dif[0] + *vec.add(1) * dif[1] + *vec.add(2) * dif[2];
        let c: f64 = dif[0] * dif[0] + dif[1] * dif[1] + dif[2] * dif[2] - dist_sqr;

        // solve a*x^2 + 2*b*x + c = 0
        let mut xx = [0.0f64; 2];
        let x: f64 = ray_quad(a, b, c, xx.as_mut_ptr());

        // compute normal if required
        if !normal.is_null() {
            if x < 0.0 {
                crate::engine::engine_util_blas::mju_zero3(normal);
            } else {
                // normal at surface intersection s (global frame)
                let mut s = [0.0f64; 3];
                crate::engine::engine_util_blas::mju_add_scl3(s.as_mut_ptr(), pnt, vec, x);
                crate::engine::engine_util_blas::mju_sub3(normal, s.as_ptr(), pos);
                crate::engine::engine_util_blas::mju_normalize3(normal);
            }
        }

        x
    }
}

/// C: ray_capsule (engine/engine_ray.c:272)
/// Calls: mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_capsule(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: caller guarantees pos[3], mat[9], size[3], pnt[3], vec[3] valid; normal may be null
    unsafe {
        // bounding sphere test
        let ssz: f64 = *size.add(0) + *size.add(1);
        if ray_sphere(pos, std::ptr::null(), ssz * ssz, pnt, vec, std::ptr::null_mut()) < 0.0 {
            if !normal.is_null() {
                crate::engine::engine_util_blas::mju_zero3(normal);
            }
            return -1.0;
        }

        // map to local frame
        let mut lpnt = [0.0f64; 3];
        let mut lvec = [0.0f64; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;
        let mut sol: f64;
        let mut xx = [0.0f64; 2];
        let mut type_: i32; // -1: bottom, 0: cylinder, 1: top
        type_ = 0; // placeholder, set below

        // cylinder round side: (x*lvec+lpnt)'*(x*lvec+lpnt) = size[0]*size[0]
        let mut a: f64 = lvec[0] * lvec[0] + lvec[1] * lvec[1];
        let mut b: f64 = lvec[0] * lpnt[0] + lvec[1] * lpnt[1];
        let mut c: f64 = lpnt[0] * lpnt[0] + lpnt[1] * lpnt[1] - *size.add(0) * *size.add(0);

        // solve a*x^2 + 2*b*x + c = 0
        sol = ray_quad(a, b, c, xx.as_mut_ptr());

        // make sure round solution is between flat sides
        if sol >= 0.0 && (lpnt[2] + sol * lvec[2]).abs() <= *size.add(1) {
            if x < 0.0 || sol < x {
                x = sol;
                type_ = 0;
            }
        }

        // top cap
        let mut ldif: [f64; 3] = [lpnt[0], lpnt[1], lpnt[2] - *size.add(1)];
        a = lvec[0] * lvec[0] + lvec[1] * lvec[1] + lvec[2] * lvec[2];
        b = lvec[0] * ldif[0] + lvec[1] * ldif[1] + lvec[2] * ldif[2];
        c = ldif[0] * ldif[0] + ldif[1] * ldif[1] + ldif[2] * ldif[2] - *size.add(0) * *size.add(0);
        ray_quad(a, b, c, xx.as_mut_ptr());

        // accept only top half of sphere
        for i in 0..2 {
            if xx[i] >= 0.0 && lpnt[2] + xx[i] * lvec[2] >= *size.add(1) {
                if x < 0.0 || xx[i] < x {
                    x = xx[i];
                    type_ = 1;
                }
            }
        }

        // bottom cap
        ldif[2] = lpnt[2] + *size.add(1);
        b = lvec[0] * ldif[0] + lvec[1] * ldif[1] + lvec[2] * ldif[2];
        c = ldif[0] * ldif[0] + ldif[1] * ldif[1] + ldif[2] * ldif[2] - *size.add(0) * *size.add(0);
        ray_quad(a, b, c, xx.as_mut_ptr());

        // accept only bottom half of sphere
        for i in 0..2 {
            if xx[i] >= 0.0 && lpnt[2] + xx[i] * lvec[2] <= -*size.add(1) {
                if x < 0.0 || xx[i] < x {
                    x = xx[i];
                    type_ = -1;
                }
            }
        }

        // compute normal if required
        if !normal.is_null() {
            if x < 0.0 {
                crate::engine::engine_util_blas::mju_zero3(normal);
            } else {
                *normal.add(0) = lpnt[0] + lvec[0] * x;
                *normal.add(1) = lpnt[1] + lvec[1] * x;
                *normal.add(2) = if type_ == 0 { 0.0 } else { lpnt[2] + lvec[2] * x - *size.add(1) * (type_ as f64) };

                // normalize, rotate into global frame
                crate::engine::engine_util_blas::mju_normalize3(normal);
                crate::engine::engine_util_blas::mju_mul_mat_vec3(normal, mat, normal);
            }
        }

        x
    }
}

/// C: ray_ellipsoid (engine/engine_ray.c:358)
/// Calls: mju_addScl3, mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_ellipsoid(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: caller guarantees pos[3], mat[9], size[3], pnt[3], vec[3] valid; normal may be null
    unsafe {
        // map to local frame
        let mut lpnt = [0.0f64; 3];
        let mut lvec = [0.0f64; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // invert size^2
        let s: [f64; 3] = [
            1.0 / (*size.add(0) * *size.add(0)),
            1.0 / (*size.add(1) * *size.add(1)),
            1.0 / (*size.add(2) * *size.add(2)),
        ];

        // (x*lvec+lpnt)' * diag(1./size^2) * (x*lvec+lpnt) = 1
        let a: f64 = s[0] * lvec[0] * lvec[0] + s[1] * lvec[1] * lvec[1] + s[2] * lvec[2] * lvec[2];
        let b: f64 = s[0] * lvec[0] * lpnt[0] + s[1] * lvec[1] * lpnt[1] + s[2] * lvec[2] * lpnt[2];
        let c: f64 = s[0] * lpnt[0] * lpnt[0] + s[1] * lpnt[1] * lpnt[1] + s[2] * lpnt[2] * lpnt[2] - 1.0;

        // solve a*x^2 + 2*b*x + c = 0
        let mut xx = [0.0f64; 2];
        let x: f64 = ray_quad(a, b, c, xx.as_mut_ptr());

        // compute normal if required
        if !normal.is_null() {
            if x < 0.0 {
                crate::engine::engine_util_blas::mju_zero3(normal);
            } else {
                // surface intersection (local frame)
                let mut l = [0.0f64; 3];
                crate::engine::engine_util_blas::mju_add_scl3(l.as_mut_ptr(), lpnt.as_ptr(), lvec.as_ptr(), x);

                // gradient of ellipsoid function
                *normal.add(0) = s[0] * l[0];
                *normal.add(1) = s[1] * l[1];
                *normal.add(2) = s[2] * l[2];

                // normalize, rotate into global frame
                crate::engine::engine_util_blas::mju_normalize3(normal);
                crate::engine::engine_util_blas::mju_mul_mat_vec3(normal, mat, normal);
            }
        }

        x
    }
}

/// C: ray_cylinder (engine/engine_ray.c:401)
/// Calls: mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_cylinder(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    const MJMINVAL: f64 = 1e-15;
    // SAFETY: caller guarantees pos[3], mat[9], size[3], pnt[3], vec[3] valid; normal may be null
    unsafe {
        // bounding sphere test
        let ssz: f64 = *size.add(0) * *size.add(0) + *size.add(1) * *size.add(1);
        if ray_sphere(pos, std::ptr::null(), ssz, pnt, vec, std::ptr::null_mut()) < 0.0 {
            if !normal.is_null() {
                crate::engine::engine_util_blas::mju_zero3(normal);
            }
            return -1.0;
        }

        // map to local frame
        let mut lpnt = [0.0f64; 3];
        let mut lvec = [0.0f64; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;
        let mut sol: f64;
        let mut type_: i32 = 0; // -1: bottom, 0: round, 1: top

        // flat sides
        if lvec[2].abs() > MJMINVAL {
            let mut side: i32 = -1;
            while side <= 1 {
                // solution of: lpnt[2] + x*lvec[2] = side*height_size
                sol = ((side as f64) * *size.add(1) - lpnt[2]) / lvec[2];

                // process if non-negative
                if sol >= 0.0 {
                    // intersection with horizontal face
                    let p0: f64 = lpnt[0] + sol * lvec[0];
                    let p1: f64 = lpnt[1] + sol * lvec[1];

                    // accept within radius
                    if p0 * p0 + p1 * p1 <= *size.add(0) * *size.add(0) {
                        if x < 0.0 || sol < x {
                            x = sol;
                            type_ = side;
                        }
                    }
                }
                side += 2;
            }
        }

        // round side: (x*lvec+lpnt)'*(x*lvec+lpnt) = size[0]*size[0]
        let a: f64 = lvec[0] * lvec[0] + lvec[1] * lvec[1];
        let b: f64 = lvec[0] * lpnt[0] + lvec[1] * lpnt[1];
        let c: f64 = lpnt[0] * lpnt[0] + lpnt[1] * lpnt[1] - *size.add(0) * *size.add(0);

        // solve a*x^2 + 2*b*x + c = 0
        let mut xx = [0.0f64; 2];
        sol = ray_quad(a, b, c, xx.as_mut_ptr());

        // make sure round solution is between flat sides
        if sol >= 0.0 && (lpnt[2] + sol * lvec[2]).abs() <= *size.add(1) {
            if x < 0.0 || sol < x {
                x = sol;
                type_ = 0;
            }
        }

        // compute normal if required
        if !normal.is_null() {
            if x < 0.0 {
                crate::engine::engine_util_blas::mju_zero3(normal);
            } else {
                // round side
                if type_ == 0 {
                    // normal at surface intersection (local frame)
                    *normal.add(0) = lpnt[0] + lvec[0] * x;
                    *normal.add(1) = lpnt[1] + lvec[1] * x;
                    *normal.add(2) = 0.0;
                    crate::engine::engine_util_blas::mju_normalize3(normal);
                }
                // flat sides
                else {
                    *normal.add(0) = 0.0;
                    *normal.add(1) = 0.0;
                    *normal.add(2) = type_ as f64;
                }

                // rotate into global frame
                crate::engine::engine_util_blas::mju_mul_mat_vec3(normal, mat, normal);
            }
        }

        x
    }
}

/// C: ray_box (engine/engine_ray.c:490)
/// Calls: mju_mulMatVec3, mju_zero3, ray_map, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_box(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, all: *mut f64, normal: *mut f64) -> f64 {
    const MJMINVAL: f64 = 1e-15;
    // SAFETY: caller guarantees pos[3], mat[9], size[3], pnt[3], vec[3] valid; all and normal may be null
    unsafe {
        // clear outputs
        if !all.is_null() {
            *all.add(0) = -1.0;
            *all.add(1) = -1.0;
            *all.add(2) = -1.0;
            *all.add(3) = -1.0;
            *all.add(4) = -1.0;
            *all.add(5) = -1.0;
        }
        if !normal.is_null() {
            crate::engine::engine_util_blas::mju_zero3(normal);
        }

        // bounding sphere test
        let ssz: f64 = *size.add(0) * *size.add(0) + *size.add(1) * *size.add(1) + *size.add(2) * *size.add(2);
        if ray_sphere(pos, std::ptr::null(), ssz, pnt, vec, std::ptr::null_mut()) < 0.0 {
            return -1.0;
        }

        // faces
        let iface: [[usize; 2]; 3] = [
            [1, 2],
            [0, 2],
            [0, 1],
        ];

        // map to local frame
        let mut lpnt = [0.0f64; 3];
        let mut lvec = [0.0f64; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;
        let mut sol: f64;
        let mut face_side: i32 = 0;
        let mut face_axis: i32 = -1;

        // loop over axes with non-zero vec
        for i in 0..3i32 {
            if lvec[i as usize].abs() > MJMINVAL {
                let mut side: i32 = -1;
                while side <= 1 {
                    // solution of: lpnt[i] + x*lvec[i] = side*size[i]
                    sol = ((side as f64) * *size.add(i as usize) - lpnt[i as usize]) / lvec[i as usize];

                    // process if non-negative
                    if sol >= 0.0 {
                        // intersection with face
                        let p0: f64 = lpnt[iface[i as usize][0]] + sol * lvec[iface[i as usize][0]];
                        let p1: f64 = lpnt[iface[i as usize][1]] + sol * lvec[iface[i as usize][1]];

                        // accept within rectangle
                        if p0.abs() <= *size.add(iface[i as usize][0]) &&
                           p1.abs() <= *size.add(iface[i as usize][1]) {
                            // update
                            if x < 0.0 || sol < x {
                                x = sol;
                                face_axis = i;
                                face_side = side;
                            }

                            // save in all
                            if !all.is_null() {
                                *all.add((2 * i + (side + 1) / 2) as usize) = sol;
                            }
                        }
                    }
                    side += 2;
                }
            }
        }

        // compute normal if required
        if !normal.is_null() && x >= 0.0 {
            let mut n_local: [f64; 3] = [0.0, 0.0, 0.0];
            n_local[face_axis as usize] = face_side as f64;
            crate::engine::engine_util_blas::mju_mul_mat_vec3(normal, mat, n_local.as_ptr());
        }

        x
    }
}

/// C: mju_raySlab (engine/engine_ray.c:743)
/// Calls: ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_slab(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64, vec: *const f64) -> i32 {
    // SAFETY: caller guarantees aabb[6], xpos[3], xmat[9], pnt[3], vec[3] are valid
    unsafe {
        let mut tmin: f64 = 0.0;
        let mut tmax: f64 = f64::INFINITY;

        // compute min and max
        let min: [f64; 3] = [
            *aabb.add(0) - *aabb.add(3),
            *aabb.add(1) - *aabb.add(4),
            *aabb.add(2) - *aabb.add(5),
        ];
        let max: [f64; 3] = [
            *aabb.add(0) + *aabb.add(3),
            *aabb.add(1) + *aabb.add(4),
            *aabb.add(2) + *aabb.add(5),
        ];

        // compute ray in local coordinates
        let mut src = [0.0f64; 3];
        let mut dir = [0.0f64; 3];
        ray_map(xpos, xmat, pnt, vec, src.as_mut_ptr(), dir.as_mut_ptr());

        // check intersections
        let invdir: [f64; 3] = [1.0 / dir[0], 1.0 / dir[1], 1.0 / dir[2]];
        for d in 0..3 {
            let t1: f64 = (min[d] - src[d]) * invdir[d];
            let t2: f64 = (max[d] - src[d]) * invdir[d];
            let minval: f64 = if t1 < t2 { t1 } else { t2 };
            let maxval: f64 = if t1 < t2 { t2 } else { t1 };
            tmin = if tmin > minval { tmin } else { minval };
            tmax = if tmax < maxval { tmax } else { maxval };
        }

        (tmin < tmax) as i32
    }
}

/// C: mju_rayTree (engine/engine_ray.c:771)
/// Calls: mju_addScl3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatVec3, mju_normalize3, mju_raySlab, mju_zero3, ray_map, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_tree(m: *const mjModel, d: *const mjData, id: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn mju_rayTree_impl(m: *const mjModel, d: *const mjData, id: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_rayTree_impl(m, d, id, pnt, vec, normal) }
}

/// C: mj_raySdf (engine/engine_ray.c:885)
/// Calls: mjc_distance, mjc_getSDF, mjc_gradient, mju_addScl3, mju_mulMatVec3, mju_normalize3, mju_zero3, ray_box, ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_sdf(m: *const mjModel, d: *const mjData, g: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn mj_raySdf_impl(m: *const mjModel, d: *const mjData, g: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_raySdf_impl(m, d, g, pnt, vec, normal) }
}

/// C: point_in_box (engine/engine_ray.c:1283)
/// Calls: mju_mulMatTVec3, mju_sub3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_in_box(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64) -> i32 {
    // SAFETY: caller guarantees aabb[6], xpos[3], xmat[9], pnt[3] are valid
    unsafe {
        let mut point = [0.0f64; 3];

        // compute point in local coordinates of the box
        crate::engine::engine_util_blas::mju_sub3(point.as_mut_ptr(), pnt, xpos);
        crate::engine::engine_util_blas::mju_mul_mat_t_vec3(point.as_mut_ptr(), xmat, point.as_ptr());
        crate::engine::engine_util_blas::mju_sub_from3(point.as_mut_ptr(), aabb);

        // check intersections
        for j in 0..3 {
            if point[j].abs() > *aabb.add(3 + j) {
                return 0;
            }
        }

        1
    }
}

/// C: mju_singleRay (engine/engine_ray.c:1457)
/// Calls: latitude, longitude, mj_rayHfield, mj_rayMesh, mj_raySdf, mju_add3, mju_copy3, mju_rayGeom, mju_zero3, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_single_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, ray_eliminate: *mut i32, geom_ba: *mut f64, geomid: [i32; 1], normal: *mut f64) -> f64 {
    extern "C" { fn mju_singleRay_impl(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, ray_eliminate: *mut i32, geom_ba: *mut f64, geomid: [i32; 1], normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_singleRay_impl(m, d, pnt, vec, ray_eliminate, geom_ba, geomid, normal) }
}

/// C: mju_multiRayPrepare (engine/engine_ray.h:26)
/// Calls: latitude, longitude, mju_addTo3, mju_copy, mju_dist3, mju_max, mju_message, mju_min, mju_mulMatVec3, mju_sub3, point_in_box, ray_eliminate
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_multi_ray_prepare(m: *const mjModel, d: *const mjData, pnt: *const f64, ray_xmat: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, cutoff: f64, geom_ba: *mut f64, geom_eliminate: *mut i32) {
    extern "C" { fn mju_multiRayPrepare_impl(m: *const mjModel, d: *const mjData, pnt: *const f64, ray_xmat: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, cutoff: f64, geom_ba: *mut f64, geom_eliminate: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mju_multiRayPrepare_impl(m, d, pnt, ray_xmat, geomgroup, flg_static, bodyexclude, cutoff, geom_ba, geom_eliminate) }
}

/// C: mj_multiRay (engine/engine_ray.h:34)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_dot3, mju_multiRayPrepare, mju_singleRay
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_multi_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: *mut i32, dist: *mut f64, normal: *mut f64, nray: i32, cutoff: f64) {

    extern "C" { fn mj_multiRay_impl(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: *mut i32, dist: *mut f64, normal: *mut f64, nray: i32, cutoff: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_multiRay_impl(m, d, pnt, vec, geomgroup, flg_static, bodyexclude, geomid, dist, normal, nray, cutoff) }
}

/// C: mj_ray (engine/engine_ray.h:42)
/// Calls: mj_rayHfield, mj_rayMesh, mj_raySdf, mju_copy3, mju_message, mju_norm3, mju_rayGeom, mju_zero3, ray_eliminate
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray(m: *const mjModel, d: *const mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: [i32; 1], normal: *mut f64) -> f64 {
    extern "C" { fn mj_ray_impl(m: *const mjModel, d: *const mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: [i32; 1], normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mj_ray_impl(m, d, pnt, vec, geomgroup, flg_static, bodyexclude, geomid, normal) }
}

/// C: mj_rayHfield (engine/engine_ray.h:47)
/// Calls: mju_addScl3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_normalize3, mju_round, mju_zero3, ray_box, ray_map, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_hfield(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn mj_rayHfield_impl(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_rayHfield_impl(m, d, geomid, pnt, vec, normal) }
}

/// C: ray_triangle (engine/engine_ray.h:51)
/// Calls: mju_copy3, mju_cross, mju_dot3, mju_normalize3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_triangle(v: *const [f64; 3], lpnt: *const f64, lvec: *const f64, b0: *const f64, b1: *const f64, normal: *mut f64) -> f64 {
    use crate::engine::engine_util_blas::{mju_zero3, mju_dot3, mju_sub3, mju_copy3, mju_normalize3};
    use crate::engine::engine_util_spatial::mju_cross;
    const MJMINVAL: f64 = 1e-15;

    unsafe {
        // clear normal if given
        if !normal.is_null() {
            mju_zero3(normal);
        }

        // dif = v[i] - lpnt
        let mut dif: [[f64; 3]; 3] = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                dif[i][j] = (*v.add(i))[j] - *lpnt.add(j);
            }
        }

        // project difference vectors in normal plane
        let mut planar: [[f64; 2]; 3] = [[0.0; 2]; 3];
        for i in 0..3 {
            planar[i][0] = mju_dot3(b0, dif[i].as_ptr());
            planar[i][1] = mju_dot3(b1, dif[i].as_ptr());
        }

        // reject if on the same side of any coordinate axis
        if (planar[0][0] > 0.0 && planar[1][0] > 0.0 && planar[2][0] > 0.0) ||
           (planar[0][0] < 0.0 && planar[1][0] < 0.0 && planar[2][0] < 0.0) ||
           (planar[0][1] > 0.0 && planar[1][1] > 0.0 && planar[2][1] > 0.0) ||
           (planar[0][1] < 0.0 && planar[1][1] < 0.0 && planar[2][1] < 0.0) {
            return -1.0;
        }

        // determine if origin is inside planar projection of triangle
        // A = (p0-p2, p1-p2), b = -p2, solve A*t = b
        let A: [f64; 4] = [
            planar[0][0] - planar[2][0], planar[1][0] - planar[2][0],
            planar[0][1] - planar[2][1], planar[1][1] - planar[2][1],
        ];
        let b: [f64; 2] = [-planar[2][0], -planar[2][1]];
        let det = A[0] * A[3] - A[1] * A[2];
        if det.abs() < MJMINVAL {
            return -1.0;
        }
        let t0 = (A[3] * b[0] - A[1] * b[1]) / det;
        let t1 = (-A[2] * b[0] + A[0] * b[1]) / det;

        // check if outside
        if t0 < 0.0 || t1 < 0.0 || t0 + t1 > 1.0 {
            return -1.0;
        }

        // intersect ray with plane of triangle
        mju_sub3(dif[0].as_mut_ptr(), (*v.add(0)).as_ptr(), (*v.add(2)).as_ptr());  // v0-v2
        mju_sub3(dif[1].as_mut_ptr(), (*v.add(1)).as_ptr(), (*v.add(2)).as_ptr());  // v1-v2
        mju_sub3(dif[2].as_mut_ptr(), lpnt, (*v.add(2)).as_ptr());                  // lp-v2
        let mut nrm: [f64; 3] = [0.0; 3];
        mju_cross(nrm.as_mut_ptr(), dif[0].as_ptr(), dif[1].as_ptr());  // normal to triangle plane
        let denom = mju_dot3(lvec, nrm.as_ptr());
        if denom.abs() < MJMINVAL {
            return -1.0;
        }

        // compute distance
        let x = -mju_dot3(dif[2].as_ptr(), nrm.as_ptr()) / denom;

        // compute normal if given
        if !normal.is_null() {
            mju_normalize3(nrm.as_mut_ptr());
            mju_copy3(normal, nrm.as_ptr());
        }

        x
    }
}

/// C: mj_rayMesh (engine/engine_ray.h:55)
/// Calls: mju_message, mju_rayTree, mju_zero3, ray_box
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_mesh(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn mj_rayMesh_impl(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mj_rayMesh_impl(m, d, geomid, pnt, vec, normal) }
}

/// C: mju_rayGeom (engine/engine_ray.h:59)
/// Calls: mju_message, ray_box, ray_capsule, ray_cylinder, ray_ellipsoid, ray_plane, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_geom(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, geomtype: i32, normal: *mut f64) -> f64 {
    // Geom type constants matching mjtGeom enum
    const MJGEOM_PLANE: i32 = 0;
    const MJGEOM_SPHERE: i32 = 2;
    const MJGEOM_CAPSULE: i32 = 3;
    const MJGEOM_ELLIPSOID: i32 = 4;
    const MJGEOM_CYLINDER: i32 = 5;
    const MJGEOM_BOX: i32 = 6;

    // SAFETY: caller guarantees pos[3], mat[9], size[3], pnt[3], vec[3] valid; normal may be null
    unsafe {
        match geomtype {
            MJGEOM_PLANE => ray_plane(pos, mat, size, pnt, vec, normal),
            MJGEOM_SPHERE => ray_sphere(pos, mat, *size.add(0) * *size.add(0), pnt, vec, normal),
            MJGEOM_CAPSULE => ray_capsule(pos, mat, size, pnt, vec, normal),
            MJGEOM_ELLIPSOID => ray_ellipsoid(pos, mat, size, pnt, vec, normal),
            MJGEOM_CYLINDER => ray_cylinder(pos, mat, size, pnt, vec, normal),
            MJGEOM_BOX => ray_box(pos, mat, size, pnt, vec, std::ptr::null_mut(), normal),
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"unexpected geom type\0".as_ptr() as *const i8,
                );
                -1.0
            }
        }
    }
}

/// C: mj_rayFlex (engine/engine_ray.h:64)
/// Calls: mju_add3, mju_addScl3, mju_copy3, mju_cross, mju_dist3, mju_dot3, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, mju_rayGeom, mju_scl3, mju_zero3, ray_box, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_flex(m: *const mjModel, d: *const mjData, flex_layer: i32, flg_vert: mjtBool, flg_edge: mjtBool, flg_face: mjtBool, flg_skin: mjtBool, flexid: i32, pnt: *const f64, vec: *const f64, vertid: [i32; 1], normal: *mut f64) -> f64 {
    use crate::engine::engine_util_blas::{mju_zero3, mju_add3, mju_scl3, mju_add_scl3, mju_normalize3, mju_dot3, mju_copy3, mju_dist3};
    use crate::engine::engine_util_spatial::{mju_cross, mju_quat2mat, mju_quat_z2vec};
    const MJGEOM_CAPSULE: i32 = 3;
    const MJGEOM_SPHERE: i32 = 2;

    // SAFETY: raw pointer access to mjModel/mjData fields and flex arrays.
    // All pointers valid per caller contract; flexid is a valid index.
    unsafe {
        let dim: i32 = *(*m).flex_dim.add(flexid as usize);

        // helper to read mjtBool parameter as bool
        let flg_vert_b = *(&flg_vert as *const mjtBool as *const u8) != 0;
        let flg_edge_b = *(&flg_edge as *const mjtBool as *const u8) != 0;
        let flg_face_b = *(&flg_face as *const mjtBool as *const u8) != 0;
        let flg_skin_b = *(&flg_skin as *const mjtBool as *const u8) != 0;

        // clear normal if given
        if !normal.is_null() {
            mju_zero3(normal);
        }

        // compute bounding box
        let mut box_: [[f64; 2]; 3] = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
        let vert: *const f64 = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(flexid as usize) as usize);
        let vertnum: i32 = *(*m).flex_vertnum.add(flexid as usize);
        for i in 0..vertnum as usize {
            for j in 0..3usize {
                // update minimum along side j
                if box_[j][0] > *vert.add(3 * i + j) || i == 0 {
                    box_[j][0] = *vert.add(3 * i + j);
                }
                // update maximum along side j
                if box_[j][1] < *vert.add(3 * i + j) || i == 0 {
                    box_[j][1] = *vert.add(3 * i + j);
                }
            }
        }

        // adjust box for radius
        let radius: f64 = *(*m).flex_radius.add(flexid as usize);
        for j in 0..3usize {
            box_[j][0] -= radius;
            box_[j][1] += radius;
        }

        // construct box geom
        let mut pos: [f64; 3] = [0.0; 3];
        let mut size: [f64; 3] = [0.0; 3];
        let mut mat: [f64; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for j in 0..3usize {
            pos[j] = 0.5 * (box_[j][0] + box_[j][1]);
            size[j] = 0.5 * (box_[j][1] - box_[j][0]);
        }

        // apply bounding-box filter
        if ray_box(pos.as_ptr(), mat.as_ptr(), size.as_ptr(), pnt, vec, core::ptr::null_mut(), core::ptr::null_mut()) < 0.0 {
            return -1.0;
        }

        // construct basis vectors of normal plane
        let mut b0: [f64; 3] = [1.0, 1.0, 1.0];
        let mut b1: [f64; 3] = [0.0; 3];
        if (*vec.add(0)).abs() >= (*vec.add(1)).abs() && (*vec.add(0)).abs() >= (*vec.add(2)).abs() {
            b0[0] = 0.0;
        } else if (*vec.add(1)).abs() >= (*vec.add(2)).abs() {
            b0[1] = 0.0;
        } else {
            b0[2] = 0.0;
        }
        mju_add_scl3(b1.as_mut_ptr(), b0.as_ptr(), vec, -mju_dot3(vec, b0.as_ptr()) / mju_dot3(vec, vec));
        mju_normalize3(b1.as_mut_ptr());
        mju_cross(b0.as_mut_ptr(), b1.as_ptr(), vec);
        mju_normalize3(b0.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;
        let mut normal_local: [f64; 3] = [0.0; 3];

        // vertid as mutable local (signature passes by value; writes are local only)
        let mut vertid_local: [i32; 1] = vertid;

        // check edges if rendered, or if skin
        if flg_edge_b || (dim > 1 && flg_skin_b) {
            let edge_end: i32 = *(*m).flex_edgeadr.add(flexid as usize) + *(*m).flex_edgenum.add(flexid as usize);
            let mut e = *(*m).flex_edgeadr.add(flexid as usize);
            while e < edge_end {
                // get vertices for this edge
                let v1: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *(*m).flex_edge.add(2 * e as usize)) as usize);
                let v2: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *(*m).flex_edge.add(2 * e as usize + 1)) as usize);

                // construct capsule geom
                mju_add3(pos.as_mut_ptr(), v1, v2);
                mju_scl3(pos.as_mut_ptr(), pos.as_ptr(), 0.5);
                let mut dif: [f64; 3] = [
                    *v2.add(0) - *v1.add(0),
                    *v2.add(1) - *v1.add(1),
                    *v2.add(2) - *v1.add(2),
                ];
                size[0] = radius;
                size[1] = 0.5 * mju_normalize3(dif.as_mut_ptr());
                let mut quat: [f64; 4] = [0.0; 4];
                mju_quat_z2vec(quat.as_mut_ptr(), dif.as_ptr());
                mju_quat2mat(mat.as_mut_ptr(), quat.as_ptr());

                // intersect ray with capsule
                let sol: f64 = mju_ray_geom(
                    pos.as_ptr(), mat.as_ptr(), size.as_ptr(), pnt, vec, MJGEOM_CAPSULE,
                    if !normal.is_null() { normal_local.as_mut_ptr() } else { core::ptr::null_mut() },
                );

                // update
                if sol >= 0.0 && (x < 0.0 || sol < x) {
                    x = sol;
                    if !normal.is_null() {
                        mju_copy3(normal, normal_local.as_ptr());
                    }

                    // construct intersection point
                    let mut intersect: [f64; 3] = [0.0; 3];
                    mju_add_scl3(intersect.as_mut_ptr(), pnt, vec, sol);

                    // find nearest vertex
                    if mju_dist3(v1, intersect.as_ptr()) < mju_dist3(v2, intersect.as_ptr()) {
                        vertid_local[0] = *(*m).flex_edge.add(2 * e as usize);
                    } else {
                        vertid_local[0] = *(*m).flex_edge.add(2 * e as usize + 1);
                    }
                }

                e += 1;
            }
        }

        // check vertices if rendered (and edges not checked)
        else if flg_vert_b && !(dim > 1 && flg_skin_b) {
            for v in 0..*(*m).flex_vertnum.add(flexid as usize) {
                // get vertex
                let vpos: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + v) as usize);

                // construct sphere geom
                size[0] = radius;

                // intersect ray with sphere
                let sol: f64 = mju_ray_geom(
                    vpos, core::ptr::null(), size.as_ptr(), pnt, vec, MJGEOM_SPHERE,
                    if !normal.is_null() { normal_local.as_mut_ptr() } else { core::ptr::null_mut() },
                );

                // update
                if sol >= 0.0 && (x < 0.0 || sol < x) {
                    x = sol;
                    if !normal.is_null() {
                        mju_copy3(normal, normal_local.as_ptr());
                    }
                    vertid_local[0] = v;
                }
            }
        }

        // check faces if rendered
        if dim > 1 && (flg_face_b || flg_skin_b) {
            for e in 0..*(*m).flex_elemnum.add(flexid as usize) {
                // skip if 3D element is not visible
                let elayer: i32 = *(*m).flex_elemlayer.add((*(*m).flex_elemadr.add(flexid as usize) + e) as usize);
                if dim == 3 && ((flg_skin_b && elayer > 0) || (!flg_skin_b && elayer != flex_layer)) {
                    continue;
                }

                // get element data
                let edata: *const i32 = (*m).flex_elem.add((*(*m).flex_elemdataadr.add(flexid as usize) + e * (dim + 1)) as usize);
                let v1: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *edata.add(0)) as usize);
                let v2: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *edata.add(1)) as usize);
                let v3: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *edata.add(2)) as usize);
                let v4: *const f64 = if dim == 2 {
                    core::ptr::null()
                } else {
                    (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *edata.add(3)) as usize)
                };
                let vptr: [[*const f64; 3]; 4] = [
                    [v1, v2, v3],
                    [v1, v2, v4],
                    [v1, v3, v4],
                    [v2, v3, v4],
                ];
                let vid: [[i32; 3]; 4] = [
                    [0, 1, 2],
                    [0, 1, 3],
                    [0, 2, 3],
                    [1, 2, 3],
                ];

                // process triangles of this element
                let ntri = if dim == 2 { 1 } else { 4 };
                for i in 0..ntri {
                    // copy vertices into triangle representation
                    let mut v_tri: [[f64; 3]; 3] = [[0.0; 3]; 3];
                    for j in 0..3usize {
                        mju_copy3(v_tri[j].as_mut_ptr(), vptr[i][j]);
                    }

                    // intersect ray with triangle
                    let sol: f64 = ray_triangle(
                        v_tri.as_ptr(), pnt, vec, b0.as_ptr(), b1.as_ptr(),
                        if !normal.is_null() { normal_local.as_mut_ptr() } else { core::ptr::null_mut() },
                    );

                    // update
                    if sol >= 0.0 && (x < 0.0 || sol < x) {
                        x = sol;
                        if !normal.is_null() {
                            mju_copy3(normal, normal_local.as_ptr());
                        }

                        // construct intersection point
                        let mut intersect: [f64; 3] = [0.0; 3];
                        mju_add_scl3(intersect.as_mut_ptr(), pnt, vec, sol);

                        // find nearest vertex
                        let dist: [f64; 3] = [
                            mju_dist3(v_tri[0].as_ptr(), intersect.as_ptr()),
                            mju_dist3(v_tri[1].as_ptr(), intersect.as_ptr()),
                            mju_dist3(v_tri[2].as_ptr(), intersect.as_ptr()),
                        ];
                        if dist[0] <= dist[1] && dist[0] <= dist[2] {
                            vertid_local[0] = *edata.add(vid[i][0] as usize);
                        } else if dist[1] <= dist[2] {
                            vertid_local[0] = *edata.add(vid[i][1] as usize);
                        } else {
                            vertid_local[0] = *edata.add(vid[i][2] as usize);
                        }
                    }
                }
            }
        }

        x
    }
}

/// C: mju_raySkin (engine/engine_ray.h:70)
/// Calls: mju_addScl3, mju_cross, mju_dist3, mju_dot3, mju_normalize3, ray_box, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_skin(nface: i32, nvert: i32, face: *const i32, vert: *const f32, pnt: *const f64, vec: *const f64, vertid: [i32; 1]) -> f64 {
    use crate::engine::engine_util_blas::{mju_add_scl3, mju_normalize3, mju_dot3, mju_dist3};
    use crate::engine::engine_util_spatial::mju_cross;

    // NOTE: In C ABI, vertid: [i32; 1] is a pointer. vertid.as_ptr() gives that pointer.
    let vertid_ptr = vertid.as_ptr() as *mut i32;

    // SAFETY: all pointers valid per caller contract; face[3*nface], vert[3*nvert], pnt[3], vec[3]
    unsafe {
        // compute bounding box
        let mut bbox: [[f64; 2]; 3] = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
        for i in 0..nvert as usize {
            for j in 0..3usize {
                // update minimum along side j
                if bbox[j][0] > *vert.add(3*i+j) as f64 || i == 0 {
                    bbox[j][0] = *vert.add(3*i+j) as f64;
                }
                // update maximum along side j
                if bbox[j][1] < *vert.add(3*i+j) as f64 || i == 0 {
                    bbox[j][1] = *vert.add(3*i+j) as f64;
                }
            }
        }

        // construct box geom
        let mut pos: [f64; 3] = [0.0; 3];
        let mut size: [f64; 3] = [0.0; 3];
        let mat: [f64; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for j in 0..3usize {
            pos[j] = 0.5 * (bbox[j][0] + bbox[j][1]);
            size[j] = 0.5 * (bbox[j][1] - bbox[j][0]);
        }

        // apply bounding-box filter
        if ray_box(pos.as_ptr(), mat.as_ptr(), size.as_ptr(), pnt, vec, core::ptr::null_mut(), core::ptr::null_mut()) < 0.0 {
            return -1.0;
        }

        // construct basis vectors of normal plane
        let mut b0: [f64; 3] = [1.0, 1.0, 1.0];
        let mut b1: [f64; 3] = [0.0; 3];
        let vec0_abs = (*vec.add(0)).abs();
        let vec1_abs = (*vec.add(1)).abs();
        let vec2_abs = (*vec.add(2)).abs();
        if vec0_abs >= vec1_abs && vec0_abs >= vec2_abs {
            b0[0] = 0.0;
        } else if vec1_abs >= vec2_abs {
            b0[1] = 0.0;
        } else {
            b0[2] = 0.0;
        }
        mju_add_scl3(b1.as_mut_ptr(), b0.as_ptr(), vec, -mju_dot3(vec, b0.as_ptr()) / mju_dot3(vec, vec));
        mju_normalize3(b1.as_mut_ptr());
        mju_cross(b0.as_mut_ptr(), b1.as_ptr(), vec);
        mju_normalize3(b0.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;

        // process all faces
        for i in 0..nface as usize {
            // get float vertex pointers
            let vf0 = vert.add(3 * (*face.add(3*i) as usize));
            let vf1 = vert.add(3 * (*face.add(3*i+1) as usize));
            let vf2 = vert.add(3 * (*face.add(3*i+2) as usize));

            // convert to f64
            let mut v: [[f64; 3]; 3] = [[0.0; 3]; 3];
            for k in 0..3usize {
                v[0][k] = *vf0.add(k) as f64;
                v[1][k] = *vf1.add(k) as f64;
                v[2][k] = *vf2.add(k) as f64;
            }

            // solve
            let sol = ray_triangle(v.as_ptr() as *const [f64; 3], pnt, vec, b0.as_ptr(), b1.as_ptr(), core::ptr::null_mut());

            // update
            if sol >= 0.0 && (x < 0.0 || sol < x) {
                x = sol;

                // construct intersection point
                let mut intersect: [f64; 3] = [0.0; 3];
                mju_add_scl3(intersect.as_mut_ptr(), pnt, vec, sol);

                // find nearest vertex
                let mut dist = mju_dist3(intersect.as_ptr(), v[0].as_ptr());
                if !vertid_ptr.is_null() {
                    *vertid_ptr = *face.add(3*i);
                }
                for j in 1..3usize {
                    let newdist = mju_dist3(intersect.as_ptr(), v[j].as_ptr());
                    if newdist < dist {
                        dist = newdist;
                        if !vertid_ptr.is_null() {
                            *vertid_ptr = *face.add(3*i+j);
                        }
                    }
                }
            }
        }

        x
    }
}

