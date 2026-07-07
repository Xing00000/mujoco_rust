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
pub fn get_margin(m: *const mjModel, g1: i32, g2: i32, ipair: i32) -> f64 {
    // SAFETY: m is a valid pointer to mjModel, g1/g2/ipair are valid indices
    unsafe {
        if ipair >= 0 {
            crate::engine::engine_core_constraint::mj_assign_margin(m, *(*m).pair_margin.add(ipair as usize))
        } else {
            crate::engine::engine_core_constraint::mj_assign_margin(m, *(*m).geom_margin.add(g1 as usize) + *(*m).geom_margin.add(g2 as usize))
        }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, g1 : i32, g2 : i32, ipair : i32)
    // Previous return: f64
    extern "C" { fn getGap_impl (m : * const mjModel , g1 : i32 , g2 : i32 , ipair : i32) -> f64 ; } unsafe { getGap_impl (m , g1 , g2 , ipair) }
}

/// C: resetArena (engine/engine_collision_driver.c:179)
#[allow(unused_variables, non_snake_case)]
pub fn reset_arena(d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData)
    // Previous return: ()
    extern "C" { fn resetArena_impl (d : * mut mjData) ; } unsafe { resetArena_impl (d) }
}

/// C: alignArena (engine/engine_collision_driver.c:189)
#[allow(unused_variables, non_snake_case)]
pub fn align_arena(d: *mut mjData, alignment: usize) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, alignment : usize)
    // Previous return: usize
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, g1 : i32, g2 : i32)
    // Previous return: f64
    extern "C" { fn planeGeomDist_impl (m : * const mjModel , d : * mut mjData , g1 : i32 , g2 : i32) -> f64 ; } unsafe { planeGeomDist_impl (m , d , g1 , g2) }
}

/// C: hasPlane (engine/engine_collision_driver.c:210)
#[allow(unused_variables, non_snake_case)]
pub fn has_plane(m: *const mjModel, body: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, body : i32)
    // Previous return: i32
    extern "C" { fn hasPlane_impl (m : * const mjModel , body : i32) -> i32 ; } unsafe { hasPlane_impl (m , body) }
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
    // WARNING: signature changed — verify body
    // Previous params: (aabb1 : * const f64, aabb2 : * const f64, margin : f64)
    // Previous return: i32
    extern "C" { fn filterBox_impl (aabb1 : * const f64 , aabb2 : * const f64 , margin : f64) -> i32 ; } unsafe { filterBox_impl (aabb1 , aabb2 , margin) }
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
    // WARNING: signature changed — verify body
    // Previous params: (pos1 : * const f64, pos2 : * const f64, bound : f64)
    // Previous return: i32
    extern "C" { fn filterSphere_impl (pos1 : * const f64 , pos2 : * const f64 , bound : f64) -> i32 ; } unsafe { filterSphere_impl (pos1 , pos2 , bound) }
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
    // SAFETY: m, d are valid pointers; g1, g2 are valid geom indices
    unsafe {
        // neither geom is a plane
        if *(*m).geom_rbound.add(g1 as usize) > 0.0 && *(*m).geom_rbound.add(g2 as usize) > 0.0 {
            return filter_sphere(
                (*d).geom_xpos.add(3 * g1 as usize),
                (*d).geom_xpos.add(3 * g2 as usize),
                *(*m).geom_rbound.add(g1 as usize) + *(*m).geom_rbound.add(g2 as usize) + margin,
            );
        }

        // one geom is a plane
        if *(*m).geom_type.add(g1 as usize) == 0  // mjGEOM_PLANE = 0
            && *(*m).geom_rbound.add(g2 as usize) > 0.0
            && plane_geom_dist(m, d, g1, g2) > margin + *(*m).geom_rbound.add(g2 as usize)
        {
            return 1;
        }
        if *(*m).geom_type.add(g2 as usize) == 0  // mjGEOM_PLANE = 0
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
    // WARNING: signature changed — verify body
    // Previous params: (weldbody1 : i32, weldparent1 : i32, asleep1 : i32, weldbody2 : i32, weldparent2 : i32, asleep2 : i32, dsbl_filterparent : i32)
    // Previous return: i32
    extern "C" { fn filterBodyPair_impl (weldbody1 : i32 , weldparent1 : i32 , asleep1 : i32 , weldbody2 : i32 , weldparent2 : i32 , asleep2 : i32 , dsbl_filterparent : i32) -> i32 ; } unsafe { filterBodyPair_impl (weldbody1 , weldparent1 , asleep1 , weldbody2 , weldparent2 , asleep2 , dsbl_filterparent) }
}

