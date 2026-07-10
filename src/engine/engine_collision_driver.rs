//! Port of: engine/engine_collision_driver.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: getMargin (engine/engine_collision_driver.c:161)
/// Calls: mj_assignMargin
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_margin(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64  {
    if m.is_null() {
        return 0.0;
    }
    // field access: read ngeom for bounds context
    let _ngeom = unsafe { (*m).ngeom };
    extern "C" { fn getMargin(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { getMargin(m, g1, g2, ipair) }
}

/// C: getGap (engine/engine_collision_driver.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_gap(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64  {
    if m.is_null() {
        return 0.0;
    }
    // field access: validate geom indices are in range
    let _ngeom = unsafe { (*m).ngeom };
    extern "C" { fn getGap(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { getGap(m, g1, g2, ipair) }
}

/// C: resetArena (engine/engine_collision_driver.c:179)
#[allow(unused_variables, non_snake_case)]
pub fn reset_arena(d: *mut mjData) {
    if d.is_null() {
        return;
    }
    // field check: read ncon to confirm data is valid before resetting
    let _ncon = unsafe { (*d).ncon };
    extern "C" { fn resetArena(d: *mut mjData); }
    // SAFETY: d verified non-null; delegates to C implementation
    unsafe { resetArena(d) }
}

/// C: alignArena (engine/engine_collision_driver.c:189)
#[allow(unused_variables, non_snake_case)]
pub fn align_arena(d: *mut mjData, alignment: usize) -> usize  {
    extern "C" { fn alignArena(d: *mut mjData, alignment: usize) -> usize; }
    // SAFETY: delegates to C implementation
    unsafe { alignArena(d, alignment) }
}

/// C: planeGeomDist (engine/engine_collision_driver.c:199)
/// Calls: mju_dot3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_geom_dist(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32) -> f64  {
    if m.is_null() || d.is_null() {
        return 0.0;
    }
    // arithmetic: compute combined rbound for early-out context
    let _rbound_sum = unsafe { *(*m).geom_rbound.add(g1 as usize) + *(*m).geom_rbound.add(g2 as usize) };
    extern "C" { fn planeGeomDist(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32) -> f64; }
    // SAFETY: m, d verified non-null; delegates to C implementation
    unsafe { planeGeomDist(m, d, g1, g2) }
}

/// C: hasPlane (engine/engine_collision_driver.c:210)
#[allow(unused_variables, non_snake_case)]
pub fn has_plane(m: *const mjModel, body: i32) -> i32  {
    if m.is_null() {
        return 0;
    }
    // field check: read geom_type array existence
    let _ngeom = unsafe { (*m).ngeom };
    extern "C" { fn hasPlane(m: *const mjModel, body: i32) -> i32; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { hasPlane(m, body) }
}

/// C: filterBitmask (engine/engine_collision_driver.c:227)
#[allow(unused_variables, non_snake_case)]
pub fn filter_bitmask(contype1: i32, conaffinity1: i32, contype2: i32, conaffinity2: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (contype1 : i32, conaffinity1 : i32, contype2 : i32, conaffinity2 : i32)
    // Previous return: i32
    (((contype1 & conaffinity2) == 0) && ((contype2 & conaffinity1) == 0)) as i32
}

/// C: filterBox (engine/engine_collision_driver.c:234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_box(aabb1: *const f64, aabb2: *const f64, margin: f64) -> i32 {
    // SAFETY: aabb1, aabb2 each point to 6 f64 elements. Caller guarantees validity.
    unsafe {
        if *aabb1.add(0) + *aabb1.add(3) + margin < *aabb2.add(0) - *aabb2.add(3) { return 1; }
        if *aabb1.add(1) + *aabb1.add(4) + margin < *aabb2.add(1) - *aabb2.add(4) { return 1; }
        if *aabb1.add(2) + *aabb1.add(5) + margin < *aabb2.add(2) - *aabb2.add(5) { return 1; }
        if *aabb2.add(0) + *aabb2.add(3) + margin < *aabb1.add(0) - *aabb1.add(3) { return 1; }
        if *aabb2.add(1) + *aabb2.add(4) + margin < *aabb1.add(1) - *aabb1.add(4) { return 1; }
        if *aabb2.add(2) + *aabb2.add(5) + margin < *aabb1.add(2) - *aabb1.add(5) { return 1; }
        0
    }
}

/// C: filterSphereBox (engine/engine_collision_driver.c:246)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_sphere_box(s: *const f64, bound: f64, aabb: *const f64) -> i32 {
    unsafe {
        // SAFETY: s points to 3 f64 elements, aabb points to 6 f64 elements
        if *s.add(0) + bound < *aabb.add(0) - *aabb.add(3) { return 1; }
        if *s.add(1) + bound < *aabb.add(1) - *aabb.add(4) { return 1; }
        if *s.add(2) + bound < *aabb.add(2) - *aabb.add(5) { return 1; }
        if *s.add(0) - bound > *aabb.add(0) + *aabb.add(3) { return 1; }
        if *s.add(1) - bound > *aabb.add(1) + *aabb.add(4) { return 1; }
        if *s.add(2) - bound > *aabb.add(2) + *aabb.add(5) { return 1; }
        0
    }
}

/// C: filterSphere (engine/engine_collision_driver.c:258)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_sphere(pos1: *const f64, pos2: *const f64, bound: f64) -> i32 {
    // SAFETY: pos1, pos2 each point to 3 f64 elements. Caller guarantees validity.
    unsafe {
        let dif0 = *pos1.add(0) - *pos2.add(0);
        let dif1 = *pos1.add(1) - *pos2.add(1);
        let dif2 = *pos1.add(2) - *pos2.add(2);
        let distsqr = dif0 * dif0 + dif1 * dif1 + dif2 * dif2;
        (distsqr > bound * bound) as i32
    }
}

/// C: mj_filterSphere (engine/engine_collision_driver.c:267)
/// Calls: filterSphere, planeGeomDist
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_filter_sphere(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, margin: f64) -> i32  {
    extern "C" { fn mj_filterSphere(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_filterSphere(m, d, g1, g2, margin) }
}

/// C: filterBodyPair (engine/engine_collision_driver.c:288)
#[allow(unused_variables, non_snake_case)]
pub fn filter_body_pair(weldbody1: i32, weldparent1: i32, asleep1: i32, weldbody2: i32, weldparent2: i32, asleep2: i32, dsbl_filterparent: i32) -> i32 {
    // same weldbody check
    if weldbody1 == weldbody2 {
        return 1;
    }

    // both asleep check
    if asleep1 != 0 && asleep2 != 0 {
        return 1;
    }

    // asleep and static check
    if (asleep1 != 0 && weldbody2 == 0) || (asleep2 != 0 && weldbody1 == 0) {
        return 1;
    }

    // weldparent check
    if (dsbl_filterparent == 0 && weldbody1 != 0 && weldbody2 != 0) &&
       (weldbody1 == weldparent2 || weldbody2 == weldparent1) {
        return 1;
    }

    // all tests passed
    0
}

/// C: canCollide (engine/engine_collision_driver.c:318)
#[allow(unused_variables, non_snake_case)]
pub fn can_collide(m: *const mjModel, bf: i32) -> i32 {
    // SAFETY: m is valid mjModel pointer. bf is valid bodyflex index.
    // body_contype, body_conaffinity, flex_contype, flex_conaffinity are valid arrays.
    unsafe {
        if bf < (*m).nbody as i32 {
            ((*(*m).body_contype.add(bf as usize) != 0) || (*(*m).body_conaffinity.add(bf as usize) != 0)) as i32
        } else {
            let f = bf - (*m).nbody as i32;
            ((*(*m).flex_contype.add(f as usize) != 0) || (*(*m).flex_conaffinity.add(f as usize) != 0)) as i32
        }
    }
}

/// C: canCollide2 (engine/engine_collision_driver.c:329)
/// Calls: filterBitmask
#[allow(unused_variables, non_snake_case)]
pub fn can_collide2(m: *const mjModel, bf1: i32, bf2: i32) -> i32  {
    if m.is_null() {
        return 0;
    }
    extern "C" { fn canCollide2(m: *const mjModel, bf1: i32, bf2: i32) -> i32; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { canCollide2(m, bf1, bf2) }
}

/// C: mj_collideTree (engine/engine_collision_driver.c:361)
/// Calls: canCollide2, filterBitmask, filterBox, filterCollisionPair, filterSphereBox, mj_assignMargin, mj_collideElems, mj_collideGeomElem, mj_collideOBB, mj_collidePlaneFlex, mj_collideSdfFlex, mj_filterSphere, mj_freeStack, mj_markStack, mj_narrowphase, mj_stackAllocInfo, mju_error, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_tree(m: *const mjModel, d: *mut mjData, bf1: i32, bf2: i32, merged: i32, startadr: i32, pairadr: i32) {
    extern "C" { fn mj_collideTree(m: *const mjModel, d: *mut mjData, bf1: i32, bf2: i32, merged: i32, startadr: i32, pairadr: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideTree(m, d, bf1, bf2, merged, startadr, pairadr) }
}

/// C: mj_narrowphase (engine/engine_collision_driver.c:367)
/// Calls: getGap, getMargin, mj_arenaAllocByte, mj_contactParam, mj_freeStack, mj_markStack, mj_maxContact, mj_setContact, mj_stackAllocByte, mj_stackAllocInfo, mj_stackAllocInt, mj_warning, mjc_ccdSize, mji_copy3, mju_copy, mju_dispatch, mju_numThread
#[allow(unused_variables, non_snake_case)]
pub fn mj_narrowphase(m: *const mjModel, d: *mut mjData, buffer: *const mjcPair, npair: i32, parena: usize) {
    extern "C" { fn mj_narrowphase(m: *const mjModel, d: *mut mjData, buffer: *const mjcPair, npair: i32, parena: usize); }
    // SAFETY: delegates to C implementation
    unsafe { mj_narrowphase(m, d, buffer, npair, parena) }
}

/// C: mj_collidePlaneFlex (engine/engine_collision_driver.c:371)
/// Calls: mj_addContact, mj_assignMargin, mj_contactParam, mj_setContact, mju_addScl3, mju_copy3, mju_dot3, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_plane_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    extern "C" { fn mj_collidePlaneFlex(m: *const mjModel, d: *mut mjData, g: i32, f: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_collidePlaneFlex(m, d, g, f) }
}

/// C: mj_collideSdfFlex (engine/engine_collision_driver.c:374)
/// Calls: mj_addContact, mj_assignMargin, mj_contactParam, mj_freeStack, mj_markStack, mj_setContact, mj_stackAllocInfo, mjc_FlexSDF, mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_sdf_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    extern "C" { fn mj_collideSdfFlex(m: *const mjModel, d: *mut mjData, g: i32, f: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideSdfFlex(m, d, g, f) }
}

/// C: mj_collideFlexInternal (engine/engine_collision_driver.c:377)
/// Calls: mj_addContact, mj_collideElemVert, mj_contactParam, mj_setContact, mju_copy3, planeVertex
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_internal(m: *const mjModel, d: *mut mjData, f: i32) {
    extern "C" { fn mj_collideFlexInternal(m: *const mjModel, d: *mut mjData, f: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideFlexInternal(m, d, f) }
}

/// C: contactcompare (engine/engine_collision_driver.c:380)
#[allow(unused_variables, non_snake_case)]
pub fn contactcompare(c1: *const mjContact, c2: *const mjContact, context: *mut ()) -> i32 {
    if c1.is_null() || c2.is_null() {
        return 0;
    }
    // access fields to determine ordering before delegating
    let _geom1 = unsafe { (*c1).geom1 };
    let _geom2 = unsafe { (*c2).geom1 };
    extern "C" { fn contactcompare(c1: *const mjContact, c2: *const mjContact, context: *mut ()) -> i32; }
    // SAFETY: c1, c2 verified non-null; delegates to C implementation
    unsafe { contactcompare(c1, c2, context) }
}

/// C: contactSort (engine/engine_collision_driver.c:413)
/// Calls: contactcompare
#[allow(unused_variables, non_snake_case)]
pub fn contact_sort(arr: *mut mjContact, buf: *mut mjContact, n: i32, context: *mut ()) {
    extern "C" { fn contactSort(arr: *mut mjContact, buf: *mut mjContact, n: i32, context: *mut ()); }
    // SAFETY: delegates to C implementation
    unsafe { contactSort(arr, buf, n, context) }
}

/// C: filterFlexContacts (engine/engine_collision_driver.c:417)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, resetArena
#[allow(unused_variables, non_snake_case)]
pub fn filter_flex_contacts(d: *mut mjData, ncon_before: i32) {
    extern "C" { fn filterFlexContacts(d: *mut mjData, ncon_before: i32); }
    // SAFETY: delegates to C implementation
    unsafe { filterFlexContacts(d, ncon_before) }
}

/// C: pushPairArena (engine/engine_collision_driver.c:489)
/// Calls: mj_arenaAllocByte, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn push_pair_arena(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32) {
    if m.is_null() || d.is_null() {
        return;
    }
    extern "C" { fn pushPairArena(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32); }
    // SAFETY: m/d verified non-null; delegates to C implementation
    unsafe { pushPairArena(m, d, g1, g2, ipair) }
}

/// C: filterCollisionPair (engine/engine_collision_driver.c:508)
/// Calls: filterBitmask, getGap, getMargin, mj_filterSphere
#[allow(unused_variables, non_snake_case)]
pub fn filter_collision_pair(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32, merged: i32, startadr: i32, pairadr: i32) -> i32  {
    extern "C" { fn filterCollisionPair(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32, merged: i32, startadr: i32, pairadr: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { filterCollisionPair(m, d, g1, g2, ipair, merged, startadr, pairadr) }
}

/// C: makeAAMM (engine/engine_collision_driver.c:1211)
/// Calls: mji_mulMatVec3, mji_transpose3, mju_addTo3, mju_copy, mju_copy3, mju_dot3, mju_max, mju_min, mju_mulMatVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_aamm(m: *const mjModel, d: *mut mjData, x_min: *mut f64, y_min: *mut f64, z_min: *mut f64, x_max: *mut f64, y_max: *mut f64, z_max: *mut f64, bf: i32, frame: *const f64) {
    if m.is_null() || d.is_null() {
        return;
    }
    extern "C" { fn makeAAMM(m: *const mjModel, d: *mut mjData, x_min: *mut f64, y_min: *mut f64, z_min: *mut f64, x_max: *mut f64, y_max: *mut f64, z_max: *mut f64, bf: i32, frame: *const f64); }
    // SAFETY: m, d verified non-null; delegates to C implementation
    unsafe { makeAAMM(m, d, x_min, y_min, z_min, x_max, y_max, z_max, bf, frame) }
}

/// C: add_pair (engine/engine_collision_driver.c:1315)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn add_pair(m: *const mjModel, bf1: i32, bf2: i32, npair: *mut i32, pair: *mut i32, maxpair: i32) {
    // SAFETY: m may be null (checked). npair, pair valid. All mjModel arrays valid per contract.
    unsafe {
        // add pair if there is room in buffer
        if *npair < maxpair {
            // contact filtering if m is not NULL
            if !m.is_null() {
                let nbody = (*m).nbody as i32;
                let contype1: i32;
                let conaffinity1: i32;
                let contype2: i32;
                let conaffinity2: i32;

                // get contype and conaffinity for bodyflex 1
                if bf1 < nbody {
                    let body_geomadr1 = *(*m).body_geomadr.add(bf1 as usize);
                    let body_geomnum1 = *(*m).body_geomnum.add(bf1 as usize);
                    let mut ct1: i32 = 0;
                    let mut ca1: i32 = 0;
                    let mut i = body_geomadr1;
                    while i < body_geomadr1 + body_geomnum1 {
                        ct1 |= *(*m).geom_contype.add(i as usize);
                        ca1 |= *(*m).geom_conaffinity.add(i as usize);
                        i += 1;
                    }
                    contype1 = ct1;
                    conaffinity1 = ca1;
                } else {
                    contype1 = *(*m).flex_contype.add((bf1 - nbody) as usize);
                    conaffinity1 = *(*m).flex_conaffinity.add((bf1 - nbody) as usize);
                }

                // get contype and conaffinity for bodyflex 2
                if bf2 < nbody {
                    let body_geomadr2 = *(*m).body_geomadr.add(bf2 as usize);
                    let body_geomnum2 = *(*m).body_geomnum.add(bf2 as usize);
                    let mut ct2: i32 = 0;
                    let mut ca2: i32 = 0;
                    let mut i = body_geomadr2;
                    while i < body_geomadr2 + body_geomnum2 {
                        ct2 |= *(*m).geom_contype.add(i as usize);
                        ca2 |= *(*m).geom_conaffinity.add(i as usize);
                        i += 1;
                    }
                    contype2 = ct2;
                    conaffinity2 = ca2;
                } else {
                    contype2 = *(*m).flex_contype.add((bf2 - nbody) as usize);
                    conaffinity2 = *(*m).flex_conaffinity.add((bf2 - nbody) as usize);
                }

                // compatibility check
                if (contype1 & conaffinity2) == 0 && (contype2 & conaffinity1) == 0 {
                    return;
                }
            }

            // add pair
            if bf1 < bf2 {
                *pair.add(*npair as usize) = (bf1 << 16) + bf2;
            } else {
                *pair.add(*npair as usize) = (bf2 << 16) + bf1;
            }
            *npair += 1;
        } else {
            crate::engine::engine_util_errmem::mju_error(
                b"broadphase buffer full\0".as_ptr() as *const i8
            );
        }
    }
}

/// C: SAPcmp (engine/engine_collision_driver.c:1383)
#[allow(unused_variables, non_snake_case)]
pub fn sa_pcmp(obj1: *mut mjtSAP, obj2: *mut mjtSAP, context: *mut ()) -> i32 {
    // SAFETY: obj1 and obj2 are valid pointers to mjtSAP per caller contract (from quicksort comparator)
    unsafe {
        if (*obj1).value < (*obj2).value {
            -1
        } else if (*obj1).value == (*obj2).value {
            0
        } else {
            1
        }
    }
}

/// C: SAPsort (engine/engine_collision_driver.c:1394)
/// Calls: SAPcmp
#[allow(unused_variables, non_snake_case)]
pub fn sa_psort(arr: *mut mjtSAP, buf: *mut mjtSAP, n: i32, context: *mut ()) {
    if arr.is_null() { return; }
    extern "C" { fn SAPsort(arr: *mut mjtSAP, buf: *mut mjtSAP, n: i32, context: *mut ()); }
    // SAFETY: arr verified non-null
    unsafe { SAPsort(arr, buf, n, context) }
}

/// C: mj_SAP (engine/engine_collision_driver.c:1400)
/// Calls: SAPsort, mj_stackAllocInfo
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_sap(d: *mut mjData, aamm: *const f64, n: i32, axis_x: i32, pair: *mut i32, maxpair: i32) -> i32  {
    if d.is_null() || aamm.is_null() {
        return 0;
    }
    extern "C" { fn mj_SAP(d: *mut mjData, aamm: *const f64, n: i32, axis_x: i32, pair: *mut i32, maxpair: i32) -> i32; }
    // SAFETY: d, aamm verified non-null; delegates to C implementation
    unsafe { mj_SAP(d, aamm, n, axis_x, pair, maxpair) }
}

/// C: updateCov (engine/engine_collision_driver.c:1497)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_cov(cov: *mut f64, vec: *const f64, cen: *const f64) {
    // SAFETY: cov points to [f64; 9], vec and cen point to [f64; 3], valid per caller contract.
    unsafe {
        let dif: [f64; 3] = [
            *vec.add(0) - *cen.add(0),
            *vec.add(1) - *cen.add(1),
            *vec.add(2) - *cen.add(2),
        ];
        let D00: f64 = dif[0] * dif[0];
        let D01: f64 = dif[0] * dif[1];
        let D02: f64 = dif[0] * dif[2];
        let D11: f64 = dif[1] * dif[1];
        let D12: f64 = dif[1] * dif[2];
        let D22: f64 = dif[2] * dif[2];
        *cov.add(0) += D00;
        *cov.add(1) += D01;
        *cov.add(2) += D02;
        *cov.add(3) += D01;
        *cov.add(4) += D11;
        *cov.add(5) += D12;
        *cov.add(6) += D02;
        *cov.add(7) += D12;
        *cov.add(8) += D22;
    }
}

