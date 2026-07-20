//! Port of: engine/engine_util_misc.c
//! IR hash: 73a9f665ec0ecfc0
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: is_intersect (engine/engine_util_misc.c:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_intersect(p1: *const f64, p2: *const f64, p3: *const f64, p4: *const f64) -> bool {
    const MJ_MINVAL: f64 = 1E-15_f64;
    // SAFETY: caller guarantees p1..p4 point to arrays of at least 2 elements (2D points)
    unsafe {
        // compute determinant, check
        let det = (*p4.add(1) - *p3.add(1)) * (*p2.add(0) - *p1.add(0))
                - (*p4.add(0) - *p3.add(0)) * (*p2.add(1) - *p1.add(1));
        if det.abs() < MJ_MINVAL {
            return false;
        }

        // compute intersection point on each line
        let a = ((*p4.add(0) - *p3.add(0)) * (*p1.add(1) - *p3.add(1))
               - (*p4.add(1) - *p3.add(1)) * (*p1.add(0) - *p3.add(0))) / det;
        let b = ((*p2.add(0) - *p1.add(0)) * (*p1.add(1) - *p3.add(1))
               - (*p2.add(1) - *p1.add(1)) * (*p1.add(0) - *p3.add(0))) / det;

        a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0
    }
}

/// C: length_circle (engine/engine_util_misc.c:55)
/// Calls: mju_dot, mju_normalize
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn length_circle(p0: *const f64, p1: *const f64, ind: i32, radius: f64) -> f64 {
    use crate::engine::engine_util_blas::{mju_normalize, mju_dot};
    const MJ_PI: f64 = 3.14159265358979323846_f64;

    // SAFETY: caller guarantees p0[2], p1[2] are valid
    unsafe {
        let mut p0n: [f64; 2] = [*p0.add(0), *p0.add(1)];
        let mut p1n: [f64; 2] = [*p1.add(0), *p1.add(1)];

        // compute angle between 0 and pi
        mju_normalize(p0n.as_mut_ptr(), 2);
        mju_normalize(p1n.as_mut_ptr(), 2);
        let mut angle: f64 = f64::acos(mju_dot(p0n.as_ptr(), p1n.as_ptr(), 2));

        // flip if necessary
        let cross: f64 = *p0.add(1) * *p1.add(0) - *p0.add(0) * *p1.add(1);
        if (cross > 0.0 && ind != 0) || (cross < 0.0 && ind == 0) {
            angle = 2.0 * MJ_PI - angle;
        }

        radius * angle
    }
}

/// C: wrap_circle (engine/engine_util_misc.c:78)
/// Calls: is_intersect, length_circle, mju_add, mju_dot, mju_normalize, mju_sub
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_circle(pnt: *mut f64, end: *const f64, side: *const f64, radius: f64) -> f64 {
    use crate::engine::engine_util_blas::{mju_add, mju_sub, mju_normalize, mju_dot};
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees pnt[4], end[4], side[2] (or null) are valid
    unsafe {
        let sqlen0: f64 = *end.add(0) * *end.add(0) + *end.add(1) * *end.add(1);
        let sqlen1: f64 = *end.add(2) * *end.add(2) + *end.add(3) * *end.add(3);
        let sqrad: f64 = radius * radius;

        // either point inside circle or circle too small: no wrap
        if sqlen0 < sqrad || sqlen1 < sqrad || radius < MJ_MINVAL {
            return -1.0;
        }

        // points too close: no wrap
        let dif: [f64; 2] = [*end.add(2) - *end.add(0), *end.add(3) - *end.add(1)];
        let dd: f64 = dif[0] * dif[0] + dif[1] * dif[1];
        if dd < MJ_MINVAL {
            return -1.0;
        }

        // find nearest point on line segment to origin: a*dif + d0
        let mut a: f64 = -(dif[0] * *end.add(0) + dif[1] * *end.add(1)) / dd;
        if a < 0.0 {
            a = 0.0;
        } else if a > 1.0 {
            a = 1.0;
        }

        // check for intersection and side
        let tmp_check: [f64; 2] = [a * dif[0] + *end.add(0), a * dif[1] + *end.add(1)];
        if tmp_check[0] * tmp_check[0] + tmp_check[1] * tmp_check[1] > sqrad
            && (side.is_null() || mju_dot(side, tmp_check.as_ptr(), 2) >= 0.0)
        {
            return -1.0;
        }

        let sqrt0: f64 = f64::sqrt(sqlen0 - sqrad);
        let sqrt1: f64 = f64::sqrt(sqlen1 - sqrad);

        // construct the two solutions, compute goodness
        let mut sol: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];
        let mut good: [f64; 2] = [0.0; 2];
        let mut tmp: [f64; 2] = [0.0; 2];

        for i in 0..2 {
            let sgn: f64 = if i == 0 { 1.0 } else { -1.0 };

            sol[i][0][0] = (*end.add(0) * sqrad + sgn * radius * *end.add(1) * sqrt0) / sqlen0;
            sol[i][0][1] = (*end.add(1) * sqrad - sgn * radius * *end.add(0) * sqrt0) / sqlen0;
            sol[i][1][0] = (*end.add(2) * sqrad - sgn * radius * *end.add(3) * sqrt1) / sqlen1;
            sol[i][1][1] = (*end.add(3) * sqrad + sgn * radius * *end.add(2) * sqrt1) / sqlen1;

            // goodness: close to sd, or shorter path
            if !side.is_null() {
                mju_add(tmp.as_mut_ptr(), sol[i][0].as_ptr(), sol[i][1].as_ptr(), 2);
                mju_normalize(tmp.as_mut_ptr(), 2);
                good[i] = mju_dot(tmp.as_ptr(), side, 2);
            } else {
                mju_sub(tmp.as_mut_ptr(), sol[i][0].as_ptr(), sol[i][1].as_ptr(), 2);
                good[i] = -mju_dot(tmp.as_ptr(), tmp.as_ptr(), 2);
            }

            // penalize for intersection
            if is_intersect(end, sol[i][0].as_ptr(), end.add(2), sol[i][1].as_ptr()) {
                good[i] = -10000.0;
            }
        }

        // select the better solution
        let i: usize = if good[0] > good[1] { 0 } else { 1 };
        *pnt.add(0) = sol[i][0][0];
        *pnt.add(1) = sol[i][0][1];
        *pnt.add(2) = sol[i][1][0];
        *pnt.add(3) = sol[i][1][1];

        // check for intersection
        if is_intersect(end, pnt, end.add(2), pnt.add(2)) {
            return -1.0;
        }

        // return curve length
        length_circle(sol[i][0].as_ptr(), sol[i][1].as_ptr(), i as i32, radius)
    }
}

/// C: wrap_inside (engine/engine_util_misc.c:158)
/// Calls: mju_addScl, mju_copy, mju_max, mju_norm, mju_normalize, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_inside(pnt: *mut f64, end: *const f64, radius: f64) -> f64 {
    use crate::engine::engine_util_blas::{mju_add_scl, mju_copy, mju_norm, mju_normalize, mju_scl};
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees pnt[4], end[4] are valid
    unsafe {
        // algorithm parameters
        let maxiter: i32 = 20;
        let zinit: f64 = 1.0 - 1e-7;
        let tolerance: f64 = 1e-6;

        // constants
        let len0: f64 = mju_norm(end, 2);
        let len1: f64 = mju_norm(end.add(2), 2);
        let dif: [f64; 2] = [*end.add(2) - *end.add(0), *end.add(3) - *end.add(1)];
        let dd: f64 = dif[0] * dif[0] + dif[1] * dif[1];

        // either point inside circle or circle too small: no wrap
        if len0 <= radius || len1 <= radius || radius < MJ_MINVAL || len0 < MJ_MINVAL || len1 < MJ_MINVAL {
            return -1.0;
        }

        // segment-circle intersection: no wrap
        if dd > MJ_MINVAL {
            // find nearest point on line segment to origin: d0 + a*dif
            let a: f64 = -(dif[0] * *end.add(0) + dif[1] * *end.add(1)) / dd;

            // in segment
            if a > 0.0 && a < 1.0 {
                let mut tmp: [f64; 2] = [0.0; 2];
                mju_add_scl(tmp.as_mut_ptr(), end, dif.as_ptr(), a, 2);
                if mju_norm(tmp.as_ptr(), 2) <= radius {
                    return -1.0;
                }
            }
        }

        // prepare default in case of numerical failure: average
        *pnt.add(0) = 0.5 * (*end.add(0) + *end.add(2));
        *pnt.add(1) = 0.5 * (*end.add(1) + *end.add(3));
        mju_normalize(pnt, 2);
        mju_scl(pnt, pnt, radius, 2);
        *pnt.add(2) = *pnt.add(0);
        *pnt.add(3) = *pnt.add(1);

        // compute function parameters: asin(A*z) + asin(B*z) - 2*asin(z) + G = 0
        let a_coeff: f64 = radius / len0;
        let b_coeff: f64 = radius / len1;
        let cos_g: f64 = (len0 * len0 + len1 * len1 - dd) / (2.0 * len0 * len1);
        if cos_g < -1.0 + MJ_MINVAL {
            return -1.0;
        } else if cos_g > 1.0 - MJ_MINVAL {
            return 0.0;
        }
        let g: f64 = f64::acos(cos_g);

        // init
        let mut z: f64 = zinit;
        let mut f: f64 = f64::asin(a_coeff * z) + f64::asin(b_coeff * z) - 2.0 * f64::asin(z) + g;

        // make sure init is not on the other side
        if f > 0.0 {
            return 0.0;
        }

        // Newton method
        let mut iter: i32 = 0;
        while iter < maxiter && f64::abs(f) > tolerance {
            // derivative
            let df: f64 = a_coeff / f64::max(MJ_MINVAL, f64::sqrt(1.0 - z * z * a_coeff * a_coeff))
                        + b_coeff / f64::max(MJ_MINVAL, f64::sqrt(1.0 - z * z * b_coeff * b_coeff))
                        - 2.0 / f64::max(MJ_MINVAL, f64::sqrt(1.0 - z * z));

            // check sign; SHOULD NOT OCCUR
            if df > -MJ_MINVAL {
                return 0.0;
            }

            // new point
            let z1: f64 = z - f / df;

            // make sure we are moving to the left; SHOULD NOT OCCUR
            if z1 > z {
                return 0.0;
            }

            // update solution
            z = z1;
            f = f64::asin(a_coeff * z) + f64::asin(b_coeff * z) - 2.0 * f64::asin(z) + g;

            // exit if positive; SHOULD NOT OCCUR
            if f > tolerance {
                return 0.0;
            }

            iter += 1;
        }

        // check convergence
        if iter >= maxiter {
            return 0.0;
        }

        // finalize: rotation by ang from vec = a or b, depending on cross(a,b) sign
        let mut vec: [f64; 2] = [0.0; 2];
        let ang: f64;
        if *end.add(0) * *end.add(3) - *end.add(1) * *end.add(2) > 0.0 {
            mju_copy(vec.as_mut_ptr(), end, 2);
            ang = f64::asin(z) - f64::asin(a_coeff * z);
        } else {
            mju_copy(vec.as_mut_ptr(), end.add(2), 2);
            ang = f64::asin(z) - f64::asin(b_coeff * z);
        }
        mju_normalize(vec.as_mut_ptr(), 2);
        *pnt.add(0) = radius * (f64::cos(ang) * vec[0] - f64::sin(ang) * vec[1]);
        *pnt.add(1) = radius * (f64::sin(ang) * vec[0] + f64::cos(ang) * vec[1]);
        *pnt.add(2) = *pnt.add(0);
        *pnt.add(3) = *pnt.add(1);

        0.0
    }
}

/// C: flexInterpRotation (engine/engine_util_misc.c:694)
/// Calls: mju_defGradient, mju_mat2Rot, mju_negQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn flex_interp_rotation(order: i32, xpos_c: *const f64, local: *const f64, quat: *mut f64) {
    use crate::engine::engine_util_spatial::{mju_mat2rot, mju_neg_quat};

    // SAFETY: caller guarantees xpos_c, local[3], quat[4] are valid
    unsafe {
        let mut mat: [f64; 9] = [0.0; 9];

        if order > 0 {
            mju_def_gradient(mat.as_mut_ptr(), local, xpos_c, order);
        } else {
            // order 0: fallback to identity matrix
            mat[0] = 1.0;
            mat[4] = 1.0;
            mat[8] = 1.0;
        }

        // find rotation
        *quat.add(0) = 1.0;
        *quat.add(1) = 0.0;
        *quat.add(2) = 0.0;
        *quat.add(3) = 0.0;
        mju_mat2rot(quat, mat.as_ptr());
        mju_neg_quat(quat, quat);
    }
}

