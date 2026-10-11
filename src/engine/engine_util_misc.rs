//! Port of: engine/engine_util_misc.c
//! IR hash: fd5288ad7242f001
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
/// Calls: cxx:_mju_dot, cxx:_mju_normalize
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_is_intersect, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_length_circle, cxx:_mju_add, cxx:_mju_dot, cxx:_mju_normalize, cxx:_mju_sub
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
/// Calls: cxx:_mju_addScl, cxx:_mju_copy, cxx:_mju_max, cxx:_mju_norm, cxx:_mju_normalize, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_inside(pnt: *mut f64, end: *const f64, radius: f64) -> f64 {
    use crate::engine::engine_util_blas::{mju_addScl, mju_copy, mju_norm, mju_normalize, mju_scl};
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
                mju_addScl(tmp.as_mut_ptr(), end, dif.as_ptr(), a, 2);
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

/// C: mju_wrap (engine/engine_util_misc.c:283)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_wrap_circle, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_wrap_inside, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_norm3, cxx:_mju_normalize, cxx:_mju_normalize3, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_wrap(wpnt: *mut f64, x0: *const f64, x1: *const f64, xpos: *const f64, xmat: *const f64, radius: f64, r#type: i32, side: *const f64) -> f64 {
    use crate::engine::engine_util_blas::{
        mju_sub3, mju_mulMatTVec3, mju_mulMatVec3, mju_norm3,
        mju_copy3, mju_normalize3, mju_scl3, mju_addTo3,
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
        mju_mulMatTVec3(p[0].as_mut_ptr(), xmat, tmp.as_ptr());
        mju_sub3(tmp.as_mut_ptr(), x1, xpos);
        mju_mulMatTVec3(p[1].as_mut_ptr(), xmat, tmp.as_ptr());

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
            mju_mulMatTVec3(s.as_mut_ptr(), xmat, tmp.as_ptr());

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
            mju_addTo3(res.as_mut_ptr().add(3 * i), tmp.as_ptr());
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
        mju_mulMatVec3(wpnt, xmat, res.as_ptr());
        mju_mulMatVec3(wpnt.add(3), xmat, res.as_ptr().add(3));
        mju_addTo3(wpnt, xpos);
        mju_addTo3(wpnt.add(3), xpos);

        wlen
    }
}

/// C: mju_geomSemiAxes (engine/engine_util_misc.c:423)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_geom_semi_axes(semiaxes: *mut f64, size: *const f64, r#type: u32) {
    todo!() // cxx:_mju_geomSemiAxes
}

/// C: mju_insidePrimitive (engine/engine_util_misc.c:453)
/// Calls: cxx:_mju_clip, cxx:_mju_dot3, cxx:_mju_mulMatTVec3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_inside_primitive(pos: *const f64, mat: *const f64, size: *const f64, r#type: u32, point: *const f64) -> i32 {
    todo!() // cxx:_mju_insidePrimitive
}

/// C: mju_camPixelRay (engine/engine_util_misc.c:505)
/// Calls: cxx:_mju_add3, cxx:_mju_copy3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_pixel_ray(origin: *mut f64, direction: *mut f64, cam_xpos: *const f64, cam_xmat: *const f64, col: i32, row: i32, fx: f64, fy: f64, cx: f64, cy: f64, projection: i32, ortho_extent: f64) {
    todo!() // cxx:_mju_camPixelRay
}

/// C: mju_defGradient (engine/engine_util_misc.c:541)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_def_gradient(res: *mut f64, p: *const f64, dof: *const f64, order: i32) {
    todo!() // cxx:_mju_defGradient
}

/// C: mju_evalBasis (engine/engine_util_misc.c:567)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eval_basis(x: *const f64, i: i32, order: i32) -> f64 {
    todo!() // cxx:_mju_evalBasis
}

/// C: mju_evalBasisArray (engine/engine_util_misc.c:578)
/// Calls: cxx:_mju_evalBasis
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eval_basis_array(basis: *mut f64, x: *const f64, order: i32) {
    todo!() // cxx:_mju_evalBasisArray
}