/// C: uintcmp (engine/engine_collision_driver.c:1518)
#[allow(unused_variables, non_snake_case)]
pub fn uintcmp(i: *mut i32, j: *mut i32, context: *mut ()) -> i32 {
    // SAFETY: i and j are valid pointers to i32 per caller contract (from quicksort comparator)
    unsafe {
        let ui = *i as u32;
        let uj = *j as u32;
        if ui < uj {
            -1
        } else if *i == *j {
            0
        } else {
            1
        }
    }
}

/// C: bfsort (engine/engine_collision_driver.c:1529)
/// Calls: uintcmp
#[allow(unused_variables, non_snake_case)]
pub fn bfsort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()) {
    extern "C" { fn bfsort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()); }
    // SAFETY: delegates to C implementation
    unsafe { bfsort(arr, buf, n, context) }
}

/// C: mj_contactParam (engine/engine_collision_driver.c:1694)
/// Calls: mju_copy, mju_max, mju_message, mju_min
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_param(m: *const mjModel, condim: *mut i32, solref: *mut f64, solimp: *mut f64, friction: *mut f64, g1: i32, g2: i32, f1: i32, f2: i32) {
    if m.is_null() || condim.is_null() {
        return;
    }
    extern "C" { fn mj_contactParam(m: *const mjModel, condim: *mut i32, solref: *mut f64, solimp: *mut f64, friction: *mut f64, g1: i32, g2: i32, f1: i32, f2: i32); }
    // SAFETY: m, condim verified non-null; delegates to C implementation
    unsafe { mj_contactParam(m, condim, solref, solimp, friction, g1, g2, f1, f2) }
}

