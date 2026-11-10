//! Port of: engine/engine_collision_gjk.c
//! IR hash: 2994358b89a3d67a
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


/// C: replaceSimplex3 (engine/engine_collision_gjk.c:855)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_replace_simplex3_ad9c0b9d680ccdd4f5e31566e8b6f9a31a268a9397a43368ff972753a2f6f77b(pt: *mut Polytope, status: *mut mjCCDStatus, v1: i32, v2: i32, v3: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe {
const _: [(); 8] = [(); core::mem::size_of::<isize>()];
const _: [(); 8] = [(); core::mem::size_of::<*const u8>()];
const _: [(); 1] = [(); cfg!(target_endian = "little") as usize];
core::ptr::write_unaligned((status as *mut u8).add(3176).cast::<i32>(), 3i32);
{ let record_source = core::ptr::read_unaligned((pt as *const u8).add(0).cast::<*const u8>()); core::ptr::copy(record_source.offset((v1 as isize) * 80isize), (status as *mut u8).add(2856), 80usize); }
{ let record_source = core::ptr::read_unaligned((pt as *const u8).add(0).cast::<*const u8>()); core::ptr::copy(record_source.offset((v2 as isize) * 80isize), (status as *mut u8).add(2936), 80usize); }
{ let record_source = core::ptr::read_unaligned((pt as *const u8).add(0).cast::<*const u8>()); core::ptr::copy(record_source.offset((v3 as isize) * 80isize), (status as *mut u8).add(3016), 80usize); }
core::ptr::write_unaligned((pt as *mut u8).add(24).cast::<i32>(), 0i32);
core::ptr::write_unaligned((pt as *mut u8).add(8).cast::<i32>(), 0i32);
core::ptr::write_unaligned((pt as *mut u8).add(64).cast::<i32>(), 0i32);
}

}

#[doc(hidden)]
#[doc = "c2rust-bitexact: generated function alias"]
pub use self::__c2rust_replace_simplex3_ad9c0b9d680ccdd4f5e31566e8b6f9a31a268a9397a43368ff972753a2f6f77b as replaceSimplex3;