/// C: mju_cellLookup (engine/engine_util_misc.c:628)
/// Calls: cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cell_lookup(coord: *const f64, cellnum: *const i32, order: i32, local: *mut f64, nodeindices: *mut i32) -> i32 {
    todo!() // cxx:_mju_cellLookup
}

/// C: mju_interpolate3D (engine/engine_util_misc.c:673)
/// Calls: cxx:_mju_addToScl3, cxx:_mju_evalBasis, cxx:_mju_evalBasisArray
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_interpolate3d(res: *mut f64, x: *const f64, coeff: *const f64, order: i32, nodeindices: *const i32) {
    todo!() // cxx:_mju_interpolate3D
}

/// C: flexInterpRotation (engine/engine_util_misc.c:694)
/// Calls: cxx:_mju_defGradient, cxx:_mju_mat2Rot, cxx:_mju_negQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn flex_interp_rotation(order: i32, xpos_c: *const f64, local: *const f64, quat: *mut f64) {
    todo!() // cxx-internal:engine_util_misc.c.o:_flexInterpRotation
}

/// C: mju_flexGatherCellState (engine/engine_util_misc.c:721)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_flexInterpRotation, cxx:_mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_cell_state(order: i32, cy: i32, cz: i32, ci: i32, cj: i32, ck: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_c: *mut f64, vel_c: *mut f64, xpos0_c: *mut f64, nodeindices: *mut i32, quat: *mut f64) {
    todo!() // cxx:_mju_flexGatherCellState
}

/// C: mju_flexInterpRotation2D (engine/engine_util_misc.c:754)
/// Calls: cxx:_mju_cross, cxx:_mju_mat2Rot, cxx:_mju_negQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_interp_rotation2d(order: i32, xpos_f: *const f64, npe: i32, axis0: i32, axis1: i32, normal_axis: i32, local: *const f64, quat: *mut f64) {
    todo!() // cxx:_mju_flexInterpRotation2D
}

/// C: mju_flexGatherFaceState (engine/engine_util_misc.c:814)
/// Calls: cxx:_mju_copy3, cxx:_mju_flexInterpRotation2D
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_face_state(order: i32, cx: i32, cy: i32, cz: i32, face_elem_idx: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_f: *mut f64, vel_f: *mut f64, xpos0_f: *mut f64, nodeindices: *mut i32, quat: *mut f64) {
    todo!() // cxx:_mju_flexGatherFaceState
}

/// C: mju_flexFaceNormal2D (engine/engine_util_misc.c:882)
/// Calls: cxx:_mju_cross, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_face_normal2d(normal: *mut f64, t1: *mut f64, t2: *mut f64, order: i32, xpos_f: *const f64, local: *const f64) {
    todo!() // cxx:_mju_flexFaceNormal2D
}

/// C: nodeAt (engine/engine_util_misc.c:902)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn node_at(nodexpos: *const f64, ny: i32, nz: i32, i: i32, j: i32, k: i32) -> *const f64 {
    todo!() // cxx-internal:engine_util_misc.c.o:_nodeAt
}

/// C: mju_shellTrackInterior (engine/engine_util_misc.c:907)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_nodeAt, cxx:_mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_shell_track_interior(nodexpos: *mut f64, nx: i32, ny: i32, nz: i32) {
    todo!() // cxx:_mju_shellTrackInterior
}

/// C: addWeight (engine/engine_util_misc.c:984)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_weight(nb: *mut i32, body: *mut i32, bweight: *mut f64, b: i32, w: f64) {
    todo!() // cxx-internal:engine_util_misc.c.o:_addWeight
}

/// C: mju_shellTFIWeights (engine/engine_util_misc.c:1003)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_addWeight
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_shell_tfi_weights(nx: i32, ny: i32, nz: i32, i: i32, j: i32, k: i32, w: f64, nb: *mut i32, body: *mut i32, bweight: *mut f64, nodebodyid: *const i32, nstart: i32) {
    todo!() // cxx:_mju_shellTFIWeights
}