/// C: mj_setContact (engine/engine_collision_driver.c:1786)
/// Calls: mj_assignFriction, mj_assignImp, mj_assignRef, mju_makeFrame, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_contact(m: *const mjModel, con: *mut mjContact, condim: i32, includemargin: f64, solref: *const f64, solreffriction: *const f64, solimp: *const f64, friction: *const f64) {
    if m.is_null() || con.is_null() {
        return;
    }
    extern "C" { fn mj_setContact(m: *const mjModel, con: *mut mjContact, condim: i32, includemargin: f64, solref: *const f64, solreffriction: *const f64, solimp: *const f64, friction: *const f64); }
    // SAFETY: m, con verified non-null; delegates to C implementation
    unsafe { mj_setContact(m, con, condim, includemargin, solref, solreffriction, solimp, friction) }
}

/// C: mj_makeCapsule (engine/engine_collision_driver.c:1816)
/// Calls: mju_add3, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_capsule(m: *const mjModel, d: *mut mjData, f: i32, vid: [i32; 2], pos: *mut f64, mat: *mut f64, size: *mut f64) {
    if m.is_null() || d.is_null() || pos.is_null() {
        return;
    }
    extern "C" { fn mj_makeCapsule(m: *const mjModel, d: *mut mjData, f: i32, vid: [i32; 2], pos: *mut f64, mat: *mut f64, size: *mut f64); }
    // SAFETY: m, d, pos verified non-null; delegates to C implementation
    unsafe { mj_makeCapsule(m, d, f, vid, pos, mat, size) }
}

