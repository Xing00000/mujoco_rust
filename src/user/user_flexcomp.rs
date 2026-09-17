//! Port of: user/user_flexcomp.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: comperr (user/user_flexcomp.cc:49)
/// Calls: cxx:_mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn comperr(error: *mut i8, msg: *const i8, error_sz: i32) -> bool {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_flexcomp.cc:__ZL7comperrPcPKci
}

/// C: ReadFromBuffer (user/user_flexcomp.cc:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn read_from_buffer__78af5eed4dacf891(dst: *mut f64, src: *const i8) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_flexcomp.cc:__ZL14ReadFromBufferIdEvPT_PKc
}

/// C: ReadFromBuffer (user/user_flexcomp.cc:56)
#[allow(unused_variables, non_snake_case)]
pub fn read_from_buffer__c1bff261cc5e91fc(dst: *mut i32, src: *const i8) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_flexcomp.cc:__ZL14ReadFromBufferIiEvPT_PKc
}

/// C: ReadFromBuffer (user/user_flexcomp.cc:56)
#[allow(unused_variables, non_snake_case)]
pub fn read_from_buffer__17f63774581abc69(dst: *mut u64, src: *const i8) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_flexcomp.cc:__ZL14ReadFromBufferImEvPT_PKc
}

/// C: ReadStrFromBuffer (user/user_flexcomp.cc:61)
#[allow(unused_variables, non_snake_case)]
pub fn read_str_from_buffer(dest: *mut i8, src: *const i8, maxlen: i32) {
    todo!() // cxx-internal:user_flexcomp.cc.o:__ZL17ReadStrFromBufferPcPKci
}

/// C: IsValidElementOrNodeHeader22 (user/user_flexcomp.cc:65)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_element_or_node_header22(line: *const std__string) -> bool {
    todo!() // cxx:__Z28IsValidElementOrNodeHeader22RKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
}

/// C: mjCFlexcomp::MarkEmptyCells (user/user_flexcomp.cc:101)
/// Calls: cxx:__ZN7mjCFlex16ComputeCellEmptyEPKdPKiiiiS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_mark_empty_cells(self_ptr: *mut mjCFlexcomp, flex: *mut mjCFlex, points: *const f64, npnt: i32, minmax: *const f64, nx: i32, ny: i32, nz: i32) {
    todo!() // cxx:__ZN11mjCFlexcomp14MarkEmptyCellsEP7mjCFlexPKdiS3_iii
}

/// C: mjCFlexcomp::Make (user/user_flexcomp.cc:143)
/// Calls: cxx:__Z15mjuu_trnVecPosePdPKdS1_S1_, cxx:__ZN11mjCFlexcomp10MakeSquareEPci, cxx:__ZN11mjCFlexcomp14MarkEmptyCellsEP7mjCFlexPKdiS3_iii, cxx:__ZN11mjCFlexcomp7MakeBoxEPciib, cxx:__ZN11mjCFlexcomp8MakeGMSHEP8mjCModelP12mjsCompiler_PciPK6mjVFS_, cxx:__ZN11mjCFlexcomp8MakeGridEPci, cxx:__ZN11mjCFlexcomp8MakeMeshEP8mjCModelP12mjsCompiler_PciPK6mjVFS_, cxx:__ZN6mjCDef4FlexEv, cxx:__ZN7mjCFlex12PointToLocalEv, cxx:__ZN8mjCModel7AddFlexEv, cxx:__ZNK8mjCModel7DefaultEv, cxx:_mjs_addBody, cxx:_mjs_addEquality, cxx:_mjs_addJoint, cxx:_mjs_appendString, cxx:_mjs_getName, cxx:_mjs_getString, cxx:_mjs_resolveOrientation, cxx:_mjs_setDefault, cxx:_mjs_setDouble, cxx:_mjs_setFloat, cxx:_mjs_setInt, cxx:_mjs_setName, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make(self_ptr: *mut mjCFlexcomp, body: *mut mjsBody, error: *mut i8, error_sz: i32, vfs: *const mjVFS) -> bool {
    todo!() // cxx:__ZN11mjCFlexcomp4MakeEP8mjsBody_PciPK6mjVFS_
}

/// C: mjCFlexcomp::GridID (user/user_flexcomp.cc:812)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_grid_id__b91e6c8c70470167(self_ptr: *mut mjCFlexcomp, ix: i32, iy: i32) -> i32 {
    todo!() // cxx:__ZN11mjCFlexcomp6GridIDEii
}

/// C: mjCFlexcomp::GridID (user/user_flexcomp.cc:815)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_grid_id__60499c1f75ae1274(self_ptr: *mut mjCFlexcomp, ix: i32, iy: i32, iz: i32) -> i32 {
    todo!() // cxx:__ZN11mjCFlexcomp6GridIDEiii
}

/// C: mjCFlexcomp::MakeGrid (user/user_flexcomp.cc:821)
/// Calls: cxx:__ZN6mjCDef4FlexEv, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_grid(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32) -> bool {
    todo!() // cxx:__ZN11mjCFlexcomp8MakeGridEPci
}

/// C: mjCFlexcomp::BoxID (user/user_flexcomp.cc:954)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_box_id(self_ptr: *mut mjCFlexcomp, ix: i32, iy: i32, iz: i32) -> i32 {
    todo!() // cxx:__ZN11mjCFlexcomp5BoxIDEiii
}

