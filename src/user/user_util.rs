//! Port of: user/user_util.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjuu_defined (user/user_util.cc:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_defined(num: f64) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (!(num).is_nan()) }
}

/// C: mjuu_matadr (user/user_util.cc:54)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_matadr(g1: i32, g2: i32, n: i32) -> i32 {
    if g1 < 0 || g2 < 0 || g1 >= n || g2 >= n {
        return -1;
    }

    let (g1, g2) = if g1 > g2 { (g2, g1) } else { (g1, g2) };
    g1 * n + g2
}

/// C: mjuu_setvec (user/user_util.cc:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__d43c21ea458ddfdc(dest: *mut f64, x: f64, y: f64, z: f64, w: f64) {
    todo!() // cxx:__Z11mjuu_setvecPddddd
}

/// C: mjuu_setvec (user/user_util.cc:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__542837345a72d70d(dest: *mut f32, x: f64, y: f64, z: f64, w: f64) {
    todo!() // cxx:__Z11mjuu_setvecPfdddd
}

/// C: mjuu_setvec (user/user_util.cc:83)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__440c17e363895948(dest: *mut f64, x: f64, y: f64, z: f64) {
    todo!() // cxx:__Z11mjuu_setvecPdddd
}

/// C: mjuu_setvec (user/user_util.cc:88)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__3606de7b4bc26bd7(dest: *mut f32, x: f64, y: f64, z: f64) {
    todo!() // cxx:__Z11mjuu_setvecPfddd
}

/// C: mjuu_setvec (user/user_util.cc:96)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec__3db305bf9e67300a(dest: *mut f64, x: f64, y: f64) {
    todo!() // cxx:__Z11mjuu_setvecPddd
}

/// C: mjuu_addtovec (user/user_util.cc:103)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_addtovec(dest: *mut f64, src: *const f64, n: i32) {
    // SAFETY: caller guarantees dest and src point to at least n f64
    unsafe {
        let mut i: i32 = 0;
        while i < n {
            *dest.offset(i as isize) += *src.offset(i as isize);
            i += 1;
        }
    }
}

/// C: mjuu_zerovec (user/user_util.cc:108)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_zerovec__38878940bd8d44c4(dest: *mut f64, n: i32) {
    todo!() // cxx:__Z12mjuu_zerovecPdi
}

/// C: mjuu_zerovec (user/user_util.cc:113)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_zerovec__fe7aa91cc89316a9(dest: *mut f32, n: i32) {
    todo!() // cxx:__Z12mjuu_zerovecPfi
}

/// C: mjuu_dot3 (user/user_util.cc:118)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mjuu_dot3(a: *const f64, b: *const f64) -> f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { ((({ const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = a; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x00])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) } * { const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = b; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x00])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) }) + ({ const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = a; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x01])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) } * { const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = b; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x01])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) })) + ({ const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = a; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x02])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) } * { const _: [(); 8] = [(); core::mem::size_of::<f64>()]; const _: [(); 8] = [(); core::mem::align_of::<f64>()]; const _: [(); 4] = [(); core::mem::size_of::<i32>()]; const _: [(); 8] = [(); core::mem::size_of::<isize>()]; const _: [(); 1] = [(); cfg!(target_endian = "little") as usize]; let machine_base = b; let machine_index = (i32::from_be_bytes([0x00, 0x00, 0x00, 0x02])) as isize; let machine_address = if machine_index < 0 { machine_base.sub(machine_index.wrapping_neg() as usize) } else { machine_base.add(machine_index as usize) }; core::ptr::read::<f64>(machine_address) })) }
}

/// C: mjuu_dist3 (user/user_util.cc:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dist3(a: *const f64, b: *const f64) -> f64 {
    // SAFETY: caller guarantees a and b point to at least 3 f64
    unsafe {
        f64::sqrt(
            (*a.add(0) - *b.add(0)) * (*a.add(0) - *b.add(0))
          + (*a.add(1) - *b.add(1)) * (*a.add(1) - *b.add(1))
          + (*a.add(2) - *b.add(2)) * (*a.add(2) - *b.add(2))
        )
    }
}

