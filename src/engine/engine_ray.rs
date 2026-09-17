//! Port of: engine/engine_ray.c
//! IR hash: fd5288ad7242f001
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
    // SAFETY: all pointers are valid arrays of the documented sizes (caller contract)
    unsafe {
        let dif0 = *pnt.add(0) - *pos.add(0);
        let dif1 = *pnt.add(1) - *pos.add(1);
        let dif2 = *pnt.add(2) - *pos.add(2);

        // lpnt = mat' * dif
        *lpnt.add(0) = *mat.add(0)*dif0 + *mat.add(3)*dif1 + *mat.add(6)*dif2;
        *lpnt.add(1) = *mat.add(1)*dif0 + *mat.add(4)*dif1 + *mat.add(7)*dif2;
        *lpnt.add(2) = *mat.add(2)*dif0 + *mat.add(5)*dif1 + *mat.add(8)*dif2;

        // lvec = mat' * vec
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
    // SAFETY: vec points to at least 3 f64 (caller contract)
    unsafe {
        f64::atan2(*vec.add(1), *vec.add(0))
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
    // SAFETY: vec points to at least 3 f64 (caller contract)
    unsafe {
        f64::atan2(f64::sqrt(*vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1)), *vec.add(2))
    }
}

/// C: ray_eliminate (engine/engine_ray.c:68)
#[allow(unused_variables, non_snake_case)]
pub fn ray_eliminate(m: *const mjModel, d: *const mjData, geomid: i32, geomgroup: *const u8, flg_static: bool, bodyexclude: i32) -> i32 {
    const MJ_NGROUP: i32 = 6;
    // SAFETY: caller guarantees m, d are valid, geomid is a valid index into model arrays
    unsafe {
        let gid = geomid as usize;

        // body exclusion
        if *(*m).geom_bodyid.add(gid) == bodyexclude {
            return 1;
        }

        // invisible geom exclusion
        if *(*m).geom_matid.add(gid) < 0 && *(*m).geom_rgba.add(4 * gid + 3) == 0.0 {
            return 1;
        }

        // invisible material exclusion
        if *(*m).geom_matid.add(gid) >= 0
            && *(*m).mat_rgba.add(4 * (*(*m).geom_matid.add(gid)) as usize + 3) == 0.0
        {
            return 1;
        }

        // static exclusion
        if !flg_static && *(*m).body_weldid.add(*(*m).geom_bodyid.add(gid) as usize) == 0 {
            return 1;
        }

        // no geomgroup inclusion
        if geomgroup.is_null() {
            return 0;
        }

        // group inclusion/exclusion
        let group = *(*m).geom_group.add(gid);
        let groupid = if 0 > (if (MJ_NGROUP - 1) < group { MJ_NGROUP - 1 } else { group }) {
            0
        } else if (MJ_NGROUP - 1) < group {
            MJ_NGROUP - 1
        } else {
            group
        };

        (*geomgroup.add(groupid as usize) == 0) as i32
    }
}

/// C: ray_quad (engine/engine_ray.c:103)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_quad(a: f64, b: f64, c: f64, x: *mut f64) -> f64 {
    // SAFETY: x points to at least 2 f64 (caller contract)
    unsafe {
        let mut det = b*b - a*c;

        if det < 0.0 || a < 1E-15_f64 {
            *x.add(0) = -1.0;
            *x.add(1) = -1.0;
            return -1.0;
        }

        det = f64::sqrt(det);
        *x.add(0) = (-b - det) / a;
        *x.add(1) = (-b + det) / a;

        if *x.add(0) >= 0.0 {
            return *x.add(0);
        } else if *x.add(1) >= 0.0 {
            return *x.add(1);
        }

        -1.0
    }
}

