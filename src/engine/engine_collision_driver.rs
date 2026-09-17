//! Port of: engine/engine_collision_driver.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_maxContact (engine/engine_collision_driver.c:67)
#[allow(unused_variables, non_snake_case)]
pub fn mj_max_contact(m: *const mjModel, g1: i32, g2: i32, has_margin: i32) -> i32 {
    todo!() // cxx:_mj_maxContact
}

/// C: getMargin (engine/engine_collision_driver.c:160)
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

/// C: getGap (engine/engine_collision_driver.c:169)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_gap(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_getGap
}

/// C: resetArena (engine/engine_collision_driver.c:178)
#[allow(unused_variables, non_snake_case)]
pub fn reset_arena(d: *mut mjData) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_resetArena
}

/// C: alignArena (engine/engine_collision_driver.c:188)
#[allow(unused_variables, non_snake_case)]
pub fn align_arena(d: *mut mjData, alignment: usize) -> usize {
    todo!() // cxx-internal:engine_collision_driver.c.o:_alignArena
}

/// C: planeGeomDist (engine/engine_collision_driver.c:198)
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

/// C: hasPlane (engine/engine_collision_driver.c:209)
#[allow(unused_variables, non_snake_case)]
pub fn has_plane(m: *const mjModel, body: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_hasPlane
}

/// C: filterBitmask (engine/engine_collision_driver.c:226)
#[allow(unused_variables, non_snake_case)]
pub fn filter_bitmask(contype1: i32, conaffinity1: i32, contype2: i32, conaffinity2: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterBitmask
}

/// C: filterBox (engine/engine_collision_driver.c:233)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_box(aabb1: *const f64, aabb2: *const f64, margin: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterBox
}

/// C: filterSphereBox (engine/engine_collision_driver.c:245)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_sphere_box(s: *const f64, bound: f64, aabb: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterSphereBox
}

/// C: filterSphere (engine/engine_collision_driver.c:257)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_sphere(pos1: *const f64, pos2: *const f64, bound: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterSphere
}

/// C: mj_filterSphere (engine/engine_collision_driver.c:266)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterSphere, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_planeGeomDist
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_filter_sphere(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_filterSphere
}

/// C: filterBodyPair (engine/engine_collision_driver.c:287)
#[allow(unused_variables, non_snake_case)]
pub fn filter_body_pair(weldbody1: i32, weldparent1: i32, asleep1: i32, dofnum1: i32, weldbody2: i32, weldparent2: i32, asleep2: i32, dofnum2: i32, dsbl_filterparent: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterBodyPair
}

/// C: canCollide (engine/engine_collision_driver.c:322)
#[allow(unused_variables, non_snake_case)]
pub fn can_collide(m: *const mjModel, bf: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_canCollide
}

/// C: canCollide2 (engine/engine_collision_driver.c:333)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterBitmask
#[allow(unused_variables, non_snake_case)]
pub fn can_collide2(m: *const mjModel, bf1: i32, bf2: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_canCollide2
}

/// C: mj_isElemActive (engine/engine_collision_driver.c:346)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_elem_active(m: *const mjModel, f: i32, e: i32) -> i32 {
    todo!() // cxx:_mj_isElemActive
}

/// C: defaultPair (engine/engine_collision_driver.c:379)
#[allow(unused_variables, non_snake_case)]
pub fn default_pair(pair: *mut mjcPair, r#type: i32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_defaultPair
}

/// C: mj_collideTree (engine/engine_collision_driver.c:390)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_canCollide2, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_defaultPair, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterBitmask, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterBox, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterSphereBox, cxx-internal:engine_collision_driver.c.o:_filterCollisionPair, cxx-internal:engine_collision_driver.c.o:_mj_collidePlaneFlex, cxx-internal:engine_collision_driver.c.o:_mj_collideSdfFlex, cxx-internal:engine_collision_driver.c.o:_mj_filterSphere, cxx-internal:engine_collision_driver.c.o:_mj_narrowphase, cxx:_mj_assignMargin, cxx:_mj_collideElems, cxx:_mj_collideGeomElem, cxx:_mj_collideOBB, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_error, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_tree(m: *const mjModel, d: *mut mjData, bf1: i32, bf2: i32, merged: i32, startadr: i32, pairadr: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_collideTree
}