/// C: canCollide (engine/engine_collision_driver.c:318)
#[allow(unused_variables, non_snake_case)]
pub fn can_collide(m: *const mjModel, bf: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, bf : i32)
    // Previous return: i32
    extern "C" { fn canCollide_impl (m : * const mjModel , bf : i32) -> i32 ; } unsafe { canCollide_impl (m , bf) }
}

/// C: canCollide2 (engine/engine_collision_driver.c:329)
/// Calls: filterBitmask
#[allow(unused_variables, non_snake_case)]
pub fn can_collide2(m: *const mjModel, bf1: i32, bf2: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, bf1 : i32, bf2 : i32)
    // Previous return: i32
    extern "C" { fn canCollide2_impl (m : * const mjModel , bf1 : i32 , bf2 : i32) -> i32 ; } unsafe { canCollide2_impl (m , bf1 , bf2) }
}

/// C: mj_collideTree (engine/engine_collision_driver.c:361)
/// Calls: canCollide2, filterBitmask, filterBox, filterCollisionPair, filterSphereBox, mj_assignMargin, mj_collideElems, mj_collideGeomElem, mj_collideOBB, mj_collidePlaneFlex, mj_collideSdfFlex, mj_filterSphere, mj_freeStack, mj_markStack, mj_narrowphase, mj_stackAllocInfo, mju_error, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_tree(m: *const mjModel, d: *mut mjData, bf1: i32, bf2: i32, merged: i32, startadr: i32, pairadr: i32) {
    extern "C" { fn mj_collideTree_impl(m: *const mjModel, d: *mut mjData, bf1: i32, bf2: i32, merged: i32, startadr: i32, pairadr: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideTree_impl(m, d, bf1, bf2, merged, startadr, pairadr) }
}

/// C: mj_narrowphase (engine/engine_collision_driver.c:367)
/// Calls: getGap, getMargin, mj_arenaAllocByte, mj_contactParam, mj_freeStack, mj_markStack, mj_maxContact, mj_setContact, mj_stackAllocByte, mj_stackAllocInfo, mj_stackAllocInt, mj_warning, mjc_ccdSize, mji_copy3, mju_copy, mju_dispatch, mju_numThread
#[allow(unused_variables, non_snake_case)]
pub fn mj_narrowphase(m: *const mjModel, d: *mut mjData, buffer: *const mjcPair, npair: i32, parena: usize) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, buffer : * const mjcPair, npair : i32, parena : usize)
    // Previous return: ()
    todo ! ()
}

/// C: mj_collidePlaneFlex (engine/engine_collision_driver.c:371)
/// Calls: mj_addContact, mj_assignMargin, mj_contactParam, mj_setContact, mju_addScl3, mju_copy3, mju_dot3, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_plane_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    extern "C" {
        fn mj_collidePlaneFlex_impl(m: *const mjModel, d: *mut mjData, g: i32, f: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collidePlaneFlex_impl(m, d, g, f) }
}

/// C: mj_collideSdfFlex (engine/engine_collision_driver.c:374)
/// Calls: mj_addContact, mj_assignMargin, mj_contactParam, mj_freeStack, mj_markStack, mj_setContact, mj_stackAllocInfo, mjc_FlexSDF, mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_sdf_flex(m: *const mjModel, d: *mut mjData, g: i32, f: i32) {
    extern "C" { fn mj_collideSdfFlex_impl(m: *const mjModel, d: *mut mjData, g: i32, f: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideSdfFlex_impl(m, d, g, f) }
}

