//! Port of: engine/engine_collision_continuous.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: min2 (engine/engine_collision_continuous.c:27)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn min2(a: f64, b: f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_min2
}

/// C: mjc_PtTri (engine/engine_collision_continuous.c:39)
/// Calls: cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_pt_tri(p: *const f64, a: *const f64, b: *const f64, c: *const f64, cp: *mut f64, w: *mut f64) -> f64 {
    todo!() // cxx:_mjc_PtTri
}

/// C: mjc_SegSeg (engine/engine_collision_continuous.c:118)
/// Calls: cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_seg_seg(p1: *const f64, p2: *const f64, q1: *const f64, q2: *const f64, cp1: *mut f64, cp2: *mut f64, st: *mut f64) -> f64 {
    todo!() // cxx:_mjc_SegSeg
}

/// C: closestOnPoly (engine/engine_collision_continuous.c:171)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_inside
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn closest_on_poly(p: *const f64, vbase: *const f32, pv: *const i32, nv: i32, nrm: *const f64, out: *mut f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_closestOnPoly
}

/// C: mjc_GeomDist (engine/engine_collision_continuous.c:219)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_closestOnPoly, cxx:_mju_dot3, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_geom_dist(m: *const mjModel, gi: i32, gpos: *const f64, gmat: *const f64, x: *const f64, n: *mut f64, distmax: f64) -> f64 {
    todo!() // cxx:_mjc_GeomDist
}

/// C: mjc_GeomVerts (engine/engine_collision_continuous.c:356)
/// Calls: cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_geom_verts(m: *const mjModel, gi: i32, gpos: *const f64, gmat: *const f64, out: *mut f64) -> i32 {
    todo!() // cxx:_mjc_GeomVerts
}

/// C: mjc_GeomEdges (engine/engine_collision_continuous.c:387)
/// Calls: cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_geom_edges(m: *const mjModel, gi: i32, gpos: *const f64, gmat: *const f64, out: *mut f64) -> i32 {
    todo!() // cxx:_mjc_GeomEdges
}

/// C: mjc_standoff (engine/engine_collision_continuous.c:439)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_standoff(band: f64, cap: f64) -> f64 {
    todo!() // cxx:_mjc_standoff
}

/// C: gapNormal (engine/engine_collision_continuous.c:447)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn gap_normal(n: *mut f64, p1: *const f64, p2: *const f64, dd: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_gapNormal
}

/// C: mjc_pairGap (engine/engine_collision_continuous.c:463)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_gapNormal, cxx:_mjc_GeomDist, cxx:_mjc_PtTri, cxx:_mjc_SegSeg, cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_pair_gap(pair: *const mjcFlexPair, m: *const mjModel, d: *const mjData, x: *const f64, gv: *const f64, ge: *const f64, radii: *const f64, n: *mut f64, idv: *mut i32, cw: *mut f64, nidx: *mut i32, cutoff: f64) -> f64 {
    todo!() // cxx:_mjc_pairGap
}

/// C: mjc_pairVerts (engine/engine_collision_continuous.c:540)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_pair_verts(v: *mut i32, pair: *const mjcFlexPair) -> i32 {
    todo!() // cxx:_mjc_pairVerts
}

/// C: mjc_pairBand (engine/engine_collision_continuous.c:572)
/// Calls: cxx:_mjc_pairVerts
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_pair_band(pair: *const mjcFlexPair, radii: *const f64, band: f64) -> f64 {
    todo!() // cxx:_mjc_pairBand
}

/// C: conGapAdv (engine/engine_collision_continuous.c:588)
/// Calls: cxx:_mjc_GeomDist, cxx:_mjc_PtTri, cxx:_mjc_SegSeg, cxx:_mjc_pairVerts, cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn con_gap_adv(con: *const mjcFlexPair, m: *const mjModel, d: *const mjData, x: *const f64, dxw: *const f64, t: f64, gv: *const f64, ge: *const f64, radii: *const f64, fidx: *const i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_conGapAdv
}

/// C: mjc_advance (engine/engine_collision_continuous.c:625)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_conGapAdv, cxx:_mjc_pairVerts, cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_advance(m: *const mjModel, d: *const mjData, x: *const f64, dxw: *const f64, gv: *const f64, ge: *const f64, radii: *const f64, nfv: i32, fidx: *const i32, cand: *const mjcFlexPair, ncand: i32, cgap: *const f64, pt2flex: *const i32, approut: *mut i32, toiout: *mut f64) -> f64 {
    todo!() // cxx:_mjc_advance
}

/// C: bandOffset (engine/engine_collision_continuous.c:715)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:_mjc_pairGap
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn band_offset(con: *const mjcFlexPair, radii: *const f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_bandOffset
}

/// C: addCand (engine/engine_collision_continuous.c:727)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_bandOffset, cxx:_mjc_pairBand, cxx:_mjc_pairGap, cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_cand(con: mjcFlexPair, m: *const mjModel, d: *const mjData, x: *const f64, gv: *const f64, ge: *const f64, radii: *const f64, thresh: f64, dfrom: *const f64, dto: *const f64, ghat: f64, cand: *mut mjcFlexPair, nc: *mut i32, candmax: i32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_addCand
}

/// C: bvhBox (engine/engine_collision_continuous.c:768)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn bvh_box(m: *const mjModel, d: *const mjData, f: i32, c: *const f64, h: *const f64, stack: *mut i32, out: *mut i32, maxout: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_bvhBox
}

/// C: mjc_candidates (engine/engine_collision_continuous.c:802)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_addCand, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_bvhBox, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_continuous.c:_min2, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_candidates(m: *const mjModel, d: *const mjData, x: *const f64, gv: *const f64, ge: *const f64, ngv: i32, nge: i32, radii: *const f64, thresh: f64, threshGeom: f64, maxdisp: f64, dfrom: *const f64, dto: *const f64, ghat: f64, nfv: i32, npt: i32, fidx: *const i32, flist: *const i32, fxadr: *const i32, nfd: i32, pt2flex: *const i32, cand: *mut mjcFlexPair, candmax: i32) -> i32 {
    todo!() // cxx:_mjc_candidates
}

