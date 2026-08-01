//! Port of: engine/engine_ray.c
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

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

/// C: mju_raySlab (engine/engine_ray.c:743)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_raySlab(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64, vec: *const f64) -> i32 {
    // SAFETY: all pointers valid arrays of documented sizes (caller contract)
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
        let mut src: [f64; 3] = [0.0; 3];
        let mut dir: [f64; 3] = [0.0; 3];
        ray_map(xpos, xmat, pnt, vec, src.as_mut_ptr(), dir.as_mut_ptr());

        // check intersections
        let invdir: [f64; 3] = [1.0 / dir[0], 1.0 / dir[1], 1.0 / dir[2]];
        for d in 0..3 {
            let t1 = (min[d] - src[d]) * invdir[d];
            let t2 = (max[d] - src[d]) * invdir[d];
            let minval = if t1 < t2 { t1 } else { t2 };
            let maxval = if t1 < t2 { t2 } else { t1 };
            tmin = if tmin > minval { tmin } else { minval };
            tmax = if tmax < maxval { tmax } else { maxval };
        }

        (tmin < tmax) as i32
    }
}

/// C: mju_rayTree (engine/engine_ray.c:771)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_raySlab, cxx:_mju_zero3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rayTree(m: *const mjModel, d: *const mjData, id: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    const MJ_MAXTREEDEPTH: usize = 256;

    // SAFETY: m, d, pnt, vec are valid. normal may be null. id indexes valid geom.
    unsafe {
        // clear normal if given
        if !normal.is_null() {
            crate::engine::engine_util_blas::mju_zero3(normal);
        }

        let meshid = *(*m).geom_dataid.add(id as usize);
        let bvhadr = *(*m).mesh_bvhadr.add(meshid as usize);
        let faceid = (*m).bvh_nodeid.add(bvhadr as usize);
        let bvh = (*m).bvh_aabb.add(6 * bvhadr as usize);
        let child = (*m).bvh_child.add(2 * bvhadr as usize);

        if meshid == -1 {
            crate::engine::engine_util_errmem::mju_error(
                b"mesh id of geom %d is -1\0".as_ptr() as *const i8,
            );
        }

        // initialize stack
        let mut stack: [i32; MJ_MAXTREEDEPTH] = [0; MJ_MAXTREEDEPTH];
        let mut nstack: i32 = 0;
        stack[0] = 0;
        nstack = 1;

        // map to local frame
        let mut lpnt: [f64; 3] = [0.0; 3];
        let mut lvec: [f64; 3] = [0.0; 3];
        ray_map(
            (*d).geom_xpos.add(3 * id as usize),
            (*d).geom_xmat.add(9 * id as usize),
            pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr(),
        );

        // construct basis vectors of normal plane
        let mut b0: [f64; 3] = [1.0, 1.0, 1.0];
        let mut b1: [f64; 3] = [0.0; 3];
        if lvec[0].abs() >= lvec[1].abs() && lvec[0].abs() >= lvec[2].abs() {
            b0[0] = 0.0;
        } else if lvec[1].abs() >= lvec[2].abs() {
            b0[1] = 0.0;
        } else {
            b0[2] = 0.0;
        }
        let dot_lvec_b0 = crate::engine::engine_util_blas::mju_dot3(lvec.as_ptr(), b0.as_ptr());
        let dot_lvec_lvec = crate::engine::engine_util_blas::mju_dot3(lvec.as_ptr(), lvec.as_ptr());
        crate::engine::engine_util_blas::mju_addScl3(
            b1.as_mut_ptr(), b0.as_ptr(), lvec.as_ptr(), -dot_lvec_b0 / dot_lvec_lvec,
        );
        crate::engine::engine_util_blas::mju_normalize3(b1.as_mut_ptr());
        crate::engine::engine_util_spatial::mju_cross(b0.as_mut_ptr(), b1.as_ptr(), lvec.as_ptr());
        crate::engine::engine_util_blas::mju_normalize3(b0.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;
        let mut normal_local: [f64; 3] = [0.0; 3];

        while nstack > 0 {
            // pop from stack
            nstack -= 1;
            let node = stack[nstack as usize];

            // intersection test
            let intersect = mju_raySlab(
                bvh.add(6 * node as usize),
                (*d).geom_xpos.add(3 * id as usize),
                (*d).geom_xmat.add(9 * id as usize),
                pnt, vec,
            );

            // if no intersection, skip
            if intersect == 0 {
                continue;
            }

            // node is a leaf
            if *faceid.add(node as usize) != -1 {
                let face = *faceid.add(node as usize) + *(*m).mesh_faceadr.add(meshid as usize);

                // get float vertices
                let vf0 = (*m).mesh_vert.add(3 * (*(*m).mesh_face.add(3 * face as usize) + *(*m).mesh_vertadr.add(meshid as usize)) as usize);
                let vf1 = (*m).mesh_vert.add(3 * (*(*m).mesh_face.add((3 * face + 1) as usize) + *(*m).mesh_vertadr.add(meshid as usize)) as usize);
                let vf2 = (*m).mesh_vert.add(3 * (*(*m).mesh_face.add((3 * face + 2) as usize) + *(*m).mesh_vertadr.add(meshid as usize)) as usize);

                // convert to mjtNum
                let mut v: [[f64; 3]; 3] = [[0.0; 3]; 3];
                for i in 0..3usize {
                    for j in 0..3usize {
                        v[i][j] = *(*m).mesh_vert.add((3 * (*(*m).mesh_face.add((3 * face as usize + i) as usize) + *(*m).mesh_vertadr.add(meshid as usize)) as usize + j) as usize) as f64;
                    }
                }

                // solve
                let sol = ray_triangle(
                    v.as_mut_ptr() as *mut [f64; 3],
                    lpnt.as_ptr(), lvec.as_ptr(), b0.as_ptr(), b1.as_ptr(),
                    if !normal.is_null() { normal_local.as_mut_ptr() } else { std::ptr::null_mut() },
                );

                // update
                if sol >= 0.0 && (x < 0.0 || sol < x) {
                    x = sol;
                    if !normal.is_null() {
                        crate::engine::engine_util_blas::mju_copy3(normal, normal_local.as_ptr());
                    }
                }
                continue;
            }

            // add children to the stack
            for i in 0..2i32 {
                if *child.add((2 * node + i) as usize) != -1 {
                    if nstack >= MJ_MAXTREEDEPTH as i32 {
                        crate::engine::engine_util_errmem::mju_error(
                            b"BVH stack depth exceeded in geom %d.\0".as_ptr() as *const i8,
                        );
                    }
                    stack[nstack as usize] = *child.add((2 * node + i) as usize);
                    nstack += 1;
                }
            }
        }

        // rotate normal to global frame
        if !normal.is_null() && x >= 0.0 {
            crate::engine::engine_util_blas::mju_mulMatVec3(
                normal, (*d).geom_xmat.add(9 * id as usize), normal as *const f64,
            );
        }

        x
    }
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

/// C: mju_multiRayPrepare (engine/engine_ray.h:26)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_latitude, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_longitude, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_point_in_box, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_eliminate, cxx:_mju_addTo3, cxx:_mju_copy, cxx:_mju_dist3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatVec3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_multiRayPrepare(m: *const mjModel, d: *const mjData, pnt: *const f64, ray_xmat: *const f64, geomgroup: *const u8, flg_static: bool, bodyexclude: i32, cutoff: f64, geom_ba: *mut f64, geom_eliminate: *mut i32) {
    const MJ_MAXVAL: f64 = 1e10;
    const MJ_PI: f64 = std::f64::consts::PI;
    const MJ_MINVAL: f64 = 1e-15;

    // SAFETY: m, d, pnt valid. geom_ba and geom_eliminate have at least ngeom elements.
    unsafe {
        if !ray_xmat.is_null() {
            crate::engine::engine_util_errmem::mju_error(
                b"ray_xmat is currently unused, should be NULL\0".as_ptr() as *const i8,
            );
        }

        // compute eliminate flag for all geoms
        for geomid in 0..(*m).ngeom as i32 {
            *geom_eliminate.add(geomid as usize) = ray_eliminate(m, d, geomid, geomgroup, flg_static, bodyexclude);
        }

        for b in 0..(*m).nbody as i32 {
            // skip precomputation if no bounding volume is available
            if *(*m).body_bvhadr.add(b as usize) == -1 {
                continue;
            }

            // loop over child geoms, compute bounding angles
            for i in 0..*(*m).body_geomnum.add(b as usize) {
                let g = i + *(*m).body_geomadr.add(b as usize);
                let mut AABB: [f64; 4] = [MJ_MAXVAL, MJ_MAXVAL, -MJ_MAXVAL, -MJ_MAXVAL];
                let aabb = (*m).geom_aabb.add(6 * g as usize);
                let xpos = (*d).geom_xpos.add(3 * g as usize);
                let xmat = (*d).geom_xmat.add(9 * g as usize);

                // skip if eliminated by flags
                if *geom_eliminate.add(g as usize) != 0 {
                    continue;
                }

                // add to geom_eliminate if distance of bounding sphere is above cutoff
                if crate::engine::engine_util_blas::mju_dist3((*d).geom_xpos.add(3 * g as usize), pnt)
                    > cutoff + *(*m).geom_rbound.add(g as usize) {
                    *geom_eliminate.add(g as usize) = 1;
                    continue;
                }

                if point_in_box(aabb, xpos, xmat, pnt) != 0 {
                    *geom_ba.add(4 * g as usize + 0) = -MJ_PI;
                    *geom_ba.add(4 * g as usize + 1) = 0.0;
                    *geom_ba.add(4 * g as usize + 2) = MJ_PI;
                    *geom_ba.add(4 * g as usize + 3) = MJ_PI;
                    continue;
                }

                // loop over box vertices, compute spherical aperture
                for v in 0..8i32 {
                    let mut vert: [f64; 3] = [0.0; 3];
                    let mut bx: [f64; 3] = [0.0; 3];
                    vert[0] = if v & 1 != 0 { *aabb.add(0) + *aabb.add(3) } else { *aabb.add(0) - *aabb.add(3) };
                    vert[1] = if v & 2 != 0 { *aabb.add(1) + *aabb.add(4) } else { *aabb.add(1) - *aabb.add(4) };
                    vert[2] = if v & 4 != 0 { *aabb.add(2) + *aabb.add(5) } else { *aabb.add(2) - *aabb.add(5) };

                    // rotate to the world frame
                    crate::engine::engine_util_blas::mju_mulMatVec3(bx.as_mut_ptr(), xmat, vert.as_ptr());
                    crate::engine::engine_util_blas::mju_addTo3(bx.as_mut_ptr(), xpos);

                    // spherical coordinates
                    crate::engine::engine_util_blas::mju_sub3(vert.as_mut_ptr(), bx.as_ptr(), pnt);
                    let azimuth = longitude(vert.as_ptr());
                    let elevation = latitude(vert.as_ptr());

                    // update bounds
                    AABB[0] = crate::engine::engine_util_misc::mju_min(AABB[0], azimuth);
                    AABB[1] = crate::engine::engine_util_misc::mju_min(AABB[1], elevation);
                    AABB[2] = crate::engine::engine_util_misc::mju_max(AABB[2], azimuth);
                    AABB[3] = crate::engine::engine_util_misc::mju_max(AABB[3], elevation);
                }

                // add distance-dependent angular margin to account for edge/face curvature
                let max_half = crate::engine::engine_util_misc::mju_max(
                    *aabb.add(3),
                    crate::engine::engine_util_misc::mju_max(*aabb.add(4), *aabb.add(5)),
                );
                let dist = crate::engine::engine_util_blas::mju_dist3(pnt, xpos);
                if dist > MJ_MINVAL {
                    let margin = max_half.atan2(dist);
                    AABB[0] -= margin;
                    AABB[1] -= margin;
                    AABB[2] += margin;
                    AABB[3] += margin;
                }

                // azimuth crosses discontinuity, fall back to no angular culling
                if AABB[2] - AABB[0] > MJ_PI {
                    AABB[0] = -MJ_PI;
                    AABB[1] = 0.0;
                    AABB[2] = MJ_PI;
                    AABB[3] = MJ_PI;
                }

                // elevation overflow, fall back to no angular culling
                if AABB[3] - AABB[1] > MJ_PI {
                    AABB[0] = -MJ_PI;
                    AABB[1] = 0.0;
                    AABB[2] = MJ_PI;
                    AABB[3] = MJ_PI;
                }

                crate::engine::engine_util_blas::mju_copy(geom_ba.add(4 * g as usize), AABB.as_ptr(), 4);
            }
        }
    }
}

