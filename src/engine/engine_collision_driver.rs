//! Port of: engine/engine_collision_driver.c
//! IR hash: 8cbd078414266fa8
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
pub fn get_margin(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64 {
    // SAFETY: m is a valid pointer to mjModel; g1, g2, ipair are valid indices (caller contract)
    unsafe {
        if ipair >= 0 {
            return crate::engine::engine_core_constraint::mj_assign_margin(
                m, *(*m).pair_margin.add(ipair as usize));
        }
        crate::engine::engine_core_constraint::mj_assign_margin(
            m, *(*m).geom_margin.add(g1 as usize) + *(*m).geom_margin.add(g2 as usize))
    }
}

/// C: getGap (engine/engine_collision_driver.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_gap(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64 {
    // SAFETY: m is a valid pointer to mjModel; g1, g2, ipair are valid indices (caller contract)
    unsafe {
        if ipair >= 0 {
            return *(*m).pair_gap.add(ipair as usize);
        }
        *(*m).geom_gap.add(g1 as usize) + *(*m).geom_gap.add(g2 as usize)
    }
}

/// C: resetArena (engine/engine_collision_driver.c:179)
#[allow(unused_variables, non_snake_case)]
pub fn reset_arena(d: *mut mjData) {
    // SAFETY: d is a valid pointer to mjData (caller contract)
    unsafe {
        (*d).parena = (*d).ncon as usize * std::mem::size_of::<mjContact>();
    }
}

/// C: alignArena (engine/engine_collision_driver.c:189)
#[allow(unused_variables, non_snake_case)]
pub fn align_arena(d: *mut mjData, alignment: usize) -> usize {
    // SAFETY: caller guarantees d is a valid pointer to mjData
    unsafe {
        let misalignment = (*d).parena % alignment;
        if misalignment != 0 {
            (*d).parena += alignment - misalignment;
        }
        (*d).parena
    }
}

/// C: planeGeomDist (engine/engine_collision_driver.c:199)
/// Calls: mju_dot3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_geom_dist(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32) -> f64 {
    // SAFETY: m, d are valid pointers; g1, g2 are valid geom indices (caller contract)
    unsafe {
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let norm: [f64; 3] = [*mat1.add(2), *mat1.add(5), *mat1.add(8)];
        let mut dif: [f64; 3] = [0.0; 3];

        crate::engine::engine_util_blas::mju_sub3(
            dif.as_mut_ptr(),
            (*d).geom_xpos.add(3 * g2 as usize),
            (*d).geom_xpos.add(3 * g1 as usize),
        );
        crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), norm.as_ptr())
    }
}

/// C: hasPlane (engine/engine_collision_driver.c:210)
#[allow(unused_variables, non_snake_case)]
pub fn has_plane(m: *const mjModel, body: i32) -> i32 {
    // SAFETY: m is a valid pointer to mjModel; body is a valid body index (caller contract)
    unsafe {
        const mjGEOM_PLANE: i32 = 0;
        let start = *(*m).body_geomadr.add(body as usize);
        let end = start + *(*m).body_geomnum.add(body as usize);

        let mut g = start;
        while g < end {
            if *(*m).geom_type.add(g as usize) == mjGEOM_PLANE {
                return 1;
            }
            g += 1;
        }

        0
    }
}

/// C: filterBitmask (engine/engine_collision_driver.c:227)
#[allow(unused_variables, non_snake_case)]
pub fn filter_bitmask(contype1: i32, conaffinity1: i32, contype2: i32, conaffinity2: i32) -> i32 {
    if (contype1 & conaffinity2) == 0 && (contype2 & conaffinity1) == 0 {
        1
    } else {
        0
    }
}

/// C: filterBox (engine/engine_collision_driver.c:234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_box(aabb1: *const f64, aabb2: *const f64, margin: f64) -> i32 {
    // SAFETY: aabb1 and aabb2 point to arrays of 6 f64 from caller contract
    unsafe {
        if *aabb1.add(0) + *aabb1.add(3) + margin < *aabb2.add(0) - *aabb2.add(3) { return 1; }
        if *aabb1.add(1) + *aabb1.add(4) + margin < *aabb2.add(1) - *aabb2.add(4) { return 1; }
        if *aabb1.add(2) + *aabb1.add(5) + margin < *aabb2.add(2) - *aabb2.add(5) { return 1; }
        if *aabb2.add(0) + *aabb2.add(3) + margin < *aabb1.add(0) - *aabb1.add(3) { return 1; }
        if *aabb2.add(1) + *aabb2.add(4) + margin < *aabb1.add(1) - *aabb1.add(4) { return 1; }
        if *aabb2.add(2) + *aabb2.add(5) + margin < *aabb1.add(2) - *aabb1.add(5) { return 1; }
    }
    0
}