/// C: ray_triangle (engine/engine_ray.c:133)
/// Calls: cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_triangle(v: *mut [f64; 3], lpnt: *const f64, lvec: *const f64, b0: *const f64, b1: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: v points to 3 arrays of [f64;3], lpnt/lvec/b0/b1 point to 3 f64; normal may be null
    unsafe {
        // clear normal if given
        if !normal.is_null() {
            crate::engine::engine_util_blas::mju_zero3(normal);
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
            planar[i][0] = crate::engine::engine_util_blas::mju_dot3(b0, dif[i].as_ptr());
            planar[i][1] = crate::engine::engine_util_blas::mju_dot3(b1, dif[i].as_ptr());
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
        let a_mat: [f64; 4] = [
            planar[0][0] - planar[2][0], planar[1][0] - planar[2][0],
            planar[0][1] - planar[2][1], planar[1][1] - planar[2][1],
        ];
        let bv: [f64; 2] = [-planar[2][0], -planar[2][1]];
        let det = a_mat[0]*a_mat[3] - a_mat[1]*a_mat[2];
        if f64::abs(det) < 1E-15_f64 {
            return -1.0;
        }
        let t0 = ( a_mat[3]*bv[0] - a_mat[1]*bv[1]) / det;
        let t1 = (-a_mat[2]*bv[0] + a_mat[0]*bv[1]) / det;

        // check if outside
        if t0 < 0.0 || t1 < 0.0 || t0 + t1 > 1.0 {
            return -1.0;
        }

        // intersect ray with plane of triangle
        let mut dif2: [[f64; 3]; 3] = [[0.0; 3]; 3];
        crate::engine::engine_util_blas::mju_sub3(dif2[0].as_mut_ptr(), (*v.add(0)).as_ptr(), (*v.add(2)).as_ptr()); // v0-v2
        crate::engine::engine_util_blas::mju_sub3(dif2[1].as_mut_ptr(), (*v.add(1)).as_ptr(), (*v.add(2)).as_ptr()); // v1-v2
        crate::engine::engine_util_blas::mju_sub3(dif2[2].as_mut_ptr(), lpnt, (*v.add(2)).as_ptr()); // lp-v2

        let mut nrm: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_cross(nrm.as_mut_ptr(), dif2[0].as_ptr(), dif2[1].as_ptr());

        let denom = crate::engine::engine_util_blas::mju_dot3(lvec, nrm.as_ptr());
        if f64::abs(denom) < 1E-15_f64 {
            return -1.0;
        }

        // compute distance
        let x = -crate::engine::engine_util_blas::mju_dot3(dif2[2].as_ptr(), nrm.as_ptr()) / denom;

        // compute normal if given
        if !normal.is_null() {
            crate::engine::engine_util_blas::mju_normalize3(nrm.as_mut_ptr());
            crate::engine::engine_util_blas::mju_copy3(normal, nrm.as_ptr());
        }

        x
    }
}

