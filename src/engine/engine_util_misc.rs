//! Port of: engine/engine_util_misc.c
//! IR hash: 05737965add36adb
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
    // SAFETY: caller guarantees pnt[4] and end[4] are valid buffers
    unsafe {
        const MJMINVAL: f64 = 1e-15;
        // algorithm parameters
        let maxiter: i32 = 20;
        let zinit: f64 = 1.0 - 1e-7;
        let tolerance: f64 = 1e-6;

        // constants
        let len0: f64 = crate::engine::engine_util_blas::mju_norm(end, 2);
        let len1: f64 = crate::engine::engine_util_blas::mju_norm(end.add(2), 2);
        let dif: [f64; 2] = [*end.add(2) - *end.add(0), *end.add(3) - *end.add(1)];
        let dd: f64 = dif[0] * dif[0] + dif[1] * dif[1];

        // either point inside circle or circle too small: no wrap
        if len0 <= radius || len1 <= radius || radius < MJMINVAL || len0 < MJMINVAL || len1 < MJMINVAL {
            return -1.0;
        }

        // segment-circle intersection: no wrap
        if dd > MJMINVAL {
            // find nearest point on line segment to origin: d0 + a*dif
            let a: f64 = -(dif[0] * *end.add(0) + dif[1] * *end.add(1)) / dd;

            // in segment
            if a > 0.0 && a < 1.0 {
                let mut tmp: [f64; 2] = [0.0; 2];
                crate::engine::engine_util_blas::mju_add_scl(tmp.as_mut_ptr(), end, dif.as_ptr(), a, 2);
                if crate::engine::engine_util_blas::mju_norm(tmp.as_ptr(), 2) <= radius {
                    return -1.0;
                }
            }
        }

        // prepare default in case of numerical failure: average
        *pnt.add(0) = 0.5 * (*end.add(0) + *end.add(2));
        *pnt.add(1) = 0.5 * (*end.add(1) + *end.add(3));
        crate::engine::engine_util_blas::mju_normalize(pnt, 2);
        crate::engine::engine_util_blas::mju_scl(pnt, pnt, radius, 2);
        *pnt.add(2) = *pnt.add(0);
        *pnt.add(3) = *pnt.add(1);

        // compute function parameters: asin(A*z) + asin(B*z) - 2*asin(z) + G = 0
        let A: f64 = radius / len0;
        let B: f64 = radius / len1;
        let cosG: f64 = (len0 * len0 + len1 * len1 - dd) / (2.0 * len0 * len1);
        if cosG < -1.0 + MJMINVAL {
            return -1.0;
        } else if cosG > 1.0 - MJMINVAL {
            return 0.0;
        }
        let G: f64 = cosG.acos();

        // init
        let mut z: f64 = zinit;
        let mut f: f64 = (A * z).asin() + (B * z).asin() - 2.0 * z.asin() + G;

        // make sure init is not on the other side
        if f > 0.0 {
            return 0.0;
        }

        // Newton method
        let mut iter: i32 = 0;
        while iter < maxiter && f.abs() > tolerance {
            // derivative
            let df: f64 = A / crate::engine::engine_util_misc::mju_max(MJMINVAL, (1.0 - z * z * A * A).sqrt())
                        + B / crate::engine::engine_util_misc::mju_max(MJMINVAL, (1.0 - z * z * B * B).sqrt())
                        - 2.0 / crate::engine::engine_util_misc::mju_max(MJMINVAL, (1.0 - z * z).sqrt());

            // check sign; SHOULD NOT OCCUR
            if df > -MJMINVAL {
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
            f = (A * z).asin() + (B * z).asin() - 2.0 * z.asin() + G;

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
            crate::engine::engine_util_blas::mju_copy(vec.as_mut_ptr(), end, 2);
            ang = z.asin() - (A * z).asin();
        } else {
            crate::engine::engine_util_blas::mju_copy(vec.as_mut_ptr(), end.add(2), 2);
            ang = z.asin() - (B * z).asin();
        }
        crate::engine::engine_util_blas::mju_normalize(vec.as_mut_ptr(), 2);
        *pnt.add(0) = radius * (ang.cos() * vec[0] - ang.sin() * vec[1]);
        *pnt.add(1) = radius * (ang.sin() * vec[0] + ang.cos() * vec[1]);
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
    extern "C" {
        fn flexInterpRotation_impl(order: i32, xpos_c: *const f64, local: *const f64, quat: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { flexInterpRotation_impl(order, xpos_c, local, quat) }
}

/// C: nodeAt (engine/engine_util_misc.c:902)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn node_at(nodexpos: *const f64, ny: i32, nz: i32, i: i32, j: i32, k: i32) -> *const f64 {
    // WARNING: signature changed — verify body
    // Previous params: (nodexpos : * const f64, ny : i32, nz : i32, i : i32, j : i32, k : i32)
    // Previous return: * const f64
    unsafe { nodexpos . add (3 * (i * ny * nz + j * nz + k) as usize) }
}

/// C: addWeight (engine/engine_util_misc.c:984)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_weight(nb: *mut i32, body: *mut i32, bweight: *mut f64, b: i32, w: f64) {
    // SAFETY: nb, body, bweight are valid pointers per caller contract
    unsafe {
        let n = *nb;
        for i in 0..n as usize {
            if *body.add(i) == b {
                if !bweight.is_null() {
                    *bweight.add(i) += w;
                }
                return;
            }
        }
        *body.add(n as usize) = b;
        if !bweight.is_null() {
            *bweight.add(n as usize) = w;
        }
        *nb += 1;
    }
}

/// C: _decode (engine/engine_util_misc.c:1217)
#[allow(unused_variables, non_snake_case)]
pub fn decode(ch: i8) -> u32 {
    // WARNING: signature changed — verify body
    // Previous params: (ch : i8)
    // Previous return: u32
    let c = ch as u8 ; if c >= b'A' && c <= b'Z' { return (c - b'A') as u32 ; } if c >= b'a' && c <= b'z' { return (c - b'a') as u32 + 26 ; } if c >= b'0' && c <= b'9' { return (c - b'0') as u32 + 52 ; } if c == b'+' { return 62 ; } if c == b'/' { return 63 ; } 0
}

/// C: historyPhysicalIndex (engine/engine_util_misc.c:1359)
#[allow(unused_variables, non_snake_case)]
pub fn history_physical_index(cursor: i32, n: i32, logical: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (cursor : i32, n : i32, logical : i32)
    // Previous return: i32
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
    // WARNING: signature changed — verify body
    // Previous params: (times : * const f64, n : i32, cursor : i32, t : f64)
    // Previous return: i32
    unsafe { let oldest_phys = history_physical_index (cursor , n , 0) ; let newest_phys = history_physical_index (cursor , n , n - 1) ; let t_oldest = * times . add (oldest_phys as usize) ; let t_newest = * times . add (newest_phys as usize) ; if t <= t_oldest { return 0 ; } if t > t_newest { return n ; } let mut lo : i32 = 0 ; let mut hi : i32 = n - 1 ; while hi - lo > 1 { let mid = (lo + hi) / 2 ; let mid_phys = history_physical_index (cursor , n , mid) ; if * times . add (mid_phys as usize) < t { lo = mid ; } else { hi = mid ; } } hi }
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
    extern "C" {
        fn mju_wrap_impl(wpnt: *mut f64, x0: *const f64, x1: *const f64, xpos: *const f64, xmat: *const f64, radius: f64, r#type: i32, side: *const f64) -> f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_wrap_impl(wpnt, x0, x1, xpos, xmat, radius, r#type, side) }
}

/// C: mju_muscleGainLength (engine/engine_util_misc.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_gain_length(length: f64, lmin: f64, lmax: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (length : f64, lmin : f64, lmax : f64)
    // Previous return: f64
    const mjMINVAL : f64 = 1e-15 ; if lmin <= length && length <= lmax { let a = 0.5 * (lmin + 1.0) ; let b = 0.5 * (1.0 + lmax) ; if length <= a { let denom = if a - lmin > mjMINVAL { a - lmin } else { mjMINVAL } ; let x = (length - lmin) / denom ; 0.5 * x * x } else if length <= 1.0 { let denom = if 1.0 - a > mjMINVAL { 1.0 - a } else { mjMINVAL } ; let x = (1.0 - length) / denom ; 1.0 - 0.5 * x * x } else if length <= b { let denom = if b - 1.0 > mjMINVAL { b - 1.0 } else { mjMINVAL } ; let x = (length - 1.0) / denom ; 1.0 - 0.5 * x * x } else { let denom = if lmax - b > mjMINVAL { lmax - b } else { mjMINVAL } ; let x = (lmax - length) / denom ; 0.5 * x * x } } else { 0.0 }
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
    // WARNING: signature changed — verify body
    // Previous params: (len : f64, vel : f64, lengthrange : * const f64, acc0 : f64, prm : * const f64)
    // Previous return: f64
    const mjMINVAL : f64 = 1e-15 ; unsafe { let range0 = * prm . add (0) ; let range1 = * prm . add (1) ; let mut force = * prm . add (2) ; let scale = * prm . add (3) ; let lmin = * prm . add (4) ; let lmax = * prm . add (5) ; let vmax = * prm . add (6) ; let fvmax = * prm . add (8) ; if force < 0.0 { let denom = if acc0 > mjMINVAL { acc0 } else { mjMINVAL } ; force = scale / denom ; } let denom = if range1 - range0 > mjMINVAL { range1 - range0 } else { mjMINVAL } ; let L0 = (* lengthrange . add (1) - * lengthrange . add (0)) / denom ; let denom_L0 = if L0 > mjMINVAL { L0 } else { mjMINVAL } ; let L = range0 + (len - * lengthrange . add (0)) / denom_L0 ; let l0vmax = L0 * vmax ; let denom_v = if l0vmax > mjMINVAL { l0vmax } else { mjMINVAL } ; let V = vel / denom_v ; let FL = mju_muscle_gain_length (L , lmin , lmax) ; let y = fvmax - 1.0 ; let FV : f64 ; if V <= - 1.0 { FV = 0.0 ; } else if V <= 0.0 { FV = (V + 1.0) * (V + 1.0) ; } else if V <= y { let denom_y = if y > mjMINVAL { y } else { mjMINVAL } ; FV = fvmax - (y - V) * (y - V) / denom_y ; } else { FV = fvmax ; } - force * FL * FV }
}

/// C: mju_muscleBias (engine/engine_util_misc.h:43)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_muscle_bias(len: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (len : f64, lengthrange : * const f64, acc0 : f64, prm : * const f64)
    // Previous return: f64
    const mjMINVAL : f64 = 1e-15 ; unsafe { let range0 = * prm . add (0) ; let range1 = * prm . add (1) ; let mut force = * prm . add (2) ; let scale = * prm . add (3) ; let lmax = * prm . add (5) ; let fpmax = * prm . add (7) ; if force < 0.0 { let denom = if acc0 > mjMINVAL { acc0 } else { mjMINVAL } ; force = scale / denom ; } let denom = if range1 - range0 > mjMINVAL { range1 - range0 } else { mjMINVAL } ; let L0 = (* lengthrange . add (1) - * lengthrange . add (0)) / denom ; let denom_L0 = if L0 > mjMINVAL { L0 } else { mjMINVAL } ; let L = range0 + (len - * lengthrange . add (0)) / denom_L0 ; let b = 0.5 * (1.0 + lmax) ; if L <= 1.0 { 0.0 } else if L <= b { let denom_b = if b - 1.0 > mjMINVAL { b - 1.0 } else { mjMINVAL } ; let x = (L - 1.0) / denom_b ; - force * fpmax * 0.5 * x * x } else { let denom_b = if b - 1.0 > mjMINVAL { b - 1.0 } else { mjMINVAL } ; let x = (L - b) / denom_b ; - force * fpmax * (0.5 + x) } }
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
    // WARNING: signature changed — verify body
    // Previous params: (dctrl : f64, tau_act : f64, tau_deact : f64, smoothing_width : f64)
    // Previous return: f64
    const mjMINVAL : f64 = 1e-15 ; if smoothing_width < mjMINVAL { if dctrl > 0.0 { tau_act } else { tau_deact } } else { tau_deact + (tau_act - tau_deact) * mju_sigmoid (dctrl / smoothing_width + 0.5) }
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
    // WARNING: signature changed — verify body
    // Previous params: (ctrl : f64, act : f64, prm : * const f64)
    // Previous return: f64
    const mjMINVAL : f64 = 1e-15 ; unsafe { let ctrlclamp = mju_clip (ctrl , 0.0 , 1.0) ; let actclamp = mju_clip (act , 0.0 , 1.0) ; let tau_act = * prm . add (0) * (0.5 + 1.5 * actclamp) ; let tau_deact = * prm . add (1) / (0.5 + 1.5 * actclamp) ; let smoothing_width = * prm . add (2) ; let dctrl = ctrlclamp - act ; let tau = mju_muscle_dynamics_timescale (dctrl , tau_act , tau_deact , smoothing_width) ; let denom = if tau > mjMINVAL { tau } else { mjMINVAL } ; dctrl / denom }
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
    // WARNING: signature changed — verify body
    // Previous params: (velocity : f64, F_C : f64, F_S : f64, v_S : f64)
    // Previous return: f64
    const mjMINVAL : f64 = 1e-15 ; let denom = crate :: engine :: engine_util_misc :: mju_max (mjMINVAL , v_S) ; let ratio = velocity / denom ; F_C + (F_S - F_C) * (- ratio * ratio) . exp ()
}

/// C: mj_dcmotorSlots (engine/engine_util_misc.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_dcmotor_slots(dynprm: *const f64, gainprm: *const f64) -> mjDCMotorSlots {
    // WARNING: signature changed — verify body
    // Previous params: (dynprm : * const f64, gainprm : * const f64)
    // Previous return: mjDCMotorSlots
    extern "C" { fn mj_dcmotorSlots_impl (dynprm : * const f64 , gainprm : * const f64) -> mjDCMotorSlots ; } unsafe { mj_dcmotorSlots_impl (dynprm , gainprm) }
}