/// C: mjuu_L1 (user/user_util.cc:132)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_l1(a: *const f64, b: *const f64, n: i32) -> f64 {
    todo!() // cxx:__Z7mjuu_L1PKdS0_i
}

/// C: mjuu_normvec (user/user_util.cc:141)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_normvec__e510c0c396215e47(vec: *mut f64, n: i32) -> f64 {
    todo!() // cxx:__Z12mjuu_normvecPdi
}

/// C: mjuu_normvec (user/user_util.cc:159)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_normvec__be2f524e17320b64(vec: *mut f32, n: i32) -> f32 {
    todo!() // cxx:__Z12mjuu_normvecPfi
}

/// C: mjuu_scalevec (user/user_util.cc:176)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_scalevec(res: *mut f64, vec: *const f64, s: f64, n: i32) {
    // SAFETY: res and vec are valid arrays of length >= n from caller
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = s * *vec.add(i);
        }
    }
}

/// C: mjuu_quat2mat (user/user_util.cc:181)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_quat2mat(res: *mut f64, quat: *const f64) {
    // SAFETY: res is 9-element array, quat is 4-element array from caller
    unsafe {
        // identity quat: identity mat
        if *quat.add(0) == 1.0 && *quat.add(1) == 0.0 && *quat.add(2) == 0.0 && *quat.add(3) == 0.0 {
            *res.add(0) = 1.0;
            *res.add(1) = 0.0;
            *res.add(2) = 0.0;
            *res.add(3) = 0.0;
            *res.add(4) = 1.0;
            *res.add(5) = 0.0;
            *res.add(6) = 0.0;
            *res.add(7) = 0.0;
            *res.add(8) = 1.0;
            return;
        }

        // regular processing
        let q00 = *quat.add(0) * *quat.add(0);
        let q01 = *quat.add(0) * *quat.add(1);
        let q02 = *quat.add(0) * *quat.add(2);
        let q03 = *quat.add(0) * *quat.add(3);
        let q11 = *quat.add(1) * *quat.add(1);
        let q12 = *quat.add(1) * *quat.add(2);
        let q13 = *quat.add(1) * *quat.add(3);
        let q22 = *quat.add(2) * *quat.add(2);
        let q23 = *quat.add(2) * *quat.add(3);
        let q33 = *quat.add(3) * *quat.add(3);

        *res.add(0) = q00 + q11 - q22 - q33;
        *res.add(4) = q00 - q11 + q22 - q33;
        *res.add(8) = q00 - q11 - q22 + q33;

        *res.add(1) = 2.0 * (q12 - q03);
        *res.add(2) = 2.0 * (q13 + q02);
        *res.add(3) = 2.0 * (q12 + q03);
        *res.add(5) = 2.0 * (q23 - q01);
        *res.add(6) = 2.0 * (q13 - q02);
        *res.add(7) = 2.0 * (q23 + q01);
    }
}

/// C: mjuu_mulquat (user/user_util.cc:222)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulquat(res: *mut f64, qa: *const f64, qb: *const f64) {
    todo!() // cxx:__Z12mjuu_mulquatPdPKdS1_
}

/// C: mjuu_mulvecmat (user/user_util.cc:234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmat(res: *mut f64, vec: *const f64, mat: *const f64) {
    // SAFETY: res is 3-element, vec is 3-element, mat is 9-element (3x3 row-major)
    unsafe {
        let tmp0 = *mat.add(0) * *vec.add(0) + *mat.add(1) * *vec.add(1) + *mat.add(2) * *vec.add(2);
        let tmp1 = *mat.add(3) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(5) * *vec.add(2);
        let tmp2 = *mat.add(6) * *vec.add(0) + *mat.add(7) * *vec.add(1) + *mat.add(8) * *vec.add(2);
        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
    }
}

/// C: mjuu_mulvecmatT (user/user_util.cc:246)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmat_t(res: *mut f64, vec: *const f64, mat: *const f64) {
    todo!() // cxx:__Z15mjuu_mulvecmatTPdPKdS1_
}

/// C: mjuu_mulRMRT (user/user_util.cc:258)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mul_rmrt(res: *mut f64, R: *const f64, M: *const f64) {
    todo!() // cxx:__Z12mjuu_mulRMRTPdPKdS1_
}