/// C: nodeAt (engine/engine_util_misc.c:902)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn node_at(nodexpos: *const f64, ny: i32, nz: i32, i: i32, j: i32, k: i32) -> *const f64 {
    // SAFETY: caller guarantees nodexpos has sufficient extent for the index computation
    unsafe {
        nodexpos.add(3 * (i as usize * ny as usize * nz as usize + j as usize * nz as usize + k as usize))
    }
}

/// C: addWeight (engine/engine_util_misc.c:984)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_weight(nb: *mut i32, body: *mut i32, bweight: *mut f64, b: i32, w: f64) {
    // SAFETY: caller guarantees nb, body, bweight valid; body/bweight have sufficient capacity
    unsafe {
        for i in 0..*nb as usize {
            if *body.add(i) == b {
                if !bweight.is_null() {
                    *bweight.add(i) += w;
                }
                return;
            }
        }
        *body.add(*nb as usize) = b;
        if !bweight.is_null() {
            *bweight.add(*nb as usize) = w;
        }
        *nb += 1;
    }
}

/// C: _decode (engine/engine_util_misc.c:1217)
#[allow(unused_variables, non_snake_case)]
pub fn decode(ch: i8) -> u32 {
    let ch = ch as u8 as char;
    if ch >= 'A' && ch <= 'Z' {
        return (ch as u32) - ('A' as u32);
    }
    if ch >= 'a' && ch <= 'z' {
        return (ch as u32) - ('a' as u32) + 26;
    }
    if ch >= '0' && ch <= '9' {
        return (ch as u32) - ('0' as u32) + 52;
    }
    if ch == '+' {
        return 62;
    }
    if ch == '/' {
        return 63;
    }
    0
}

/// C: historyPhysicalIndex (engine/engine_util_misc.c:1359)
#[allow(unused_variables, non_snake_case)]
pub fn history_physical_index(cursor: i32, n: i32, logical: i32) -> i32 {
    (cursor + 1 + logical) % n
}

/// C: historyFindIndex (engine/engine_util_misc.c:1367)
/// Calls: historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn history_find_index(times: *const f64, n: i32, cursor: i32, t: f64) -> i32 {
    // SAFETY: caller guarantees times[n] is valid
    unsafe {
        let oldest_phys = history_physical_index(cursor, n, 0);
        let newest_phys = history_physical_index(cursor, n, n - 1);
        let t_oldest = *times.add(oldest_phys as usize);
        let t_newest = *times.add(newest_phys as usize);

        // before or at first element
        if t <= t_oldest {
            return 0;
        }

        // after last element
        if t > t_newest {
            return n;
        }

        // circular binary search
        let mut lo: i32 = 0;
        let mut hi: i32 = n - 1;
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            let mid_phys = history_physical_index(cursor, n, mid);
            if *times.add(mid_phys as usize) < t {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        hi
    }
}

/// C: mju_wrap (engine/engine_util_misc.h:32)
/// Calls: mju_addTo3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_norm3, mju_normalize, mju_normalize3, mju_scl, mju_scl3, mju_sub3, wrap_circle, wrap_inside
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_wrap(wpnt: *mut f64, x0: *const f64, x1: *const f64, xpos: *const f64, xmat: *const f64, radius: f64, r#type: i32, side: *const f64) -> f64 {
    use crate::engine::engine_util_blas::{
        mju_sub3, mju_mul_mat_t_vec3, mju_mul_mat_vec3, mju_norm3,
        mju_copy3, mju_normalize3, mju_scl3, mju_add_to3,
        mju_dot3, mju_dot, mju_normalize, mju_scl, mju_norm,
    };
    use crate::engine::engine_util_spatial::mju_cross;

    const MJ_MINVAL: f64 = 1E-15_f64;
    const MJ_WRAP_SPHERE: i32 = 4;
    const MJ_WRAP_CYLINDER: i32 = 5;

    // SAFETY: caller guarantees all pointers are valid with appropriate sizes
    unsafe {
        // check object type; SHOULD NOT OCCUR
        if r#type != MJ_WRAP_SPHERE && r#type != MJ_WRAP_CYLINDER {
            panic!("mju_wrap: unknown wrapping object type {}", r#type);
        }

        // map sites to wrap object's local frame
        let mut tmp: [f64; 3] = [0.0; 3];
        mju_sub3(tmp.as_mut_ptr(), x0, xpos);
        let mut p: [[f64; 3]; 2] = [[0.0; 3]; 2];
        mju_mul_mat_t_vec3(p[0].as_mut_ptr(), xmat, tmp.as_ptr());
        mju_sub3(tmp.as_mut_ptr(), x1, xpos);
        mju_mul_mat_t_vec3(p[1].as_mut_ptr(), xmat, tmp.as_ptr());

        // too close to origin: return
        if mju_norm3(p[0].as_ptr()) < MJ_MINVAL || mju_norm3(p[1].as_ptr()) < MJ_MINVAL {
            return -1.0;
        }

        // construct 2D frame for circle wrap
        let mut axis: [[f64; 3]; 2] = [[0.0; 3]; 2];
        if r#type == MJ_WRAP_SPHERE {
            // 1st axis = p0
            mju_copy3(axis[0].as_mut_ptr(), p[0].as_ptr());
            mju_normalize3(axis[0].as_mut_ptr());

            // normal to p0-0-p1 plane = cross(p0, p1)
            let mut normal: [f64; 3] = [0.0; 3];
            mju_cross(normal.as_mut_ptr(), p[0].as_ptr(), p[1].as_ptr());
            let nrm: f64 = mju_normalize3(normal.as_mut_ptr());

            // if (p0, p1) parallel: different normal
            if nrm < MJ_MINVAL {
                // find max component of axis0
                let mut i: usize = 0;
                if f64::abs(axis[0][1]) > f64::abs(axis[0][0])
                    && f64::abs(axis[0][1]) > f64::abs(axis[0][2])
                {
                    i = 1;
                }
                if f64::abs(axis[0][2]) > f64::abs(axis[0][0])
                    && f64::abs(axis[0][2]) > f64::abs(axis[0][1])
                {
                    i = 2;
                }

                // init second axis: 0 at i; 1 elsewhere
                axis[1][0] = 1.0;
                axis[1][1] = 1.0;
                axis[1][2] = 1.0;
                axis[1][i] = 0.0;

                // recompute normal
                mju_cross(normal.as_mut_ptr(), axis[0].as_ptr(), axis[1].as_ptr());
                mju_normalize3(normal.as_mut_ptr());
            }

            // 2nd axis = cross(normal, p0)
            mju_cross(axis[1].as_mut_ptr(), normal.as_ptr(), axis[0].as_ptr());
            mju_normalize3(axis[1].as_mut_ptr());
        } else {
            // 1st axis = x
            axis[0][0] = 1.0;
            axis[0][1] = 0.0;
            axis[0][2] = 0.0;

            // 2nd axis = y
            axis[1][0] = 0.0;
            axis[1][1] = 1.0;
            axis[1][2] = 0.0;
        }

        // project points in 2D frame: p => d
        let mut s: [f64; 3] = [0.0; 3];
        let mut d: [f64; 4] = [0.0; 4];
        let mut sd: [f64; 2] = [0.0; 2];
        d[0] = mju_dot3(p[0].as_ptr(), axis[0].as_ptr());
        d[1] = mju_dot3(p[0].as_ptr(), axis[1].as_ptr());
        d[2] = mju_dot3(p[1].as_ptr(), axis[0].as_ptr());
        d[3] = mju_dot3(p[1].as_ptr(), axis[1].as_ptr());

        // handle sidesite
        if !side.is_null() {
            // side point: apply same projection as x0, x1
            mju_sub3(tmp.as_mut_ptr(), side, xpos);
            mju_mul_mat_t_vec3(s.as_mut_ptr(), xmat, tmp.as_ptr());

            // side point: project and rescale
            sd[0] = mju_dot3(s.as_ptr(), axis[0].as_ptr());
            sd[1] = mju_dot3(s.as_ptr(), axis[1].as_ptr());
            mju_normalize(sd.as_mut_ptr(), 2);
            mju_scl(sd.as_mut_ptr(), sd.as_ptr(), radius, 2);
        }

        // apply inside wrap or circle wrap
        let wlen: f64;
        let mut pnt: [f64; 4] = [0.0; 4];
        if !side.is_null() && mju_norm3(s.as_ptr()) < radius {
            wlen = wrap_inside(pnt.as_mut_ptr(), d.as_ptr(), radius);
        } else {
            wlen = wrap_circle(
                pnt.as_mut_ptr(),
                d.as_ptr(),
                if !side.is_null() { sd.as_ptr() } else { std::ptr::null() },
                radius,
            );
        }

        // no wrap: return
        if wlen < 0.0 {
            return -1.0;
        }

        // reconstruct 3D points in local frame: res
        let mut res: [f64; 6] = [0.0; 6];
        for i in 0..2 {
            // res = axis0*d0 + axis1*d1
            mju_scl3(res.as_mut_ptr().add(3 * i), axis[0].as_ptr(), pnt[2 * i]);
            mju_scl3(tmp.as_mut_ptr(), axis[1].as_ptr(), pnt[2 * i + 1]);
            mju_add_to3(res.as_mut_ptr().add(3 * i), tmp.as_ptr());
        }

        // cylinder: correct along z
        let mut wlen = wlen;
        if r#type == MJ_WRAP_CYLINDER {
            // set vertical coordinates
            let l0: f64 = f64::sqrt(
                (p[0][0] - res[0]) * (p[0][0] - res[0]) + (p[0][1] - res[1]) * (p[0][1] - res[1]),
            );
            let l1: f64 = f64::sqrt(
                (p[1][0] - res[3]) * (p[1][0] - res[3]) + (p[1][1] - res[4]) * (p[1][1] - res[4]),
            );
            res[2] = p[0][2] + (p[1][2] - p[0][2]) * l0 / (l0 + wlen + l1);
            res[5] = p[0][2] + (p[1][2] - p[0][2]) * (l0 + wlen) / (l0 + wlen + l1);

            // correct wlen for height
            let height: f64 = f64::abs(res[5] - res[2]);
            wlen = f64::sqrt(wlen * wlen + height * height);
        }

        // map back to global frame: wpnt
        mju_mul_mat_vec3(wpnt, xmat, res.as_ptr());
        mju_mul_mat_vec3(wpnt.add(3), xmat, res.as_ptr().add(3));
        mju_add_to3(wpnt, xpos);
        mju_add_to3(wpnt.add(3), xpos);

        wlen
    }
}

/// C: mju_muscleGainLength (engine/engine_util_misc.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain_length(length: f64, lmin: f64, lmax: f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    if lmin <= length && length <= lmax {
        let a: f64 = 0.5 * (lmin + 1.0);
        let b: f64 = 0.5 * (1.0 + lmax);

        if length <= a {
            let x: f64 = (length - lmin) / f64::max(MJ_MINVAL, a - lmin);
            return 0.5 * x * x;
        } else if length <= 1.0 {
            let x: f64 = (1.0 - length) / f64::max(MJ_MINVAL, 1.0 - a);
            return 1.0 - 0.5 * x * x;
        } else if length <= b {
            let x: f64 = (length - 1.0) / f64::max(MJ_MINVAL, b - 1.0);
            return 1.0 - 0.5 * x * x;
        } else {
            let x: f64 = (lmax - length) / f64::max(MJ_MINVAL, lmax - b);
            return 0.5 * x * x;
        }
    }

    0.0
}

/// C: mju_muscleGain (engine/engine_util_misc.h:39)
/// Calls: mju_muscleGainLength
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees lengthrange[2] and prm[9] are valid
    unsafe {
        // unpack parameters
        let range0 = *prm.add(0);
        let range1 = *prm.add(1);
        let mut force = *prm.add(2);
        let scale = *prm.add(3);
        let lmin = *prm.add(4);
        let lmax = *prm.add(5);
        let vmax = *prm.add(6);
        let fvmax = *prm.add(8);

        // scale force if negative
        if force < 0.0 {
            force = scale / f64::max(MJ_MINVAL, acc0);
        }

        // optimum length
        let L0 = (*lengthrange.add(1) - *lengthrange.add(0)) / f64::max(MJ_MINVAL, range1 - range0);

        // normalized length and velocity
        let L = range0 + (len - *lengthrange.add(0)) / f64::max(MJ_MINVAL, L0);
        let V = vel / f64::max(MJ_MINVAL, L0 * vmax);

        // length curve
        let FL = mju_muscle_gain_length(L, lmin, lmax);

        // velocity curve
        let FV: f64;
        let y = fvmax - 1.0;
        if V <= -1.0 {
            FV = 0.0;
        } else if V <= 0.0 {
            FV = (V + 1.0) * (V + 1.0);
        } else if V <= y {
            FV = fvmax - (y - V) * (y - V) / f64::max(MJ_MINVAL, y);
        } else {
            FV = fvmax;
        }

        // compute FVL and scale, make it negative
        -force * FL * FV
    }
}