/// C: mju_geomSemiAxes (engine/engine_util_misc.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_geom_semi_axes(semiaxes: *mut f64, size: *const f64, r#type: mjtGeom) {
    extern "C" {
        fn mju_geomSemiAxes_impl(semiaxes: *mut f64, size: *const f64, r#type: mjtGeom);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_geomSemiAxes_impl(semiaxes, size, r#type) }
}

/// C: mju_insideGeom (engine/engine_util_misc.h:74)
/// Calls: mju_clip, mju_dot3, mju_mulMatTVec3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_inside_geom(pos: *const f64, mat: *const f64, size: *const f64, r#type: mjtGeom, point: *const f64) -> i32 {
    extern "C" {
        fn mju_insideGeom_impl(pos: *const f64, mat: *const f64, size: *const f64, r#type: mjtGeom, point: *const f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_insideGeom_impl(pos, mat, size, r#type, point) }
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
    extern "C" { fn mju_camPixelRay_impl(origin: *mut f64, direction: *mut f64, cam_xpos: *const f64, cam_xmat: *const f64, col: i32, row: i32, fx: f64, fy: f64, cx: f64, cy: f64, projection: i32, ortho_extent: f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_camPixelRay_impl(origin, direction, cam_xpos, cam_xmat, col, row, fx, fy, cx, cy, projection, ortho_extent) }
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
    extern "C" {
        fn mju_defGradient_impl(res: *mut f64, p: *const f64, dof: *const f64, order: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_defGradient_impl(res, p, dof, order) }
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
    // SAFETY: caller guarantees x points to at least 3 elements
    unsafe {
        if order == 1 {
            return mju_flex_phi(*x.add(2), i & 1, order)
                 * mju_flex_phi(*x.add(1), i & 2, order)
                 * mju_flex_phi(*x.add(0), i & 4, order);
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
    // SAFETY: caller guarantees basis has enough space for (order+1)^3 elements, x has 3 elements
    unsafe {
        if order == 1 {
            let p: [[f64; 2]; 3] = [
                [1.0 - *x.add(0), *x.add(0)],
                [1.0 - *x.add(1), *x.add(1)],
                [1.0 - *x.add(2), *x.add(2)],
            ];
            let mut j: i32 = 0;
            for i0 in 0..2 {
                let w0 = p[0][i0];
                for i1 in 0..2 {
                    let w01 = w0 * p[1][i1];
                    for i2 in 0..2 {
                        *basis.add(j as usize) = w01 * p[2][i2];
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
            let mut j: i32 = 0;
            for i0 in 0..3 {
                let w0 = p[0][i0];
                for i1 in 0..3 {
                    let w01 = w0 * p[1][i1];
                    for i2 in 0..3 {
                        *basis.add(j as usize) = w01 * p[2][i2];
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
pub fn mju_cell_lookup(coord: *const f64, cellnum: [i32; 3], order: i32, local: *mut f64, nodeindices: *mut i32) -> i32 {
    // SAFETY: caller guarantees valid coord[3], local[3], and nodeindices buffer
    unsafe {
        let cx = cellnum[0];
        let cy = cellnum[1];
        let cz = cellnum[2];

        // find containing cell
        let mut ci = (*coord.add(0) * cx as f64).floor() as i32;
        let mut cj = (*coord.add(1) * cy as f64).floor() as i32;
        let mut ck = (*coord.add(2) * cz as f64).floor() as i32;
        ci = if ci < cx - 1 { ci } else { cx - 1 };
        ci = if ci > 0 { ci } else { 0 };
        cj = if cj < cy - 1 { cj } else { cy - 1 };
        cj = if cj > 0 { cj } else { 0 };
        ck = if ck < cz - 1 { ck } else { cz - 1 };
        ck = if ck > 0 { ck } else { 0 };

        // local parametric coordinates within cell
        *local.add(0) = crate::engine::engine_util_misc::mju_clip(*coord.add(0) * cx as f64 - ci as f64, 0.0, 1.0);
        *local.add(1) = crate::engine::engine_util_misc::mju_clip(*coord.add(1) * cy as f64 - cj as f64, 0.0, 1.0);
        *local.add(2) = crate::engine::engine_util_misc::mju_clip(*coord.add(2) * cz as f64 - ck as f64, 0.0, 1.0);

        // build node indices for this cell
        if !nodeindices.is_null() {
            let gi_base = ci * order;
            let gj_base = cj * order;
            let gk_base = ck * order;
            let ny_g = cy * order + 1;
            let nz_g = cz * order + 1;
            let mut ni: i32 = 0;
            for li in 0..=order {
                let gi = gi_base + li;
                let gi_stride = gi * ny_g * nz_g;
                for lj in 0..=order {
                    let gj = gj_base + lj;
                    let gj_stride = gi_stride + gj * nz_g;
                    for lk in 0..=order {
                        let gk = gk_base + lk;
                        *nodeindices.add(ni as usize) = gj_stride + gk;
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
    // SAFETY: res, x, coeff are valid pointers per caller contract
    unsafe {
        let npoint: i32 = (order + 1) * (order + 1) * (order + 1);

        if npoint > 27 {
            for j in 0..npoint {
                let idx = if !nodeindices.is_null() { *nodeindices.add(j as usize) } else { j };
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    res,
                    coeff.add(3 * idx as usize),
                    mju_eval_basis(x, j, order),
                );
            }
            return;
        }

        let mut basis = [0.0f64; 27];
        mju_eval_basis_array(basis.as_mut_ptr(), x, order);

        for j in 0..npoint {
            let idx = if !nodeindices.is_null() { *nodeindices.add(j as usize) } else { j };
            crate::engine::engine_util_blas::mju_add_to_scl3(
                res,
                coeff.add(3 * idx as usize),
                basis[j as usize],
            );
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
    extern "C" {
        fn mju_flexGatherCellState_impl(order: i32, cy: i32, cz: i32, ci: i32, cj: i32, ck: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_c: *mut f64, vel_c: *mut f64, xpos0_c: *mut f64, nodeindices: *mut i32, quat: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_flexGatherCellState_impl(order, cy, cz, ci, cj, ck, xpos_g, vel_g, xpos0_g, xpos_c, vel_c, xpos0_c, nodeindices, quat) }
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
    extern "C" {
        fn mju_flexGatherFaceState_impl(order: i32, cx: i32, cy: i32, cz: i32, face_elem_idx: i32, xpos_g: *const f64, vel_g: *const f64, xpos0_g: *const f64, xpos_f: *mut f64, vel_f: *mut f64, xpos0_f: *mut f64, nodeindices: *mut i32, quat: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_flexGatherFaceState_impl(order, cx, cy, cz, face_elem_idx, xpos_g, vel_g, xpos0_g, xpos_f, vel_f, xpos0_f, nodeindices, quat) }
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
    extern "C" {
        fn mju_flexInterpRotation2D_impl(order: i32, xpos_f: *const f64, npe: i32, axis0: i32, axis1: i32, normal_axis: i32, local: *const f64, quat: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_flexInterpRotation2D_impl(order, xpos_f, npe, axis0, axis1, normal_axis, local, quat) }
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
    extern "C" {
        fn mju_flexFaceNormal2D_impl(normal: *mut f64, t1: *mut f64, t2: *mut f64, order: i32, xpos_f: *const f64, local: *const f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_flexFaceNormal2D_impl(normal, t1, t2, order, xpos_f, local) }
}

/// C: mju_flexPhi (engine/engine_util_misc.h:130)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_phi(s: f64, i: i32, order: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (s : f64, i : i32, order : i32)
    // Previous return: f64
    if order == 1 { return if i == 0 { 1.0 - s } else { s } ; } match i { 0 => 2.0 * s * s - 3.0 * s + 1.0 , 1 => 4.0 * (s - s * s) , 2 => 2.0 * s * s - s , _ => 0.0 , }
}

/// C: mju_flexDphi (engine/engine_util_misc.h:141)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_dphi(s: f64, i: i32, order: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (s : f64, i : i32, order : i32)
    // Previous return: f64
    if order == 1 { return if i == 0 { - 1.0 } else { 1.0 } ; } match i { 0 => 4.0 * s - 3.0 , 1 => 4.0 * (1.0 - 2.0 * s) , 2 => 4.0 * s - 1.0 , _ => 0.0 , }
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
    // SAFETY: caller guarantees nodexpos has at least nx*ny*nz*3 elements
    unsafe {
        // need at least 3 nodes in each direction to have interior nodes
        if nx < 3 || ny < 3 || nz < 3 {
            return;
        }

        // helper: get nodexpos pointer for node (i,j,k)
        #[inline(always)]
        unsafe fn node_at(nodexpos: *mut f64, ny: i32, nz: i32, i: i32, j: i32, k: i32) -> *const f64 {
            nodexpos.add(3 * (i * ny * nz + j * nz + k) as usize) as *const f64
        }

        for i in 1..nx-1 {
            for j in 1..ny-1 {
                for k in 1..nz-1 {
                    // parametric coordinates in [0, 1]
                    let s: f64 = i as f64 / (nx - 1) as f64;
                    let t: f64 = j as f64 / (ny - 1) as f64;
                    let u: f64 = k as f64 / (nz - 1) as f64;

                    let mut result: [f64; 3] = [0.0, 0.0, 0.0];

                    // --- face contributions (bilinear interpolation on each face pair) ---
                    // x-faces: i=0 and i=nx-1
                    for d in 0..3usize {
                        result[d] += (1.0 - s) * *node_at(nodexpos, ny, nz, 0, j, k).add(d)
                                   +        s  * *node_at(nodexpos, ny, nz, nx-1, j, k).add(d);
                    }
                    // y-faces: j=0 and j=ny-1
                    for d in 0..3usize {
                        result[d] += (1.0 - t) * *node_at(nodexpos, ny, nz, i, 0, k).add(d)
                                   +        t  * *node_at(nodexpos, ny, nz, i, ny-1, k).add(d);
                    }
                    // z-faces: k=0 and k=nz-1
                    for d in 0..3usize {
                        result[d] += (1.0 - u) * *node_at(nodexpos, ny, nz, i, j, 0).add(d)
                                   +        u  * *node_at(nodexpos, ny, nz, i, j, nz-1).add(d);
                    }

                    // --- edge corrections (subtract 12 edges) ---
                    // edges along x
                    for d in 0..3usize {
                        result[d] -= (1.0 - t) * (1.0 - u) * *node_at(nodexpos, ny, nz, i, 0, 0).add(d);
                        result[d] -= (1.0 - t) *        u  * *node_at(nodexpos, ny, nz, i, 0, nz-1).add(d);
                        result[d] -=        t  * (1.0 - u) * *node_at(nodexpos, ny, nz, i, ny-1, 0).add(d);
                        result[d] -=        t  *        u  * *node_at(nodexpos, ny, nz, i, ny-1, nz-1).add(d);
                    }
                    // edges along y
                    for d in 0..3usize {
                        result[d] -= (1.0 - s) * (1.0 - u) * *node_at(nodexpos, ny, nz, 0, j, 0).add(d);
                        result[d] -= (1.0 - s) *        u  * *node_at(nodexpos, ny, nz, 0, j, nz-1).add(d);
                        result[d] -=        s  * (1.0 - u) * *node_at(nodexpos, ny, nz, nx-1, j, 0).add(d);
                        result[d] -=        s  *        u  * *node_at(nodexpos, ny, nz, nx-1, j, nz-1).add(d);
                    }
                    // edges along z
                    for d in 0..3usize {
                        result[d] -= (1.0 - s) * (1.0 - t) * *node_at(nodexpos, ny, nz, 0, 0, k).add(d);
                        result[d] -= (1.0 - s) *        t  * *node_at(nodexpos, ny, nz, 0, ny-1, k).add(d);
                        result[d] -=        s  * (1.0 - t) * *node_at(nodexpos, ny, nz, nx-1, 0, k).add(d);
                        result[d] -=        s  *        t  * *node_at(nodexpos, ny, nz, nx-1, ny-1, k).add(d);
                    }

                    // --- corner corrections (add 8 corners back) ---
                    for d in 0..3usize {
                        result[d] += (1.0 - s) * (1.0 - t) * (1.0 - u) * *node_at(nodexpos, ny, nz, 0, 0, 0).add(d);
                        result[d] += (1.0 - s) * (1.0 - t) *        u  * *node_at(nodexpos, ny, nz, 0, 0, nz-1).add(d);
                        result[d] += (1.0 - s) *        t  * (1.0 - u) * *node_at(nodexpos, ny, nz, 0, ny-1, 0).add(d);
                        result[d] += (1.0 - s) *        t  *        u  * *node_at(nodexpos, ny, nz, 0, ny-1, nz-1).add(d);
                        result[d] +=        s  * (1.0 - t) * (1.0 - u) * *node_at(nodexpos, ny, nz, nx-1, 0, 0).add(d);
                        result[d] +=        s  * (1.0 - t) *        u  * *node_at(nodexpos, ny, nz, nx-1, 0, nz-1).add(d);
                        result[d] +=        s  *        t  * (1.0 - u) * *node_at(nodexpos, ny, nz, nx-1, ny-1, 0).add(d);
                        result[d] +=        s  *        t  *        u  * *node_at(nodexpos, ny, nz, nx-1, ny-1, nz-1).add(d);
                    }

                    // write result to interior node
                    crate::engine::engine_util_blas::mju_copy3(
                        nodexpos.add(3 * (i * ny * nz + j * nz + k) as usize),
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
    extern "C" {
        fn mju_shellTFIWeights_impl(nx: i32, ny: i32, nz: i32, i: i32, j: i32, k: i32, w: f64, nb: *mut i32, body: *mut i32, bweight: *mut f64, nodebodyid: *const i32, nstart: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_shellTFIWeights_impl(nx, ny, nz, i, j, k, w, nb, body, bweight, nodebodyid, nstart) }
}

/// C: mju_encodeBase64 (engine/engine_util_misc.h:163)
#[allow(unused_variables, non_snake_case)]
pub fn mju_encode_base64(buf: *mut i8, data: *const u8, ndata: usize) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (buf : * mut i8, data : * const u8, ndata : usize)
    // Previous return: usize
    unsafe { const TABLE : & [u8 ; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" ; let mut i : usize = 0 ; let mut j : usize = 0 ; while i + 3 <= ndata { let byte_1 : u32 = * data . add (i) as u32 ; i += 1 ; let byte_2 : u32 = * data . add (i) as u32 ; i += 1 ; let byte_3 : u32 = * data . add (i) as u32 ; i += 1 ; let k : u32 = (byte_1 << 16) | (byte_2 << 8) | byte_3 ; * buf . add (j) = TABLE [((k >> 18) & 63) as usize] as i8 ; j += 1 ; * buf . add (j) = TABLE [((k >> 12) & 63) as usize] as i8 ; j += 1 ; * buf . add (j) = TABLE [((k >> 6) & 63) as usize] as i8 ; j += 1 ; * buf . add (j) = TABLE [((k >> 0) & 63) as usize] as i8 ; j += 1 ; } if i + 1 == ndata { let byte_1 : u32 = * data . add (i) as u32 ; let k : u32 = byte_1 << 16 ; * buf . add (j) = TABLE [((k >> 18) & 63) as usize] as i8 ; j += 1 ; * buf . add (j) = TABLE [((k >> 12) & 63) as usize] as i8 ; j += 1 ; * buf . add (j) = b'=' as i8 ; j += 1 ; * buf . add (j) = b'=' as i8 ; j += 1 ; } if i + 2 == ndata { let byte_1 : u32 = * data . add (i) as u32 ; i += 1 ; let byte_2 : u32 = * data . add (i) as u32 ; let k : u32 = (byte_1 << 16) + (byte_2 << 8) ; * buf . add (j) = TABLE [((k >> 18) & 63) as usize] as i8 ; j += 1 ; * buf . add (j) = TABLE [((k >> 12) & 63) as usize] as i8 ; j += 1 ; * buf . add (j) = TABLE [((k >> 6) & 63) as usize] as i8 ; j += 1 ; * buf . add (j) = b'=' as i8 ; j += 1 ; } * buf . add (j) = 0 ; 4 * ((ndata + 2) / 3) + 1 }
}

/// C: mju_isValidBase64 (engine/engine_util_misc.h:167)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_valid_base64(s: *const i8) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const i8)
    // Previous return: usize
    unsafe { let mut i : usize = 0 ; let mut pad : i32 = 0 ; while * s . add (i) != 0 && * s . add (i) != b'=' as i8 { let c = * s . add (i) as u8 ; let is_alnum = (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z') || (c >= b'0' && c <= b'9') ; if ! is_alnum && c != b'/' && c != b'+' { return 0 ; } i += 1 ; } if * s . add (i) == b'=' as i8 { if * s . add (i + 1) == 0 { pad = 1 ; } else if * s . add (i + 1) == b'=' as i8 && * s . add (i + 2) == 0 { pad = 2 ; } else { return 0 ; } } let len : i32 = i as i32 + pad ; if len % 4 != 0 { 0 } else { (3 * (len / 4) - pad) as usize } }
}

/// C: mju_decodeBase64 (engine/engine_util_misc.h:171)
/// Calls: _decode
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_base64(buf: *mut u8, s: *const i8) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (buf : * mut u8, s : * const i8)
    // Previous return: usize
    unsafe { let mut i : usize = 0 ; let mut j : usize = 0 ; while * s . add (i) != 0 { let char_1 : u32 = decode (* s . add (i)) ; i += 1 ; let char_2 : u32 = decode (* s . add (i)) ; i += 1 ; let char_3 : u32 = decode (* s . add (i)) ; i += 1 ; let char_4 : u32 = decode (* s . add (i)) ; i += 1 ; let k : u32 = (char_1 << 18) | (char_2 << 12) | (char_3 << 6) | char_4 ; * buf . add (j) = ((k >> 16) & 0xFF) as u8 ; j += 1 ; if * s . add (i - 2) != b'=' as i8 { * buf . add (j) = ((k >> 8) & 0xFF) as u8 ; j += 1 ; } if * s . add (i - 1) != b'=' as i8 { * buf . add (j) = (k & 0xFF) as u8 ; j += 1 ; } } j }
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
    extern "C" { fn mju_historyInsert_impl(buf: *mut f64, n: i32, dim: i32, t: f64) -> *mut f64; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_historyInsert_impl(buf, n, dim, t) }
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
    extern "C" { fn mju_historyRead_impl (buf : * const f64 , n : i32 , dim : i32 , res : * mut f64 , t : f64 , interp : i32) -> * const f64 ; } unsafe { mju_historyRead_impl (buf , n , dim , res , t , interp) }
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
    // WARNING: signature changed — verify body
    // Previous params: (pyramid : * mut f64, force : * const f64, mu : * const f64, dim : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_decodePyramid (engine/engine_util_misc.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_pyramid(force: *mut f64, pyramid: *const f64, mu: *const f64, dim: i32) {
    extern "C" {
        fn mju_decodePyramid_impl(force: *mut f64, pyramid: *const f64, mu: *const f64, dim: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_decodePyramid_impl(force, pyramid, mu, dim) }
}

/// C: mju_springDamper (engine/engine_util_misc.h:208)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_spring_damper(pos0: f64, vel0: f64, Kp: f64, Kv: f64, dt: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (pos0 : f64, vel0 : f64, Kp : f64, Kv : f64, dt : f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (point : * const f64, pos : * const f64, mat : * const f64, size : * const f64, inflate : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mju_printMat (engine/engine_util_misc.h:217)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat(mat: *const f64, nr: i32, nc: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (mat : * const f64, nr : i32, nc : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_printMatSparse (engine/engine_util_misc.h:220)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_print_mat_sparse(mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (mat : * const f64, nr : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_min (engine/engine_util_misc.h:225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_min(a: f64, b: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (a : f64, b : f64)
    // Previous return: f64
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
    // WARNING: signature changed — verify body
    // Previous params: (a : f64, b : f64)
    // Previous return: f64
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
    // WARNING: signature changed — verify body
    // Previous params: (x : f64, min : f64, max : f64)
    // Previous return: f64
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
    // WARNING: signature changed — verify body
    // Previous params: (x : f64)
    // Previous return: f64
    if x < 0.0 { - 1.0 } else if x > 0.0 { 1.0 } else { 0.0 }
}

/// C: mju_round (engine/engine_util_misc.h:237)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_round(x: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (x : f64)
    // Previous return: i32
    let lower = x . floor () ; let upper = x . ceil () ; if x - lower < upper - x { lower as i32 } else { upper as i32 }
}

/// C: mju_type2Str (engine/engine_util_misc.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn mju_type2str(r#type: i32) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (r#type : i32)
    // Previous return: * const i8
    extern "C" { fn mju_type2Str_impl (r#type : i32) -> * const i8 ; } unsafe { mju_type2Str_impl (r#type) }
}

/// C: mju_str2Type (engine/engine_util_misc.h:243)
#[allow(unused_variables, non_snake_case)]
pub fn mju_str2type(str: *const i8) -> i32 {
    extern "C" { fn mju_str2Type_impl(str: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mju_str2Type_impl(str) }
}

/// C: mju_writeNumBytes (engine/engine_util_misc.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_num_bytes(nbytes: usize) -> *const i8 {
    extern "C" { fn mju_writeNumBytes_impl(nbytes: usize) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mju_writeNumBytes_impl(nbytes) }
}

/// C: mju_warningText (engine/engine_util_misc.h:249)
/// Calls: mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning_text(warning: i32, info: usize) -> *const i8 {
    extern "C" { fn mju_warningText_impl(warning: i32, info: usize) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mju_warningText_impl(warning, info) }
}

/// C: mju_isBad (engine/engine_util_misc.h:252)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_bad(x: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (x : f64)
    // Previous return: i32
    const mjMAXVAL : f64 = 1e10 ; (x . is_nan () || x > mjMAXVAL || x < - mjMAXVAL) as i32
}

/// C: mju_isZero (engine/engine_util_misc.h:255)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero(vec: *const f64, n: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (vec : * const f64, n : i32)
    // Previous return: i32
    unsafe { for i in 0 .. n { if * vec . add (i as usize) != 0.0 { return 0 ; } } } 1
}

/// C: mju_isZeroByte (engine/engine_util_misc.h:258)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_zero_byte(vec: *const u8, n: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (vec : * const u8, n : i32)
    // Previous return: i32
    unsafe { if n == 0 { return 1 ; } if * vec != 0 { return 0 ; } for i in 1 .. n { if * vec . add (i as usize) != * vec { return 0 ; } } 1 }
}

/// C: mju_zeroInt (engine/engine_util_misc.h:261)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_int(res: *mut i32, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut i32, n : i32)
    // Previous return: ()
    unsafe { std :: ptr :: write_bytes (res , 0 , n as usize) ; }
}

/// C: mju_copyInt (engine/engine_util_misc.h:264)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_int(res: *mut i32, vec: *const i32, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut i32, vec : * const i32, n : i32)
    // Previous return: ()
    unsafe { std :: ptr :: copy_nonoverlapping (vec , res , n as usize) ; }
}

/// C: mju_fillInt (engine/engine_util_misc.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fill_int(res: *mut i32, val: i32, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut i32, val : i32, n : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n { * res . add (i as usize) = val ; } }
}

/// C: mju_standardNormal (engine/engine_util_misc.h:270)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_standard_normal(num2: *mut f64) -> f64 {
    extern "C" { fn mju_standardNormal_impl(num2: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_standardNormal_impl(num2) }
}

/// C: mju_f2n (engine/engine_util_misc.h:273)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_f2n(res: *mut f64, vec: *const f32, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f32, n : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n { * res . add (i as usize) = * vec . add (i as usize) as f64 ; } }
}

/// C: mju_n2f (engine/engine_util_misc.h:276)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_n2f(res: *mut f32, vec: *const f64, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f32, vec : * const f64, n : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n { * res . add (i as usize) = * vec . add (i as usize) as f32 ; } }
}

/// C: mju_d2n (engine/engine_util_misc.h:279)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_d2n(res: *mut f64, vec: *const f64, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, n : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n { * res . add (i as usize) = * vec . add (i as usize) ; } }
}

/// C: mju_n2d (engine/engine_util_misc.h:282)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_n2d(res: *mut f64, vec: *const f64, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, n : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n { * res . add (i as usize) = * vec . add (i as usize) ; } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, ind : * const i32, n : i32)
    // Previous return: ()
    unsafe { if ind . is_null () { crate :: engine :: engine_util_blas :: mju_copy (res , vec , n) ; return ; } for i in 0 .. n { * res . add (i as usize) = * vec . add (* ind . add (i as usize) as usize) ; } }
}

/// C: mju_gatherMasked (engine/engine_util_misc.h:288)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_masked(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, ind : * const i32, n : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n { let idx = * ind . add (i as usize) ; * res . add (i as usize) = if idx >= 0 { * vec . add (idx as usize) } else { 0.0 } ; } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, ind : * const i32, n : i32)
    // Previous return: ()
    unsafe { if ind . is_null () { crate :: engine :: engine_util_blas :: mju_copy (res , vec , n) ; return ; } for i in 0 .. n { * res . add (* ind . add (i as usize) as usize) = * vec . add (i as usize) ; } }
}

/// C: mju_gatherInt (engine/engine_util_misc.h:294)
#[allow(unused_variables, non_snake_case)]
pub fn mju_gather_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut i32, vec : * const i32, ind : * const i32, n : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n { * res . add (i as usize) = * vec . add (* ind . add (i as usize) as usize) ; } }
}

/// C: mju_scatterInt (engine/engine_util_misc.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scatter_int(res: *mut i32, vec: *const i32, ind: *const i32, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut i32, vec : * const i32, ind : * const i32, n : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n { * res . add (* ind . add (i as usize) as usize) = * vec . add (i as usize) ; } }
}

/// C: mju_sparseMap (engine/engine_util_misc.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sparse_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (map : * mut i32, nr : i32, res_rowadr : * const i32, res_rownnz : * const i32, res_colind : * const i32, src_rowadr : * const i32, src_rownnz : * const i32, src_colind : * const i32)
    // Previous return: ()
    unsafe { for i in 0 .. nr { let mut res_cursor = * res_rowadr . add (i as usize) ; let res_end = res_cursor + * res_rownnz . add (i as usize) ; let mut src_cursor = * src_rowadr . add (i as usize) ; let src_end = src_cursor + * src_rownnz . add (i as usize) ; while res_cursor < res_end { let res_col = * res_colind . add (res_cursor as usize) ; while src_cursor < src_end && * src_colind . add (src_cursor as usize) < res_col { src_cursor += 1 ; } * map . add (res_cursor as usize) = src_cursor ; res_cursor += 1 ; src_cursor += 1 ; } } }
}

/// C: mju_lower2SymMap (engine/engine_util_misc.h:306)
/// Calls: mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_lower2sym_map(map: *mut i32, nr: i32, res_rowadr: *const i32, res_rownnz: *const i32, res_colind: *const i32, src_rowadr: *const i32, src_rownnz: *const i32, src_colind: *const i32, cursor: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (map : * mut i32, nr : i32, res_rowadr : * const i32, res_rownnz : * const i32, res_colind : * const i32, src_rowadr : * const i32, src_rownnz : * const i32, src_colind : * const i32, cursor : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_insertionSort (engine/engine_util_misc.h:312)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort(list: *mut f64, n: i32) {
    extern "C" { fn mju_insertionSort_impl(list: *mut f64, n: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mju_insertionSort_impl(list, n) }
}

/// C: mju_insertionSortInt (engine/engine_util_misc.h:315)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort_int(list: *mut i32, n: i32) {
    extern "C" { fn mju_insertionSortInt_impl(list: *mut i32, n: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_insertionSortInt_impl(list, n) }
}

/// C: mju_Halton (engine/engine_util_misc.h:318)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_halton(index: i32, base: i32) -> f64 {
    // SAFETY: pure arithmetic, no pointer operations
    let mut n0 = index;
    let b = base as f64;
    let mut f = 1.0 / b;
    let mut hn: f64 = 0.0;

    while n0 > 0 {
        let n1 = n0 / base;
        let r = n0 - n1 * base;
        hn += f * r as f64;
        f /= b;
        n0 = n1;
    }

    hn
}

/// C: mju_strncpy (engine/engine_util_misc.h:321)
#[allow(unused_variables, non_snake_case)]
pub fn mju_strncpy(dst: *mut i8, src: *const i8, n: i32) -> *mut i8 {
    // WARNING: signature changed — verify body
    // Previous params: (dst : * mut i8, src : * const i8, n : i32)
    // Previous return: * mut i8
    unsafe { if ! dst . is_null () && ! src . is_null () && n > 0 { extern "C" { fn strncpy (dst : * mut i8 , src : * const i8 , n : usize) -> * mut i8 ; } strncpy (dst , src , n as usize) ; * dst . add ((n - 1) as usize) = 0 ; } dst }
}

/// C: mju_polyForce (engine/engine_util_misc.h:326)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (linear : f64, poly : * const f64, x : f64, n : i32, flg_odd : i32)
    // Previous return: f64
    unsafe { let x = if flg_odd != 0 { x . abs () } else { x } ; let mut res : f64 = linear ; let mut xpow : f64 = 1.0 ; for i in 0 .. n as usize { xpow *= x ; res += * poly . add (i) * xpow ; } res }
}

/// C: mjd_xPolyForce (engine/engine_util_misc.h:329)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_x_poly_force(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (linear : f64, poly : * const f64, x : f64, n : i32, flg_odd : i32)
    // Previous return: f64
    unsafe { let x = if flg_odd != 0 { x . abs () } else { x } ; let mut res : f64 = linear ; let mut xpow : f64 = 1.0 ; for i in 0 .. n as usize { xpow *= x ; res += (i as f64 + 2.0) * * poly . add (i) * xpow ; } res }
}

/// C: mju_polyPotential (engine/engine_util_misc.h:332)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_poly_potential(linear: f64, poly: *const f64, x: f64, n: i32, flg_odd: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (linear : f64, poly : * const f64, x : f64, n : i32, flg_odd : i32)
    // Previous return: f64
    unsafe { let x = if flg_odd != 0 { x . abs () } else { x } ; let mut res : f64 = 0.5 * linear * (x * x) ; let mut xpow : f64 = x ; for i in 0 .. n as usize { xpow *= x ; res += * poly . add (i) / (i as f64 + 3.0) * (xpow * x) ; } res }
}

/// C: mju_sigmoid (engine/engine_util_misc.h:335)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sigmoid(x: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (x : f64)
    // Previous return: f64
    if x <= 0.0 { return 0.0 ; } if x >= 1.0 { return 1.0 ; } x * x * x * (3.0 * x * (2.0 * x - 5.0) + 10.0)
}

