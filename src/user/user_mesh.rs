//! Port of: user/user_mesh.cc
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: Fovea (user/user_mesh.cc:83)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn Fovea(x: f64, gamma: f64) -> f64 {
    if gamma == 0.0 {
        return x;
    }
    let g = if 1.0 < gamma { 1.0 } else { gamma };
    let g = if 0.0 > g { 0.0 } else { g };
    g * x.powf(5.0) + (1.0 - g) * x
}

/// C: LinSpace (user/user_mesh.cc:93)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn LinSpace(lower: f64, upper: f64, n: i32, array: *mut f64) {
    // SAFETY: caller guarantees array has at least n elements
    unsafe {
        let increment = if n > 1 { (upper - lower) / (n - 1) as f64 } else { 0.0 };
        let mut lower = lower;
        let mut ptr = array;
        for _i in 0..n {
            *ptr = lower;
            ptr = ptr.add(1);
            lower += increment;
        }
    }
}

/// C: BinEdges (user/user_mesh.cc:103)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_15FoveaEdd, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_mesh.cc:__ZN12_GLOBAL__N_18LinSpaceEddiPd, cxx:__Z13mjuu_scalevecPdPKddi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn BinEdges(x_edges: *mut f64, y_edges: *mut f64, size: *mut i32, fov: *mut f64, gamma: f64) {
    // SAFETY: caller guarantees size[0..2], fov[0..2], x_edges[0..size[0]+1], y_edges[0..size[1]+1]
    unsafe {
        let s0 = *size.add(0);
        let s1 = *size.add(1);
        LinSpace(-1.0, 1.0, s0 + 1, x_edges);
        LinSpace(-1.0, 1.0, s1 + 1, y_edges);
        for i in 0..(s0 + 1) as usize {
            *x_edges.add(i) = Fovea(*x_edges.add(i), gamma);
        }
        for i in 0..(s1 + 1) as usize {
            *y_edges.add(i) = Fovea(*y_edges.add(i), gamma);
        }
        crate::user::user_util::mjuu_scalevec(
            x_edges, x_edges, *fov.add(0) * std::f64::consts::PI / 180.0, s0 + 1,
        );
        crate::user::user_util::mjuu_scalevec(
            y_edges, y_edges, *fov.add(1) * std::f64::consts::PI / 180.0, s1 + 1,
        );
    }
}

/// C: SphericalToCartesian (user/user_mesh.cc:123)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn SphericalToCartesian(aer: *const f64, xyz: *mut f32) {
    // SAFETY: caller guarantees aer[0..3] and xyz[0..3]
    unsafe {
        let a = *aer.add(0);
        let e = *aer.add(1);
        let r = *aer.add(2);
        *xyz.add(0) = (r * e.cos() * a.sin()) as f32;
        *xyz.add(1) = (r * e.sin()) as f32;
        *xyz.add(2) = (-r * e.cos() * a.cos()) as f32;
    }
}

/// C: triangle (user/user_mesh.cc:154)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn triangle(normal: *mut f64, center: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64) -> f64 {
    const MJ_MINVAL: f64 = 1e-15;

    // SAFETY: v1, v2, v3 are valid f64[3] pointers from mujoco mesh data
    unsafe {
        let mut normal_local: [f64; 3] = [0.0; 3];
        let normal_ptr: *mut f64 = if !normal.is_null() { normal } else { normal_local.as_mut_ptr() };

        // center
        if !center.is_null() {
            *center.offset(0) = (*v1.offset(0) + *v2.offset(0) + *v3.offset(0)) / 3.0;
            *center.offset(1) = (*v1.offset(1) + *v2.offset(1) + *v3.offset(1)) / 3.0;
            *center.offset(2) = (*v1.offset(2) + *v2.offset(2) + *v3.offset(2)) / 3.0;
        }

        // normal = (v2-v1) cross (v3-v1)
        let mut b: [f64; 3] = [
            *v2.offset(0) - *v1.offset(0),
            *v2.offset(1) - *v1.offset(1),
            *v2.offset(2) - *v1.offset(2),
        ];
        let mut c: [f64; 3] = [
            *v3.offset(0) - *v1.offset(0),
            *v3.offset(1) - *v1.offset(1),
            *v3.offset(2) - *v1.offset(2),
        ];
        crate::user::user_util::mjuu_crossvec(normal_ptr, b.as_ptr(), c.as_ptr());

        // get length
        let len = f64::sqrt(crate::user::user_util::mjuu_dot3(normal_ptr, normal_ptr));

        // ignore small faces
        if len < MJ_MINVAL {
            return 0.0;
        }

        // normalize
        if !normal.is_null() {
            *normal_ptr.offset(0) /= len;
            *normal_ptr.offset(1) /= len;
            *normal_ptr.offset(2) /= len;
        }

        // return area
        0.5 * len
    }
}