/// C: mj_rayHfield (engine/engine_ray.h:47)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_map, cxx-internal:engine_ray.c.o:_ray_box, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_round, cxx:_mju_zero3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_rayHfield(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: m, d, pnt, vec valid. normal may be null. geomid indexes a valid hfield geom.
    unsafe {
        // clear normal if given
        if !normal.is_null() {
            crate::engine::engine_util_blas::mju_zero3(normal);
        }

        // check geom type
        if *(*m).geom_type.add(geomid as usize) as u32 != 5 { // mjGEOM_HFIELD = 5
            crate::engine::engine_util_errmem::mju_error(
                b"geom with hfield type expected\0".as_ptr() as *const i8,
            );
        }

        // hfield id and dimensions
        let hid = *(*m).geom_dataid.add(geomid as usize);
        let nrow = *(*m).hfield_nrow.add(hid as usize);
        let ncol = *(*m).hfield_ncol.add(hid as usize);
        let size = (*m).hfield_size.add(4 * hid as usize);
        let data = (*m).hfield_data.add(*(*m).hfield_adr.add(hid as usize) as usize);

        // compute size and pos of base box
        let base_size: [f64; 3] = [*size.add(0), *size.add(1), *size.add(3) * 0.5];
        let xmat = (*d).geom_xmat.add(9 * geomid as usize);
        let xpos = (*d).geom_xpos.add(3 * geomid as usize);
        let base_pos: [f64; 3] = [
            *xpos.add(0) - *xmat.add(2) * *size.add(3) * 0.5,
            *xpos.add(1) - *xmat.add(5) * *size.add(3) * 0.5,
            *xpos.add(2) - *xmat.add(8) * *size.add(3) * 0.5,
        ];

        // compute size and pos of top box
        let top_size: [f64; 3] = [*size.add(0), *size.add(1), *size.add(2) * 0.5];
        let top_pos: [f64; 3] = [
            *xpos.add(0) + *xmat.add(2) * *size.add(2) * 0.5,
            *xpos.add(1) + *xmat.add(5) * *size.add(2) * 0.5,
            *xpos.add(2) + *xmat.add(8) * *size.add(2) * 0.5,
        ];

        // init: intersection with base box
        let mut normal_base: [f64; 3] = [0.0; 3];
        let mut x: f64 = ray_box(
            base_pos.as_ptr(), xmat, base_size.as_ptr(), pnt, vec,
            std::ptr::null_mut(),
            if !normal.is_null() { normal_base.as_mut_ptr() } else { std::ptr::null_mut() },
        );

        // check top box: done if no intersection
        let mut all: [f64; 6] = [0.0; 6];
        let top_intersect = ray_box(
            top_pos.as_ptr(), xmat, top_size.as_ptr(), pnt, vec,
            all.as_mut_ptr(), std::ptr::null_mut(),
        );
        if top_intersect < 0.0 {
            if !normal.is_null() && x >= 0.0 {
                crate::engine::engine_util_blas::mju_copy3(normal, normal_base.as_ptr());
            }
            return x;
        }

        // map to local frame
        let mut lpnt: [f64; 3] = [0.0; 3];
        let mut lvec: [f64; 3] = [0.0; 3];
        ray_map(xpos, xmat, pnt, vec, lpnt.as_mut_ptr(), lvec.as_mut_ptr());

        // construct basis vectors of normal plane
        let mut b0: [f64; 3] = [1.0, 1.0, 1.0];
        let mut b1: [f64; 3] = [0.0; 3];
        if lvec[0].abs() >= lvec[1].abs() && lvec[0].abs() >= lvec[2].abs() {
            b0[0] = 0.0;
        } else if lvec[1].abs() >= lvec[2].abs() {
            b0[1] = 0.0;
        } else {
            b0[2] = 0.0;
        }
        let dot_lvec_b0 = crate::engine::engine_util_blas::mju_dot3(lvec.as_ptr(), b0.as_ptr());
        let dot_lvec_lvec = crate::engine::engine_util_blas::mju_dot3(lvec.as_ptr(), lvec.as_ptr());
        crate::engine::engine_util_blas::mju_addScl3(
            b1.as_mut_ptr(), b0.as_ptr(), lvec.as_ptr(), -dot_lvec_b0 / dot_lvec_lvec,
        );
        crate::engine::engine_util_blas::mju_normalize3(b1.as_mut_ptr());
        crate::engine::engine_util_spatial::mju_cross(b0.as_mut_ptr(), b1.as_ptr(), lvec.as_ptr());
        crate::engine::engine_util_blas::mju_normalize3(b0.as_mut_ptr());

        // find ray segment intersecting top box
        let mut seg: [f64; 2] = [0.0, top_intersect];
        for i in 0..6 {
            if all[i] > seg[1] {
                seg[0] = top_intersect;
                seg[1] = all[i];
            }
        }

        // project segment endpoints in horizontal plane, discretize
        let dx = (2.0 * *size.add(0)) / (ncol - 1) as f64;
        let dy = (2.0 * *size.add(1)) / (nrow - 1) as f64;
        let mut SX: [f64; 2] = [0.0; 2];
        let mut SY: [f64; 2] = [0.0; 2];
        for i in 0..2 {
            SX[i] = (lpnt[0] + seg[i] * lvec[0] + *size.add(0)) / dx;
            SY[i] = (lpnt[1] + seg[i] * lvec[1] + *size.add(1)) / dy;
        }

        // compute ranges, with +1 padding
        let sx_min = if SX[0] < SX[1] { SX[0] } else { SX[1] };
        let sx_max = if SX[0] > SX[1] { SX[0] } else { SX[1] };
        let sy_min = if SY[0] < SY[1] { SY[0] } else { SY[1] };
        let sy_max = if SY[0] > SY[1] { SY[0] } else { SY[1] };
        let cmin = if 0 > (sx_min.floor() as i32 - 1) { 0 } else { sx_min.floor() as i32 - 1 };
        let cmax = if ncol - 1 < (sx_max.ceil() as i32 + 1) { ncol - 1 } else { sx_max.ceil() as i32 + 1 };
        let rmin = if 0 > (sy_min.floor() as i32 - 1) { 0 } else { sy_min.floor() as i32 - 1 };
        let rmax = if nrow - 1 < (sy_max.ceil() as i32 + 1) { nrow - 1 } else { sy_max.ceil() as i32 + 1 };

        // local normal
        let mut normal_local: [f64; 3] = [0.0; 3];
        if !normal.is_null() && x >= 0.0 {
            crate::engine::engine_util_blas::mju_mulMatTVec3(
                normal_local.as_mut_ptr(), xmat, normal_base.as_ptr(),
            );
        }

        // check triangles within bounds
        let mut r = rmin;
        while r < rmax {
            let mut c = cmin;
            while c < cmax {
                let mut normal_tri: [f64; 3] = [0.0; 3];

                // first triangle
                let mut va: [[f64; 3]; 3] = [
                    [dx * c as f64 - *size.add(0), dy * r as f64 - *size.add(1), *data.add((r * ncol + c) as usize) as f64 * *size.add(2)],
                    [dx * (c + 1) as f64 - *size.add(0), dy * r as f64 - *size.add(1), *data.add((r * ncol + (c + 1)) as usize) as f64 * *size.add(2)],
                    [dx * (c + 1) as f64 - *size.add(0), dy * (r + 1) as f64 - *size.add(1), *data.add(((r + 1) * ncol + (c + 1)) as usize) as f64 * *size.add(2)],
                ];
                let sol = ray_triangle(
                    va.as_mut_ptr() as *mut [f64; 3], lpnt.as_ptr(), lvec.as_ptr(),
                    b0.as_ptr(), b1.as_ptr(),
                    if !normal.is_null() { normal_tri.as_mut_ptr() } else { std::ptr::null_mut() },
                );
                if sol >= 0.0 && (x < 0.0 || sol < x) {
                    x = sol;
                    if !normal.is_null() {
                        crate::engine::engine_util_blas::mju_copy3(normal_local.as_mut_ptr(), normal_tri.as_ptr());
                    }
                }

                // second triangle
                let mut vb: [[f64; 3]; 3] = [
                    [dx * c as f64 - *size.add(0), dy * r as f64 - *size.add(1), *data.add((r * ncol + c) as usize) as f64 * *size.add(2)],
                    [dx * (c + 1) as f64 - *size.add(0), dy * (r + 1) as f64 - *size.add(1), *data.add(((r + 1) * ncol + (c + 1)) as usize) as f64 * *size.add(2)],
                    [dx * c as f64 - *size.add(0), dy * (r + 1) as f64 - *size.add(1), *data.add(((r + 1) * ncol + c) as usize) as f64 * *size.add(2)],
                ];
                let sol2 = ray_triangle(
                    vb.as_mut_ptr() as *mut [f64; 3], lpnt.as_ptr(), lvec.as_ptr(),
                    b0.as_ptr(), b1.as_ptr(),
                    if !normal.is_null() { normal_tri.as_mut_ptr() } else { std::ptr::null_mut() },
                );
                if sol2 >= 0.0 && (x < 0.0 || sol2 < x) {
                    x = sol2;
                    if !normal.is_null() {
                        crate::engine::engine_util_blas::mju_copy3(normal_local.as_mut_ptr(), normal_tri.as_ptr());
                    }
                }

                c += 1;
            }
            r += 1;
        }

        // check viable sides of top box
        for i in 0..4i32 {
            if all[i as usize] >= 0.0 && (all[i as usize] < x || x < 0.0) {
                // normalized height of intersection point
                let z = (lpnt[2] + all[i as usize] * lvec[2]) / *size.add(2);

                let y: f64;
                let y0: f64;
                let z0: f64;
                let z1: f64;

                // side normal to x-axis
                if i < 2 {
                    y = (lpnt[1] + all[i as usize] * lvec[1] + *size.add(1)) / dy;
                    y0 = 0.0f64.max(((nrow - 2) as f64).min(y.floor()));
                    z0 = *data.add((crate::engine::engine_util_misc::mju_round(y0) * ncol + if i == 1 { ncol - 1 } else { 0 }) as usize) as f64;
                    z1 = *data.add((crate::engine::engine_util_misc::mju_round(y0 + 1.0) * ncol + if i == 1 { ncol - 1 } else { 0 }) as usize) as f64;
                }
                // side normal to y-axis
                else {
                    y = (lpnt[0] + all[i as usize] * lvec[0] + *size.add(0)) / dx;
                    y0 = 0.0f64.max(((ncol - 2) as f64).min(y.floor()));
                    z0 = *data.add((crate::engine::engine_util_misc::mju_round(y0) + if i == 3 { (nrow - 1) * ncol } else { 0 }) as usize) as f64;
                    z1 = *data.add((crate::engine::engine_util_misc::mju_round(y0 + 1.0) + if i == 3 { (nrow - 1) * ncol } else { 0 }) as usize) as f64;
                }

                // check if point is below line segment
                if z < z0 * (y0 + 1.0 - y) + z1 * (y - y0) {
                    x = all[i as usize];

                    // compute normal
                    if !normal.is_null() {
                        crate::engine::engine_util_blas::mju_zero3(normal_local.as_mut_ptr());
                        if i == 0 { normal_local[0] = -1.0; }
                        else if i == 1 { normal_local[0] = 1.0; }
                        else if i == 2 { normal_local[1] = -1.0; }
                        else if i == 3 { normal_local[1] = 1.0; }
                    }
                }
            }
        }

        // rotate normal to global frame
        if !normal.is_null() && x >= 0.0 {
            crate::engine::engine_util_blas::mju_mulMatVec3(normal, xmat, normal_local.as_ptr());
        }

        x
    }
}