/// C: mju_muscleBias (engine/engine_util_misc.h:43)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_bias(len: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees lengthrange[2] and prm[9] are valid
    unsafe {
        // unpack parameters
        let range0 = *prm.add(0);
        let range1 = *prm.add(1);
        let mut force = *prm.add(2);
        let scale = *prm.add(3);
        let lmax = *prm.add(5);
        let fpmax = *prm.add(7);

        // scale force if negative
        if force < 0.0 {
            force = scale / f64::max(MJ_MINVAL, acc0);
        }

        // optimum length
        let L0 = (*lengthrange.add(1) - *lengthrange.add(0)) / f64::max(MJ_MINVAL, range1 - range0);

        // normalized length
        let L = range0 + (len - *lengthrange.add(0)) / f64::max(MJ_MINVAL, L0);

        // half-quadratic to (L0+lmax)/2, linear beyond
        let b = 0.5 * (1.0 + lmax);
        if L <= 1.0 {
            0.0
        } else if L <= b {
            let x = (L - 1.0) / f64::max(MJ_MINVAL, b - 1.0);
            -force * fpmax * 0.5 * x * x
        } else {
            let x = (L - b) / f64::max(MJ_MINVAL, b - 1.0);
            -force * fpmax * (0.5 + x)
        }
    }
}

/// C: mju_muscleDynamicsTimescale (engine/engine_util_misc.h:47)
/// Calls: mju_sigmoid
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_dynamics_timescale(dctrl: f64, tau_act: f64, tau_deact: f64, smoothing_width: f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // hard switching
    if smoothing_width < MJ_MINVAL {
        if dctrl > 0.0 { tau_act } else { tau_deact }
    }
    // smooth switching
    else {
        tau_deact + (tau_act - tau_deact) * mju_sigmoid(dctrl / smoothing_width + 0.5)
    }
}

/// C: mju_muscleDynamics (engine/engine_util_misc.h:51)
/// Calls: mju_clip, mju_muscleDynamicsTimescale
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_dynamics(ctrl: f64, act: f64, prm: *const f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees prm points to at least 3 f64
    unsafe {
        // clamp control
        let ctrlclamp = mju_clip(ctrl, 0.0, 1.0);

        // clamp activation
        let actclamp = mju_clip(act, 0.0, 1.0);

        // compute timescales as in Millard et al. (2013)
        let tau_act = *prm.add(0) * (0.5 + 1.5 * actclamp);
        let tau_deact = *prm.add(1) / (0.5 + 1.5 * actclamp);
        let smoothing_width = *prm.add(2);
        let dctrl = ctrlclamp - act;

        let tau = mju_muscle_dynamics_timescale(dctrl, tau_act, tau_deact, smoothing_width);

        // filter output
        dctrl / f64::max(MJ_MINVAL, tau)
    }
}

/// C: mj_lugreStribeck (engine/engine_util_misc.h:54)
/// Calls: mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_lugre_stribeck(velocity: f64, F_C: f64, F_S: f64, v_S: f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: pure math, no pointer dereference
    let ratio: f64 = velocity / f64::max(MJ_MINVAL, v_S);
    F_C + (F_S - F_C) * f64::exp(-ratio * ratio)
}

/// C: mj_dcmotorSlots (engine/engine_util_misc.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_dcmotor_slots(dynprm: *const f64, gainprm: *const f64) -> mjDCMotorSlots {
    // SAFETY: dynprm and gainprm point to valid parameter arrays (caller contract)
    unsafe {
        let mut s = mjDCMotorSlots {
            slew: -1,
            integral: -1,
            temperature: -1,
            bristle: -1,
            current: -1,
            num_slots: 0,
        };
        if *dynprm.add(7) > 0.0 { s.slew = s.num_slots; s.num_slots += 1; }        // slew rate limiting
        if *gainprm.add(5) > 0.0 { s.integral = s.num_slots; s.num_slots += 1; }    // PI integral
        if *dynprm.add(2) > 0.0 { s.temperature = s.num_slots; s.num_slots += 1; }  // thermal model
        if *dynprm.add(5) > 0.0 { s.bristle = s.num_slots; s.num_slots += 1; }      // LuGre bristle
        if *dynprm.add(0) > 0.0 { s.current = s.num_slots; s.num_slots += 1; }      // current filter

        s
    }
}

/// C: mju_geomSemiAxes (engine/engine_util_misc.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_geom_semi_axes(semiaxes: *mut f64, size: *const f64, r#type: u32) {
    const GEOM_SPHERE: u32 = 2;
    const GEOM_CAPSULE: u32 = 3;
    const GEOM_CYLINDER: u32 = 5;

    // SAFETY: semiaxes and size point to valid f64 arrays of length >= 3 per caller contract
    unsafe {
        match r#type {
            GEOM_SPHERE => {
                *semiaxes.add(0) = *size.add(0);
                *semiaxes.add(1) = *size.add(0);
                *semiaxes.add(2) = *size.add(0);
            }
            GEOM_CAPSULE => {
                *semiaxes.add(0) = *size.add(0);
                *semiaxes.add(1) = *size.add(0);
                *semiaxes.add(2) = *size.add(1) + *size.add(0);
            }
            GEOM_CYLINDER => {
                *semiaxes.add(0) = *size.add(0);
                *semiaxes.add(1) = *size.add(0);
                *semiaxes.add(2) = *size.add(1);
            }
            _ => {
                *semiaxes.add(0) = *size.add(0);
                *semiaxes.add(1) = *size.add(1);
                *semiaxes.add(2) = *size.add(2);
            }
        }
    }
}

/// C: mju_insideGeom (engine/engine_util_misc.h:74)
/// Calls: mju_clip, mju_dot3, mju_mulMatTVec3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_inside_geom(pos: *const f64, mat: *const f64, size: *const f64, r#type: u32, point: *const f64) -> i32 {
    use crate::engine::engine_util_blas::{mju_sub3, mju_dot3, mju_mul_mat_t_vec3};

    const GEOM_PLANE: u32 = 0;
    const GEOM_SPHERE: u32 = 2;
    const GEOM_CAPSULE: u32 = 3;
    const GEOM_ELLIPSOID: u32 = 4;
    const GEOM_CYLINDER: u32 = 5;
    const GEOM_BOX: u32 = 6;

    // SAFETY: pos, mat, size, point all point to valid memory per caller contract
    unsafe {
        // vector from geom to point
        let mut vec: [f64; 3] = [0.0; 3];
        mju_sub3(vec.as_mut_ptr(), point, pos);

        // quick return for spheres
        if r#type == GEOM_SPHERE {
            return if mju_dot3(vec.as_ptr(), vec.as_ptr()) < *size.add(0) * *size.add(0) { 1 } else { 0 };
        }

        // rotate into local frame
        let mut plocal: [f64; 3] = [0.0; 3];
        mju_mul_mat_t_vec3(plocal.as_mut_ptr(), mat, vec.as_ptr());

        match r#type {
            GEOM_CAPSULE => {
                let z = plocal[2];
                let z_clamped = mju_clip(z, -*size.add(1), *size.add(1));
                let z_dist_sq = (z - z_clamped) * (z - z_clamped);
                if plocal[0] * plocal[0] + plocal[1] * plocal[1] + z_dist_sq < *size.add(0) * *size.add(0) { 1 } else { 0 }
            }
            GEOM_ELLIPSOID => {
                if plocal[0] * plocal[0] / (*size.add(0) * *size.add(0))
                    + plocal[1] * plocal[1] / (*size.add(1) * *size.add(1))
                    + plocal[2] * plocal[2] / (*size.add(2) * *size.add(2)) < 1.0 { 1 } else { 0 }
            }
            GEOM_CYLINDER => {
                if f64::abs(plocal[2]) < *size.add(1)
                    && plocal[0] * plocal[0] + plocal[1] * plocal[1] < *size.add(0) * *size.add(0) { 1 } else { 0 }
            }
            GEOM_BOX => {
                if f64::abs(plocal[0]) < *size.add(0)
                    && f64::abs(plocal[1]) < *size.add(1)
                    && f64::abs(plocal[2]) < *size.add(2) { 1 } else { 0 }
            }
            GEOM_PLANE => {
                if plocal[2] < 0.0 { 1 } else { 0 }
            }
            _ => 0
        }
    }
}

/// C: mju_camPixelRay (engine/engine_util_misc.h:79)
/// Calls: mju_add3, mju_copy3, mju_mulMatVec3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_pixel_ray(origin: *mut f64, direction: *mut f64, cam_xpos: *const f64, cam_xmat: *const f64, col: i32, row: i32, fx: f64, fy: f64, cx: f64, cy: f64, projection: i32, ortho_extent: f64) {
    // SAFETY: caller guarantees origin[3], direction[3], cam_xpos[3], cam_xmat[9] are valid
    unsafe {
        // pixel center (row 0 = top of image)
        let px: f64 = col as f64 + 0.5 - cx;
        let py: f64 = row as f64 + 0.5 - cy;

        if projection == 0 {
            // perspective: origin is camera position
            crate::engine::engine_util_blas::mju_copy3(origin, cam_xpos);

            // direction in camera frame: (x/fx, -y/fy, -1), then normalized
            let mut dir_cam: [f64; 3] = [px / fx, -py / fy, -1.0];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(direction, cam_xmat, dir_cam.as_ptr());
            crate::engine::engine_util_blas::mju_normalize3(direction);
        } else {
            // orthographic: parallel rays, direction is -Z in camera frame
            *direction.add(0) = -*cam_xmat.add(2);
            *direction.add(1) = -*cam_xmat.add(5);
            *direction.add(2) = -*cam_xmat.add(8);

            // origin offset in camera frame
            let half_extent: f64 = ortho_extent / 2.0;
            let mut offset_cam: [f64; 3] = [px / fx * half_extent, -py / fy * half_extent, 0.0];
            let mut offset_world: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(
                offset_world.as_mut_ptr(), cam_xmat, offset_cam.as_ptr(),
            );
            crate::engine::engine_util_blas::mju_add3(origin, cam_xpos, offset_world.as_ptr());
        }
    }
}

/// C: mju_defGradient (engine/engine_util_misc.h:87)
/// Calls: mju_flexDphi, mju_flexPhi, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_def_gradient(res: *mut f64, p: *const f64, dof: *const f64, order: i32) {
    // SAFETY: caller guarantees res[9], p[3], dof[3*(order+1)^3] are valid
    unsafe {
        crate::engine::engine_util_blas::mju_zero(res, 9);
        let mut idx: usize = 0;
        let mut gradient: [f64; 3] = [0.0; 3];
        for i in 0..=order {
            for j in 0..=order {
                for k in 0..=order {
                    gradient[0] = mju_flex_dphi(*p.add(0), i, order)
                                * mju_flex_phi(*p.add(1), j, order)
                                * mju_flex_phi(*p.add(2), k, order);
                    gradient[1] = mju_flex_phi(*p.add(0), i, order)
                                * mju_flex_dphi(*p.add(1), j, order)
                                * mju_flex_phi(*p.add(2), k, order);
                    gradient[2] = mju_flex_phi(*p.add(0), i, order)
                                * mju_flex_phi(*p.add(1), j, order)
                                * mju_flex_dphi(*p.add(2), k, order);
                    *res.add(0) += *dof.add(3 * idx + 0) * gradient[0];
                    *res.add(1) += *dof.add(3 * idx + 0) * gradient[1];
                    *res.add(2) += *dof.add(3 * idx + 0) * gradient[2];
                    *res.add(3) += *dof.add(3 * idx + 1) * gradient[0];
                    *res.add(4) += *dof.add(3 * idx + 1) * gradient[1];
                    *res.add(5) += *dof.add(3 * idx + 1) * gradient[2];
                    *res.add(6) += *dof.add(3 * idx + 2) * gradient[0];
                    *res.add(7) += *dof.add(3 * idx + 2) * gradient[1];
                    *res.add(8) += *dof.add(3 * idx + 2) * gradient[2];
                    idx += 1;
                }
            }
        }
    }
}

/// C: mju_evalBasis (engine/engine_util_misc.h:90)
/// Calls: mju_flexPhi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eval_basis(x: *const f64, i: i32, order: i32) -> f64 {
    // SAFETY: caller guarantees x points to at least 3 f64
    unsafe {
        if order == 1 {
            return mju_flex_phi(*x.add(2), i & 1, order)
                 * mju_flex_phi(*x.add(1), (i >> 1) & 1, order)
                 * mju_flex_phi(*x.add(0), (i >> 2) & 1, order);
        } else if order == 2 {
            return mju_flex_phi(*x.add(2), i % 3, order)
                 * mju_flex_phi(*x.add(1), (i / 3) % 3, order)
                 * mju_flex_phi(*x.add(0), i / 9, order);
        } else {
            return -1.0;
        }
    }
}