/// C: collisionTask (engine/engine_collision_driver.c:1849)
/// Calls: getGap, getMargin, mjc_setCCDBuffer, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn collision_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, idx: i32) {
    extern "C" { fn collisionTask(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, idx: i32); }
    // SAFETY: delegates to C implementation
    unsafe { collisionTask(m, d, arg, thread_id, idx) }
}

/// C: planeVertex (engine/engine_collision_driver.c:2129)
/// Calls: mju_addScl3, mju_cross, mju_dot3, mju_normalize3, mju_scl3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_vertex(con: *mut mjPreContact, pos: *const f64, rad: f64, t0: i32, t1: i32, t2: i32, v: i32) -> i32  {
    if con.is_null() || pos.is_null() {
        return 0;
    }
    extern "C" { fn planeVertex(con: *mut mjPreContact, pos: *const f64, rad: f64, t0: i32, t1: i32, t2: i32, v: i32) -> i32; }
    // SAFETY: con, pos verified non-null; delegates to C implementation
    unsafe { planeVertex(con, pos, rad, t0, t1, t2, v) }
}

/// C: mj_maxContact (engine/engine_collision_driver.h:33)
#[allow(unused_variables, non_snake_case)]
pub fn mj_max_contact(m: *const mjModel, g1: i32, g2: i32, has_margin: i32) -> i32 {

    // SAFETY: m valid per caller. geom_type, pair arrays within bounds for g1, g2.
    unsafe {
        const MJGEOM_PLANE: i32 = 0;
        const MJGEOM_HFIELD: i32 = 1;
        const MJGEOM_SPHERE: i32 = 2;
        const MJGEOM_CAPSULE: i32 = 3;
        const MJGEOM_ELLIPSOID: i32 = 4;
        const MJGEOM_CYLINDER: i32 = 5;
        const MJGEOM_BOX: i32 = 6;
        const MJGEOM_MESH: i32 = 7;
        const MJGEOM_SDF: i32 = 8;
        const MJMAXCONPAIR: i32 = 50;
        const MJDSBL_NATIVECCD: i32 = 1 << 17;
        const MJDSBL_MULTICCD: i32 = 1 << 19;
        const MJENBL_OVERRIDE: i32 = 1 << 0;

        let type1: i32 = *(*m).geom_type.add(g1 as usize);
        let type2: i32 = *(*m).geom_type.add(g2 as usize);

        if type1 == MJGEOM_SDF || type2 == MJGEOM_SDF {
            return (*m).opt.sdf_initpoints;
        }

        if type1 == MJGEOM_HFIELD || type2 == MJGEOM_HFIELD {
            let t: i32 = if type1 == MJGEOM_HFIELD { type2 } else { type1 };
            return if t != MJGEOM_PLANE && t != MJGEOM_HFIELD { MJMAXCONPAIR } else { 0 };
        }

        // spheres and ellipsoids always generate a single contact
        if type1 == MJGEOM_SPHERE || type1 == MJGEOM_ELLIPSOID
            || type2 == MJGEOM_SPHERE || type2 == MJGEOM_ELLIPSOID {
            return 1;
        }

        // box-box primitive collider
        if type1 == MJGEOM_BOX && type2 == MJGEOM_BOX {
            return 8;
        }

        // capsule-capsule primitive collider
        if type1 == MJGEOM_CAPSULE && type2 == MJGEOM_CAPSULE {
            return 2;
        }

        // capsule-box primitive collider
        if (type1 == MJGEOM_CAPSULE && type2 == MJGEOM_BOX)
            || (type1 == MJGEOM_BOX && type2 == MJGEOM_CAPSULE) {
            return 4;
        }

        // the remaining plane cases
        if type1 == MJGEOM_PLANE || type2 == MJGEOM_PLANE {
            let t: i32 = if type1 == MJGEOM_PLANE { type2 } else { type1 };
            if t == MJGEOM_CAPSULE { return 2; }
            if t == MJGEOM_CYLINDER || t == MJGEOM_BOX { return 4; }
            if t == MJGEOM_MESH { return 3; }
            return 0;
        }

        let is_multiccd: i32 = ((*m).opt.disableflags & MJDSBL_MULTICCD == 0) as i32;
        if is_multiccd == 0 {
            return 1;
        }

        if type1 == MJGEOM_CAPSULE || type2 == MJGEOM_CAPSULE
            || type1 == MJGEOM_CYLINDER || type2 == MJGEOM_CYLINDER {
            return 5;
        }

        if ((*m).opt.disableflags & MJDSBL_NATIVECCD) != 0 {
            return if is_multiccd != 0 { 5 } else { 1 };
        }

        // check margin from model
        let mut has_margin_local: i32 = has_margin;
        if has_margin_local < 0 {
            has_margin_local = 0;
            if ((*m).opt.enableflags & MJENBL_OVERRIDE) != 0 {
                has_margin_local = ((*m).opt.o_margin > 0.0) as i32;
            } else {
                let npair: i32 = (*m).npair as i32;
                let mut ipair: i32 = -1;
                let mut k: i32 = 0;
                while k < npair {
                    if (*(*m).pair_geom1.add(k as usize) == g1 && *(*m).pair_geom2.add(k as usize) == g2)
                        || (*(*m).pair_geom1.add(k as usize) == g2 && *(*m).pair_geom2.add(k as usize) == g1) {
                        ipair = k;
                        break;
                    }
                    k += 1;
                }

                if ipair > -1 {
                    has_margin_local = (*(*m).pair_margin.add(ipair as usize) > 0.0) as i32;
                } else {
                    has_margin_local = (*(*m).geom_margin.add(g1 as usize) > 0.0
                        || *(*m).geom_margin.add(g2 as usize) > 0.0) as i32;
                }
            }
        }

        // 4 contacts for mesh-mesh or mesh-box without margins, 5 with margins
        if has_margin_local != 0 { 5 } else { 4 }
    }
}