/// C: mj_collideFlexInternal (engine/engine_collision_driver.c:377)
/// Calls: mj_addContact, mj_collideElemVert, mj_contactParam, mj_setContact, mju_copy3, planeVertex
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_internal(m: *const mjModel, d: *mut mjData, f: i32) {
    extern "C" { fn mj_collideFlexInternal_impl(m: *const mjModel, d: *mut mjData, f: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideFlexInternal_impl(m, d, f) }
}

/// C: contactcompare (engine/engine_collision_driver.c:380)
#[allow(unused_variables, non_snake_case)]
pub fn contactcompare(c1: *const mjContact, c2: *const mjContact, context: *mut ()) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (c1 : * const mjContact, c2 : * const mjContact, context : * mut ())
    // Previous return: i32
    extern "C" { fn contactcompare_impl (c1 : * const mjContact , c2 : * const mjContact , context : * mut ()) -> i32 ; } unsafe { contactcompare_impl (c1 , c2 , context) }
}

/// C: contactSort (engine/engine_collision_driver.c:413)
/// Calls: contactcompare
#[allow(unused_variables, non_snake_case)]
pub fn contact_sort(arr: *mut mjContact, buf: *mut mjContact, n: i32, context: *mut ()) {
    // WARNING: signature changed — verify body
    // Previous params: (arr : * mut mjContact, buf : * mut mjContact, n : i32, context : * mut ())
    // Previous return: ()
    extern "C" { fn contactSort_impl (arr : * mut mjContact , buf : * mut mjContact , n : i32 , context : * mut ()) ; } unsafe { contactSort_impl (arr , buf , n , context) }
}

/// C: filterFlexContacts (engine/engine_collision_driver.c:417)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, resetArena
#[allow(unused_variables, non_snake_case)]
pub fn filter_flex_contacts(d: *mut mjData, ncon_before: i32) {
    const MJ_MAXCONPAIR: i32 = 50;
    const MJ_MAXVAL: f64 = 1e10;

    // SAFETY: d is a valid pointer to mjData; ncon_before is a valid contact index
    unsafe {
        let n = (*d).ncon - ncon_before;
        if n <= MJ_MAXCONPAIR {
            return;
        }

        let contacts: *mut mjContact = (*d).contact.add(ncon_before as usize);

        crate::engine::engine_memory::mj_mark_stack(d);
        let selected: *mut u8 = crate::engine::engine_memory::mj_stack_alloc_info(
            d,
            (n as usize) * std::mem::size_of::<u8>(),
            std::mem::align_of::<u8>(),
            b"filterFlexContacts\0".as_ptr() as *const i8,
            0,
        ) as *mut u8;
        let min_dist: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_info(
            d,
            (n as usize) * std::mem::size_of::<f64>(),
            std::mem::align_of::<f64>(),
            b"filterFlexContacts\0".as_ptr() as *const i8,
            0,
        ) as *mut f64;
        std::ptr::write_bytes(selected, 0, n as usize);

        for i in 0..n {
            *min_dist.add(i as usize) = MJ_MAXVAL;
        }

        // start with the deepest penetrating contact
        let mut nselected: i32 = 0;
        let mut best: i32 = 0;
        let mut bestdist: f64 = -(*contacts.add(0)).dist;
        for i in 1..n {
            if -(*contacts.add(i as usize)).dist > bestdist {
                bestdist = -(*contacts.add(i as usize)).dist;
                best = i;
            }
        }

        while nselected < MJ_MAXCONPAIR && best >= 0 {
            *selected.add(best as usize) = 1;
            let bestpos: *mut f64 = (*contacts.add(best as usize)).pos.as_mut_ptr();

            let mut nextbest: i32 = -1;
            let mut nextbestdist: f64 = -1.0;
            for i in 0..n {
                if *selected.add(i as usize) != 0 {
                    continue;
                }

                let dx: f64 = (*contacts.add(i as usize)).pos[0] - *bestpos.add(0);
                let dy: f64 = (*contacts.add(i as usize)).pos[1] - *bestpos.add(1);
                let dz: f64 = (*contacts.add(i as usize)).pos[2] - *bestpos.add(2);
                let d2: f64 = dx * dx + dy * dy + dz * dz;
                if d2 < *min_dist.add(i as usize) {
                    *min_dist.add(i as usize) = d2;
                }
                if *min_dist.add(i as usize) > nextbestdist {
                    nextbestdist = *min_dist.add(i as usize);
                    nextbest = i;
                }
            }

            if nselected < MJ_MAXCONPAIR - 1 {
                let temp: mjContact = *contacts.add(nselected as usize);
                *contacts.add(nselected as usize) = *contacts.add(best as usize);
                *contacts.add(best as usize) = temp;

                if nextbest == nselected {
                    nextbest = best;
                }
            }

            nselected += 1;
            best = nextbest;
        }

        crate::engine::engine_memory::mj_free_stack(d);

        (*d).ncon = ncon_before + nselected;
        reset_arena(d);
    }
}