/// C: mju_evalBasisArray (engine/engine_util_misc.h:93)
/// Calls: mju_evalBasis, mju_flexPhi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eval_basis_array(basis: *mut f64, x: *const f64, order: i32) {
    // SAFETY: caller guarantees x[3] valid, basis has space for (order+1)^3 elements
    unsafe {
        if order == 1 {
            let p: [[f64; 2]; 3] = [
                [1.0 - *x.add(0), *x.add(0)],
                [1.0 - *x.add(1), *x.add(1)],
                [1.0 - *x.add(2), *x.add(2)],
            ];
            let mut j: usize = 0;
            for i0 in 0..2 {
                let w0 = p[0][i0];
                for i1 in 0..2 {
                    let w01 = w0 * p[1][i1];
                    for i2 in 0..2 {
                        *basis.add(j) = w01 * p[2][i2];
                        j += 1;
                    }
                }
            }
        } else if order == 2 {
            let mut p: [[f64; 3]; 3] = [[0.0; 3]; 3];
            for d in 0..3 {
                for i in 0..3 {
                    p[d][i] = mju_flex_phi(*x.add(d), i as i32, 2);
                }
            }
            let mut j: usize = 0;
            for i0 in 0..3 {
                let w0 = p[0][i0];
                for i1 in 0..3 {
                    let w01 = w0 * p[1][i1];
                    for i2 in 0..3 {
                        *basis.add(j) = w01 * p[2][i2];
                        j += 1;
                    }
                }
            }
        }
    }
}

/// C: mju_cellLookup (engine/engine_util_misc.h:96)
/// Calls: mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cell_lookup(coord: *const f64, cellnum: *const i32, order: i32, local: *mut f64, nodeindices: *mut i32) -> i32 {
    // SAFETY: caller guarantees coord[3], cellnum[3], local[3] are valid; nodeindices may be null
    unsafe {
        let cx: i32 = *cellnum.add(0);
        let cy: i32 = *cellnum.add(1);
        let cz: i32 = *cellnum.add(2);

        // find containing cell
        let mut ci: i32 = (*coord.add(0) * cx as f64).floor() as i32;
        let mut cj: i32 = (*coord.add(1) * cy as f64).floor() as i32;
        let mut ck: i32 = (*coord.add(2) * cz as f64).floor() as i32;
        ci = if ci < cx - 1 { ci } else { cx - 1 }; ci = if ci > 0 { ci } else { 0 };
        cj = if cj < cy - 1 { cj } else { cy - 1 }; cj = if cj > 0 { cj } else { 0 };
        ck = if ck < cz - 1 { ck } else { cz - 1 }; ck = if ck > 0 { ck } else { 0 };

        // local parametric coordinates within cell
        *local.add(0) = mju_clip(*coord.add(0) * cx as f64 - ci as f64, 0.0, 1.0);
        *local.add(1) = mju_clip(*coord.add(1) * cy as f64 - cj as f64, 0.0, 1.0);
        *local.add(2) = mju_clip(*coord.add(2) * cz as f64 - ck as f64, 0.0, 1.0);

        // build node indices for this cell
        if !nodeindices.is_null() {
            let gi_base: i32 = ci * order;
            let gj_base: i32 = cj * order;
            let gk_base: i32 = ck * order;
            let ny_g: i32 = cy * order + 1;
            let nz_g: i32 = cz * order + 1;
            let mut ni: usize = 0;
            for li in 0..=order {
                let gi: i32 = gi_base + li;
                let gi_stride: i32 = gi * ny_g * nz_g;
                for lj in 0..=order {
                    let gj: i32 = gj_base + lj;
                    let gj_stride: i32 = gi_stride + gj * nz_g;
                    for lk in 0..=order {
                        let gk: i32 = gk_base + lk;
                        *nodeindices.add(ni) = gj_stride + gk;
                        ni += 1;
                    }
                }
            }
        }

        let npc: i32 = (order + 1) * (order + 1) * (order + 1);
        npc
    }
}

/// C: mju_interpolate3D (engine/engine_util_misc.h:100)
/// Calls: mju_addToScl3, mju_evalBasis, mju_evalBasisArray
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_interpolate3d(res: *mut f64, x: *const f64, coeff: *const f64, order: i32, nodeindices: *const i32) {
    use crate::engine::engine_util_blas::mju_add_to_scl3;

    let npoint: i32 = (order + 1) * (order + 1) * (order + 1);

    // SAFETY: caller guarantees res[3], x[3], coeff[3*N], nodeindices[npoint] are valid
    unsafe {
        if npoint > 27 {
            for j in 0..npoint {
                let idx = if !nodeindices.is_null() { *nodeindices.add(j as usize) } else { j };
                mju_add_to_scl3(res, coeff.add(3 * idx as usize), mju_eval_basis(x, j, order));
            }
            return;
        }

        let mut basis: [f64; 27] = [0.0; 27];
        mju_eval_basis_array(basis.as_mut_ptr(), x, order);

        for j in 0..npoint {
            let idx = if !nodeindices.is_null() { *nodeindices.add(j as usize) } else { j };
            mju_add_to_scl3(res, coeff.add(3 * idx as usize), basis[j as usize]);
        }
    }
}

/// C: mju_flexGatherCellState (engine/engine_util_misc.h:104)
/// Calls: flexInterpRotation, mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_cell_state(order: i32, cy: i32, cz: i32, ci: i32, cj: i32, ck: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_c: *mut f64, vel_c: *mut f64, xpos0_c: *mut f64, nodeindices: *mut i32, quat: *mut f64) {
    use crate::engine::engine_util_blas::mju_copy3;

    // SAFETY: caller guarantees all non-null pointers have sufficient extent
    unsafe {
        let ny_g: i32 = cy * order + 1;
        let nz_g: i32 = cz * order + 1;

        let mut local_idx: usize = 0;
        for li in 0..=order {
            for lj in 0..=order {
                for lk in 0..=order {
                    let gi: i32 = ci * order + li;
                    let gj: i32 = cj * order + lj;
                    let gk: i32 = ck * order + lk;
                    let gidx: i32 = gi * ny_g * nz_g + gj * nz_g + gk;

                    if !xpos_c.is_null() && !xpos_g.is_null() {
                        mju_copy3(xpos_c.add(3 * local_idx), xpos_g.add(3 * gidx as usize));
                    }
                    if !vel_c.is_null() && !vel_g.is_null() {
                        mju_copy3(vel_c.add(3 * local_idx), vel_g.add(3 * gidx as usize));
                    }
                    if !xpos0_c.is_null() && !xpos0_g.is_null() {
                        mju_copy3(xpos0_c.add(3 * local_idx), xpos0_g.add(3 * gidx as usize));
                    }
                    if !nodeindices.is_null() {
                        *nodeindices.add(local_idx) = gidx;
                    }

                    local_idx += 1;
                }
            }
        }

        if !quat.is_null() && !xpos_c.is_null() {
            let p: [f64; 3] = [0.5, 0.5, 0.5];
            flex_interp_rotation(order, xpos_c, p.as_ptr(), quat);
        }
    }
}

/// C: mju_flexGatherFaceState (engine/engine_util_misc.h:110)
/// Calls: mju_copy3, mju_flexInterpRotation2D
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_face_state(order: i32, cx: i32, cy: i32, cz: i32, face_elem_idx: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_f: *mut f64, vel_f: *mut f64, xpos0_f: *mut f64, nodeindices: *mut i32, quat: *mut f64) {
    use crate::engine::engine_util_blas::mju_copy3;

    // SAFETY: caller guarantees all non-null pointers have sufficient extent
    unsafe {
        let ny_g: i32 = cy * order + 1;
        let nz_g: i32 = cz * order + 1;
        let npe: i32 = (order + 1) * (order + 1);

        // face sizes and properties
        let face_sizes: [i32; 6] = [cy * cz, cy * cz, cx * cz, cx * cz, cx * cy, cx * cy];
        let face_normal: [i32; 6] = [0, 0, 1, 1, 2, 2];
        let face_count1: [i32; 6] = [cz, cz, cx, cx, cy, cy];
        let face_fixed_vals: [i32; 6] = [0, cx * order, 0, cy * order, 0, cz * order];

        // determine which face and quad within face
        let mut face_id: usize = 0;
        let mut within_face: i32 = face_elem_idx;
        let mut cumul: i32 = 0;
        for f in 0..6 {
            if face_elem_idx < cumul + face_sizes[f] {
                face_id = f;
                within_face = face_elem_idx - cumul;
                break;
            }
            cumul += face_sizes[f];
        }

        let normal_axis: i32 = face_normal[face_id];
        let na0: i32 = (normal_axis + 1) % 3;  // slow in-plane axis
        let na1: i32 = (normal_axis + 2) % 3;  // fast in-plane axis
        let c1: i32 = face_count1[face_id];
        let g_fixed: i32 = face_fixed_vals[face_id];
        let q0: i32 = within_face / c1;
        let q1: i32 = within_face % c1;

        // gather nodes
        let mut local_idx: usize = 0;
        for l0 in 0..=order {
            for l1 in 0..=order {
                let mut g: [i32; 3] = [0; 3];
                g[normal_axis as usize] = g_fixed;
                g[na0 as usize] = q0 * order + l0;
                g[na1 as usize] = q1 * order + l1;
                let gidx: i32 = g[0] * ny_g * nz_g + g[1] * nz_g + g[2];

                if !xpos_f.is_null() && !xpos_g.is_null() {
                    mju_copy3(xpos_f.add(3 * local_idx), xpos_g.add(3 * gidx as usize));
                }
                if !vel_f.is_null() && !vel_g.is_null() {
                    mju_copy3(vel_f.add(3 * local_idx), vel_g.add(3 * gidx as usize));
                }
                if !xpos0_f.is_null() && !xpos0_g.is_null() {
                    mju_copy3(xpos0_f.add(3 * local_idx), xpos0_g.add(3 * gidx as usize));
                }
                if !nodeindices.is_null() {
                    *nodeindices.add(local_idx) = gidx;
                }

                local_idx += 1;
            }
        }

        if !quat.is_null() && !xpos_f.is_null() {
            let p: [f64; 2] = [0.5, 0.5];
            mju_flex_interp_rotation2d(order, xpos_f, npe, na0, na1, normal_axis, p.as_ptr(), quat);
        }
    }
}

/// C: mju_flexInterpRotation2D (engine/engine_util_misc.h:118)
/// Calls: mju_cross, mju_flexDphi, mju_flexPhi, mju_mat2Rot, mju_negQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_interp_rotation2d(order: i32, xpos_f: *const f64, npe: i32, axis0: i32, axis1: i32, normal_axis: i32, local: *const f64, quat: *mut f64) {
    use crate::engine::engine_util_spatial::{mju_cross, mju_mat2rot, mju_neg_quat};

    // SAFETY: caller guarantees xpos_f[3*npe], local[2], quat[4] are valid
    unsafe {
        // compute 3x2 deformation gradient F at parametric point local
        let mut t1: [f64; 3] = [0.0, 0.0, 0.0];
        let mut t2: [f64; 3] = [0.0, 0.0, 0.0];
        let mut idx: usize = 0;
        for l0 in 0..=order {
            for l1 in 0..=order {
                let grad0: f64 = mju_flex_dphi(*local.add(0), l0, order)
                               * mju_flex_phi(*local.add(1), l1, order);
                let grad1: f64 = mju_flex_phi(*local.add(0), l0, order)
                               * mju_flex_dphi(*local.add(1), l1, order);
                for d in 0..3 {
                    t1[d] += *xpos_f.add(3 * idx + d) * grad0;
                    t2[d] += *xpos_f.add(3 * idx + d) * grad1;
                }
                idx += 1;
            }
        }

        // normal = t1 x t2
        let mut normal: [f64; 3] = [0.0; 3];
        mju_cross(normal.as_mut_ptr(), t1.as_ptr(), t2.as_ptr());

        // build 3x3 matrix with columns assigned to canonical axes (row-major)
        let mut vecs: [*const f64; 3] = [std::ptr::null(); 3];
        vecs[axis0 as usize] = t1.as_ptr();
        vecs[axis1 as usize] = t2.as_ptr();
        vecs[normal_axis as usize] = normal.as_ptr();

        let mut mat: [f64; 9] = [0.0; 9];
        for col in 0..3 {
            mat[0 * 3 + col] = *vecs[col].add(0);
            mat[1 * 3 + col] = *vecs[col].add(1);
            mat[2 * 3 + col] = *vecs[col].add(2);
        }

        // extract rotation via polar decomposition
        *quat.add(0) = 1.0;
        *quat.add(1) = 0.0;
        *quat.add(2) = 0.0;
        *quat.add(3) = 0.0;
        mju_mat2rot(quat, mat.as_ptr());
        mju_neg_quat(quat, quat);
    }
}

