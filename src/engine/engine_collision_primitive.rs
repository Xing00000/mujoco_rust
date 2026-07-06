//! Port of: engine/engine_collision_primitive.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjraw_PlaneSphere (engine/engine_collision_primitive.c:28)
/// Calls: mji_add3, mji_scl3, mji_zero3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_plane_sphere(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * mut mjPreContact, margin : f64, pos1 : * const f64, mat1 : * const f64, size1 : * const f64, pos2 : * const f64, mat2 : * const f64, size2 : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjraw_SphereSphere (engine/engine_collision_primitive.c:262)
/// Calls: mji_addTo3, mji_cross, mji_scl3, mji_sub3, mji_zero3, mju_dot3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_sphere(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    // SAFETY: all pointers are valid; pos/size are f64[3], mat is f64[9], con is mjPreContact array
    unsafe {
        const MJ_MINVAL: f64 = 1e-15;

        // check bounding spheres (this is called from other functions)
        let dif: [f64; 3] = [
            *pos1.add(0) - *pos2.add(0),
            *pos1.add(1) - *pos2.add(1),
            *pos1.add(2) - *pos2.add(2),
        ];
        let cdist_sqr: f64 = crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), dif.as_ptr());
        let min_dist: f64 = margin + *size1.add(0) + *size2.add(0);
        if cdist_sqr > min_dist * min_dist {
            return 0;
        }

        // depth and normal
        (*con.add(0)).dist = cdist_sqr.sqrt() - *size1.add(0) - *size2.add(0);
        crate::engine::engine_inline::mji_sub3((*con.add(0)).normal.as_mut_ptr(), pos2, pos1);
        let len: f64 = crate::engine::engine_util_blas::mju_normalize3((*con.add(0)).normal.as_mut_ptr());

        // if centers are the same, norm = cross-product of z axes
        //  if z axes are parallel, norm = [1;0;0]
        if len < MJ_MINVAL {
            let axis1: [f64; 3] = [*mat1.add(2), *mat1.add(5), *mat1.add(8)];
            let axis2: [f64; 3] = [*mat2.add(2), *mat2.add(5), *mat2.add(8)];
            crate::engine::engine_inline::mji_cross((*con.add(0)).normal.as_mut_ptr(), axis1.as_ptr(), axis2.as_ptr());
            crate::engine::engine_util_blas::mju_normalize3((*con.add(0)).normal.as_mut_ptr());
        }

        // position
        crate::engine::engine_inline::mji_scl3(
            (*con.add(0)).pos.as_mut_ptr(),
            (*con.add(0)).normal.as_ptr(),
            *size1.add(0) + (*con.add(0)).dist / 2.0,
        );
        crate::engine::engine_inline::mji_add_to3((*con.add(0)).pos.as_mut_ptr(), pos1);

        // axis
        crate::engine::engine_inline::mji_zero3((*con.add(0)).tangent.as_mut_ptr());

        1
    }
}