/// C: quadratureGaussLegendre (user/user_mesh.cc:3727)
/// Calls: cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn quadratureGaussLegendre(points: *mut f64, weights: *mut f64, order: i32, a: f64, b: f64) {
    // Gauss-Legendre quadrature on [a,b] mapped from [-1,1].
    // SAFETY: points and weights are valid arrays sized appropriately for order.
    unsafe {
        if order > 3 {
            crate::engine::engine_util_errmem::mju_error(
                b"Integration order > 3 not yet supported.\0".as_ptr() as *const i8);
        }
        let p0 = (a + b) / 2.0;
        let dpdx = (b - a) / 2.0;
        if order == 2 {
            *points.add(0) = -dpdx / (3.0f64).sqrt() + p0;
            *points.add(1) =  dpdx / (3.0f64).sqrt() + p0;
            *weights.add(0) = dpdx;
            *weights.add(1) = dpdx;
        } else {
            *points.add(0) = p0;
            *points.add(1) = -dpdx / (3.0f64 / 5.0).sqrt() + p0;
            *points.add(2) =  dpdx / (3.0f64 / 5.0).sqrt() + p0;
            *weights.add(0) = 8.0 / 9.0 * dpdx;
            *weights.add(1) = 5.0 / 9.0 * dpdx;
            *weights.add(2) = 5.0 / 9.0 * dpdx;
        }
    }
}

/// C: phi (user/user_mesh.cc:3752)
/// Calls: cxx:_mju_error, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn phi(s: f64, i: i32, order: i32) -> f64 {
    if order == 1 {
        if i == 0 { 1.0 - s } else { s }
    } else if order == 2 {
        match i {
            0 => 2.0 * s * s - 3.0 * s + 1.0,
            1 => 4.0 * (s - s * s),
            2 => 2.0 * s * s - s,
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid index %d\0".as_ptr() as *const i8);
                0.0
            }
        }
    } else {
        crate::engine::engine_util_errmem::mju_error(
            b"Order must be 1 or 2.\0".as_ptr() as *const i8);
        0.0
    }
}

/// C: dphi (user/user_mesh.cc:3774)
/// Calls: cxx:_mju_error, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dphi(s: f64, i: i32, order: i32) -> f64 {
    if order == 1 {
        if i == 0 { -1.0 } else { 1.0 }
    } else if order == 2 {
        match i {
            0 => 4.0 * s - 3.0,
            1 => 4.0 * (1.0 - 2.0 * s),
            2 => 4.0 * s - 1.0,
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid index %d, must be 0, 1, or 2\0".as_ptr() as *const i8);
                0.0
            }
        }
    } else {
        crate::engine::engine_util_errmem::mju_error(
            b"Order must be 1 or 2.\0".as_ptr() as *const i8);
        0.0
    }
}