/// C: filterSphereBox (engine/engine_collision_driver.c:246)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_sphere_box(s: *const f64, bound: f64, aabb: *const f64) -> i32 {
    // SAFETY: s points to array of 3 f64, aabb points to array of 6 f64 from caller contract
    unsafe {
        if *s.add(0) + bound < *aabb.add(0) - *aabb.add(3) { return 1; }
        if *s.add(1) + bound < *aabb.add(1) - *aabb.add(4) { return 1; }
        if *s.add(2) + bound < *aabb.add(2) - *aabb.add(5) { return 1; }
        if *s.add(0) - bound > *aabb.add(0) + *aabb.add(3) { return 1; }
        if *s.add(1) - bound > *aabb.add(1) + *aabb.add(4) { return 1; }
        if *s.add(2) - bound > *aabb.add(2) + *aabb.add(5) { return 1; }
    }
    0
}

/// C: filterSphere (engine/engine_collision_driver.c:258)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn filter_sphere(pos1: *const f64, pos2: *const f64, bound: f64) -> i32 {
    // SAFETY: pos1 and pos2 point to arrays of 3 f64 from caller contract
    unsafe {
        let dif0 = *pos1.add(0) - *pos2.add(0);
        let dif1 = *pos1.add(1) - *pos2.add(1);
        let dif2 = *pos1.add(2) - *pos2.add(2);
        let distsqr = dif0 * dif0 + dif1 * dif1 + dif2 * dif2;
        if distsqr > bound * bound { 1 } else { 0 }
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
pub fn mj_filter_sphere(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: caller guarantees m, d are valid; g1, g2 are valid geom indices
    unsafe {
        const mjGEOM_PLANE: i32 = 0;

        // neither geom is a plane
        if *(*m).geom_rbound.add(g1 as usize) > 0.0
            && *(*m).geom_rbound.add(g2 as usize) > 0.0
        {
            return filter_sphere(
                (*d).geom_xpos.add(3 * g1 as usize),
                (*d).geom_xpos.add(3 * g2 as usize),
                *(*m).geom_rbound.add(g1 as usize) + *(*m).geom_rbound.add(g2 as usize) + margin,
            );
        }

        // one geom is a plane
        if *(*m).geom_type.add(g1 as usize) == mjGEOM_PLANE
            && *(*m).geom_rbound.add(g2 as usize) > 0.0
            && plane_geom_dist(m, d, g1, g2) > margin + *(*m).geom_rbound.add(g2 as usize)
        {
            return 1;
        }
        if *(*m).geom_type.add(g2 as usize) == mjGEOM_PLANE
            && *(*m).geom_rbound.add(g1 as usize) > 0.0
            && plane_geom_dist(m, d, g2, g1) > margin + *(*m).geom_rbound.add(g1 as usize)
        {
            return 1;
        }
        0
    }
}

/// C: filterBodyPair (engine/engine_collision_driver.c:288)
#[allow(unused_variables, non_snake_case)]
pub fn filter_body_pair(weldbody1: i32, weldparent1: i32, asleep1: i32, weldbody2: i32, weldparent2: i32, asleep2: i32, dsbl_filterparent: i32) -> i32 {
    if weldbody1 == weldbody2 { return 1; }
    if asleep1 != 0 && asleep2 != 0 { return 1; }
    if (asleep1 != 0 && weldbody2 == 0) || (asleep2 != 0 && weldbody1 == 0) { return 1; }
    if (dsbl_filterparent == 0 && weldbody1 != 0 && weldbody2 != 0)
        && (weldbody1 == weldparent2 || weldbody2 == weldparent1) { return 1; }
    0
}

/// C: canCollide (engine/engine_collision_driver.c:318)
#[allow(unused_variables, non_snake_case)]
pub fn can_collide(m: *const mjModel, bf: i32) -> i32 {
    // SAFETY: caller guarantees m is a valid pointer to mjModel with valid array fields
    unsafe {
        if (bf as usize) < (*m).nbody {
            if *(*m).body_contype.add(bf as usize) != 0 || *(*m).body_conaffinity.add(bf as usize) != 0 {
                1
            } else {
                0
            }
        } else {
            let f = bf as usize - (*m).nbody;
            if *(*m).flex_contype.add(f) != 0 || *(*m).flex_conaffinity.add(f) != 0 {
                1
            } else {
                0
            }
        }
    }
}

/// C: canCollide2 (engine/engine_collision_driver.c:329)
/// Calls: filterBitmask
#[allow(unused_variables, non_snake_case)]
pub fn can_collide2(m: *const mjModel, bf1: i32, bf2: i32) -> i32 {
    // SAFETY: caller guarantees m is a valid pointer to mjModel with valid array fields
    unsafe {
        let nbody = (*m).nbody;
        let contype1: i32;
        let conaffinity1: i32;
        let contype2: i32;
        let conaffinity2: i32;

        if (bf1 as usize) < nbody {
            contype1 = *(*m).body_contype.add(bf1 as usize);
            conaffinity1 = *(*m).body_conaffinity.add(bf1 as usize);
        } else {
            let f = bf1 as usize - nbody;
            contype1 = *(*m).flex_contype.add(f);
            conaffinity1 = *(*m).flex_conaffinity.add(f);
        }

        if (bf2 as usize) < nbody {
            contype2 = *(*m).body_contype.add(bf2 as usize);
            conaffinity2 = *(*m).body_conaffinity.add(bf2 as usize);
        } else {
            let f = bf2 as usize - nbody;
            contype2 = *(*m).flex_contype.add(f);
            conaffinity2 = *(*m).flex_conaffinity.add(f);
        }

        if (contype1 & conaffinity2) != 0 || (contype2 & conaffinity1) != 0 {
            1
        } else {
            0
        }
    }
}

/// C: mj_collideTree (engine/engine_collision_driver.c:361)
/// Calls: canCollide2, filterBitmask, filterBox, filterCollisionPair, filterSphereBox, mj_assignMargin, mj_collideElems, mj_collideGeomElem, mj_collideOBB, mj_collidePlaneFlex, mj_collideSdfFlex, mj_filterSphere, mj_freeStack, mj_markStack, mj_narrowphase, mj_stackAllocInfo, mju_error, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_tree(m: *const mjModel, d: *mut mjData, bf1: i32, bf2: i32, merged: i32, startadr: i32, pairadr: i32) {
    todo!() // mj_collideTree
}