/// C: mjuu_mulmat (user/user_util.cc:286)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulmat(res: *mut f64, A: *const f64, B: *const f64) {
    todo!() // cxx:__Z11mjuu_mulmatPdPKdS1_
}

/// C: mjuu_transposemat (user/user_util.cc:304)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_transposemat(res: *mut f64, mat: *const f64) {
    todo!() // cxx:__Z17mjuu_transposematPdPKd
}

/// C: mjuu_localaxis (user/user_util.cc:311)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z14mjuu_mulvecmatPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localaxis(al: *mut f64, ag: *const f64, quat: *const f64) {
    // SAFETY: al[3], ag[3], quat[4] valid (caller contract)
    unsafe {
        let mut mat: [f64; 9] = [0.0; 9];
        let qneg: [f64; 4] = [*quat.add(0), -*quat.add(1), -*quat.add(2), -*quat.add(3)];
        mjuu_quat2mat(mat.as_mut_ptr(), qneg.as_ptr());
        mjuu_mulvecmat(al, ag, mat.as_ptr());
    }
}

/// C: mjuu_localpos (user/user_util.cc:320)
/// Calls: cxx:__Z14mjuu_localaxisPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localpos(pl: *mut f64, pg: *const f64, pos: *const f64, quat: *const f64) {
    // SAFETY: pl[3], pg[3], pos[3], quat[4] valid (caller contract)
    unsafe {
        let a: [f64; 3] = [
            *pg.add(0) - *pos.add(0),
            *pg.add(1) - *pos.add(1),
            *pg.add(2) - *pos.add(2),
        ];
        mjuu_localaxis(pl, a.as_ptr(), quat);
    }
}

/// C: mjuu_localquat (user/user_util.cc:327)
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

/// C: mjuu_crossvec (user/user_util.cc:335)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_crossvec(a: *mut f64, b: *const f64, c: *const f64) {
    // SAFETY: a, b, c are valid 3-element arrays from caller
    unsafe {
        *a.add(0) = *b.add(1) * *c.add(2) - *b.add(2) * *c.add(1);
        *a.add(1) = *b.add(2) * *c.add(0) - *b.add(0) * *c.add(2);
        *a.add(2) = *b.add(0) * *c.add(1) - *b.add(1) * *c.add(0);
    }
}

/// C: mjuu_makenormal (user/user_util.cc:345)
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

/// C: mjuu_makenormal (user/user_util.cc:345)
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

/// C: mjuu_z2quat (user/user_util.cc:376)
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

/// C: mjuu_frame2quat (user/user_util.cc:393)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frame2quat(quat: *mut f64, x: *const f64, y: *const f64, z: *const f64) {
    todo!() // cxx:__Z15mjuu_frame2quatPdPKdS1_S1_
}

/// C: mjuu_frameinvert (user/user_util.cc:436)
/// Calls: cxx:__Z14mjuu_localaxisPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameinvert(newpos: *mut f64, newquat: *mut f64, oldpos: *const f64, oldquat: *const f64) {
    // SAFETY: newpos[3], newquat[4], oldpos[3], oldquat[4] valid (caller contract)
    unsafe {
        // position
        mjuu_localaxis(newpos, oldpos, oldquat);
        *newpos.add(0) = -*newpos.add(0);
        *newpos.add(1) = -*newpos.add(1);
        *newpos.add(2) = -*newpos.add(2);

        // orientation
        *newquat.add(0) = *oldquat.add(0);
        *newquat.add(1) = -*oldquat.add(1);
        *newquat.add(2) = -*oldquat.add(2);
        *newquat.add(3) = -*oldquat.add(3);
    }
}

/// C: mjuu_frameaccum (user/user_util.cc:455)
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

/// C: mjuu_frameaccumChild (user/user_util.cc:471)
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

/// C: mjuu_frameaccuminv (user/user_util.cc:484)
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