/// C: mju_flexFaceNormal2D (engine/engine_util_misc.h:124)
/// Calls: mju_cross, mju_flexDphi, mju_flexPhi, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_face_normal2d(normal: *mut f64, t1: *mut f64, t2: *mut f64, order: i32, xpos_f: *const f64, local: *const f64) {
    use crate::engine::engine_util_blas::mju_zero3;
    use crate::engine::engine_util_spatial::mju_cross;

    // SAFETY: caller guarantees normal[3], t1[3], t2[3], xpos_f[3*npe], local[2] are valid
    unsafe {
        mju_zero3(t1);
        mju_zero3(t2);
        let mut idx: usize = 0;
        for l0 in 0..=order {
            for l1 in 0..=order {
                let grad0: f64 = mju_flex_dphi(*local.add(0), l0, order)
                               * mju_flex_phi(*local.add(1), l1, order);
                let grad1: f64 = mju_flex_phi(*local.add(0), l0, order)
                               * mju_flex_dphi(*local.add(1), l1, order);
                for d in 0..3 {
                    *t1.add(d) += *xpos_f.add(3 * idx + d) * grad0;
                    *t2.add(d) += *xpos_f.add(3 * idx + d) * grad1;
                }
                idx += 1;
            }
        }
        mju_cross(normal, t1, t2);
    }
}

/// C: mju_flexPhi (engine/engine_util_misc.h:130)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_phi(s: f64, i: i32, order: i32) -> f64 {
    if order == 1 {
        return if i == 0 { 1.0 - s } else { s };
    }
    match i {
        0 => 2.0 * s * s - 3.0 * s + 1.0,
        1 => 4.0 * (s - s * s),
        2 => 2.0 * s * s - s,
        _ => 0.0,
    }
}

/// C: mju_flexDphi (engine/engine_util_misc.h:141)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_dphi(s: f64, i: i32, order: i32) -> f64 {
    if order == 1 {
        return if i == 0 { -1.0 } else { 1.0 };
    }
    match i {
        0 => 4.0 * s - 3.0,
        1 => 4.0 * (1.0 - 2.0 * s),
        2 => 4.0 * s - 1.0,
        _ => 0.0,
    }
}

/// C: mju_shellTrackInterior (engine/engine_util_misc.h:151)
/// Calls: mju_copy3, nodeAt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_shell_track_interior(nodexpos: *mut f64, nx: i32, ny: i32, nz: i32) {
    // need at least 3 nodes in each direction to have interior nodes
    if nx < 3 || ny < 3 || nz < 3 {
        return;
    }

    // SAFETY: caller guarantees nodexpos has dimensions [nx*ny*nz][3]
    unsafe {
        for i in 1..nx - 1 {
            for j in 1..ny - 1 {
                for k in 1..nz - 1 {
                    // parametric coordinates in [0, 1]
                    let s: f64 = i as f64 / (nx - 1) as f64;
                    let t: f64 = j as f64 / (ny - 1) as f64;
                    let u: f64 = k as f64 / (nz - 1) as f64;

                    let mut result: [f64; 3] = [0.0, 0.0, 0.0];

                    // --- face contributions (bilinear interpolation on each face pair) ---
                    // x-faces: i=0 and i=nx-1
                    for d in 0..3 {
                        *result.as_mut_ptr().add(d) += (1.0 - s) * *node_at(nodexpos, ny, nz, 0, j, k).add(d)
                                                     + s * *node_at(nodexpos, ny, nz, nx - 1, j, k).add(d);
                    }
                    // y-faces: j=0 and j=ny-1
                    for d in 0..3 {
                        *result.as_mut_ptr().add(d) += (1.0 - t) * *node_at(nodexpos, ny, nz, i, 0, k).add(d)
                                                     + t * *node_at(nodexpos, ny, nz, i, ny - 1, k).add(d);
                    }
                    // z-faces: k=0 and k=nz-1
                    for d in 0..3 {
                        *result.as_mut_ptr().add(d) += (1.0 - u) * *node_at(nodexpos, ny, nz, i, j, 0).add(d)
                                                     + u * *node_at(nodexpos, ny, nz, i, j, nz - 1).add(d);
                    }

                    // --- edge corrections (subtract 12 edges, each linearly interpolated) ---
                    // edges along x (4 edges: (j,k) at corners of y-z face)
                    for d in 0..3 {
                        result[d] -= (1.0 - t) * (1.0 - u) * *node_at(nodexpos, ny, nz, i, 0, 0).add(d);
                        result[d] -= (1.0 - t) * u * *node_at(nodexpos, ny, nz, i, 0, nz - 1).add(d);
                        result[d] -= t * (1.0 - u) * *node_at(nodexpos, ny, nz, i, ny - 1, 0).add(d);
                        result[d] -= t * u * *node_at(nodexpos, ny, nz, i, ny - 1, nz - 1).add(d);
                    }
                    // edges along y (4 edges: (i,k) at corners of x-z face)
                    for d in 0..3 {
                        result[d] -= (1.0 - s) * (1.0 - u) * *node_at(nodexpos, ny, nz, 0, j, 0).add(d);
                        result[d] -= (1.0 - s) * u * *node_at(nodexpos, ny, nz, 0, j, nz - 1).add(d);
                        result[d] -= s * (1.0 - u) * *node_at(nodexpos, ny, nz, nx - 1, j, 0).add(d);
                        result[d] -= s * u * *node_at(nodexpos, ny, nz, nx - 1, j, nz - 1).add(d);
                    }
                    // edges along z (4 edges: (i,j) at corners of x-y face)
                    for d in 0..3 {
                        result[d] -= (1.0 - s) * (1.0 - t) * *node_at(nodexpos, ny, nz, 0, 0, k).add(d);
                        result[d] -= (1.0 - s) * t * *node_at(nodexpos, ny, nz, 0, ny - 1, k).add(d);
                        result[d] -= s * (1.0 - t) * *node_at(nodexpos, ny, nz, nx - 1, 0, k).add(d);
                        result[d] -= s * t * *node_at(nodexpos, ny, nz, nx - 1, ny - 1, k).add(d);
                    }

                    // --- corner corrections (add 8 corners back) ---
                    for d in 0..3 {
                        result[d] += (1.0 - s) * (1.0 - t) * (1.0 - u) * *node_at(nodexpos, ny, nz, 0, 0, 0).add(d);
                        result[d] += (1.0 - s) * (1.0 - t) * u * *node_at(nodexpos, ny, nz, 0, 0, nz - 1).add(d);
                        result[d] += (1.0 - s) * t * (1.0 - u) * *node_at(nodexpos, ny, nz, 0, ny - 1, 0).add(d);
                        result[d] += (1.0 - s) * t * u * *node_at(nodexpos, ny, nz, 0, ny - 1, nz - 1).add(d);
                        result[d] += s * (1.0 - t) * (1.0 - u) * *node_at(nodexpos, ny, nz, nx - 1, 0, 0).add(d);
                        result[d] += s * (1.0 - t) * u * *node_at(nodexpos, ny, nz, nx - 1, 0, nz - 1).add(d);
                        result[d] += s * t * (1.0 - u) * *node_at(nodexpos, ny, nz, nx - 1, ny - 1, 0).add(d);
                        result[d] += s * t * u * *node_at(nodexpos, ny, nz, nx - 1, ny - 1, nz - 1).add(d);
                    }

                    // write result to interior node
                    crate::engine::engine_util_blas::mju_copy3(
                        nodexpos.add(3 * (i as usize * ny as usize * nz as usize + j as usize * nz as usize + k as usize)),
                        result.as_ptr(),
                    );
                }
            }
        }
    }
}

/// C: mju_shellTFIWeights (engine/engine_util_misc.h:154)
/// Calls: addWeight
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_shell_tfi_weights(nx: i32, ny: i32, nz: i32, i: i32, j: i32, k: i32, w: f64, nb: *mut i32, body: *mut i32, bweight: *mut f64, nodebodyid: *const i32, nstart: i32) {
    // SAFETY: caller guarantees all pointers valid, nodebodyid indexed by nstart + grid offsets
    unsafe {
        let s: f64 = i as f64 / (nx - 1) as f64;
        let t: f64 = j as f64 / (ny - 1) as f64;
        let u: f64 = k as f64 / (nz - 1) as f64;

        let nstart = nstart as usize;
        let ny_uz = ny as usize;
        let nz_uz = nz as usize;
        let i_uz = i as usize;
        let j_uz = j as usize;
        let k_uz = k as usize;
        let nx1 = (nx - 1) as usize;
        let ny1 = (ny - 1) as usize;
        let nz1 = (nz - 1) as usize;

        // face contributions
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + 0 * ny_uz * nz_uz + j_uz * nz_uz + k_uz), w * (1.0 - s));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + nx1 * ny_uz * nz_uz + j_uz * nz_uz + k_uz), w * s);

        add_weight(nb, body, bweight, *nodebodyid.add(nstart + i_uz * ny_uz * nz_uz + 0 * nz_uz + k_uz), w * (1.0 - t));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + i_uz * ny_uz * nz_uz + ny1 * nz_uz + k_uz), w * t);

        add_weight(nb, body, bweight, *nodebodyid.add(nstart + i_uz * ny_uz * nz_uz + j_uz * nz_uz + 0), w * (1.0 - u));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + i_uz * ny_uz * nz_uz + j_uz * nz_uz + nz1), w * u);

        // edge corrections
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + i_uz * ny_uz * nz_uz + 0 * nz_uz + 0), -w * (1.0 - t) * (1.0 - u));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + i_uz * ny_uz * nz_uz + 0 * nz_uz + nz1), -w * (1.0 - t) * u);
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + i_uz * ny_uz * nz_uz + ny1 * nz_uz + 0), -w * t * (1.0 - u));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + i_uz * ny_uz * nz_uz + ny1 * nz_uz + nz1), -w * t * u);

        add_weight(nb, body, bweight, *nodebodyid.add(nstart + 0 * ny_uz * nz_uz + j_uz * nz_uz + 0), -w * (1.0 - s) * (1.0 - u));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + 0 * ny_uz * nz_uz + j_uz * nz_uz + nz1), -w * (1.0 - s) * u);
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + nx1 * ny_uz * nz_uz + j_uz * nz_uz + 0), -w * s * (1.0 - u));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + nx1 * ny_uz * nz_uz + j_uz * nz_uz + nz1), -w * s * u);

        add_weight(nb, body, bweight, *nodebodyid.add(nstart + 0 * ny_uz * nz_uz + 0 * nz_uz + k_uz), -w * (1.0 - s) * (1.0 - t));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + 0 * ny_uz * nz_uz + ny1 * nz_uz + k_uz), -w * (1.0 - s) * t);
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + nx1 * ny_uz * nz_uz + 0 * nz_uz + k_uz), -w * s * (1.0 - t));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + nx1 * ny_uz * nz_uz + ny1 * nz_uz + k_uz), -w * s * t);

        // corner corrections
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + 0 * ny_uz * nz_uz + 0 * nz_uz + 0), w * (1.0 - s) * (1.0 - t) * (1.0 - u));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + 0 * ny_uz * nz_uz + 0 * nz_uz + nz1), w * (1.0 - s) * (1.0 - t) * u);
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + 0 * ny_uz * nz_uz + ny1 * nz_uz + 0), w * (1.0 - s) * t * (1.0 - u));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + 0 * ny_uz * nz_uz + ny1 * nz_uz + nz1), w * (1.0 - s) * t * u);
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + nx1 * ny_uz * nz_uz + 0 * nz_uz + 0), w * s * (1.0 - t) * (1.0 - u));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + nx1 * ny_uz * nz_uz + 0 * nz_uz + nz1), w * s * (1.0 - t) * u);
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + nx1 * ny_uz * nz_uz + ny1 * nz_uz + 0), w * s * t * (1.0 - u));
        add_weight(nb, body, bweight, *nodebodyid.add(nstart + nx1 * ny_uz * nz_uz + ny1 * nz_uz + nz1), w * s * t * u);
    }
}

