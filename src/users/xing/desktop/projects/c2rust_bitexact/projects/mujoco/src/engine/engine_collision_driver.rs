//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: getMargin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:161)
/// Calls: cxx:_mj_assignMargin
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_margin(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_getMargin
}

/// C: getGap (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_gap(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_getGap
}

/// C: resetArena (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:179)
#[allow(unused_variables, non_snake_case)]
pub fn reset_arena(d: *mut mjData) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_resetArena
}

/// C: alignArena (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:189)
#[allow(unused_variables, non_snake_case)]
pub fn align_arena(d: *mut mjData, alignment: usize) -> usize {
    todo!() // cxx-internal:engine_collision_driver.c.o:_alignArena
}

/// C: planeGeomDist (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:199)
/// Calls: cxx:_mju_dot3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_geom_dist(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32) -> f64 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_planeGeomDist
}

/// C: hasPlane (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:210)
#[allow(unused_variables, non_snake_case)]
pub fn has_plane(m: *const mjModel, body: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_hasPlane
}

/// C: filterBitmask (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:227)
#[allow(unused_variables, non_snake_case)]
pub fn filter_bitmask(contype1: i32, conaffinity1: i32, contype2: i32, conaffinity2: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterBitmask
}

/// C: filterBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_box(aabb1: *const f64, aabb2: *const f64, margin: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterBox
}

/// C: filterSphereBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:246)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_sphere_box(s: *const f64, bound: f64, aabb: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterSphereBox
}

/// C: filterSphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:258)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_sphere(pos1: *const f64, pos2: *const f64, bound: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterSphere
}

/// C: mj_filterSphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:267)
/// Calls: cxx-internal:engine_collision_driver.c.o:_filterSphere, cxx-internal:engine_collision_driver.c.o:_planeGeomDist
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_filter_sphere(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_filterSphere
}

/// C: filterBodyPair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:288)
#[allow(unused_variables, non_snake_case)]
pub fn filter_body_pair(weldbody1: i32, weldparent1: i32, asleep1: i32, weldbody2: i32, weldparent2: i32, asleep2: i32, dsbl_filterparent: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterBodyPair
}

/// C: canCollide (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:318)
#[allow(unused_variables, non_snake_case)]
pub fn can_collide(m: *const mjModel, bf: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_canCollide
}

/// C: canCollide2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:329)
/// Calls: cxx-internal:engine_collision_driver.c.o:_filterBitmask
#[allow(unused_variables, non_snake_case)]
pub fn can_collide2(m: *const mjModel, bf1: i32, bf2: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_canCollide2
}

/// C: mj_collideTree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:361)
/// Calls: cxx-internal:engine_collision_driver.c.o:_canCollide2, cxx-internal:engine_collision_driver.c.o:_filterBitmask, cxx-internal:engine_collision_driver.c.o:_filterBox, cxx-internal:engine_collision_driver.c.o:_filterCollisionPair, cxx-internal:engine_collision_driver.c.o:_filterSphereBox, cxx-internal:engine_collision_driver.c.o:_mj_collidePlaneFlex, cxx-internal:engine_collision_driver.c.o:_mj_collideSdfFlex, cxx-internal:engine_collision_driver.c.o:_mj_filterSphere, cxx-internal:engine_collision_driver.c.o:_mj_narrowphase, cxx:_mj_assignMargin, cxx:_mj_collideElems, cxx:_mj_collideGeomElem, cxx:_mj_collideOBB, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_error, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_tree(m: *const mjModel, d: *mut mjData, bf1: i32, bf2: i32, merged: i32, startadr: i32, pairadr: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_collideTree
}

/// C: mj_narrowphase (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:367)
/// Calls: cxx-internal:engine_collision_driver.c.o:_getGap, cxx-internal:engine_collision_driver.c.o:_getMargin, cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_arenaAllocByte, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_maxContact, cxx:_mj_stackAllocByte, cxx:_mj_stackAllocInfo, cxx:_mj_stackAllocInt, cxx:_mj_warning, cxx:_mjc_ccdSize, cxx:_mju_copy, cxx:_mju_dispatch, cxx:_mju_numThread
#[allow(unused_variables, non_snake_case)]
pub fn mj_narrowphase(m: *const mjModel, d: *mut mjData, buffer: *const mjcPair, npair: i32, parena: usize) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_narrowphase
}

/// C: mj_collidePlaneFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:371)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_addContact, cxx:_mj_assignMargin, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_plane_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_collidePlaneFlex
}

/// C: mj_collideSdfFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:374)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_addContact, cxx:_mj_assignMargin, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mjc_FlexSDF, cxx:_mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_sdf_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_collideSdfFlex
}

/// C: mj_collideFlexInternal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:377)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx-internal:engine_collision_driver.c.o:_planeVertex, cxx:_mj_addContact, cxx:_mj_collideElemVert, cxx:_mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_internal(m: *const mjModel, d: *mut mjData, f: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_collideFlexInternal
}