/// C: mj_broadphase (engine/engine_collision_driver.c:399)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_bfsort, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_canCollide, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterBodyPair, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_hasPlane, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_makeAAMM, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_updateCov, cxx-internal:engine_collision_driver.c.o:_add_pair, cxx-internal:engine_collision_driver.c.o:_mj_SAP, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_addTo3, cxx:_mju_eig3, cxx:_mju_message, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_broadphase(m: *const mjModel, d: *mut mjData, bfpair: *mut mjPacked32, maxpair: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_mj_broadphase
}

/// C: mj_narrowphase (engine/engine_collision_driver.c:404)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_getGap, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_getMargin, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_arenaAllocByte, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_maxContact, cxx:_mj_stackAllocByte, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjc_ccdSize, cxx:_mju_copy, cxx:_mju_dispatch, cxx:_mju_numThread
#[allow(unused_variables, non_snake_case)]
pub fn mj_narrowphase(m: *const mjModel, d: *mut mjData, buffer: *const mjcPair, npair: i32, parena: usize) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_narrowphase
}

/// C: mj_collidePlaneFlex (engine/engine_collision_driver.c:408)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_addContact, cxx:_mj_assignMargin, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_plane_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_collidePlaneFlex
}

/// C: mj_collideSdfFlex (engine/engine_collision_driver.c:411)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_addContact, cxx:_mj_assignMargin, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mjc_FlexSDF, cxx:_mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_sdf_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_collideSdfFlex
}

/// C: mj_collideFlexInternal (engine/engine_collision_driver.c:414)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx-internal:engine_collision_driver.c.o:_planeVertex, cxx:_mj_addContact, cxx:_mj_collideElemVert, cxx:_mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_internal(m: *const mjModel, d: *mut mjData, f: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_collideFlexInternal
}

/// C: contactcompare (engine/engine_collision_driver.c:417)
#[allow(unused_variables, non_snake_case)]
pub fn contactcompare(c1: *const mjContact, c2: *const mjContact, context: *mut ()) -> i32 {
    // SAFETY: caller guarantees c1, c2 are valid mjContact pointers; context is a valid mjModel*
    unsafe {
        let m = context as *const mjModel;

        // get colliding object ids
        let mut con1_obj1 = if (*c1).geom[0] >= 0 { (*c1).geom[0] }
            else if (*c1).elem[0] >= 0 { (*c1).elem[0] }
            else { (*c1).vert[0] };
        let mut con1_obj2 = if (*c1).geom[1] >= 0 { (*c1).geom[1] }
            else if (*c1).elem[1] >= 0 { (*c1).elem[1] }
            else { (*c1).vert[1] };
        let mut con2_obj1 = if (*c2).geom[0] >= 0 { (*c2).geom[0] }
            else if (*c2).elem[0] >= 0 { (*c2).elem[0] }
            else { (*c2).vert[0] };
        let mut con2_obj2 = if (*c2).geom[1] >= 0 { (*c2).geom[1] }
            else if (*c2).elem[1] >= 0 { (*c2).elem[1] }
            else { (*c2).vert[1] };

        // for geom:geom, reproduce the order of contacts without mj_collideTree
        if (*c1).geom[0] >= 0 && (*c1).geom[1] >= 0
            && (*c2).geom[0] >= 0 && (*c2).geom[1] >= 0
        {
            if *(*m).geom_type.add(con1_obj1 as usize) > *(*m).geom_type.add(con1_obj2 as usize) {
                let tmp = con1_obj1;
                con1_obj1 = con1_obj2;
                con1_obj2 = tmp;
            }
            if *(*m).geom_type.add(con2_obj1 as usize) > *(*m).geom_type.add(con2_obj2 as usize) {
                let tmp = con2_obj1;
                con2_obj1 = con2_obj2;
                con2_obj2 = tmp;
            }
        }

        if con1_obj1 < con2_obj1 { return -1; }
        if con1_obj1 > con2_obj1 { return 1; }
        if con1_obj2 < con2_obj2 { return -1; }
        if con1_obj2 > con2_obj2 { return 1; }
        0
    }
}