/// C: mjuu_globalinertia (user/user_util.cc:499)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_globalinertia(global: *mut f64, local: *const f64, quat: *const f64) {
    // SAFETY: global[6], local[3], quat[4] valid (caller contract)
    unsafe {
        let mut mat: [f64; 9] = [0.0; 9];
        mjuu_quat2mat(mat.as_mut_ptr(), quat);
        let tmp: [f64; 9] = [
            mat[0] * *local.add(0), mat[3] * *local.add(0), mat[6] * *local.add(0),
            mat[1] * *local.add(1), mat[4] * *local.add(1), mat[7] * *local.add(1),
            mat[2] * *local.add(2), mat[5] * *local.add(2), mat[8] * *local.add(2),
        ];
        *global.add(0) = mat[0] * tmp[0] + mat[1] * tmp[3] + mat[2] * tmp[6];
        *global.add(1) = mat[3] * tmp[1] + mat[4] * tmp[4] + mat[5] * tmp[7];
        *global.add(2) = mat[6] * tmp[2] + mat[7] * tmp[5] + mat[8] * tmp[8];
        *global.add(3) = mat[0] * tmp[1] + mat[1] * tmp[4] + mat[2] * tmp[7];
        *global.add(4) = mat[0] * tmp[2] + mat[1] * tmp[5] + mat[2] * tmp[8];
        *global.add(5) = mat[3] * tmp[2] + mat[4] * tmp[5] + mat[5] * tmp[8];
    }
}

/// C: mjuu_offcenter (user/user_util.cc:521)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_offcenter(res: *mut f64, mass: f64, vec: *const f64) {
    // SAFETY: res[6], vec[3] valid (caller contract)
    unsafe {
        *res.add(0) = mass * (*vec.add(1) * *vec.add(1) + *vec.add(2) * *vec.add(2));
        *res.add(1) = mass * (*vec.add(0) * *vec.add(0) + *vec.add(2) * *vec.add(2));
        *res.add(2) = mass * (*vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1));
        *res.add(3) = -mass * *vec.add(0) * *vec.add(1);
        *res.add(4) = -mass * *vec.add(0) * *vec.add(2);
        *res.add(5) = -mass * *vec.add(1) * *vec.add(2);
    }
}

/// C: mjuu_visccoef (user/user_util.cc:532)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_visccoef(visccoef: *mut f64, mass: f64, inertia: *const f64, scl: f64) {
    // SAFETY: visccoef[6], inertia[3] valid (caller contract). mass > 0 assumed.
    const MJ_EPS: f64 = 1e-14;
    unsafe {
        // compute equivalent box
        let e0 = (((*inertia.add(1) + *inertia.add(2) - *inertia.add(0)) / mass * 6.0).max(MJ_EPS)).sqrt();
        let e1 = (((*inertia.add(0) + *inertia.add(2) - *inertia.add(1)) / mass * 6.0).max(MJ_EPS)).sqrt();
        let e2 = (((*inertia.add(0) + *inertia.add(1) - *inertia.add(2)) / mass * 6.0).max(MJ_EPS)).sqrt();
        // torque components
        *visccoef.add(0) = scl * 4.0 / 3.0 * e0 * (e1*e1*e1 + e2*e2*e2);
        *visccoef.add(1) = scl * 4.0 / 3.0 * e1 * (e0*e0*e0 + e2*e2*e2);
        *visccoef.add(2) = scl * 4.0 / 3.0 * e2 * (e0*e0*e0 + e1*e1*e1);
        // force components
        *visccoef.add(3) = scl * 4.0 * e1 * e2;
        *visccoef.add(4) = scl * 4.0 * e0 * e2;
        *visccoef.add(5) = scl * 4.0 * e0 * e1;
    }
}

/// C: mjuu_axisAngle2Quat (user/user_util.cc:557)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    todo!() // cxx-internal:user_util.cc.o:__ZL19mjuu_axisAngle2QuatPdPKdd
}

/// C: mjuu_rotVecQuat (user/user_util.cc:577)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    todo!() // cxx:__Z15mjuu_rotVecQuatPdPKdS1_
}

/// C: mjuu_updateFrame (user/user_util.cc:617)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZL19mjuu_axisAngle2QuatPdPKdd, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z15mjuu_frame2quatPdPKdS1_S1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_update_frame(quat: *mut f64, normal: *mut f64, edge: *const f64, tprv: *const f64, tnxt: *const f64, first: i32) -> f64 {
    todo!() // cxx:__Z16mjuu_updateFramePdS_PKdS1_S1_i
}

