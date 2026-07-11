//! Port of: engine/engine_util_misc.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: is_intersect (engine/engine_util_misc.c:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_intersect(p1: *const f64, p2: *const f64, p3: *const f64, p4: *const f64) -> mjtBool {
    // WARNING: signature changed — verify body
    // Previous params: (p1 : * const f64, p2 : * const f64, p3 : * const f64, p4 : * const f64)
    // Previous return: mjtBool
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (p0 : * const f64, p1 : * const f64, ind : i32, radius : f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pnt : * mut f64, end : * const f64, side : * const f64, radius : f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pnt : * mut f64, end : * const f64, radius : f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (order : i32, xpos_c : * const f64, local : * const f64, quat : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: nodeAt (engine/engine_util_misc.c:902)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn node_at(nodexpos: *const f64, ny: i32, nz: i32, i: i32, j: i32, k: i32) -> *const f64 {
    // SAFETY: caller guarantees nodexpos points to valid array with enough elements
    unsafe {
        nodexpos.add((3 * (i * ny * nz + j * nz + k)) as usize)
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
    // WARNING: signature changed — verify body
    // Previous params: (nb : * mut i32, body : * mut i32, bweight : * mut f64, b : i32, w : f64)
    // Previous return: ()
    todo ! ()
}

/// C: _decode (engine/engine_util_misc.c:1217)
#[allow(unused_variables, non_snake_case)]
pub fn decode(ch: i8) -> u32 {
    // WARNING: signature changed — verify body
    // Previous params: (ch : i8)
    // Previous return: u32
    todo ! ()
}

/// C: historyPhysicalIndex (engine/engine_util_misc.c:1359)
#[allow(unused_variables, non_snake_case)]
pub fn history_physical_index(cursor: i32, n: i32, logical: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (cursor : i32, n : i32, logical : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (times : * const f64, n : i32, cursor : i32, t : f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (wpnt : * mut f64, x0 : * const f64, x1 : * const f64, xpos : * const f64, xmat : * const f64, radius : f64, r#type : i32, side : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: mju_muscleGainLength (engine/engine_util_misc.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain_length(length: f64, lmin: f64, lmax: f64) -> f64 {
    const MJMINVAL: f64 = 1e-15;
    if lmin <= length && length <= lmax {
        // mid-ranges (maximum is at 1.0)
        let a = 0.5 * (lmin + 1.0);
        let b = 0.5 * (1.0 + lmax);

        if length <= a {
            let denom = if a - lmin > MJMINVAL { a - lmin } else { MJMINVAL };
            let x = (length - lmin) / denom;
            return 0.5 * x * x;
        } else if length <= 1.0 {
            let denom = if 1.0 - a > MJMINVAL { 1.0 - a } else { MJMINVAL };
            let x = (1.0 - length) / denom;
            return 1.0 - 0.5 * x * x;
        } else if length <= b {
            let denom = if b - 1.0 > MJMINVAL { b - 1.0 } else { MJMINVAL };
            let x = (length - 1.0) / denom;
            return 1.0 - 0.5 * x * x;
        } else {
            let denom = if lmax - b > MJMINVAL { lmax - b } else { MJMINVAL };
            let x = (lmax - length) / denom;
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
    const MJMINVAL: f64 = 1e-15;
    // SAFETY: caller guarantees lengthrange points to 2, prm points to 9 f64
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
            force = scale / (if acc0 > MJMINVAL { acc0 } else { MJMINVAL });
        }

        // optimum length
        let L0 = (*lengthrange.add(1) - *lengthrange.add(0)) /
                 (if range1 - range0 > MJMINVAL { range1 - range0 } else { MJMINVAL });

        // normalized length and velocity
        let L = range0 + (len - *lengthrange.add(0)) / (if L0 > MJMINVAL { L0 } else { MJMINVAL });
        let V = vel / (if L0 * vmax > MJMINVAL { L0 * vmax } else { MJMINVAL });

        // length curve
        let FL = mju_muscle_gain_length(L, lmin, lmax);

        // velocity curve
        let FV;
        let y = fvmax - 1.0;
        if V <= -1.0 {
            FV = 0.0;
        } else if V <= 0.0 {
            FV = (V + 1.0) * (V + 1.0);
        } else if V <= y {
            FV = fvmax - (y - V) * (y - V) / (if y > MJMINVAL { y } else { MJMINVAL });
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
    const MJMINVAL: f64 = 1e-15;
    // SAFETY: caller guarantees lengthrange points to 2, prm points to 9 f64
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
            force = scale / (if acc0 > MJMINVAL { acc0 } else { MJMINVAL });
        }

        // optimum length
        let L0 = (*lengthrange.add(1) - *lengthrange.add(0)) /
                 (if range1 - range0 > MJMINVAL { range1 - range0 } else { MJMINVAL });

        // normalized length
        let L = range0 + (len - *lengthrange.add(0)) / (if L0 > MJMINVAL { L0 } else { MJMINVAL });

        // half-quadratic to (L0+lmax)/2, linear beyond
        let b = 0.5 * (1.0 + lmax);
        if L <= 1.0 {
            0.0
        } else if L <= b {
            let denom = if b - 1.0 > MJMINVAL { b - 1.0 } else { MJMINVAL };
            let x = (L - 1.0) / denom;
            -force * fpmax * 0.5 * x * x
        } else {
            let denom = if b - 1.0 > MJMINVAL { b - 1.0 } else { MJMINVAL };
            let x = (L - b) / denom;
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
    const MJMINVAL: f64 = 1e-15;
    let tau: f64;

    // hard switching
    if smoothing_width < MJMINVAL {
        tau = if dctrl > 0.0 { tau_act } else { tau_deact };
    }
    // smooth switching
    else {
        // scale by width, center around 0.5 midpoint, rescale to bounds
        tau = tau_deact + (tau_act - tau_deact) * mju_sigmoid(dctrl / smoothing_width + 0.5);
    }
    tau
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
    const MJMINVAL: f64 = 1e-15;
    // SAFETY: caller guarantees prm points to at least 3 contiguous f64
    unsafe {
        // clamp control
        let ctrlclamp = mju_clip(ctrl, 0.0, 1.0);

        // clamp activation
        let actclamp = mju_clip(act, 0.0, 1.0);

        // compute timescales as in Millard et al. (2013)
        let tau_act = *prm.add(0) * (0.5 + 1.5 * actclamp);    // activation timescale
        let tau_deact = *prm.add(1) / (0.5 + 1.5 * actclamp);  // deactivation timescale
        let smoothing_width = *prm.add(2);                      // width of smoothing sigmoid
        let dctrl = ctrlclamp - act;                             // excess excitation

        let tau = mju_muscle_dynamics_timescale(dctrl, tau_act, tau_deact, smoothing_width);

        // filter output
        dctrl / mju_max(MJMINVAL, tau)
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
    const MJMINVAL: f64 = 1e-15;
    let denom = if v_S > MJMINVAL { v_S } else { MJMINVAL };
    let ratio = velocity / denom;
    F_C + (F_S - F_C) * (-ratio * ratio).exp()
}

/// C: mj_dcmotorSlots (engine/engine_util_misc.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_dcmotor_slots(dynprm: *const f64, gainprm: *const f64) -> mjDCMotorSlots {
    todo!("C++: requires non-opaque mjDCMotorSlots struct definition")
}

/// C: mju_geomSemiAxes (engine/engine_util_misc.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_geom_semi_axes(semiaxes: *mut f64, size: *const f64, r#type: u32) {
    // mjtGeom enum values
    const MJGEOM_SPHERE: u32 = 2;
    const MJGEOM_CAPSULE: u32 = 3;
    const MJGEOM_CYLINDER: u32 = 5;

    // SAFETY: caller guarantees semiaxes points to 3 f64, size points to 3 f64
    unsafe {
        match r#type {
            MJGEOM_SPHERE => {
                *semiaxes.add(0) = *size.add(0);
                *semiaxes.add(1) = *size.add(0);
                *semiaxes.add(2) = *size.add(0);
            }
            MJGEOM_CAPSULE => {
                *semiaxes.add(0) = *size.add(0);
                *semiaxes.add(1) = *size.add(0);
                *semiaxes.add(2) = *size.add(1) + *size.add(0);
            }
            MJGEOM_CYLINDER => {
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

    // mjtGeom enum values
    const MJGEOM_PLANE: u32 = 0;
    const MJGEOM_SPHERE: u32 = 2;
    const MJGEOM_CAPSULE: u32 = 3;
    const MJGEOM_ELLIPSOID: u32 = 4;
    const MJGEOM_CYLINDER: u32 = 5;
    const MJGEOM_BOX: u32 = 6;

    // SAFETY: caller guarantees all pointers are valid
    unsafe {
        // vector from geom to point
        let mut vec = [0.0f64; 3];
        mju_sub3(vec.as_mut_ptr(), point, pos);

        // quick return for spheres, frame rotation not required
        if r#type == MJGEOM_SPHERE {
            return (mju_dot3(vec.as_ptr(), vec.as_ptr()) < *size.add(0) * *size.add(0)) as i32;
        }

        // rotate into local frame
        let mut plocal = [0.0f64; 3];
        mju_mul_mat_t_vec3(plocal.as_mut_ptr(), mat, vec.as_ptr());

        // handle other geom types
        match r#type {
            MJGEOM_CAPSULE => {
                let z = plocal[2];
                let z_clamped = mju_clip(z, -*size.add(1), *size.add(1));
                let z_dist_sq = (z - z_clamped) * (z - z_clamped);
                (plocal[0] * plocal[0] + plocal[1] * plocal[1] + z_dist_sq < *size.add(0) * *size.add(0)) as i32
            }
            MJGEOM_ELLIPSOID => {
                (plocal[0] * plocal[0] / (*size.add(0) * *size.add(0))
                    + plocal[1] * plocal[1] / (*size.add(1) * *size.add(1))
                    + plocal[2] * plocal[2] / (*size.add(2) * *size.add(2)) < 1.0) as i32
            }
            MJGEOM_CYLINDER => {
                (plocal[2].abs() < *size.add(1)
                    && plocal[0] * plocal[0] + plocal[1] * plocal[1] < *size.add(0) * *size.add(0)) as i32
            }
            MJGEOM_BOX => {
                (plocal[0].abs() < *size.add(0)
                    && plocal[1].abs() < *size.add(1)
                    && plocal[2].abs() < *size.add(2)) as i32
            }
            MJGEOM_PLANE => {
                (plocal[2] < 0.0) as i32
            }
            _ => 0,
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
    use crate::engine::engine_util_blas::{mju_copy3, mju_mul_mat_vec3, mju_normalize3, mju_add3};

    const MJPROJ_PERSPECTIVE: i32 = 0;

    // SAFETY: caller guarantees all pointers valid
    unsafe {
        // pixel center (row 0 = top of image)
        let px = col as f64 + 0.5 - cx;
        let py = row as f64 + 0.5 - cy;

        if projection == MJPROJ_PERSPECTIVE {
            // origin is camera position
            mju_copy3(origin, cam_xpos);

            // direction in camera frame: (x/fx, -y/fy, -1), then normalized
            let dir_cam = [px / fx, -py / fy, -1.0];
            mju_mul_mat_vec3(direction, cam_xmat, dir_cam.as_ptr());
            mju_normalize3(direction);
        } else {
            // orthographic: parallel rays, direction is -Z in camera frame
            *direction.add(0) = -*cam_xmat.add(2);
            *direction.add(1) = -*cam_xmat.add(5);
            *direction.add(2) = -*cam_xmat.add(8);

            // origin offset in camera frame (ortho_extent is full height, use half for each side)
            let half_extent = ortho_extent / 2.0;
            let offset_cam = [px / fx * half_extent, -py / fy * half_extent, 0.0];
            let mut offset_world = [0.0f64; 3];
            mju_mul_mat_vec3(offset_world.as_mut_ptr(), cam_xmat, offset_cam.as_ptr());
            mju_add3(origin, cam_xpos, offset_world.as_ptr());
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
    // SAFETY: caller guarantees res points to 9 f64, p to 3 f64,
    // dof to 3*(order+1)^3 f64.
    unsafe {
        let mut idx: i32 = 0;
        let mut gradient: [f64; 3];
        crate::engine::engine_util_blas::mju_zero(res, 9);

        let p0 = *p.add(0);
        let p1 = *p.add(1);
        let p2 = *p.add(2);

        let mut i = 0;
        while i <= order {
            let mut j = 0;
            while j <= order {
                let mut k = 0;
                while k <= order {
                    gradient = [
                        mju_flex_dphi(p0, i, order) * mju_flex_phi(p1, j, order) * mju_flex_phi(p2, k, order),
                        mju_flex_phi(p0, i, order) * mju_flex_dphi(p1, j, order) * mju_flex_phi(p2, k, order),
                        mju_flex_phi(p0, i, order) * mju_flex_phi(p1, j, order) * mju_flex_dphi(p2, k, order),
                    ];
                    *res.add(0) += *dof.add((3 * idx + 0) as usize) * gradient[0];
                    *res.add(1) += *dof.add((3 * idx + 0) as usize) * gradient[1];
                    *res.add(2) += *dof.add((3 * idx + 0) as usize) * gradient[2];
                    *res.add(3) += *dof.add((3 * idx + 1) as usize) * gradient[0];
                    *res.add(4) += *dof.add((3 * idx + 1) as usize) * gradient[1];
                    *res.add(5) += *dof.add((3 * idx + 1) as usize) * gradient[2];
                    *res.add(6) += *dof.add((3 * idx + 2) as usize) * gradient[0];
                    *res.add(7) += *dof.add((3 * idx + 2) as usize) * gradient[1];
                    *res.add(8) += *dof.add((3 * idx + 2) as usize) * gradient[2];
                    idx += 1;
                    k += 1;
                }
                j += 1;
            }
            i += 1;
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
    // inline phi (mju_flexPhi)
    #[inline(always)]
    fn phi(s: f64, i: i32, order: i32) -> f64 {
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

    // SAFETY: caller guarantees x points to at least 3 f64.
    unsafe {
        let x0 = *x.add(0);
        let x1 = *x.add(1);
        let x2 = *x.add(2);

        if order == 1 {
            phi(x2, i & 1, order) * phi(x1, i & 2, order) * phi(x0, i & 4, order)
        } else if order == 2 {
            phi(x2, i % 3, order) * phi(x1, (i / 3) % 3, order) * phi(x0, i / 9, order)
        } else {
            -1.0
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
    // inline phi (mju_flexPhi)
    #[inline(always)]
    fn phi(s: f64, i: i32, order: i32) -> f64 {
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

    // SAFETY: caller guarantees basis has enough space, x points to 3 f64.
    unsafe {
        let x0 = *x.add(0);
        let x1 = *x.add(1);
        let x2 = *x.add(2);

        if order == 1 {
            let p: [[f64; 2]; 3] = [
                [1.0 - x0, x0],
                [1.0 - x1, x1],
                [1.0 - x2, x2],
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
            let mut p = [[0.0f64; 3]; 3];
            for d in 0..3 {
                let xd = *x.add(d);
                for i in 0..3 {
                    p[d][i] = phi(xd, i as i32, 2);
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
        } else {
            let npoint = (order + 1) * (order + 1) * (order + 1);
            for j in 0..npoint {
                *basis.add(j as usize) = mju_eval_basis(x, j, order);
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
    // SAFETY: caller guarantees coord[3], cellnum[3], local[3], nodeindices has enough space
    unsafe {
        let cx = *cellnum.add(0);
        let cy = *cellnum.add(1);
        let cz = *cellnum.add(2);

        // find containing cell
        let mut ci = (*coord.add(0) * cx as f64).floor() as i32;
        let mut cj = (*coord.add(1) * cy as f64).floor() as i32;
        let mut ck = (*coord.add(2) * cz as f64).floor() as i32;
        ci = ci.min(cx - 1); ci = ci.max(0);
        cj = cj.min(cy - 1); cj = cj.max(0);
        ck = ck.min(cz - 1); ck = ck.max(0);

        // local parametric coordinates within cell
        *local.add(0) = mju_clip(*coord.add(0) * cx as f64 - ci as f64, 0.0, 1.0);
        *local.add(1) = mju_clip(*coord.add(1) * cy as f64 - cj as f64, 0.0, 1.0);
        *local.add(2) = mju_clip(*coord.add(2) * cz as f64 - ck as f64, 0.0, 1.0);

        // build node indices for this cell
        if !nodeindices.is_null() {
            let gi_base = ci * order;
            let gj_base = cj * order;
            let gk_base = ck * order;
            let ny_g = cy * order + 1;
            let nz_g = cz * order + 1;
            let mut ni = 0;
            for li in 0..=order {
                let gi = gi_base + li;
                let gi_stride = gi * ny_g * nz_g;
                for lj in 0..=order {
                    let gj = gj_base + lj;
                    let gj_stride = gi_stride + gj * nz_g;
                    for lk in 0..=order {
                        let gk = gk_base + lk;
                        *nodeindices.add(ni) = gj_stride + gk;
                        ni += 1;
                    }
                }
            }
        }

        let npc = (order + 1) * (order + 1) * (order + 1);
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
    // SAFETY: caller guarantees all pointers are valid with sufficient sizes.
    unsafe {
        let npoint = (order + 1) * (order + 1) * (order + 1);

        if npoint > 27 {
            for j in 0..npoint {
                let idx = if !nodeindices.is_null() { *nodeindices.add(j as usize) } else { j };
                let basis_val = mju_eval_basis(x, j, order);
                crate::engine::engine_util_blas::mju_add_to_scl3(res, coeff.add(3 * idx as usize), basis_val);
            }
            return;
        }

        let mut basis = [0.0f64; 27];
        mju_eval_basis_array(basis.as_mut_ptr(), x, order);

        for j in 0..npoint {
            let idx = if !nodeindices.is_null() { *nodeindices.add(j as usize) } else { j };
            crate::engine::engine_util_blas::mju_add_to_scl3(res, coeff.add(3 * idx as usize), basis[j as usize]);
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
    // WARNING: signature changed — verify body
    // Previous params: (order : i32, cy : i32, cz : i32, ci : i32, cj : i32, ck : i32, xpos_g : * const f64, vel_g : * const f64, xpos0_g : * const f64, xpos_c : * mut f64, vel_c : * mut f64, xpos0_c : * mut f64, nodeindices : * mut i32, quat : * mut f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (order : i32, cx : i32, cy : i32, cz : i32, face_elem_idx : i32, xpos_g : * const f64, vel_g : * const f64, xpos0_g : * const f64, xpos_f : * mut f64, vel_f : * mut f64, xpos0_f : * mut f64, nodeindices : * mut i32, quat : * mut f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (order : i32, xpos_f : * const f64, npe : i32, axis0 : i32, axis1 : i32, normal_axis : i32, local : * const f64, quat : * mut f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (normal : * mut f64, t1 : * mut f64, t2 : * mut f64, order : i32, xpos_f : * const f64, local : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (nodexpos : * mut f64, nx : i32, ny : i32, nz : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (nx : i32, ny : i32, nz : i32, i : i32, j : i32, k : i32, w : f64, nb : * mut i32, body : * mut i32, bweight : * mut f64, nodebodyid : * const i32, nstart : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_encodeBase64 (engine/engine_util_misc.h:163)
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_base64(buf: *mut i8, data: *const u8, ndata: usize) -> usize {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    // SAFETY: caller guarantees buf has enough space, data points to ndata bytes
    unsafe {
        let mut i: usize = 0;
        let mut j: usize = 0;

        // loop over 24-bit chunks
        while i + 3 <= ndata {
            let byte_1 = *data.add(i) as u32; i += 1;
            let byte_2 = *data.add(i) as u32; i += 1;
            let byte_3 = *data.add(i) as u32; i += 1;

            let k = (byte_1 << 16) | (byte_2 << 8) | byte_3;

            *buf.add(j) = TABLE[((k >> 18) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 12) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 6) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 0) & 63) as usize] as i8; j += 1;
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
            *buf.add(j) = TABLE[((k >> 6) & 63) as usize] as i8; j += 1;
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
        let mut pad: usize = 0;

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
        let len = i + pad;
        if len % 4 != 0 {
            0
        } else {
            3 * (len / 4) - pad
        }
    }
}

/// C: mju_decodeBase64 (engine/engine_util_misc.h:171)
/// Calls: _decode
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_base64(buf: *mut u8, s: *const i8) -> usize {
    // inline _decode helper
    #[inline(always)]
    fn decode_char(ch: i8) -> u32 {
        let c = ch as u8;
        if c >= b'A' && c <= b'Z' {
            return (c - b'A') as u32;
        }
        if c >= b'a' && c <= b'z' {
            return (c - b'a') as u32 + 26;
        }
        if c >= b'0' && c <= b'9' {
            return (c - b'0') as u32 + 52;
        }
        if c == b'+' {
            return 62;
        }
        if c == b'/' {
            return 63;
        }
        0
    }

    // SAFETY: caller guarantees buf has enough space for decoded output,
    // s is a valid null-terminated C string.
    unsafe {
        let mut i: usize = 0;
        let mut j: usize = 0;

        // loop over 24 bit chunks
        while *s.add(i) != 0 {
            // take next 24 bit chunk (4 chars; 6 bits each)
            let char_1 = decode_char(*s.add(i));
            i += 1;
            let char_2 = decode_char(*s.add(i));
            i += 1;
            let char_3 = decode_char(*s.add(i));
            i += 1;
            let char_4 = decode_char(*s.add(i));
            i += 1;

            // merge into 32 bit int
            let k = (char_1 << 18) | (char_2 << 12) | (char_3 << 6) | char_4;

            // write up to three bytes (exclude padding at end)
            *buf.add(j) = ((k >> 16) & 0xFF) as u8;
            j += 1;
            if *s.add(i - 2) != b'=' as i8 {
                *buf.add(j) = ((k >> 8) & 0xFF) as u8;
                j += 1;
            }
            if *s.add(i - 1) != b'=' as i8 {
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
    // WARNING: signature changed — verify body
    // Previous params: (buf : * mut f64, n : i32, dim : i32, times : * const f64, values : * const f64, user : f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (buf : * mut f64, n : i32, dim : i32, t : f64)
    // Previous return: * mut f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (buf : * const f64, n : i32, dim : i32, res : * mut f64, t : f64, interp : i32)
    // Previous return: * const f64
    todo ! ()
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
    // SAFETY: caller guarantees pyramid, force, mu have enough elements
    unsafe {
        let a = *force.add(0) / (dim - 1) as f64;

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
    // SAFETY: caller guarantees force, pyramid, mu have enough elements
    unsafe {
        // special handling of frictionless contacts
        if dim == 1 {
            *force.add(0) = *pyramid.add(0);
            return;
        }

        // force_normal = sum(pyramid0_i + pyramid1_i)
        *force.add(0) = 0.0;
        for i in 0..2 * (dim - 1) as usize {
            *force.add(0) += *pyramid.add(i);
        }

        // force_tangent_i = (pyramid0_i - pyramid1_i) * mu_i
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
pub fn mju_spring_damper(pos0: f64, vel0: f64, k: f64, b: f64, t: f64) -> f64 {
    const MJMINVAL: f64 = 1e-15;

    // determinant of characteristic equation
    let det = b * b - 4.0 * k;

    // overdamping
    if det > MJMINVAL {
        // compute w = sqrt(det)/2
        let w = det.sqrt() / 2.0;

        // compute r1,r2
        let r1 = -b / 2.0 + w;
        let r2 = -b / 2.0 - w;

        // compute coefficients
        let c1 = (pos0 * r2 - vel0) / (r2 - r1);
        let c2 = (pos0 * r1 - vel0) / (r1 - r2);

        // evaluate result
        c1 * (r1 * t).exp() + c2 * (r2 * t).exp()
    }
    // critical damping
    else if det <= MJMINVAL && det >= -MJMINVAL {
        // compute coefficients
        let c1 = pos0;
        let c2 = vel0 + b * c1 / 2.0;

        // evaluate result
        (-b * t / 2.0).exp() * (c1 + c2 * t)
    }
    // underdamping
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

/// C: mju_outsideBox (engine/engine_util_misc.h:213)
/// Calls: mju_message, mju_mulMatTVec3, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_outside_box(point: *const f64, pos: *const f64, mat: *const f64, size: *const f64, inflate: f64) -> i32 {
    // SAFETY: caller guarantees point, pos are [3], mat is [9], size is [3]
    unsafe {
        // check inflation coefficient
        if inflate < 1.0 {
            crate::engine::engine_util_errmem::mju_error(
                b"inflation coefficient must be >= 1\0".as_ptr() as *const i8
            );
        }

        // vector from pos to point, projected to box frame
        let mut vec: [f64; 3] = [
            *point.add(0) - *pos.add(0),
            *point.add(1) - *pos.add(1),
            *point.add(2) - *pos.add(2),
        ];
        crate::engine::engine_util_blas::mju_mul_mat_t_vec3(
            vec.as_mut_ptr(), mat, vec.as_ptr()
        );

        // big: inflated box
        let mut big: [f64; 3] = [*size.add(0), *size.add(1), *size.add(2)];
        if inflate > 1.0 {
            crate::engine::engine_util_blas::mju_scl3(
                big.as_mut_ptr(), big.as_ptr(), inflate
            );
        }

        // check if outside big box
        if vec[0] > big[0] || vec[0] < -big[0] ||
           vec[1] > big[1] || vec[1] < -big[1] ||
           vec[2] > big[2] || vec[2] < -big[2] {
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
        if vec[0] < small[0] && vec[0] > -small[0] &&
           vec[1] < small[1] && vec[1] > -small[1] &&
           vec[2] < small[2] && vec[2] > -small[2] {
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
    // SAFETY: caller guarantees mat points to nr*nc f64 values
    unsafe {
        for r in 0..nr {
            for c in 0..nc {
                print!("{:.8} ", *mat.add((r * nc + c) as usize));
            }
            println!();
        }
        println!();
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
    // SAFETY: caller guarantees all pointers are valid for the sparse matrix dimensions
    unsafe {
        for r in 0..nr {
            let adr_start = *rowadr.add(r as usize);
            let adr_end = adr_start + *rownnz.add(r as usize);
            for adr in adr_start..adr_end {
                print!("({} {}): {:9.6}  ", r, *colind.add(adr as usize), *mat.add(adr as usize));
            }
            println!();
        }
        println!();
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
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// C: mju_sign (engine/engine_util_misc.h:234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sign(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else if x > 0.0 {
        1.0
    } else {
        0.0
    }
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

    if x - lower < upper - x {
        lower as i32
    } else {
        upper as i32
    }
}

/// C: mju_type2Str (engine/engine_util_misc.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn mju_type2str(r#type: i32) -> *const i8 {
    // mjtObj enum values from mjtype.h
    const MJOBJ_UNKNOWN: i32 = 0;
    const MJOBJ_BODY: i32 = 1;
    const MJOBJ_XBODY: i32 = 2;
    const MJOBJ_JOINT: i32 = 3;
    const MJOBJ_DOF: i32 = 4;
    const MJOBJ_GEOM: i32 = 5;
    const MJOBJ_SITE: i32 = 6;
    const MJOBJ_CAMERA: i32 = 7;
    const MJOBJ_LIGHT: i32 = 8;
    const MJOBJ_FLEX: i32 = 9;
    const MJOBJ_MESH: i32 = 10;
    const MJOBJ_SKIN: i32 = 11;
    const MJOBJ_HFIELD: i32 = 12;
    const MJOBJ_TEXTURE: i32 = 13;
    const MJOBJ_MATERIAL: i32 = 14;
    const MJOBJ_PAIR: i32 = 15;
    const MJOBJ_EXCLUDE: i32 = 16;
    const MJOBJ_EQUALITY: i32 = 17;
    const MJOBJ_TENDON: i32 = 18;
    const MJOBJ_ACTUATOR: i32 = 19;
    const MJOBJ_SENSOR: i32 = 20;
    const MJOBJ_NUMERIC: i32 = 21;
    const MJOBJ_TEXT: i32 = 22;
    const MJOBJ_TUPLE: i32 = 23;
    const MJOBJ_KEY: i32 = 24;
    const MJOBJ_PLUGIN: i32 = 25;
    const MJOBJ_FRAME: i32 = 100;

    match r#type {
        MJOBJ_BODY => b"body\0".as_ptr() as *const i8,
        MJOBJ_XBODY => b"xbody\0".as_ptr() as *const i8,
        MJOBJ_JOINT => b"joint\0".as_ptr() as *const i8,
        MJOBJ_DOF => b"dof\0".as_ptr() as *const i8,
        MJOBJ_GEOM => b"geom\0".as_ptr() as *const i8,
        MJOBJ_SITE => b"site\0".as_ptr() as *const i8,
        MJOBJ_CAMERA => b"camera\0".as_ptr() as *const i8,
        MJOBJ_LIGHT => b"light\0".as_ptr() as *const i8,
        MJOBJ_FLEX => b"flex\0".as_ptr() as *const i8,
        MJOBJ_MESH => b"mesh\0".as_ptr() as *const i8,
        MJOBJ_SKIN => b"skin\0".as_ptr() as *const i8,
        MJOBJ_HFIELD => b"hfield\0".as_ptr() as *const i8,
        MJOBJ_TEXTURE => b"texture\0".as_ptr() as *const i8,
        MJOBJ_MATERIAL => b"material\0".as_ptr() as *const i8,
        MJOBJ_PAIR => b"pair\0".as_ptr() as *const i8,
        MJOBJ_EXCLUDE => b"exclude\0".as_ptr() as *const i8,
        MJOBJ_EQUALITY => b"equality\0".as_ptr() as *const i8,
        MJOBJ_TENDON => b"tendon\0".as_ptr() as *const i8,
        MJOBJ_ACTUATOR => b"actuator\0".as_ptr() as *const i8,
        MJOBJ_SENSOR => b"sensor\0".as_ptr() as *const i8,
        MJOBJ_NUMERIC => b"numeric\0".as_ptr() as *const i8,
        MJOBJ_TEXT => b"text\0".as_ptr() as *const i8,
        MJOBJ_TUPLE => b"tuple\0".as_ptr() as *const i8,
        MJOBJ_KEY => b"key\0".as_ptr() as *const i8,
        MJOBJ_PLUGIN => b"plugin\0".as_ptr() as *const i8,
        MJOBJ_FRAME => b"frame\0".as_ptr() as *const i8,
        _ => std::ptr::null(),
    }
}

/// C: mju_str2Type (engine/engine_util_misc.h:243)
#[allow(unused_variables, non_snake_case)]
pub fn mju_str2type(str_ptr: *const i8) -> i32 {
    // mjOBJ enum values
    const mjOBJ_UNKNOWN: i32 = 0;
    const mjOBJ_BODY: i32 = 1;
    const mjOBJ_XBODY: i32 = 2;
    const mjOBJ_JOINT: i32 = 3;
    const mjOBJ_DOF: i32 = 4;
    const mjOBJ_GEOM: i32 = 5;
    const mjOBJ_SITE: i32 = 6;
    const mjOBJ_CAMERA: i32 = 7;
    const mjOBJ_LIGHT: i32 = 8;
    const mjOBJ_FLEX: i32 = 9;
    const mjOBJ_MESH: i32 = 10;
    const mjOBJ_SKIN: i32 = 11;
    const mjOBJ_HFIELD: i32 = 12;
    const mjOBJ_TEXTURE: i32 = 13;
    const mjOBJ_MATERIAL: i32 = 14;
    const mjOBJ_PAIR: i32 = 15;
    const mjOBJ_EXCLUDE: i32 = 16;
    const mjOBJ_EQUALITY: i32 = 17;
    const mjOBJ_TENDON: i32 = 18;
    const mjOBJ_ACTUATOR: i32 = 19;
    const mjOBJ_SENSOR: i32 = 20;
    const mjOBJ_NUMERIC: i32 = 21;
    const mjOBJ_TEXT: i32 = 22;
    const mjOBJ_TUPLE: i32 = 23;
    const mjOBJ_KEY: i32 = 24;
    const mjOBJ_PLUGIN: i32 = 25;

    // SAFETY: caller guarantees str_ptr is a valid null-terminated C string
    unsafe {
        let s = core::ffi::CStr::from_ptr(str_ptr);
        if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"body\0") {
            mjOBJ_BODY
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"xbody\0") {
            mjOBJ_XBODY
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"joint\0") {
            mjOBJ_JOINT
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"dof\0") {
            mjOBJ_DOF
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"geom\0") {
            mjOBJ_GEOM
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"site\0") {
            mjOBJ_SITE
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"camera\0") {
            mjOBJ_CAMERA
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"light\0") {
            mjOBJ_LIGHT
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"flex\0") {
            mjOBJ_FLEX
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"mesh\0") {
            mjOBJ_MESH
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"skin\0") {
            mjOBJ_SKIN
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"hfield\0") {
            mjOBJ_HFIELD
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"texture\0") {
            mjOBJ_TEXTURE
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"material\0") {
            mjOBJ_MATERIAL
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"pair\0") {
            mjOBJ_PAIR
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"exclude\0") {
            mjOBJ_EXCLUDE
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"equality\0") {
            mjOBJ_EQUALITY
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"tendon\0") {
            mjOBJ_TENDON
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"actuator\0") {
            mjOBJ_ACTUATOR
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"sensor\0") {
            mjOBJ_SENSOR
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"numeric\0") {
            mjOBJ_NUMERIC
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"text\0") {
            mjOBJ_TEXT
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"tuple\0") {
            mjOBJ_TUPLE
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"key\0") {
            mjOBJ_KEY
        } else if s == core::ffi::CStr::from_bytes_with_nul_unchecked(b"plugin\0") {
            mjOBJ_PLUGIN
        } else {
            mjOBJ_UNKNOWN
        }
    }
}

/// C: mju_writeNumBytes (engine/engine_util_misc.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_num_bytes(nbytes: usize) -> *const i8 {
    use std::cell::RefCell;
    use std::fmt::Write;

    thread_local! {
        static MESSAGE: RefCell<[u8; 20]> = RefCell::new([0u8; 20]);
    }

    const SUFFIX: &[u8; 7] = b" KMGTPE";

    // find the largest suffix that divides evenly
    let mut idx = 0usize;
    for i in 0..6usize {
        let bits: usize = 1 << (10 * (6 - i));
        if nbytes >= bits && (nbytes & (bits - 1)) == 0 {
            idx = i;
            break;
        }
        if i == 5 {
            idx = 6;
        }
    }

    MESSAGE.with(|msg| {
        let mut buf = msg.borrow_mut();
        let shifted = nbytes >> (10 * (6 - idx));
        let mut s = String::new();
        if idx < 6 {
            let _ = write!(s, "{}{}", shifted, SUFFIX[6 - idx] as char);
        } else {
            let _ = write!(s, "{}", shifted);
        }
        let bytes = s.as_bytes();
        let len = bytes.len().min(19);
        buf[..len].copy_from_slice(&bytes[..len]);
        buf[len] = 0;
        buf.as_ptr() as *const i8
    })
}

/// C: mju_warningText (engine/engine_util_misc.h:249)
/// Calls: mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning_text(warning: i32, info: usize) -> *const i8 {
    use std::cell::RefCell;
    use std::fmt::Write;

    thread_local! {
        static STR: RefCell<[u8; 1000]> = RefCell::new([0u8; 1000]);
    }

    // mjtWarning enum values
    const MJWARN_INERTIA: i32 = 0;
    const MJWARN_CONTACTFULL: i32 = 1;
    const MJWARN_CNSTRFULL: i32 = 2;
    const MJWARN_BADQPOS: i32 = 4;
    const MJWARN_BADQVEL: i32 = 5;
    const MJWARN_BADQACC: i32 = 6;
    const MJWARN_BADCTRL: i32 = 7;

    STR.with(|cell| {
        let mut buf = cell.borrow_mut();
        let mut s = String::new();
        match warning {
            MJWARN_INERTIA => {
                let _ = write!(s, "Inertia matrix is too close to singular at DOF {}. Check model.", info);
            }
            MJWARN_CONTACTFULL => {
                let _ = write!(s, "Too many contacts. The arena memory is full, increase arena memory allocation.(ncon = {})", info);
            }
            MJWARN_CNSTRFULL => {
                // NOTE: in C this calls mju_writeNumBytes but we inline a simpler version
                let _ = write!(s, "Insufficient arena memory for the number of constraints generated. Increase arena memory allocation above {} bytes.", info);
            }
            MJWARN_BADQPOS => {
                let _ = write!(s, "Nan, Inf or huge value in QPOS at DOF {}. The simulation is unstable.", info);
            }
            MJWARN_BADQVEL => {
                let _ = write!(s, "Nan, Inf or huge value in QVEL at DOF {}. The simulation is unstable.", info);
            }
            MJWARN_BADQACC => {
                let _ = write!(s, "Nan, Inf or huge value in QACC at DOF {}. The simulation is unstable.", info);
            }
            MJWARN_BADCTRL => {
                let _ = write!(s, "Nan, Inf or huge value in CTRL at ACTUATOR {}. The simulation is unstable.", info);
            }
            _ => {
                let _ = write!(s, "Unknown warning type {}.", warning);
            }
        }
        let bytes = s.as_bytes();
        let len = bytes.len().min(999);
        buf[..len].copy_from_slice(&bytes[..len]);
        buf[len] = 0;
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
    const MJMAXVAL: f64 = 1e10;
    (x != x || x > MJMAXVAL || x < -MJMAXVAL) as i32
}

/// C: mju_isZero (engine/engine_util_misc.h:255)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero(vec: *const f64, n: i32) -> i32 {
    // SAFETY: caller guarantees vec points to at least n contiguous f64
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
    // SAFETY: caller guarantees vec points to at least n contiguous bytes
    // C logic: if (!n || *vec) return !n; return memcmp(vec, vec+1, n-1) == 0;
    unsafe {
        let n = n as usize;
        if n == 0 {
            return 1;
        }
        if *vec != 0 {
            return 0;
        }
        // all bytes equal to first byte (which is 0)?
        for i in 1..n {
            if *vec.add(i) != 0 {
                return 0;
            }
        }
        1
    }
}

/// C: mju_zeroInt (engine/engine_util_misc.h:261)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_int(res: *mut i32, n: i32) {
    // SAFETY: caller guarantees res points to at least n contiguous i32
    unsafe {
        std::ptr::write_bytes(res, 0, n as usize);
    }
}

/// C: mju_copyInt (engine/engine_util_misc.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_int(res: *mut i32, vec: *const i32, n: i32) {
    // SAFETY: caller guarantees res, vec point to at least n contiguous i32
    unsafe {
        std::ptr::copy_nonoverlapping(vec, res, n as usize);
    }
}

/// C: mju_fillInt (engine/engine_util_misc.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fill_int(res: *mut i32, val: i32, n: i32) {
    // SAFETY: caller guarantees res points to at least n contiguous i32
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
    const RAND_MAX: f64 = 2147483647.0;
    let scale: f64 = 2.0 / RAND_MAX;

    // SAFETY: caller guarantees num2 is either null or points to a valid f64
    unsafe {
        let mut x1: f64;
        let mut x2: f64;
        let mut w: f64;

        loop {
            x1 = scale * (rand() as f64) - 1.0;
            x2 = scale * (rand() as f64) - 1.0;
            w = x1 * x1 + x2 * x2;
            if !(w >= 1.0 || w == 0.0) {
                break;
            }
        }

        w = ((-2.0 * w.ln()) / w).sqrt();
        if !num2.is_null() {
            *num2 = x2 * w;
        }

        x1 * w
    }
}

extern "C" {
    fn rand() -> i32;
}

/// C: mju_f2n (engine/engine_util_misc.h:273)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_f2n(res: *mut f64, vec: *const f32, n: i32) {
    // SAFETY: caller guarantees res and vec have at least n elements
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
    // SAFETY: caller guarantees res and vec have at least n elements
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
    // SAFETY: caller guarantees res and vec have at least n elements
    // Note: mjtNum is f64 so this is just a copy
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
    // SAFETY: caller guarantees res and vec have at least n elements
    // Note: mjtNum is f64 so this is just a copy
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
    if ind.is_null() {
        crate::engine::engine_util_blas::mju_copy(res, vec, n);
        return;
    }
    // SAFETY: caller guarantees res points to n, vec has enough elements, ind points to n indices
    unsafe {
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
    // SAFETY: caller guarantees res points to n, vec has enough elements, ind points to n indices
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
    if ind.is_null() {
        crate::engine::engine_util_blas::mju_copy(res, vec, n);
        return;
    }
    // SAFETY: caller guarantees res has enough elements, vec points to n, ind points to n indices
    unsafe {
        for i in 0..n as usize {
            *res.add(*ind.add(i) as usize) = *vec.add(i);
        }
    }
}

/// C: mju_gatherInt (engine/engine_util_misc.h:294)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees res points to n, vec has enough elements, ind points to n indices
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec.add(*ind.add(i) as usize);
        }
    }
}

/// C: mju_scatterInt (engine/engine_util_misc.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scatter_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees res, vec, ind are valid for n elements,
    // and ind[i] are valid indices into res
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
            let mut res_cursor = *res_rowadr.add(i) as usize;
            let res_end = res_cursor + *res_rownnz.add(i) as usize;
            let mut src_cursor = *src_rowadr.add(i) as usize;
            let src_end = src_cursor + *src_rownnz.add(i) as usize;

            while res_cursor < res_end {
                let res_col = *res_colind.add(res_cursor);
                while src_cursor < src_end && *src_colind.add(src_cursor) < res_col {
                    src_cursor += 1;
                }

                // found match, set index and advance cursors
                *map.add(res_cursor) = src_cursor as i32;
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

    // SAFETY: caller guarantees all pointers are valid for the sparse matrix dimensions
    unsafe {
        // default all map entries to "no source"
        let nnz = *res_rowadr.add(nr as usize - 1) + *res_rownnz.add(nr as usize - 1);
        mju_fill_int(map, -1, nnz);

        // initialize per-row cursor
        for i in 0..nr as usize {
            *cursor.add(i) = *res_rowadr.add(i);
        }

        // sweep src rows; for each lower (i,j) set res(i,j) and res(j,i)
        for i in 0..nr as usize {
            let src_start = *src_rowadr.add(i) as usize;
            let src_end = src_start + *src_rownnz.add(i) as usize;

            // sweep src row
            for k in src_start..src_end {
                let j = *src_colind.add(k) as usize;
                if j > i {
                    break; // use only lower triangle of src
                }

                // --- lower triangle: res(i, j)
                let res_start = *res_rowadr.add(i) as usize;
                let res_end = res_start + *res_rownnz.add(i) as usize;
                let mut c = *cursor.add(i) as usize;

                // increment c until there is a match
                while c < res_end && (*res_colind.add(c) as usize) < j {
                    c += 1;
                }

                // found match, set index, advance and save cursor
                if c < res_end && *res_colind.add(c) as usize == j {
                    *map.add(c) = k as i32;
                    c += 1;
                }
                *cursor.add(i) = c as i32;

                // --- upper mirror: res(j, i)
                if j != i {
                    let res_start2 = *res_rowadr.add(j) as usize;
                    let res_end2 = res_start2 + *res_rownnz.add(j) as usize;
                    let mut c2 = *cursor.add(j) as usize;

                    // increment c until there is a match
                    while c2 < res_end2 && (*res_colind.add(c2) as usize) < i {
                        c2 += 1;
                    }

                    // found match, set index and advance and save cursor
                    if c2 < res_end2 && *res_colind.add(c2) as usize == i {
                        *map.add(c2) = k as i32;
                        c2 += 1;
                    }
                    *cursor.add(j) = c2 as i32;
                }
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
    // SAFETY: caller guarantees list has at least n elements
    unsafe {
        for i in 1..n as usize {
            let x = *list.add(i);
            let mut j = i as i32 - 1;
            while j >= 0 && *list.add(j as usize) > x {
                *list.add(j as usize + 1) = *list.add(j as usize);
                j -= 1;
            }
            *list.add((j + 1) as usize) = x;
        }
    }
}

/// C: mju_insertionSortInt (engine/engine_util_misc.h:315)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort_int(list: *mut i32, n: i32) {
    // SAFETY: caller guarantees list has at least n elements
    unsafe {
        for i in 1..n as usize {
            let x = *list.add(i);
            let mut j = i as i32 - 1;
            while j >= 0 && *list.add(j as usize) > x {
                *list.add(j as usize + 1) = *list.add(j as usize);
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
    let mut n0 = index;
    let b = base as f64;
    let mut f = 1.0 / b;
    let mut hn: f64 = 0.0;

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
    if !dst.is_null() && !src.is_null() && n > 0 {
        // SAFETY: caller guarantees dst has at least n bytes, src is valid
        unsafe {
            let n_usize = n as usize;
            // copy up to n bytes
            let mut i = 0usize;
            while i < n_usize {
                *dst.add(i) = *src.add(i);
                if *src.add(i) == 0 {
                    break;
                }
                i += 1;
            }
            // ensure null-termination at n-1
            *dst.add(n_usize - 1) = 0;
        }
    }
    dst
}

/// C: mju_polyForce (engine/engine_util_misc.h:326)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    let x = if flg_odd != 0 { x.abs() } else { x };
    let mut res = linear;

    // SAFETY: caller guarantees poly points to at least n contiguous f64
    unsafe {
        let mut xpow: f64 = 1.0;
        for i in 0..n as usize {
            xpow *= x;
            res += *poly.add(i) * xpow;
        }
    }

    res
}

/// C: mjd_xPolyForce (engine/engine_util_misc.h:329)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_x_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    let x = if flg_odd != 0 { x.abs() } else { x };
    let mut res = linear;

    // SAFETY: caller guarantees poly points to at least n contiguous f64
    unsafe {
        let mut xpow: f64 = 1.0;
        for i in 0..n as usize {
            xpow *= x;
            res += (i as f64 + 2.0) * *poly.add(i) * xpow;
        }
    }

    res
}

/// C: mju_polyPotential (engine/engine_util_misc.h:332)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_potential(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    // SAFETY: caller guarantees poly points to at least n contiguous f64
    unsafe {
        let x = if flg_odd != 0 { x.abs() } else { x };
        let mut res = 0.5 * linear * (x * x);

        let mut xpow = x;
        for i in 0..n as usize {
            xpow *= x;
            res += *poly.add(i) / (i as f64 + 3.0) * (xpow * x);
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