/// C: ray_plane (engine/engine_ray.c:204)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_plane(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: all pointers are valid arrays of documented sizes (caller contract), normal may be null
    unsafe {
        if !normal.is_null() {
            crate::engine::engine_util_blas::mju_zero3(normal);
        }

        let mut lpnt: [f64; 3] = [0.0; 3];
        let mut lvec: [f64; 3] = [0.0; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // z-vec not pointing towards front face: reject
        if lvec[2] > -1E-15_f64 {
            return -1.0;
        }

        let x = -lpnt[2] / lvec[2];
        if x < 0.0 {
            return -1.0;
        }

        let p0 = lpnt[0] + x * lvec[0];
        let p1 = lpnt[1] + x * lvec[1];

        if (*size.add(0) <= 0.0 || f64::abs(p0) <= *size.add(0)) &&
           (*size.add(1) <= 0.0 || f64::abs(p1) <= *size.add(1)) {
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_quad, cxx:_mju_addScl3, cxx:_mju_normalize3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_sphere(pos: *const f64, mat: *const f64, dist_sqr: f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: pos, pnt, vec point to 3 f64; normal may be null (caller contract)
    unsafe {
        let dif0 = *pnt.add(0) - *pos.add(0);
        let dif1 = *pnt.add(1) - *pos.add(1);
        let dif2 = *pnt.add(2) - *pos.add(2);

        let a = *vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1) + *vec.add(2) * *vec.add(2);
        let b = *vec.add(0) * dif0 + *vec.add(1) * dif1 + *vec.add(2) * dif2;
        let c = dif0*dif0 + dif1*dif1 + dif2*dif2 - dist_sqr;

        let mut xx: [f64; 2] = [0.0; 2];
        let x = ray_quad(a, b, c, xx.as_mut_ptr());

        if !normal.is_null() {
            if x < 0.0 {
                crate::engine::engine_util_blas::mju_zero3(normal);
            } else {
                let mut s: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_addScl3(s.as_mut_ptr(), pnt, vec, x);
                crate::engine::engine_util_blas::mju_sub3(normal, s.as_ptr(), pos);
                crate::engine::engine_util_blas::mju_normalize3(normal);
            }
        }

        x
    }
}

/// C: ray_capsule (engine/engine_ray.c:272)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_quad, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_capsule(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: all pointers valid arrays of documented sizes; normal may be null (caller contract)
    unsafe {
        // bounding sphere test
        let ssz = *size.add(0) + *size.add(1);
        if ray_sphere(pos, std::ptr::null(), ssz * ssz, pnt, vec, std::ptr::null_mut()) < 0.0 {
            if !normal.is_null() {
                crate::engine::engine_util_blas::mju_zero3(normal);
            }
            return -1.0;
        }

        // map to local frame
        let mut lpnt: [f64; 3] = [0.0; 3];
        let mut lvec: [f64; 3] = [0.0; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;
        let mut xx: [f64; 2] = [0.0; 2];
        let mut r#type: i32 = 0; // -1: bottom, 0: cylinder, 1: top

        // cylinder round side
        let mut a = lvec[0]*lvec[0] + lvec[1]*lvec[1];
        let mut b = lvec[0]*lpnt[0] + lvec[1]*lpnt[1];
        let mut c = lpnt[0]*lpnt[0] + lpnt[1]*lpnt[1] - *size.add(0) * *size.add(0);

        let sol = ray_quad(a, b, c, xx.as_mut_ptr());

        // make sure round solution is between flat sides
        if sol >= 0.0 && f64::abs(lpnt[2] + sol*lvec[2]) <= *size.add(1) {
            if x < 0.0 || sol < x {
                x = sol;
                r#type = 0;
            }
        }

        // top cap
        let mut ldif: [f64; 3] = [lpnt[0], lpnt[1], lpnt[2] - *size.add(1)];
        a = lvec[0]*lvec[0] + lvec[1]*lvec[1] + lvec[2]*lvec[2];
        b = lvec[0]*ldif[0] + lvec[1]*ldif[1] + lvec[2]*ldif[2];
        c = ldif[0]*ldif[0] + ldif[1]*ldif[1] + ldif[2]*ldif[2] - *size.add(0) * *size.add(0);
        ray_quad(a, b, c, xx.as_mut_ptr());

        // accept only top half of sphere
        for i in 0..2 {
            if xx[i] >= 0.0 && lpnt[2] + xx[i]*lvec[2] >= *size.add(1) {
                if x < 0.0 || xx[i] < x {
                    x = xx[i];
                    r#type = 1;
                }
            }
        }

        // bottom cap
        ldif[2] = lpnt[2] + *size.add(1);
        b = lvec[0]*ldif[0] + lvec[1]*ldif[1] + lvec[2]*ldif[2];
        c = ldif[0]*ldif[0] + ldif[1]*ldif[1] + ldif[2]*ldif[2] - *size.add(0) * *size.add(0);
        ray_quad(a, b, c, xx.as_mut_ptr());

        // accept only bottom half of sphere
        for i in 0..2 {
            if xx[i] >= 0.0 && lpnt[2] + xx[i]*lvec[2] <= -*size.add(1) {
                if x < 0.0 || xx[i] < x {
                    x = xx[i];
                    r#type = -1;
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
                *normal.add(2) = if r#type == 0 { 0.0 } else { lpnt[2] + lvec[2] * x - *size.add(1) * (r#type as f64) };

                // normalize, rotate into global frame
                crate::engine::engine_util_blas::mju_normalize3(normal);
                crate::engine::engine_util_blas::mju_mulMatVec3(normal, mat, normal);
            }
        }

        x
    }
}

/// C: ray_ellipsoid (engine/engine_ray.c:358)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_quad, cxx:_mju_addScl3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_ellipsoid(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: all pointers valid arrays of documented sizes; normal may be null (caller contract)
    unsafe {
        // map to local frame
        let mut lpnt: [f64; 3] = [0.0; 3];
        let mut lvec: [f64; 3] = [0.0; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // invert size^2
        let s: [f64; 3] = [
            1.0 / (*size.add(0) * *size.add(0)),
            1.0 / (*size.add(1) * *size.add(1)),
            1.0 / (*size.add(2) * *size.add(2)),
        ];

        // (x*lvec+lpnt)' * diag(1./size^2) * (x*lvec+lpnt) = 1
        let a = s[0]*lvec[0]*lvec[0] + s[1]*lvec[1]*lvec[1] + s[2]*lvec[2]*lvec[2];
        let b = s[0]*lvec[0]*lpnt[0] + s[1]*lvec[1]*lpnt[1] + s[2]*lvec[2]*lpnt[2];
        let c = s[0]*lpnt[0]*lpnt[0] + s[1]*lpnt[1]*lpnt[1] + s[2]*lpnt[2]*lpnt[2] - 1.0;

        let mut xx: [f64; 2] = [0.0; 2];
        let x = ray_quad(a, b, c, xx.as_mut_ptr());

        // compute normal if required
        if !normal.is_null() {
            if x < 0.0 {
                crate::engine::engine_util_blas::mju_zero3(normal);
            } else {
                // surface intersection (local frame)
                let mut l: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_addScl3(l.as_mut_ptr(), lpnt.as_ptr(), lvec.as_ptr(), x);

                // gradient of ellipsoid function
                *normal.add(0) = s[0] * l[0];
                *normal.add(1) = s[1] * l[1];
                *normal.add(2) = s[2] * l[2];

                // normalize, rotate into global frame
                crate::engine::engine_util_blas::mju_normalize3(normal);
                crate::engine::engine_util_blas::mju_mulMatVec3(normal, mat, normal);
            }
        }

        x
    }
}

/// C: ray_cylinder (engine/engine_ray.c:401)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_quad, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_cylinder(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: all pointers valid arrays of documented sizes; normal may be null (caller contract)
    unsafe {
        // bounding sphere test
        let ssz = *size.add(0) * *size.add(0) + *size.add(1) * *size.add(1);
        if ray_sphere(pos, std::ptr::null(), ssz, pnt, vec, std::ptr::null_mut()) < 0.0 {
            if !normal.is_null() {
                crate::engine::engine_util_blas::mju_zero3(normal);
            }
            return -1.0;
        }

        // map to local frame
        let mut lpnt: [f64; 3] = [0.0; 3];
        let mut lvec: [f64; 3] = [0.0; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;
        let mut sol: f64;
        let mut r#type: i32 = 0; // -1: bottom, 0: round, 1: top

        // flat sides
        if f64::abs(lvec[2]) > 1E-15_f64 {
            let mut side: i32 = -1;
            while side <= 1 {
                // solution of: lpnt[2] + x*lvec[2] = side*size[1]
                sol = ((side as f64) * *size.add(1) - lpnt[2]) / lvec[2];

                // process if non-negative
                if sol >= 0.0 {
                    let p0 = lpnt[0] + sol * lvec[0];
                    let p1 = lpnt[1] + sol * lvec[1];

                    // accept within radius
                    if p0*p0 + p1*p1 <= *size.add(0) * *size.add(0) {
                        if x < 0.0 || sol < x {
                            x = sol;
                            r#type = side;
                        }
                    }
                }
                side += 2;
            }
        }

        // round side
        let a = lvec[0]*lvec[0] + lvec[1]*lvec[1];
        let b = lvec[0]*lpnt[0] + lvec[1]*lpnt[1];
        let c = lpnt[0]*lpnt[0] + lpnt[1]*lpnt[1] - *size.add(0) * *size.add(0);

        let mut xx: [f64; 2] = [0.0; 2];
        sol = ray_quad(a, b, c, xx.as_mut_ptr());

        // make sure round solution is between flat sides
        if sol >= 0.0 && f64::abs(lpnt[2] + sol*lvec[2]) <= *size.add(1) {
            if x < 0.0 || sol < x {
                x = sol;
                r#type = 0;
            }
        }

        // compute normal if required
        if !normal.is_null() {
            if x < 0.0 {
                crate::engine::engine_util_blas::mju_zero3(normal);
            } else {
                if r#type == 0 {
                    // normal at surface intersection (local frame)
                    *normal.add(0) = lpnt[0] + lvec[0] * x;
                    *normal.add(1) = lpnt[1] + lvec[1] * x;
                    *normal.add(2) = 0.0;
                    crate::engine::engine_util_blas::mju_normalize3(normal);
                } else {
                    // flat sides
                    *normal.add(0) = 0.0;
                    *normal.add(1) = 0.0;
                    *normal.add(2) = r#type as f64;
                }

                // rotate into global frame
                crate::engine::engine_util_blas::mju_mulMatVec3(normal, mat, normal);
            }
        }

        x
    }
}

/// C: ray_box (engine/engine_ray.c:490)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mju_mulMatVec3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_box(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, all: *mut f64, normal: *mut f64) -> f64 {
    // SAFETY: all pointers valid arrays of documented sizes; all and normal may be null (caller contract)
    unsafe {
        // clear outputs
        if !all.is_null() {
            *all.add(0) = -1.0; *all.add(1) = -1.0; *all.add(2) = -1.0;
            *all.add(3) = -1.0; *all.add(4) = -1.0; *all.add(5) = -1.0;
        }
        if !normal.is_null() {
            crate::engine::engine_util_blas::mju_zero3(normal);
        }

        // bounding sphere test
        let ssz = *size.add(0) * *size.add(0) + *size.add(1) * *size.add(1) + *size.add(2) * *size.add(2);
        if ray_sphere(pos, std::ptr::null(), ssz, pnt, vec, std::ptr::null_mut()) < 0.0 {
            return -1.0;
        }

        // faces
        let iface: [[usize; 2]; 3] = [[1, 2], [0, 2], [0, 1]];

        // map to local frame
        let mut lpnt: [f64; 3] = [0.0; 3];
        let mut lvec: [f64; 3] = [0.0; 3];
        ray_map(pos, mat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;
        let mut sol: f64;
        let mut face_side: i32 = 0;
        let mut face_axis: i32 = -1;

        // loop over axes with non-zero vec
        for i in 0..3 {
            if f64::abs(lvec[i]) > 1E-15_f64 {
                let mut side: i32 = -1;
                while side <= 1 {
                    // solution of: lpnt[i] + x*lvec[i] = side*size[i]
                    sol = ((side as f64) * *size.add(i) - lpnt[i]) / lvec[i];

                    // process if non-negative
                    if sol >= 0.0 {
                        let p0 = lpnt[iface[i][0]] + sol * lvec[iface[i][0]];
                        let p1 = lpnt[iface[i][1]] + sol * lvec[iface[i][1]];

                        // accept within rectangle
                        if f64::abs(p0) <= *size.add(iface[i][0]) &&
                           f64::abs(p1) <= *size.add(iface[i][1]) {
                            if x < 0.0 || sol < x {
                                x = sol;
                                face_axis = i as i32;
                                face_side = side;
                            }

                            // save in all
                            if !all.is_null() {
                                *all.add(2*i + ((side + 1) / 2) as usize) = sol;
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
            crate::engine::engine_util_blas::mju_mulMatVec3(normal, mat, n_local.as_ptr());
        }

        x
    }
}

/// C: mj_rayHfield (engine/engine_ray.c:563)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx-internal:engine_ray.c.o:_ray_box, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_round, cxx:_mju_zero3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_hfield(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx:_mj_rayHfield
}

/// C: mju_raySlab (engine/engine_ray.c:743)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_slab(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64, vec: *const f64) -> i32 {
    todo!() // cxx:_mju_raySlab
}

/// C: mju_rayTree (engine/engine_ray.c:771)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_raySlab, cxx:_mju_zero3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_tree(m: *const mjModel, d: *const mjData, id: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx:_mju_rayTree
}

/// C: mj_raySdf (engine/engine_ray.c:885)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx-internal:engine_ray.c.o:_ray_box, cxx:_mjc_distance, cxx:_mjc_getSDF, cxx:_mjc_gradient, cxx:_mju_addScl3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_sdf(m: *const mjModel, d: *const mjData, g: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_mj_raySdf
}

/// C: mj_rayMesh (engine/engine_ray.c:953)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx:_mju_message, cxx:_mju_rayTree, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_mesh(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx:_mj_rayMesh
}

/// C: mju_rayGeom (engine/engine_ray.c:974)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_capsule, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_cylinder, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_ellipsoid, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_plane, cxx-internal:engine_ray.c.o:_ray_box, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_geom(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, geomtype: i32, normal: *mut f64) -> f64 {
    todo!() // cxx:_mju_rayGeom
}

/// C: mj_rayFlex (engine/engine_ray.c:1005)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx:_mju_add3, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dist3, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec, cxx:_mju_rayGeom, cxx:_mju_scl3, cxx:_mju_zero3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_flex(m: *const mjModel, d: *const mjData, flex_layer: i32, flg_vert: bool, flg_edge: bool, flg_face: bool, flg_skin: bool, flexid: i32, pnt: *const f64, vec: *const f64, vertid: *mut i32, normal: *mut f64) -> f64 {
    todo!() // cxx:_mj_rayFlex
}