/// C: mjuu_eig3 (user/user_util.cc:657)
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

/// C: mjuu_eigendecompose (user/user_util.cc:752)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eigendecompose(mat: *mut f64, eigval: *mut f64, eigvec: *mut f64, n: i32) -> i32 {
    // Jacobi eigendecomposition of a symmetric matrix.
    // SAFETY: mat[n*n], eigval[n], eigvec[n*n] are valid pointers (caller contract).
    unsafe {
        let n = n as usize;

        // initialize eigvec to identity
        for k in 0..(n * n) {
            *eigvec.add(k) = 0.0;
        }
        for i in 0..n {
            *eigvec.add(i * n + i) = 1.0;
        }

        let max_sweeps: i32 = 200;
        let tol: f64 = 1e-12;

        let mut sweep: i32 = 0;
        while sweep < max_sweeps {
            // check convergence: sum of squared off-diagonal elements
            let mut off_diag: f64 = 0.0;
            for i in 0..n {
                for j in (i + 1)..n {
                    let v = *mat.add(i * n + j);
                    off_diag += v * v;
                }
            }
            if off_diag < tol * tol { break; }

            // sweep over all off-diagonal pairs
            for p in 0..n {
                for q in (p + 1)..n {
                    let apq = *mat.add(p * n + q);
                    if apq.abs() < tol * 1e-3 { continue; }

                    let app = *mat.add(p * n + p);
                    let aqq = *mat.add(q * n + q);
                    let tau = (aqq - app) / (2.0 * apq);
                    let t = (if tau >= 0.0 { 1.0 } else { -1.0 })
                        / (tau.abs() + (1.0 + tau * tau).sqrt());
                    let c = 1.0 / (1.0 + t * t).sqrt();
                    let s = t * c;

                    // update matrix (Jacobi rotation)
                    *mat.add(p * n + p) -= t * apq;
                    *mat.add(q * n + q) += t * apq;
                    *mat.add(p * n + q) = 0.0;
                    *mat.add(q * n + p) = 0.0;

                    for r in 0..n {
                        if r == p || r == q { continue; }
                        let mrp = *mat.add(r * n + p);
                        let mrq = *mat.add(r * n + q);
                        let new_rp = c * mrp - s * mrq;
                        let new_rq = s * mrp + c * mrq;
                        *mat.add(r * n + p) = new_rp;
                        *mat.add(p * n + r) = new_rp;
                        *mat.add(r * n + q) = new_rq;
                        *mat.add(q * n + r) = new_rq;
                    }

                    // accumulate eigenvectors
                    for r in 0..n {
                        let vrp = *eigvec.add(r * n + p);
                        let vrq = *eigvec.add(r * n + q);
                        *eigvec.add(r * n + p) = c * vrp - s * vrq;
                        *eigvec.add(r * n + q) = s * vrp + c * vrq;
                    }
                }
            }
            sweep += 1;
        }

        // extract eigenvalues from diagonal
        for i in 0..n {
            *eigval.add(i) = *mat.add(i * n + i);
        }

        sweep
    }
}

/// C: mjuu_trnVecPose (user/user_util.cc:819)
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

/// C: mjuu_strippath (user/user_util.cc:828)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_strippath(filename: std__string) -> std__string {
    todo!() // cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
}

/// C: mjuu_fullInertia (user/user_util.cc:845)
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

/// C: mjuu_stripext (user/user_util.cc:874)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_stripext(filename: std__string) -> std__string {
    todo!() // cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
}

/// C: mjuu_getext (user/user_util.cc:885)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_getext(filename: std__string_view) -> std__string {
    todo!() // cxx:__Z11mjuu_getextNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: mjuu_isabspath (user/user_util.cc:894)
/// Calls: cxx:_mjp_getResourceProvider
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_isabspath(path: std__string) -> bool {
    todo!() // cxx:__Z14mjuu_isabspathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
}

/// C: mjuu_combinePaths (user/user_util.cc:916)
/// Calls: cxx:__Z14mjuu_isabspathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_combine_paths__3bc593ff5e06992b(path1: *const std__string, path2: *const std__string) -> std__string {
    todo!() // cxx:__Z17mjuu_combinePathsRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEES7_
}

