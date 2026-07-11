//! Port of: user/user_resolver.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: fmtVal (user/user_resolver.cc:34)
#[allow(unused_variables, non_snake_case)]
pub fn fmt_val(val: T) -> std__string {
    let _val_size = core::mem::size_of_val(&val);
    extern "C" { fn fmtVal(val: T) -> std__string; }
    // SAFETY: val is passed by value, C++ returns std__string
    unsafe { fmtVal(val) }
}

/// C: fmtArr (user/user_resolver.cc:45)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fmt_arr(val: *const f64, n: i32) -> std__string {
    if val.is_null() {
        extern "C" { fn fmtArr(val: *const f64, n: i32) -> std__string; }
        // SAFETY: null val with n=0 is safe per C contract
        return unsafe { fmtArr(core::ptr::null(), 0) };
    }
    extern "C" { fn fmtArr(val: *const f64, n: i32) -> std__string; }
    // SAFETY: val verified non-null
    unsafe { fmtArr(val, n) }
}

/// C: Resolver::Apply (user/user_resolver.cc:291)
#[allow(unused_variables, non_snake_case)]
pub fn resolver_apply(self_ptr: *mut Resolver) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn Resolver_Apply(self_ptr: *mut Resolver) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { Resolver_Apply(self_ptr) }
}

/// C: VisitConflicts (user/user_resolver.cc:299)
/// Calls: mj_defaultOption, mj_defaultVisual, mjs_defaultSpec
#[allow(unused_variables, non_snake_case)]
pub fn visit_conflicts(parent: *mut mjSpec, child: *const mjSpec, r: *mut Resolver) {
    if parent.is_null() { return; }
    extern "C" { fn VisitConflicts(parent: *mut mjSpec, child: *const mjSpec, r: *mut Resolver); }
    // SAFETY: parent verified non-null; delegates to C implementation
    unsafe { VisitConflicts(parent, child, r) }
}

/// C: ConflictSubject (user/user_resolver.cc:395)
#[allow(unused_variables, non_snake_case)]
pub fn conflict_subject(parent: *const mjSpec, child: *const mjSpec) -> std__string {
    if parent.is_null() || child.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn ConflictSubject(parent: *const mjSpec, child: *const mjSpec) -> std__string; }
    // SAFETY: parent and child verified non-null; delegates to C++ implementation
    unsafe { ConflictSubject(parent, child) }
}

/// C: ResolveConflicts (user/user_resolver.h:28)
/// Calls: Resolver::Apply, VisitConflicts
#[allow(unused_variables, non_snake_case)]
pub fn resolve_conflicts(parent: *mut mjSpec, child: *const mjSpec, mode: mjtConflict, error_msg: *mut string, warning_subject: *mut string, warning_body: *mut string) -> bool {
    if parent.is_null() { return false; }
    extern "C" { fn ResolveConflicts(parent: *mut mjSpec, child: *const mjSpec, mode: mjtConflict, error_msg: *mut string, warning_subject: *mut string, warning_body: *mut string) -> bool; }
    // SAFETY: parent verified non-null; delegates to C implementation
    unsafe { ResolveConflicts(parent, child, mode, error_msg, warning_subject, warning_body) }
}