/// C: mju_raySkin (engine/engine_ray.c:1192)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx:_mju_addScl3, cxx:_mju_cross, cxx:_mju_dist3, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_skin(nface: i32, nvert: i32, face: *const i32, vert: *const f32, pnt: *const f64, vec: *const f64, vertid: *mut i32) -> f64 {
    todo!() // cxx:_mju_raySkin
}

/// C: point_in_box (engine/engine_ray.c:1283)
/// Calls: cxx:_mju_mulMatTVec3, cxx:_mju_sub3, cxx:_mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_in_box(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64) -> i32 {
    // SAFETY: all pointers valid arrays of documented sizes (caller contract)
    unsafe {
        let mut point: [f64; 3] = [0.0; 3];

        // compute point in local coordinates of the box
        crate::engine::engine_util_blas::mju_sub3(point.as_mut_ptr(), pnt, xpos);
        crate::engine::engine_util_blas::mju_mulMatTVec3(point.as_mut_ptr(), xmat, point.as_ptr());
        crate::engine::engine_util_blas::mju_subFrom3(point.as_mut_ptr(), aabb);

        // check intersections
        for j in 0..3 {
            if f64::abs(point[j]) > *aabb.add(3 + j) {
                return 0;
            }
        }

        1
    }
}

/// C: mj_ray (engine/engine_ray.c:1310)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_eliminate, cxx-internal:engine_ray.c.o:_mj_raySdf, cxx:_mj_rayHfield, cxx:_mj_rayMesh, cxx:_mju_copy3, cxx:_mju_message, cxx:_mju_norm3, cxx:_mju_rayGeom, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray(m: *const mjModel, d: *const mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: bool, bodyexclude: i32, geomid: *mut i32, normal: *mut f64) -> f64 {
    todo!() // cxx:_mj_ray
}