/// C: mju_muscleGainLength (engine/engine_util_misc.c:1049)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain_length(length: f64, lmin: f64, lmax: f64) -> f64 {
    // mjMINVAL = 1E-15
    const MJ_MINVAL: f64 = 1e-15;

    // Inline mjMAX macro: ((a) > (b)) ? (a) : (b)
    #[inline(always)]
    fn mj_max(a: f64, b: f64) -> f64 {
        if a > b { a } else { b }
    }

    if lmin <= length && length <= lmax {
        // mid-ranges (maximum is at 1.0)
        let a = 0.5 * (lmin + 1.0);
        let b = 0.5 * (1.0 + lmax);

        if length <= a {
            let x = (length - lmin) / mj_max(MJ_MINVAL, a - lmin);
            return 0.5 * x * x;
        } else if length <= 1.0 {
            let x = (1.0 - length) / mj_max(MJ_MINVAL, 1.0 - a);
            return 1.0 - 0.5 * x * x;
        } else if length <= b {
            let x = (length - 1.0) / mj_max(MJ_MINVAL, b - 1.0);
            return 1.0 - 0.5 * x * x;
        } else {
            let x = (lmax - length) / mj_max(MJ_MINVAL, lmax - b);
            return 0.5 * x * x;
        }
    }

    0.0
}

/// C: mju_muscleGain (engine/engine_util_misc.c:1076)
/// Calls: cxx:_mju_muscleGainLength
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // cxx:_mju_muscleGain
}

/// C: mju_muscleBias (engine/engine_util_misc.c:1121)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_bias(len: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // cxx:_mju_muscleBias
}

/// C: mju_muscleDynamicsTimescale (engine/engine_util_misc.c:1156)
/// Calls: cxx:_mju_sigmoid
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_dynamics_timescale(dctrl: f64, tau_act: f64, tau_deact: f64, smoothing_width: f64) -> f64 {
    // C: mju_muscleDynamicsTimescale
    const MJ_MINVAL: f64 = 1e-15;
    
    // hard switching
    if smoothing_width < MJ_MINVAL {
        if dctrl > 0.0 { tau_act } else { tau_deact }
    }
    // smooth switching
    else {
        // scale by width, center around 0.5 midpoint, rescale to bounds
        tau_deact + (tau_act - tau_deact) * mju_sigmoid(dctrl / smoothing_width + 0.5)
    }
}

/// C: mju_muscleDynamics (engine/engine_util_misc.c:1174)
/// Calls: cxx:_mju_clip, cxx:_mju_muscleDynamicsTimescale
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_dynamics(ctrl: f64, act: f64, prm: *const f64) -> f64 {
    todo!() // cxx:_mju_muscleDynamics
}

/// C: mj_lugreStribeck (engine/engine_util_misc.c:1195)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_lugre_stribeck(velocity: f64, F_C: f64, F_S: f64, v_S: f64) -> f64 {
    // C: mjtNum ratio = velocity / mju_max(mjMINVAL, v_S);
    //    return F_C + (F_S - F_C) * mju_exp(-ratio*ratio);
    const MJ_MINVAL: f64 = 1e-15;
    let ratio = velocity / mju_max(MJ_MINVAL, v_S);
    F_C + (F_S - F_C) * (-ratio * ratio).exp()
}

/// C: mj_dcmotorSlots (engine/engine_util_misc.c:1202)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_dcmotor_slots(dynprm: *const f64, gainprm: *const f64) -> mjDCMotorSlots {
    todo!() // cxx:_mj_dcmotorSlots
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

/// C: mju_encodeBase64 (engine/engine_util_misc.c:1244)
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_base64(buf: *mut i8, data: *const u8, ndata: usize) -> usize {
    todo!() // cxx:_mju_encodeBase64
}

/// C: mju_isValidBase64 (engine/engine_util_misc.c:1297)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_valid_base64(s: *const i8) -> usize {
    todo!() // cxx:_mju_isValidBase64
}

/// C: mju_decodeBase64 (engine/engine_util_misc.c:1327)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:__decode
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_base64(buf: *mut u8, s: *const i8) -> usize {
    todo!() // cxx:_mju_decodeBase64
}

/// C: historyPhysicalIndex (engine/engine_util_misc.c:1359)
#[allow(unused_variables, non_snake_case)]
pub fn history_physical_index(cursor: i32, n: i32, logical: i32) -> i32 {
    (cursor + 1 + logical) % n
}