/// C: mj_narrowphase (engine/engine_collision_driver.c:367)
/// Calls: getGap, getMargin, mj_arenaAllocByte, mj_contactParam, mj_freeStack, mj_markStack, mj_maxContact, mj_setContact, mj_stackAllocByte, mj_stackAllocInfo, mj_stackAllocInt, mj_warning, mjc_ccdSize, mji_copy3, mju_copy, mju_dispatch, mju_numThread
#[allow(unused_variables, non_snake_case)]
pub fn mj_narrowphase(m: *const mjModel, d: *mut mjData, buffer: *const mjcPair, npair: i32, parena: usize) {
    todo!() // mj_narrowphase
}

/// C: mj_collidePlaneFlex (engine/engine_collision_driver.c:371)
/// Calls: mj_addContact, mj_assignMargin, mj_contactParam, mj_setContact, mju_addScl3, mju_copy3, mju_dot3, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_plane_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    todo!() // mj_collidePlaneFlex
}

/// C: mj_collideSdfFlex (engine/engine_collision_driver.c:374)
/// Calls: mj_addContact, mj_assignMargin, mj_contactParam, mj_freeStack, mj_markStack, mj_setContact, mj_stackAllocInfo, mjc_FlexSDF, mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_sdf_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    todo!() // mj_collideSdfFlex
}

/// C: mj_collideFlexInternal (engine/engine_collision_driver.c:377)
/// Calls: mj_addContact, mj_collideElemVert, mj_contactParam, mj_setContact, mju_copy3, planeVertex
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_internal(m: *const mjModel, d: *mut mjData, f: i32) {
    todo!() // mj_collideFlexInternal
}

/// C: contactcompare (engine/engine_collision_driver.c:380)
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

/// C: contactSort (engine/engine_collision_driver.c:413)
/// Calls: contactcompare
#[allow(unused_variables, non_snake_case)]
pub fn contact_sort(arr: *mut mjContact, buf: *mut mjContact, n: i32, context: *mut ()) {
    todo!() // contactSort
}

/// C: filterFlexContacts (engine/engine_collision_driver.c:417)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, resetArena
#[allow(unused_variables, non_snake_case)]
pub fn filter_flex_contacts(d: *mut mjData, ncon_before: i32) {
    todo!() // filterFlexContacts
}

/// C: pushPairArena (engine/engine_collision_driver.c:489)
/// Calls: mj_arenaAllocByte, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn push_pair_arena(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32) {
    todo!() // pushPairArena
}