/// C: mjuu_combinePaths (user/user_util.cc:929)
/// Calls: cxx:__ZNK11MeshPolygon5PathsEv
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_combine_paths__1b3858e571e2befa(path1: *const std__string, path2: *const std__string, path3: *const std__string) -> std__string {
    todo!() // cxx:__Z17mjuu_combinePathsRKNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEES7_S7_
}

/// C: mjuu_isValidContentType (user/user_util.cc:936)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_is_valid_content_type(text: std__string_view) -> bool {
    todo!() // cxx-internal:user_util.cc.o:__ZL23mjuu_isValidContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: mjuu_parseContentTypeAttrType (user/user_util.cc:958)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZL23mjuu_isValidContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_type(text: std__string_view) -> *const () {
    todo!() // cxx:__Z29mjuu_parseContentTypeAttrTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: mjuu_parseContentTypeAttrSubtype (user/user_util.cc:967)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZL23mjuu_isValidContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_subtype(text: std__string_view) -> *const () {
    todo!() // cxx:__Z32mjuu_parseContentTypeAttrSubtypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: mjuu_extToContentType (user/user_util.cc:979)
/// Calls: cxx:__Z11mjuu_getextNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_ext_to_content_type(filename: std__string_view) -> std__string {
    todo!() // cxx:__Z21mjuu_extToContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
}

/// C: mjuu_dirnamelen (user/user_util.cc:1000)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dirnamelen(path: *const i8) -> i32 {
    if path.is_null() {
        return 0;
    }
    // SAFETY: path is a valid null-terminated C string (caller contract)
    unsafe {
        let mut pos: i32 = -1;
        let mut i: i32 = 0;
        while *path.offset(i as isize) != 0 {
            if *path.offset(i as isize) == b'/' as i8 || *path.offset(i as isize) == b'\\' as i8 {
                pos = i;
            }
            i += 1;
        }
        pos + 1
    }
}

/// C: FilePath::Combine (user/user_util.cc:1013)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_combine(s1: *const std__string, s2: *const std__string) -> std__string {
    todo!() // cxx:__ZN6mujoco4user8FilePath7CombineERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_
}

/// C: FilePath::PathReduce (user/user_util.cc:1022)
/// Calls: cxx:__ZN6mujoco4user8FilePath11IsSeparatorEc
#[allow(unused_variables, non_snake_case)]
pub fn file_path_path_reduce(str: *const std__string) -> std__string {
    todo!() // cxx:__ZN6mujoco4user8FilePath10PathReduceERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

/// C: FilePath::Ext (user/user_util.cc:1055)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_ext(self_ptr: *mut mujoco__user__FilePath) -> std__string {
    todo!() // cxx:__ZNK6mujoco4user8FilePath3ExtEv
}

/// C: FilePath::StripExt (user/user_util.cc:1062)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_ext(self_ptr: *mut mujoco__user__FilePath) -> mujoco__user__FilePath {
    todo!() // cxx:__ZNK6mujoco4user8FilePath8StripExtEv
}

/// C: FilePath::AbsPrefix (user/user_util.cc:1073)
/// Calls: cxx:_mjp_getResourceProvider
#[allow(unused_variables, non_snake_case)]
pub fn file_path_abs_prefix__d028c51d65d3465a(str: *const std__string) -> std__string {
    todo!() // cxx:__ZN6mujoco4user8FilePath9AbsPrefixERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

/// C: FilePath::StripPath (user/user_util.cc:1098)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_path(self_ptr: *mut mujoco__user__FilePath) -> mujoco__user__FilePath {
    todo!() // cxx:__ZNK6mujoco4user8FilePath9StripPathEv
}

/// C: FilePath::StrLower (user/user_util.cc:1108)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str_lower(self_ptr: *mut mujoco__user__FilePath) -> std__string {
    todo!() // cxx:__ZNK6mujoco4user8FilePath8StrLowerEv
}

/// C: FileToMemory (user/user_util.cc:1117)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn file_to_memory(filename: *const i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user12FileToMemoryEPKc
}