/// C: historyFindIndex (engine/engine_util_misc.c:1367)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn history_find_index(times: *const f64, n: i32, cursor: i32, t: f64) -> i32 {
    todo!() // cxx-internal:engine_util_misc.c.o:_historyFindIndex
}

/// C: mju_historyInit (engine/engine_util_misc.c:1404)
/// Calls: cxx:_mju_copy, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_init(buf: *mut f64, n: i32, dim: i32, times: *const f64, values: *const f64, user: f64) {
    todo!() // cxx:_mju_historyInit
}

/// C: mju_historyInsert (engine/engine_util_misc.c:1429)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_historyFindIndex, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_historyPhysicalIndex, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_insert(buf: *mut f64, n: i32, dim: i32, t: f64) -> *mut f64 {
    todo!() // cxx:_mju_historyInsert
}

/// C: mju_historyRead (engine/engine_util_misc.c:1478)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_historyFindIndex, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.c:_historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_read(buf: *const f64, n: i32, dim: i32, res: *mut f64, t: f64, interp: i32) -> *const f64 {
    todo!() // cxx:_mju_historyRead
}

/// C: mju_encodePyramid (engine/engine_util_misc.c:1569)
/// Calls: cxx:_mju_min
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_pyramid(pyramid: *mut f64, force: *const f64, mu: *const f64, dim: i32) {
    todo!() // cxx:_mju_encodePyramid
}

/// C: mju_decodePyramid (engine/engine_util_misc.c:1584)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_pyramid(force: *mut f64, pyramid: *const f64, mu: *const f64, dim: i32) {
    todo!() // cxx:_mju_decodePyramid
}

/// C: mju_springDamper (engine/engine_util_misc.c:1605)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_spring_damper(pos0: f64, vel0: f64, Kp: f64, Kv: f64, dt: f64) -> f64 {
    let (k, b, t) = (Kp, Kv, dt);
    // C: mju_springDamper - spring-damper dynamics
    const MJ_MINVAL: f64 = 1e-15;
    
    // determinant of characteristic equation
    let det = b * b - 4.0 * k;
    
    // scale tolerance with determinant terms to make regime selection invariant to time units
    let det_tol = MJ_MINVAL * mju_max(b * b, 4.0 * k.abs());
    
    // overdamping
    //  pos(t) = c1*exp(r1*t) + c2*exp(r2*t);  r12 = (-b +- sqrt(det))/2
    if det > det_tol {
        // compute w = sqrt(det)/2
        let w = det.sqrt() / 2.0;
        
        // avoid cancellation: compute the root of larger magnitude, then use r1*r2 = k
        let (r1, r2) = if b >= 0.0 {
            let r2 = -b / 2.0 - w;
            let r1 = k / r2;
            (r1, r2)
        } else {
            let r1 = -b / 2.0 + w;
            let r2 = k / r1;
            (r1, r2)
        };
        
        // compute coefficients
        let c1 = (pos0 * r2 - vel0) / (r2 - r1);
        let c2 = (pos0 * r1 - vel0) / (r1 - r2);
        
        // evaluate result
        c1 * (r1 * t).exp() + c2 * (r2 * t).exp()
    }
    // critical damping
    //  pos(t) = exp(-b*t/2) * (c1 + c2*t)
    else if det <= det_tol && det >= -det_tol {
        // compute coefficients
        let c1 = pos0;
        let c2 = vel0 + b * c1 / 2.0;
        
        // evaluate result
        (-b * t / 2.0).exp() * (c1 + c2 * t)
    }
    // underdamping
    //  pos(t) = exp(-b*t/2) * (c1*cos(w*t) + c2*sin(w*t));  w = sqrt(abs(det))/2
    else {
        // compute w
        let w = det.abs().sqrt() / 2.0;
        
        // compute coefficients
        let c1 = pos0;
        let c2 = (vel0 + b * c1 / 2.0) / w;
        
        // evaluate result
        (-b * t / 2.0).exp() * (c1 * (w * t).cos() + c2 * (w * t).sin())
    }
}

/// C: mju_outsideBox (engine/engine_util_misc.c:1668)
/// Calls: cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_outside_box(point: *const f64, pos: *const f64, mat: *const f64, size: *const f64, inflate: f64) -> i32 {
    todo!() // cxx:_mju_outsideBox
}