/// C: filterCollisionPair (engine/engine_collision_driver.c:508)
/// Calls: filterBitmask, getGap, getMargin, mj_filterSphere
#[allow(unused_variables, non_snake_case)]
pub fn filter_collision_pair(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32, merged: i32, startadr: i32, pairadr: i32) -> i32 {
    const mjENBL_SLEEP: i32 = 1 << 4;
    const mjS_AWAKE: i32 = 1;

    // SAFETY: m, d are valid pointers; g1, g2 are valid geom indices; all array
    // accesses are within bounds guaranteed by caller contract.
    unsafe {
        // merged, find matching pair
        if merged != 0 {
            let mut k = startadr;
            while k < pairadr {
                if (*(*m).pair_geom1.add(k as usize) == g1 && *(*m).pair_geom2.add(k as usize) == g2)
                    || (*(*m).pair_geom1.add(k as usize) == g2 && *(*m).pair_geom2.add(k as usize) == g1)
                {
                    return 0;
                }
                k += 1;
            }
        }

        if ipair >= 0 {
            if ((*m).opt.enableflags & mjENBL_SLEEP) != 0 {
                let b1 = *(*m).geom_bodyid.add(g1 as usize);
                let b2 = *(*m).geom_bodyid.add(g2 as usize);
                if *(*d).body_awake.add(b1 as usize) != mjS_AWAKE
                    && *(*d).body_awake.add(b2 as usize) != mjS_AWAKE
                {
                    return 0;
                }
            }
        }

        if ipair < 0 {
            extern "C" {
                static mut mjcb_contactfilter: Option<unsafe extern "C" fn(*const mjModel, *mut mjData, i32, i32) -> i32>;
            }
            if let Some(filter_fn) = mjcb_contactfilter {
                if filter_fn(m, d, g1, g2) != 0 {
                    return 0;
                }
            } else if filter_bitmask(
                *(*m).geom_contype.add(g1 as usize),
                *(*m).geom_conaffinity.add(g1 as usize),
                *(*m).geom_contype.add(g2 as usize),
                *(*m).geom_conaffinity.add(g2 as usize),
            ) != 0
            {
                return 0;
            }
        }

        // bounding sphere filter
        let margin = get_margin(m, g1, g2, ipair);
        let gap = get_gap(m, g1, g2, ipair);
        if mj_filter_sphere(m, d, g1, g2, margin + gap) != 0 {
            return 0;
        }

        // check collision function is well-defined
        let t1 = *(*m).geom_type.add(g1 as usize);
        let t2 = *(*m).geom_type.add(g2 as usize);
        let type1 = if t1 < t2 { t1 } else { t2 };
        let type2 = if t1 > t2 { t1 } else { t2 };

        // check if mjCOLLISIONFUNC[type1][type2] != NULL
        let guard = crate::types::MJCOLLISIONFUNC.lock().unwrap();
        let idx = (type1 as usize) * 9 + (type2 as usize);
        let ptr_bytes: [u8; 8] = [
            guard[idx * 8],
            guard[idx * 8 + 1],
            guard[idx * 8 + 2],
            guard[idx * 8 + 3],
            guard[idx * 8 + 4],
            guard[idx * 8 + 5],
            guard[idx * 8 + 6],
            guard[idx * 8 + 7],
        ];
        let fptr = usize::from_ne_bytes(ptr_bytes);
        if fptr != 0 { 1 } else { 0 }
    }
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
    todo!() // makeAAMM
}

/// C: add_pair (engine/engine_collision_driver.c:1315)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn add_pair(m: *const mjModel, bf1: i32, bf2: i32, npair: *mut i32, pair: *mut i32, maxpair: i32) {
    // SAFETY: m may be null (skip filtering); npair/pair valid arrays (caller contract)
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
                    for i in body_geomadr1..(body_geomadr1 + body_geomnum1) {
                        ct1 |= *(*m).geom_contype.add(i as usize);
                        ca1 |= *(*m).geom_conaffinity.add(i as usize);
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
                    for i in body_geomadr2..(body_geomadr2 + body_geomnum2) {
                        ct2 |= *(*m).geom_contype.add(i as usize);
                        ca2 |= *(*m).geom_conaffinity.add(i as usize);
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
            extern "C" {
                fn mju_error(msg: *const i8);
            }
            mju_error(b"broadphase buffer full\0".as_ptr() as *const i8);
        }
    }
}

