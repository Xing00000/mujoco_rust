//! Port of: engine/engine_util_misc.c
//! IR hash: 545f394232195ad9
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
    unsafe { nodexpos.add(3 * (i * ny * nz + j * nz + k) as usize) }
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
    todo ! ()
}

/// C: mju_geomSemiAxes (engine/engine_util_misc.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_geom_semi_axes(semiaxes: *mut f64, size: *const f64, r#type: mjtGeom) {
    // WARNING: signature changed — verify body
    // Previous params: (semiaxes : * mut f64, size : * const f64, r#type : mjtGeom)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, mat : * const f64, size : * const f64, r#type : mjtGeom, point : * const f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (origin : * mut f64, direction : * mut f64, cam_xpos : * const f64, cam_xmat : * const f64, col : i32, row : i32, fx : f64, fy : f64, cx : f64, cy : f64, projection : i32, ortho_extent : f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, p : * const f64, dof : * const f64, order : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (x : * const f64, i : i32, order : i32)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (basis : * mut f64, x : * const f64, order : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (coord : * const f64, cellnum : [i32 ; 3], order : i32, local : * mut f64, nodeindices : * mut i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, x : * const f64, coeff : * const f64, order : i32, nodeindices : * const i32)
    // Previous return: ()
    todo ! ()
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
    if order == 1 { return if i == 0 { 1.0 - s } else { s }; }
    match i {
        0 => 2.0*s*s - 3.0*s + 1.0,
        1 => 4.0*(s - s*s),
        2 => 2.0*s*s - s,
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
    if order == 1 { return if i == 0 { -1.0 } else { 1.0 }; }
    match i {
        0 => 4.0*s - 3.0,
        1 => 4.0*(1.0 - 2.0*s),
        2 => 4.0*s - 1.0,
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
    unsafe {
        const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut i: usize = 0;
        let mut j: usize = 0;

        // loop over 24 bit chunks
        while i + 3 <= ndata {
            let byte_1: u32 = *data.add(i) as u32; i += 1;
            let byte_2: u32 = *data.add(i) as u32; i += 1;
            let byte_3: u32 = *data.add(i) as u32; i += 1;

            let k: u32 = (byte_1 << 16) | (byte_2 << 8) | byte_3;

            *buf.add(j) = TABLE[((k >> 18) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 12) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 6) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 0) & 63) as usize] as i8; j += 1;
        }

        // one byte left
        if i + 1 == ndata {
            let byte_1: u32 = *data.add(i) as u32;
            let k: u32 = byte_1 << 16;
            *buf.add(j) = TABLE[((k >> 18) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 12) & 63) as usize] as i8; j += 1;
            *buf.add(j) = b'=' as i8; j += 1;
            *buf.add(j) = b'=' as i8; j += 1;
        }

        // two bytes left
        if i + 2 == ndata {
            let byte_1: u32 = *data.add(i) as u32; i += 1;
            let byte_2: u32 = *data.add(i) as u32;

            let k: u32 = (byte_1 << 16) + (byte_2 << 8);

            *buf.add(j) = TABLE[((k >> 18) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 12) & 63) as usize] as i8; j += 1;
            *buf.add(j) = TABLE[((k >> 6) & 63) as usize] as i8; j += 1;
            *buf.add(j) = b'=' as i8; j += 1;
        }

        *buf.add(j) = 0;
        4 * ((ndata + 2) / 3) + 1
    }
}

/// C: mju_isValidBase64 (engine/engine_util_misc.h:167)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_valid_base64(s: *const i8) -> usize {
    unsafe {
        let mut i: usize = 0;
        let mut pad: i32 = 0;

        // validate chars
        while *s.add(i) != 0 && *s.add(i) != b'=' as i8 {
            let c = *s.add(i) as u8;
            let is_alnum = (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z') || (c >= b'0' && c <= b'9');
            if !is_alnum && c != b'/' && c != b'+' {
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
        if len % 4 != 0 { 0 } else { (3 * (len / 4) - pad) as usize }
    }
}

/// C: mju_decodeBase64 (engine/engine_util_misc.h:171)
/// Calls: _decode
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_base64(buf: *mut u8, s: *const i8) -> usize {
    unsafe {
        let mut i: usize = 0;
        let mut j: usize = 0;

        // loop over 24 bit chunks
        while *s.add(i) != 0 {
            let char_1: u32 = decode(*s.add(i)); i += 1;
            let char_2: u32 = decode(*s.add(i)); i += 1;
            let char_3: u32 = decode(*s.add(i)); i += 1;
            let char_4: u32 = decode(*s.add(i)); i += 1;

            // merge into 32 bit int
            let k: u32 = (char_1 << 18) | (char_2 << 12) | (char_3 << 6) | char_4;

            // write up to three bytes (exclude padding at end)
            *buf.add(j) = ((k >> 16) & 0xFF) as u8; j += 1;
            if *s.add(i - 2) != b'=' as i8 {
                *buf.add(j) = ((k >> 8) & 0xFF) as u8; j += 1;
            }
            if *s.add(i - 1) != b'=' as i8 {
                *buf.add(j) = (k & 0xFF) as u8; j += 1;
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
    // WARNING: signature changed — verify body
    // Previous params: (force : * mut f64, pyramid : * const f64, mu : * const f64, dim : i32)
    // Previous return: ()
    todo ! ()
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
    todo ! ()
}

/// C: mju_str2Type (engine/engine_util_misc.h:243)
#[allow(unused_variables, non_snake_case)]
pub fn mju_str2type(str: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i8)
    // Previous return: i32
    todo ! ()
}

/// C: mju_writeNumBytes (engine/engine_util_misc.h:246)
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_num_bytes(nbytes: usize) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (nbytes : usize)
    // Previous return: * const i8
    todo ! ()
}

/// C: mju_warningText (engine/engine_util_misc.h:249)
/// Calls: mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning_text(warning: i32, info: usize) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (warning : i32, info : usize)
    // Previous return: * const i8
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (num2 : * mut f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut f64, n : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_insertionSortInt (engine/engine_util_misc.h:315)
#[allow(unused_variables, non_snake_case)]
pub fn mju_insertion_sort_int(list: *mut i32, n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (list : * mut i32, n : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_Halton (engine/engine_util_misc.h:318)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_halton(index: i32, base: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (index : i32, base : i32)
    // Previous return: f64
    todo ! ()
}

/// C: mju_strncpy (engine/engine_util_misc.h:321)
#[allow(unused_variables, non_snake_case)]
pub fn mju_strncpy(dst: *mut i8, src: *const i8, n: i32) -> *mut i8 {
    unsafe {
        if !dst.is_null() && !src.is_null() && n > 0 {
            extern "C" { fn strncpy(dst: *mut i8, src: *const i8, n: usize) -> *mut i8; }
            strncpy(dst, src, n as usize);
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
    unsafe {
        let x = if flg_odd != 0 { x.abs() } else { x };
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
    // WARNING: signature changed — verify body
    // Previous params: (linear : f64, poly : * const f64, x : f64, n : i32, flg_odd : i32)
    // Previous return: f64
    todo ! ()
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
    todo ! ()
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