/// C: mju_printMat (engine/engine_util_misc.c:1710)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat(mat: *const f64, nr: i32, nc: i32) {
    todo!() // cxx:_mju_printMat
}

/// C: mju_printMatSparse (engine/engine_util_misc.c:1724)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat_sparse(mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // cxx:_mju_printMatSparse
}

/// C: mju_min (engine/engine_util_misc.c:1736)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_min(a: f64, b: f64) -> f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { if (a <= b) { a } else { b } }
}

/// C: mju_max (engine/engine_util_misc.c:1742)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_max(a: f64, b: f64) -> f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { if (a >= b) { a } else { b } }
}

/// C: mju_clip (engine/engine_util_misc.c:1748)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_clip(x: f64, min: f64, max: f64) -> f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { if (x < min) { min } else { if (x > max) { max } else { x } } }
}

/// C: mju_sign (engine/engine_util_misc.c:1754)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sign(x: f64) -> f64 {
    if x < 0.0 { -1.0 } else if x > 0.0 { 1.0 } else { 0.0 }
}

/// C: mju_round (engine/engine_util_misc.c:1760)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_round(x: f64) -> i32 {
    // C: int mju_round(mjtNum x) {
    //      double d = (double)x;
    //      if (d > INT_MAX) return INT_MAX;
    //      if (d < INT_MIN) return INT_MIN;
    //      return (int)round(d);
    //    }
    let d = x;
    if d > i32::MAX as f64 {
        return i32::MAX;
    }
    if d < i32::MIN as f64 {
        return i32::MIN;
    }
    d.round() as i32
}

/// C: mju_type2Str (engine/engine_util_misc.c:1769)
#[allow(unused_variables, non_snake_case)]
pub fn mju_type2str(r#type: i32) -> *const i8 {
    todo!() // cxx:_mju_type2Str
}

/// C: mju_str2Type (engine/engine_util_misc.c:1856)
#[allow(unused_variables, non_snake_case)]
pub fn mju_str2type(str: *const i8) -> i32 {
    todo!() // cxx:_mju_str2Type
}

/// C: mju_writeNumBytes (engine/engine_util_misc.c:1964)
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_num_bytes(nbytes: usize) -> *const i8 {
    todo!() // cxx:_mju_writeNumBytes
}

/// C: mju_warningText (engine/engine_util_misc.c:1984)
/// Calls: cxx:_mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning_text(warning: i32, info: usize) -> *const i8 {
    todo!() // cxx:_mju_warningText
}

/// C: mju_isBad (engine/engine_util_misc.c:2030)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_bad(x: f64) -> i32 {
    // C: return (x != x || x > mjMAXVAL || x < -mjMAXVAL);
    // mjMAXVAL = 1E+10
    const MJ_MAXVAL: f64 = 1e10;
    // x != x is the NaN check
    if x != x || x > MJ_MAXVAL || x < -MJ_MAXVAL {
        1
    } else {
        0
    }
}

/// C: mju_isZero (engine/engine_util_misc.c:2036)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero(vec: *const f64, n: i32) -> i32 {
    todo!() // cxx:_mju_isZero
}

/// C: mju_isZeroByte (engine/engine_util_misc.c:2048)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero_byte(vec: *const u8, n: i32) -> i32 {
    // C: if (!n || *vec) return !n;
    //    return memcmp(vec, vec + 1, n - 1) == 0;
    // SAFETY: caller guarantees vec points to valid memory of at least n bytes
    unsafe {
        if n == 0 || *vec != 0 {
            return if n == 0 { 1 } else { 0 };
        }
        // Compare vec[0..n-1] with vec[1..n]
        // All bytes are 0 iff vec[0] == 0 and all subsequent bytes equal vec[0]
        for i in 1..n as usize {
            if *vec.add(i) != *vec {
                return 0;
            }
        }
        1
    }
}

/// C: mju_zeroInt (engine/engine_util_misc.c:2055)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_int(res: *mut i32, n: i32) {
    // C: memset(res, 0, n*sizeof(int));
    // SAFETY: caller guarantees res points to valid memory of at least n i32 elements
    unsafe {
        std::ptr::write_bytes(res, 0, n as usize);
    }
}