/// C: SAPcmp (engine/engine_collision_driver.c:1383)
#[allow(unused_variables, non_snake_case)]
pub fn sa_pcmp(obj1: *mut mjtSAP, obj2: *mut mjtSAP, context: *mut ()) -> i32 {
    // SAFETY: obj1, obj2 are valid mjtSAP pointers (caller contract)
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
    todo!() // SAPsort
}

/// C: mj_SAP (engine/engine_collision_driver.c:1400)
/// Calls: SAPsort, mj_stackAllocInfo
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_sap(d: *mut mjData, aamm: *const f64, n: i32, axis_x: i32, pair: *mut i32, maxpair: i32) -> i32 {
    todo!() // mj_SAP
}

/// C: updateCov (engine/engine_collision_driver.c:1497)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_cov(cov: *mut f64, vec: *const f64, cen: *const f64) {
    // SAFETY: cov points to 9 f64 (3x3 matrix), vec and cen point to 3 f64 each (caller contract)
    unsafe {
        let dif: [f64; 3] = [
            *vec.add(0) - *cen.add(0),
            *vec.add(1) - *cen.add(1),
            *vec.add(2) - *cen.add(2),
        ];
        let D00 = dif[0] * dif[0];
        let D01 = dif[0] * dif[1];
        let D02 = dif[0] * dif[2];
        let D11 = dif[1] * dif[1];
        let D12 = dif[1] * dif[2];
        let D22 = dif[2] * dif[2];
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
    // SAFETY: i, j are valid pointers to i32 values (caller contract)
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
    todo!() // bfsort
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
    todo!() // mj_contactParam
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
    todo!() // mj_setContact
}

/// C: mj_makeCapsule (engine/engine_collision_driver.c:1816)
/// Calls: mju_add3, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_capsule(m: *const mjModel, d: *mut mjData, f: i32, vid: *const i32, pos: *mut f64, mat: *mut f64, size: *mut f64) {
    todo!() // mj_makeCapsule
}

/// C: collisionTask (engine/engine_collision_driver.c:1849)
/// Calls: getGap, getMargin, mjc_setCCDBuffer, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn collision_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, idx: i32) {
    todo!() // collisionTask
}

/// C: planeVertex (engine/engine_collision_driver.c:2129)
/// Calls: mju_addScl3, mju_cross, mju_dot3, mju_normalize3, mju_scl3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_vertex(con: *mut mjPreContact, pos: *const f64, rad: f64, t0: i32, t1: i32, t2: i32, v: i32) -> i32 {
    // SAFETY: con is valid mjPreContact pointer; pos points to vertex array (3 f64 per vertex);
    // t0, t1, t2, v are valid vertex indices (caller contract)
    unsafe {
        // make t0 the origin
        let mut e1: [f64; 3] = [0.0; 3];
        let mut e2: [f64; 3] = [0.0; 3];
        let mut ev: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_sub3(e1.as_mut_ptr(), pos.add(3 * t1 as usize), pos.add(3 * t0 as usize));
        crate::engine::engine_util_blas::mju_sub3(e2.as_mut_ptr(), pos.add(3 * t2 as usize), pos.add(3 * t0 as usize));
        crate::engine::engine_util_blas::mju_sub3(ev.as_mut_ptr(), pos.add(3 * v as usize), pos.add(3 * t0 as usize));

        // compute normal
        let mut nrm: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_cross(nrm.as_mut_ptr(), e1.as_ptr(), e2.as_ptr());
        crate::engine::engine_util_blas::mju_normalize3(nrm.as_mut_ptr());

        // project, check distance
        let dst = crate::engine::engine_util_blas::mju_dot3(ev.as_ptr(), nrm.as_ptr());
        if dst <= -2.0 * rad {
            return 0;
        }

        // construct contact
        (*con).dist = -dst - 2.0 * rad;
        crate::engine::engine_util_blas::mju_scl3((*con).normal.as_mut_ptr(), nrm.as_ptr(), -1.0);
        crate::engine::engine_util_blas::mju_zero3((*con).tangent.as_mut_ptr());
        crate::engine::engine_util_blas::mju_add_scl3((*con).pos.as_mut_ptr(), pos.add(3 * v as usize), nrm.as_ptr(), -0.5 * dst);
        1
    }
}