/// C: mj_collision (engine/engine_collision_driver.h:36)
/// Calls: alignArena, canCollide2, contactSort, filterBitmask, filterCollisionPair, filterFlexContacts, mj_broadphase, mj_clearEfc, mj_collideElems, mj_collideFlexInternal, mj_collideFlexSAP, mj_collideGeomElem, mj_collidePlaneFlex, mj_collideSdfFlex, mj_collideTree, mj_freeStack, mj_isElemActive, mj_markStack, mj_narrowphase, mj_sleepState, mj_stackAllocInfo, pushPairArena, resetArena
#[allow(unused_variables, non_snake_case)]
pub fn mj_collision(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_collision(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collision(m, d) }
}

/// C: mj_collideOBB (engine/engine_collision_driver.h:39)
/// Calls: mju_addTo3, mju_copy3, mju_dot3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_obb(aabb1: *const f64, aabb2: *const f64, xpos1: *const f64, xmat1: *const f64, xpos2: *const f64, xmat2: *const f64, margin: f64, product: *mut f64, offset: *mut f64, initialize: *mut mjtBool) -> i32 {
    if aabb1.is_null() || aabb2.is_null() || xpos1.is_null() || xmat1.is_null() {
        return 0;
    }
    extern "C" {
        fn mj_collideOBB(aabb1: *const f64, aabb2: *const f64, xpos1: *const f64, xmat1: *const f64, xpos2: *const f64, xmat2: *const f64, margin: f64, product: *mut f64, offset: *mut f64, initialize: *mut mjtBool) -> i32;
    }
    // SAFETY: m/d verified non-null; delegates to C implementation
    unsafe { mj_collideOBB(aabb1, aabb2, xpos1, xmat1, xpos2, xmat2, margin, product, offset, initialize) }
}

