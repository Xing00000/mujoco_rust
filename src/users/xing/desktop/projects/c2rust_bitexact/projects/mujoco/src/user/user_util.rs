//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjuu_normvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:149)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_normvec__e510c0c396215e47(vec: *mut f64, n: i32) -> f64 {
    todo!() // cxx:__Z12mjuu_normvecPdi
}

/// C: mjuu_normvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:173)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_normvec__be2f524e17320b64(vec: *mut f32, n: i32) -> f32 {
    todo!() // cxx:__Z12mjuu_normvecPfi
}

/// C: mjuu_makenormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:368)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_makenormal__ee9a03b7f77fdd7f(normal: *mut f64, a: *const f64, b: *const f64, c: *const f64) -> f64 {
    todo!() // cxx:__Z15mjuu_makenormalIdEdPdPKT_S3_S3_
}

/// C: mjuu_makenormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:368)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_makenormal__94d41330a722bb3b(normal: *mut f64, a: *const f32, b: *const f32, c: *const f32) -> f64 {
    todo!() // cxx:__Z15mjuu_makenormalIfEdPdPKT_S3_S3_
}

/// C: mjuu_offcenter (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:531)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_offcenter(res: *mut f64, mass: f64, vec: *const f64) {
    todo!() // cxx:__Z14mjuu_offcenterPddPKd
}

/// C: mjuu_axisAngle2Quat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:564)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    todo!() // cxx-internal:user_util.cc.o:__ZL19mjuu_axisAngle2QuatPdPKdd
}

/// C: mjuu_isValidContentType (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:973)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_is_valid_content_type(text: std__string_view) -> bool {
    todo!() // cxx-internal:user_util.cc.o:__ZL23mjuu_isValidContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: VectorToString (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1256)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string__ce3b7950e5301140(v: *const ()) -> std__string {
    todo!() // cxx:__ZN6mujoco4user14VectorToStringINSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEEES8_RKNS2_6vectorIT_NS6_ISA_EEEE
}

/// C: VectorToString (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1256)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string__be6542ac13a133f2(v: *const ()) -> std__string {
    todo!() // cxx:__ZN6mujoco4user14VectorToStringIdEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEERKNS2_6vectorIT_NS6_ISA_EEEE
}

/// C: VectorToString (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1256)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string__4d440bb93bdd000e(v: *const ()) -> std__string {
    todo!() // cxx:__ZN6mujoco4user14VectorToStringIfEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEERKNS2_6vectorIT_NS6_ISA_EEEE
}

/// C: VectorToString (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1256)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string__fb2827af09aa72b1(v: *const ()) -> std__string {
    todo!() // cxx:__ZN6mujoco4user14VectorToStringIiEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEERKNS2_6vectorIT_NS6_ISA_EEEE
}

/// C: StrToNum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1277)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num__2d0a406bbc8ce2b7(str: *mut i8, c: *mut *mut i8) -> i32 {
    todo!() // cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_18StrToNumIiEET_PcPS4_
}

/// C: StrToNum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1283)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num__a1c9d2478a24adf2(str: *mut i8, c: *mut *mut i8) -> f32 {
    todo!() // cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_18StrToNumIfEET_PcPS4_
}

/// C: StrToNum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1289)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num__551aad3f816cb32c(str: *mut i8, c: *mut *mut i8) -> f64 {
    todo!() // cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_18StrToNumIdEET_PcPS4_
}

/// C: StrToNum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1295)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num__6d68765b7b4e8c9a(str: *mut i8, c: *mut *mut i8) -> u8 {
    todo!() // cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_18StrToNumIhEET_PcPS4_
}

/// C: IsNullOrSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1301)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_or_space(c: *mut i8) -> bool {
    todo!() // cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc
}

/// C: SkipSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1305)
/// Calls: cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn skip_space(c: *mut i8) -> *mut i8 {
    todo!() // cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1315)