/// C: mj_maxContact (engine/engine_collision_driver.h:33)
#[allow(unused_variables, non_snake_case)]
pub fn mj_max_contact(m: *const mjModel, g1: i32, g2: i32, mut has_margin: i32) -> i32 {
    const mjGEOM_PLANE: i32 = 0;
    const mjGEOM_HFIELD: i32 = 1;
    const mjGEOM_SPHERE: i32 = 2;
    const mjGEOM_CAPSULE: i32 = 3;
    const mjGEOM_ELLIPSOID: i32 = 4;
    const mjGEOM_CYLINDER: i32 = 5;
    const mjGEOM_BOX: i32 = 6;
    const mjGEOM_MESH: i32 = 7;
    const mjGEOM_SDF: i32 = 8;
    const mjMAXCONPAIR: i32 = 50;
    const mjDSBL_MULTICCD: i32 = 1 << 19;
    const mjDSBL_NATIVECCD: i32 = 1 << 17;
    const mjENBL_OVERRIDE: i32 = 1 << 0;

    // SAFETY: m is a valid pointer to mjModel; g1, g2 are valid geom indices (caller contract)
    unsafe {
        let type1 = *(*m).geom_type.add(g1 as usize);
        let type2 = *(*m).geom_type.add(g2 as usize);

        if type1 == mjGEOM_SDF || type2 == mjGEOM_SDF {
            return (*m).opt.sdf_initpoints;
        }

        if type1 == mjGEOM_HFIELD || type2 == mjGEOM_HFIELD {
            let typ = if type1 == mjGEOM_HFIELD { type2 } else { type1 };
            return if typ != mjGEOM_PLANE && typ != mjGEOM_HFIELD { mjMAXCONPAIR } else { 0 };
        }

        // spheres and ellipsoids always generate a single contact
        if type1 == mjGEOM_SPHERE || type1 == mjGEOM_ELLIPSOID
            || type2 == mjGEOM_SPHERE || type2 == mjGEOM_ELLIPSOID
        {
            return 1;
        }

        // box-box primitive collider
        if type1 == mjGEOM_BOX && type2 == mjGEOM_BOX {
            return 8;
        }

        // capsule-capsule primitive collider
        if type1 == mjGEOM_CAPSULE && type2 == mjGEOM_CAPSULE {
            return 2;
        }

        // capsule-box primitive collider
        if (type1 == mjGEOM_CAPSULE && type2 == mjGEOM_BOX)
            || (type1 == mjGEOM_BOX && type2 == mjGEOM_CAPSULE)
        {
            return 4;
        }

        // the remaining plane cases
        if type1 == mjGEOM_PLANE || type2 == mjGEOM_PLANE {
            let typ = if type1 == mjGEOM_PLANE { type2 } else { type1 };
            return match typ {
                mjGEOM_CAPSULE => 2,
                mjGEOM_CYLINDER | mjGEOM_BOX => 4,
                mjGEOM_MESH => 3,
                _ => 0,
            };
        }

        let is_multiccd = ((*m).opt.disableflags & mjDSBL_MULTICCD) == 0;
        if !is_multiccd {
            return 1;
        }

        if type1 == mjGEOM_CAPSULE || type2 == mjGEOM_CAPSULE
            || type1 == mjGEOM_CYLINDER || type2 == mjGEOM_CYLINDER
        {
            return 5;
        }

        if ((*m).opt.disableflags & mjDSBL_NATIVECCD) != 0 {
            return if is_multiccd { 5 } else { 1 };  // mesh-mesh or mesh-box with libccd
        }

        // check margin from model
        if has_margin < 0 {
            has_margin = 0;
            if ((*m).opt.enableflags & mjENBL_OVERRIDE) != 0 {
                has_margin = ((*m).opt.o_margin > 0.0) as i32;
            } else {
                let npair = (*m).npair as i32;
                let mut ipair: i32 = -1;
                for k in 0..npair {
                    if (*(*m).pair_geom1.add(k as usize) == g1
                        && *(*m).pair_geom2.add(k as usize) == g2)
                        || (*(*m).pair_geom1.add(k as usize) == g2
                            && *(*m).pair_geom2.add(k as usize) == g1)
                    {
                        ipair = k;
                        break;
                    }
                }

                if ipair > -1 {
                    has_margin = (*(*m).pair_margin.add(ipair as usize) > 0.0) as i32;
                } else {
                    has_margin = (*(*m).geom_margin.add(g1 as usize) > 0.0
                        || *(*m).geom_margin.add(g2 as usize) > 0.0) as i32;
                }
            }
        }

        // 4 contacts for mesh-mesh or mesh-box without margins, 5 with margins
        if has_margin != 0 { 5 } else { 4 }
    }
}