/// C: mju_copyInt (engine/engine_util_misc.c:2061)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_int(res: *mut i32, vec: *const i32, n: i32) {
    // C: memcpy(res, vec, n*sizeof(int));
    // SAFETY: caller guarantees res and vec point to valid memory of at least n i32 elements
    unsafe {
        std::ptr::copy_nonoverlapping(vec, res, n as usize);
    }
}

/// C: mju_fillInt (engine/engine_util_misc.c:2066)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fill_int(res: *mut i32, val: i32, n: i32) {
    // C: for (int i = 0; i < n; i++) { res[i] = val; }
    // SAFETY: caller guarantees res points to valid memory of at least n i32 elements
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = val;
        }
    }
}

/// C: mju_standardNormal (engine/engine_util_misc.c:2073)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_standard_normal(num2: *mut f64) -> f64 {
    todo!() // cxx:_mju_standardNormal
}

/// C: mju_f2n (engine/engine_util_misc.c:2093)
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

/// C: mju_n2f (engine/engine_util_misc.c:2101)
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

/// C: mju_d2n (engine/engine_util_misc.c:2109)
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

/// C: mju_n2d (engine/engine_util_misc.c:2117)
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

/// C: mju_gather (engine/engine_util_misc.c:2125)
/// Calls: cxx:_mju_copy
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

/// C: mju_gatherMasked (engine/engine_util_misc.c:2139)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_masked(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_gatherMasked
}

/// C: mju_scatter (engine/engine_util_misc.c:2147)
/// Calls: cxx:_mju_copy
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

/// C: mju_gatherInt (engine/engine_util_misc.c:2160)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_gatherInt
}

/// C: mju_scatterInt (engine/engine_util_misc.c:2168)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scatter_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_scatterInt
}

/// C: mju_sparseMap (engine/engine_util_misc.c:2178)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sparse_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32) {
    todo!() // cxx:_mju_sparseMap
}

/// C: mju_lower2SymMap (engine/engine_util_misc.c:2203)
/// Calls: cxx:_mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_lower2sym_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32, cursor: *mut i32) {
    todo!() // cxx:_mju_lower2SymMap
}

/// C: mju_insertionSort (engine/engine_util_misc.c:2263)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort(list: *mut f64, n: i32) {
    todo!() // cxx:_mju_insertionSort
}

/// C: mju_insertionSortInt (engine/engine_util_misc.c:2277)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort_int(list: *mut i32, n: i32) {
    todo!() // cxx:_mju_insertionSortInt
}

/// C: mju_Halton (engine/engine_util_misc.c:2291)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_halton(index: i32, base: i32) -> f64 {
    todo!() // cxx:_mju_Halton
}

/// C: mju_strncpy (engine/engine_util_misc.c:2309)
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

/// C: mju_polyForce (engine/engine_util_misc.c:2322)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    todo!() // cxx:_mju_polyForce
}

/// C: mjd_xPolyForce (engine/engine_util_misc.c:2337)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_x_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    todo!() // cxx:_mjd_xPolyForce
}

/// C: mju_polyPotential (engine/engine_util_misc.c:2352)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_potential(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    todo!() // cxx:_mju_polyPotential
}

/// C: mju_sigmoid (engine/engine_util_misc.c:2367)
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

/// C: mju_flexPhi (engine/engine_util_misc.h:129)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_phi__1b46cb51e400ee77(s: f64, i: i32, order: i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:__ZL11mju_flexPhidii
}

/// C: mju_flexPhi (engine/engine_util_misc.h:129)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_phi__8d7a9c11408190cd(s: f64, i: i32, order: i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:_mju_flexPhi
}

/// C: mju_flexDphi (engine/engine_util_misc.h:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_dphi__8e74c89990d8e888(s: f64, i: i32, order: i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:__ZL12mju_flexDphidii
}

/// C: mju_flexDphi (engine/engine_util_misc.h:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_dphi__896b1972ce438b6e(s: f64, i: i32, order: i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_misc.h:_mju_flexDphi
}