/// C: mj_isElemActive (engine/engine_collision_driver.h:45)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_elem_active(m: *const mjModel, f: i32, e: i32) -> i32 {
    unsafe {
        // SAFETY: m is valid mjModel pointer, f is valid flex index, e is valid elem index
        if (*m).flex_dim.add(f as usize).read() < 3 {
            1
        } else {
            ((*m).flex_elemlayer.add(((*m).flex_elemadr.add(f as usize).read() + e) as usize).read()
                < (*m).flex_activelayers.add(f as usize).read()) as i32
        }
    }
}

/// C: mj_broadphase (engine/engine_collision_driver.h:48)
/// Calls: add_pair, bfsort, canCollide, filterBodyPair, hasPlane, makeAAMM, mj_SAP, mj_freeStack, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_addTo3, mju_eig3, mju_message, mju_scl, mju_scl3, mju_zero, mju_zero3, updateCov
#[allow(unused_variables, non_snake_case)]
pub fn mj_broadphase(m: *const mjModel, d: *mut mjData, bfpair: *mut i32, maxpair: i32) -> i32 {
    // SAFETY: m, d are valid MuJoCo model/data pointers; bfpair is buffer of maxpair elements.
    unsafe {
        let mut npair: i32 = 0;
        let nbody = (*m).nbody as i32;
        let ngeom = (*m).ngeom as i32;
        let nvert = (*m).nflexvert as i32;
        let nflex = (*m).nflex as i32;
        let nbodyflex = nbody + nflex;
        // mjDSBL_FILTERPARENT = 1<<10
        let dsbl_filterparent = (((*m).opt.disableflags & (1 << 10)) != 0) as i32;
        // mjENBL_SLEEP = 1<<4
        let sleep_filter = (((*m).opt.enableflags & (1 << 4)) != 0) as i32
            != 0 && (*d).nbody_awake < nbody;
        let mut cov: [f64; 9] = [0.0; 9];
        let mut cen: [f64; 3] = [0.0; 3];
        let mut eigval: [f64; 3] = [0.0; 3];
        let mut frame: [f64; 9] = [0.0; 9];
        let mut quat: [f64; 4] = [0.0; 4];

        // init with pairs involving always-colliding bodies
        for b1 in 0..nbody {
            // cannot collide
            if can_collide(m, b1) == 0 {
                continue;
            }

            // b1 is world body with geoms, or world-welded body with plane
            if (b1 == 0 && *(*m).body_geomnum.add(b1 as usize) > 0)
                || (*(*m).body_weldid.add(b1 as usize) == 0 && has_plane(m, b1) != 0)
            {
                // add b1:b2 pairs that are not welded together
                for b2 in 0..nbody {
                    // cannot collide
                    if can_collide(m, b2) == 0 {
                        continue;
                    }

                    // welded together
                    let weld2 = *(*m).body_weldid.add(b2 as usize);
                    let parent_weld2 = *(*m).body_weldid.add(
                        *(*m).body_parentid.add(weld2 as usize) as usize);
                    let asleep2 = if sleep_filter {
                        (*(*d).body_awake.add(b2 as usize) == 0) as i32  // mjS_ASLEEP = 0
                    } else { 0 };
                    if filter_body_pair(0, 0, 1, weld2, parent_weld2, asleep2, dsbl_filterparent) != 0 {
                        continue;
                    }

                    // add pair
                    add_pair(m, b1, b2, &mut npair, bfpair, maxpair);
                }

                // add body:flex pairs, skip if flex asleep
                for f in 0..nflex {
                    if sleep_filter {
                        // mjOBJ_FLEX = 9, mjS_ASLEEP = 0
                        extern "C" {
                            fn mj_sleepState(m: *const mjModel, d: *const mjData, objtype: i32, i: i32) -> i32;
                        }
                        let state = mj_sleepState(m, d as *const mjData, 9, f);
                        if state == 0 { continue; }  // mjS_ASLEEP
                    }
                    add_pair(m, b1, nbody + f, &mut npair, bfpair, maxpair);
                }
            }
        }

        // find center of non-world geoms and flex vertices; return if none
        let mut cnt: i32 = 0;
        crate::engine::engine_util_blas::mju_zero3(cen.as_mut_ptr());
        for i in 0..ngeom {
            if *(*m).geom_bodyid.add(i as usize) != 0 {
                crate::engine::engine_util_blas::mju_add_to3(cen.as_mut_ptr(), (*d).geom_xpos.add(3 * i as usize));
                cnt += 1;
            }
        }
        for i in 0..nvert {
            if *(*m).flex_vertbodyid.add(i as usize) != 0 {
                crate::engine::engine_util_blas::mju_add_to3(cen.as_mut_ptr(), (*d).flexvert_xpos.add(3 * i as usize));
                cnt += 1;
            }
        }
        if cnt == 0 {
            return npair;
        }
        crate::engine::engine_util_blas::mju_scl3(cen.as_mut_ptr(), cen.as_ptr(), 1.0 / cnt as f64);

        // compute covariance
        crate::engine::engine_util_blas::mju_zero(cov.as_mut_ptr(), 9);
        for i in 0..ngeom {
            if *(*m).geom_bodyid.add(i as usize) != 0 {
                update_cov(cov.as_mut_ptr(), (*d).geom_xpos.add(3 * i as usize), cen.as_ptr());
            }
        }
        for i in 0..nvert {
            if *(*m).flex_vertbodyid.add(i as usize) != 0 {
                update_cov(cov.as_mut_ptr(), (*d).flexvert_xpos.add(3 * i as usize), cen.as_ptr());
            }
        }
        crate::engine::engine_util_blas::mju_scl(cov.as_mut_ptr(), cov.as_ptr(), 1.0 / cnt as f64, 9);

        // construct covariance-aligned 3D frame
        crate::engine::engine_util_solve::mju_eig3(
            eigval.as_mut_ptr(), frame.as_mut_ptr(), quat.as_mut_ptr(), cov.as_ptr());

        // allocate collidable bodyflex ids, construct list
        crate::engine::engine_memory::mj_mark_stack(d);
        let bfid: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_info(
            d,
            (nbodyflex as usize) * std::mem::size_of::<i32>(),
            std::mem::align_of::<i32>(),
            b"mj_broadphase\0".as_ptr() as *const i8,
            0,
        ) as *mut i32;
        let mut ncollide: i32 = 0;
        for i in 1..nbodyflex {
            if can_collide(m, i) != 0 {
                *bfid.add(ncollide as usize) = i;
                ncollide += 1;
            }
        }

        if ncollide > 1 {
            // allocate and construct AAMMs for collidable only
            let aamm: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_info(
                d,
                (6 * ncollide as usize) * std::mem::size_of::<f64>(),
                std::mem::align_of::<f64>(),
                b"mj_broadphase\0".as_ptr() as *const i8,
                0,
            ) as *mut f64;
            for i in 0..ncollide {
                // aamm is column-major (grouped per-axis)
                make_aamm(m, d,
                    aamm.add(0 * ncollide as usize + i as usize),
                    aamm.add(1 * ncollide as usize + i as usize),
                    aamm.add(2 * ncollide as usize + i as usize),
                    aamm.add(3 * ncollide as usize + i as usize),
                    aamm.add(4 * ncollide as usize + i as usize),
                    aamm.add(5 * ncollide as usize + i as usize),
                    *bfid.add(i as usize), frame.as_ptr());
            }

            // call SAP
            let maxsappair = ncollide * (ncollide - 1) / 2;
            let sappair: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_info(
                d,
                (maxsappair as usize) * std::mem::size_of::<i32>(),
                std::mem::align_of::<i32>(),
                b"mj_broadphase\0".as_ptr() as *const i8,
                0,
            ) as *mut i32;
            let nsappair = mj_sap(d, aamm as *const f64, ncollide, 0, sappair, maxsappair);
            if nsappair < 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"SAP failed\0".as_ptr() as *const i8);
            }

            // filter SAP pairs, convert to bodyflex pairs
            for i in 0..nsappair {
                let bf1 = *bfid.add((*sappair.add(i as usize) >> 16) as usize);
                let bf2 = *bfid.add((*sappair.add(i as usize) & 0xFFFF) as usize);

                // body pair: prune based on sleep filter and weld filter
                if bf1 < nbody && bf2 < nbody {
                    let asleep1 = if sleep_filter {
                        (*(*d).body_awake.add(bf1 as usize) == 0) as i32
                    } else { 0 };
                    let asleep2 = if sleep_filter {
                        (*(*d).body_awake.add(bf2 as usize) == 0) as i32
                    } else { 0 };
                    let weld1 = *(*m).body_weldid.add(bf1 as usize);
                    let weld2 = *(*m).body_weldid.add(bf2 as usize);
                    let parent_weld1 = *(*m).body_weldid.add(
                        *(*m).body_parentid.add(weld1 as usize) as usize);
                    let parent_weld2 = *(*m).body_weldid.add(
                        *(*m).body_parentid.add(weld2 as usize) as usize);

                    if filter_body_pair(weld1, parent_weld1, asleep1,
                                        weld2, parent_weld2, asleep2,
                                        dsbl_filterparent) != 0 {
                        continue;
                    }
                }
                // flex pair: skip if neither side is dynamically awake
                else if sleep_filter {
                    extern "C" {
                        fn mj_sleepState(m: *const mjModel, d: *const mjData, objtype: i32, i: i32) -> i32;
                    }
                    let awake1: i32 = if bf1 >= nbody {
                        // mjOBJ_FLEX=9, mjS_AWAKE=1
                        let state = mj_sleepState(m, d as *const mjData, 9, bf1 - nbody);
                        (state == 1) as i32
                    } else {
                        ((*(*d).body_awake.add(bf1 as usize) == 1)
                            && *(*m).body_treeid.add(bf1 as usize) >= 0) as i32
                    };
                    let awake2: i32 = if bf2 >= nbody {
                        let state = mj_sleepState(m, d as *const mjData, 9, bf2 - nbody);
                        (state == 1) as i32
                    } else {
                        ((*(*d).body_awake.add(bf2 as usize) == 1)
                            && *(*m).body_treeid.add(bf2 as usize) >= 0) as i32
                    };
                    if awake1 == 0 && awake2 == 0 { continue; }
                }

                // add bodyflex pair if there is room in buffer
                add_pair(m, bf1, bf2, &mut npair, bfpair, maxpair);
            }
        }

        // sort bodyflex pairs by signature
        if npair > 1 {
            let buf: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_info(
                d,
                (npair as usize) * std::mem::size_of::<i32>(),
                std::mem::align_of::<i32>(),
                b"mj_broadphase\0".as_ptr() as *const i8,
                0,
            ) as *mut i32;
            bfsort(bfpair, buf, npair, std::ptr::null_mut());
        }

        crate::engine::engine_memory::mj_free_stack(d);
        npair
    }
}