/// C: mj_collision (engine/engine_collision_driver.h:36)
/// Calls: alignArena, canCollide2, contactSort, filterBitmask, filterCollisionPair, filterFlexContacts, mj_broadphase, mj_clearEfc, mj_collideElems, mj_collideFlexInternal, mj_collideFlexSAP, mj_collideGeomElem, mj_collidePlaneFlex, mj_collideSdfFlex, mj_collideTree, mj_freeStack, mj_isElemActive, mj_markStack, mj_narrowphase, mj_sleepState, mj_stackAllocInfo, pushPairArena, resetArena
#[allow(unused_variables, non_snake_case)]
pub fn mj_collision(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_collision
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
    const MJ_MAXVAL: f64 = 1E10;

    // SAFETY: all pointers are valid arrays of sufficient size (caller contract)
    unsafe {
        // get infinite dimensions (planes only)
        let inf1: [u8; 3] = [
            (*aabb1.add(3) >= MJ_MAXVAL) as u8,
            (*aabb1.add(4) >= MJ_MAXVAL) as u8,
            (*aabb1.add(5) >= MJ_MAXVAL) as u8,
        ];
        let inf2: [u8; 3] = [
            (*aabb2.add(3) >= MJ_MAXVAL) as u8,
            (*aabb2.add(4) >= MJ_MAXVAL) as u8,
            (*aabb2.add(5) >= MJ_MAXVAL) as u8,
        ];

        // if a bounding box is infinite, there must be a collision
        if (inf1[0] != 0 && inf1[1] != 0 && inf1[2] != 0)
            || (inf2[0] != 0 && inf2[1] != 0 && inf2[2] != 0)
        {
            return 1;
        }

        let aabb: [*const f64; 2] = [aabb1, aabb2];
        let xmat: [*const f64; 2] = [xmat1, xmat2];
        let xpos: [*const f64; 2] = [xpos1, xpos2];
        let mut xcenter: [[f64; 3]; 2] = [[0.0; 3]; 2];
        let mut normal: [[[f64; 3]; 3]; 2] = [[[0.0; 3]; 3]; 2];
        let mut proj: [f64; 2] = [0.0; 2];
        let mut radius: [f64; 2] = [0.0; 2];
        let infinite: [u8; 2] = [
            (inf1[0] != 0 || inf1[1] != 0 || inf1[2] != 0) as u8,
            (inf2[0] != 0 || inf2[1] != 0 || inf2[2] != 0) as u8,
        ];

        // compute centers in local coordinates
        if product.is_null() {
            for i in 0..2_usize {
                if !xmat[i].is_null() {
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(
                        xcenter[i].as_mut_ptr(),
                        xmat[i],
                        aabb[i],
                    );
                } else {
                    crate::engine::engine_util_blas::mju_copy3(xcenter[i].as_mut_ptr(), aabb[i]);
                }

                if !xpos[i].is_null() {
                    crate::engine::engine_util_blas::mju_add_to3(xcenter[i].as_mut_ptr(), xpos[i]);
                }
            }
        }

        // compute normals in global coordinates
        for i in 0..2_usize {
            for j in 0..3_usize {
                for k in 0..3_usize {
                    if !xmat[i].is_null() {
                        normal[i][j][k] = *xmat[i].add(3 * k + j);
                    } else {
                        normal[i][j][k] = if j == k { 1.0 } else { 0.0 };
                    }
                }
            }
        }

        // precompute dot products
        if !product.is_null() && !offset.is_null() && (*initialize)._data[0] != 0 {
            for i in 0..2_usize {
                for j in 0..2_usize {
                    for k in 0..3_usize {
                        for l in 0..3_usize {
                            *product.add(18 * i + 9 * j + 3 * k + l) =
                                crate::engine::engine_util_blas::mju_dot3(
                                    normal[i][l].as_ptr(),
                                    normal[j][k].as_ptr(),
                                );
                        }
                        *offset.add(6 * i + 3 * j + k) = if !xpos[i].is_null() {
                            crate::engine::engine_util_blas::mju_dot3(xpos[i], normal[j][k].as_ptr())
                        } else {
                            0.0
                        };
                    }
                }
            }
            *initialize = mjtBool { _data: [0] };
        }

        // check intersections
        for j in 0..2_usize {
            if infinite[1 - j] != 0 {
                continue; // skip test against an infinite body
            }
            for k in 0..3_usize {
                for i in 0..2_usize {
                    if product.is_null() {
                        proj[i] = crate::engine::engine_util_blas::mju_dot3(
                            xcenter[i].as_ptr(),
                            normal[j][k].as_ptr(),
                        );
                        radius[i] =
                            (*aabb[i].add(3) * crate::engine::engine_util_blas::mju_dot3(
                                normal[i][0].as_ptr(), normal[j][k].as_ptr())).abs()
                            + (*aabb[i].add(4) * crate::engine::engine_util_blas::mju_dot3(
                                normal[i][1].as_ptr(), normal[j][k].as_ptr())).abs()
                            + (*aabb[i].add(5) * crate::engine::engine_util_blas::mju_dot3(
                                normal[i][2].as_ptr(), normal[j][k].as_ptr())).abs();
                    } else {
                        let adr: usize = 18 * i + 9 * j + 3 * k;
                        proj[i] = *aabb[i].add(0) * *product.add(adr + 0)
                            + *aabb[i].add(1) * *product.add(adr + 1)
                            + *aabb[i].add(2) * *product.add(adr + 2)
                            + *offset.add(6 * i + 3 * j + k);
                        radius[i] = (*aabb[i].add(3) * *product.add(adr + 0)).abs()
                            + (*aabb[i].add(4) * *product.add(adr + 1)).abs()
                            + (*aabb[i].add(5) * *product.add(adr + 2)).abs();
                    }
                }

                if radius[0] + radius[1] + margin < (proj[1] - proj[0]).abs() {
                    return 0;
                }
            }
        }

        1
    }
}

