//! Port of: engine/engine_inline.h
//! IR hash: b156fa1eb890654a
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


/// C: mji_crossMotion (engine/engine_inline.h:429)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mji_cross_motion(res: *mut f64, vel: *const f64, v: *const f64) {
    // Bitexact translation of mji_crossMotion from engine_inline.h:429
    // Uses fnmsub/fmadd semantics to match C compiler codegen for NaN handling:
    // C: -a*b + c*d  compiles to: fnmsub(a,b,0) + fmadd(c,d,0) on ARM64
    // The key insight: fnmsub computes -(a*b) as a single operation, which
    // propagates NaN differently than Rust's (-a)*b (fneg + fmul).
    // To achieve bitexact match: use 0.0 - (a*b) instead of (-a)*b
    // This compiles to fsub(0, fmul(a,b)) which matches the NaN propagation.
    
    // res[0] = -vel[2]*v[1] + vel[1]*v[2];
    *res.add(0) = (0.0 - *vel.add(2) * *v.add(1)) + (*vel.add(1) * *v.add(2));
    // res[1] = vel[2]*v[0] - vel[0]*v[2];
    *res.add(1) = (*vel.add(2) * *v.add(0)) - (*vel.add(0) * *v.add(2));
    // res[2] = -vel[1]*v[0] + vel[0]*v[1];
    *res.add(2) = (0.0 - *vel.add(1) * *v.add(0)) + (*vel.add(0) * *v.add(1));
    // res[3] = -vel[2]*v[4] + vel[1]*v[5];
    *res.add(3) = (0.0 - *vel.add(2) * *v.add(4)) + (*vel.add(1) * *v.add(5));
    // res[4] = vel[2]*v[3] - vel[0]*v[5];
    *res.add(4) = (*vel.add(2) * *v.add(3)) - (*vel.add(0) * *v.add(5));
    // res[5] = -vel[1]*v[3] + vel[0]*v[4];
    *res.add(5) = (0.0 - *vel.add(1) * *v.add(3)) + (*vel.add(0) * *v.add(4));
    // res[3] += -vel[5]*v[1] + vel[4]*v[2];
    *res.add(3) = *res.add(3) + ((0.0 - *vel.add(5) * *v.add(1)) + (*vel.add(4) * *v.add(2)));
    // res[4] += vel[5]*v[0] - vel[3]*v[2];
    *res.add(4) = *res.add(4) + ((*vel.add(5) * *v.add(0)) - (*vel.add(3) * *v.add(2)));
    // res[5] += -vel[4]*v[0] + vel[3]*v[1];
    *res.add(5) = *res.add(5) + ((0.0 - *vel.add(4) * *v.add(0)) + (*vel.add(3) * *v.add(1)));
}

#[doc(hidden)]
#[doc = "c2rust-bitexact: generated function alias"]
pub use self::mji_cross_motion as mji_crossMotion;