/// C: VectorToString (user/user_util.cc:1176)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string__ce3b7950e5301140(v: *const ()) -> std__string {
    todo!() // cxx:__ZN6mujoco4user14VectorToStringINSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEEES8_RKNS2_6vectorIT_NS6_ISA_EEEE
}

/// C: VectorToString (user/user_util.cc:1176)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string__be6542ac13a133f2(v: *const ()) -> std__string {
    todo!() // cxx:__ZN6mujoco4user14VectorToStringIdEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEERKNS2_6vectorIT_NS6_ISA_EEEE
}

/// C: VectorToString (user/user_util.cc:1176)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string__4d440bb93bdd000e(v: *const ()) -> std__string {
    todo!() // cxx:__ZN6mujoco4user14VectorToStringIfEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEERKNS2_6vectorIT_NS6_ISA_EEEE
}

/// C: VectorToString (user/user_util.cc:1176)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string__fb2827af09aa72b1(v: *const ()) -> std__string {
    todo!() // cxx:__ZN6mujoco4user14VectorToStringIiEENSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEERKNS2_6vectorIT_NS6_ISA_EEEE
}

/// C: StrToNum (user/user_util.cc:1197)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num__70ea69e2fe61fbdc(str: *mut i8, c: *mut *mut i8) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_18StrToNumIiEET_PcPS4_
}

/// C: StrToNum (user/user_util.cc:1204)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num__37fab230446b1876(str: *mut i8, c: *mut *mut i8) -> f32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_18StrToNumIfEET_PcPS4_
}

/// C: StrToNum (user/user_util.cc:1211)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num__d5504e69e014d78e(str: *mut i8, c: *mut *mut i8) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_18StrToNumIdEET_PcPS4_
}

/// C: StrToNum (user/user_util.cc:1218)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num__ff56cab0ab8e5878(str: *mut i8, c: *mut *mut i8) -> u8 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_18StrToNumIhEET_PcPS4_
}

/// C: IsNullOrSpace (user/user_util.cc:1224)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_or_space(c: *mut i8) -> bool {
    todo!() // cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc
}

/// C: SkipSpace (user/user_util.cc:1228)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn skip_space(c: *mut i8) -> *mut i8 {
    todo!() // cxx-internal:user_util.cc.o:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
}

/// C: StringToVector (user/user_util.cc:1237)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__3f70338a6d4f85b3(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIdEENSt3__16vectorIT_NS2_9allocatorIS4_EEEEPc
}

/// C: StringToVector (user/user_util.cc:1237)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__6fe2984925e6eac0(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIfEENSt3__16vectorIT_NS2_9allocatorIS4_EEEEPc
}

/// C: StringToVector (user/user_util.cc:1237)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__5b25f4b369f4c788(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIhEENSt3__16vectorIT_NS2_9allocatorIS4_EEEEPc
}

/// C: StringToVector (user/user_util.cc:1237)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_113IsNullOrSpaceEPc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_util.cc:__ZN6mujoco4user12_GLOBAL__N_19SkipSpaceEPc
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__55b8cea2a076b21d(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIiEENSt3__16vectorIT_NS2_9allocatorIS4_EEEEPc
}

/// C: StringToVector (user/user_util.cc:1260)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__9e96ad153a89eb0e(cs: *mut i8) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorINSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEEENS2_6vectorIT_NS6_ISA_EEEEPc
}

/// C: StringToVector (user/user_util.cc:1265)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__a3c80793fb10bbd9(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorINSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEEENS2_6vectorIT_NS6_ISA_EEEERKS8_
}

/// C: StringToVector (user/user_util.cc:1280)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__16d073969cae2933(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIdEENSt3__16vectorIT_NS2_9allocatorIS4_EEEERKNS2_12basic_stringIcNS2_11char_traitsIcEENS5_IcEEEE
}

/// C: StringToVector (user/user_util.cc:1280)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__0148d1c40a74ba8d(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIfEENSt3__16vectorIT_NS2_9allocatorIS4_EEEERKNS2_12basic_stringIcNS2_11char_traitsIcEENS5_IcEEEE
}