/// C: mj_isElemActive (engine/engine_collision_driver.h:45)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_elem_active(m: *const mjModel, f: i32, e: i32) -> i32 {
    // SAFETY: m is a valid mjModel pointer; f is a valid flex index, e is a valid elem index
    unsafe {
        if *(*m).flex_dim.add(f as usize) < 3 {
            1
        } else {
            let idx = *(*m).flex_elemadr.add(f as usize) + e;
            if *(*m).flex_elemlayer.add(idx as usize) < *(*m).flex_activelayers.add(f as usize) {
                1
            } else {
                0
            }
        }
    }
}

/// C: mj_broadphase (engine/engine_collision_driver.h:48)
/// Calls: add_pair, bfsort, canCollide, filterBodyPair, hasPlane, makeAAMM, mj_SAP, mj_freeStack, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_addTo3, mju_eig3, mju_message, mju_scl, mju_scl3, mju_zero, mju_zero3, updateCov
#[allow(unused_variables, non_snake_case)]
pub fn mj_broadphase(m: *const mjModel, d: *mut mjData, bfpair: *mut i32, maxpair: i32) -> i32 {
    todo!() // mj_broadphase
}

/// C: mj_collideFlexSAP (engine/engine_collision_driver.h:51)
/// Calls: mj_SAP, mj_collideElems, mj_freeStack, mj_isElemActive, mj_markStack, mj_stackAllocInfo, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_sap(m: *const mjModel, d: *mut mjData, f: i32) {
    todo!() // mj_collideFlexSAP
}

/// C: mj_collideGeomElem (engine/engine_collision_driver.h:54)
/// Calls: filterSphereBox, mj_arenaAllocByte, mj_assignMargin, mj_contactParam, mj_freeStack, mj_makeCapsule, mj_markStack, mj_setContact, mj_stackAllocInfo, mj_warning, mjc_ConvexElem, mjc_HFieldElem, mjraw_BoxTriangle, mjraw_CapsuleBox, mjraw_CapsuleCapsule, mjraw_CapsuleTriangle, mjraw_SphereCapsule, mjraw_SphereTriangle, mju_copy3, mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_geom_elem(m: *const mjModel, d: *mut mjData, g: i32, f: i32, e: i32) {
    todo!() // mj_collideGeomElem
}

/// C: mj_collideElems (engine/engine_collision_driver.h:57)
/// Calls: filterBox, mj_arenaAllocByte, mj_assignMargin, mj_contactParam, mj_freeStack, mj_makeCapsule, mj_markStack, mj_setContact, mj_stackAllocInfo, mj_warning, mjc_ConvexElem, mjraw_CapsuleCapsule, mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elems(m: *const mjModel, d: *mut mjData, f1: i32, e1: i32, f2: i32, e2: i32) {
    todo!() // mj_collideElems
}

/// C: mj_collideElemVert (engine/engine_collision_driver.h:60)
/// Calls: mj_arenaAllocByte, mj_assignMargin, mj_contactParam, mj_freeStack, mj_makeCapsule, mj_markStack, mj_setContact, mj_stackAllocInfo, mj_warning, mjc_ConvexElem, mjraw_SphereCapsule, mjraw_SphereTriangle, mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elem_vert(m: *const mjModel, d: *mut mjData, f: i32, e: i32, v: i32) {
    todo!() // mj_collideElemVert
}