/// C: areaSign (engine/engine_collision_primitive.c:534)
/// Calls: mju_sign
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn area_sign(p1: *const f64, p2: *const f64, p3: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (p1 : * const f64, p2 : * const f64, p3 : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: pointSegment (engine/engine_collision_primitive.c:540)
/// Calls: mju_addScl, mju_dot, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_segment(res: *mut f64, p: *const f64, u: *const f64, v: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, p : * const f64, u : * const f64, v : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: mjraw_SphereCapsule (engine/engine_collision_primitive.h:28)
/// Calls: mji_addTo3, mji_scl3, mjraw_SphereSphere, mju_clip, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_capsule(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * mut mjPreContact, margin : f64, pos1 : * const f64, mat1 : * const f64, size1 : * const f64, pos2 : * const f64, mat2 : * const f64, size2 : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjraw_CapsuleCapsule (engine/engine_collision_primitive.h:31)
/// Calls: mji_add3, mji_addTo3, mji_scl3, mji_sub3, mjraw_SphereSphere, mju_clip, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_capsule(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * mut mjPreContact, margin : f64, pos1 : * const f64, mat1 : * const f64, size1 : * const f64, pos2 : * const f64, mat2 : * const f64, size2 : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjraw_CapsuleBox (engine/engine_collision_primitive.h:34)
/// Calls: mji_addToScl3, mji_copy3, mji_mulMatTVec3, mji_scl3, mji_sub3, mji_subFrom3, mjraw_SphereBox, mju_addTo3, mju_copy3, mju_dot3, mju_mulMatVec3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_box(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * mut mjPreContact, margin : f64, pos1 : * const f64, mat1 : * const f64, size1 : * const f64, pos2 : * const f64, mat2 : * const f64, size2 : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjraw_SphereTriangle (engine/engine_collision_primitive.h:37)
/// Calls: areaSign, mji_addScl3, mji_addToScl3, mji_copy3, mji_cross, mji_scl3, mju_dot3, mju_normalize3, mju_zero3, pointSegment
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_triangle(con: *mut mjPreContact, margin: f64, s: *const f64, rs: f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * mut mjPreContact, margin : f64, s : * const f64, rs : f64, t1 : * const f64, t2 : * const f64, t3 : * const f64, rt : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjraw_BoxTriangle (engine/engine_collision_primitive.h:39)
/// Calls: mji_addScl3, mjraw_SphereTriangle, mju_addTo3, mju_mulMatTVec3, mju_mulMatVec3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_box_triangle(con: *mut mjPreContact, margin: f64, pos: *const f64, mat: *const f64, size: *const f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * mut mjPreContact, margin : f64, pos : * const f64, mat : * const f64, size : * const f64, t1 : * const f64, t2 : * const f64, t3 : * const f64, rt : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjraw_CapsuleTriangle (engine/engine_collision_primitive.h:42)
/// Calls: mji_add3, mji_addScl3, mji_addToScl3, mji_copy3, mjraw_SphereTriangle, mju_addScl3, mju_dot3, mju_normalize3, mju_scl3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_triangle(con: *mut mjPreContact, margin: f64, pos: *const f64, mat: *const f64, size: *const f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * mut mjPreContact, margin : f64, pos : * const f64, mat : * const f64, size : * const f64, t1 : * const f64, t2 : * const f64, t3 : * const f64, rt : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_PlaneSphere (engine/engine_collision_primitive.h:47)
/// Calls: mjraw_PlaneSphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_sphere(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_PlaneCapsule (engine/engine_collision_primitive.h:49)
/// Calls: mji_copy3, mjraw_PlaneSphere, mju_add3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_capsule(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_PlaneCylinder (engine/engine_collision_primitive.h:51)
/// Calls: mji_add3, mji_addTo3, mji_addToScl3, mji_copy3, mji_cross, mji_sub3, mji_subFrom3, mji_zero3, mju_dot3, mju_normalize3, mju_scl3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_cylinder(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_PlaneBox (engine/engine_collision_primitive.h:53)
/// Calls: mji_add3, mji_addTo3, mji_copy3, mji_scl3, mji_zero3, mju_dot3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_SphereSphere (engine/engine_collision_primitive.h:57)
/// Calls: mjraw_SphereSphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_sphere(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_SphereCapsule (engine/engine_collision_primitive.h:59)
/// Calls: mjraw_SphereCapsule
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_capsule(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_SphereCylinder (engine/engine_collision_primitive.h:61)
/// Calls: mji_addTo3, mji_scl3, mji_sub3, mjraw_PlaneSphere, mjraw_SphereSphere, mju_addScl3, mju_dot3, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_cylinder(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_CapsuleCapsule (engine/engine_collision_primitive.h:63)
/// Calls: mjraw_CapsuleCapsule
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_capsule(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_CapsuleBox (engine/engine_collision_primitive.h:67)
/// Calls: mjraw_CapsuleBox
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_SphereBox (engine/engine_collision_primitive.h:69)
/// Calls: mjraw_SphereBox
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_BoxBox (engine/engine_collision_primitive.h:71)
/// Calls: _boxbox, mju_outsideBox
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_box_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