/// C: mju_multiRayPrepare (engine/engine_ray.c:1357)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_latitude, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_longitude, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_point_in_box, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_eliminate, cxx:_mju_addTo3, cxx:_mju_copy, cxx:_mju_dist3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatVec3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_multi_ray_prepare(m: *const mjModel, d: *const mjData, pnt: *const f64, ray_xmat: *const f64, geomgroup: *const u8, flg_static: bool, bodyexclude: i32, cutoff: f64, geom_ba: *mut f64, geom_eliminate: *mut i32) {
    todo!() // cxx:_mju_multiRayPrepare
}

/// C: mju_singleRay (engine/engine_ray.c:1457)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_latitude, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_longitude, cxx-internal:engine_ray.c.o:_mj_raySdf, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mj_rayHfield, cxx:_mj_rayMesh, cxx:_mju_add3, cxx:_mju_copy3, cxx:_mju_rayGeom, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_single_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, ray_eliminate: *mut i32, geom_ba: *mut f64, geomid: *mut i32, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_mju_singleRay
}

/// C: mj_multiRay (engine/engine_ray.c:1546)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_mju_singleRay, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_dot3, cxx:_mju_multiRayPrepare
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_multi_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: bool, bodyexclude: i32, geomid: *mut i32, dist: *mut f64, normal: *mut f64, nray: i32, cutoff: f64) {
    todo!() // cxx:_mj_multiRay
}

