//! Port of: engine/engine_collision_box.c
//! IR hash: adc2f24e872d94f7
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_clampVec (engine/engine_collision_box.c:23)
/// Calls: mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_clamp_vec(vec: *mut f64, limit: *const f64, n: i32) {
    use crate::engine::engine_util_misc::{mju_max, mju_min};
    // SAFETY: caller guarantees vec has at least n elements, limit has at least 2*n elements
    unsafe {
        for i in 0..n as usize {
            *vec.add(i) = mju_max(*limit.add(2 * i), mju_min(*limit.add(2 * i + 1), *vec.add(i)));
        }
    }
}

/// C: mjraw_SphereBox (engine/engine_collision_box.c:34)
/// Calls: mji_add3, mji_addToScl3, mji_copy3, mji_mulMatTVec3, mji_mulMatVec3, mji_sub3, mji_zero3, mju_clampVec, mju_normalize3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_box(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    const MJ_MINVAL: f64 = 1E-15;
    // SAFETY: caller guarantees all pointers are valid and arrays properly sized
    unsafe {
        let mut tmp: [f64; 3] = [0.0; 3];
        let mut center: [f64; 3] = [0.0; 3];
        let mut clamped: [f64; 3] = [0.0; 3];
        let mut deepest: [f64; 3] = [0.0; 3];
        let mut pos: [f64; 3] = [0.0; 3];

        crate::engine::engine_inline::mji_sub3(tmp.as_mut_ptr(), pos1, pos2);
        crate::engine::engine_inline::mji_mul_mat_t_vec3(center.as_mut_ptr(), mat2, tmp.as_ptr());

        crate::engine::engine_inline::mji_copy3(clamped.as_mut_ptr(), center.as_ptr());
        mju_clamp_vec(clamped.as_mut_ptr(), size2, 3);

        crate::engine::engine_inline::mji_copy3(deepest.as_mut_ptr(), center.as_ptr());
        crate::engine::engine_inline::mji_sub3(tmp.as_mut_ptr(), clamped.as_ptr(), center.as_ptr());
        let mut dist = crate::engine::engine_util_blas::mju_normalize3(tmp.as_mut_ptr());

        if dist - *size1.add(0) > margin {
            return 0;
        }

        // sphere center inside box
        if dist <= MJ_MINVAL {
            let mut closest = (*size2.add(0) + *size2.add(1) + *size2.add(2)) * 2.0;
            let mut k: usize = 0;

            for i in 0..6usize {
                let sign: f64 = if i % 2 != 0 { 1.0 } else { -1.0 };
                let val = (sign * *size2.add(i / 2) - center[i / 2]).abs();
                if closest > val {
                    closest = val;
                    k = i;
                }
            }

            let mut nearest: [f64; 3] = [0.0; 3];
            nearest[k / 2] = if k % 2 != 0 { -1.0 } else { 1.0 };

            crate::engine::engine_inline::mji_copy3(pos.as_mut_ptr(), center.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(pos.as_mut_ptr(), nearest.as_ptr(), (*size1.add(0) - closest) / 2.0);
            crate::engine::engine_inline::mji_mul_mat_vec3((*con).normal.as_mut_ptr(), mat2, nearest.as_ptr());
            dist = -closest;
        } else {
            crate::engine::engine_inline::mji_add_to_scl3(deepest.as_mut_ptr(), tmp.as_ptr(), *size1.add(0));
            crate::engine::engine_util_blas::mju_zero3(pos.as_mut_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(pos.as_mut_ptr(), clamped.as_ptr(), 0.5);
            crate::engine::engine_inline::mji_add_to_scl3(pos.as_mut_ptr(), deepest.as_ptr(), 0.5);
            crate::engine::engine_inline::mji_mul_mat_vec3((*con).normal.as_mut_ptr(), mat2, tmp.as_ptr());
        }

        crate::engine::engine_inline::mji_mul_mat_vec3(tmp.as_mut_ptr(), mat2, pos.as_ptr());
        crate::engine::engine_inline::mji_add3((*con).pos.as_mut_ptr(), tmp.as_ptr(), pos2);
        (*con).dist = dist - *size1.add(0);
        crate::engine::engine_inline::mji_zero3((*con).tangent.as_mut_ptr());
        1
    }
}

/// C: _boxbox (engine/engine_collision_box.c:605)
/// Calls: mji_add3, mji_addTo3, mji_addToScl3, mji_copy3, mji_mulMatTVec3, mji_mulMatVec3, mji_scl3, mji_sub3, mji_zero3, mju_copy3, mju_dot3, mju_mulMatMatT3, mju_mulMatTMat3, mju_normalize3, mju_scl3, mju_transpose, mju_zero, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn boxbox(M: *const mjModel, D: *const mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // _boxbox
}