/// C: StringToVector (user/user_util.cc:1280)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__10769be161d17871(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIhEENSt3__16vectorIT_NS2_9allocatorIS4_EEEERKNS2_12basic_stringIcNS2_11char_traitsIcEENS5_IcEEEE
}

/// C: StringToVector (user/user_util.cc:1280)
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector__e4a893e611311120(s: *const std__string) -> *const () {
    todo!() // cxx:__ZN6mujoco4user14StringToVectorIiEENSt3__16vectorIT_NS2_9allocatorIS4_EEEERKNS2_12basic_stringIcNS2_11char_traitsIcEENS5_IcEEEE
}

/// C: mjuu_copyvec (user/user_util.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec__62e6986629b2e5d1(dest: *mut f64, src: *const f64, n: i32) {
    todo!() // cxx:__Z12mjuu_copyvecIddEvPT_PKT0_i
}

/// C: mjuu_copyvec (user/user_util.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec__73ebe39ec905588c(dest: *mut f32, src: *const f64, n: i32) {
    todo!() // cxx:__Z12mjuu_copyvecIfdEvPT_PKT0_i
}

/// C: mjuu_copyvec (user/user_util.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec__f703a8ca95dae886(dest: *mut f32, src: *const f32, n: i32) {
    todo!() // cxx:__Z12mjuu_copyvecIffEvPT_PKT0_i
}

/// C: mjuu_copyvec (user/user_util.h:54)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec__299d862b4988b52d(dest: *mut i32, src: *const i32, n: i32) {
    todo!() // cxx:__Z12mjuu_copyvecIiiEvPT_PKT0_i
}

/// C: FilePath::IsAbs (user/user_util.h:203)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_abs(self_ptr: *mut mujoco__user__FilePath) -> bool {
    todo!() // cxx:__ZNK6mujoco4user8FilePath5IsAbsEv
}

/// C: FilePath::AbsPrefix (user/user_util.h:207)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_abs_prefix__a707c1fcc63b7653(self_ptr: *mut mujoco__user__FilePath) -> std__string {
    todo!() // cxx:__ZNK6mujoco4user8FilePath9AbsPrefixEv
}

/// C: FilePath::Str (user/user_util.h:210)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str(self_ptr: *mut mujoco__user__FilePath) -> *const std__string {
    todo!() // cxx:__ZNK6mujoco4user8FilePath3StrEv
}

/// C: FilePath::Lower (user/user_util.h:229)
/// Calls: cxx:__ZNK6mujoco4user8FilePath8StrLowerEv
#[allow(unused_variables, non_snake_case)]
pub fn file_path_lower(self_ptr: *mut mujoco__user__FilePath) -> mujoco__user__FilePath {
    todo!() // cxx:__ZNK6mujoco4user8FilePath5LowerEv
}

/// C: FilePath::size (user/user_util.h:232)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_size(self_ptr: *mut mujoco__user__FilePath) -> u64 {
    todo!() // cxx:__ZNK6mujoco4user8FilePath4sizeEv
}

/// C: FilePath::c_str (user/user_util.h:233)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_c_str(self_ptr: *mut mujoco__user__FilePath) -> *const i8 {
    todo!() // cxx:__ZNK6mujoco4user8FilePath5c_strEv
}

/// C: FilePath::empty (user/user_util.h:234)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_empty(self_ptr: *mut mujoco__user__FilePath) -> bool {
    todo!() // cxx:__ZNK6mujoco4user8FilePath5emptyEv
}

/// C: FilePath::IsSeparator (user/user_util.h:240)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_separator(c: i8) -> bool {
    todo!() // cxx:__ZN6mujoco4user8FilePath11IsSeparatorEc
}

/// C: FilePath::FilePathFast (user/user_util.h:244)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_file_path_fast__1712d7c1a2d4309e(str: *const std__string) -> mujoco__user__FilePath {
    todo!() // cxx:__ZN6mujoco4user8FilePath12FilePathFastERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

/// C: FilePath::FilePathFast (user/user_util.h:250)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_file_path_fast__926008bc00e1e678(str: *mut *mut std__string) -> mujoco__user__FilePath {
    todo!() // cxx:__ZN6mujoco4user8FilePath12FilePathFastEONSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