/// C: contactSort (engine/engine_collision_driver.c:450)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_contactcompare
#[allow(unused_variables, non_snake_case)]
pub fn contact_sort(arr: *mut mjContact, buf: *mut mjContact, n: i32, context: *mut ()) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_contactSort
}

/// C: filterFlexContacts (engine/engine_collision_driver.c:454)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_resetArena, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn filter_flex_contacts(d: *mut mjData, ncon_before: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterFlexContacts
}

/// C: pushPairArena (engine/engine_collision_driver.c:526)
/// Calls: cxx:_mj_arenaAllocByte, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn push_pair_arena(d: *mut mjData, pair: *mut mjcPair) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_pushPairArena
}

/// C: pushGeomGeom (engine/engine_collision_driver.c:537)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_defaultPair, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_pushPairArena
#[allow(unused_variables, non_snake_case)]
pub fn push_geom_geom(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_pushGeomGeom
}

/// C: filterCollisionPair (engine/engine_collision_driver.c:554)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterBitmask, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_getGap, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_getMargin, cxx-internal:engine_collision_driver.c.o:_mj_filterSphere
#[allow(unused_variables, non_snake_case)]
pub fn filter_collision_pair(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32, merged: i32, startadr: i32, pairadr: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_filterCollisionPair
}

/// C: mj_collision (engine/engine_collision_driver.c:602)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_alignArena, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_canCollide2, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_contactSort, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterBitmask, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_mj_broadphase, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_mj_collideFlexInternal, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_pushGeomGeom, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_resetArena, cxx-internal:engine_collision_driver.c.o:_filterCollisionPair, cxx-internal:engine_collision_driver.c.o:_filterFlexContacts, cxx-internal:engine_collision_driver.c.o:_mj_collidePlaneFlex, cxx-internal:engine_collision_driver.c.o:_mj_collideSdfFlex, cxx-internal:engine_collision_driver.c.o:_mj_collideTree, cxx-internal:engine_collision_driver.c.o:_mj_narrowphase, cxx:_mj_collideElems, cxx:_mj_collideFlexSAP, cxx:_mj_collideGeomElem, cxx:_mj_freeStack, cxx:_mj_isElemActive, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mj_collision(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_collision
}

/// C: mj_collideOBB (engine/engine_collision_driver.c:908)
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

/// C: makeAAMM (engine/engine_collision_driver.c:1257)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_transpose3, cxx:_mju_addTo3, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_aamm(m: *const mjModel, d: *mut mjData, x_min: *mut f64, y_min: *mut f64, z_min: *mut f64, x_max: *mut f64, y_max: *mut f64, z_max: *mut f64, bf: i32, frame: *const f64) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_makeAAMM
}

/// C: add_pair (engine/engine_collision_driver.c:1361)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn add_pair(m: *const mjModel, bf1: i32, bf2: i32, npair: *mut i32, pair: *mut mjPacked32, maxpair: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_add_pair
}

/// C: SAPcmp (engine/engine_collision_driver.c:1426)
#[allow(unused_variables, non_snake_case)]
pub fn sa_pcmp(obj1: *mut mjtSAP, obj2: *mut mjtSAP, context: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_SAPcmp
}

/// C: SAPsort (engine/engine_collision_driver.c:1437)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_SAPcmp
#[allow(unused_variables, non_snake_case)]
pub fn sa_psort(arr: *mut mjtSAP, buf: *mut mjtSAP, n: i32, context: *mut ()) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_SAPsort
}

/// C: mj_SAP (engine/engine_collision_driver.c:1443)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_SAPsort, cxx:_mj_stackAllocInfo
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_sap(d: *mut mjData, aamm: *const f64, n: i32, axis_x: i32, pair: *mut mjPacked32, maxpair: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_SAP
}

