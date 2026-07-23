//! Port of: engine/engine_collision_box.c
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_clampVec (engine/engine_collision_box.c:23)
/// Calls: mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_clamp_vec(vec: *mut f64, limit: *const f64, n: i32) {
    use crate::engine::engine_util_misc::{mju_max, mju_min};
    // SAFETY: caller guarantees vec has at least n elements, limit has at least 2*n elements
    unsafe {
        for i in 0..n as usize {
            *vec.add(i) = mju_max(*limit.add(2 * i), mju_min(*limit.add(2 * i + 1), *vec.add(i)));
        }
    }
}

/// C: mjraw_SphereBox (engine/engine_collision_box.c:34)
/// Calls: mji_add3, mji_addToScl3, mji_copy3, mji_mulMatTVec3, mji_mulMatVec3, mji_sub3, mji_zero3, mju_clampVec, mju_normalize3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_box(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    const MJ_MINVAL: f64 = 1E-15;
    // SAFETY: caller guarantees all pointers are valid and arrays properly sized
    unsafe {
        let mut tmp: [f64; 3] = [0.0; 3];
        let mut center: [f64; 3] = [0.0; 3];
        let mut clamped: [f64; 3] = [0.0; 3];
        let mut deepest: [f64; 3] = [0.0; 3];
        let mut pos: [f64; 3] = [0.0; 3];

        crate::engine::engine_inline::mji_sub3(tmp.as_mut_ptr(), pos1, pos2);
        crate::engine::engine_inline::mji_mul_mat_t_vec3(center.as_mut_ptr(), mat2, tmp.as_ptr());

        crate::engine::engine_inline::mji_copy3(clamped.as_mut_ptr(), center.as_ptr());
        mju_clamp_vec(clamped.as_mut_ptr(), size2, 3);

        crate::engine::engine_inline::mji_copy3(deepest.as_mut_ptr(), center.as_ptr());
        crate::engine::engine_inline::mji_sub3(tmp.as_mut_ptr(), clamped.as_ptr(), center.as_ptr());
        let mut dist = crate::engine::engine_util_blas::mju_normalize3(tmp.as_mut_ptr());

        if dist - *size1.add(0) > margin {
            return 0;
        }

        // sphere center inside box
        if dist <= MJ_MINVAL {
            let mut closest = (*size2.add(0) + *size2.add(1) + *size2.add(2)) * 2.0;
            let mut k: usize = 0;

            for i in 0..6usize {
                let sign: f64 = if i % 2 != 0 { 1.0 } else { -1.0 };
                let val = (sign * *size2.add(i / 2) - center[i / 2]).abs();
                if closest > val {
                    closest = val;
                    k = i;
                }
            }

            let mut nearest: [f64; 3] = [0.0; 3];
            nearest[k / 2] = if k % 2 != 0 { -1.0 } else { 1.0 };

            crate::engine::engine_inline::mji_copy3(pos.as_mut_ptr(), center.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(pos.as_mut_ptr(), nearest.as_ptr(), (*size1.add(0) - closest) / 2.0);
            crate::engine::engine_inline::mji_mul_mat_vec3((*con).normal.as_mut_ptr(), mat2, nearest.as_ptr());
            dist = -closest;
        } else {
            crate::engine::engine_inline::mji_add_to_scl3(deepest.as_mut_ptr(), tmp.as_ptr(), *size1.add(0));
            crate::engine::engine_util_blas::mju_zero3(pos.as_mut_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(pos.as_mut_ptr(), clamped.as_ptr(), 0.5);
            crate::engine::engine_inline::mji_add_to_scl3(pos.as_mut_ptr(), deepest.as_ptr(), 0.5);
            crate::engine::engine_inline::mji_mul_mat_vec3((*con).normal.as_mut_ptr(), mat2, tmp.as_ptr());
        }

        crate::engine::engine_inline::mji_mul_mat_vec3(tmp.as_mut_ptr(), mat2, pos.as_ptr());
        crate::engine::engine_inline::mji_add3((*con).pos.as_mut_ptr(), tmp.as_ptr(), pos2);
        (*con).dist = dist - *size1.add(0);
        crate::engine::engine_inline::mji_zero3((*con).tangent.as_mut_ptr());
        1
    }
}

/// C: _boxbox (engine/engine_collision_box.c:605)
/// Calls: mji_add3, mji_addTo3, mji_addToScl3, mji_copy3, mji_mulMatTVec3, mji_mulMatVec3, mji_scl3, mji_sub3, mji_zero3, mju_copy3, mju_dot3, mju_mulMatMatT3, mju_mulMatTMat3, mju_normalize3, mju_scl3, mju_transpose, mju_zero, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn boxbox(M: *const mjModel, D: *const mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    const MJ_MAXCONPAIR: usize = 50;
    const MJ_MINVAL: f64 = 1E-15;
    // SAFETY: caller guarantees M, D, con valid
    unsafe {
        let pos1 = (*D).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*D).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*D).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*D).geom_xmat.add(9 * g2 as usize);
        let size1 = (*M).geom_size.add(3 * g1 as usize);
        let size2 = (*M).geom_size.add(3 * g2 as usize);

        let mut pos12: [f64; 3] = [0.0; 3];
        let mut pos21: [f64; 3] = [0.0; 3];
        let mut rot: [f64; 9] = [0.0; 9];
        let mut rott: [f64; 9] = [0.0; 9];
        let mut rotabs: [f64; 9] = [0.0; 9];
        let mut rottabs: [f64; 9] = [0.0; 9];
        let mut tmp1: [f64; 3] = [0.0; 3];
        let mut tmp2: [f64; 3] = [0.0; 3];
        let mut plen1: [f64; 3] = [0.0; 3];
        let mut plen2: [f64; 3] = [0.0; 3];
        let mut rotmore: [f64; 9] = [0.0; 9];
        let mut p: [f64; 3] = [0.0; 3];
        let mut r: [f64; 9] = [0.0; 9];
        let mut s: [f64; 3] = [0.0; 3];
        let mut ss: [f64; 3] = [0.0; 3];
        let mut lp: [f64; 3] = [0.0; 3];
        let mut rt: [f64; 9] = [0.0; 9];
        let mut points: [[f64; 3]; MJ_MAXCONPAIR] = [[0.0; 3]; MJ_MAXCONPAIR];
        let mut depth: [f64; MJ_MAXCONPAIR] = [0.0; MJ_MAXCONPAIR];
        let mut pts: [[f64; 3]; 6] = [[0.0; 3]; 6];
        let mut ppts2: [[f64; 2]; 4] = [[0.0; 2]; 4];
        let mut pu: [[f64; 3]; 4] = [[0.0; 3]; 4];
        let mut axi: [[f64; 3]; 3] = [[0.0; 3]; 3];
        let mut linesu: [[f64; 6]; 4] = [[0.0; 6]; 4];
        let mut lines: [[f64; 6]; 4] = [[0.0; 6]; 4];
        let mut clnorm: [f64; 3] = [0.0; 3];
        let mut rnorm: [f64; 3] = [0.0; 3];

        let mut n: i32 = 0;
        let mut code: i32 = -1;
        let margin2 = margin * margin;
        let mut cle1: i32 = 0;
        let mut cle2: i32 = 0;
        let mut in_flag: i32 = 0;

        crate::engine::engine_inline::mji_sub3(tmp1.as_mut_ptr(), pos2, pos1);
        crate::engine::engine_inline::mji_mul_mat_t_vec3(pos21.as_mut_ptr(), mat1, tmp1.as_ptr());

        crate::engine::engine_inline::mji_sub3(tmp1.as_mut_ptr(), pos1, pos2);
        crate::engine::engine_inline::mji_mul_mat_t_vec3(pos12.as_mut_ptr(), mat2, tmp1.as_ptr());

        crate::engine::engine_util_blas::mju_mul_mat_t_mat3(rot.as_mut_ptr(), mat1, mat2);
        crate::engine::engine_util_blas::mju_transpose(rott.as_mut_ptr(), rot.as_ptr(), 3, 3);

        for i in 0..9 { rotabs[i] = rot[i].abs(); }
        for i in 0..9 { rottabs[i] = rott[i].abs(); }

        crate::engine::engine_inline::mji_mul_mat_vec3(plen2.as_mut_ptr(), rotabs.as_ptr(), size2);
        crate::engine::engine_inline::mji_mul_mat_t_vec3(plen1.as_mut_ptr(), rotabs.as_ptr(), size1);

        let mut penetration: f64 = margin;
        for i in 0..3usize {
            penetration += *size1.add(i) * 3.0 + *size2.add(i) * 3.0;
        }

        for i in 0..3usize {
            let c1 = -pos21[i].abs() + *size1.add(i) + plen2[i];
            let c2 = -pos12[i].abs() + *size2.add(i) + plen1[i];

            if c1 < -margin || c2 < -margin { return 0; }

            if c1 < penetration {
                penetration = c1;
                code = i as i32 + 3 * (pos21[i] < 0.0) as i32;
            }
            if c2 < penetration {
                penetration = c2;
                code = i as i32 + 3 * (pos12[i] < 0.0) as i32 + 6;
            }
        }

        // edge-edge SAT tests
        for i in 0..3i32 {
            for j in 0..3i32 {
                crate::engine::engine_util_blas::mju_zero3(tmp2.as_mut_ptr());
                if i == 0 { tmp2[1] = -rott[3*j as usize+2]; tmp2[2] = rott[3*j as usize+1]; }
                else if i == 1 { tmp2[0] = rott[3*j as usize+2]; tmp2[2] = -rott[3*j as usize]; }
                else { tmp2[0] = -rott[3*j as usize+1]; tmp2[1] = rott[3*j as usize]; }

                let c1_norm = crate::engine::engine_util_blas::mju_normalize3(tmp2.as_mut_ptr());
                if c1_norm < MJ_MINVAL { continue; }

                let c2_val = crate::engine::engine_util_blas::mju_dot3(pos21.as_ptr(), tmp2.as_ptr());

                let mut c3: f64 = 0.0;
                for k in 0..3i32 {
                    if k != i { c3 += *size1.add(k as usize) * tmp2[k as usize].abs(); }
                }
                for k in 0..3i32 {
                    if k != j { c3 += *size2.add(k as usize) * rotabs[3*i as usize + 3 - k as usize - j as usize] / c1_norm; }
                }
                c3 -= c2_val.abs();

                if c3 < -margin { return 0; }

                if c3 < penetration * (1.0 - 1e-12) {
                    penetration = c3;
                    cle1 = 0;
                    for k in 0..3i32 {
                        if k != i {
                            if (tmp2[k as usize] > 0.0) ^ (c2_val < 0.0) { cle1 += 1 << k; }
                        }
                    }
                    cle2 = 0;
                    for k in 0..3i32 {
                        if k != j {
                            if (rot[3*i as usize + 3 - k as usize - j as usize] > 0.0) ^ (c2_val < 0.0) ^ ((k - j + 3) % 3 == 1) { cle2 += 1 << k; }
                        }
                    }
                    code = 12 + i * 3 + j;
                    crate::engine::engine_inline::mji_copy3(clnorm.as_mut_ptr(), tmp2.as_ptr());
                    in_flag = (c2_val < 0.0) as i32;
                }
            }
        }

        if code == -1 { return 0; }

        // The C code uses goto edgeedge when code >= 12.
        // We handle both paths inline with an if/else.
        if code < 12 {
            // FACE-FACE path
            let q1 = code % 6;
            let q2 = code / 6;

            crate::engine::engine_util_blas::mju_zero(rotmore.as_mut_ptr(), 9);
            match q1 {
                0 => { rotmore[2] = -1.0; rotmore[4] = 1.0; rotmore[6] = 1.0; }
                1 => { rotmore[0] = 1.0; rotmore[5] = -1.0; rotmore[7] = 1.0; }
                2 => { rotmore[0] = 1.0; rotmore[4] = 1.0; rotmore[8] = 1.0; }
                3 => { rotmore[2] = 1.0; rotmore[4] = 1.0; rotmore[6] = -1.0; }
                4 => { rotmore[0] = 1.0; rotmore[5] = 1.0; rotmore[7] = -1.0; }
                5 => { rotmore[0] = -1.0; rotmore[4] = 1.0; rotmore[8] = -1.0; }
                _ => {}
            }

            let (i0, i1, i2, f0, f1, f2): (usize, usize, usize, f64, f64, f64) = match q1 {
                0 => (2, 1, 0, -1.0, 1.0, 1.0),
                1 => (0, 2, 1, 1.0, -1.0, 1.0),
                2 => (0, 1, 2, 1.0, 1.0, 1.0),
                3 => (2, 1, 0, 1.0, 1.0, -1.0),
                4 => (0, 2, 1, 1.0, 1.0, -1.0),
                5 => (0, 1, 2, -1.0, 1.0, -1.0),
                _ => (0, 1, 2, 1.0, 1.0, 1.0),
            };

            // rotaxis macro: vecres[0]=vecin[i0]*f0, etc.
            macro_rules! rotaxis { ($res:expr, $inp:expr) => { $res[0]=$inp[i0]*f0; $res[1]=$inp[i1]*f1; $res[2]=$inp[i2]*f2; } }
            // rotmatx macro
            macro_rules! rotmatx { ($res:expr, $inp:expr) => {
                crate::engine::engine_inline::mji_scl3(($res).as_mut_ptr().add(0), ($inp).as_ptr().add(i0*3), f0);
                crate::engine::engine_inline::mji_scl3(($res).as_mut_ptr().add(3), ($inp).as_ptr().add(i1*3), f1);
                crate::engine::engine_inline::mji_scl3(($res).as_mut_ptr().add(6), ($inp).as_ptr().add(i2*3), f2);
            } }

            if q2 != 0 {
                crate::engine::engine_util_blas::mju_mul_mat_mat_t3(r.as_mut_ptr(), rotmore.as_ptr(), rot.as_ptr());
                rotaxis!(p, pos12);
                rotaxis!(tmp1, std::slice::from_raw_parts(size2, 3));
                crate::engine::engine_inline::mji_copy3(s.as_mut_ptr(), size1);
            } else {
                rotmatx!(r, rot);
                rotaxis!(p, pos21);
                rotaxis!(tmp1, std::slice::from_raw_parts(size1, 3));
                crate::engine::engine_inline::mji_copy3(s.as_mut_ptr(), size2);
            }

            crate::engine::engine_util_blas::mju_transpose(rt.as_mut_ptr(), r.as_ptr(), 3, 3);

            for i in 0..3usize { ss[i] = tmp1[i].abs(); }

            let lx = ss[0];
            let ly = ss[1];
            let hz = ss[2];
            p[2] -= hz;
            crate::engine::engine_inline::mji_copy3(lp.as_mut_ptr(), p.as_ptr());

            let mut clcorner: i32 = 0;
            for i in 0..3usize { if r[6+i] < 0.0 { clcorner += 1 << i; } }

            crate::engine::engine_inline::mji_add_to_scl3(lp.as_mut_ptr(), rt.as_ptr().add(0), s[0] * if clcorner & 1 != 0 { 1.0 } else { -1.0 });
            crate::engine::engine_inline::mji_add_to_scl3(lp.as_mut_ptr(), rt.as_ptr().add(3), s[1] * if clcorner & 2 != 0 { 1.0 } else { -1.0 });
            crate::engine::engine_inline::mji_add_to_scl3(lp.as_mut_ptr(), rt.as_ptr().add(6), s[2] * if clcorner & 4 != 0 { 1.0 } else { -1.0 });

            let mut m_count: i32 = 0;
            let mut k_count: i32 = 0;
            crate::engine::engine_inline::mji_copy3(pts[m_count as usize].as_mut_ptr(), lp.as_ptr());
            m_count += 1;

            for i in 0..3usize {
                if r[6+i].abs() < 0.5 {
                    crate::engine::engine_util_blas::mju_scl3(pts[m_count as usize].as_mut_ptr(), rt.as_ptr().add(3*i), s[i] * if clcorner & (1 << i) != 0 { -2.0 } else { 2.0 });
                    m_count += 1;
                }
            }

            crate::engine::engine_inline::mji_add3(pts[3].as_mut_ptr(), pts[0].as_ptr(), pts[1].as_ptr());
            crate::engine::engine_inline::mji_add3(pts[4].as_mut_ptr(), pts[0].as_ptr(), pts[2].as_ptr());
            crate::engine::engine_inline::mji_add3(pts[5].as_mut_ptr(), pts[3].as_ptr(), pts[2].as_ptr());

            if m_count > 1 {
                crate::engine::engine_inline::mji_copy3(lines[k_count as usize].as_mut_ptr().add(0), pts[0].as_ptr());
                crate::engine::engine_inline::mji_copy3(lines[k_count as usize].as_mut_ptr().add(3), pts[1].as_ptr());
                k_count += 1;
            }
            if m_count > 2 {
                crate::engine::engine_inline::mji_copy3(lines[k_count as usize].as_mut_ptr().add(0), pts[0].as_ptr());
                crate::engine::engine_inline::mji_copy3(lines[k_count as usize].as_mut_ptr().add(3), pts[2].as_ptr());
                k_count += 1;
                crate::engine::engine_inline::mji_copy3(lines[k_count as usize].as_mut_ptr().add(0), pts[3].as_ptr());
                crate::engine::engine_inline::mji_copy3(lines[k_count as usize].as_mut_ptr().add(3), pts[2].as_ptr());
                k_count += 1;
                crate::engine::engine_inline::mji_copy3(lines[k_count as usize].as_mut_ptr().add(0), pts[4].as_ptr());
                crate::engine::engine_inline::mji_copy3(lines[k_count as usize].as_mut_ptr().add(3), pts[1].as_ptr());
                k_count += 1;
            }

            // clip lines against box face
            for i in 0..k_count as usize {
                for q in 0..2usize {
                    let a = lines[i][0+q];
                    let b = lines[i][3+q];
                    let c_val = lines[i][1-q];
                    let d_val = lines[i][4-q];

                    if b.abs() > MJ_MINVAL {
                        for jj in [-1i32, 1].iter() {
                            let l = ss[q] * *jj as f64;
                            let c1 = (l - a) / b;
                            if c1 < 0.0 || c1 > 1.0 { continue; }
                            let c2 = c_val + d_val * c1;
                            if c2.abs() > ss[1-q] { continue; }
                            if (n as usize) < MJ_MAXCONPAIR {
                                crate::engine::engine_inline::mji_copy3(points[n as usize].as_mut_ptr(), lines[i].as_ptr());
                                crate::engine::engine_inline::mji_add_to_scl3(points[n as usize].as_mut_ptr(), lines[i].as_ptr().add(3), c1);
                                n += 1;
                            }
                        }
                    }
                }
            }

            // interior points
            let a = pts[1][0]; let b = pts[2][0]; let c_val = pts[1][1]; let d_val = pts[2][1];
            let c1_det = a * d_val - b * c_val;

            if m_count > 2 {
                for i in 0..4usize {
                    let llx = if i / 2 != 0 { lx } else { -lx };
                    let lly = if i % 2 != 0 { ly } else { -ly };
                    let x = llx - pts[0][0];
                    let y = lly - pts[0][1];
                    let u = (x * d_val - y * b) / c1_det;
                    let v = (y * a - x * c_val) / c1_det;
                    if u <= 0.0 || v <= 0.0 || u >= 1.0 || v >= 1.0 { continue; }
                    if (n as usize) < MJ_MAXCONPAIR {
                        points[n as usize][0] = llx;
                        points[n as usize][1] = lly;
                        points[n as usize][2] = pts[0][2] + u * pts[1][2] + v * pts[2][2];
                        n += 1;
                    }
                }
            }

            for i in 0..(1 << (m_count - 1)) as usize {
                crate::engine::engine_inline::mji_copy3(tmp1.as_mut_ptr(), pts[if i == 0 { 0 } else { i + 2 }].as_ptr());
                if i != 0 {
                    if tmp1[0] <= -lx || tmp1[0] >= lx { continue; }
                    if tmp1[1] <= -ly || tmp1[1] >= ly { continue; }
                }
                if (n as usize) < MJ_MAXCONPAIR {
                    crate::engine::engine_inline::mji_copy3(points[n as usize].as_mut_ptr(), tmp1.as_ptr());
                    n += 1;
                }
            }

            // filter by margin
            let m_pts = n;
            n = 0;
            for i in 0..m_pts as usize {
                if points[i][2] > margin { continue; }
                if n as usize != i { crate::engine::engine_inline::mji_copy3(points[n as usize].as_mut_ptr(), points[i].as_ptr()); }
                depth[n as usize] = points[n as usize][2];
                points[n as usize][2] *= 0.5;
                n += 1;
            }

            // transform to global frame
            crate::engine::engine_util_blas::mju_mul_mat_mat_t3(r.as_mut_ptr(), if q2 != 0 { mat2 } else { mat1 }, rotmore.as_ptr());
            crate::engine::engine_util_blas::mju_copy3(p.as_mut_ptr(), if q2 != 0 { pos2 } else { pos1 });

            tmp2[0] = (if q2 != 0 { -1.0 } else { 1.0 }) * r[2];
            tmp2[1] = (if q2 != 0 { -1.0 } else { 1.0 }) * r[5];
            tmp2[2] = (if q2 != 0 { -1.0 } else { 1.0 }) * r[8];

            crate::engine::engine_inline::mji_copy3((*con).normal.as_mut_ptr(), tmp2.as_ptr());
            crate::engine::engine_inline::mji_zero3((*con).tangent.as_mut_ptr());

            for i in 0..n as usize {
                (*con.add(i)).dist = 2.0 * points[i][2];
                points[i][2] += hz;
                crate::engine::engine_inline::mji_mul_mat_vec3(tmp2.as_mut_ptr(), r.as_ptr(), points[i].as_ptr());
                crate::engine::engine_inline::mji_add3((*con.add(i)).pos.as_mut_ptr(), tmp2.as_ptr(), p.as_ptr());
                if i > 0 {
                    crate::engine::engine_inline::mji_copy3((*con.add(i)).normal.as_mut_ptr(), (*con).normal.as_ptr());
                    crate::engine::engine_inline::mji_zero3((*con.add(i)).tangent.as_mut_ptr());
                }
            }

            return n;
        }

        // EDGE-EDGE path (code >= 12)
        // This is the edgeedge label in C
        let code = code - 12;
        let q1 = code / 3;
        let q2_ee = code % 3;

        let (mut ax1, mut ax2): (usize, usize) = match q2_ee {
            0 => (1, 2), 1 => (0, 2), _ => (1, 0),
        };
        let (mut pax1, mut pax2): (usize, usize) = match q1 {
            0 => (1, 2), 1 => (0, 2), _ => (1, 0),
        };

        if rotabs[3*q1 as usize + ax1] < rotabs[3*q1 as usize + ax2] {
            ax1 = ax2; ax2 = 3 - q2_ee as usize - ax1;
        }
        if rottabs[3*q2_ee as usize + pax1] < rottabs[3*q2_ee as usize + pax2] {
            pax1 = pax2; pax2 = 3 - q1 as usize - pax1;
        }

        let clface: i32 = if cle1 & (1 << pax2 as i32) != 0 { pax2 as i32 } else { pax2 as i32 + 3 };

        crate::engine::engine_util_blas::mju_zero(rotmore.as_mut_ptr(), 9);
        match clface {
            0 => { rotmore[2] = -1.0; rotmore[4] = 1.0; rotmore[6] = 1.0; }
            1 => { rotmore[0] = 1.0; rotmore[5] = -1.0; rotmore[7] = 1.0; }
            2 => { rotmore[0] = 1.0; rotmore[4] = 1.0; rotmore[8] = 1.0; }
            3 => { rotmore[2] = 1.0; rotmore[4] = 1.0; rotmore[6] = -1.0; }
            4 => { rotmore[0] = 1.0; rotmore[5] = 1.0; rotmore[7] = -1.0; }
            5 => { rotmore[0] = -1.0; rotmore[4] = 1.0; rotmore[8] = -1.0; }
            _ => {}
        }

        let (i0, i1, i2, f0, f1, f2): (usize, usize, usize, f64, f64, f64) = match clface {
            0 => (2, 1, 0, -1.0, 1.0, 1.0),
            1 => (0, 2, 1, 1.0, -1.0, 1.0),
            2 => (0, 1, 2, 1.0, 1.0, 1.0),
            3 => (2, 1, 0, 1.0, 1.0, -1.0),
            4 => (0, 2, 1, 1.0, 1.0, -1.0),
            5 => (0, 1, 2, -1.0, 1.0, -1.0),
            _ => (0, 1, 2, 1.0, 1.0, 1.0),
        };

        // rotaxis: p from pos21, rnorm from clnorm
        p[0] = pos21[i0]*f0; p[1] = pos21[i1]*f1; p[2] = pos21[i2]*f2;
        rnorm[0] = clnorm[i0]*f0; rnorm[1] = clnorm[i1]*f1; rnorm[2] = clnorm[i2]*f2;

        // rotmatx: r from rot
        crate::engine::engine_inline::mji_scl3(r.as_mut_ptr().add(0), rot.as_ptr().add(i0*3), f0);
        crate::engine::engine_inline::mji_scl3(r.as_mut_ptr().add(3), rot.as_ptr().add(i1*3), f1);
        crate::engine::engine_inline::mji_scl3(r.as_mut_ptr().add(6), rot.as_ptr().add(i2*3), f2);

        crate::engine::engine_inline::mji_mul_mat_t_vec3(tmp1.as_mut_ptr(), rotmore.as_ptr(), size1);
        for i in 0..3usize { s[i] = tmp1[i].abs(); }

        crate::engine::engine_util_blas::mju_transpose(rt.as_mut_ptr(), r.as_ptr(), 3, 3);

        let lx = s[0]; let ly = s[1]; let hz = s[2];
        p[2] -= hz;

        n = 0;
        // Build edge segment points
        crate::engine::engine_inline::mji_copy3(points[0].as_mut_ptr(), p.as_ptr());
        crate::engine::engine_inline::mji_add_to_scl3(points[0].as_mut_ptr(), rt.as_ptr().add(3*ax1), *size2.add(ax1) * if cle2 & (1 << ax1 as i32) != 0 { 1.0 } else { -1.0 });
        crate::engine::engine_inline::mji_add_to_scl3(points[0].as_mut_ptr(), rt.as_ptr().add(3*ax2), *size2.add(ax2) * if cle2 & (1 << ax2 as i32) != 0 { 1.0 } else { -1.0 });
        crate::engine::engine_inline::mji_copy3(points[1].as_mut_ptr(), points[0].as_ptr());
        crate::engine::engine_inline::mji_add_to_scl3(points[0].as_mut_ptr(), rt.as_ptr().add(3*q2_ee as usize), *size2.add(q2_ee as usize));
        crate::engine::engine_inline::mji_add_to_scl3(points[1].as_mut_ptr(), rt.as_ptr().add(3*q2_ee as usize), -*size2.add(q2_ee as usize));

        crate::engine::engine_inline::mji_copy3(points[2].as_mut_ptr(), p.as_ptr());
        crate::engine::engine_inline::mji_add_to_scl3(points[2].as_mut_ptr(), rt.as_ptr().add(3*ax1), *size2.add(ax1) * if cle2 & (1 << ax1 as i32) != 0 { -1.0 } else { 1.0 });
        crate::engine::engine_inline::mji_add_to_scl3(points[2].as_mut_ptr(), rt.as_ptr().add(3*ax2), *size2.add(ax2) * if cle2 & (1 << ax2 as i32) != 0 { 1.0 } else { -1.0 });
        crate::engine::engine_inline::mji_copy3(points[3].as_mut_ptr(), points[2].as_ptr());
        crate::engine::engine_inline::mji_add_to_scl3(points[2].as_mut_ptr(), rt.as_ptr().add(3*q2_ee as usize), *size2.add(q2_ee as usize));
        crate::engine::engine_inline::mji_add_to_scl3(points[3].as_mut_ptr(), rt.as_ptr().add(3*q2_ee as usize), -*size2.add(q2_ee as usize));

        crate::engine::engine_inline::mji_copy3(axi[0].as_mut_ptr(), points[0].as_ptr());
        crate::engine::engine_inline::mji_sub3(axi[1].as_mut_ptr(), points[1].as_ptr(), points[0].as_ptr());
        crate::engine::engine_inline::mji_sub3(axi[2].as_mut_ptr(), points[2].as_ptr(), points[0].as_ptr());

        if rnorm[2].abs() < MJ_MINVAL { return 0; }
        let innorm = (1.0 / rnorm[2]) * if in_flag != 0 { -1.0 } else { 1.0 };

        for i in 0..4usize {
            let c1 = -points[i][2] / rnorm[2];
            crate::engine::engine_inline::mji_copy3(pu[i].as_mut_ptr(), points[i].as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(points[i].as_mut_ptr(), rnorm.as_ptr(), c1);
            ppts2[i][0] = points[i][0];
            ppts2[i][1] = points[i][1];
        }

        crate::engine::engine_inline::mji_copy3(pts[0].as_mut_ptr(), points[0].as_ptr());
        crate::engine::engine_inline::mji_sub3(pts[1].as_mut_ptr(), points[1].as_ptr(), points[0].as_ptr());
        crate::engine::engine_inline::mji_sub3(pts[2].as_mut_ptr(), points[2].as_ptr(), points[0].as_ptr());

        let m_count: i32 = 3;
        let mut k_count: i32 = 0;
        n = 0;

        // build lines (always m_count > 2 for edge-edge)
        crate::engine::engine_inline::mji_copy3(lines[0].as_mut_ptr().add(0), pts[0].as_ptr());
        crate::engine::engine_inline::mji_copy3(lines[0].as_mut_ptr().add(3), pts[1].as_ptr());
        crate::engine::engine_inline::mji_copy3(linesu[0].as_mut_ptr().add(0), axi[0].as_ptr());
        crate::engine::engine_inline::mji_copy3(linesu[0].as_mut_ptr().add(3), axi[1].as_ptr());
        k_count += 1;

        crate::engine::engine_inline::mji_copy3(lines[1].as_mut_ptr().add(0), pts[0].as_ptr());
        crate::engine::engine_inline::mji_copy3(lines[1].as_mut_ptr().add(3), pts[2].as_ptr());
        crate::engine::engine_inline::mji_copy3(linesu[1].as_mut_ptr().add(0), axi[0].as_ptr());
        crate::engine::engine_inline::mji_copy3(linesu[1].as_mut_ptr().add(3), axi[2].as_ptr());
        k_count += 1;

        let mut tmp_line: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_add3(tmp_line.as_mut_ptr(), pts[0].as_ptr(), pts[1].as_ptr());
        crate::engine::engine_inline::mji_copy3(lines[2].as_mut_ptr().add(0), tmp_line.as_ptr());
        crate::engine::engine_inline::mji_copy3(lines[2].as_mut_ptr().add(3), pts[2].as_ptr());
        crate::engine::engine_inline::mji_add3(tmp_line.as_mut_ptr(), axi[0].as_ptr(), axi[1].as_ptr());
        crate::engine::engine_inline::mji_copy3(linesu[2].as_mut_ptr().add(0), tmp_line.as_ptr());
        crate::engine::engine_inline::mji_copy3(linesu[2].as_mut_ptr().add(3), axi[2].as_ptr());
        k_count += 1;

        crate::engine::engine_inline::mji_add3(tmp_line.as_mut_ptr(), pts[0].as_ptr(), pts[2].as_ptr());
        crate::engine::engine_inline::mji_copy3(lines[3].as_mut_ptr().add(0), tmp_line.as_ptr());
        crate::engine::engine_inline::mji_copy3(lines[3].as_mut_ptr().add(3), pts[1].as_ptr());
        crate::engine::engine_inline::mji_add3(tmp_line.as_mut_ptr(), axi[0].as_ptr(), axi[2].as_ptr());
        crate::engine::engine_inline::mji_copy3(linesu[3].as_mut_ptr().add(0), tmp_line.as_ptr());
        crate::engine::engine_inline::mji_copy3(linesu[3].as_mut_ptr().add(3), axi[1].as_ptr());
        k_count += 1;

        // clip lines against face
        for i in 0..k_count as usize {
            for q in 0..2usize {
                let a = lines[i][0+q]; let b = lines[i][3+q];
                let c_val = lines[i][1-q]; let d_val = lines[i][4-q];
                if b.abs() > MJ_MINVAL {
                    for jj in [-1i32, 1].iter() {
                        if (n as usize) < MJ_MAXCONPAIR {
                            let l = s[q] * *jj as f64;
                            let c1 = (l - a) / b;
                            if c1 < 0.0 || c1 > 1.0 { continue; }
                            let c2 = c_val + d_val * c1;
                            if c2.abs() > s[1-q] { continue; }
                            if (linesu[i][2] + linesu[i][5] * c1) * innorm > margin { continue; }

                            crate::engine::engine_inline::mji_scl3(points[n as usize].as_mut_ptr(), linesu[i].as_ptr(), 0.5);
                            crate::engine::engine_inline::mji_add_to_scl3(points[n as usize].as_mut_ptr(), linesu[i].as_ptr().add(3), 0.5 * c1);
                            points[n as usize][0+q] += 0.5 * l;
                            points[n as usize][1-q] += 0.5 * c2;
                            depth[n as usize] = points[n as usize][2] * innorm * 2.0;
                            n += 1;
                        }
                    }
                }
            }
        }

        let nl = n;

        // face corners
        let a = pts[1][0]; let b = pts[2][0]; let c_val = pts[1][1]; let d_val = pts[2][1];
        let c1_det = a * d_val - b * c_val;

        for i in 0..4usize {
            if (n as usize) < MJ_MAXCONPAIR {
                let llx = if i / 2 != 0 { lx } else { -lx };
                let lly = if i % 2 != 0 { ly } else { -ly };
                let x = llx - pts[0][0]; let y = lly - pts[0][1];
                let u = (x * d_val - y * b) / c1_det;
                let v = (y * a - x * c_val) / c1_det;

                if nl == 0 {
                    if (u < 0.0 || u > 1.0) && (v < 0.0 || v > 1.0) { continue; }
                } else {
                    if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 { continue; }
                }
                let u = if u < 0.0 { 0.0 } else if u > 1.0 { 1.0 } else { u };
                let v = if v < 0.0 { 0.0 } else if v > 1.0 { 1.0 } else { v };

                crate::engine::engine_inline::mji_scl3(tmp1.as_mut_ptr(), pu[0].as_ptr(), 1.0 - u - v);
                crate::engine::engine_inline::mji_add_to_scl3(tmp1.as_mut_ptr(), pu[1].as_ptr(), u);
                crate::engine::engine_inline::mji_add_to_scl3(tmp1.as_mut_ptr(), pu[2].as_ptr(), v);

                points[n as usize][0] = llx; points[n as usize][1] = lly; points[n as usize][2] = 0.0;
                crate::engine::engine_inline::mji_sub3(tmp2.as_mut_ptr(), points[n as usize].as_ptr(), tmp1.as_ptr());
                let c1_sq = crate::engine::engine_util_blas::mju_dot3(tmp2.as_ptr(), tmp2.as_ptr());
                if tmp1[2] > 0.0 { if c1_sq > margin2 { continue; } }

                crate::engine::engine_inline::mji_add_to3(points[n as usize].as_mut_ptr(), tmp1.as_ptr());
                crate::engine::engine_util_blas::mju_scl3(points[n as usize].as_mut_ptr(), points[n as usize].as_ptr(), 0.5);
                depth[n as usize] = c1_sq.sqrt() * if tmp1[2] < 0.0 { -1.0 } else { 1.0 };
                n += 1;
            }
        }

        let nf = n;

        // edge segment vertices
        for i in 0..4usize {
            if (n as usize) < MJ_MAXCONPAIR {
                let x = ppts2[i][0]; let y = ppts2[i][1];
                if nl == 0 {
                    if nf != 0 {
                        if (x < -lx || x > lx) && (y < -ly || y > ly) { continue; }
                    }
                } else {
                    if x < -lx || x > lx || y < -ly || y > ly { continue; }
                }

                let mut c1_sq: f64 = 0.0;
                for j in 0..2usize {
                    if ppts2[i][j] < -s[j] { c1_sq += (ppts2[i][j] + s[j]) * (ppts2[i][j] + s[j]); }
                    else if ppts2[i][j] > s[j] { c1_sq += (ppts2[i][j] - s[j]) * (ppts2[i][j] - s[j]); }
                }
                c1_sq += pu[i][2] * innorm * pu[i][2] * innorm;
                if pu[i][2] > 0.0 { if c1_sq > margin2 { continue; } }

                tmp1[0] = ppts2[i][0] * 0.5; tmp1[1] = ppts2[i][1] * 0.5; tmp1[2] = 0.0;
                for j in 0..2usize {
                    if ppts2[i][j] < -s[j] { tmp1[j] = -s[j] * 0.5; }
                    else if ppts2[i][j] > s[j] { tmp1[j] = s[j] * 0.5; }
                }
                crate::engine::engine_inline::mji_add_to_scl3(tmp1.as_mut_ptr(), pu[i].as_ptr(), 0.5);
                crate::engine::engine_inline::mji_copy3(points[n as usize].as_mut_ptr(), tmp1.as_ptr());
                depth[n as usize] = c1_sq.sqrt() * if pu[i][2] < 0.0 { -1.0 } else { 1.0 };
                n += 1;
            }
        }

        // transform to global
        crate::engine::engine_util_blas::mju_mul_mat_mat_t3(r.as_mut_ptr(), mat1, rotmore.as_ptr());
        crate::engine::engine_inline::mji_mul_mat_vec3(tmp1.as_mut_ptr(), r.as_ptr(), rnorm.as_ptr());
        crate::engine::engine_inline::mji_scl3((*con).normal.as_mut_ptr(), tmp1.as_ptr(), if in_flag != 0 { -1.0 } else { 1.0 });
        crate::engine::engine_inline::mji_zero3((*con).tangent.as_mut_ptr());

        for i in 0..n as usize {
            (*con.add(i)).dist = depth[i];
            points[i][2] += hz;
            crate::engine::engine_inline::mji_mul_mat_vec3(tmp2.as_mut_ptr(), r.as_ptr(), points[i].as_ptr());
            crate::engine::engine_inline::mji_add3((*con.add(i)).pos.as_mut_ptr(), tmp2.as_ptr(), pos1);
            crate::engine::engine_inline::mji_copy3((*con.add(i)).normal.as_mut_ptr(), (*con).normal.as_ptr());
            crate::engine::engine_inline::mji_zero3((*con.add(i)).tangent.as_mut_ptr());
        }

        n
    }
}