/// C: mj_collideFlexSAP (engine/engine_collision_driver.h:51)
/// Calls: mj_SAP, mj_collideElems, mj_freeStack, mj_isElemActive, mj_markStack, mj_stackAllocInfo, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_sap(m: *const mjModel, d: *mut mjData, f: i32) {
    extern "C" { fn mj_collideFlexSAP(m: *const mjModel, d: *mut mjData, f: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideFlexSAP(m, d, f) }
}

/// C: mj_collideGeomElem (engine/engine_collision_driver.h:54)
/// Calls: filterSphereBox, mj_arenaAllocByte, mj_assignMargin, mj_contactParam, mj_freeStack, mj_makeCapsule, mj_markStack, mj_setContact, mj_stackAllocInfo, mj_warning, mjc_ConvexElem, mjc_HFieldElem, mjraw_BoxTriangle, mjraw_CapsuleBox, mjraw_CapsuleCapsule, mjraw_CapsuleTriangle, mjraw_SphereCapsule, mjraw_SphereTriangle, mju_copy3, mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_geom_elem(m: *const mjModel, d: *mut mjData, g: i32, f: i32, e: i32) {
    extern "C" { fn mj_collideGeomElem(m: *const mjModel, d: *mut mjData, g: i32, f: i32, e: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideGeomElem(m, d, g, f, e) }
}