/// Calls: cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc, cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__3f70338a6d4f85b3(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIdEENSt3__16vectorIT_NS2_9allocatorIS4_EEEEPc
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1315)
/// Calls: cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc, cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__6fe2984925e6eac0(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIfEENSt3__16vectorIT_NS2_9allocatorIS4_EEEEPc
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1315)
/// Calls: cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc, cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__5b25f4b369f4c788(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIhEENSt3__16vectorIT_NS2_9allocatorIS4_EEEEPc
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1315)
/// Calls: cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc, cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__55b8cea2a076b21d(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIiEENSt3__16vectorIT_NS2_9allocatorIS4_EEEEPc
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1357)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__16d073969cae2933(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIdEENSt3__16vectorIT_NS2_9allocatorIS4_EEEERKNS2_12basic_stringIcNS2_11char_traitsIcEENS5_IcEEEE
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1357)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__0148d1c40a74ba8d(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIfEENSt3__16vectorIT_NS2_9allocatorIS4_EEEERKNS2_12basic_stringIcNS2_11char_traitsIcEENS5_IcEEEE
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1357)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__10769be161d17871(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIhEENSt3__16vectorIT_NS2_9allocatorIS4_EEEERKNS2_12basic_stringIcNS2_11char_traitsIcEENS5_IcEEEE
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:1357)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__e4a893e611311120(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIiEENSt3__16vectorIT_NS2_9allocatorIS4_EEEERKNS2_12basic_stringIcNS2_11char_traitsIcEENS5_IcEEEE
}

/// C: mjuu_defined (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:35)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_defined(num: f64) -> bool {
    todo!() // cxx:__Z12mjuu_definedd
}

/// C: mjuu_matadr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_matadr(g1: i32, g2: i32, n: i32) -> i32 {
    todo!() // cxx:__Z11mjuu_matadriii
}

/// C: mjuu_setvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:42)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__d43c21ea458ddfdc(dest: *mut f64, x: f64, y: f64, z: f64, w: f64) {
    todo!() // cxx:__Z11mjuu_setvecPddddd
}

/// C: mjuu_setvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:43)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__542837345a72d70d(dest: *mut f32, x: f64, y: f64, z: f64, w: f64) {
    todo!() // cxx:__Z11mjuu_setvecPfdddd
}

/// C: mjuu_setvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:46)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__440c17e363895948(dest: *mut f64, x: f64, y: f64, z: f64) {
    todo!() // cxx:__Z11mjuu_setvecPdddd
}

/// C: mjuu_setvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:47)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__3606de7b4bc26bd7(dest: *mut f32, x: f64, y: f64, z: f64) {
    todo!() // cxx:__Z11mjuu_setvecPfddd
}

/// C: mjuu_setvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:50)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__3db305bf9e67300a(dest: *mut f64, x: f64, y: f64) {
    todo!() // cxx:__Z11mjuu_setvecPddd
}

/// C: mjuu_copyvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec__62e6986629b2e5d1(dest: *mut f64, src: *const f64, n: i32) {
    todo!() // cxx:__Z12mjuu_copyvecIddEvPT_PKT0_i
}

/// C: mjuu_copyvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec__73ebe39ec905588c(dest: *mut f32, src: *const f64, n: i32) {
    todo!() // cxx:__Z12mjuu_copyvecIfdEvPT_PKT0_i
}

/// C: mjuu_copyvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec__f703a8ca95dae886(dest: *mut f32, src: *const f32, n: i32) {
    todo!() // cxx:__Z12mjuu_copyvecIffEvPT_PKT0_i
}

/// C: mjuu_copyvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:54)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec__299d862b4988b52d(dest: *mut i32, src: *const i32, n: i32) {
    todo!() // cxx:__Z12mjuu_copyvecIiiEvPT_PKT0_i
}

/// C: mjuu_addtovec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:59)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_addtovec(dest: *mut f64, src: *const f64, n: i32) {
    todo!() // cxx:__Z13mjuu_addtovecPdPKdi
}

/// C: mjuu_zerovec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:62)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_zerovec__38878940bd8d44c4(dest: *mut f64, n: i32) {
    todo!() // cxx:__Z12mjuu_zerovecPdi
}

/// C: mjuu_zerovec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:65)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_zerovec__fe7aa91cc89316a9(dest: *mut f32, n: i32) {
    todo!() // cxx:__Z12mjuu_zerovecPfi
}

/// C: mjuu_dot3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dot3(a: *const f64, b: *const f64) -> f64 {
    todo!() // cxx:__Z9mjuu_dot3PKdS0_
}

/// C: mjuu_dist3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dist3(a: *const f64, b: *const f64) -> f64 {
    todo!() // cxx:__Z10mjuu_dist3PKdS0_
}

/// C: mjuu_L1 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_l1(a: *const f64, b: *const f64, n: i32) -> f64 {
    todo!() // cxx:__Z7mjuu_L1PKdS0_i
}

/// C: mjuu_scalevec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:82)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_scalevec(res: *mut f64, vec: *const f64, s: f64, n: i32) {
    todo!() // cxx:__Z13mjuu_scalevecPdPKddi
}