/// C: updateCov (engine/engine_collision_driver.c:1542)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_cov(cov: *mut f64, vec: *const f64, cen: *const f64) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_updateCov
}

/// C: mjPacked32cmp (engine/engine_collision_driver.c:1563)
#[allow(unused_variables, non_snake_case)]
pub fn mj_packed32cmp(i: *const mjPacked32, j: *const mjPacked32, context: *mut ()) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_mjPacked32cmp
}

/// C: bfsort (engine/engine_collision_driver.c:1572)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_mjPacked32cmp
#[allow(unused_variables, non_snake_case)]
pub fn bfsort(arr: *mut mjPacked32, buf: *mut mjPacked32, n: i32, context: *mut ()) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_bfsort
}

/// C: mj_contactParam (engine/engine_collision_driver.c:1744)
/// Calls: cxx:_mju_copy, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_param(m: *const mjModel, condim: *mut i32, solref: *mut f64, solimp: *mut f64, friction: *mut f64, adhesion: *mut f64, g1: i32, g2: i32, f1: i32, f2: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_contactParam
}

/// C: mj_setContact (engine/engine_collision_driver.c:1843)
/// Calls: cxx:_mj_assignFriction, cxx:_mj_assignImp, cxx:_mj_assignRef, cxx:_mju_makeFrame, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_contact(m: *const mjModel, con: *mut mjContact, condim: i32, includemargin: f64, solref: *const f64, solreffriction: *const f64, solimp: *const f64, friction: *const f64, adhesion: f64) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mj_setContact
}

/// C: mj_makeCapsule (engine/engine_collision_driver.c:1883)
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

/// C: collisionTask (engine/engine_collision_driver.c:1923)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_getGap, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_getMargin, cxx:_mjc_setCCDBuffer, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn collision_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, idx: i32) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_collisionTask
}

/// C: planeVertex (engine/engine_collision_driver.c:2204)
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

/// C: mj_collideFlexSAP (engine/engine_collision_driver.c:2319)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_SAP, cxx:_mj_collideElems, cxx:_mj_freeStack, cxx:_mj_isElemActive, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_sap(m: *const mjModel, d: *mut mjData, f: i32) {
    todo!() // cxx:_mj_collideFlexSAP
}

/// C: mj_collideGeomElem (engine/engine_collision_driver.c:2376)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterSphereBox, cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_makeCapsule, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_arenaAllocByte, cxx:_mj_assignMargin, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjc_ConvexElem, cxx:_mjc_HFieldElem, cxx:_mjraw_BoxTriangle, cxx:_mjraw_CapsuleBox, cxx:_mjraw_CapsuleCapsule, cxx:_mjraw_CapsuleTriangle, cxx:_mjraw_SphereCapsule, cxx:_mjraw_SphereTriangle, cxx:_mju_copy3, cxx:_mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_geom_elem(m: *const mjModel, d: *mut mjData, g: i32, f: i32, e: i32) {
    todo!() // cxx:_mj_collideGeomElem
}

/// C: mj_collideElems (engine/engine_collision_driver.c:2522)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_driver.c:_filterBox, cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_makeCapsule, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_arenaAllocByte, cxx:_mj_assignMargin, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjc_ConvexElem, cxx:_mjraw_CapsuleCapsule, cxx:_mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elems(m: *const mjModel, d: *mut mjData, f1: i32, e1: i32, f2: i32, e2: i32) {
    todo!() // cxx:_mj_collideElems
}

/// C: mj_collideElemVert (engine/engine_collision_driver.c:2624)
/// Calls: cxx-internal:engine_collision_driver.c.o:_mj_contactParam, cxx-internal:engine_collision_driver.c.o:_mj_makeCapsule, cxx-internal:engine_collision_driver.c.o:_mj_setContact, cxx:_mj_arenaAllocByte, cxx:_mj_assignMargin, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjc_ConvexElem, cxx:_mjraw_SphereCapsule, cxx:_mjraw_SphereTriangle, cxx:_mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elem_vert(m: *const mjModel, d: *mut mjData, f: i32, e: i32, v: i32) {
    todo!() // cxx:_mj_collideElemVert
}