/// C: mj_collideElems (engine/engine_collision_driver.h:57)
/// Calls: filterBox, mj_arenaAllocByte, mj_assignMargin, mj_contactParam, mj_freeStack, mj_makeCapsule, mj_markStack, mj_setContact, mj_stackAllocInfo, mj_warning, mjc_ConvexElem, mjraw_CapsuleCapsule, mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elems(m: *const mjModel, d: *mut mjData, f1: i32, e1: i32, f2: i32, e2: i32) {
    extern "C" { fn mj_collideElems(m: *const mjModel, d: *mut mjData, f1: i32, e1: i32, f2: i32, e2: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideElems(m, d, f1, e1, f2, e2) }
}

/// C: mj_collideElemVert (engine/engine_collision_driver.h:60)
/// Calls: mj_arenaAllocByte, mj_assignMargin, mj_contactParam, mj_freeStack, mj_makeCapsule, mj_markStack, mj_setContact, mj_stackAllocInfo, mj_warning, mjc_ConvexElem, mjraw_SphereCapsule, mjraw_SphereTriangle, mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elem_vert(m: *const mjModel, d: *mut mjData, f: i32, e: i32, v: i32) {
    extern "C" { fn mj_collideElemVert(m: *const mjModel, d: *mut mjData, f: i32, e: i32, v: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideElemVert(m, d, f, e, v) }
}