/// C: contactcompare (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:380)
#[allow(unused_variables, non_snake_case)]
pub fn contactcompare(c1: *const mjContact, c2: *const mjContact, context: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_contactcompare
}

/// C: contactSort (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:413)
/// Calls: cxx-internal:engine_collision_driver.c.o:_contactcompare
#[allow(unused_variables, non_snake_case)]
pub fn contact_sort(arr: *mut mjContact, buf: *mut mjContact, n: i32, context: *mut ()) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_contactSort
}

/// C: filterFlexContacts (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:417)
/// Calls: cxx-internal:engine_collision_driver.c.o:_resetArena, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn filter_flex_contacts(d: *mut mjData, ncon_before: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterFlexContacts
}

/// C: pushPairArena (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:489)
/// Calls: cxx:_mj_arenaAllocByte, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn push_pair_arena(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_pushPairArena
}

/// C: filterCollisionPair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:508)
/// Calls: cxx-internal:engine_collision_driver.c.o:_filterBitmask, cxx-internal:engine_collision_driver.c.o:_getGap, cxx-internal:engine_collision_driver.c.o:_getMargin, cxx-internal:engine_collision_driver.c.o:_mj_filterSphere
#[allow(unused_variables, non_snake_case)]
pub fn filter_collision_pair(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32, merged: i32, startadr: i32, pairadr: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterCollisionPair
}

/// C: makeAAMM (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1211)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mji_transpose3, cxx:_mju_addTo3, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_aamm(m: *const mjModel, d: *mut mjData, x_min: *mut f64, y_min: *mut f64, z_min: *mut f64, x_max: *mut f64, y_max: *mut f64, z_max: *mut f64, bf: i32, frame: *const f64) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_makeAAMM
}

/// C: add_pair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1315)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn add_pair(m: *const mjModel, bf1: i32, bf2: i32, npair: *mut i32, pair: *mut i32, maxpair: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_add_pair
}

/// C: SAPcmp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1383)
#[allow(unused_variables, non_snake_case)]
pub fn sa_pcmp(obj1: *mut mjtSAP, obj2: *mut mjtSAP, context: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_SAPcmp
}

/// C: SAPsort (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1394)
/// Calls: cxx-internal:engine_collision_driver.c.o:_SAPcmp
#[allow(unused_variables, non_snake_case)]
pub fn sa_psort(arr: *mut mjtSAP, buf: *mut mjtSAP, n: i32, context: *mut ()) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_SAPsort
}

/// C: mj_SAP (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1400)
/// Calls: cxx-internal:engine_collision_driver.c.o:_SAPsort, cxx:_mj_stackAllocInfo
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_sap(d: *mut mjData, aamm: *const f64, n: i32, axis_x: i32, pair: *mut i32, maxpair: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_SAP
}

/// C: updateCov (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1497)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_cov(cov: *mut f64, vec: *const f64, cen: *const f64) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_updateCov
}

/// C: uintcmp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1518)
#[allow(unused_variables, non_snake_case)]
pub fn uintcmp(i: *mut i32, j: *mut i32, context: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_uintcmp
}

/// C: bfsort (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1529)
/// Calls: cxx-internal:engine_collision_driver.c.o:_uintcmp
#[allow(unused_variables, non_snake_case)]
pub fn bfsort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_bfsort
}

/// C: mj_contactParam (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1694)
/// Calls: cxx:_mju_copy, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_param(m: *const mjModel, condim: *mut i32, solref: *mut f64, solimp: *mut f64, friction: *mut f64, g1: i32, g2: i32, f1: i32, f2: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_contactParam
}

/// C: mj_setContact (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1786)
/// Calls: cxx:_mj_assignFriction, cxx:_mj_assignImp, cxx:_mj_assignRef, cxx:_mju_makeFrame, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_contact(m: *const mjModel, con: *mut mjContact, condim: i32, includemargin: f64, solref: *const f64, solreffriction: *const f64, solimp: *const f64, friction: *const f64) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_setContact
}

/// C: mj_makeCapsule (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1816)
/// Calls: cxx:_mju_add3, cxx:_mju_normalize3, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_capsule(m: *const mjModel, d: *mut mjData, f: i32, vid: *const i32, pos: *mut f64, mat: *mut f64, size: *mut f64) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_makeCapsule
}

/// C: collisionTask (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:1849)
/// Calls: cxx-internal:engine_collision_driver.c.o:_getGap, cxx-internal:engine_collision_driver.c.o:_getMargin, cxx:_mjc_setCCDBuffer, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn collision_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, idx: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_collisionTask
}

/// C: planeVertex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:2129)
/// Calls: cxx:_mju_addScl3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_vertex(con: *mut mjPreContact, pos: *const f64, rad: f64, t0: i32, t1: i32, t2: i32, v: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_planeVertex
}

