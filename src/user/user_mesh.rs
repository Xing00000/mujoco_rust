//! Port of: user/user_mesh.cc
//! IR hash: 9f495ccbb9edeaab
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


pub unsafe fn __c2rust_mj_c_mesh_copy_polygon_normals_6e7f39d317a2d97ae2443aa3f1d5c358461afa6c5c6d15f2c3708e4ca17b3650(self_ptr: *mut mjCMesh, arr: *mut f64) -> () { unsafe { extern "C" { fn c2rust_sequence_output_len_aed7883af03ab8e46782b0aa0a7e7a926439c25fe70e73782c5b69ba7e21d1dd(sequence: *const core::ffi::c_void) -> usize; fn c2rust_sequence_output_get_aed7883af03ab8e46782b0aa0a7e7a926439c25fe70e73782c5b69ba7e21d1dd(sequence: *const core::ffi::c_void, index: usize) -> f64; } let sequence = (self_ptr as *const u8).add(1920).cast(); let mut i = 0usize; while i < c2rust_sequence_output_len_aed7883af03ab8e46782b0aa0a7e7a926439c25fe70e73782c5b69ba7e21d1dd(sequence) { arr.add(i + 0).write(c2rust_sequence_output_get_aed7883af03ab8e46782b0aa0a7e7a926439c25fe70e73782c5b69ba7e21d1dd(sequence, i + 0)); arr.add(i + 1).write(c2rust_sequence_output_get_aed7883af03ab8e46782b0aa0a7e7a926439c25fe70e73782c5b69ba7e21d1dd(sequence, i + 1)); arr.add(i + 2).write(c2rust_sequence_output_get_aed7883af03ab8e46782b0aa0a7e7a926439c25fe70e73782c5b69ba7e21d1dd(sequence, i + 2)); i += 3; } } }