/// C: mju_encodeBase64 (engine/engine_util_misc.h:163)
/// Calls: next
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_base64(buf: *mut i8, data: *const u8, ndata: usize) -> usize {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // SAFETY: buf points to a buffer large enough for base64 output, data points to ndata bytes
    unsafe {
        let mut i: usize = 0;
        let mut j: usize = 0;

        // loop over 24 bit chunks
        while i + 3 <= ndata {
            let byte_1 = *data.add(i) as u32; i += 1;
            let byte_2 = *data.add(i) as u32; i += 1;
            let byte_3 = *data.add(i) as u32; i += 1;

            let k = (byte_1 << 16) | (byte_2 << 8) | byte_3;

            *buf.add(j) = TABLE[((k >> 18) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 12) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >>  6) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >>  0) & 63) as usize] as i8; j += 1;
        }

        // one byte left
        if i + 1 == ndata {
            let byte_1 = *data.add(i) as u32;
            let k = byte_1 << 16;
            *buf.add(j) = TABLE[((k >> 18) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 12) & 63) as usize] as i8; j += 1;
            *buf.add(j) = b'=' as i8; j += 1;
            *buf.add(j) = b'=' as i8; j += 1;
        }

        // two bytes left
        if i + 2 == ndata {
            let byte_1 = *data.add(i) as u32; i += 1;
            let byte_2 = *data.add(i) as u32;

            let k = (byte_1 << 16) + (byte_2 << 8);

            *buf.add(j) = TABLE[((k >> 18) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 12) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >>  6) & 63) as usize] as i8; j += 1;
            *buf.add(j) = b'=' as i8; j += 1;
        }

        *buf.add(j) = 0; // null terminator
        4 * ((ndata + 2) / 3) + 1
    }
}

/// C: mju_isValidBase64 (engine/engine_util_misc.h:167)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_valid_base64(s: *const i8) -> usize {
    // SAFETY: caller guarantees s is a valid null-terminated string
    unsafe {
        let mut i: usize = 0;
        let mut pad: i32 = 0;

        // validate chars
        while *s.add(i) != 0 && *s.add(i) != b'=' as i8 {
            let c = *s.add(i) as u8;
            if !c.is_ascii_alphanumeric() && c != b'/' && c != b'+' {
                return 0;
            }
            i += 1;
        }

        // padding at end
        if *s.add(i) == b'=' as i8 {
            if *s.add(i + 1) == 0 {
                pad = 1;
            } else if *s.add(i + 1) == b'=' as i8 && *s.add(i + 2) == 0 {
                pad = 2;
            } else {
                return 0;
            }
        }

        // strlen(s) must be a multiple of 4
        let len: i32 = i as i32 + pad;
        if len % 4 != 0 {
            return 0;
        }
        (3 * (len / 4) - pad) as usize
    }
}

/// C: mju_decodeBase64 (engine/engine_util_misc.h:171)
/// Calls: _decode
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_base64(buf: *mut u8, s: *const i8) -> usize {
    // SAFETY: caller guarantees buf has enough space and s is a valid null-terminated base64 string
    unsafe {
        // inline base64 char decoder
        let decode = |ch: u8| -> u32 {
            if ch >= b'A' && ch <= b'Z' { return (ch - b'A') as u32; }
            if ch >= b'a' && ch <= b'z' { return (ch - b'a') as u32 + 26; }
            if ch >= b'0' && ch <= b'9' { return (ch - b'0') as u32 + 52; }
            if ch == b'+' { return 62; }
            if ch == b'/' { return 63; }
            0
        };

        let mut i: usize = 0;
        let mut j: usize = 0;

        // loop over 24 bit chunks
        while *s.add(i) != 0 {
            // take next 24 bit chunk (4 chars; 6 bits each)
            let char_1 = decode(*s.add(i) as u8);
            i += 1;
            let char_2 = decode(*s.add(i) as u8);
            i += 1;
            let char_3 = decode(*s.add(i) as u8);
            i += 1;
            let char_4 = decode(*s.add(i) as u8);
            i += 1;

            // merge into 32 bit int
            let k: u32 = (char_1 << 18) | (char_2 << 12) | (char_3 << 6) | char_4;

            // write up to three bytes (exclude padding at end)
            *buf.add(j) = ((k >> 16) & 0xFF) as u8;
            j += 1;
            if *s.add(i - 2) as u8 != b'=' {
                *buf.add(j) = ((k >> 8) & 0xFF) as u8;
                j += 1;
            }
            if *s.add(i - 1) as u8 != b'=' {
                *buf.add(j) = (k & 0xFF) as u8;
                j += 1;
            }
        }
        j
    }
}

/// C: mju_historyInit (engine/engine_util_misc.h:184)
/// Calls: mju_copy, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_init(buf: *mut f64, n: i32, dim: i32, times: *const f64, values: *const f64, user: f64) {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees buf[2 + n*(1+dim)], times[n], values[n*dim] are valid
    unsafe {
        // check strict monotonicity of times
        for i in 0..(n - 1) as usize {
            if *times.add(i + 1) - *times.add(i) < MJ_MINVAL {
                panic!("mju_historyInit: times must be strictly increasing");
            }
        }

        // buf layout: [user(1), cursor(1), times(n), values(n*dim)]
        *buf.add(0) = user;
        *buf.add(1) = (n - 1) as f64;  // cursor points to newest

        let buf_times = buf.add(2);
        let buf_values = buf.add(2 + n as usize);

        if times != buf_times {
            crate::engine::engine_util_blas::mju_copy(buf_times, times, n);
        }
        if !values.is_null() {
            crate::engine::engine_util_blas::mju_copy(buf_values, values, n * dim);
        }
    }
}

/// C: mju_historyInsert (engine/engine_util_misc.h:189)
/// Calls: historyFindIndex, historyPhysicalIndex, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_insert(buf: *mut f64, n: i32, dim: i32, t: f64) -> *mut f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees buf has layout [user(1), cursor(1), times(n), values(n*dim)]
    unsafe {
        let cursor_ptr = buf.add(1);
        let mut cursor = *cursor_ptr as i32;
        let times = buf.add(2);
        let values = buf.add(2 + n as usize);

        // find logical insertion index: times[i-1] < t <= times[i]
        let i = history_find_index(times, n, cursor, t);

        // exact match at logical i: return pointer to existing slot
        if i < n {
            let phys_i = history_physical_index(cursor, n, i);
            if f64::abs(t - *times.add(phys_i as usize)) < MJ_MINVAL {
                return values.add((phys_i as usize) * (dim as usize));
            }
        }

        // logical i == 0: new sample is older than oldest, replace oldest slot
        if i == 0 {
            let oldest_phys = history_physical_index(cursor, n, 0);
            *times.add(oldest_phys as usize) = t;
            return values.add((oldest_phys as usize) * (dim as usize));
        }

        // logical i == n: new sample is newer than newest, advance cursor and write
        if i == n {
            cursor = (cursor + 1) % n;
            *cursor_ptr = cursor as f64;
            *times.add(cursor as usize) = t;
            return values.add((cursor as usize) * (dim as usize));
        }

        // 0 < i < n: out-of-order insertion, shift [1, i-1] left (dropping 0), insert at i-1
        for j in 0..i - 1 {
            let src_phys = history_physical_index(cursor, n, j + 1);
            let dst_phys = history_physical_index(cursor, n, j);
            *times.add(dst_phys as usize) = *times.add(src_phys as usize);
            crate::engine::engine_util_blas::mju_copy(
                values.add((dst_phys as usize) * (dim as usize)),
                values.add((src_phys as usize) * (dim as usize)),
                dim,
            );
        }
        let insert_phys = history_physical_index(cursor, n, i - 1);
        *times.add(insert_phys as usize) = t;
        values.add((insert_phys as usize) * (dim as usize))
    }
}

/// C: mju_historyRead (engine/engine_util_misc.h:194)
/// Calls: historyFindIndex, historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_read(buf: *const f64, n: i32, dim: i32, res: *mut f64, t: f64, interp: i32) -> *const f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees buf has layout [capacity, cursor, times[n], values[n*dim]]
    //         and res has space for dim elements
    unsafe {
        let cursor = *buf.add(1) as i32;
        let times = buf.add(2);
        let values = buf.add(2 + n as usize);

        let oldest_phys = history_physical_index(cursor, n, 0);
        let newest_phys = history_physical_index(cursor, n, n - 1);
        let t_oldest = *times.add(oldest_phys as usize);
        let t_newest = *times.add(newest_phys as usize);

        // extrapolate before oldest: return pointer to oldest value
        if t <= t_oldest + MJ_MINVAL {
            return values.add((oldest_phys as usize) * (dim as usize));
        }

        // extrapolate after newest: return pointer to newest value
        if t >= t_newest - MJ_MINVAL {
            return values.add((newest_phys as usize) * (dim as usize));
        }

        // find bracketing logical index: times[i-1] < t <= times[i]
        let i = history_find_index(times, n, cursor, t);
        let phys_i = history_physical_index(cursor, n, i);

        // check for exact match at i
        if (t - *times.add(phys_i as usize)).abs() < MJ_MINVAL {
            return values.add((phys_i as usize) * (dim as usize));
        }

        // lo = i-1, hi = i (we know i > 0 because t > t_oldest)
        let phys_lo = history_physical_index(cursor, n, i - 1);
        let phys_hi = phys_i;

        // zero-order hold: return pointer to lo (most recent sample <= t)
        if interp == 0 {
            return values.add((phys_lo as usize) * (dim as usize));
        }

        let dt = *times.add(phys_hi as usize) - *times.add(phys_lo as usize);
        let alpha = (t - *times.add(phys_lo as usize)) / dt;

        // piecewise linear interpolation
        if interp == 1 {
            for d in 0..dim as usize {
                *res.add(d) = *values.add((phys_lo as usize) * (dim as usize) + d)
                    + alpha * (*values.add((phys_hi as usize) * (dim as usize) + d)
                        - *values.add((phys_lo as usize) * (dim as usize) + d));
            }
        } else {
            // cubic spline interpolation - Hermite basis functions
            let alpha2 = alpha * alpha;
            let alpha3 = alpha2 * alpha;
            let h00 = 2.0 * alpha3 - 3.0 * alpha2 + 1.0;
            let h10 = alpha3 - 2.0 * alpha2 + alpha;
            let h01 = -2.0 * alpha3 + 3.0 * alpha2;
            let h11 = alpha3 - alpha2;

            for d in 0..dim as usize {
                // finite differenced catmull-rom slopes, 0 at endpoints
                let mut m_lo: f64 = 0.0;
                if i > 1 {
                    let phys_lo_prev = history_physical_index(cursor, n, i - 2);
                    let dt_lo = *times.add(phys_hi as usize) - *times.add(phys_lo_prev as usize);
                    m_lo = (*values.add((phys_hi as usize) * (dim as usize) + d)
                        - *values.add((phys_lo_prev as usize) * (dim as usize) + d))
                        / dt_lo;
                }

                let mut m_hi: f64 = 0.0;
                if i < n - 1 {
                    let phys_hi_next = history_physical_index(cursor, n, i + 1);
                    let dt_hi = *times.add(phys_hi_next as usize) - *times.add(phys_lo as usize);
                    m_hi = (*values.add((phys_hi_next as usize) * (dim as usize) + d)
                        - *values.add((phys_lo as usize) * (dim as usize) + d))
                        / dt_hi;
                }

                *res.add(d) = h00 * *values.add((phys_lo as usize) * (dim as usize) + d)
                    + h10 * dt * m_lo
                    + h01 * *values.add((phys_hi as usize) * (dim as usize) + d)
                    + h11 * dt * m_hi;
            }
        }

        std::ptr::null()
    }
}

/// C: mju_encodePyramid (engine/engine_util_misc.h:200)
/// Calls: mju_min
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_pyramid(pyramid: *mut f64, force: *const f64, mu: *const f64, dim: i32) {
    // SAFETY: caller guarantees pyramid, force, mu point to valid arrays of appropriate size
    unsafe {
        let a = *force.add(0) / ((dim - 1) as f64);

        for i in 0..(dim - 1) as usize {
            let b = mju_min(a, *force.add(i + 1) / *mu.add(i));
            *pyramid.add(2 * i) = 0.5 * (a + b);
            *pyramid.add(2 * i + 1) = 0.5 * (a - b);
        }
    }
}

/// C: mju_decodePyramid (engine/engine_util_misc.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_pyramid(force: *mut f64, pyramid: *const f64, mu: *const f64, dim: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (force : * mut f64, pyramid : * const f64, mu : * const f64, dim : i32)
    // Previous return: ()
    // SAFETY: caller guarantees force, pyramid, mu arrays are valid for dim
    unsafe {
        if dim == 1 {
            *force.add(0) = *pyramid.add(0);
            return;
        }
        *force.add(0) = 0.0;
        for i in 0..2 * (dim - 1) as usize {
            *force.add(0) += *pyramid.add(i);
        }
        for i in 0..(dim - 1) as usize {
            *force.add(i + 1) = (*pyramid.add(2 * i) - *pyramid.add(2 * i + 1)) * *mu.add(i);
        }
    }
}

