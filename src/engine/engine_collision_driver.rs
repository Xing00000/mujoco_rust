//! Port of: engine/engine_collision_driver.c
//! IR hash: 3fb6da908ad9d71c
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
        if (bf as usize) < (*m).nbody as usize {
            if *(*m).body_contype.add(bf as usize) != 0 || *(*m).body_conaffinity.add(bf as usize) != 0 {
                1
            } else {
                0
            }
        } else {
            let f = bf as usize - (*m).nbody as usize;
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
        let nbody = (*m).nbody as usize;
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
    const MJNREF: i32 = 2;
    const MJNIMP: i32 = 5;

    // SAFETY: m, d are valid pointers; g is geom index, f is flex index (caller contract)
    unsafe {
        let mut con_buf = [0u8; std::mem::size_of::<mjContact>()];
        let con = &mut *(con_buf.as_mut_ptr() as *mut mjContact);
        let radius = *(*m).flex_radius.add(f as usize);
        let pos = (*d).geom_xpos.add(3 * g as usize);
        let mat = (*d).geom_xmat.add(9 * g as usize);
        let nrm: [f64; 3] = [*mat.add(2), *mat.add(5), *mat.add(8)];

        // prepare contact parameters
        let margin = crate::engine::engine_core_constraint::mj_assign_margin(
            m, *(*m).geom_margin.add(g as usize) + *(*m).flex_margin.add(f as usize));
        let mut condim: i32 = 0;
        let flex_vertnum = *(*m).flex_vertnum.add(f as usize);
        let gap = *(*m).geom_gap.add(g as usize) + *(*m).flex_gap.add(f as usize);
        let mut solref: [f64; 2] = [0.0; 2];
        let mut solimp: [f64; 5] = [0.0; 5];
        let mut friction: [f64; 5] = [0.0; 5];
        let solreffriction: [f64; 2] = [0.0; 2];
        mj_contact_param(m, &mut condim, solref.as_mut_ptr(), solimp.as_mut_ptr(),
                         friction.as_mut_ptr(), g, -1, -1, f);

        // collide all flex vertices with plane
        for i in 0..flex_vertnum {
            let v = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(f as usize) + i) as usize);

            // distance from plane to vertex
            let dif: [f64; 3] = [*v.add(0) - *pos.add(0), *v.add(1) - *pos.add(1), *v.add(2) - *pos.add(2)];
            let dist = crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), nrm.as_ptr());

            // no contact
            if dist > margin + gap + radius {
                continue;
            }

            // create contact
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
            mj_set_contact(m, con as *mut mjContact, condim, margin, solref.as_ptr(),
                          solreffriction.as_ptr(), solimp.as_ptr(), friction.as_ptr());

            // add to mjData, abort if too many
            if crate::engine::engine_core_constraint::mj_add_contact(m, d, con as *const mjContact) != 0 {
                return;
            }
        }
    }
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
    const MJ_MAX_CON_PAIR: i32 = 50;
    const MJ_MAX_VAL: f64 = 1e10;

    // SAFETY: d is a valid mjData pointer with contact array (caller contract).
    unsafe {
        let n = (*d).ncon - ncon_before;
        if n <= MJ_MAX_CON_PAIR {
            return;
        }

        let contacts: *mut crate::types::mjContact = (*d).contact.add(ncon_before as usize);

        crate::engine::engine_memory::mj_mark_stack(d);
        let selected: *mut u8 = crate::engine::engine_memory::mj_stack_alloc_byte(d, n as usize, 1) as *mut u8;
        let min_dist: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, n as usize);
        std::ptr::write_bytes(selected, 0, n as usize);

        for i in 0..n {
            *min_dist.add(i as usize) = MJ_MAX_VAL;
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

        while nselected < MJ_MAX_CON_PAIR && best >= 0 {
            *selected.add(best as usize) = 1;
            let bestpos: *const f64 = (*contacts.add(best as usize)).pos.as_ptr();

            let mut nextbest: i32 = -1;
            let mut nextbestdist: f64 = -1.0;
            for i in 0..n {
                if *selected.add(i as usize) != 0 {
                    continue;
                }

                let dx = (*contacts.add(i as usize)).pos[0] - *bestpos.add(0);
                let dy = (*contacts.add(i as usize)).pos[1] - *bestpos.add(1);
                let dz = (*contacts.add(i as usize)).pos[2] - *bestpos.add(2);
                let d2 = dx * dx + dy * dy + dz * dz;
                if d2 < *min_dist.add(i as usize) {
                    *min_dist.add(i as usize) = d2;
                }
                if *min_dist.add(i as usize) > nextbestdist {
                    nextbestdist = *min_dist.add(i as usize);
                    nextbest = i;
                }
            }

            if nselected < MJ_MAX_CON_PAIR - 1 {
                let temp = *contacts.add(nselected as usize);
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
    // mjcPair layout: { g1: i32, g2: i32, ipair: i32 } = 12 bytes, align 4
    const SIZEOF_MJCPAIR: usize = 12;
    const ALIGNOF_MJCPAIR: usize = 4;

    // SAFETY: m is a valid mjModel, d is a valid mjData (caller contract)
    unsafe {
        // allocate geom pair on the arena
        let pair = crate::engine::engine_memory::mj_arena_alloc_byte(
            d, SIZEOF_MJCPAIR, ALIGNOF_MJCPAIR) as *mut i32;
        if pair.is_null() {
            crate::engine::engine_util_errmem::mju_error(
                b"arena too small to allocate geom pair\0".as_ptr() as *const i8);
        }

        // swap if g1/g2 both valid and geom_type[g1] > geom_type[g2]
        if g1 >= 0 && g2 >= 0 && *(*m).geom_type.add(g1 as usize) > *(*m).geom_type.add(g2 as usize) {
            *pair.add(0) = g2;        // pair->g1
            *pair.add(1) = g1;        // pair->g2
        } else {
            *pair.add(0) = g1;        // pair->g1
            *pair.add(1) = g2;        // pair->g2
        }
        *pair.add(2) = ipair;         // pair->ipair
    }
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
            if let Some(filter_fn) = crate::engine::engine_callback::mjcb_contactfilter {
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
    const MJ_ENBL_OVERRIDE: i32 = 1 << 0;
    // SAFETY: caller guarantees m, d, frame valid; output pointers valid
    unsafe {
        let mut aamm: [f64; 6] = [0.0; 6];
        let override_margin = if ((*m).opt.enableflags & MJ_ENBL_OVERRIDE) != 0 {
            0.5 * (*m).opt.o_margin
        } else {
            0.0
        };

        // body
        if (bf as i64) < (*m).nbody {
            let body = bf as usize;
            let body_geomnum = *(*m).body_geomnum.add(body);

            for i in 0..body_geomnum as usize {
                let geom = *(*m).body_geomadr.add(body) as usize + i;
                let margin = if override_margin != 0.0 {
                    override_margin
                } else {
                    *(*m).geom_margin.add(geom) + *(*m).geom_gap.add(geom)
                };
                let mut _aamm: [f64; 6] = [0.0; 6];

                let aabb = (*m).geom_aabb.add(6 * geom);
                let size = (*m).geom_aabb.add(6 * geom + 3);
                let xpos = (*d).geom_xpos.add(3 * geom);
                let xmat = (*d).geom_xmat.add(9 * geom);

                // compute center in global coordinates
                let mut pos: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_mul_mat_vec3(pos.as_mut_ptr(), xmat, aabb);
                crate::engine::engine_util_blas::mju_add_to3(pos.as_mut_ptr(), xpos);

                let mut axis: [f64; 9] = [0.0; 9];
                crate::engine::engine_inline::mji_transpose3(axis.as_mut_ptr(), xmat);
                let r_half = *(*m).geom_rbound.add(geom);

                for j in 0..3usize {
                    let frame_j = frame.add(3 * j);
                    let aabb_cen = crate::engine::engine_util_blas::mju_dot3(pos.as_ptr(), frame_j);
                    let aabb_half =
                        (*size.add(0) * crate::engine::engine_util_blas::mju_dot3(axis.as_ptr().add(0), frame_j)).abs()
                        + (*size.add(1) * crate::engine::engine_util_blas::mju_dot3(axis.as_ptr().add(3), frame_j)).abs()
                        + (*size.add(2) * crate::engine::engine_util_blas::mju_dot3(axis.as_ptr().add(6), frame_j)).abs();
                    let r_cen = crate::engine::engine_util_blas::mju_dot3(xpos, frame_j);
                    _aamm[j + 0] = crate::engine::engine_util_misc::mju_max(r_cen - r_half, aabb_cen - aabb_half) - margin;
                    _aamm[j + 3] = crate::engine::engine_util_misc::mju_min(r_cen + r_half, aabb_cen + aabb_half) + margin;
                }

                // update body aamm
                if i == 0 {
                    crate::engine::engine_util_blas::mju_copy(aamm.as_mut_ptr(), _aamm.as_ptr(), 6);
                } else {
                    for j in 0..3usize {
                        aamm[j] = crate::engine::engine_util_misc::mju_min(aamm[j], _aamm[j]);
                        aamm[j + 3] = crate::engine::engine_util_misc::mju_max(aamm[j + 3], _aamm[j + 3]);
                    }
                }
            }
        }
        // flex
        else {
            let f = bf as usize - (*m).nbody as usize;
            let flex_vertnum = *(*m).flex_vertnum.add(f);
            let vbase = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(f) as usize);

            for i in 0..flex_vertnum as usize {
                let mut v: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_mul_mat_vec(
                    v.as_mut_ptr(), frame, vbase.add(3 * i), 3, 3);

                if i == 0 {
                    crate::engine::engine_util_blas::mju_copy3(aamm.as_mut_ptr(), v.as_ptr());
                    crate::engine::engine_util_blas::mju_copy3(aamm.as_mut_ptr().add(3), v.as_ptr());
                } else {
                    for j in 0..3usize {
                        aamm[j] = crate::engine::engine_util_misc::mju_min(aamm[j], v[j]);
                        aamm[j + 3] = crate::engine::engine_util_misc::mju_max(aamm[j + 3], v[j]);
                    }
                }
            }

            // correct for flex radius and margin
            let margin = if override_margin != 0.0 {
                override_margin
            } else {
                *(*m).flex_margin.add(f) + *(*m).flex_gap.add(f)
            };
            let bound = *(*m).flex_radius.add(f) + margin;
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
    const RUN_SIZE: i32 = 32;

    // SAFETY: arr and buf point to valid arrays of at least n mjtSAP elements (caller contract).
    unsafe {
        // SAPcmp: compare by value field
        #[inline(always)]
        unsafe fn sap_cmp(a: *const mjtSAP, b: *const mjtSAP) -> i32 {
            if (*a).value < (*b).value { -1 }
            else if (*a).value == (*b).value { 0 }
            else { 1 }
        }

        // insertion sort on sub-array [start, end)
        for start in (0..n).step_by(RUN_SIZE as usize) {
            let end = if start + RUN_SIZE < n { start + RUN_SIZE } else { n };
            for j in (start + 1)..end {
                let tmp = *arr.add(j as usize);
                let mut k = j - 1;
                while k >= start && sap_cmp(arr.add(k as usize), &tmp) > 0 {
                    *arr.add((k + 1) as usize) = *arr.add(k as usize);
                    k -= 1;
                }
                *arr.add((k + 1) as usize) = tmp;
            }
        }

        // merge passes
        let mut src: *mut mjtSAP = arr;
        let mut dest: *mut mjtSAP = buf;
        let mut len: i32 = RUN_SIZE;
        while len < n {
            let mut start = 0i32;
            while start < n {
                let mid = start + len;
                let end = if start + 2 * len < n { start + 2 * len } else { n };
                if mid < end {
                    // merge [start, mid) and [mid, end)
                    let mut i = start;
                    let mut j = mid;
                    let mut k = start;
                    while i < mid && j < end {
                        if sap_cmp(src.add(i as usize), src.add(j as usize)) <= 0 {
                            *dest.add(k as usize) = *src.add(i as usize);
                            i += 1;
                        } else {
                            *dest.add(k as usize) = *src.add(j as usize);
                            j += 1;
                        }
                        k += 1;
                    }
                    if i < mid {
                        std::ptr::copy_nonoverlapping(src.add(i as usize), dest.add(k as usize), (mid - i) as usize);
                    } else if j < end {
                        std::ptr::copy_nonoverlapping(src.add(j as usize), dest.add(k as usize), (end - j) as usize);
                    }
                } else {
                    std::ptr::copy_nonoverlapping(src.add(start as usize), dest.add(start as usize), (end - start) as usize);
                }
                start += 2 * len;
            }
            // swap src and dest
            let tmp = src;
            src = dest;
            dest = tmp;
            len *= 2;
        }

        // if result ended up in buf, copy back to arr
        if src != arr {
            std::ptr::copy_nonoverlapping(src, arr, n as usize);
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
    // SAFETY: d is valid for stack alloc. aamm has 6*n elements. pair has maxpair capacity.
    unsafe {
        // check inputs
        if n >= 0x10000 || axis_x < 0 || axis_x > 2 || maxpair < 1 {
            return -1;
        }

        // allocate sort buffer (mjtSAP = 8 bytes, align 4)
        crate::engine::engine_memory::mj_mark_stack(d);
        let sortbuf: *mut mjtSAP = crate::engine::engine_memory::mj_stack_alloc_byte(
            d, (2 * n as usize) * std::mem::size_of::<mjtSAP>(), std::mem::align_of::<mjtSAP>()) as *mut mjtSAP;
        let activebuf: *mut mjtSAP = crate::engine::engine_memory::mj_stack_alloc_byte(
            d, (2 * n as usize) * std::mem::size_of::<mjtSAP>(), std::mem::align_of::<mjtSAP>()) as *mut mjtSAP;

        // get AAMM pointers for primary "x" axis
        let x_min: *const f64 = aamm.add((n * (axis_x + 0)) as usize);
        let x_max: *const f64 = aamm.add((n * (axis_x + 3)) as usize);

        // init sortbuf with specified axis
        for i in 0..n {
            (*sortbuf.add((2 * i) as usize)).id_ismax = i;
            (*sortbuf.add((2 * i) as usize)).value = *x_min.add(i as usize) as f32;
            (*sortbuf.add((2 * i + 1) as usize)).id_ismax = i + 0x10000;
            (*sortbuf.add((2 * i + 1) as usize)).value = *x_max.add(i as usize) as f32;
        }

        // sort along specified axis
        let buf: *mut mjtSAP = crate::engine::engine_memory::mj_stack_alloc_byte(
            d, (2 * n as usize) * std::mem::size_of::<mjtSAP>(), std::mem::align_of::<mjtSAP>()) as *mut mjtSAP;
        sa_psort(sortbuf, buf, 2 * n, std::ptr::null_mut());

        // define the other two axes
        let (axis_y, axis_z) = if axis_x == 0 {
            (1i32, 2i32)
        } else if axis_x == 1 {
            (0i32, 2i32)
        } else {
            (0i32, 1i32)
        };

        // get AAMM pointers to secondary "y, z" axes
        let y_min: *const f64 = aamm.add((n * (axis_y + 0)) as usize);
        let y_max: *const f64 = aamm.add((n * (axis_y + 3)) as usize);
        let z_min: *const f64 = aamm.add((n * (axis_z + 0)) as usize);
        let z_max: *const f64 = aamm.add((n * (axis_z + 3)) as usize);

        // sweep and prune
        let mut cnt: i32 = 0; // size of active list
        let mut npair: i32 = 0; // number of pairs added
        for i in 0..(2 * n) {
            // min value: collide with all in list, add
            if ((*sortbuf.add(i as usize)).id_ismax & 0x10000) == 0 {
                for j in 0..cnt {
                    let id1 = (*activebuf.add(j as usize)).id_ismax;
                    let id2 = (*sortbuf.add(i as usize)).id_ismax;

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
                        crate::engine::engine_memory::mj_free_stack(d);
                        return maxpair;
                    }
                }

                // add to list
                *activebuf.add(cnt as usize) = *sortbuf.add(i as usize);
                cnt += 1;
            }
            // max value: remove corresponding min value from list
            else {
                let toremove = (*sortbuf.add(i as usize)).id_ismax & 0xFFFF;
                for j in 0..cnt {
                    if (*activebuf.add(j as usize)).id_ismax == toremove {
                        if j < cnt - 1 {
                            std::ptr::copy(
                                activebuf.add((j + 1) as usize),
                                activebuf.add(j as usize),
                                (cnt - 1 - j) as usize);
                        }
                        cnt -= 1;
                        break;
                    }
                }
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
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
    const MJNREF: i32 = 2;
    const MJNIMP: i32 = 5;
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m is valid mjModel, all out pointers are valid (caller contract)
    unsafe {
        let mut fri: [f64; 3] = [0.0; 3];

        // get parameters from geom1 or flex1
        let priority1 = if f1 < 0 { *(*m).geom_priority.add(g1 as usize) } else { *(*m).flex_priority.add(f1 as usize) };
        let condim1 = if f1 < 0 { *(*m).geom_condim.add(g1 as usize) } else { *(*m).flex_condim.add(f1 as usize) };
        let solmix1 = if f1 < 0 { *(*m).geom_solmix.add(g1 as usize) } else { *(*m).flex_solmix.add(f1 as usize) };
        let solref1 = if f1 < 0 { (*m).geom_solref.add((g1 * MJNREF) as usize) } else { (*m).flex_solref.add((f1 * MJNREF) as usize) };
        let solimp1 = if f1 < 0 { (*m).geom_solimp.add((g1 * MJNIMP) as usize) } else { (*m).flex_solimp.add((f1 * MJNIMP) as usize) };
        let friction1 = if f1 < 0 { (*m).geom_friction.add((g1 * 3) as usize) } else { (*m).flex_friction.add((f1 * 3) as usize) };

        // get parameters from geom2 or flex2
        let priority2 = if f2 < 0 { *(*m).geom_priority.add(g2 as usize) } else { *(*m).flex_priority.add(f2 as usize) };
        let condim2 = if f2 < 0 { *(*m).geom_condim.add(g2 as usize) } else { *(*m).flex_condim.add(f2 as usize) };
        let solmix2 = if f2 < 0 { *(*m).geom_solmix.add(g2 as usize) } else { *(*m).flex_solmix.add(f2 as usize) };
        let solref2 = if f2 < 0 { (*m).geom_solref.add((g2 * MJNREF) as usize) } else { (*m).flex_solref.add((f2 * MJNREF) as usize) };
        let solimp2 = if f2 < 0 { (*m).geom_solimp.add((g2 * MJNIMP) as usize) } else { (*m).flex_solimp.add((f2 * MJNIMP) as usize) };
        let friction2 = if f2 < 0 { (*m).geom_friction.add((g2 * 3) as usize) } else { (*m).flex_friction.add((f2 * 3) as usize) };

        // different priority: copy from higher
        if priority1 > priority2 {
            *condim = condim1;
            crate::engine::engine_util_blas::mju_copy(solref, solref1, MJNREF);
            crate::engine::engine_util_blas::mju_copy(solimp, solimp1, MJNIMP);
            crate::engine::engine_util_blas::mju_copy(fri.as_mut_ptr(), friction1, 3);
        } else if priority1 < priority2 {
            *condim = condim2;
            crate::engine::engine_util_blas::mju_copy(solref, solref2, MJNREF);
            crate::engine::engine_util_blas::mju_copy(solimp, solimp2, MJNIMP);
            crate::engine::engine_util_blas::mju_copy(fri.as_mut_ptr(), friction2, 3);
        }
        // same priority
        else {
            // condim: max
            *condim = if condim1 > condim2 { condim1 } else { condim2 };

            // compute solver mix factor
            let mix: f64;
            if solmix1 >= MJMINVAL && solmix2 >= MJMINVAL {
                mix = solmix1 / (solmix1 + solmix2);
            } else if solmix1 < MJMINVAL && solmix2 < MJMINVAL {
                mix = 0.5;
            } else if solmix1 < MJMINVAL {
                mix = 0.0;
            } else {
                mix = 1.0;
            }

            // reference standard: mix
            if *solref1.add(0) > 0.0 && *solref2.add(0) > 0.0 {
                for i in 0..MJNREF {
                    *solref.add(i as usize) = mix * *solref1.add(i as usize) + (1.0 - mix) * *solref2.add(i as usize);
                }
            }
            // reference direct: min
            else {
                for i in 0..MJNREF {
                    let v1 = *solref1.add(i as usize);
                    let v2 = *solref2.add(i as usize);
                    *solref.add(i as usize) = if v1 < v2 { v1 } else { v2 };
                }
            }

            // impedance: mix
            for i in 0..MJNIMP {
                *solimp.add(i as usize) = mix * *solimp1.add(i as usize) + (1.0 - mix) * *solimp2.add(i as usize);
            }

            // friction: max
            for i in 0..3_i32 {
                let v1 = *friction1.add(i as usize);
                let v2 = *friction2.add(i as usize);
                fri[i as usize] = if v1 > v2 { v1 } else { v2 };
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
                b"Invalid condim value: %d\0".as_ptr() as *const i8);
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
    // SAFETY: caller guarantees m, con are valid; array pointers are valid
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
pub fn mj_make_capsule(m: *const mjModel, d: *mut mjData, f: i32, vid: *const i32, pos: *mut f64, mat: *mut f64, size: *mut f64) {
    // SAFETY: caller guarantees m, d valid; vid points to [2] array; pos[3], mat[9], size[2]
    unsafe {
        // get vertex positions
        let v1 = (*d).flexvert_xpos.add(
            3 * (*(*m).flex_vertadr.add(f as usize) + *vid.add(0)) as usize
        );
        let v2 = (*d).flexvert_xpos.add(
            3 * (*(*m).flex_vertadr.add(f as usize) + *vid.add(1)) as usize
        );

        // construct capsule from vertices
        let mut dif: [f64; 3] = [
            *v1.add(0) - *v2.add(0),
            *v1.add(1) - *v2.add(1),
            *v1.add(2) - *v2.add(2),
        ];
        *size.add(0) = *(*m).flex_radius.add(f as usize);
        *size.add(1) = 0.5 * crate::engine::engine_util_blas::mju_normalize3(dif.as_mut_ptr());

        crate::engine::engine_util_blas::mju_add3(pos, v1, v2);
        crate::engine::engine_util_blas::mju_scl3(pos, pos, 0.5);

        let mut quat: [f64; 4] = [0.0; 4];
        crate::engine::engine_util_spatial::mju_quat_z2vec(quat.as_mut_ptr(), dif.as_ptr());
        crate::engine::engine_util_spatial::mju_quat2mat(mat, quat.as_ptr());
    }
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
pub fn mj_max_contact(m: *const mjModel, g1: i32, g2: i32, has_margin: i32) -> i32 {
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

    // SAFETY: m is valid mjModel pointer; g1, g2 are valid geom indices (caller contract)
    unsafe {
        let type1 = *(*m).geom_type.add(g1 as usize);
        let type2 = *(*m).geom_type.add(g2 as usize);

        if type1 == mjGEOM_SDF || type2 == mjGEOM_SDF {
            return (*m).opt.sdf_initpoints;
        }

        if type1 == mjGEOM_HFIELD || type2 == mjGEOM_HFIELD {
            let t = if type1 == mjGEOM_HFIELD { type2 } else { type1 };
            return if t != mjGEOM_PLANE && t != mjGEOM_HFIELD { mjMAXCONPAIR } else { 0 };
        }

        if type1 == mjGEOM_SPHERE || type1 == mjGEOM_ELLIPSOID
            || type2 == mjGEOM_SPHERE || type2 == mjGEOM_ELLIPSOID
        {
            return 1;
        }

        if type1 == mjGEOM_BOX && type2 == mjGEOM_BOX {
            return 8;
        }

        if type1 == mjGEOM_CAPSULE && type2 == mjGEOM_CAPSULE {
            return 2;
        }

        if (type1 == mjGEOM_CAPSULE && type2 == mjGEOM_BOX)
            || (type1 == mjGEOM_BOX && type2 == mjGEOM_CAPSULE)
        {
            return 4;
        }

        if type1 == mjGEOM_PLANE || type2 == mjGEOM_PLANE {
            let t = if type1 == mjGEOM_PLANE { type2 } else { type1 };
            return match t {
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
            return if is_multiccd { 5 } else { 1 };
        }

        // check margin from model
        let mut hm = has_margin;
        if hm < 0 {
            hm = 0;
            if ((*m).opt.enableflags & mjENBL_OVERRIDE) != 0 {
                hm = ((*m).opt.o_margin > 0.0) as i32;
            } else {
                let npair = (*m).npair as i32;
                let mut ipair: i32 = -1;
                let mut k: i32 = 0;
                while k < npair {
                    if (*(*m).pair_geom1.add(k as usize) == g1
                        && *(*m).pair_geom2.add(k as usize) == g2)
                        || (*(*m).pair_geom1.add(k as usize) == g2
                            && *(*m).pair_geom2.add(k as usize) == g1)
                    {
                        ipair = k;
                        break;
                    }
                    k += 1;
                }

                if ipair > -1 {
                    hm = (*(*m).pair_margin.add(ipair as usize) > 0.0) as i32;
                } else {
                    hm = (*(*m).geom_margin.add(g1 as usize) > 0.0
                        || *(*m).geom_margin.add(g2 as usize) > 0.0) as i32;
                }
            }
        }

        // 4 contacts for mesh-mesh or mesh-box without margins, 5 with margins
        if hm != 0 { 5 } else { 4 }
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
pub fn mj_collide_obb(aabb1: *const f64, aabb2: *const f64, xpos1: *const f64, xmat1: *const f64, xpos2: *const f64, xmat2: *const f64, margin: f64, product: *mut f64, offset: *mut f64, initialize: *mut bool) -> i32 {
    const MJ_MAXVAL: f64 = 1E10;
    // SAFETY: caller guarantees all pointers valid with proper sizes
    unsafe {
        // get infinite dimensions (planes only)
        let inf1: [bool; 3] = [
            *aabb1.add(3) >= MJ_MAXVAL,
            *aabb1.add(4) >= MJ_MAXVAL,
            *aabb1.add(5) >= MJ_MAXVAL,
        ];
        let inf2: [bool; 3] = [
            *aabb2.add(3) >= MJ_MAXVAL,
            *aabb2.add(4) >= MJ_MAXVAL,
            *aabb2.add(5) >= MJ_MAXVAL,
        ];

        // if a bounding box is infinite, there must be a collision
        if (inf1[0] && inf1[1] && inf1[2]) || (inf2[0] && inf2[1] && inf2[2]) {
            return 1;
        }

        let aabb: [*const f64; 2] = [aabb1, aabb2];
        let xmat: [*const f64; 2] = [xmat1, xmat2];
        let xpos: [*const f64; 2] = [xpos1, xpos2];
        let mut xcenter: [[f64; 3]; 2] = [[0.0; 3]; 2];
        let mut normal: [[[f64; 3]; 3]; 2] = [[[0.0; 3]; 3]; 2];
        let mut proj: [f64; 2] = [0.0; 2];
        let mut radius: [f64; 2] = [0.0; 2];
        let infinite: [bool; 2] = [inf1[0] || inf1[1] || inf1[2], inf2[0] || inf2[1] || inf2[2]];

        // compute centers in local coordinates
        if product.is_null() {
            for i in 0..2usize {
                if !xmat[i].is_null() {
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(
                        xcenter[i].as_mut_ptr(), xmat[i], aabb[i]);
                } else {
                    crate::engine::engine_util_blas::mju_copy3(xcenter[i].as_mut_ptr(), aabb[i]);
                }
                if !xpos[i].is_null() {
                    crate::engine::engine_util_blas::mju_add_to3(xcenter[i].as_mut_ptr(), xpos[i]);
                }
            }
        }

        // compute normals in global coordinates
        for i in 0..2usize {
            for j in 0..3usize {
                for k in 0..3usize {
                    if !xmat[i].is_null() {
                        normal[i][j][k] = *xmat[i].add(3 * k + j);
                    } else {
                        normal[i][j][k] = if j == k { 1.0 } else { 0.0 };
                    }
                }
            }
        }

        // precompute dot products
        if !product.is_null() && !offset.is_null() && *initialize {
            for i in 0..2usize {
                for j in 0..2usize {
                    for k in 0..3usize {
                        for l in 0..3usize {
                            *product.add(18 * i + 9 * j + 3 * k + l) =
                                crate::engine::engine_util_blas::mju_dot3(
                                    normal[i][l].as_ptr(), normal[j][k].as_ptr());
                        }
                        *offset.add(6 * i + 3 * j + k) = if !xpos[i].is_null() {
                            crate::engine::engine_util_blas::mju_dot3(xpos[i], normal[j][k].as_ptr())
                        } else {
                            0.0
                        };
                    }
                }
            }
            *initialize = false;
        }

        // check intersections
        for j in 0..2usize {
            if infinite[1 - j] {
                continue;
            }
            for k in 0..3usize {
                for i in 0..2usize {
                    if product.is_null() {
                        proj[i] = crate::engine::engine_util_blas::mju_dot3(
                            xcenter[i].as_ptr(), normal[j][k].as_ptr());
                        radius[i] =
                            (*aabb[i].add(3) * crate::engine::engine_util_blas::mju_dot3(
                                normal[i][0].as_ptr(), normal[j][k].as_ptr())).abs()
                            + (*aabb[i].add(4) * crate::engine::engine_util_blas::mju_dot3(
                                normal[i][1].as_ptr(), normal[j][k].as_ptr())).abs()
                            + (*aabb[i].add(5) * crate::engine::engine_util_blas::mju_dot3(
                                normal[i][2].as_ptr(), normal[j][k].as_ptr())).abs();
                    } else {
                        let adr = 18 * i + 9 * j + 3 * k;
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
    const mjDSBL_FILTERPARENT: i32 = 1 << 10;
    const mjENBL_SLEEP: i32 = 1 << 4;
    const mjS_ASLEEP: i32 = 0;
    const mjS_AWAKE: i32 = 1;
    const mjOBJ_FLEX: u32 = 9;

    // SAFETY: m, d, bfpair are valid pointers (caller contract).
    unsafe {
        let mut npair: i32 = 0;
        let nbody = (*m).nbody as i32;
        let ngeom = (*m).ngeom as i32;
        let nvert = (*m).nflexvert as i32;
        let nflex = (*m).nflex as i32;
        let nbodyflex = nbody + nflex;
        let dsbl_filterparent = if ((*m).opt.disableflags & mjDSBL_FILTERPARENT) != 0 { 1 } else { 0 };
        let sleep_filter = (((*m).opt.enableflags & mjENBL_SLEEP) != 0)
            && ((*d).nbody_awake < nbody);
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
                    if can_collide(m, b2) == 0 {
                        continue;
                    }

                    let weld2 = *(*m).body_weldid.add(b2 as usize);
                    let parent_weld2 = *(*m).body_weldid.add(*(*m).body_parentid.add(weld2 as usize) as usize);
                    let asleep2 = if sleep_filter { if *(*d).body_awake.add(b2 as usize) == mjS_ASLEEP { 1 } else { 0 } } else { 0 };
                    if filter_body_pair(0, 0, 1, weld2, parent_weld2, asleep2, dsbl_filterparent) != 0 {
                        continue;
                    }

                    add_pair(m, b1, b2, &mut npair as *mut i32, bfpair, maxpair);
                }

                // add body:flex pairs, skip if flex asleep
                for f in 0..nflex {
                    if sleep_filter && crate::engine::engine_sleep::mj_sleep_state(
                        m, d as *const crate::types::mjData, mjOBJ_FLEX, f) == mjS_ASLEEP {
                        continue;
                    }
                    add_pair(m, b1, nbody + f, &mut npair as *mut i32, bfpair, maxpair);
                }
            }
        }

        // find center of non-world geoms and flex vertices; return if none
        let mut cnt: i32 = 0;
        crate::engine::engine_util_blas::mju_zero3(cen.as_mut_ptr());
        for i in 0..ngeom {
            if *(*m).geom_bodyid.add(i as usize) != 0 {
                crate::engine::engine_util_blas::mju_add_to3(cen.as_mut_ptr(), (*d).geom_xpos.add((3 * i) as usize));
                cnt += 1;
            }
        }
        for i in 0..nvert {
            if *(*m).flex_vertbodyid.add(i as usize) != 0 {
                crate::engine::engine_util_blas::mju_add_to3(cen.as_mut_ptr(), (*d).flexvert_xpos.add((3 * i) as usize));
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
                update_cov(cov.as_mut_ptr(), (*d).geom_xpos.add((3 * i) as usize), cen.as_ptr());
            }
        }
        for i in 0..nvert {
            if *(*m).flex_vertbodyid.add(i as usize) != 0 {
                update_cov(cov.as_mut_ptr(), (*d).flexvert_xpos.add((3 * i) as usize), cen.as_ptr());
            }
        }
        crate::engine::engine_util_blas::mju_scl(cov.as_mut_ptr(), cov.as_ptr(), 1.0 / cnt as f64, 9);

        // construct covariance-aligned 3D frame
        crate::engine::engine_util_solve::mju_eig3(
            eigval.as_mut_ptr(), frame.as_mut_ptr(), quat.as_mut_ptr(), cov.as_ptr());

        // allocate collidable bodyflex ids, construct list
        crate::engine::engine_memory::mj_mark_stack(d);
        let bfid: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, nbodyflex as usize);
        let mut ncollide: i32 = 0;
        for i in 1..nbodyflex {
            if can_collide(m, i) != 0 {
                *bfid.add(ncollide as usize) = i;
                ncollide += 1;
            }
        }

        if ncollide > 1 {
            // allocate and construct AAMMs for collidable only
            let aamm: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, (6 * ncollide) as usize);
            for i in 0..ncollide {
                make_aamm(m, d,
                    aamm.add((0 * ncollide + i) as usize),
                    aamm.add((1 * ncollide + i) as usize),
                    aamm.add((2 * ncollide + i) as usize),
                    aamm.add((3 * ncollide + i) as usize),
                    aamm.add((4 * ncollide + i) as usize),
                    aamm.add((5 * ncollide + i) as usize),
                    *bfid.add(i as usize), frame.as_ptr());
            }

            // call SAP
            let maxsappair = ncollide * (ncollide - 1) / 2;
            let sappair: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, maxsappair as usize);
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
                    let asleep1 = if sleep_filter { if *(*d).body_awake.add(bf1 as usize) == mjS_ASLEEP { 1 } else { 0 } } else { 0 };
                    let asleep2 = if sleep_filter { if *(*d).body_awake.add(bf2 as usize) == mjS_ASLEEP { 1 } else { 0 } } else { 0 };
                    let weld1 = *(*m).body_weldid.add(bf1 as usize);
                    let weld2 = *(*m).body_weldid.add(bf2 as usize);
                    let parent_weld1 = *(*m).body_weldid.add(*(*m).body_parentid.add(weld1 as usize) as usize);
                    let parent_weld2 = *(*m).body_weldid.add(*(*m).body_parentid.add(weld2 as usize) as usize);

                    if filter_body_pair(weld1, parent_weld1, asleep1,
                                       weld2, parent_weld2, asleep2,
                                       dsbl_filterparent) != 0 {
                        continue;
                    }
                }
                // flex pair: skip if neither side is dynamically awake
                else if sleep_filter {
                    let awake1 = if bf1 >= nbody {
                        if crate::engine::engine_sleep::mj_sleep_state(m, d as *const crate::types::mjData, mjOBJ_FLEX, bf1 - nbody) == mjS_AWAKE { 1 } else { 0 }
                    } else {
                        if *(*d).body_awake.add(bf1 as usize) == mjS_AWAKE && *(*m).body_treeid.add(bf1 as usize) >= 0 { 1 } else { 0 }
                    };
                    let awake2 = if bf2 >= nbody {
                        if crate::engine::engine_sleep::mj_sleep_state(m, d as *const crate::types::mjData, mjOBJ_FLEX, bf2 - nbody) == mjS_AWAKE { 1 } else { 0 }
                    } else {
                        if *(*d).body_awake.add(bf2 as usize) == mjS_AWAKE && *(*m).body_treeid.add(bf2 as usize) >= 0 { 1 } else { 0 }
                    };
                    if awake1 == 0 && awake2 == 0 {
                        continue;
                    }
                }

                // add bodyflex pair if there is room in buffer
                add_pair(m, bf1, bf2, &mut npair as *mut i32, bfpair, maxpair);
            }
        }

        // sort bodyflex pairs by signature
        if npair > 1 {
            let buf: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, npair as usize);
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

