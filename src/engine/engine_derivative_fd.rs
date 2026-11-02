//! Port of: engine/engine_derivative_fd.c
//! IR hash: e0334873dc5acd04
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


/// C: inRange (engine/engine_derivative_fd.c:106)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub unsafe fn in_range(x1: f64, x2: f64, range: *const f64) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (((((x1 >= { const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = range; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x00])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) }) && (x1 <= { const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = range; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x01])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) })) && (x2 >= { const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = range; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x00])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) })) && (x2 <= { const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = range; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x01])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) }))) as i32 }
}

#[doc(hidden)]
#[doc = "c2rust-bitexact: generated function alias"]
pub use self::in_range as inRange;