/// C: mju_springDamper (engine/engine_util_misc.h:208)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_spring_damper(pos0: f64, vel0: f64, Kp: f64, Kv: f64, dt: f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // determinant of characteristic equation
    let det: f64 = Kv * Kv - 4.0 * Kp;

    // overdamping
    if det > MJ_MINVAL {
        let w: f64 = f64::sqrt(det) / 2.0;
        let r1: f64 = -Kv / 2.0 + w;
        let r2: f64 = -Kv / 2.0 - w;
        let c1: f64 = (pos0 * r2 - vel0) / (r2 - r1);
        let c2: f64 = (pos0 * r1 - vel0) / (r1 - r2);
        c1 * f64::exp(r1 * dt) + c2 * f64::exp(r2 * dt)
    }
    // critical damping
    else if det <= MJ_MINVAL && det >= -MJ_MINVAL {
        let c1: f64 = pos0;
        let c2: f64 = vel0 + Kv * c1 / 2.0;
        f64::exp(-Kv * dt / 2.0) * (c1 + c2 * dt)
    }
    // underdamping
    else {
        let w: f64 = f64::sqrt(f64::abs(det)) / 2.0;
        let c1: f64 = pos0;
        let c2: f64 = (vel0 + Kv * c1 / 2.0) / w;
        f64::exp(-Kv * dt / 2.0) * (c1 * f64::cos(w * dt) + c2 * f64::sin(w * dt))
    }
}

/// C: mju_outsideBox (engine/engine_util_misc.h:213)
/// Calls: mju_message, mju_mulMatTVec3, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_outside_box(point: *const f64, pos: *const f64, mat: *const f64, size: *const f64, inflate: f64) -> i32 {
    use crate::engine::engine_util_blas::{mju_mul_mat_t_vec3, mju_scl3};

    // check inflation coefficient
    if inflate < 1.0 {
        panic!("mju_outsideBox: inflation coefficient must be >= 1");
    }

    // SAFETY: caller guarantees point[3], pos[3], mat[9], size[3] are valid
    unsafe {
        // vector from pos to point, projected to box frame
        let mut vec: [f64; 3] = [
            *point.add(0) - *pos.add(0),
            *point.add(1) - *pos.add(1),
            *point.add(2) - *pos.add(2),
        ];
        mju_mul_mat_t_vec3(vec.as_mut_ptr(), mat, vec.as_ptr());

        // big: inflated box
        let mut big: [f64; 3] = [*size.add(0), *size.add(1), *size.add(2)];
        if inflate > 1.0 {
            mju_scl3(big.as_mut_ptr(), big.as_ptr(), inflate);
        }

        // check if outside big box
        if vec[0] > big[0] || vec[0] < -big[0]
            || vec[1] > big[1] || vec[1] < -big[1]
            || vec[2] > big[2] || vec[2] < -big[2]
        {
            return 1;
        }

        // quick return if no inflation
        if inflate == 1.0 {
            return -1;
        }

        // check if inside small (deflated) box
        let small: [f64; 3] = [
            *size.add(0) / inflate,
            *size.add(1) / inflate,
            *size.add(2) / inflate,
        ];
        if vec[0] < small[0] && vec[0] > -small[0]
            && vec[1] < small[1] && vec[1] > -small[1]
            && vec[2] < small[2] && vec[2] > -small[2]
        {
            return -1;
        }

        // within margin between small and big box
        0
    }
}

/// C: mju_printMat (engine/engine_util_misc.h:217)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat(mat: *const f64, nr: i32, nc: i32) {
    extern "C" { fn printf(fmt: *const i8, ...) -> i32; }
    // SAFETY: mat points to nr*nc contiguous f64 values, caller guarantees valid pointer
    unsafe {
        for r in 0..nr {
            for c in 0..nc {
                printf(b"%.8f \0".as_ptr() as *const i8, *mat.add((r * nc + c) as usize));
            }
            printf(b"\n\0".as_ptr() as *const i8);
        }
        printf(b"\n\0".as_ptr() as *const i8);
    }
}

/// C: mju_printMatSparse (engine/engine_util_misc.h:220)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat_sparse(mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    extern "C" { fn printf(fmt: *const i8, ...) -> i32; }
    // SAFETY: mat, rownnz, rowadr, colind are valid pointers from caller
    unsafe {
        for r in 0..nr {
            let adr_start = *rowadr.add(r as usize);
            let adr_end = adr_start + *rownnz.add(r as usize);
            for adr in adr_start..adr_end {
                printf(
                    b"(%d %d): %9.6f  \0".as_ptr() as *const i8,
                    r,
                    *colind.add(adr as usize),
                    *mat.add(adr as usize),
                );
            }
            printf(b"\n\0".as_ptr() as *const i8);
        }
        printf(b"\n\0".as_ptr() as *const i8);
    }
}

/// C: mju_min (engine/engine_util_misc.h:225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_min(a: f64, b: f64) -> f64 {
    if a <= b { a } else { b }
}

/// C: mju_max (engine/engine_util_misc.h:228)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_max(a: f64, b: f64) -> f64 {
    if a >= b { a } else { b }
}

/// C: mju_clip (engine/engine_util_misc.h:231)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_clip(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}

/// C: mju_sign (engine/engine_util_misc.h:234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sign(x: f64) -> f64 {
    if x < 0.0 { -1.0 } else if x > 0.0 { 1.0 } else { 0.0 }
}

/// C: mju_round (engine/engine_util_misc.h:237)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_round(x: f64) -> i32 {
    let lower = x.floor();
    let upper = x.ceil();
    if x - lower < upper - x { lower as i32 } else { upper as i32 }
}

/// C: mju_type2Str (engine/engine_util_misc.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn mju_type2str(r#type: i32) -> *const i8 {
    // SAFETY: Return static C string pointer based on type enum
    match r#type {
        1 => b"body\0".as_ptr() as *const i8,
        2 => b"xbody\0".as_ptr() as *const i8,
        3 => b"joint\0".as_ptr() as *const i8,
        4 => b"dof\0".as_ptr() as *const i8,
        5 => b"geom\0".as_ptr() as *const i8,
        6 => b"site\0".as_ptr() as *const i8,
        7 => b"camera\0".as_ptr() as *const i8,
        8 => b"light\0".as_ptr() as *const i8,
        9 => b"flex\0".as_ptr() as *const i8,
        10 => b"mesh\0".as_ptr() as *const i8,
        11 => b"skin\0".as_ptr() as *const i8,
        12 => b"hfield\0".as_ptr() as *const i8,
        13 => b"texture\0".as_ptr() as *const i8,
        14 => b"material\0".as_ptr() as *const i8,
        15 => b"pair\0".as_ptr() as *const i8,
        16 => b"exclude\0".as_ptr() as *const i8,
        17 => b"equality\0".as_ptr() as *const i8,
        18 => b"tendon\0".as_ptr() as *const i8,
        19 => b"actuator\0".as_ptr() as *const i8,
        20 => b"sensor\0".as_ptr() as *const i8,
        21 => b"numeric\0".as_ptr() as *const i8,
        22 => b"text\0".as_ptr() as *const i8,
        23 => b"tuple\0".as_ptr() as *const i8,
        24 => b"key\0".as_ptr() as *const i8,
        25 => b"plugin\0".as_ptr() as *const i8,
        100 => b"frame\0".as_ptr() as *const i8,
        _ => core::ptr::null(),
    }
}

/// C: mju_str2Type (engine/engine_util_misc.h:243)
#[allow(unused_variables, non_snake_case)]
pub fn mju_str2type(str: *const i8) -> i32 {
    unsafe {
        // SAFETY: Compare C string against known type names
        let s = str;
        let cmp = |lit: &[u8]| -> bool {
            let mut i = 0;
            loop {
                let c = *s.add(i) as u8;
                if i >= lit.len() - 1 { return c == 0; }
                if c != lit[i] { return false; }
                i += 1;
            }
        };
        if cmp(b"body\0") { return 1; }
        if cmp(b"xbody\0") { return 2; }
        if cmp(b"joint\0") { return 3; }
        if cmp(b"dof\0") { return 4; }
        if cmp(b"geom\0") { return 5; }
        if cmp(b"site\0") { return 6; }
        if cmp(b"camera\0") { return 7; }
        if cmp(b"light\0") { return 8; }
        if cmp(b"flex\0") { return 9; }
        if cmp(b"mesh\0") { return 10; }
        if cmp(b"skin\0") { return 11; }
        if cmp(b"hfield\0") { return 12; }
        if cmp(b"texture\0") { return 13; }
        if cmp(b"material\0") { return 14; }
        if cmp(b"pair\0") { return 15; }
        if cmp(b"exclude\0") { return 16; }
        if cmp(b"equality\0") { return 17; }
        if cmp(b"tendon\0") { return 18; }
        if cmp(b"actuator\0") { return 19; }
        if cmp(b"sensor\0") { return 20; }
        if cmp(b"numeric\0") { return 21; }
        if cmp(b"text\0") { return 22; }
        if cmp(b"tuple\0") { return 23; }
        if cmp(b"key\0") { return 24; }
        if cmp(b"plugin\0") { return 25; }
        0 // mjOBJ_UNKNOWN
    }
}

/// C: mju_writeNumBytes (engine/engine_util_misc.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_num_bytes(nbytes: usize) -> *const i8 {
    thread_local! {
        static MESSAGE: std::cell::RefCell<[u8; 20]> = std::cell::RefCell::new([0u8; 20]);
    }
    extern "C" { fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32; }
    const SUFFIX: &[u8; 7] = b" KMGTPE";

    let mut i: i32 = 0;
    while i < 6 {
        let bits: usize = 1usize << (10 * (6 - i));
        if nbytes >= bits && (nbytes & (bits - 1)) == 0 {
            break;
        }
        i += 1;
    }

    MESSAGE.with(|cell| {
        let mut buf = cell.borrow_mut();
        let ptr = buf.as_mut_ptr() as *mut i8;
        // SAFETY: ptr points to 20-byte thread-local buffer, snprintf will not overflow
        unsafe {
            if i < 6 {
                snprintf(
                    ptr, 20,
                    b"%zu%c\0".as_ptr() as *const i8,
                    nbytes >> (10 * (6 - i)),
                    SUFFIX[(6 - i) as usize] as i32,
                );
            } else {
                snprintf(
                    ptr, 20,
                    b"%zu\0".as_ptr() as *const i8,
                    nbytes >> (10 * (6 - i)),
                );
            }
        }
        buf.as_ptr() as *const i8
    })
}

/// C: mju_warningText (engine/engine_util_misc.h:249)
/// Calls: mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning_text(warning: i32, info: usize) -> *const i8 {
    // thread-local buffer for warning string
    thread_local! {
        static STR: std::cell::RefCell<[u8; 1000]> = std::cell::RefCell::new([0u8; 1000]);
    }

    extern "C" {
        fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
    }

    const mjWARN_INERTIA: i32 = 0;
    const mjWARN_CONTACTFULL: i32 = 1;
    const mjWARN_CNSTRFULL: i32 = 2;
    const mjWARN_BADQPOS: i32 = 3;
    const mjWARN_BADQVEL: i32 = 4;
    const mjWARN_BADQACC: i32 = 5;
    const mjWARN_BADCTRL: i32 = 6;

    STR.with(|cell| {
        let mut buf = cell.borrow_mut();
        let ptr = buf.as_mut_ptr() as *mut i8;
        // SAFETY: ptr is a valid buffer of 1000 bytes, format strings are null-terminated
        unsafe {
            match warning {
                mjWARN_INERTIA => {
                    snprintf(ptr, 1000,
                        b"Inertia matrix is too close to singular at DOF %zu. Check model.\0".as_ptr() as *const i8,
                        info);
                }
                mjWARN_CONTACTFULL => {
                    snprintf(ptr, 1000,
                        b"Too many contacts. The arena memory is full, increase arena memory allocation.(ncon = %zu)\0".as_ptr() as *const i8,
                        info);
                }
                mjWARN_CNSTRFULL => {
                    let bytes_str = mju_write_num_bytes(info);
                    snprintf(ptr, 1000,
                        b"Insufficient arena memory for the number of constraints generated. Increase arena memory allocation above %s bytes.\0".as_ptr() as *const i8,
                        bytes_str);
                }
                mjWARN_BADQPOS => {
                    snprintf(ptr, 1000,
                        b"Nan, Inf or huge value in QPOS at DOF %zu. The simulation is unstable.\0".as_ptr() as *const i8,
                        info);
                }
                mjWARN_BADQVEL => {
                    snprintf(ptr, 1000,
                        b"Nan, Inf or huge value in QVEL at DOF %zu. The simulation is unstable.\0".as_ptr() as *const i8,
                        info);
                }
                mjWARN_BADQACC => {
                    snprintf(ptr, 1000,
                        b"Nan, Inf or huge value in QACC at DOF %zu. The simulation is unstable.\0".as_ptr() as *const i8,
                        info);
                }
                mjWARN_BADCTRL => {
                    snprintf(ptr, 1000,
                        b"Nan, Inf or huge value in CTRL at ACTUATOR %zu. The simulation is unstable.\0".as_ptr() as *const i8,
                        info);
                }
                _ => {
                    snprintf(ptr, 1000,
                        b"Unknown warning type %d.\0".as_ptr() as *const i8,
                        warning);
                }
            }
        }
        buf.as_ptr() as *const i8
    })
}

