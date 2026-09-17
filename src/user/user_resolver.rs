//! Port of: user/user_resolver.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: fmtVal (user/user_resolver.cc:34)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fmt_val__a8e42cde3de0b5f2(val: f64) -> std__string {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resolver.cc:__ZN6mujoco12_GLOBAL__N_16fmtValIdEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEET_
}

/// C: fmtVal (user/user_resolver.cc:34)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fmt_val__f366d4ab17bb1a43(val: f32) -> std__string {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resolver.cc:__ZN6mujoco12_GLOBAL__N_16fmtValIfEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEET_
}

/// C: fmtVal (user/user_resolver.cc:34)
#[allow(unused_variables, non_snake_case)]
pub fn fmt_val__b3f38e84a0e503a8(val: i32) -> std__string {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resolver.cc:__ZN6mujoco12_GLOBAL__N_16fmtValIiEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEET_
}

/// C: fmtVal (user/user_resolver.cc:34)
#[allow(unused_variables, non_snake_case)]
pub fn fmt_val__008fe32054a8d8ea(val: i64) -> std__string {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resolver.cc:__ZN6mujoco12_GLOBAL__N_16fmtValIxEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEET_
}

/// C: fmtArr (user/user_resolver.cc:45)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fmt_arr(val: *const f64, n: i32) -> std__string {
    todo!() // cxx-internal:user_resolver.cc.o:__ZN6mujoco12_GLOBAL__N_16fmtArrEPKdi
}

/// C: Resolver::Apply (user/user_resolver.cc:294)
#[allow(unused_variables, non_snake_case)]
pub fn resolver_apply(self_ptr: *mut mujoco__Resolver) -> bool {
    todo!() // cxx-internal:user_resolver.cc.o:__ZN6mujoco12_GLOBAL__N_18Resolver5ApplyEv
}

/// C: VisitConflicts (user/user_resolver.cc:302)
/// Calls: cxx:_mj_defaultOption, cxx:_mj_defaultVisual, cxx:_mjs_defaultSpec
#[allow(unused_variables, non_snake_case)]
pub fn visit_conflicts(parent: *mut mjSpec, child: *const mjSpec, r: *mut mujoco__Resolver) {
    todo!() // cxx-internal:user_resolver.cc.o:__ZN6mujoco12_GLOBAL__N_114VisitConflictsEP7mjSpec_PKS1_RNS0_8ResolverE
}

/// C: ConflictSubject (user/user_resolver.cc:391)
#[allow(unused_variables, non_snake_case)]
pub fn conflict_subject(parent: *const mjSpec, child: *const mjSpec) -> std__string {
    todo!() // cxx-internal:user_resolver.cc.o:__ZN6mujoco12_GLOBAL__N_115ConflictSubjectEPK7mjSpec_S3_
}

/// C: ResolveConflicts (user/user_resolver.cc:423)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resolver.cc:__ZN6mujoco12_GLOBAL__N_114VisitConflictsEP7mjSpec_PKS1_RNS0_8ResolverE, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resolver.cc:__ZN6mujoco12_GLOBAL__N_18Resolver5ApplyEv, cxx-internal:user_resolver.cc.o:__ZN6mujoco12_GLOBAL__N_115ConflictSubjectEPK7mjSpec_S3_
#[allow(unused_variables, non_snake_case)]
pub fn resolve_conflicts(parent: *mut mjSpec, child: *const mjSpec, mode: u32, error_msg: *mut std__string, warning_subject: *mut std__string, warning_body: *mut std__string) -> bool {
    todo!() // cxx:__ZN6mujoco16ResolveConflictsEP7mjSpec_PKS0_11mjtConflictPNSt3__112basic_stringIcNS5_11char_traitsIcEENS5_9allocatorIcEEEESC_SC_
}