/// C: mjCFlexcomp::BoxProject (user/user_flexcomp.cc:999)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_box_project(self_ptr: *mut mjCFlexcomp, pos: *mut f64, ix: i32, iy: i32, iz: i32) {
    todo!() // cxx:__ZN11mjCFlexcomp10BoxProjectEPdiii
}

/// C: mjCFlexcomp::MakeSquare (user/user_flexcomp.cc:1037)
/// Calls: cxx:__ZN11mjCFlexcomp8MakeGridEPci
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_square(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32) -> bool {
    todo!() // cxx:__ZN11mjCFlexcomp10MakeSquareEPci
}

/// C: mat2lin (user/user_flexcomp.cc:1064)
#[allow(unused_variables, non_snake_case)]
pub fn mat2lin(ix: i32, iy: i32, iz: i32, count: *const i32) -> i32 {
    // SAFETY: count points to an array of at least 3 ints (caller contract)
    unsafe {
        ix * *count.add(1) * *count.add(2) + iy * *count.add(2) + iz
    }
}

/// C: mjCFlexcomp::MakeBox (user/user_flexcomp.cc:1070)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_flexcomp.cc:__ZL7mat2liniiiPKi, cxx:__ZN11mjCFlexcomp10BoxProjectEPdiii, cxx:__ZN11mjCFlexcomp5BoxIDEiii, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_box(self_ptr: *mut mjCFlexcomp, error: *mut i8, error_sz: i32, dim: i32, open: bool) -> bool {
    todo!() // cxx:__ZN11mjCFlexcomp7MakeBoxEPciib
}

/// C: mjCFlexcomp::MakeMesh (user/user_flexcomp.cc:1278)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:__ZN7mjCMesh16LoadFromResourceEP11mjResource_b, cxx:__ZNK7mjCMesh11HasTexcoordEv, cxx:__ZNK7mjCMesh12FaceTexcoordEv, cxx:__ZNK7mjCMesh4FaceEv, cxx:__ZNK7mjCMesh8TexcoordEv, cxx:_mjs_getString, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_mesh(self_ptr: *mut mjCFlexcomp, model: *mut mjCModel, compiler: *mut mjsCompiler, error: *mut i8, error_sz: i32, vfs: *const mjVFS) -> bool {
    todo!() // cxx:__ZN11mjCFlexcomp8MakeMeshEP8mjCModelP12mjsCompiler_PciPK6mjVFS_
}

/// C: findstring (user/user_flexcomp.cc:1367)
#[allow(unused_variables, non_snake_case)]
pub fn findstring(buffer: *const i8, buffer_sz: i32, str: *const i8) -> i32 {
    // SAFETY: buffer and str are valid C strings/buffers from mujoco
    unsafe {
        // manual strlen
        let mut len: i32 = 0;
        while *str.offset(len as isize) != 0 {
            len += 1;
        }

        // scan buffer
        let mut i: i32 = 0;
        while i < buffer_sz - len {
            // check for string at position i
            let mut found = true;
            let mut k: i32 = 0;
            while k < len {
                if *buffer.offset((i + k) as isize) != *str.offset(k as isize) {
                    found = false;
                    break;
                }
                k += 1;
            }

            // string found
            if found {
                return i;
            }
            i += 1;
        }

        // not found
        -1
    }
}

/// C: mjCFlexcomp::MakeGMSH (user/user_flexcomp.cc:1392)
/// Calls: cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN11mjCFlexcomp8LoadGMSHEP8mjCModelP11mjResource_, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:_mjs_getString, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_make_gmsh(self_ptr: *mut mjCFlexcomp, model: *mut mjCModel, compiler: *mut mjsCompiler, error: *mut i8, error_sz: i32, vfs: *const mjVFS) -> bool {
    todo!() // cxx:__ZN11mjCFlexcomp8MakeGMSHEP8mjCModelP12mjsCompiler_PciPK6mjVFS_
}

/// C: mjCFlexcomp::LoadGMSH41 (user/user_flexcomp.cc:1424)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_load_gmsh41(self_ptr: *mut mjCFlexcomp, buffer: *mut i8, binary: i32, nodeend: i32, nodebegin: i32, elemend: i32, elembegin: i32) {
    todo!() // cxx:__ZN11mjCFlexcomp10LoadGMSH41EPciiiii
}

/// C: mjCFlexcomp::LoadGMSH22 (user/user_flexcomp.cc:1659)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_flexcomp.cc:__ZL17ReadStrFromBufferPcPKci, cxx:__Z28IsValidElementOrNodeHeader22RKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_load_gmsh22(self_ptr: *mut mjCFlexcomp, buffer: *mut i8, binary: i32, nodeend: i32, nodebegin: i32, elemend: i32, elembegin: i32) {
    todo!() // cxx:__ZN11mjCFlexcomp10LoadGMSH22EPciiiii
}

/// C: mjCFlexcomp::LoadGMSH (user/user_flexcomp.cc:1954)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_flexcomp.cc:__ZL10findstringPKciS0_, cxx:__ZN11mjCFlexcomp10LoadGMSH22EPciiiii, cxx:__ZN11mjCFlexcomp10LoadGMSH41EPciiiii, cxx:_mju_readResource, cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flexcomp_load_gmsh(self_ptr: *mut mjCFlexcomp, model: *mut mjCModel, resource: *mut mjResource) {
    todo!() // cxx:__ZN11mjCFlexcomp8LoadGMSHEP8mjCModelP11mjResource_
}

