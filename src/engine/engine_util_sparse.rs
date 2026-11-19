//! Port of: engine/engine_util_sparse.h
//! IR hash: a9108fa8b619f70f
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


/// # Safety
/// When `n` is positive, both ranges must be live, initialized, and aligned for `n` `f64` values;
/// `res` must be writable, and there must be no conflicting concurrent access.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mju_add_to_scl_scl_99411cf4084c72e7ae1b44483979d0f3c38db64f75aa03d7d33b823c17da79ab(res: *mut f64, vec: *const f64, scl1: f64, scl2: f64, n: i32) -> () {
    unsafe {
        for i in 0..n { let dst = res.add(i as usize); dst.write(dst.read() * scl1 + vec.add(i as usize).read() * scl2); }
    }
}
