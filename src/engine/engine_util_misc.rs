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
    extern "C" { fn is_intersect(p1: *const f64, p2: *const f64, p3: *const f64, p4: *const f64) -> mjtBool; }
    // SAFETY: delegates to C implementation
    unsafe { is_intersect(p1, p2, p3, p4) }
}

/// C: length_circle (engine/engine_util_misc.c:55)
/// Calls: mju_dot, mju_normalize
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn length_circle(p0: *const f64, p1: *const f64, ind: i32, radius: f64) -> f64  {
    extern "C" { fn length_circle(p0: *const f64, p1: *const f64, ind: i32, radius: f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { length_circle(p0, p1, ind, radius) }
}

/// C: wrap_circle (engine/engine_util_misc.c:78)
/// Calls: is_intersect, length_circle, mju_add, mju_dot, mju_normalize, mju_sub
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_circle(pnt: *mut f64, end: *const f64, side: *const f64, radius: f64) -> f64  {
    extern "C" { fn wrap_circle(pnt: *mut f64, end: *const f64, side: *const f64, radius: f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { wrap_circle(pnt, end, side, radius) }
}

/// C: wrap_inside (engine/engine_util_misc.c:158)
/// Calls: mju_addScl, mju_copy, mju_max, mju_norm, mju_normalize, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_inside(pnt: *mut f64, end: *const f64, radius: f64) -> f64  {
    extern "C" { fn wrap_inside(pnt: *mut f64, end: *const f64, radius: f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { wrap_inside(pnt, end, radius) }
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
    extern "C" { fn flexInterpRotation(order: i32, xpos_c: *const f64, local: *const f64, quat: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { flexInterpRotation(order, xpos_c, local, quat) }
}

/// C: nodeAt (engine/engine_util_misc.c:902)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn node_at(nodexpos: *const f64, ny: i32, nz: i32, i: i32, j: i32, k: i32) -> *const f64  {
    extern "C" { fn nodeAt(nodexpos: *const f64, ny: i32, nz: i32, i: i32, j: i32, k: i32) -> *const f64; }
    // SAFETY: delegates to C implementation
    unsafe { nodeAt(nodexpos, ny, nz, i, j, k) }
}

/// C: addWeight (engine/engine_util_misc.c:984)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_weight(nb: *mut i32, body: *mut i32, bweight: *mut f64, b: i32, w: f64) {
    extern "C" { fn addWeight(nb: *mut i32, body: *mut i32, bweight: *mut f64, b: i32, w: f64); }
    // SAFETY: delegates to C implementation
    unsafe { addWeight(nb, body, bweight, b, w) }
}

/// C: _decode (engine/engine_util_misc.c:1217)
#[allow(unused_variables, non_snake_case)]
pub fn decode(ch: i8) -> u32  {
    extern "C" { fn _decode(ch: i8) -> u32; }
    // SAFETY: delegates to C implementation
    unsafe { _decode(ch) }
}

/// C: historyPhysicalIndex (engine/engine_util_misc.c:1359)
#[allow(unused_variables, non_snake_case)]
pub fn history_physical_index(cursor: i32, n: i32, logical: i32) -> i32  {
    // C: return (cursor - logical % n + n) % n;
    if n <= 0 {
        return 0;
    }
    let logical_mod = ((logical % n) + n) % n;
    ((cursor - logical_mod) % n + n) % n
}

/// C: historyFindIndex (engine/engine_util_misc.c:1367)
/// Calls: historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn history_find_index(times: *const f64, n: i32, cursor: i32, t: f64) -> i32  {
    extern "C" { fn historyFindIndex(times: *const f64, n: i32, cursor: i32, t: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { historyFindIndex(times, n, cursor, t) }
}

/// C: mju_wrap (engine/engine_util_misc.h:32)
/// Calls: mju_addTo3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_norm3, mju_normalize, mju_normalize3, mju_scl, mju_scl3, mju_sub3, wrap_circle, wrap_inside
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_wrap(wpnt: *mut f64, x0: *const f64, x1: *const f64, xpos: *const f64, xmat: *const f64, radius: f64, r#type: i32, side: *const f64) -> f64  {
    extern "C" { fn mju_wrap(wpnt: *mut f64, x0: *const f64, x1: *const f64, xpos: *const f64, xmat: *const f64, radius: f64, r#type: i32, side: *const f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_wrap(wpnt, x0, x1, xpos, xmat, radius, r#type, side) }
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
    if dynprm.is_null() || gainprm.is_null() {
        // SAFETY: zeroed mjDCMotorSlots is a valid default state
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn mj_dcmotorSlots(dynprm: *const f64, gainprm: *const f64) -> mjDCMotorSlots; }
    // SAFETY: dynprm and gainprm verified non-null; delegates to C implementation
    unsafe { mj_dcmotorSlots(dynprm, gainprm) }
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
        fn mju_geomSemiAxes(semiaxes: *mut f64, size: *const f64, r#type: mjtGeom);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_geomSemiAxes(semiaxes, size, r#type) }
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
        fn mju_insideGeom(pos: *const f64, mat: *const f64, size: *const f64, r#type: mjtGeom, point: *const f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_insideGeom(pos, mat, size, r#type, point) }
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
    // SAFETY: origin and direction have 3 elements. cam_xpos has 3 elements.
    // cam_xmat has 9 elements. All valid per caller contract.
    unsafe {
        const MJPROJ_PERSPECTIVE: i32 = 0;

        // pixel center (row 0 = top of image)
        let px: f64 = col as f64 + 0.5 - cx;
        let py: f64 = row as f64 + 0.5 - cy;

        if projection == MJPROJ_PERSPECTIVE {
            // origin is camera position
            crate::engine::engine_util_blas::mju_copy3(origin, cam_xpos);

            // direction in camera frame: (x/fx, -y/fy, -1), then normalized
            let dir_cam: [f64; 3] = [px / fx, -py / fy, -1.0];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(direction, cam_xmat, dir_cam.as_ptr());
            crate::engine::engine_util_blas::mju_normalize3(direction);
        } else {
            // orthographic: parallel rays, direction is -Z in camera frame
            *direction.add(0) = -*cam_xmat.add(2);
            *direction.add(1) = -*cam_xmat.add(5);
            *direction.add(2) = -*cam_xmat.add(8);

            // origin offset in camera frame
            let half_extent: f64 = ortho_extent / 2.0;
            let offset_cam: [f64; 3] = [px / fx * half_extent, -py / fy * half_extent, 0.0];
            let mut offset_world: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_vec3(offset_world.as_mut_ptr(), cam_xmat, offset_cam.as_ptr());
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
    // SAFETY: res has 9 elements, p has 3, dof has 3*(order+1)^3 elements. All valid.
    unsafe {
        let mut idx: i32 = 0;
        let mut gradient: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_zero(res, 9);
        let mut i: i32 = 0;
        while i <= order {
            let mut j: i32 = 0;
            while j <= order {
                let mut k: i32 = 0;
                while k <= order {
                    gradient[0] = mju_flex_dphi(*p.add(0), i, order) * mju_flex_phi(*p.add(1), j, order) * mju_flex_phi(*p.add(2), k, order);
                    gradient[1] = mju_flex_phi(*p.add(0), i, order) * mju_flex_dphi(*p.add(1), j, order) * mju_flex_phi(*p.add(2), k, order);
                    gradient[2] = mju_flex_phi(*p.add(0), i, order) * mju_flex_phi(*p.add(1), j, order) * mju_flex_dphi(*p.add(2), k, order);
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
    // SAFETY: all pointers valid (some may be null — checked before use).
    // Grid arrays sized per global grid; cell-local arrays sized per (order+1)^3 nodes.
    unsafe {
        let ny_g: i32 = cy * order + 1;
        let nz_g: i32 = cz * order + 1;

        let mut local: i32 = 0;
        let mut li: i32 = 0;
        while li <= order {
            let mut lj: i32 = 0;
            while lj <= order {
                let mut lk: i32 = 0;
                while lk <= order {
                    let gi: i32 = ci * order + li;
                    let gj: i32 = cj * order + lj;
                    let gk: i32 = ck * order + lk;
                    let gidx: i32 = gi * ny_g * nz_g + gj * nz_g + gk;

                    if !xpos_c.is_null() && !xpos_g.is_null() {
                        crate::engine::engine_util_blas::mju_copy3(xpos_c.add(3 * local as usize), xpos_g.add(3 * gidx as usize));
                    }
                    if !vel_c.is_null() && !vel_g.is_null() {
                        crate::engine::engine_util_blas::mju_copy3(vel_c.add(3 * local as usize), vel_g.add(3 * gidx as usize));
                    }
                    if !xpos0_c.is_null() && !xpos0_g.is_null() {
                        crate::engine::engine_util_blas::mju_copy3(xpos0_c.add(3 * local as usize), xpos0_g.add(3 * gidx as usize));
                    }
                    if !nodeindices.is_null() {
                        *nodeindices.add(local as usize) = gidx;
                    }

                    local += 1;
                    lk += 1;
                }
                lj += 1;
            }
            li += 1;
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
    // SAFETY: all pointers valid (some may be null — checked before use).
    // Arrays have adequate capacity for the given order and cell counts.
    unsafe {
        let ny_g = cy * order + 1;
        let nz_g = cz * order + 1;
        let npe = (order + 1) * (order + 1);

        // face sizes and properties
        let face_sizes: [i32; 6] = [cy*cz, cy*cz, cx*cz, cx*cz, cx*cy, cx*cy];
        let face_normal: [i32; 6] = [0, 0, 1, 1, 2, 2];
        let face_count1: [i32; 6] = [cz, cz, cx, cx, cy, cy];
        let face_fixed_vals: [i32; 6] = [0, cx*order, 0, cy*order, 0, cz*order];

        // determine which face and quad within face
        let mut face_id: i32 = 0;
        let mut within_face: i32 = face_elem_idx;
        let mut cumul: i32 = 0;
        let mut f: i32 = 0;
        while f < 6 {
            if face_elem_idx < cumul + face_sizes[f as usize] {
                face_id = f;
                within_face = face_elem_idx - cumul;
                break;
            }
            cumul += face_sizes[f as usize];
            f += 1;
        }

        let normal_axis = face_normal[face_id as usize];
        let na0 = (normal_axis + 1) % 3;
        let na1 = (normal_axis + 2) % 3;
        let c1 = face_count1[face_id as usize];
        let g_fixed = face_fixed_vals[face_id as usize];
        let q0 = within_face / c1;
        let q1 = within_face % c1;

        // gather nodes
        let mut local: i32 = 0;
        let mut l0: i32 = 0;
        while l0 <= order {
            let mut l1: i32 = 0;
            while l1 <= order {
                let mut g: [i32; 3] = [0; 3];
                g[normal_axis as usize] = g_fixed;
                g[na0 as usize] = q0 * order + l0;
                g[na1 as usize] = q1 * order + l1;
                let gidx = g[0] * ny_g * nz_g + g[1] * nz_g + g[2];

                if !xpos_f.is_null() && !xpos_g.is_null() {
                    crate::engine::engine_util_blas::mju_copy3(xpos_f.add(3 * local as usize), xpos_g.add(3 * gidx as usize));
                }
                if !vel_f.is_null() && !vel_g.is_null() {
                    crate::engine::engine_util_blas::mju_copy3(vel_f.add(3 * local as usize), vel_g.add(3 * gidx as usize));
                }
                if !xpos0_f.is_null() && !xpos0_g.is_null() {
                    crate::engine::engine_util_blas::mju_copy3(xpos0_f.add(3 * local as usize), xpos0_g.add(3 * gidx as usize));
                }
                if !nodeindices.is_null() {
                    *nodeindices.add(local as usize) = gidx;
                }

                local += 1;
                l1 += 1;
            }
            l0 += 1;
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
    // SAFETY: xpos_f has 3*npe elements, local has 2, quat has 4. All valid.
    unsafe {
        // compute 3x2 deformation gradient F at parametric point
        let mut t1: [f64; 3] = [0.0, 0.0, 0.0];
        let mut t2: [f64; 3] = [0.0, 0.0, 0.0];
        let mut idx: i32 = 0;
        let mut l0: i32 = 0;
        while l0 <= order {
            let mut l1: i32 = 0;
            while l1 <= order {
                let grad0 = mju_flex_dphi(*local.add(0), l0, order) * mju_flex_phi(*local.add(1), l1, order);
                let grad1 = mju_flex_phi(*local.add(0), l0, order) * mju_flex_dphi(*local.add(1), l1, order);
                let mut d: i32 = 0;
                while d < 3 {
                    t1[d as usize] += *xpos_f.add((3 * idx + d) as usize) * grad0;
                    t2[d as usize] += *xpos_f.add((3 * idx + d) as usize) * grad1;
                    d += 1;
                }
                idx += 1;
                l1 += 1;
            }
            l0 += 1;
        }

        // normal = t1 x t2
        let mut normal: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_cross(normal.as_mut_ptr(), t1.as_ptr(), t2.as_ptr());

        // build 3x3 matrix with columns assigned to canonical axes (row-major)
        let mut mat: [f64; 9] = [0.0; 9];
        let mut vecs: [*const f64; 3] = [core::ptr::null(); 3];
        vecs[axis0 as usize] = t1.as_ptr();
        vecs[axis1 as usize] = t2.as_ptr();
        vecs[normal_axis as usize] = normal.as_ptr();

        let mut col: i32 = 0;
        while col < 3 {
            mat[(0 * 3 + col) as usize] = *vecs[col as usize].add(0);
            mat[(1 * 3 + col) as usize] = *vecs[col as usize].add(1);
            mat[(2 * 3 + col) as usize] = *vecs[col as usize].add(2);
            col += 1;
        }

        // extract rotation via polar decomposition
        *quat.add(0) = 1.0;
        *quat.add(1) = 0.0;
        *quat.add(2) = 0.0;
        *quat.add(3) = 0.0;
        crate::engine::engine_util_spatial::mju_mat2rot(quat, mat.as_ptr());
        crate::engine::engine_util_spatial::mju_neg_quat(quat, quat);
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
    // SAFETY: normal/t1/t2 have 3 elements. xpos_f has 3*(order+1)^2 elements. local has 2 elements.
    unsafe {
        crate::engine::engine_util_blas::mju_zero3(t1);
        crate::engine::engine_util_blas::mju_zero3(t2);
        let mut idx: i32 = 0;
        let mut l0: i32 = 0;
        while l0 <= order {
            let mut l1: i32 = 0;
            while l1 <= order {
                let grad0 = mju_flex_dphi(*local.add(0), l0, order) * mju_flex_phi(*local.add(1), l1, order);
                let grad1 = mju_flex_phi(*local.add(0), l0, order) * mju_flex_dphi(*local.add(1), l1, order);
                let mut d: i32 = 0;
                while d < 3 {
                    *t1.add(d as usize) += *xpos_f.add((3 * idx + d) as usize) * grad0;
                    *t2.add(d as usize) += *xpos_f.add((3 * idx + d) as usize) * grad1;
                    d += 1;
                }
                idx += 1;
                l1 += 1;
            }
            l0 += 1;
        }
        crate::engine::engine_util_spatial::mju_cross(normal, t1, t2);
    }
}

/// C: mju_flexPhi (engine/engine_util_misc.h:130)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_phi(s: f64, i: i32, order: i32) -> f64  {
    extern "C" { fn mju_flexPhi(s: f64, i: i32, order: i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_flexPhi(s, i, order) }
}

/// C: mju_flexDphi (engine/engine_util_misc.h:141)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_dphi(s: f64, i: i32, order: i32) -> f64  {
    extern "C" { fn mju_flexDphi(s: f64, i: i32, order: i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_flexDphi(s, i, order) }
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
    // SAFETY: all pointers valid. nodebodyid has adequate capacity for the indexing below.
    unsafe {
        let s: f64 = i as f64 / (nx - 1) as f64;
        let t: f64 = j as f64 / (ny - 1) as f64;
        let u: f64 = k as f64 / (nz - 1) as f64;

        // face contributions
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + 0*ny*nz + j*nz + k) as usize), w * (1.0-s));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + (nx-1)*ny*nz + j*nz + k) as usize), w * s);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + i*ny*nz + 0*nz + k) as usize), w * (1.0-t));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + i*ny*nz + (ny-1)*nz + k) as usize), w * t);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + i*ny*nz + j*nz + 0) as usize), w * (1.0-u));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + i*ny*nz + j*nz + (nz-1)) as usize), w * u);

        // edge corrections
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + i*ny*nz + 0*nz + 0) as usize), -w * (1.0-t)*(1.0-u));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + i*ny*nz + 0*nz + (nz-1)) as usize), -w * (1.0-t)*u);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + i*ny*nz + (ny-1)*nz + 0) as usize), -w * t*(1.0-u));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + i*ny*nz + (ny-1)*nz + (nz-1)) as usize), -w * t*u);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + 0*ny*nz + j*nz + 0) as usize), -w * (1.0-s)*(1.0-u));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + 0*ny*nz + j*nz + (nz-1)) as usize), -w * (1.0-s)*u);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + (nx-1)*ny*nz + j*nz + 0) as usize), -w * s*(1.0-u));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + (nx-1)*ny*nz + j*nz + (nz-1)) as usize), -w * s*u);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + 0*ny*nz + 0*nz + k) as usize), -w * (1.0-s)*(1.0-t));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + 0*ny*nz + (ny-1)*nz + k) as usize), -w * (1.0-s)*t);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + (nx-1)*ny*nz + 0*nz + k) as usize), -w * s*(1.0-t));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + (nx-1)*ny*nz + (ny-1)*nz + k) as usize), -w * s*t);

        // corner corrections
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + 0*ny*nz + 0*nz + 0) as usize), w * (1.0-s)*(1.0-t)*(1.0-u));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + 0*ny*nz + 0*nz + (nz-1)) as usize), w * (1.0-s)*(1.0-t)*u);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + 0*ny*nz + (ny-1)*nz + 0) as usize), w * (1.0-s)*t*(1.0-u));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + 0*ny*nz + (ny-1)*nz + (nz-1)) as usize), w * (1.0-s)*t*u);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + (nx-1)*ny*nz + 0*nz + 0) as usize), w * s*(1.0-t)*(1.0-u));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + (nx-1)*ny*nz + 0*nz + (nz-1)) as usize), w * s*(1.0-t)*u);
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + (nx-1)*ny*nz + (ny-1)*nz + 0) as usize), w * s*t*(1.0-u));
        add_weight(nb, body, bweight, *nodebodyid.add((nstart + (nx-1)*ny*nz + (ny-1)*nz + (nz-1)) as usize), w * s*t*u);
    }
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
    extern "C" { fn mju_historyInit(buf: *mut f64, n: i32, dim: i32, times: *const f64, values: *const f64, user: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mju_historyInit(buf, n, dim, times, values, user) }
}