/// C: pushPairArena (engine/engine_collision_driver.c:489)
/// Calls: mj_arenaAllocByte, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn push_pair_arena(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32) {
    extern "C" {
        fn pushPairArena_impl(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { pushPairArena_impl(m, d, g1, g2, ipair) }
}

/// C: filterCollisionPair (engine/engine_collision_driver.c:508)
/// Calls: filterBitmask, getGap, getMargin, mj_filterSphere
#[allow(unused_variables, non_snake_case)]
pub fn filter_collision_pair(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, ipair: i32, merged: i32, startadr: i32, pairadr: i32) -> i32 {
    // SAFETY: m, d are valid pointers; g1, g2 are valid geom indices.
    unsafe {
        // merged, find matching pair
        if merged != 0 {
            for k in startadr..pairadr {
                if (*(*m).pair_geom1.add(k as usize) == g1 && *(*m).pair_geom2.add(k as usize) == g2)
                    || (*(*m).pair_geom1.add(k as usize) == g2 && *(*m).pair_geom2.add(k as usize) == g1)
                {
                    return 0;
                }
            }
        }

        if ipair >= 0 {
            // mjENABLED(mjENBL_SLEEP) => (m->opt.enableflags & (1<<4)) != 0
            if ((*m).opt.enableflags & (1 << 4)) != 0 {
                let b1 = *(*m).geom_bodyid.add(g1 as usize);
                let b2 = *(*m).geom_bodyid.add(g2 as usize);
                // mjS_AWAKE = 1
                if *(*d).body_awake.add(b1 as usize) != 1
                    && *(*d).body_awake.add(b2 as usize) != 1
                {
                    return 0;
                }
            }
        }

        if ipair < 0 {
            // mjcb_contactfilter is a global callback; access via extern
            extern "C" {
                static mjcb_contactfilter: Option<unsafe extern "C" fn(*const mjModel, *mut mjData, i32, i32) -> i32>;
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
        let margin: f64 = get_margin(m, g1, g2, ipair);
        let gap: f64 = get_gap(m, g1, g2, ipair);
        if mj_filter_sphere(m, d, g1, g2, margin + gap) != 0 {
            return 0;
        }

        // highly unlikely, but check collision function is well-defined
        let type1: i32 = if *(*m).geom_type.add(g1 as usize) < *(*m).geom_type.add(g2 as usize) {
            *(*m).geom_type.add(g1 as usize)
        } else {
            *(*m).geom_type.add(g2 as usize)
        };
        let type2: i32 = if *(*m).geom_type.add(g1 as usize) > *(*m).geom_type.add(g2 as usize) {
            *(*m).geom_type.add(g1 as usize)
        } else {
            *(*m).geom_type.add(g2 as usize)
        };

        // mjCOLLISIONFUNC is a global extern array
        extern "C" {
            static mjCOLLISIONFUNC: [[Option<unsafe extern "C" fn()>; 9]; 9]; // mjNGEOMTYPES = 9
        }
        (mjCOLLISIONFUNC[type1 as usize][type2 as usize].is_some()) as i32
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
    // SAFETY: m, d are valid pointers; bf is a valid bodyflex index; frame is 9-element rotation matrix
    unsafe {
        let mut aamm: [f64; 6] = [0.0; 6];
        let override_margin: f64 = if ((*m).opt.enableflags & (1 << 0)) != 0 {
            0.5 * (*m).opt.o_margin
        } else {
            0.0
        };

        // body
        if (bf as usize) < (*m).nbody {
            let body = bf;
            let body_geomnum = *(*m).body_geomnum.add(body as usize);

            // process all body geoms
            for i in 0..body_geomnum {
                let geom = *(*m).body_geomadr.add(body as usize) + i;
                let margin: f64 = if override_margin != 0.0 {
                    override_margin
                } else {
                    *(*m).geom_margin.add(geom as usize) + *(*m).geom_gap.add(geom as usize)
                };
                let mut _aamm: [f64; 6] = [0.0; 6];

                let aabb: *const f64 = (*m).geom_aabb.add(6 * geom as usize);
                let size: *const f64 = (*m).geom_aabb.add(6 * geom as usize + 3);
                let xpos: *const f64 = (*d).geom_xpos.add(3 * geom as usize);
                let xmat: *const f64 = (*d).geom_xmat.add(9 * geom as usize);

                // compute center in global coordinates
                let mut pos: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_mul_mat_vec3(pos.as_mut_ptr(), xmat, aabb);
                crate::engine::engine_util_blas::mju_add_to3(pos.as_mut_ptr(), xpos);

                let mut axis: [f64; 9] = [0.0; 9];
                crate::engine::engine_inline::mji_transpose3(axis.as_mut_ptr(), xmat);
                let r_half: f64 = *(*m).geom_rbound.add(geom as usize);

                for j in 0..3 {
                    let frame_j: *const f64 = frame.add(3 * j);
                    let aabb_cen: f64 = crate::engine::engine_util_blas::mju_dot3(pos.as_ptr(), frame_j);
                    let aabb_half: f64 =
                        (*size.add(0) * crate::engine::engine_util_blas::mju_dot3(axis.as_ptr().add(0), frame_j)).abs()
                        + (*size.add(1) * crate::engine::engine_util_blas::mju_dot3(axis.as_ptr().add(3), frame_j)).abs()
                        + (*size.add(2) * crate::engine::engine_util_blas::mju_dot3(axis.as_ptr().add(6), frame_j)).abs();
                    let r_cen: f64 = crate::engine::engine_util_blas::mju_dot3(xpos, frame_j);
                    _aamm[j + 0] = crate::engine::engine_util_misc::mju_max(r_cen - r_half, aabb_cen - aabb_half) - margin;
                    _aamm[j + 3] = crate::engine::engine_util_misc::mju_min(r_cen + r_half, aabb_cen + aabb_half) + margin;
                }

                // update body aamm
                if i == 0 {
                    crate::engine::engine_util_blas::mju_copy(aamm.as_mut_ptr(), _aamm.as_ptr(), 6);
                } else {
                    for j in 0..3 {
                        aamm[j] = crate::engine::engine_util_misc::mju_min(aamm[j], _aamm[j]);
                        aamm[j + 3] = crate::engine::engine_util_misc::mju_max(aamm[j + 3], _aamm[j + 3]);
                    }
                }
            }
        }
        // flex
        else {
            let f = bf - (*m).nbody as i32;
            let flex_vertnum = *(*m).flex_vertnum.add(f as usize);
            let vbase: *const f64 = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(f as usize) as usize);

            // process flex vertices
            for i in 0..flex_vertnum {
                let mut v: [f64; 3] = [0.0; 3];

                // compute vertex coordinates in given frame
                crate::engine::engine_util_blas::mju_mul_mat_vec(v.as_mut_ptr(), frame, vbase.add(3 * i as usize), 3, 3);

                // update aamm
                if i == 0 {
                    crate::engine::engine_util_blas::mju_copy3(aamm.as_mut_ptr(), v.as_ptr());
                    crate::engine::engine_util_blas::mju_copy3(aamm.as_mut_ptr().add(3), v.as_ptr());
                } else {
                    for j in 0..3_usize {
                        aamm[j] = crate::engine::engine_util_misc::mju_min(aamm[j], v[j]);
                        aamm[j + 3] = crate::engine::engine_util_misc::mju_max(aamm[j + 3], v[j]);
                    }
                }
            }

            // correct for flex radius and margin
            let margin: f64 = if override_margin != 0.0 {
                override_margin
            } else {
                *(*m).flex_margin.add(f as usize) + *(*m).flex_gap.add(f as usize)
            };
            let bound: f64 = *(*m).flex_radius.add(f as usize) + margin;
            aamm[0] -= bound;
            aamm[1] -= bound;
            aamm[2] -= bound;
            aamm[3] += bound;
            aamm[4] += bound;
            aamm[5] += bound;
        }

        // assign outputs
        *x_min = aamm[0];
        *y_min = aamm[1];
        *z_min = aamm[2];
        *x_max = aamm[3];
        *y_max = aamm[4];
        *z_max = aamm[5];
    }
}

/// C: add_pair (engine/engine_collision_driver.c:1315)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn add_pair(m: *const mjModel, bf1: i32, bf2: i32, npair: *mut i32, pair: *mut i32, maxpair: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, bf1 : i32, bf2 : i32, npair : * mut i32, pair : * mut i32, maxpair : i32)
    // Previous return: ()
    extern "C" { fn add_pair_impl (m : * const mjModel , bf1 : i32 , bf2 : i32 , npair : * mut i32 , pair : * mut i32 , maxpair : i32) ; } unsafe { add_pair_impl (m , bf1 , bf2 , npair , pair , maxpair) }
}

/// C: SAPcmp (engine/engine_collision_driver.c:1383)
#[allow(unused_variables, non_snake_case)]
pub fn sa_pcmp(obj1: *mut mjtSAP, obj2: *mut mjtSAP, context: *mut ()) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (obj1 : * mut mjtSAP, obj2 : * mut mjtSAP, context : * mut ())
    // Previous return: i32
    todo ! ()
}

/// C: SAPsort (engine/engine_collision_driver.c:1394)
/// Calls: SAPcmp
#[allow(unused_variables, non_snake_case)]
pub fn sa_psort(arr: *mut mjtSAP, buf: *mut mjtSAP, n: i32, context: *mut ()) {
    // WARNING: signature changed — verify body
    // Previous params: (arr : * mut mjtSAP, buf : * mut mjtSAP, n : i32, context : * mut ())
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, aamm : * const f64, n : i32, axis_x : i32, pair : * mut i32, maxpair : i32)
    // Previous return: i32
    extern "C" { fn mj_SAP_impl (d : * mut mjData , aamm : * const f64 , n : i32 , axis_x : i32 , pair : * mut i32 , maxpair : i32) -> i32 ; } unsafe { mj_SAP_impl (d , aamm , n , axis_x , pair , maxpair) }
}