/// C: mjuu_quat2mat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:85)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_quat2mat(res: *mut f64, quat: *const f64) {
    todo!() // cxx:__Z13mjuu_quat2matPdPKd
}

/// C: mjuu_mulquat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:88)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulquat(res: *mut f64, qa: *const f64, qb: *const f64) {
    todo!() // cxx:__Z12mjuu_mulquatPdPKdS1_
}

/// C: mjuu_mulvecmat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:91)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmat(res: *mut f64, vec: *const f64, mat: *const f64) {
    todo!() // cxx:__Z14mjuu_mulvecmatPdPKdS1_
}

/// C: mjuu_mulvecmatT (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:94)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmat_t(res: *mut f64, vec: *const f64, mat: *const f64) {
    todo!() // cxx:__Z15mjuu_mulvecmatTPdPKdS1_
}

/// C: mjuu_mulRMRT (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:97)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mul_rmrt(res: *mut f64, R: *const f64, M: *const f64) {
    todo!() // cxx:__Z12mjuu_mulRMRTPdPKdS1_
}

/// C: mjuu_mulmat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:100)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulmat(res: *mut f64, A: *const f64, B: *const f64) {
    todo!() // cxx:__Z11mjuu_mulmatPdPKdS1_
}

/// C: mjuu_transposemat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:103)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_transposemat(res: *mut f64, mat: *const f64) {
    todo!() // cxx:__Z17mjuu_transposematPdPKd
}

/// C: mjuu_localaxis (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:106)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z14mjuu_mulvecmatPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localaxis(al: *mut f64, ag: *const f64, quat: *const f64) {
    todo!() // cxx:__Z14mjuu_localaxisPdPKdS1_
}

/// C: mjuu_localpos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:109)
/// Calls: cxx:__Z14mjuu_localaxisPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localpos(pl: *mut f64, pg: *const f64, pos: *const f64, quat: *const f64) {
    todo!() // cxx:__Z13mjuu_localposPdPKdS1_S1_
}

/// C: mjuu_localquat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:112)
/// Calls: cxx:__Z12mjuu_mulquatPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localquat(local: *mut f64, child: *const f64, parent: *const f64) {
    todo!() // cxx:__Z14mjuu_localquatPdPKdS1_
}

/// C: mjuu_crossvec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_crossvec(a: *mut f64, b: *const f64, c: *const f64) {
    todo!() // cxx:__Z13mjuu_crossvecPdPKdS1_
}

/// C: mjuu_z2quat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:122)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_z2quat(quat: *mut f64, vec: *const f64) {
    todo!() // cxx:__Z11mjuu_z2quatPdPKd
}

/// C: mjuu_frame2quat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frame2quat(quat: *mut f64, x: *const f64, y: *const f64, z: *const f64) {
    todo!() // cxx:__Z15mjuu_frame2quatPdPKdS1_S1_
}

/// C: mjuu_frameinvert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:128)
/// Calls: cxx:__Z14mjuu_localaxisPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameinvert(newpos: *mut f64, newquat: *mut f64, oldpos: *const f64, oldquat: *const f64) {
    todo!() // cxx:__Z16mjuu_frameinvertPdS_PKdS1_
}

/// C: mjuu_frameaccum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:132)
/// Calls: cxx:__Z12mjuu_mulquatPdPKdS1_, cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z14mjuu_mulvecmatPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccum(pos: *mut f64, quat: *mut f64, childpos: *const f64, childquat: *const f64) {
    todo!() // cxx:__Z15mjuu_frameaccumPdS_PKdS1_
}

/// C: mjuu_frameaccumChild (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:136)
/// Calls: cxx:__Z15mjuu_frameaccumPdS_PKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccum_child(pos: *const f64, quat: *const f64, childpos: *mut f64, childquat: *mut f64) {
    todo!() // cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_
}

/// C: mjuu_frameaccuminv (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:140)
/// Calls: cxx:__Z12mjuu_mulquatPdPKdS1_, cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z14mjuu_mulvecmatPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccuminv(pos: *mut f64, quat: *mut f64, childpos: *const f64, childquat: *const f64) {
    todo!() // cxx:__Z18mjuu_frameaccuminvPdS_PKdS1_
}

/// C: mjuu_globalinertia (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:144)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_globalinertia(global: *mut f64, local: *const f64, quat: *const f64) {
    todo!() // cxx:__Z18mjuu_globalinertiaPdPKdS1_
}

