//! Port of: render/classic/render_util.c
//! IR hash: 01b2a55b6c0d1185
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


/// Computes the product of two column-major 4×4 matrices.
///
/// # Safety
/// The caller must provide live, initialized, writable arrays of 16 `f32` values for `res` and live, initialized arrays of 16 `f32` values for `A` and `B`.
/// Access is restricted when concurrent operations use these arrays to avoid data races.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mjr_mul_mat44_6eda8effdd13119322d73f275c49d7d366a5253c8fe901563061f9ba9810097a(res: *mut f32, A: *const f32, B: *const f32) -> () {
    unsafe {
        for row in 0usize..4 {
            for column in 0usize..4 {
                let slot = res.add(row + 4 * column);
                slot.write(0.0_f32);
                for inner in 0usize..4 {
                    let product = A.add(row + 4 * inner).read() * B.add(inner + 4 * column).read();
                    slot.write(slot.read() + product);
                }
            }
        }
    }
}