/// C: updateCov (engine/engine_collision_driver.c:1497)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_cov(cov: *mut f64, vec: *const f64, cen: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (cov : * mut f64, vec : * const f64, cen : * const f64)
    // Previous return: ()
    extern "C" { fn updateCov_impl (cov : * mut f64 , vec : * const f64 , cen : * const f64) ; } unsafe { updateCov_impl (cov , vec , cen) }
}

/// C: uintcmp (engine/engine_collision_driver.c:1518)
#[allow(unused_variables, non_snake_case)]
pub fn uintcmp(i: *mut i32, j: *mut i32, context: *mut ()) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (i : * mut i32, j : * mut i32, context : * mut ())
    // Previous return: i32
    todo ! ()
}

/// C: bfsort (engine/engine_collision_driver.c:1529)
/// Calls: uintcmp
#[allow(unused_variables, non_snake_case)]
pub fn bfsort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()) {
    // WARNING: signature changed — verify body
    // Previous params: (arr : * mut i32, buf : * mut i32, n : i32, context : * mut ())
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, condim : * mut i32, solref : * mut f64, solimp : * mut f64, friction : * mut f64, g1 : i32, g2 : i32, f1 : i32, f2 : i32)
    // Previous return: ()
    extern "C" { fn mj_contactParam_impl (m : * const mjModel , condim : * mut i32 , solref : * mut f64 , solimp : * mut f64 , friction : * mut f64 , g1 : i32 , g2 : i32 , f1 : i32 , f2 : i32) ; } unsafe { mj_contactParam_impl (m , condim , solref , solimp , friction , g1 , g2 , f1 , f2) }
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
    // SAFETY: m, con are valid pointers; solref, solreffriction, solimp, friction are valid
    // pointer inputs (or the assign functions handle null).
    unsafe {
        // set parameters
        (*con).dim = condim;
        (*con).includemargin = includemargin;
        crate::engine::engine_core_constraint::mj_assign_ref(m, (*con).solref.as_mut_ptr(), solref);
        crate::engine::engine_core_constraint::mj_assign_ref(m, (*con).solreffriction.as_mut_ptr(), solreffriction);
        crate::engine::engine_core_constraint::mj_assign_imp(m, (*con).solimp.as_mut_ptr(), solimp);
        crate::engine::engine_core_constraint::mj_assign_friction(m, (*con).friction.as_mut_ptr(), friction);

        // exclude in gap
        (*con).exclude = ((*con).dist >= includemargin) as i32;

        // complete frame
        crate::engine::engine_util_spatial::mju_make_frame((*con).frame.as_mut_ptr());

        // clear fields that are computed later
        (*con).efc_address = -1;
        (*con).mu = 0.0;
        crate::engine::engine_util_blas::mju_zero((*con).H.as_mut_ptr(), 36);

        // set deprecated fields
        (*con).geom1 = (*con).geom[0];
        (*con).geom2 = (*con).geom[1];
    }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, f : i32, vid : [i32 ; 2], pos : * mut f64, mat : * mut f64, size : * mut f64)
    // Previous return: ()
    extern "C" { fn mj_makeCapsule_impl (m : * const mjModel , d : * mut mjData , f : i32 , vid : [i32 ; 2] , pos : * mut f64 , mat : * mut f64 , size : * mut f64) ; } unsafe { mj_makeCapsule_impl (m , d , f , vid , pos , mat , size) }
}

