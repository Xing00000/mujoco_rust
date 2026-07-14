//! Port of: engine/engine_collision_primitive.c
//! IR hash: 47ee20b2bff3660e
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
    todo!() // mjraw_PlaneSphere
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
    todo!() // mjraw_SphereSphere
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
    // SAFETY: p1, p2, p3 each point to at least 2 f64 elements (caller contract)
    unsafe {
        let val = (*p1.add(0) - *p3.add(0)) * (*p2.add(1) - *p3.add(1))
                - (*p2.add(0) - *p3.add(0)) * (*p1.add(1) - *p3.add(1));
        if val > 0.0 { 1.0 } else if val < 0.0 { -1.0 } else { 0.0 }
    }
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
    use crate::engine::engine_util_blas::{mju_dot, mju_add_scl};
    use crate::engine::engine_util_misc::mju_max;

    const MJ_MINVAL: f64 = 1E-15;

    // SAFETY: res, p, u, v each point to at least 2 f64 (caller contract)
    unsafe {
        // make u the origin
        let uv: [f64; 2] = [*v.add(0) - *u.add(0), *v.add(1) - *u.add(1)];
        let up: [f64; 2] = [*p.add(0) - *u.add(0), *p.add(1) - *u.add(1)];

        // project: find a s.t. uv is orthogonal to (up-a*uv)
        let a = mju_dot(uv.as_ptr(), up.as_ptr(), 2)
              / mju_max(MJ_MINVAL, mju_dot(uv.as_ptr(), uv.as_ptr(), 2));

        // find nearest point to p, clamp to u or v if a is not in (0,1)
        if a <= 0.0 {
            *res.add(0) = *u.add(0);
            *res.add(1) = *u.add(1);
        } else if a >= 1.0 {
            *res.add(0) = *v.add(0);
            *res.add(1) = *v.add(1);
        } else {
            mju_add_scl(res, u, uv.as_ptr(), a, 2);
        }

        // compute distance
        f64::sqrt((*res.add(0) - *p.add(0)) * (*res.add(0) - *p.add(0))
                + (*res.add(1) - *p.add(1)) * (*res.add(1) - *p.add(1)))
    }
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
    todo!() // mjraw_SphereCapsule
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
    todo!() // mjraw_CapsuleCapsule
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
    todo!() // mjraw_CapsuleBox
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
    todo!() // mjraw_SphereTriangle
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
    todo!() // mjraw_BoxTriangle
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
    todo!() // mjraw_CapsuleTriangle
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
    todo!() // mjc_PlaneSphere
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
    todo!() // mjc_PlaneCapsule
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
    todo!() // mjc_PlaneCylinder
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
    todo!() // mjc_PlaneBox
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
    todo!() // mjc_SphereSphere
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
    todo!() // mjc_SphereCapsule
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
    todo!() // mjc_SphereCylinder
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
    todo!() // mjc_CapsuleCapsule
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
    todo!() // mjc_CapsuleBox
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
    todo!() // mjc_SphereBox
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
    todo!() // mjc_BoxBox
}