/// C: mjuu_visccoef (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_visccoef(visccoef: *mut f64, mass: f64, inertia: *const f64, scl: f64) {
    todo!() // cxx:__Z13mjuu_visccoefPddPKdd
}

/// C: mjuu_rotVecQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:153)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    todo!() // cxx:__Z15mjuu_rotVecQuatPdPKdS1_
}

/// C: mjuu_updateFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:156)
/// Calls: cxx-internal:user_util.cc.o:__ZL19mjuu_axisAngle2QuatPdPKdd, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z15mjuu_frame2quatPdPKdS1_S1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_update_frame(quat: *mut f64, normal: *mut f64, edge: *const f64, tprv: *const f64, tnxt: *const f64, first: i32) -> f64 {
    todo!() // cxx:__Z16mjuu_updateFramePdS_PKdS1_S1_i
}

/// C: mjuu_eig3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:160)
/// Calls: cxx:__Z11mjuu_mulmatPdPKdS1_, cxx:__Z12mjuu_mulquatPdPKdS1_, cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z17mjuu_transposematPdPKd
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eig3(eigval: *mut f64, eigvec: *mut f64, quat: *mut f64, mat: *const f64) -> i32 {
    todo!() // cxx:__Z9mjuu_eig3PdS_S_PKd
}

/// C: mjuu_eigendecompose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:166)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eigendecompose(mat: *mut f64, eigval: *mut f64, eigvec: *mut f64, n: i32) -> i32 {
    todo!() // cxx:__Z19mjuu_eigendecomposePdS_S_i
}

/// C: mjuu_trnVecPose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:169)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_trn_vec_pose(res: *mut f64, pos: *const f64, quat: *const f64, vec: *const f64) {
    todo!() // cxx:__Z15mjuu_trnVecPosePdPKdS1_S1_
}

/// C: mjuu_fullInertia (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:172)
/// Calls: cxx:__Z12mjuu_definedd, cxx:__Z9mjuu_eig3PdS_S_PKd
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_full_inertia(quat: *mut f64, inertia: *mut f64, fullinertia: *const f64) -> *const i8 {
    todo!() // cxx:__Z16mjuu_fullInertiaPdS_PKd
}

/// C: FilePath::IsAbs (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_abs(self_ptr: *mut mujoco__user__FilePath) -> bool {
    todo!() // cxx:__ZNK6mujoco4user8FilePath5IsAbsEv
}

/// C: FilePath::AbsPrefix (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:195)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_abs_prefix__a707c1fcc63b7653(self_ptr: *mut mujoco__user__FilePath) -> std__string {
    todo!() // cxx:__ZNK6mujoco4user8FilePath9AbsPrefixEv
}

/// C: FilePath::Str (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:198)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str(self_ptr: *mut mujoco__user__FilePath) -> *const std__string {
    todo!() // cxx:__ZNK6mujoco4user8FilePath3StrEv
}

/// C: FilePath::StrLower (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:202)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str_lower(self_ptr: *mut mujoco__user__FilePath) -> std__string {
    todo!() // cxx:__ZNK6mujoco4user8FilePath8StrLowerEv
}

/// C: FilePath::Ext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:205)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_ext(self_ptr: *mut mujoco__user__FilePath) -> std__string {
    todo!() // cxx:__ZNK6mujoco4user8FilePath3ExtEv
}

/// C: FilePath::StripExt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:211)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_ext(self_ptr: *mut mujoco__user__FilePath) -> mujoco__user__FilePath {
    todo!() // cxx:__ZNK6mujoco4user8FilePath8StripExtEv
}

/// C: FilePath::StripPath (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:214)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_path(self_ptr: *mut mujoco__user__FilePath) -> mujoco__user__FilePath {
    todo!() // cxx:__ZNK6mujoco4user8FilePath9StripPathEv
}

/// C: FilePath::Lower (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:217)
/// Calls: cxx:__ZNK6mujoco4user8FilePath8StrLowerEv
#[allow(unused_variables, non_snake_case)]
pub fn file_path_lower(self_ptr: *mut mujoco__user__FilePath) -> mujoco__user__FilePath {
    todo!() // cxx:__ZNK6mujoco4user8FilePath5LowerEv
}

/// C: FilePath::size (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_size(self_ptr: *mut mujoco__user__FilePath) -> u64 {
    todo!() // cxx:__ZNK6mujoco4user8FilePath4sizeEv
}

/// C: FilePath::c_str (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_c_str(self_ptr: *mut mujoco__user__FilePath) -> *const i8 {
    todo!() // cxx:__ZNK6mujoco4user8FilePath5c_strEv
}