/// C: ray_triangle (engine/engine_ray.h:51)
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

/// C: mj_rayMesh (engine/engine_ray.h:55)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx:_mju_message, cxx:_mju_rayTree, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_rayMesh(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // SAFETY: m, d, pnt, vec valid. normal may be null. geomid indexes a valid geom.
    unsafe {
        // clear normal if given
        if !normal.is_null() {
            crate::engine::engine_util_blas::mju_zero3(normal);
        }

        // check geom type
        if *(*m).geom_type.add(geomid as usize) as u32 != 7 { // mjGEOM_MESH = 7
            crate::engine::engine_util_errmem::mju_error(
                b"geom with mesh type expected\0".as_ptr() as *const i8,
            );
        }

        // bounding box test
        if ray_box(
            (*d).geom_xpos.add(3 * geomid as usize),
            (*d).geom_xmat.add(9 * geomid as usize),
            (*m).geom_size.add(3 * geomid as usize),
            pnt, vec, std::ptr::null_mut(), std::ptr::null_mut(),
        ) < 0.0 {
            return -1.0;
        }

        mju_rayTree(m, d, geomid, pnt, vec, normal)
    }
}

/// C: mju_rayGeom (engine/engine_ray.h:59)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_capsule, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_cylinder, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_ellipsoid, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:_ray_plane, cxx-internal:engine_ray.c.o:_ray_box, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rayGeom(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, geomtype: i32, normal: *mut f64) -> f64 {
    // SAFETY: all pointers valid arrays of documented sizes (caller contract)
    unsafe {
        match geomtype {
            0 => { // mjGEOM_PLANE
                ray_plane(pos, mat, size, pnt, vec, normal)
            }
            2 => { // mjGEOM_SPHERE
                ray_sphere(pos, mat, *size.add(0) * *size.add(0), pnt, vec, normal)
            }
            3 => { // mjGEOM_CAPSULE
                ray_capsule(pos, mat, size, pnt, vec, normal)
            }
            4 => { // mjGEOM_ELLIPSOID
                ray_ellipsoid(pos, mat, size, pnt, vec, normal)
            }
            5 => { // mjGEOM_CYLINDER
                ray_cylinder(pos, mat, size, pnt, vec, normal)
            }
            6 => { // mjGEOM_BOX
                ray_box(pos, mat, size, pnt, vec, std::ptr::null_mut(), normal)
            }
            _ => {
                -1.0
            }
        }
    }
}