/// C: ComputeWarpMode (user/user_mesh.cc:4007)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ComputeWarpMode(warp: *mut f64, pos: *const f64, npe: i32, order: i32, normal_axis: i32) {
    // Uses a dynamic-size rigid matrix. Max npe in practice is ~25 (order=4).
    // ndof = 3*npe, rigid matrix = 6*ndof.
    // SAFETY: warp[3*npe], pos[3*npe] are valid (caller contract).
    unsafe {
        let npe = npe as usize;
        let ndof = 3 * npe;
        let nbasis = (order + 1) as usize;

        // zero out warp
        for k in 0..ndof { *warp.add(k) = 0.0; }

        // evaluate warp pattern at each node
        for b0 in 0..nbasis {
            for b1 in 0..nbasis {
                let node = b0 * nbasis + b1;
                let s = b0 as f64 / (nbasis as f64 - 1.0);
                let t = b1 as f64 / (nbasis as f64 - 1.0);
                *warp.add(3 * node + normal_axis as usize) = (1.0 - 2.0 * s) * (1.0 - 2.0 * t);
            }
        }

        // compute centroid
        let mut centroid = [0.0f64; 3];
        for n in 0..npe {
            for k in 0..3 {
                centroid[k] += *pos.add(3 * n + k);
            }
        }
        for k in 0..3 { centroid[k] /= npe as f64; }

        // build rigid body modes (6 * ndof)
        let mut rigid = vec![0.0f64; 6 * ndof];

        // translations
        for n in 0..npe {
            rigid[0 * ndof + 3 * n + 0] = 1.0;
            rigid[1 * ndof + 3 * n + 1] = 1.0;
            rigid[2 * ndof + 3 * n + 2] = 1.0;
        }

        // rotations about centroid
        for n in 0..npe {
            let rx = *pos.add(3 * n + 0) - centroid[0];
            let ry = *pos.add(3 * n + 1) - centroid[1];
            let rz = *pos.add(3 * n + 2) - centroid[2];
            rigid[3 * ndof + 3 * n + 1] = -rz;
            rigid[3 * ndof + 3 * n + 2] =  ry;
            rigid[4 * ndof + 3 * n + 0] =  rz;
            rigid[4 * ndof + 3 * n + 2] = -rx;
            rigid[5 * ndof + 3 * n + 0] = -ry;
            rigid[5 * ndof + 3 * n + 1] =  rx;
        }

        // orthonormalize rigid modes via modified Gram-Schmidt
        for i in 0..6 {
            let ri_start = i * ndof;
            for j in 0..i {
                let rj_start = j * ndof;
                let mut dot = 0.0f64;
                for k in 0..ndof { dot += rigid[ri_start + k] * rigid[rj_start + k]; }
                for k in 0..ndof { rigid[ri_start + k] -= dot * rigid[rj_start + k]; }
            }
            let mut norm2 = 0.0f64;
            for k in 0..ndof { norm2 += rigid[ri_start + k] * rigid[ri_start + k]; }
            if norm2 > 1e-20 {
                let inv_norm = 1.0 / norm2.sqrt();
                for k in 0..ndof { rigid[ri_start + k] *= inv_norm; }
            }
        }

        // project warp against rigid modes
        for i in 0..6 {
            let ri_start = i * ndof;
            let mut dot = 0.0f64;
            for k in 0..ndof { dot += *warp.add(k) * rigid[ri_start + k]; }
            for k in 0..ndof { *warp.add(k) -= dot * rigid[ri_start + k]; }
        }

        // normalize
        let mut norm2 = 0.0f64;
        for k in 0..ndof { norm2 += *warp.add(k) * *warp.add(k); }
        if norm2 > 1e-20 {
            let inv_norm = 1.0 / norm2.sqrt();
            for k in 0..ndof { *warp.add(k) *= inv_norm; }
        }
    }
}

