//! Port of: engine/engine_collision_continuous.c
//! IR hash: 35fd346e6869b510
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

#[allow(unused_variables, non_snake_case)]
pub fn mjc_geom_edges(m: *const mjModel, gi: i32, gpos: *const f64, gmat: *const f64, out: *mut f64) -> i32 {
    unsafe {
        let geom_type = *(*m).geom_type.offset(gi as isize);

        if geom_type == mjtGeom_mjGEOM_BOX as i32 {
            let size = (*m).geom_size.offset(3 * gi as isize);
            let mut n = 0i32;

            for axis in 0..3i32 {
                let a1 = (axis + 1) % 3;
                let a2 = (axis + 2) % 3;

                let mut s1 = -1i32;
                while s1 <= 1 {
                    let mut s2 = -1i32;
                    while s2 <= 1 {
                        let mut lo = [0.0f64; 3];
                        let mut hi = [0.0f64; 3];
                        let mut w1 = [0.0f64; 3];
                        let mut w2 = [0.0f64; 3];

                        lo[axis as usize] = -*size.offset(axis as isize);
                        hi[axis as usize] = *size.offset(axis as isize);
                        lo[a1 as usize] = (s1 as f64) * *size.offset(a1 as isize);
                        hi[a1 as usize] = lo[a1 as usize];
                        lo[a2 as usize] = (s2 as f64) * *size.offset(a2 as isize);
                        hi[a2 as usize] = lo[a2 as usize];

                        crate::engine::engine_util_blas::mju_mul_mat_vec3(w1.as_mut_ptr(), gmat, lo.as_ptr());
                        crate::engine::engine_util_blas::mju_mul_mat_vec3(w2.as_mut_ptr(), gmat, hi.as_ptr());

                        for k in 0..3i32 {
                            *out.offset((6 * n + k) as isize) = *gpos.offset(k as isize) + w1[k as usize];
                            *out.offset((6 * n + 3 + k) as isize) = *gpos.offset(k as isize) + w2[k as usize];
                        }
                        n += 1;

                        s2 += 2;
                    }
                    s1 += 2;
                }
            }
            return n;
        }

        if geom_type == mjtGeom_mjGEOM_MESH as i32 {
            let mid = *(*m).geom_dataid.offset(gi as isize);
            let vb = (*m).mesh_vert.offset(3 * *(*m).mesh_vertadr.offset(mid as isize) as isize);
            let pa = *(*m).mesh_polyadr.offset(mid as isize);
            let pn = *(*m).mesh_polynum.offset(mid as isize);
            let mut n = 0i32;

            for p in 0..pn {
                let adr = *(*m).mesh_polyvertadr.offset((pa + p) as isize);
                let nvp = *(*m).mesh_polyvertnum.offset((pa + p) as isize);

                for j in 0..nvp {
                    let a = *(*m).mesh_polyvert.offset((adr + j) as isize);
                    let b = *(*m).mesh_polyvert.offset((adr + (j + 1) % nvp) as isize);

                    if a >= b {
                        continue;
                    }

                    let la: [f64; 3] = [
                        *vb.offset((3 * a) as isize) as f64,
                        *vb.offset((3 * a + 1) as isize) as f64,
                        *vb.offset((3 * a + 2) as isize) as f64,
                    ];
                    let lb: [f64; 3] = [
                        *vb.offset((3 * b) as isize) as f64,
                        *vb.offset((3 * b + 1) as isize) as f64,
                        *vb.offset((3 * b + 2) as isize) as f64,
                    ];
                    let mut wa = [0.0f64; 3];
                    let mut wb = [0.0f64; 3];

                    crate::engine::engine_util_blas::mju_mul_mat_vec3(wa.as_mut_ptr(), gmat, la.as_ptr());
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(wb.as_mut_ptr(), gmat, lb.as_ptr());

                    for k in 0..3i32 {
                        *out.offset((6 * n + k) as isize) = *gpos.offset(k as isize) + wa[k as usize];
                        *out.offset((6 * n + 3 + k) as isize) = *gpos.offset(k as isize) + wb[k as usize];
                    }
                    n += 1;
                }
            }
            return n;
        }

        0
    }
}