/// C: mju_historyInsert (engine/engine_util_misc.h:189)
/// Calls: historyFindIndex, historyPhysicalIndex, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_insert(buf: *mut f64, n: i32, dim: i32, t: f64) -> *mut f64  {
    extern "C" { fn mju_historyInsert(buf: *mut f64, n: i32, dim: i32, t: f64) -> *mut f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_historyInsert(buf, n, dim, t) }
}

/// C: mju_historyRead (engine/engine_util_misc.h:194)
/// Calls: historyFindIndex, historyPhysicalIndex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_history_read(buf: *const f64, n: i32, dim: i32, res: *mut f64, t: f64, interp: i32) -> *const f64  {
    extern "C" { fn mju_historyRead(buf: *const f64, n: i32, dim: i32, res: *mut f64, t: f64, interp: i32) -> *const f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_historyRead(buf, n, dim, res, t, interp) }
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
    extern "C" { fn mju_encodePyramid(pyramid: *mut f64, force: *const f64, mu: *const f64, dim: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mju_encodePyramid(pyramid, force, mu, dim) }
}

/// C: mju_decodePyramid (engine/engine_util_misc.h:204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_pyramid(force: *mut f64, pyramid: *const f64, mu: *const f64, dim: i32) {
    // SAFETY: force has dim elements, pyramid has 2*(dim-1) elements, mu has dim-1 elements.
    unsafe {
        // special handling of frictionless contacts
        if dim == 1 {
            *force.add(0) = *pyramid.add(0);
            return;
        }

        // force_normal = sum(pyramid0_i + pyramid1_i)
        *force.add(0) = 0.0;
        let mut i: i32 = 0;
        while i < 2 * (dim - 1) {
            *force.add(0) += *pyramid.add(i as usize);
            i += 1;
        }

        // force_tangent_i = (pyramid0_i - pyramid1_i) * mu_i
        i = 0;
        while i < dim - 1 {
            *force.add((i + 1) as usize) = (*pyramid.add((2 * i) as usize) - *pyramid.add((2 * i + 1) as usize)) * *mu.add(i as usize);
            i += 1;
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
    // SAFETY: pure arithmetic, no pointers
    // C params: k=Kp, b=Kv, t=dt
    const MJMINVAL: f64 = 1e-15;

    let det: f64;
    let c1: f64;
    let c2: f64;
    let r1: f64;
    let r2: f64;
    let w: f64;

    // determinant of characteristic equation
    det = Kv * Kv - 4.0 * Kp;

    // overdamping
    //  pos(t) = c1*exp(r1*t) + c2*exp(r2*t);  r12 = (-b +- sqrt(det))/2
    if det > MJMINVAL {
        // compute w = sqrt(det)/2
        w = det.sqrt() / 2.0;

        // compute r1,r2
        r1 = -Kv / 2.0 + w;
        r2 = -Kv / 2.0 - w;

        // compute coefficients
        c1 = (pos0 * r2 - vel0) / (r2 - r1);
        c2 = (pos0 * r1 - vel0) / (r1 - r2);

        // evaluate result
        return c1 * (r1 * dt).exp() + c2 * (r2 * dt).exp();
    }

    // critical damping
    //  pos(t) = exp(-b*t/2) * (c1 + c2*t)
    else if det <= MJMINVAL && det >= -MJMINVAL {
        // compute coefficients
        c1 = pos0;
        c2 = vel0 + Kv * c1 / 2.0;

        // evaluate result
        return (-Kv * dt / 2.0).exp() * (c1 + c2 * dt);
    }

    // underdamping
    //  pos(t) = exp(-b*t/2) * (c1*cos(w*t) + c2*sin(w*t));  w = sqrt(abs(det))/2
    else {
        // compute w
        w = det.abs().sqrt() / 2.0;

        // compute coefficients
        c1 = pos0;
        c2 = (vel0 + Kv * c1 / 2.0) / w;

        // evaluate result
        return (-Kv * dt / 2.0).exp() * (c1 * (w * dt).cos() + c2 * (w * dt).sin());
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
    // SAFETY: all pointers are valid arrays per caller contract (point[3], pos[3], mat[9], size[3])
    unsafe {
        // check inflation coefficient
        if inflate < 1.0 {
            crate::engine::engine_util_errmem::mju_error(
                b"inflation coefficient must be >= 1\0".as_ptr() as *const i8,
            );
        }

        // vector from pos to point, projected to box frame
        let mut vec: [f64; 3] = [
            *point.add(0) - *pos.add(0),
            *point.add(1) - *pos.add(1),
            *point.add(2) - *pos.add(2),
        ];
        crate::engine::engine_util_blas::mju_mul_mat_t_vec3(
            vec.as_mut_ptr(),
            mat,
            vec.as_ptr(),
        );

        // big: inflated box
        let mut big: [f64; 3] = [*size.add(0), *size.add(1), *size.add(2)];
        if inflate > 1.0 {
            crate::engine::engine_util_blas::mju_scl3(big.as_mut_ptr(), big.as_ptr(), inflate);
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
    extern "C" {
        fn printf(format: *const i8, ...) -> i32;
    }
    // SAFETY: mat points to valid array of nr*nc doubles. printf is standard C library.
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
    extern "C" {
        fn printf(format: *const i8, ...) -> i32;
    }
    // SAFETY: mat, rownnz, rowadr, colind are valid sparse matrix arrays.
    // printf is standard C library.
    unsafe {
        for r in 0..nr {
            let adr_start = *rowadr.add(r as usize);
            let adr_end = adr_start + *rownnz.add(r as usize);
            let mut adr = adr_start;
            while adr < adr_end {
                printf(
                    b"(%d %d): %9.6f  \0".as_ptr() as *const i8,
                    r,
                    *colind.add(adr as usize),
                    *mat.add(adr as usize),
                );
                adr += 1;
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
    match r#type {
        1  => b"body\0".as_ptr() as *const i8,
        2  => b"xbody\0".as_ptr() as *const i8,
        3  => b"joint\0".as_ptr() as *const i8,
        4  => b"dof\0".as_ptr() as *const i8,
        5  => b"geom\0".as_ptr() as *const i8,
        6  => b"site\0".as_ptr() as *const i8,
        7  => b"camera\0".as_ptr() as *const i8,
        8  => b"light\0".as_ptr() as *const i8,
        9  => b"flex\0".as_ptr() as *const i8,
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
        26 => b"frame\0".as_ptr() as *const i8,
        _  => std::ptr::null(),
    }
}

/// C: mju_str2Type (engine/engine_util_misc.h:243)
#[allow(unused_variables, non_snake_case)]
pub fn mju_str2type(str: *const i8) -> i32 {
    extern "C" {
        fn strcmp(s1: *const i8, s2: *const i8) -> i32;
    }
    // SAFETY: str is a valid null-terminated C string per caller contract.
    // strcmp is standard C library.
    unsafe {
        // mjOBJ enum values (0-based sequential, then mjOBJ_FRAME=100)
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

        if strcmp(str, b"body\0".as_ptr() as *const i8) == 0 { return mjOBJ_BODY; }
        if strcmp(str, b"xbody\0".as_ptr() as *const i8) == 0 { return mjOBJ_XBODY; }
        if strcmp(str, b"joint\0".as_ptr() as *const i8) == 0 { return mjOBJ_JOINT; }
        if strcmp(str, b"dof\0".as_ptr() as *const i8) == 0 { return mjOBJ_DOF; }
        if strcmp(str, b"geom\0".as_ptr() as *const i8) == 0 { return mjOBJ_GEOM; }
        if strcmp(str, b"site\0".as_ptr() as *const i8) == 0 { return mjOBJ_SITE; }
        if strcmp(str, b"camera\0".as_ptr() as *const i8) == 0 { return mjOBJ_CAMERA; }
        if strcmp(str, b"light\0".as_ptr() as *const i8) == 0 { return mjOBJ_LIGHT; }
        if strcmp(str, b"flex\0".as_ptr() as *const i8) == 0 { return mjOBJ_FLEX; }
        if strcmp(str, b"mesh\0".as_ptr() as *const i8) == 0 { return mjOBJ_MESH; }
        if strcmp(str, b"skin\0".as_ptr() as *const i8) == 0 { return mjOBJ_SKIN; }
        if strcmp(str, b"hfield\0".as_ptr() as *const i8) == 0 { return mjOBJ_HFIELD; }
        if strcmp(str, b"texture\0".as_ptr() as *const i8) == 0 { return mjOBJ_TEXTURE; }
        if strcmp(str, b"material\0".as_ptr() as *const i8) == 0 { return mjOBJ_MATERIAL; }
        if strcmp(str, b"pair\0".as_ptr() as *const i8) == 0 { return mjOBJ_PAIR; }
        if strcmp(str, b"exclude\0".as_ptr() as *const i8) == 0 { return mjOBJ_EXCLUDE; }
        if strcmp(str, b"equality\0".as_ptr() as *const i8) == 0 { return mjOBJ_EQUALITY; }
        if strcmp(str, b"tendon\0".as_ptr() as *const i8) == 0 { return mjOBJ_TENDON; }
        if strcmp(str, b"actuator\0".as_ptr() as *const i8) == 0 { return mjOBJ_ACTUATOR; }
        if strcmp(str, b"sensor\0".as_ptr() as *const i8) == 0 { return mjOBJ_SENSOR; }
        if strcmp(str, b"numeric\0".as_ptr() as *const i8) == 0 { return mjOBJ_NUMERIC; }
        if strcmp(str, b"text\0".as_ptr() as *const i8) == 0 { return mjOBJ_TEXT; }
        if strcmp(str, b"tuple\0".as_ptr() as *const i8) == 0 { return mjOBJ_TUPLE; }
        if strcmp(str, b"key\0".as_ptr() as *const i8) == 0 { return mjOBJ_KEY; }
        if strcmp(str, b"plugin\0".as_ptr() as *const i8) == 0 { return mjOBJ_PLUGIN; }
        mjOBJ_UNKNOWN
    }
}

/// C: mju_writeNumBytes (engine/engine_util_misc.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_num_bytes(nbytes: usize) -> *const i8 {
    extern "C" {
        fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;
    }
    // SAFETY: Uses a static buffer (mirrors C thread-local static). 
    // snprintf is standard C library. The returned pointer is valid until next call.
    static mut MESSAGE: [i8; 20] = [0; 20];
    static SUFFIX: &[u8] = b" KMGTPE";
    unsafe {
        let mut i: i32 = 0;
        while i < 6 {
            let bits: usize = 1usize << (10 * (6 - i));
            if nbytes >= bits && (nbytes & (bits - 1)) == 0 {
                break;
            }
            i += 1;
        }
        if i < 6 {
            snprintf(
                MESSAGE.as_mut_ptr(), 20,
                b"%zu%c\0".as_ptr() as *const i8,
                nbytes >> (10 * (6 - i) as u32),
                SUFFIX[(6 - i) as usize] as i32,
            );
        } else {
            snprintf(
                MESSAGE.as_mut_ptr(), 20,
                b"%zu\0".as_ptr() as *const i8,
                nbytes >> (10 * (6 - i) as u32),
            );
        }
        MESSAGE.as_ptr()
    }
}

/// C: mju_warningText (engine/engine_util_misc.h:249)
/// Calls: mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning_text(warning: i32, info: usize) -> *const i8 {
    extern "C" {
        fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;
    }
    // SAFETY: Uses a static buffer (mirrors C thread-local static).
    // snprintf is standard C library. The returned pointer is valid until next call.
    static mut STR: [i8; 1000] = [0; 1000];

    const mjWARN_INERTIA: i32 = 0;
    const mjWARN_CONTACTFULL: i32 = 1;
    const mjWARN_CNSTRFULL: i32 = 2;
    const mjWARN_BADQPOS: i32 = 3;
    const mjWARN_BADQVEL: i32 = 4;
    const mjWARN_BADQACC: i32 = 5;
    const mjWARN_BADCTRL: i32 = 6;

    unsafe {
        match warning {
            mjWARN_INERTIA => {
                snprintf(STR.as_mut_ptr(), 1000,
                    b"Inertia matrix is too close to singular at DOF %zu. Check model.\0".as_ptr() as *const i8,
                    info);
            }
            mjWARN_CONTACTFULL => {
                snprintf(STR.as_mut_ptr(), 1000,
                    b"Too many contacts. The arena memory is full, increase arena memory allocation.(ncon = %zu)\0".as_ptr() as *const i8,
                    info);
            }
            mjWARN_CNSTRFULL => {
                snprintf(STR.as_mut_ptr(), 1000,
                    b"Insufficient arena memory for the number of constraints generated. Increase arena memory allocation above %s bytes.\0".as_ptr() as *const i8,
                    mju_write_num_bytes(info));
            }
            mjWARN_BADQPOS => {
                snprintf(STR.as_mut_ptr(), 1000,
                    b"Nan, Inf or huge value in QPOS at DOF %zu. The simulation is unstable.\0".as_ptr() as *const i8,
                    info);
            }
            mjWARN_BADQVEL => {
                snprintf(STR.as_mut_ptr(), 1000,
                    b"Nan, Inf or huge value in QVEL at DOF %zu. The simulation is unstable.\0".as_ptr() as *const i8,
                    info);
            }
            mjWARN_BADQACC => {
                snprintf(STR.as_mut_ptr(), 1000,
                    b"Nan, Inf or huge value in QACC at DOF %zu. The simulation is unstable.\0".as_ptr() as *const i8,
                    info);
            }
            mjWARN_BADCTRL => {
                snprintf(STR.as_mut_ptr(), 1000,
                    b"Nan, Inf or huge value in CTRL at ACTUATOR %zu. The simulation is unstable.\0".as_ptr() as *const i8,
                    info);
            }
            _ => {
                snprintf(STR.as_mut_ptr(), 1000,
                    b"Unknown warning type %d.\0".as_ptr() as *const i8,
                    warning);
            }
        }
        STR.as_ptr()
    }
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
    // SAFETY: num2 may be null; if non-null must point to valid f64
    unsafe {
        extern "C" {
            fn rand() -> i32;
        }
        const RAND_MAX: f64 = 2147483647.0; // 0x7FFFFFFF
        let scale: f64 = 2.0 / RAND_MAX;
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

    if nr == 0 {
        return;
    }

    // SAFETY: all pointers valid per caller contract, indices within bounds
    unsafe {
        // default all map entries to "no source"
        let nnz: i32 = *res_rowadr.add((nr - 1) as usize) + *res_rownnz.add((nr - 1) as usize);
        crate::engine::engine_util_misc::mju_fill_int(map, -1, nnz);

        // initialize per-row cursor
        let mut i: i32 = 0;
        while i < nr {
            *cursor.add(i as usize) = *res_rowadr.add(i as usize);
            i += 1;
        }

        // sweep src rows; for each lower (i,j) set res(i,j) and res(j,i)
        i = 0;
        while i < nr {
            let src_start: i32 = *src_rowadr.add(i as usize);
            let src_end: i32 = src_start + *src_rownnz.add(i as usize);

            // sweep src row
            let mut k: i32 = src_start;
            while k < src_end {
                let j: i32 = *src_colind.add(k as usize);
                if j > i { break; } // use only lower triangle of src

                // --- lower triangle: res(i, j)
                let res_start_i: i32 = *res_rowadr.add(i as usize);
                let res_end_i: i32 = res_start_i + *res_rownnz.add(i as usize);
                let mut c: i32 = *cursor.add(i as usize);

                // increment c until there is a match
                while c < res_end_i && *res_colind.add(c as usize) < j { c += 1; }

                // found match, set index, advance and save cursor
                if c < res_end_i && *res_colind.add(c as usize) == j {
                    *map.add(c as usize) = k;
                    c += 1;
                }
                *cursor.add(i as usize) = c;

                // --- upper mirror: res(j, i)
                if j != i {
                    let res_start_j: i32 = *res_rowadr.add(j as usize);
                    let res_end_j: i32 = res_start_j + *res_rownnz.add(j as usize);
                    c = *cursor.add(j as usize);

                    // increment c until there is a match
                    while c < res_end_j && *res_colind.add(c as usize) < i { c += 1; }

                    // found match, set index and advance and save cursor
                    if c < res_end_j && *res_colind.add(c as usize) == i {
                        *map.add(c as usize) = k;
                        c += 1;
                    }
                    *cursor.add(j as usize) = c;
                }

                k += 1;
            }
            i += 1;
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
    // SAFETY: list points to at least n elements, valid per caller contract.
    unsafe {
        let mut i: i32 = 1;
        while i < n {
            let x: f64 = *list.add(i as usize);
            let mut j: i32 = i - 1;
            while j >= 0 && *list.add(j as usize) > x {
                *list.add((j + 1) as usize) = *list.add(j as usize);
                j -= 1;
            }
            *list.add((j + 1) as usize) = x;
            i += 1;
        }
    }
}

/// C: mju_insertionSortInt (engine/engine_util_misc.h:315)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort_int(list: *mut i32, n: i32) {
    // SAFETY: list has at least n elements, valid per caller contract
    unsafe {
    let mut i: i32 = 1;
    while i < n {
        let x: i32 = *list.add(i as usize);
        let mut j: i32 = i - 1;
        while j >= 0 && *list.add(j as usize) > x {
            *list.add((j + 1) as usize) = *list.add(j as usize);
            j -= 1;
        }
        *list.add((j + 1) as usize) = x;
        i += 1;
    }
    } // unsafe
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