/// C: collisionTask (engine/engine_collision_driver.c:1849)
/// Calls: getGap, getMargin, mjc_setCCDBuffer, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn collision_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, idx: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, arg : * mut (), thread_id : i32, idx : i32)
    // Previous return: ()
    todo ! ()
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
    extern "C" {
        fn planeVertex_impl(con: *mut mjPreContact, pos: *const f64, rad: f64, t0: i32, t1: i32, t2: i32, v: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { planeVertex_impl(con, pos, rad, t0, t1, t2, v) }
}

/// C: mj_maxContact (engine/engine_collision_driver.h:33)
#[allow(unused_variables, non_snake_case)]
pub fn mj_max_contact(m: *const mjModel, g1: i32, g2: i32, has_margin: i32) -> i32 {

    extern "C" { fn mj_maxContact_impl(m: *const mjModel, g1: i32, g2: i32, has_margin: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_maxContact_impl(m, g1, g2, has_margin) }
}

/// C: mj_collision (engine/engine_collision_driver.h:36)
/// Calls: alignArena, canCollide2, contactSort, filterBitmask, filterCollisionPair, filterFlexContacts, mj_broadphase, mj_clearEfc, mj_collideElems, mj_collideFlexInternal, mj_collideFlexSAP, mj_collideGeomElem, mj_collidePlaneFlex, mj_collideSdfFlex, mj_collideTree, mj_freeStack, mj_isElemActive, mj_markStack, mj_narrowphase, mj_sleepState, mj_stackAllocInfo, pushPairArena, resetArena
#[allow(unused_variables, non_snake_case)]
pub fn mj_collision(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
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
    extern "C" {
        fn mj_collideOBB_impl(aabb1: *const f64, aabb2: *const f64, xpos1: *const f64, xmat1: *const f64, xpos2: *const f64, xmat2: *const f64, margin: f64, product: *mut f64, offset: *mut f64, initialize: *mut mjtBool) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideOBB_impl(aabb1, aabb2, xpos1, xmat1, xpos2, xmat2, margin, product, offset, initialize) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, bfpair : * mut i32, maxpair : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_collideFlexSAP (engine/engine_collision_driver.h:51)
/// Calls: mj_SAP, mj_collideElems, mj_freeStack, mj_isElemActive, mj_markStack, mj_stackAllocInfo, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_flex_sap(m: *const mjModel, d: *mut mjData, f: i32) {
    extern "C" { fn mj_collideFlexSAP_impl(m: *const mjModel, d: *mut mjData, f: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideFlexSAP_impl(m, d, f) }
}

/// C: mj_collideGeomElem (engine/engine_collision_driver.h:54)
/// Calls: filterSphereBox, mj_arenaAllocByte, mj_assignMargin, mj_contactParam, mj_freeStack, mj_makeCapsule, mj_markStack, mj_setContact, mj_stackAllocInfo, mj_warning, mjc_ConvexElem, mjc_HFieldElem, mjraw_BoxTriangle, mjraw_CapsuleBox, mjraw_CapsuleCapsule, mjraw_CapsuleTriangle, mjraw_SphereCapsule, mjraw_SphereTriangle, mju_copy3, mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_geom_elem(m: *const mjModel, d: *mut mjData, g: i32, f: i32, e: i32) {
    extern "C" { fn mj_collideGeomElem_impl(m: *const mjModel, d: *mut mjData, g: i32, f: i32, e: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideGeomElem_impl(m, d, g, f, e) }
}

/// C: mj_collideElems (engine/engine_collision_driver.h:57)
/// Calls: filterBox, mj_arenaAllocByte, mj_assignMargin, mj_contactParam, mj_freeStack, mj_makeCapsule, mj_markStack, mj_setContact, mj_stackAllocInfo, mj_warning, mjc_ConvexElem, mjraw_CapsuleCapsule, mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elems(m: *const mjModel, d: *mut mjData, f1: i32, e1: i32, f2: i32, e2: i32) {
    extern "C" { fn mj_collideElems_impl(m: *const mjModel, d: *mut mjData, f1: i32, e1: i32, f2: i32, e2: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideElems_impl(m, d, f1, e1, f2, e2) }
}

/// C: mj_collideElemVert (engine/engine_collision_driver.h:60)
/// Calls: mj_arenaAllocByte, mj_assignMargin, mj_contactParam, mj_freeStack, mj_makeCapsule, mj_markStack, mj_setContact, mj_stackAllocInfo, mj_warning, mjc_ConvexElem, mjraw_SphereCapsule, mjraw_SphereTriangle, mju_copy3
#[allow(unused_variables, non_snake_case)]
pub fn mj_collide_elem_vert(m: *const mjModel, d: *mut mjData, f: i32, e: i32, v: i32) {
    extern "C" { fn mj_collideElemVert_impl(m: *const mjModel, d: *mut mjData, f: i32, e: i32, v: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_collideElemVert_impl(m, d, f, e, v) }
}

