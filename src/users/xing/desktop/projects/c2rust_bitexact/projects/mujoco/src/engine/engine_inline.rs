//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mji_transpose3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:200)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_transpose3(res: *mut f64, mat: *const f64) {
    todo!() // cxx-internal:engine_collision_driver.c.o:_mji_transpose3
}

/// C: mji__normalize4 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:229)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_normalize4(vec: *mut f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji__normalize4
}

/// C: mji_mulQuatAxis (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:298)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_quat_axis(res: *mut f64, quat: *const f64, axis: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulQuatAxis
}

/// C: mji_mat2Quat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:361)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji__normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mat2quat(quat: *mut f64, mat: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mat2Quat
}

/// C: mji_quatIntegrate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:401)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji__normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_quat_integrate(quat: *mut f64, vel: *const f64, scale: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_quatIntegrate
}

/// C: mji_crossForce (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:446)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross_force(res: *mut f64, vel: *const f64, f: *const f64) {
    todo!() // cxx-internal:engine_core_smooth.c.o:_mji_crossForce
}

/// C: mji_dot6 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:463)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_dot6(vec1: *const f64, vec2: *const f64) -> f64 {
    todo!() // cxx-internal:engine_core_smooth.c.o:_mji_dot6
}

