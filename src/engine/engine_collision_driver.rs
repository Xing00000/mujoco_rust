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
    // SAFETY: m is a valid pointer to mjModel, g1/g2/ipair are valid indices
    unsafe {
        if ipair >= 0 {
            *(*m).pair_gap.add(ipair as usize)
        } else {
            *(*m).geom_gap.add(g1 as usize) + *(*m).geom_gap.add(g2 as usize)
        }
    }
}

/// C: resetArena (engine/engine_collision_driver.c:179)
#[allow(unused_variables, non_snake_case)]
pub fn reset_arena(d: *mut mjData) {
    // SAFETY: d is a valid pointer to mjData
    unsafe {
        (*d).parena = (*d).ncon as usize * core::mem::size_of::<mjContact>();
    }
}

/// C: alignArena (engine/engine_collision_driver.c:189)
#[allow(unused_variables, non_snake_case)]
pub fn align_arena(d: *mut mjData, alignment: usize) -> usize {
    // SAFETY: d is a valid pointer to mjData
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
    // SAFETY: m, d are valid pointers; g1 is a plane geom, g2 is any geom
    unsafe {
        let mat1: *const f64 = (*d).geom_xmat.add(9 * g1 as usize);
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
    // SAFETY: m is valid mjModel pointer; body is a valid body index
    unsafe {
        let start = *(*m).body_geomadr.add(body as usize);
        let end = start + *(*m).body_geomnum.add(body as usize);
        let mut g = start;
        while g < end {
            if *(*m).geom_type.add(g as usize) == 0 {  // mjGEOM_PLANE = 0
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
pub fn can_collide2(m: *const mjModel, bf1: i32, bf2: i32) -> i32 {
    // SAFETY: m is valid mjModel pointer; bf1, bf2 are valid bodyflex indices
    unsafe {
        let nbody = (*m).nbody as i32;
        let contype1: i32 = if bf1 < nbody {
            *(*m).body_contype.add(bf1 as usize)
        } else {
            *(*m).flex_contype.add((bf1 - nbody) as usize)
        };
        let conaffinity1: i32 = if bf1 < nbody {
            *(*m).body_conaffinity.add(bf1 as usize)
        } else {
            *(*m).flex_conaffinity.add((bf1 - nbody) as usize)
        };
        let contype2: i32 = if bf2 < nbody {
            *(*m).body_contype.add(bf2 as usize)
        } else {
            *(*m).flex_contype.add((bf2 - nbody) as usize)
        };
        let conaffinity2: i32 = if bf2 < nbody {
            *(*m).body_conaffinity.add(bf2 as usize)
        } else {
            *(*m).flex_conaffinity.add((bf2 - nbody) as usize)
        };
        // opposite of bitmask filter
        (filter_bitmask(contype1, conaffinity1, contype2, conaffinity2) == 0) as i32
    }
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
    // SAFETY: m, d valid per caller. All pointer arithmetic within allocated model/data arrays.
    unsafe {
        const MJ_NREF: usize = 2;
        const MJ_NIMP: usize = 5;

        let radius: f64 = *(*m).flex_radius.add(f as usize);
        let pos: *const f64 = (*d).geom_xpos.add(3 * g as usize);
        let mat: *const f64 = (*d).geom_xmat.add(9 * g as usize);
        let nrm: [f64; 3] = [*mat.add(2), *mat.add(5), *mat.add(8)];

        // prepare contact parameters (same for all vertices)
        let margin: f64 = crate::engine::engine_core_constraint::mj_assign_margin(
            m, *(*m).geom_margin.add(g as usize) + *(*m).flex_margin.add(f as usize));
        let mut condim: i32 = 0;
        let flex_vertnum: i32 = *(*m).flex_vertnum.add(f as usize);
        let gap: f64 = *(*m).geom_gap.add(g as usize) + *(*m).flex_gap.add(f as usize);
        let mut solref: [f64; 2] = [0.0; 2];
        let mut solimp: [f64; 5] = [0.0; 5];
        let mut friction: [f64; 5] = [0.0; 5];
        let solreffriction: [f64; 2] = [0.0; 2];
        crate::engine::engine_collision_driver::mj_contact_param(
            m, &mut condim, solref.as_mut_ptr(), solimp.as_mut_ptr(),
            friction.as_mut_ptr(), g, -1, -1, f);

        // collide all flex vertices with plane
        let mut i: i32 = 0;
        while i < flex_vertnum {
            let v: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(f as usize) + i) as usize);

            // distance from plane to vertex
            let dif: [f64; 3] = [
                *v.add(0) - *pos.add(0),
                *v.add(1) - *pos.add(1),
                *v.add(2) - *pos.add(2),
            ];
            let dist: f64 = crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), nrm.as_ptr());

            // no contact
            if dist > margin + gap + radius {
                i += 1;
                continue;
            }

            // create contact
            let mut con: mjContact = core::mem::zeroed();
            con.dist = dist - radius;
            crate::engine::engine_util_blas::mju_add_scl3(
                con.pos.as_mut_ptr(), v, nrm.as_ptr(), -con.dist * 0.5 - radius);
            crate::engine::engine_util_blas::mju_copy3(con.frame.as_mut_ptr(), nrm.as_ptr());
            crate::engine::engine_util_blas::mju_zero3(con.frame.as_mut_ptr().add(3));

            // set contact ids
            con.geom[0] = g;
            con.geom[1] = -1;
            con.flex[0] = -1;
            con.flex[1] = f;
            con.elem[0] = -1;
            con.elem[1] = -1;
            con.vert[0] = -1;
            con.vert[1] = i;

            // set remaining contact parameters
            crate::engine::engine_collision_driver::mj_set_contact(
                m, &mut con, condim, margin,
                solref.as_ptr(), solreffriction.as_ptr(), solimp.as_ptr(), friction.as_ptr());

            // add to mjData, abort if too many contacts
            if crate::engine::engine_core_constraint::mj_add_contact(m, d, &con) != 0 {
                return;
            }

            i += 1;
        }
    }
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
    // SAFETY: c1, c2 are valid mjContact pointers; context is a valid mjModel pointer
    unsafe {
        let m: *const mjModel = context as *const mjModel;

        // get colliding object ids
        let mut con1_obj1: i32 = if (*c1).geom[0] >= 0 { (*c1).geom[0] }
            else if (*c1).elem[0] >= 0 { (*c1).elem[0] }
            else { (*c1).vert[0] };
        let mut con1_obj2: i32 = if (*c1).geom[1] >= 0 { (*c1).geom[1] }
            else if (*c1).elem[1] >= 0 { (*c1).elem[1] }
            else { (*c1).vert[1] };
        let mut con2_obj1: i32 = if (*c2).geom[0] >= 0 { (*c2).geom[0] }
            else if (*c2).elem[0] >= 0 { (*c2).elem[0] }
            else { (*c2).vert[0] };
        let mut con2_obj2: i32 = if (*c2).geom[1] >= 0 { (*c2).geom[1] }
            else if (*c2).elem[1] >= 0 { (*c2).elem[1] }
            else { (*c2).vert[1] };

        // for geom:geom, undo type-based swapping for sorting purposes
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
    // SAFETY: arr and buf are valid arrays of at least n elements.
    // Implements mjSORT(contactSort, mjContact, contactcompare) — timsort.
    unsafe {
        const RUNSIZE: i32 = 32;

        // insertion sort for small runs
        let mut start: i32 = 0;
        while start < n {
            let end = if start + RUNSIZE < n { start + RUNSIZE } else { n };
            let mut j = start + 1;
            while j < end {
                let tmp = *arr.add(j as usize);
                let mut k = j - 1;
                while k >= start && contactcompare(arr.add(k as usize), &tmp as *const mjContact, context) > 0 {
                    *arr.add(k as usize + 1) = *arr.add(k as usize);
                    k -= 1;
                }
                *arr.add(k as usize + 1) = tmp;
                j += 1;
            }
            start += RUNSIZE;
        }

        // bottom-up merge
        let mut src = arr;
        let mut dest = buf;
        let mut len: i32 = RUNSIZE;
        while len < n {
            let mut ms: i32 = 0;
            while ms < n {
                let mid = ms + len;
                let end = if ms + 2 * len < n { ms + 2 * len } else { n };
                if mid < end {
                    let mut i = ms;
                    let mut j_idx = mid;
                    let mut k = ms;
                    while i < mid && j_idx < end {
                        if contactcompare(src.add(i as usize), src.add(j_idx as usize), context) <= 0 {
                            *dest.add(k as usize) = *src.add(i as usize);
                            i += 1;
                        } else {
                            *dest.add(k as usize) = *src.add(j_idx as usize);
                            j_idx += 1;
                        }
                        k += 1;
                    }
                    if i < mid {
                        core::ptr::copy_nonoverlapping(
                            src.add(i as usize),
                            dest.add(k as usize),
                            (mid - i) as usize,
                        );
                    } else if j_idx < end {
                        core::ptr::copy_nonoverlapping(
                            src.add(j_idx as usize),
                            dest.add(k as usize),
                            (end - j_idx) as usize,
                        );
                    }
                } else {
                    core::ptr::copy_nonoverlapping(
                        src.add(ms as usize),
                        dest.add(ms as usize),
                        (end - ms) as usize,
                    );
                }
                ms += 2 * len;
            }
            let tmp = src;
            src = dest;
            dest = tmp;
            len *= 2;
        }
        if src != arr {
            core::ptr::copy_nonoverlapping(src, arr, n as usize);
        }
    }
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
    // SAFETY: m, d are valid pointers. g1/g2 are valid geom indices.
    // mjcPair layout: { g1: i32, g2: i32, ipair: i32 } = 12 bytes, align 4
    unsafe {
        let pair_ptr: *mut u8 = crate::engine::engine_memory::mj_arena_alloc_byte(
            d, 12, 4
        ) as *mut u8;
        if pair_ptr.is_null() {
            extern "C" { fn mju_error(msg: *const i8, ...); }
            mju_error(b"arena too small to allocate geom pair\0".as_ptr() as *const i8);
            return;
        }

        // swap g1/g2 if both valid and type[g1] > type[g2]
        let (out_g1, out_g2) = if g1 >= 0 && g2 >= 0
            && *(*m).geom_type.add(g1 as usize) > *(*m).geom_type.add(g2 as usize)
        {
            (g2, g1)
        } else {
            (g1, g2)
        };

        *(pair_ptr.add(0) as *mut i32) = out_g1;
        *(pair_ptr.add(4) as *mut i32) = out_g2;
        *(pair_ptr.add(8) as *mut i32) = ipair;
    }
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
    // SAFETY: arr and buf are valid arrays of at least n elements.
    // Implements mjSORT(SAPsort, mjtSAP, SAPcmp) — timsort.
    unsafe {
        const RUNSIZE: i32 = 32;

        // insertion sort for small runs
        let mut start: i32 = 0;
        while start < n {
            let end = if start + RUNSIZE < n { start + RUNSIZE } else { n };
            let mut j = start + 1;
            while j < end {
                let tmp = *arr.add(j as usize);
                let mut k = j - 1;
                while k >= start && sa_pcmp(arr.add(k as usize), &tmp as *const mjtSAP as *mut mjtSAP, context) > 0 {
                    *arr.add(k as usize + 1) = *arr.add(k as usize);
                    k -= 1;
                }
                *arr.add(k as usize + 1) = tmp;
                j += 1;
            }
            start += RUNSIZE;
        }

        // bottom-up merge
        let mut src = arr;
        let mut dest = buf;
        let mut len: i32 = RUNSIZE;
        while len < n {
            let mut ms: i32 = 0;
            while ms < n {
                let mid = ms + len;
                let end = if ms + 2 * len < n { ms + 2 * len } else { n };
                if mid < end {
                    let mut i = ms;
                    let mut j_idx = mid;
                    let mut k = ms;
                    while i < mid && j_idx < end {
                        if sa_pcmp(src.add(i as usize), src.add(j_idx as usize), context) <= 0 {
                            *dest.add(k as usize) = *src.add(i as usize);
                            i += 1;
                        } else {
                            *dest.add(k as usize) = *src.add(j_idx as usize);
                            j_idx += 1;
                        }
                        k += 1;
                    }
                    if i < mid {
                        core::ptr::copy_nonoverlapping(
                            src.add(i as usize),
                            dest.add(k as usize),
                            (mid - i) as usize,
                        );
                    } else if j_idx < end {
                        core::ptr::copy_nonoverlapping(
                            src.add(j_idx as usize),
                            dest.add(k as usize),
                            (end - j_idx) as usize,
                        );
                    }
                } else {
                    core::ptr::copy_nonoverlapping(
                        src.add(ms as usize),
                        dest.add(ms as usize),
                        (end - ms) as usize,
                    );
                }
                ms += 2 * len;
            }
            let tmp = src;
            src = dest;
            dest = tmp;
            len *= 2;
        }
        if src != arr {
            core::ptr::copy_nonoverlapping(src, arr, n as usize);
        }
    }
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
    // SAFETY: d valid mjData; aamm has 6*n elements (column-major); pair has maxpair capacity
    unsafe {
        // check inputs
        if n >= 0x10000 || axis_x < 0 || axis_x > 2 || maxpair < 1 {
            return -1;
        }

        // allocate sort buffer
        let sortbuf: *mut mjtSAP = crate::engine::engine_memory::mj_stack_alloc_info(
            d,
            (2 * n as usize) * core::mem::size_of::<mjtSAP>(),
            core::mem::align_of::<mjtSAP>(),
            b"mj_SAP\0".as_ptr() as *const i8,
            0,
        ) as *mut mjtSAP;
        let activebuf: *mut mjtSAP = crate::engine::engine_memory::mj_stack_alloc_info(
            d,
            (2 * n as usize) * core::mem::size_of::<mjtSAP>(),
            core::mem::align_of::<mjtSAP>(),
            b"mj_SAP\0".as_ptr() as *const i8,
            0,
        ) as *mut mjtSAP;

        // get AAMM pointers for primary "x" axis
        let x_min: *const f64 = aamm.add(n as usize * (axis_x as usize + 0));
        let x_max: *const f64 = aamm.add(n as usize * (axis_x as usize + 3));

        // init sortbuf with specified axis
        for i in 0..n {
            (*sortbuf.add(2 * i as usize)).id_ismax = i;
            (*sortbuf.add(2 * i as usize)).value = *x_min.add(i as usize) as f32;
            (*sortbuf.add(2 * i as usize + 1)).id_ismax = i + 0x10000;
            (*sortbuf.add(2 * i as usize + 1)).value = *x_max.add(i as usize) as f32;
        }

        // sort along specified axis
        let buf: *mut mjtSAP = crate::engine::engine_memory::mj_stack_alloc_info(
            d,
            (2 * n as usize) * core::mem::size_of::<mjtSAP>(),
            core::mem::align_of::<mjtSAP>(),
            b"mj_SAP\0".as_ptr() as *const i8,
            0,
        ) as *mut mjtSAP;
        sa_psort(sortbuf, buf, 2 * n, core::ptr::null_mut());

        // define the other two axes
        let axis_y: i32;
        let axis_z: i32;
        if axis_x == 0 {
            axis_y = 1;
            axis_z = 2;
        } else if axis_x == 1 {
            axis_y = 0;
            axis_z = 2;
        } else {
            axis_y = 0;
            axis_z = 1;
        }

        // get AAMM pointers to secondary "y, z" axes
        let y_min: *const f64 = aamm.add(n as usize * (axis_y as usize + 0));
        let y_max: *const f64 = aamm.add(n as usize * (axis_y as usize + 3));
        let z_min: *const f64 = aamm.add(n as usize * (axis_z as usize + 0));
        let z_max: *const f64 = aamm.add(n as usize * (axis_z as usize + 3));

        // sweep and prune
        let mut cnt: i32 = 0;    // size of active list
        let mut npair: i32 = 0;  // number of pairs added
        for i in 0..(2 * n) {
            // min value: collide with all in list, add
            if ((*sortbuf.add(i as usize)).id_ismax & 0x10000) == 0 {
                for j in 0..cnt {
                    let id1: i32 = (*activebuf.add(j as usize)).id_ismax;
                    let id2: i32 = (*sortbuf.add(i as usize)).id_ismax;

                    // use the other two axes to prune if possible
                    if *y_min.add(id1 as usize) > *y_max.add(id2 as usize)
                        || *y_min.add(id2 as usize) > *y_max.add(id1 as usize)
                        || *z_min.add(id1 as usize) > *z_max.add(id2 as usize)
                        || *z_min.add(id2 as usize) > *z_max.add(id1 as usize)
                    {
                        continue;
                    }

                    // add pair, check buffer size
                    *pair.add(npair as usize) = (id1 << 16) + id2;
                    npair += 1;
                    if npair >= maxpair {
                        return maxpair;
                    }
                }

                // add to list
                *activebuf.add(cnt as usize) = *sortbuf.add(i as usize);
                cnt += 1;
            }
            // max value: remove corresponding min value from list
            else {
                let toremove: i32 = (*sortbuf.add(i as usize)).id_ismax & 0xFFFF;
                for j in 0..cnt {
                    if (*activebuf.add(j as usize)).id_ismax == toremove {
                        if j < cnt - 1 {
                            core::ptr::copy(
                                activebuf.add(j as usize + 1),
                                activebuf.add(j as usize),
                                (cnt - 1 - j) as usize,
                            );
                        }
                        cnt -= 1;
                        break;
                    }
                }
            }
        }

        npair
    }
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
    // SAFETY: arr and buf are valid arrays of at least n elements.
    // Implements mjSORT(bfsort, int, uintcmp) macro expansion — timsort.
    unsafe {
        const RUNSIZE: i32 = 32;

        // insertion sort for small runs
        let mut start: i32 = 0;
        while start < n {
            let end = if start + RUNSIZE < n { start + RUNSIZE } else { n };
            // _mjINSERTION_SORT
            let mut j = start + 1;
            while j < end {
                let tmp = *arr.add(j as usize);
                let mut k = j - 1;
                while k >= start && uintcmp(arr.add(k as usize), &tmp as *const i32 as *mut i32, context) > 0 {
                    *arr.add(k as usize + 1) = *arr.add(k as usize);
                    k -= 1;
                }
                *arr.add(k as usize + 1) = tmp;
                j += 1;
            }
            start += RUNSIZE;
        }

        // bottom-up merge
        let mut src = arr;
        let mut dest = buf;
        let mut len: i32 = RUNSIZE;
        while len < n {
            let mut ms: i32 = 0;
            while ms < n {
                let mid = ms + len;
                let end = if ms + 2 * len < n { ms + 2 * len } else { n };
                if mid < end {
                    // _mjMERGE
                    let mut i = ms;
                    let mut j = mid;
                    let mut k = ms;
                    while i < mid && j < end {
                        if uintcmp(src.add(i as usize), src.add(j as usize), context) <= 0 {
                            *dest.add(k as usize) = *src.add(i as usize);
                            i += 1;
                        } else {
                            *dest.add(k as usize) = *src.add(j as usize);
                            j += 1;
                        }
                        k += 1;
                    }
                    if i < mid {
                        core::ptr::copy_nonoverlapping(
                            src.add(i as usize),
                            dest.add(k as usize),
                            (mid - i) as usize,
                        );
                    } else if j < end {
                        core::ptr::copy_nonoverlapping(
                            src.add(j as usize),
                            dest.add(k as usize),
                            (end - j) as usize,
                        );
                    }
                } else {
                    core::ptr::copy_nonoverlapping(
                        src.add(ms as usize),
                        dest.add(ms as usize),
                        (end - ms) as usize,
                    );
                }
                ms += 2 * len;
            }
            let tmp = src;
            src = dest;
            dest = tmp;
            len *= 2;
        }
        if src != arr {
            core::ptr::copy_nonoverlapping(src, arr, n as usize);
        }
    }
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
    // SAFETY: m valid; condim, solref, solimp, friction are valid output buffers
    unsafe {
        const MJ_NREF: usize = 2;
        const MJ_NIMP: usize = 5;
        const MJ_MINVAL: f64 = 1e-15;

        let mut fri: [f64; 3] = [0.0; 3];

        // get parameters from geom1 or flex1
        let priority1: i32 = if f1 < 0 { *(*m).geom_priority.add(g1 as usize) } else { *(*m).flex_priority.add(f1 as usize) };
        let condim1: i32 = if f1 < 0 { *(*m).geom_condim.add(g1 as usize) } else { *(*m).flex_condim.add(f1 as usize) };
        let solmix1: f64 = if f1 < 0 { *(*m).geom_solmix.add(g1 as usize) } else { *(*m).flex_solmix.add(f1 as usize) };
        let solref1: *const f64 = if f1 < 0 { (*m).geom_solref.add(g1 as usize * MJ_NREF) } else { (*m).flex_solref.add(f1 as usize * MJ_NREF) };
        let solimp1: *const f64 = if f1 < 0 { (*m).geom_solimp.add(g1 as usize * MJ_NIMP) } else { (*m).flex_solimp.add(f1 as usize * MJ_NIMP) };
        let friction1: *const f64 = if f1 < 0 { (*m).geom_friction.add(g1 as usize * 3) } else { (*m).flex_friction.add(f1 as usize * 3) };

        // get parameters from geom2 or flex2
        let priority2: i32 = if f2 < 0 { *(*m).geom_priority.add(g2 as usize) } else { *(*m).flex_priority.add(f2 as usize) };
        let condim2: i32 = if f2 < 0 { *(*m).geom_condim.add(g2 as usize) } else { *(*m).flex_condim.add(f2 as usize) };
        let solmix2: f64 = if f2 < 0 { *(*m).geom_solmix.add(g2 as usize) } else { *(*m).flex_solmix.add(f2 as usize) };
        let solref2: *const f64 = if f2 < 0 { (*m).geom_solref.add(g2 as usize * MJ_NREF) } else { (*m).flex_solref.add(f2 as usize * MJ_NREF) };
        let solimp2: *const f64 = if f2 < 0 { (*m).geom_solimp.add(g2 as usize * MJ_NIMP) } else { (*m).flex_solimp.add(f2 as usize * MJ_NIMP) };
        let friction2: *const f64 = if f2 < 0 { (*m).geom_friction.add(g2 as usize * 3) } else { (*m).flex_friction.add(f2 as usize * 3) };

        // different priority: copy from item with higher priority
        if priority1 > priority2 {
            *condim = condim1;
            crate::engine::engine_util_blas::mju_copy(solref, solref1, MJ_NREF as i32);
            crate::engine::engine_util_blas::mju_copy(solimp, solimp1, MJ_NIMP as i32);
            crate::engine::engine_util_blas::mju_copy(fri.as_mut_ptr(), friction1, 3);
        } else if priority1 < priority2 {
            *condim = condim2;
            crate::engine::engine_util_blas::mju_copy(solref, solref2, MJ_NREF as i32);
            crate::engine::engine_util_blas::mju_copy(solimp, solimp2, MJ_NIMP as i32);
            crate::engine::engine_util_blas::mju_copy(fri.as_mut_ptr(), friction2, 3);
        }
        // same priority
        else {
            // condim: max
            *condim = if condim1 > condim2 { condim1 } else { condim2 };

            // compute solver mix factor
            let mix: f64;
            if solmix1 >= MJ_MINVAL && solmix2 >= MJ_MINVAL {
                mix = solmix1 / (solmix1 + solmix2);
            } else if solmix1 < MJ_MINVAL && solmix2 < MJ_MINVAL {
                mix = 0.5;
            } else if solmix1 < MJ_MINVAL {
                mix = 0.0;
            } else {
                mix = 1.0;
            }

            // reference standard: mix
            if *solref1.add(0) > 0.0 && *solref2.add(0) > 0.0 {
                for i in 0..MJ_NREF {
                    *solref.add(i) = mix * *solref1.add(i) + (1.0 - mix) * *solref2.add(i);
                }
            }
            // reference direct: min
            else {
                for i in 0..MJ_NREF {
                    *solref.add(i) = crate::engine::engine_util_misc::mju_min(*solref1.add(i), *solref2.add(i));
                }
            }

            // impedance: mix
            for i in 0..MJ_NIMP {
                *solimp.add(i) = mix * *solimp1.add(i) + (1.0 - mix) * *solimp2.add(i);
            }

            // friction: max
            for i in 0..3_usize {
                fri[i] = crate::engine::engine_util_misc::mju_max(*friction1.add(i), *friction2.add(i));
            }
        }

        // unpack 5D friction
        *friction.add(0) = fri[0];
        *friction.add(1) = fri[0];
        *friction.add(2) = fri[1];
        *friction.add(3) = fri[2];
        *friction.add(4) = fri[2];

        // SHOULD NOT OCCUR
        if *condim > 6 || *condim < 1 {
            crate::engine::engine_util_errmem::mju_error(
                b"Invalid condim value\0".as_ptr() as *const i8,
            );
        }
    }
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
    use crate::engine::engine_util_blas::{mju_add3, mju_scl3, mju_normalize3};
    use crate::engine::engine_util_spatial::{mju_quat_z2vec, mju_quat2mat};
    // SAFETY: m, d valid; pos[3], mat[9], size[2] writable
    unsafe {
        // get vertex positions
        let vertadr = *(*m).flex_vertadr.add(f as usize);
        let v1 = (*d).flexvert_xpos.add(3 * (vertadr + vid[0]) as usize);
        let v2 = (*d).flexvert_xpos.add(3 * (vertadr + vid[1]) as usize);

        // construct capsule from vertices
        let mut dif: [f64; 3] = [
            *v1.add(0) - *v2.add(0),
            *v1.add(1) - *v2.add(1),
            *v1.add(2) - *v2.add(2),
        ];
        *size.add(0) = *(*m).flex_radius.add(f as usize);
        *size.add(1) = 0.5 * mju_normalize3(dif.as_mut_ptr());

        mju_add3(pos, v1, v2);
        mju_scl3(pos, pos as *const f64, 0.5);

        let mut quat: [f64; 4] = [0.0; 4];
        mju_quat_z2vec(quat.as_mut_ptr(), dif.as_ptr());
        mju_quat2mat(mat, quat.as_ptr());
    }
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
pub fn plane_vertex(con: *mut mjPreContact, pos: *const f64, rad: f64, t0: i32, t1: i32, t2: i32, v: i32) -> i32 {
    use crate::engine::engine_util_blas::{mju_sub3, mju_normalize3, mju_dot3, mju_scl3, mju_zero3, mju_add_scl3};
    use crate::engine::engine_util_spatial::mju_cross;
    // SAFETY: pos points to vertex array, con is valid mjPreContact
    unsafe {
        // make t0 the origin
        let mut e1: [f64; 3] = [0.0; 3];
        let mut e2: [f64; 3] = [0.0; 3];
        let mut ev: [f64; 3] = [0.0; 3];
        mju_sub3(e1.as_mut_ptr(), pos.add(3 * t1 as usize), pos.add(3 * t0 as usize));
        mju_sub3(e2.as_mut_ptr(), pos.add(3 * t2 as usize), pos.add(3 * t0 as usize));
        mju_sub3(ev.as_mut_ptr(), pos.add(3 * v as usize), pos.add(3 * t0 as usize));

        // compute normal
        let mut nrm: [f64; 3] = [0.0; 3];
        mju_cross(nrm.as_mut_ptr(), e1.as_ptr(), e2.as_ptr());
        mju_normalize3(nrm.as_mut_ptr());

        // project, check distance
        let dst: f64 = mju_dot3(ev.as_ptr(), nrm.as_ptr());
        if dst <= -2.0 * rad {
            return 0;
        }

        // construct contact
        (*con).dist = -dst - 2.0 * rad;
        mju_scl3((*con).normal.as_mut_ptr(), nrm.as_ptr(), -1.0);
        mju_zero3((*con).tangent.as_mut_ptr());
        mju_add_scl3((*con).pos.as_mut_ptr(), pos.add(3 * v as usize), nrm.as_ptr(), -0.5 * dst);
        1
    }
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
    extern "C" {
        fn mj_collideOBB(aabb1: *const f64, aabb2: *const f64, xpos1: *const f64, xmat1: *const f64, xpos2: *const f64, xmat2: *const f64, margin: f64, product: *mut f64, offset: *mut f64, initialize: *mut mjtBool) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
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