/// C: ComputeWarpStiffness (user/user_mesh.cc:4104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ComputeWarpStiffness(pos: *const f64, npe: i32, normal_axis: i32, E: f64, nu: f64, thickness: f64) -> f64 {
    // SAFETY: pos is a valid f64 array of length 3*npe, npe >= 2 (caller contract).
    unsafe {
        let axis0 = ((normal_axis + 1) % 3) as usize;
        let axis1 = ((normal_axis + 2) % 3) as usize;
        let d0 = (*pos.add(3 * (npe as usize - 1) + axis0) - *pos.add(axis0)).abs();
        let d1 = (*pos.add(3 * (npe as usize - 1) + axis1) - *pos.add(axis1)).abs();
        if d0 < 1e-30 || d1 < 1e-30 { return 0.0; }
        // plate bending rigidity: D = E*t³ / (12*(1-ν²))
        let D = E * thickness * thickness * thickness / (12.0 * (1.0 - nu * nu));
        // warp stiffness: D*(1-ν)*4 / (d0*d1)
        D * (1.0 - nu) * 4.0 / (d0 * d1)
    }
}

/// C: EigendecomposeStiffness (user/user_mesh.cc:4130)
/// Calls: cxx:__Z19mjuu_eigendecomposePdS_S_i
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn EigendecomposeStiffness(K_cell_data: *const f64, out: *mut f64, ndof: i32) -> i32 {
    // SAFETY: K_cell_data[ndof*ndof], out[1+ndof*ndof] are valid (caller contract).
    unsafe {
        let ndof = ndof as usize;
        let n2 = ndof * ndof;

        // copy K_cell for in-place decomposition
        let mut mat = vec![0.0f64; n2];
        std::ptr::copy_nonoverlapping(K_cell_data, mat.as_mut_ptr(), n2);
        let mut eigval = vec![0.0f64; ndof];
        let mut eigvec = vec![0.0f64; n2];

        crate::user::user_util::mjuu_eigendecompose(mat.as_mut_ptr(), eigval.as_mut_ptr(), eigvec.as_mut_ptr(), ndof as i32);

        // K_stored = -K_physical, so physical eigenvalue = -eigval[i]
        let mut max_eigval: f64 = 0.0;
        for i in 0..ndof {
            let abs_v = eigval[i].abs();
            if abs_v > max_eigval { max_eigval = abs_v; }
        }
        let threshold = max_eigval * 1e-8;

        let mut neig: i32 = 0;
        for i in 0..ndof {
            let lambda_phys = -eigval[i];
            if lambda_phys > threshold {
                let scale = lambda_phys.sqrt();
                let w = out.add(1 + neig as usize * ndof);
                for j in 0..ndof {
                    *w.add(j) = scale * eigvec[j * ndof + i];
                }
                neig += 1;
            }
        }

        *out.add(0) = neig as f64;
        neig
    }
}

pub fn cot (x : * const f64 , v0 : i32 , v1 : i32 , v2 : i32) -> f64
{
    // SAFETY: caller guarantees x points to vertex array with valid indices v0, v1, v2
    unsafe {
        let mut normal: [f64; 3] = [0.0; 3];
        let edge1: [f64; 3] = [
            *x.add(3 * v1 as usize) - *x.add(3 * v0 as usize),
            *x.add(3 * v1 as usize + 1) - *x.add(3 * v0 as usize + 1),
            *x.add(3 * v1 as usize + 2) - *x.add(3 * v0 as usize + 2),
        ];
        let edge2: [f64; 3] = [
            *x.add(3 * v2 as usize) - *x.add(3 * v0 as usize),
            *x.add(3 * v2 as usize + 1) - *x.add(3 * v0 as usize + 1),
            *x.add(3 * v2 as usize + 2) - *x.add(3 * v0 as usize + 2),
        ];

        crate::user::user_util::mjuu_crossvec(normal.as_mut_ptr(), edge1.as_ptr(), edge2.as_ptr());
        let dot_e1e2 = crate::user::user_util::mjuu_dot3(edge1.as_ptr(), edge2.as_ptr());
        let dot_nn = crate::user::user_util::mjuu_dot3(normal.as_ptr(), normal.as_ptr());
        dot_e1e2 / dot_nn.sqrt()
    }
}