/// C: mj_maxContact (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.h:33)
#[allow(unused_variables, non_snake_case)]
pub fn mj_max_contact(m: *const mjModel, g1: i32, g2: i32, has_margin: i32) -> i32 {
    todo!() // cxx:_mj_maxContact
}

/// C: mj_collision (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.h:36)
/// Calls: cxx-internal:engine_collision_driver.c.o:_alignArena, cxx-internal:engine_collision_driver.c.o:_canCollide2, cxx-internal:engine_collision_driver.c.o:_contactSort, cxx-internal:engine_collision_driver.c.o:_filterBitmask, cxx-internal:engine_collision_driver.c.o:_filterCollisionPair, cxx-internal:engine_collision_driver.c.o:_filterFlexContacts, cxx-internal:engine_collision_driver.c.o:_mj_collideFlexInternal, cxx-internal:engine_collision_driver.c.o:_mj_collidePlaneFlex, cxx-internal:engine_collision_driver.c.o:_mj_collideSdfFlex, cxx-internal:engine_collision_driver.c.o:_mj_collideTree, cxx-internal:engine_collision_driver.c.o:_mj_narrowphase, cxx-internal:engine_collision_driver.c.o:_pushPairArena, cxx-internal:engine_collision_driver.c.o:_resetArena, cxx:_mj_broadphase, cxx:_mj_collideElems, cxx:_mj_collideFlexSAP, cxx:_mj_collideGeomElem, cxx:_mj_freeStack, cxx:_mj_isElemActive, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mj_collision(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_collision
}

/// C: mj_collideOBB (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.h:39)
/// Calls: cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_obb(aabb1: *const f64, aabb2: *const f64, xpos1: *const f64, xmat1: *const f64, xpos2: *const f64, xmat2: *const f64, margin: f64, product: *mut f64, offset: *mut f64, initialize: *mut bool) -> i32 {
    todo!() // cxx:_mj_collideOBB
}

/// C: mj_isElemActive (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.h:45)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_elem_active(m: *const mjModel, f: i32, e: i32) -> i32 {
    todo!() // cxx:_mj_isElemActive
}

/// C: mj_broadphase (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.h:48)
/// Calls: cxx-internal:engine_collision_driver.c.o:_add_pair, cxx-internal:engine_collision_driver.c.o:_bfsort, cxx-internal:engine_collision_driver.c.o:_canCollide, cxx-internal:engine_collision_driver.c.o:_filterBodyPair, cxx-internal:engine_collision_driver.c.o:_hasPlane, cxx-internal:engine_collision_driver.c.o:_makeAAMM, cxx-internal:engine_collision_driver.c.o:_mj_SAP, cxx-internal:engine_collision_driver.c.o:_updateCov, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_addTo3, cxx:_mju_eig3, cxx:_mju_message, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_broadphase(m: *const mjModel, d: *mut mjData, bfpair: *mut i32, maxpair: i32) -> i32 {
    todo!() // cxx:_mj_broadphase
}

/// C: mj_collideFlexSAP (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.h:51)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_SAP, cxx:_mj_collideElems, cxx:_mj_freeStack, cxx:_mj_isElemActive, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_sap(m: *const mjModel, d: *mut mjData, f: i32) {
    todo!() // cxx:_mj_collideFlexSAP
}

/// C: mj_collideGeomElem (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.h:54)
/// Calls: cxx-internal:engine_collision_driver.c.o:_filterSphereBox, cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_makeCapsule, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_arenaAllocByte, cxx:_mj_assignMargin, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjc_ConvexElem, cxx:_mjc_HFieldElem, cxx:_mjraw_BoxTriangle, cxx:_mjraw_CapsuleBox, cxx:_mjraw_CapsuleCapsule, cxx:_mjraw_CapsuleTriangle, cxx:_mjraw_SphereCapsule, cxx:_mjraw_SphereTriangle, cxx:_mju_copy3, cxx:_mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_geom_elem(m: *const mjModel, d: *mut mjData, g: i32, f: i32, e: i32) {
    todo!() // cxx:_mj_collideGeomElem
}

/// C: mj_collideElems (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.h:57)
/// Calls: cxx-internal:engine_collision_driver.c.o:_filterBox, cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_makeCapsule, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_arenaAllocByte, cxx:_mj_assignMargin, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjc_ConvexElem, cxx:_mjraw_CapsuleCapsule, cxx:_mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elems(m: *const mjModel, d: *mut mjData, f1: i32, e1: i32, f2: i32, e2: i32) {
    todo!() // cxx:_mj_collideElems
}

/// C: mj_collideElemVert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.h:60)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_makeCapsule, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_arenaAllocByte, cxx:_mj_assignMargin, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjc_ConvexElem, cxx:_mjraw_SphereCapsule, cxx:_mjraw_SphereTriangle, cxx:_mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elem_vert(m: *const mjModel, d: *mut mjData, f: i32, e: i32, v: i32) {
    todo!() // cxx:_mj_collideElemVert
}