/// C: FilePath::empty (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_empty(self_ptr: *mut mujoco__user__FilePath) -> bool {
    todo!() // cxx:__ZNK6mujoco4user8FilePath5emptyEv
}

/// C: FilePath::AbsPrefix (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:226)
/// Calls: cxx:_mjp_getResourceProvider
#[allow(unused_variables, non_snake_case)]
pub fn file_path_abs_prefix__d028c51d65d3465a(str: *const std__string) -> std__string {
    todo!() // cxx:__ZN6mujoco4user8FilePath9AbsPrefixERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

/// C: FilePath::PathReduce (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:227)
/// Calls: cxx:__ZN6mujoco4user8FilePath11IsSeparatorEc
#[allow(unused_variables, non_snake_case)]
pub fn file_path_path_reduce(str: *const std__string) -> std__string {
    todo!() // cxx:__ZN6mujoco4user8FilePath10PathReduceERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

/// C: FilePath::IsSeparator (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_separator(c: i8) -> bool {
    todo!() // cxx:__ZN6mujoco4user8FilePath11IsSeparatorEc
}

/// C: FilePath::Combine (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:231)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_combine(s1: *const std__string, s2: *const std__string) -> std__string {
    todo!() // cxx:__ZN6mujoco4user8FilePath7CombineERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_
}

/// C: FilePath::FilePathFast (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:234)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_file_path_fast__1712d7c1a2d4309e(str: *const std__string) -> mujoco__user__FilePath {
    todo!() // cxx:__ZN6mujoco4user8FilePath12FilePathFastERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

/// C: FilePath::FilePathFast (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_file_path_fast__926008bc00e1e678(str: *mut *mut std__string) -> mujoco__user__FilePath {
    todo!() // cxx:__ZN6mujoco4user8FilePath12FilePathFastEONSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

/// C: FileToMemory (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:259)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn file_to_memory(filename: *const i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user12FileToMemoryEPKc
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:267)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__9e96ad153a89eb0e(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorINSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEEENS2_6vectorIT_NS6_ISA_EEEEPc
}

/// C: StringToVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:268)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__a3c80793fb10bbd9(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorINSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEEENS2_6vectorIT_NS6_ISA_EEEERKS8_
}

/// C: mjuu_strippath (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:273)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_strippath(filename: std__string) -> std__string {
    todo!() // cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
}

/// C: mjuu_stripext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:276)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_stripext(filename: std__string) -> std__string {
    todo!() // cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
}

/// C: mjuu_getext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:279)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_getext(filename: std__string_view) -> std__string {
    todo!() // cxx:__Z11mjuu_getextNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: mjuu_isabspath (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:282)
/// Calls: cxx:_mjp_getResourceProvider
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_isabspath(path: std__string) -> bool {
    todo!() // cxx:__Z14mjuu_isabspathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
}

/// C: mjuu_combinePaths (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:285)
/// Calls: cxx:__Z14mjuu_isabspathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_combine_paths__3bc593ff5e06992b(path1: *const std__string, path2: *const std__string) -> std__string {
    todo!() // cxx:__Z17mjuu_combinePathsRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEES7_
}

/// C: mjuu_combinePaths (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:286)
/// Calls: cxx:__ZNK11MeshPolygon5PathsEv
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_combine_paths__1b3858e571e2befa(path1: *const std__string, path2: *const std__string, path3: *const std__string) -> std__string {
    todo!() // cxx:__Z17mjuu_combinePathsRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEES7_S7_
}

/// C: mjuu_parseContentTypeAttrType (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:290)
/// Calls: cxx-internal:user_util.cc.o:__ZL23mjuu_isValidContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_type(text: std__string_view) -> *const () {
    todo!() // cxx:__Z29mjuu_parseContentTypeAttrTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: mjuu_parseContentTypeAttrSubtype (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:293)
/// Calls: cxx-internal:user_util.cc.o:__ZL23mjuu_isValidContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_subtype(text: std__string_view) -> *const () {
    todo!() // cxx:__Z32mjuu_parseContentTypeAttrSubtypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: mjuu_extToContentType (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:296)
/// Calls: cxx:__Z11mjuu_getextNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_ext_to_content_type(filename: std__string_view) -> std__string {
    todo!() // cxx:__Z21mjuu_extToContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: mjuu_dirnamelen (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dirnamelen(path: *const i8) -> i32 {
    todo!() // cxx:__Z15mjuu_dirnamelenPKc
}