/// C: mju_isBad (engine/engine_util_misc.h:252)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_bad(x: f64) -> i32 {
    const MJ_MAXVAL: f64 = 1E10_f64;
    if x.is_nan() || x > MJ_MAXVAL || x < -MJ_MAXVAL {
        1
    } else {
        0
    }
}

/// C: mju_isZero (engine/engine_util_misc.h:255)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero(vec: *const f64, n: i32) -> i32 {
    // SAFETY: caller guarantees vec points to valid array of at least n f64
    unsafe {
        for i in 0..n as usize {
            if *vec.add(i) != 0.0 {
                return 0;
            }
        }
    }
    1
}

/// C: mju_isZeroByte (engine/engine_util_misc.h:258)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero_byte(vec: *const u8, n: i32) -> i32 {
    // SAFETY: caller guarantees vec points to valid array of at least n bytes
    unsafe {
        if n == 0 {
            return 1;
        }
        if *vec != 0 {
            return 0;
        }
        // compare vec[0..n-1] with vec[1..n] — if first byte is 0 and all equal, all are 0
        for i in 1..n as usize {
            if *vec.add(i) != *vec {
                return 0;
            }
        }
    }
    1
}

/// C: mju_zeroInt (engine/engine_util_misc.h:261)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_int(res: *mut i32, n: i32) {
    // SAFETY: caller guarantees res points to valid array of at least n i32 elements
    unsafe { std::ptr::write_bytes(res, 0, n as usize); }
}

/// C: mju_copyInt (engine/engine_util_misc.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_int(res: *mut i32, vec: *const i32, n: i32) {
    // SAFETY: caller guarantees res and vec point to valid arrays of at least n i32 elements
    unsafe { std::ptr::copy_nonoverlapping(vec, res, n as usize); }
}

/// C: mju_fillInt (engine/engine_util_misc.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fill_int(res: *mut i32, val: i32, n: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut i32, val : i32, n : i32)
    // Previous return: ()
    // SAFETY: caller guarantees res points to valid array of at least n i32 elements
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = val;
        }
    }
}

/// C: mju_standardNormal (engine/engine_util_misc.h:270)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_standard_normal(num2: *mut f64) -> f64 {
    extern "C" { fn rand() -> i32; }
    const RAND_MAX: f64 = 2147483647.0;
    let scale: f64 = 2.0 / RAND_MAX;

    let mut x1: f64;
    let mut x2: f64;
    let mut w: f64;
    // SAFETY: rand() is a standard C library function, always safe to call
    unsafe {
        loop {
            x1 = scale * (rand() as f64) - 1.0;
            x2 = scale * (rand() as f64) - 1.0;
            w = x1 * x1 + x2 * x2;
            if !(w >= 1.0 || w == 0.0) {
                break;
            }
        }
    }
    w = ((-2.0 * w.ln()) / w).sqrt();
    if !num2.is_null() {
        // SAFETY: caller guarantees num2 is valid when non-null
        unsafe { *num2 = x2 * w; }
    }
    x1 * w
}

/// C: mju_f2n (engine/engine_util_misc.h:273)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_f2n(res: *mut f64, vec: *const f32, n: i32) {
    // SAFETY: caller guarantees res and vec point to valid arrays of at least n elements
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec.add(i) as f64;
        }
    }
}

/// C: mju_n2f (engine/engine_util_misc.h:276)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_n2f(res: *mut f32, vec: *const f64, n: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f32, vec : * const f64, n : i32)
    // Previous return: ()
    // SAFETY: caller guarantees res points to n f32 and vec points to n f64
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec.add(i) as f32;
        }
    }
}

/// C: mju_d2n (engine/engine_util_misc.h:279)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_d2n(res: *mut f64, vec: *const f64, n: i32) {
    // SAFETY: caller guarantees res and vec are valid for n elements
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec.add(i);
        }
    }
}

/// C: mju_n2d (engine/engine_util_misc.h:282)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_n2d(res: *mut f64, vec: *const f64, n: i32) {
    // SAFETY: caller guarantees res and vec point to valid arrays of at least n elements
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec.add(i);
        }
    }
}

/// C: mju_gather (engine/engine_util_misc.h:285)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees res, vec, ind are valid for n elements
    unsafe {
        if ind.is_null() {
            crate::engine::engine_util_blas::mju_copy(res, vec, n);
            return;
        }

        for i in 0..n as usize {
            *res.add(i) = *vec.add(*ind.add(i) as usize);
        }
    }
}

/// C: mju_gatherMasked (engine/engine_util_misc.h:288)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_masked(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    // SAFETY: res, vec, ind point to valid arrays of length >= n per caller contract
    unsafe {
        for i in 0..n as usize {
            let idx = *ind.add(i);
            *res.add(i) = if idx >= 0 { *vec.add(idx as usize) } else { 0.0 };
        }
    }
}

/// C: mju_scatter (engine/engine_util_misc.h:291)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scatter(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees res, vec, ind point to valid memory
    unsafe {
        if ind.is_null() {
            crate::engine::engine_util_blas::mju_copy(res, vec, n);
            return;
        }
        for i in 0..n as usize {
            *res.add(*ind.add(i) as usize) = *vec.add(i);
        }
    }
}

/// C: mju_gatherInt (engine/engine_util_misc.h:294)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees all pointers valid for n elements
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec.add(*ind.add(i) as usize);
        }
    }
}

/// C: mju_scatterInt (engine/engine_util_misc.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scatter_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees all pointers valid for n elements
    unsafe {
        for i in 0..n as usize {
            *res.add(*ind.add(i) as usize) = *vec.add(i);
        }
    }
}

/// C: mju_sparseMap (engine/engine_util_misc.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sparse_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32) {
    // SAFETY: caller guarantees all pointers are valid for the sparse matrix dimensions
    unsafe {
        for i in 0..nr as usize {
            let mut res_cursor = *res_rowadr.add(i);
            let res_end = res_cursor + *res_rownnz.add(i);
            let mut src_cursor = *src_rowadr.add(i);
            let src_end = src_cursor + *src_rownnz.add(i);

            while res_cursor < res_end {
                let res_col = *res_colind.add(res_cursor as usize);
                while src_cursor < src_end && *src_colind.add(src_cursor as usize) < res_col {
                    src_cursor += 1;
                }
                *map.add(res_cursor as usize) = src_cursor;
                res_cursor += 1;
                src_cursor += 1;
            }
        }
    }
}

/// C: mju_lower2SymMap (engine/engine_util_misc.h:306)
/// Calls: mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_lower2sym_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32, cursor: *mut i32) {
    if nr == 0 {
        return;
    }

    // SAFETY: caller guarantees all pointers valid with sufficient lengths
    unsafe {
        // default all map entries to "no source"
        let nnz: i32 = *res_rowadr.add((nr - 1) as usize) + *res_rownnz.add((nr - 1) as usize);
        mju_fill_int(map, -1, nnz);

        // initialize per-row cursor
        for i in 0..nr as usize {
            *cursor.add(i) = *res_rowadr.add(i);
        }

        // sweep src rows; for each lower (i,j) set res(i,j) and res(j,i)
        for i in 0..nr {
            let src_start: i32 = *src_rowadr.add(i as usize);
            let src_end: i32 = src_start + *src_rownnz.add(i as usize);

            // sweep src row
            let mut k = src_start;
            while k < src_end {
                let j: i32 = *src_colind.add(k as usize);
                if j > i {
                    break; // use only lower triangle of src
                }

                // --- lower triangle: res(i, j)
                let res_start: i32 = *res_rowadr.add(i as usize);
                let res_end: i32 = res_start + *res_rownnz.add(i as usize);
                let mut c: i32 = *cursor.add(i as usize);

                // increment c until there is a match
                while c < res_end && *res_colind.add(c as usize) < j {
                    c += 1;
                }

                // found match, set index, advance and save cursor
                if c < res_end && *res_colind.add(c as usize) == j {
                    *map.add(c as usize) = k;
                    c += 1;
                }
                *cursor.add(i as usize) = c;

                // --- upper mirror: res(j, i)
                if j != i {
                    let res_start_j: i32 = *res_rowadr.add(j as usize);
                    let res_end_j: i32 = res_start_j + *res_rownnz.add(j as usize);
                    let mut c_j: i32 = *cursor.add(j as usize);

                    // increment c until there is a match
                    while c_j < res_end_j && *res_colind.add(c_j as usize) < i {
                        c_j += 1;
                    }

                    // found match, set index and advance and save cursor
                    if c_j < res_end_j && *res_colind.add(c_j as usize) == i {
                        *map.add(c_j as usize) = k;
                        c_j += 1;
                    }
                    *cursor.add(j as usize) = c_j;
                }

                k += 1;
            }
        }
    }
}

/// C: mju_insertionSort (engine/engine_util_misc.h:312)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort(list: *mut f64, n: i32) {
    // SAFETY: caller guarantees list points to a valid array of n elements
    unsafe {
        for i in 1..n as usize {
            let x = *list.add(i);
            let mut j: i32 = i as i32 - 1;
            while j >= 0 && *list.add(j as usize) > x {
                *list.add((j + 1) as usize) = *list.add(j as usize);
                j -= 1;
            }
            *list.add((j + 1) as usize) = x;
        }
    }
}

/// C: mju_insertionSortInt (engine/engine_util_misc.h:315)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort_int(list: *mut i32, n: i32) {
    // SAFETY: caller guarantees list points to a valid array of n elements
    unsafe {
        for i in 1..n as usize {
            let x = *list.add(i);
            let mut j: i32 = i as i32 - 1;
            while j >= 0 && *list.add(j as usize) > x {
                *list.add((j + 1) as usize) = *list.add(j as usize);
                j -= 1;
            }
            *list.add((j + 1) as usize) = x;
        }
    }
}

/// C: mju_Halton (engine/engine_util_misc.h:318)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_halton(index: i32, base: i32) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (index : i32, base : i32)
    // Previous return: f64
    let mut n0 = index;
    let b = base as f64;
    let mut f = 1.0 / b;
    let mut hn = 0.0;
    while n0 > 0 {
        let n1 = n0 / base;
        let r = n0 - n1 * base;
        hn += f * (r as f64);
        f /= b;
        n0 = n1;
    }
    hn
}

/// C: mju_strncpy (engine/engine_util_misc.h:321)
#[allow(unused_variables, non_snake_case)]
pub fn mju_strncpy(dst: *mut i8, src: *const i8, n: i32) -> *mut i8 {
    // SAFETY: caller guarantees dst/src point to valid buffers of at least n bytes
    unsafe {
        if !dst.is_null() && !src.is_null() && n > 0 {
            std::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, n as usize);
            *dst.add((n - 1) as usize) = 0;
        }
        dst
    }
}

/// C: mju_polyForce (engine/engine_util_misc.h:326)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    // SAFETY: poly points to at least n f64 (caller contract)
    unsafe {
        let x = if flg_odd != 0 { f64::abs(x) } else { x };
        let mut res: f64 = linear;

        let mut xpow: f64 = 1.0;
        for i in 0..n as usize {
            xpow *= x;
            res += *poly.add(i) * xpow;
        }

        res
    }
}

/// C: mjd_xPolyForce (engine/engine_util_misc.h:329)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_x_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    // SAFETY: poly points to at least n f64 (caller contract)
    unsafe {
        let x = if flg_odd != 0 { f64::abs(x) } else { x };
        let mut res: f64 = linear;

        let mut xpow: f64 = 1.0;
        for i in 0..n as usize {
            xpow *= x;
            res += (i as f64 + 2.0) * *poly.add(i) * xpow;
        }

        res
    }
}

/// C: mju_polyPotential (engine/engine_util_misc.h:332)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_potential(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    // SAFETY: caller guarantees poly points to valid array of at least n f64
    unsafe {
        let x = if flg_odd != 0 { x.abs() } else { x };
        let mut res = 0.5 * linear * (x * x);

        let mut xpow = x;
        for i in 0..n as usize {
            xpow *= x;
            res += *poly.add(i) / ((i as f64) + 3.0) * (xpow * x);
        }

        res
    }
}

/// C: mju_sigmoid (engine/engine_util_misc.h:335)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sigmoid(x: f64) -> f64 {
    // fast return
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    // sigmoid: f(x) = 6*x^5 - 15*x^4 + 10*x^3
    x * x * x * (3.0 * x * (2.0 * x - 5.0) + 10.0)
}

