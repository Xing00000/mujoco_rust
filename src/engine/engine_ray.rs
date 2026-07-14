//! Port of: engine/engine_ray.c
//! IR hash: 9614bd3d92e7766b
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
pub fn ray_eliminate(m: *const mjModel, d: *const mjData, geomid: i32, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32) -> i32 {
    // SAFETY: m points to valid mjModel, geomid is within bounds (caller contract)
    unsafe {
        let m = &*m;
        let gid = geomid as usize;

        // body exclusion
        if *m.geom_bodyid.add(gid) == bodyexclude {
            return 1;
        }

        // invisible geom exclusion
        if *m.geom_matid.add(gid) < 0 && *m.geom_rgba.add(4 * gid + 3) == 0.0 {
            return 1;
        }

        // invisible material exclusion
        if *m.geom_matid.add(gid) >= 0
            && *m.mat_rgba.add(4 * (*m.geom_matid.add(gid) as usize) + 3) == 0.0
        {
            return 1;
        }

        // static exclusion
        if flg_static._data[0] == 0
            && *m.body_weldid.add(*m.geom_bodyid.add(gid) as usize) == 0
        {
            return 1;
        }

        // no geomgroup inclusion
        if geomgroup.is_null() {
            return 0;
        }

        // group inclusion/exclusion (mjNGROUP = 6)
        let raw_group = *m.geom_group.add(gid);
        let groupid = if 5 < (if 0 > raw_group { 0 } else { raw_group }) {
            5
        } else {
            if 0 > raw_group { 0 } else { raw_group }
        } as usize;
        (*geomgroup.add(groupid) == 0) as i32
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
/// Calls: mju_zero3, ray_map
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
/// Calls: mju_addScl3, mju_normalize3, mju_sub3, mju_zero3, ray_quad
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
/// Calls: mju_addScl3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatVec3, mju_normalize3, mju_raySlab, mju_zero3, ray_map, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_tree(m: *const mjModel, d: *const mjData, id: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // mju_rayTree
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
    todo!() // mj_raySdf
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
    // SAFETY: all pointers valid arrays of documented sizes (caller contract)
    unsafe {
        let mut point: [f64; 3] = [0.0; 3];

        // compute point in local coordinates of the box
        crate::engine::engine_util_blas::mju_sub3(point.as_mut_ptr(), pnt, xpos);
        crate::engine::engine_util_blas::mju_mul_mat_t_vec3(point.as_mut_ptr(), xmat, point.as_ptr());
        crate::engine::engine_util_blas::mju_sub_from3(point.as_mut_ptr(), aabb);

        // check intersections
        for j in 0..3 {
            if f64::abs(point[j]) > *aabb.add(3 + j) {
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
pub fn mju_single_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, ray_eliminate: *mut i32, geom_ba: *mut f64, geomid: *mut i32, normal: *mut f64) -> f64 {
    todo!() // mju_singleRay
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
    todo!() // mju_multiRayPrepare
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
    todo!() // mj_multiRay
}

/// C: mj_ray (engine/engine_ray.h:42)
/// Calls: mj_rayHfield, mj_rayMesh, mj_raySdf, mju_copy3, mju_message, mju_norm3, mju_rayGeom, mju_zero3, ray_eliminate
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray(m: *const mjModel, d: *const mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: *mut i32, normal: *mut f64) -> f64 {
    todo!() // mj_ray
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
    todo!() // mj_rayHfield
}

/// C: ray_triangle (engine/engine_ray.h:51)
/// Calls: mju_copy3, mju_cross, mju_dot3, mju_normalize3, mju_sub3, mju_zero3
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
/// Calls: mju_message, mju_rayTree, mju_zero3, ray_box
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_mesh(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // mj_rayMesh
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

/// C: mj_rayFlex (engine/engine_ray.h:64)
/// Calls: mju_add3, mju_addScl3, mju_copy3, mju_cross, mju_dist3, mju_dot3, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, mju_rayGeom, mju_scl3, mju_zero3, ray_box, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_flex(m: *const mjModel, d: *const mjData, flex_layer: i32, flg_vert: mjtBool, flg_edge: mjtBool, flg_face: mjtBool, flg_skin: mjtBool, flexid: i32, pnt: *const f64, vec: *const f64, vertid: *mut i32, normal: *mut f64) -> f64 {
    todo!() // mj_rayFlex
}

/// C: mju_raySkin (engine/engine_ray.h:70)
/// Calls: mju_addScl3, mju_cross, mju_dist3, mju_dot3, mju_normalize3, ray_box, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_skin(nface: i32, nvert: i32, face: *const i32, vert: *const f32, pnt: *const f64, vec: *const f64, vertid: *mut i32) -> f64 {
    // SAFETY: all pointers valid arrays of documented sizes (caller contract)
    unsafe {
        // compute bounding box
        let mut box_: [[f64; 2]; 3] = [[0.0; 2]; 3];
        for i in 0..nvert as usize {
            for j in 0..3 {
                let val = *vert.add(3*i + j) as f64;
                if box_[j][0] > val || i == 0 {
                    box_[j][0] = val;
                }
                if box_[j][1] < val || i == 0 {
                    box_[j][1] = val;
                }
            }
        }

        // construct box geom
        let mut pos: [f64; 3] = [0.0; 3];
        let mut size: [f64; 3] = [0.0; 3];
        let mat: [f64; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for j in 0..3 {
            pos[j] = 0.5 * (box_[j][0] + box_[j][1]);
            size[j] = 0.5 * (box_[j][1] - box_[j][0]);
        }

        // apply bounding-box filter
        if ray_box(pos.as_ptr(), mat.as_ptr(), size.as_ptr(), pnt, vec, std::ptr::null_mut(), std::ptr::null_mut()) < 0.0 {
            return -1.0;
        }

        // construct basis vectors of normal plane
        let mut b0: [f64; 3] = [1.0, 1.0, 1.0];
        let mut b1: [f64; 3] = [0.0; 3];
        if f64::abs(*vec.add(0)) >= f64::abs(*vec.add(1)) && f64::abs(*vec.add(0)) >= f64::abs(*vec.add(2)) {
            b0[0] = 0.0;
        } else if f64::abs(*vec.add(1)) >= f64::abs(*vec.add(2)) {
            b0[1] = 0.0;
        } else {
            b0[2] = 0.0;
        }
        let dot_lvec_b0 = crate::engine::engine_util_blas::mju_dot3(vec, b0.as_ptr());
        let dot_lvec_lvec = crate::engine::engine_util_blas::mju_dot3(vec, vec);
        crate::engine::engine_util_blas::mju_add_scl3(b1.as_mut_ptr(), b0.as_ptr(), vec, -dot_lvec_b0 / dot_lvec_lvec);
        crate::engine::engine_util_blas::mju_normalize3(b1.as_mut_ptr());
        crate::engine::engine_util_spatial::mju_cross(b0.as_mut_ptr(), b1.as_ptr(), vec);
        crate::engine::engine_util_blas::mju_normalize3(b0.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;

        // process all faces
        for i in 0..nface as usize {
            // get float vertices
            let vf0 = vert.add(3 * (*face.add(3*i + 0) as usize));
            let vf1 = vert.add(3 * (*face.add(3*i + 1) as usize));
            let vf2 = vert.add(3 * (*face.add(3*i + 2) as usize));

            // convert to mjtNum
            let mut v: [[f64; 3]; 3] = [[0.0; 3]; 3];
            for k in 0..3 {
                v[0][k] = *vf0.add(k) as f64;
                v[1][k] = *vf1.add(k) as f64;
                v[2][k] = *vf2.add(k) as f64;
            }

            // solve
            let sol = ray_triangle(v.as_mut_ptr(), pnt, vec, b0.as_ptr(), b1.as_ptr(), std::ptr::null_mut());

            // update
            if sol >= 0.0 && (x < 0.0 || sol < x) {
                x = sol;

                // construct intersection point
                let mut intersect: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_add_scl3(intersect.as_mut_ptr(), pnt, vec, sol);

                // find nearest vertex
                let mut dist = crate::engine::engine_util_blas::mju_dist3(intersect.as_ptr(), v[0].as_ptr());
                if !vertid.is_null() {
                    *vertid = *face.add(3*i);
                }
                for j in 1..3 {
                    let newdist = crate::engine::engine_util_blas::mju_dist3(intersect.as_ptr(), v[j].as_ptr());
                    if newdist < dist {
                        dist = newdist;
                        if !vertid.is_null() {
                            *vertid = *face.add(3*i + j);
                        }
                    }
                }
            }
        }

        x
    }
}

