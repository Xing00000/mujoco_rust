//! Port of: engine/engine_collision_box.c
//! IR hash: 05737965add36adb
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
    // WARNING: signature changed — verify body
    // Previous params: (vec : * mut f64, limit : * const f64, n : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n as usize { if * limit . add (i) > 0.0 { let lim = * limit . add (i) ; let v = * vec . add (i) ; * vec . add (i) = if v < - lim { - lim } else if v > lim { lim } else { v } ; } } }
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
    const mjMINVAL: f64 = 1e-15;
    // SAFETY: all pointers valid per caller contract. con[0] writable.
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
        let dist: f64 = crate::engine::engine_util_blas::mju_normalize3(tmp.as_mut_ptr());

        if dist - *size1.add(0) > margin {
            return 0;
        }

        // sphere center inside box
        if dist <= mjMINVAL {
            let mut closest: f64 = (*size2.add(0) + *size2.add(1) + *size2.add(2)) * 2.0;
            let mut k: i32 = 0;

            for i in 0..6i32 {
                let sign: f64 = if i % 2 != 0 { 1.0 } else { -1.0 };
                let val = (sign * *size2.add((i / 2) as usize) - center[(i / 2) as usize]).abs();
                if closest > val {
                    closest = val;
                    k = i;
                }
            }

            let mut nearest: [f64; 3] = [0.0; 3];
            nearest[(k / 2) as usize] = if k % 2 != 0 { -1.0 } else { 1.0 };

            crate::engine::engine_inline::mji_copy3(pos.as_mut_ptr(), center.as_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(pos.as_mut_ptr(), nearest.as_ptr(), (*size1.add(0) - closest) / 2.0);
            crate::engine::engine_inline::mji_mul_mat_vec3((*con.add(0)).normal.as_mut_ptr(), mat2, nearest.as_ptr());
            // dist stays as it was (will be negated below via -closest)
            let dist = -closest;
            crate::engine::engine_inline::mji_mul_mat_vec3(tmp.as_mut_ptr(), mat2, pos.as_ptr());
            crate::engine::engine_inline::mji_add3((*con.add(0)).pos.as_mut_ptr(), tmp.as_ptr(), pos2);
            (*con.add(0)).dist = dist - *size1.add(0);
            crate::engine::engine_inline::mji_zero3((*con.add(0)).tangent.as_mut_ptr());
            return 1;
        } else {
            crate::engine::engine_inline::mji_add_to_scl3(deepest.as_mut_ptr(), tmp.as_ptr(), *size1.add(0));
            crate::engine::engine_util_blas::mju_zero3(pos.as_mut_ptr());
            crate::engine::engine_inline::mji_add_to_scl3(pos.as_mut_ptr(), clamped.as_ptr(), 0.5);
            crate::engine::engine_inline::mji_add_to_scl3(pos.as_mut_ptr(), deepest.as_ptr(), 0.5);
            crate::engine::engine_inline::mji_mul_mat_vec3((*con.add(0)).normal.as_mut_ptr(), mat2, tmp.as_ptr());
        }

        crate::engine::engine_inline::mji_mul_mat_vec3(tmp.as_mut_ptr(), mat2, pos.as_ptr());
        crate::engine::engine_inline::mji_add3((*con.add(0)).pos.as_mut_ptr(), tmp.as_ptr(), pos2);
        (*con.add(0)).dist = dist - *size1.add(0);
        crate::engine::engine_inline::mji_zero3((*con.add(0)).tangent.as_mut_ptr());
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
    use crate::engine::engine_inline::*;
    use crate::engine::engine_util_blas::*;

    const mjMINVAL: f64 = 1e-15;
    const mjMAXCONPAIR: usize = 50;

    // SAFETY: all pointer accesses through mjModel/mjData fields are valid per caller contract.
    // con points to a buffer of at least mjMAXCONPAIR elements.
    unsafe {
        let pos1 = (*D).geom_xpos.add(3 * g1 as usize);
        let pos2 = (*D).geom_xpos.add(3 * g2 as usize);
        let mat1 = (*D).geom_xmat.add(9 * g1 as usize);
        let mat2 = (*D).geom_xmat.add(9 * g2 as usize);
        let size1 = (*M).geom_size.add(3 * g1 as usize);
        let size2 = (*M).geom_size.add(3 * g2 as usize);

        let mut pos12 = [0.0f64; 3];
        let mut pos21 = [0.0f64; 3];
        let mut rot = [0.0f64; 9];
        let mut rott = [0.0f64; 9];
        let mut rotabs = [0.0f64; 9];
        let mut rottabs = [0.0f64; 9];
        let mut tmp1 = [0.0f64; 3];
        let mut tmp2 = [0.0f64; 3];
        let mut plen1 = [0.0f64; 3];
        let mut plen2 = [0.0f64; 3];

        let mut rotmore = [0.0f64; 9];
        let mut p = [0.0f64; 3];
        let mut r = [0.0f64; 9];
        let mut s = [0.0f64; 3];
        let mut ss = [0.0f64; 3];
        let mut lp = [0.0f64; 3];
        let mut rt = [0.0f64; 9];
        let mut points = [[0.0f64; 3]; mjMAXCONPAIR];
        let mut depth = [0.0f64; mjMAXCONPAIR];
        let mut pts = [[0.0f64; 3]; 6];
        let mut ppts2 = [[0.0f64; 2]; 4];
        let mut pu = [[0.0f64; 3]; 4];
        let mut axi = [[0.0f64; 3]; 3];
        let mut linesu = [[0.0f64; 6]; 4];
        let mut lines = [[0.0f64; 6]; 4];
        let mut clnorm = [0.0f64; 3];
        let mut rnorm = [0.0f64; 3];

        let mut penetration: f64;
        let mut c1: f64;
        let mut c2: f64;
        let mut c3: f64;
        let mut a: f64;
        let mut b: f64;
        let mut c: f64;
        let mut d: f64;
        let mut lx: f64;
        let mut ly: f64;
        let mut hz: f64;
        let mut l: f64;
        let mut x: f64;
        let mut y: f64;
        let mut u: f64;
        let mut v: f64;
        let mut llx: f64;
        let mut lly: f64;
        let mut innorm: f64;
        let margin2: f64;

        let mut i0: usize;
        let mut i1: usize;
        let mut i2: usize;
        let mut f0: f64;
        let mut f1: f64;
        let mut f2: f64;

        let mut n: usize;
        let mut code: i32;
        let mut q1: i32;
        let mut q2: i32;
        let mut clcorner: i32;
        let mut m: usize;
        let mut k: usize;
        let mut cle1: i32 = 0;
        let mut cle2: i32 = 0;
        let mut in_: i32 = 0;
        let mut clface: i32;

        n = 0;
        code = -1;
        margin2 = margin * margin;

        mji_sub3(tmp1.as_mut_ptr(), pos2, pos1);
        mji_mul_mat_t_vec3(pos21.as_mut_ptr(), mat1, tmp1.as_ptr());

        mji_sub3(tmp1.as_mut_ptr(), pos1, pos2);
        mji_mul_mat_t_vec3(pos12.as_mut_ptr(), mat2, tmp1.as_ptr());

        mju_mul_mat_t_mat3(rot.as_mut_ptr(), mat1, mat2);
        mju_transpose(rott.as_mut_ptr(), rot.as_ptr(), 3, 3);

        for i in 0..9 {
            rotabs[i] = rot[i].abs();
        }
        for i in 0..9 {
            rottabs[i] = rott[i].abs();
        }

        mji_mul_mat_vec3(plen2.as_mut_ptr(), rotabs.as_ptr(), size2);
        mji_mul_mat_t_vec3(plen1.as_mut_ptr(), rotabs.as_ptr(), size1);

        penetration = margin;
        for i in 0..3 {
            penetration += *size1.add(i) * 3.0 + *size2.add(i) * 3.0;
        }

        for i in 0..3usize {
            c1 = -pos21[i].abs() + *size1.add(i) + plen2[i];
            c2 = -pos12[i].abs() + *size2.add(i) + plen1[i];

            if c1 < -margin || c2 < -margin {
                return 0;
            }

            if c1 < penetration {
                penetration = c1;
                code = i as i32 + 3 * (pos21[i] < 0.0) as i32 + 0;
            }
            if c2 < penetration {
                penetration = c2;
                code = i as i32 + 3 * (pos12[i] < 0.0) as i32 + 6;
            }
        }

        for i in 0..3i32 {
            for j in 0..3i32 {
                mju_zero3(tmp2.as_mut_ptr());
                if i == 0 {
                    tmp2[1] = -rott[(3 * j + 2) as usize];
                    tmp2[2] = rott[(3 * j + 1) as usize];
                } else if i == 1 {
                    tmp2[0] = rott[(3 * j + 2) as usize];
                    tmp2[2] = -rott[(3 * j + 0) as usize];
                } else if i == 2 {
                    tmp2[0] = -rott[(3 * j + 1) as usize];
                    tmp2[1] = rott[(3 * j + 0) as usize];
                }

                c1 = mju_normalize3(tmp2.as_mut_ptr());

                if c1 < mjMINVAL {
                    continue;
                }

                c2 = mju_dot3(pos21.as_ptr(), tmp2.as_ptr());

                c3 = 0.0;

                for k in 0..3i32 {
                    if k != i {
                        c3 += *size1.add(k as usize) * tmp2[k as usize].abs();
                    }
                }
                for k in 0..3i32 {
                    if k != j {
                        c3 += *size2.add(k as usize) * rotabs[(3 * i + 3 - k - j) as usize] / c1;
                    }
                }

                c3 -= c2.abs();

                if c3 < -margin {
                    return 0;
                }

                if c3 < penetration * (1.0 - 1e-12) {
                    penetration = c3;
                    cle1 = 0;
                    for k in 0..3i32 {
                        if k != i {
                            if (tmp2[k as usize] > 0.0) ^ (c2 < 0.0) {
                                cle1 += 1 << k;
                            }
                        }
                    }
                    cle2 = 0;
                    for k in 0..3i32 {
                        if k != j {
                            if (rot[(3 * i + 3 - k - j) as usize] > 0.0) ^ (c2 < 0.0) ^ ((k - j + 3) % 3 == 1) {
                                cle2 += 1 << k;
                            }
                        }
                    }

                    code = 12 + i * 3 + j;
                    mji_copy3(clnorm.as_mut_ptr(), tmp2.as_ptr());
                    in_ = (c2 < 0.0) as i32;
                }
            }
        }

        if code == -1 {
            return 0;
        }

        if code >= 12 {
            // --- edgeedge path ---
            let mut code_ee = code - 12;
            q1 = code_ee / 3;
            q2 = code_ee % 3;

            let mut ax1: i32;
            let mut ax2: i32;
            let mut pax1: i32;
            let mut pax2: i32;

            if q2 == 0 { ax1 = 1; ax2 = 2; }
            else if q2 == 1 { ax1 = 0; ax2 = 2; }
            else { ax1 = 1; ax2 = 0; }

            if q1 == 0 { pax1 = 1; pax2 = 2; }
            else if q1 == 1 { pax1 = 0; pax2 = 2; }
            else { pax1 = 1; pax2 = 0; }

            if rotabs[(3 * q1 + ax1) as usize] < rotabs[(3 * q1 + ax2) as usize] {
                ax1 = ax2;
                ax2 = 3 - q2 - ax1;
            }
            if rottabs[(3 * q2 + pax1) as usize] < rottabs[(3 * q2 + pax2) as usize] {
                pax1 = pax2;
                pax2 = 3 - q1 - pax1;
            }

            if cle1 & (1 << pax2) != 0 {
                clface = pax2;
            } else {
                clface = pax2 + 3;
            }

            mju_zero(rotmore.as_mut_ptr(), 9);
            if clface == 0 {
                rotmore[2] = -1.0; rotmore[4] = 1.0; rotmore[6] = 1.0;
            } else if clface == 1 {
                rotmore[0] = 1.0; rotmore[5] = -1.0; rotmore[7] = 1.0;
            } else if clface == 2 {
                rotmore[0] = 1.0; rotmore[4] = 1.0; rotmore[8] = 1.0;
            } else if clface == 3 {
                rotmore[2] = 1.0; rotmore[4] = 1.0; rotmore[6] = -1.0;
            } else if clface == 4 {
                rotmore[0] = 1.0; rotmore[5] = 1.0; rotmore[7] = -1.0;
            } else if clface == 5 {
                rotmore[0] = -1.0; rotmore[4] = 1.0; rotmore[8] = -1.0;
            }

            i0 = 0; i1 = 1; i2 = 2;
            f0 = 1.0; f1 = 1.0; f2 = 1.0;

            if clface == 0 {
                i0 = 2; f0 = -1.0; i2 = 0;
            } else if clface == 1 {
                i1 = 2; f1 = -1.0; i2 = 1;
            } else if clface == 2 {
                // identity
            } else if clface == 3 {
                i0 = 2; i2 = 0; f2 = -1.0;
            } else if clface == 4 {
                i1 = 2; i2 = 1; f2 = -1.0;
            } else if clface == 5 {
                f0 = -1.0; f2 = -1.0;
            }

            // rotaxis(p, pos21)
            p[0] = pos21[i0] * f0;
            p[1] = pos21[i1] * f1;
            p[2] = pos21[i2] * f2;

            // rotaxis(rnorm, clnorm)
            rnorm[0] = clnorm[i0] * f0;
            rnorm[1] = clnorm[i1] * f1;
            rnorm[2] = clnorm[i2] * f2;

            // rotmatx(r, rot)
            mji_scl3(r.as_mut_ptr().add(0), rot.as_ptr().add(i0 * 3), f0);
            mji_scl3(r.as_mut_ptr().add(3), rot.as_ptr().add(i1 * 3), f1);
            mji_scl3(r.as_mut_ptr().add(6), rot.as_ptr().add(i2 * 3), f2);

            mji_mul_mat_t_vec3(tmp1.as_mut_ptr(), rotmore.as_ptr(), size1);
            for i in 0..3 {
                s[i] = tmp1[i].abs();
            }

            mju_transpose(rt.as_mut_ptr(), r.as_ptr(), 3, 3);

            lx = s[0];
            ly = s[1];
            hz = s[2];
            p[2] -= hz;

            n = 0;
            mji_copy3(points[n].as_mut_ptr(), p.as_ptr());
            mji_add_to_scl3(
                points[n].as_mut_ptr(),
                rt.as_ptr().add(3 * ax1 as usize),
                *size2.add(ax1 as usize) * (if cle2 & (1 << ax1) != 0 { 1.0 } else { -1.0 }),
            );
            mji_add_to_scl3(
                points[n].as_mut_ptr(),
                rt.as_ptr().add(3 * ax2 as usize),
                *size2.add(ax2 as usize) * (if cle2 & (1 << ax2) != 0 { 1.0 } else { -1.0 }),
            );
            mji_copy3(points[n + 1].as_mut_ptr(), points[n].as_ptr());
            mji_add_to_scl3(points[n].as_mut_ptr(), rt.as_ptr().add(3 * q2 as usize), *size2.add(q2 as usize));
            n = 1;
            mji_add_to_scl3(points[n].as_mut_ptr(), rt.as_ptr().add(3 * q2 as usize), -*size2.add(q2 as usize));
            n = 2;

            mji_copy3(points[n].as_mut_ptr(), p.as_ptr());
            mji_add_to_scl3(
                points[n].as_mut_ptr(),
                rt.as_ptr().add(3 * ax1 as usize),
                *size2.add(ax1 as usize) * (if cle2 & (1 << ax1) != 0 { -1.0 } else { 1.0 }),
            );
            mji_add_to_scl3(
                points[n].as_mut_ptr(),
                rt.as_ptr().add(3 * ax2 as usize),
                *size2.add(ax2 as usize) * (if cle2 & (1 << ax2) != 0 { 1.0 } else { -1.0 }),
            );
            mji_copy3(points[n + 1].as_mut_ptr(), points[n].as_ptr());
            mji_add_to_scl3(points[n].as_mut_ptr(), rt.as_ptr().add(3 * q2 as usize), *size2.add(q2 as usize));
            n = 3;
            mji_add_to_scl3(points[n].as_mut_ptr(), rt.as_ptr().add(3 * q2 as usize), -*size2.add(q2 as usize));
            n = 4;

            mji_copy3(axi[0].as_mut_ptr(), points[0].as_ptr());
            mji_sub3(axi[1].as_mut_ptr(), points[1].as_ptr(), points[0].as_ptr());
            mji_sub3(axi[2].as_mut_ptr(), points[2].as_ptr(), points[0].as_ptr());

            if rnorm[2].abs() < mjMINVAL {
                return 0;
            }

            innorm = (1.0 / rnorm[2]) * (if in_ != 0 { -1.0 } else { 1.0 });

            for i in 0..4usize {
                c1 = -points[i][2] * (1.0 / rnorm[2]);

                mji_copy3(pu[i].as_mut_ptr(), points[i].as_ptr());

                mji_add_to_scl3(points[i].as_mut_ptr(), rnorm.as_ptr(), c1);

                ppts2[i][0] = points[i][0];
                ppts2[i][1] = points[i][1];
            }

            mji_copy3(pts[0].as_mut_ptr(), points[0].as_ptr());
            mji_sub3(pts[1].as_mut_ptr(), points[1].as_ptr(), points[0].as_ptr());
            mji_sub3(pts[2].as_mut_ptr(), points[2].as_ptr(), points[0].as_ptr());

            m = 3;
            k = 0;
            n = 0;

            if m > 1 {
                mji_copy3(lines[k].as_mut_ptr().add(0), pts[0].as_ptr());
                mji_copy3(lines[k].as_mut_ptr().add(3), pts[1].as_ptr());
                mji_copy3(linesu[k].as_mut_ptr().add(0), axi[0].as_ptr());
                mji_copy3(linesu[k].as_mut_ptr().add(3), axi[1].as_ptr());
                k += 1;
            }
            if m > 2 {
                mji_copy3(lines[k].as_mut_ptr().add(0), pts[0].as_ptr());
                mji_copy3(lines[k].as_mut_ptr().add(3), pts[2].as_ptr());
                mji_copy3(linesu[k].as_mut_ptr().add(0), axi[0].as_ptr());
                mji_copy3(linesu[k].as_mut_ptr().add(3), axi[2].as_ptr());
                k += 1;

                mji_add3(lines[k].as_mut_ptr().add(0), pts[0].as_ptr(), pts[1].as_ptr());
                mji_copy3(lines[k].as_mut_ptr().add(3), pts[2].as_ptr());
                mji_add3(linesu[k].as_mut_ptr().add(0), axi[0].as_ptr(), axi[1].as_ptr());
                mji_copy3(linesu[k].as_mut_ptr().add(3), axi[2].as_ptr());
                k += 1;

                mji_add3(lines[k].as_mut_ptr().add(0), pts[0].as_ptr(), pts[2].as_ptr());
                mji_copy3(lines[k].as_mut_ptr().add(3), pts[1].as_ptr());
                mji_add3(linesu[k].as_mut_ptr().add(0), axi[0].as_ptr(), axi[2].as_ptr());
                mji_copy3(linesu[k].as_mut_ptr().add(3), axi[1].as_ptr());
                k += 1;
            }

            for i in 0..k {
                for q in 0..2i32 {
                    a = lines[i][(0 + q) as usize];
                    b = lines[i][(3 + q) as usize];
                    c = lines[i][(1 - q) as usize];
                    d = lines[i][(4 - q) as usize];

                    if b.abs() > mjMINVAL {
                        for j in [-1i32, 1i32] {
                            if n < mjMAXCONPAIR {
                                l = s[q as usize] * j as f64;
                                c1 = (l - a) * (1.0 / b);
                                if c1 < 0.0 || c1 > 1.0 {
                                    continue;
                                }
                                c2 = c + d * c1;
                                if c2.abs() > s[(1 - q) as usize] {
                                    continue;
                                }

                                if (linesu[i][2] + linesu[i][5] * c1) * innorm > margin {
                                    continue;
                                }

                                mji_scl3(points[n].as_mut_ptr(), linesu[i].as_ptr(), 0.5);
                                mji_add_to_scl3(points[n].as_mut_ptr(), linesu[i].as_ptr().add(3), 0.5 * c1);
                                points[n][(0 + q) as usize] += 0.5 * l;
                                points[n][(1 - q) as usize] += 0.5 * c2;
                                depth[n] = points[n][2] * innorm * 2.0;
                                n += 1;
                            }
                        }
                    }
                }
            }

            let nl = n;

            a = pts[1][0];
            b = pts[2][0];
            c = pts[1][1];
            d = pts[2][1];
            c1 = a * d - b * c;

            for i in 0..4usize {
                if n < mjMAXCONPAIR {
                    llx = if i / 2 != 0 { lx } else { -lx };
                    lly = if i % 2 != 0 { ly } else { -ly };

                    x = llx - pts[0][0];
                    y = lly - pts[0][1];

                    u = (x * d - y * b) * (1.0 / c1);
                    v = (y * a - x * c) * (1.0 / c1);

                    if nl == 0 {
                        if (u < 0.0 || u > 1.0) && (v < 0.0 || v > 1.0) {
                            continue;
                        }
                    } else {
                        if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 {
                            continue;
                        }
                    }

                    if u < 0.0 { u = 0.0; }
                    if u > 1.0 { u = 1.0; }
                    if v < 0.0 { v = 0.0; }
                    if v > 1.0 { v = 1.0; }

                    mji_scl3(tmp1.as_mut_ptr(), pu[0].as_ptr(), 1.0 - u - v);
                    mji_add_to_scl3(tmp1.as_mut_ptr(), pu[1].as_ptr(), u);
                    mji_add_to_scl3(tmp1.as_mut_ptr(), pu[2].as_ptr(), v);

                    points[n][0] = llx;
                    points[n][1] = lly;
                    points[n][2] = 0.0;

                    mji_sub3(tmp2.as_mut_ptr(), points[n].as_ptr(), tmp1.as_ptr());

                    c1 = mju_dot3(tmp2.as_ptr(), tmp2.as_ptr());
                    if tmp1[2] > 0.0 {
                        if c1 > margin2 {
                            continue;
                        }
                    }

                    mji_add_to3(points[n].as_mut_ptr(), tmp1.as_ptr());
                    mju_scl3(points[n].as_mut_ptr(), points[n].as_ptr(), 0.5);

                    depth[n] = c1.sqrt() * (if tmp1[2] < 0.0 { -1.0 } else { 1.0 });
                    n += 1;
                }
            }

            let nf = n;

            for i in 0..4usize {
                if n < mjMAXCONPAIR {
                    x = ppts2[i][0];
                    y = ppts2[i][1];

                    if nl == 0 {
                        if nf == 0 {
                            // no filter
                        } else {
                            if x < -lx || x > lx {
                                if y < -ly || y > ly {
                                    continue;
                                }
                            }
                        }
                    } else {
                        if x < -lx || x > lx || y < -ly || y > ly {
                            continue;
                        }
                    }

                    c1 = 0.0;
                    for j in 0..2usize {
                        if ppts2[i][j] < -s[j] {
                            c1 += (ppts2[i][j] + s[j]) * (ppts2[i][j] + s[j]);
                        } else if ppts2[i][j] > s[j] {
                            c1 += (ppts2[i][j] - s[j]) * (ppts2[i][j] - s[j]);
                        }
                    }

                    c1 += pu[i][2] * innorm * pu[i][2] * innorm;

                    if pu[i][2] > 0.0 {
                        if c1 > margin2 {
                            continue;
                        }
                    }

                    tmp1[0] = ppts2[i][0] * 0.5;
                    tmp1[1] = ppts2[i][1] * 0.5;
                    tmp1[2] = 0.0;

                    for j in 0..2usize {
                        if ppts2[i][j] < -s[j] {
                            tmp1[j] = -s[j] * 0.5;
                        } else if ppts2[i][j] > s[j] {
                            tmp1[j] = s[j] * 0.5;
                        }
                    }
                    mji_add_to_scl3(tmp1.as_mut_ptr(), pu[i].as_ptr(), 0.5);
                    mji_copy3(points[n].as_mut_ptr(), tmp1.as_ptr());

                    depth[n] = c1.sqrt() * (if pu[i][2] < 0.0 { -1.0 } else { 1.0 });
                    n += 1;
                }
            }

            mju_mul_mat_mat_t3(r.as_mut_ptr(), mat1, rotmore.as_ptr());

            mji_mul_mat_vec3(tmp1.as_mut_ptr(), r.as_ptr(), rnorm.as_ptr());

            mji_scl3(
                (*con.add(0)).normal.as_mut_ptr(),
                tmp1.as_ptr(),
                if in_ != 0 { -1.0 } else { 1.0 },
            );
            mji_zero3((*con.add(0)).tangent.as_mut_ptr());

            for i in 0..n {
                (*con.add(i)).dist = depth[i];
                points[i][2] += hz;

                mji_mul_mat_vec3(tmp2.as_mut_ptr(), r.as_ptr(), points[i].as_ptr());

                mji_add3((*con.add(i)).pos.as_mut_ptr(), tmp2.as_ptr(), pos1);

                mji_copy3((*con.add(i)).normal.as_mut_ptr(), (*con.add(0)).normal.as_ptr());
                mji_zero3((*con.add(i)).tangent.as_mut_ptr());
            }

            return n as i32;
        }

        // --- face path ---
        q1 = code % 6;
        q2 = code / 6;

        mju_zero(rotmore.as_mut_ptr(), 9);
        if q1 == 0 {
            rotmore[2] = -1.0; rotmore[4] = 1.0; rotmore[6] = 1.0;
        } else if q1 == 1 {
            rotmore[0] = 1.0; rotmore[5] = -1.0; rotmore[7] = 1.0;
        } else if q1 == 2 {
            rotmore[0] = 1.0; rotmore[4] = 1.0; rotmore[8] = 1.0;
        } else if q1 == 3 {
            rotmore[2] = 1.0; rotmore[4] = 1.0; rotmore[6] = -1.0;
        } else if q1 == 4 {
            rotmore[0] = 1.0; rotmore[5] = 1.0; rotmore[7] = -1.0;
        } else if q1 == 5 {
            rotmore[0] = -1.0; rotmore[4] = 1.0; rotmore[8] = -1.0;
        }

        i0 = 0; i1 = 1; i2 = 2;
        f0 = 1.0; f1 = 1.0; f2 = 1.0;

        if q1 == 0 {
            i0 = 2; f0 = -1.0; i2 = 0;
        } else if q1 == 1 {
            i1 = 2; f1 = -1.0; i2 = 1;
        } else if q1 == 2 {
            // identity
        } else if q1 == 3 {
            i0 = 2; i2 = 0; f2 = -1.0;
        } else if q1 == 4 {
            i1 = 2; i2 = 1; f2 = -1.0;
        } else if q1 == 5 {
            f0 = -1.0; f2 = -1.0;
        }

        if q2 != 0 {
            mju_mul_mat_mat_t3(r.as_mut_ptr(), rotmore.as_ptr(), rot.as_ptr());

            // rotaxis(p, pos12)
            p[0] = pos12[i0] * f0;
            p[1] = pos12[i1] * f1;
            p[2] = pos12[i2] * f2;

            // rotaxis(tmp1, size2)
            tmp1[0] = *size2.add(i0) * f0;
            tmp1[1] = *size2.add(i1) * f1;
            tmp1[2] = *size2.add(i2) * f2;

            mji_copy3(s.as_mut_ptr(), size1);
        } else {
            // rotmatx(r, rot)
            mji_scl3(r.as_mut_ptr().add(0), rot.as_ptr().add(i0 * 3), f0);
            mji_scl3(r.as_mut_ptr().add(3), rot.as_ptr().add(i1 * 3), f1);
            mji_scl3(r.as_mut_ptr().add(6), rot.as_ptr().add(i2 * 3), f2);

            // rotaxis(p, pos21)
            p[0] = pos21[i0] * f0;
            p[1] = pos21[i1] * f1;
            p[2] = pos21[i2] * f2;

            // rotaxis(tmp1, size1)
            tmp1[0] = *size1.add(i0) * f0;
            tmp1[1] = *size1.add(i1) * f1;
            tmp1[2] = *size1.add(i2) * f2;

            mji_copy3(s.as_mut_ptr(), size2);
        }

        mju_transpose(rt.as_mut_ptr(), r.as_ptr(), 3, 3);

        for i in 0..3 {
            ss[i] = tmp1[i].abs();
        }

        lx = ss[0];
        ly = ss[1];
        hz = ss[2];
        p[2] -= hz;

        mji_copy3(lp.as_mut_ptr(), p.as_ptr());

        clcorner = 0;
        for i in 0..3 {
            if r[6 + i] < 0.0 {
                clcorner += 1 << i;
            }
        }

        mji_add_to_scl3(lp.as_mut_ptr(), rt.as_ptr().add(0), s[0] * (if clcorner & 1 != 0 { 1.0 } else { -1.0 }));
        mji_add_to_scl3(lp.as_mut_ptr(), rt.as_ptr().add(3), s[1] * (if clcorner & 2 != 0 { 1.0 } else { -1.0 }));
        mji_add_to_scl3(lp.as_mut_ptr(), rt.as_ptr().add(6), s[2] * (if clcorner & 4 != 0 { 1.0 } else { -1.0 }));

        m = 0;
        k = 0;
        mji_copy3(pts[m].as_mut_ptr(), lp.as_ptr());
        m += 1;

        for i in 0..3usize {
            if r[6 + i].abs() < 0.5 {
                mju_scl3(
                    pts[m].as_mut_ptr(),
                    rt.as_ptr().add(3 * i),
                    s[i] * (if clcorner & (1 << i) != 0 { -2.0 } else { 2.0 }),
                );
                m += 1;
            }
        }

        mji_add3(pts[3].as_mut_ptr(), pts[0].as_ptr(), pts[1].as_ptr());
        mji_add3(pts[4].as_mut_ptr(), pts[0].as_ptr(), pts[2].as_ptr());
        mji_add3(pts[5].as_mut_ptr(), pts[3].as_ptr(), pts[2].as_ptr());

        if m > 1 {
            mji_copy3(lines[k].as_mut_ptr().add(0), pts[0].as_ptr());
            mji_copy3(lines[k].as_mut_ptr().add(3), pts[1].as_ptr());
            k += 1;
        }
        if m > 2 {
            mji_copy3(lines[k].as_mut_ptr().add(0), pts[0].as_ptr());
            mji_copy3(lines[k].as_mut_ptr().add(3), pts[2].as_ptr());
            k += 1;
            mji_copy3(lines[k].as_mut_ptr().add(0), pts[3].as_ptr());
            mji_copy3(lines[k].as_mut_ptr().add(3), pts[2].as_ptr());
            k += 1;
            mji_copy3(lines[k].as_mut_ptr().add(0), pts[4].as_ptr());
            mji_copy3(lines[k].as_mut_ptr().add(3), pts[1].as_ptr());
            k += 1;
        }

        for i in 0..k {
            for q in 0..2i32 {
                a = lines[i][(0 + q) as usize];
                b = lines[i][(3 + q) as usize];
                c = lines[i][(1 - q) as usize];
                d = lines[i][(4 - q) as usize];

                if b.abs() > mjMINVAL {
                    for j in [-1i32, 1i32] {
                        l = ss[q as usize] * j as f64;
                        c1 = (l - a) * (1.0 / b);
                        if c1 < 0.0 || c1 > 1.0 {
                            continue;
                        }
                        c2 = c + d * c1;
                        if c2.abs() > ss[(1 - q) as usize] {
                            continue;
                        }

                        if n < mjMAXCONPAIR {
                            mji_copy3(points[n].as_mut_ptr(), lines[i].as_ptr());
                            mji_add_to_scl3(points[n].as_mut_ptr(), lines[i].as_ptr().add(3), c1);
                            n += 1;
                        }
                    }
                }
            }
        }

        a = pts[1][0];
        b = pts[2][0];
        c = pts[1][1];
        d = pts[2][1];
        c1 = a * d - b * c;

        if m > 2 {
            for i in 0..4usize {
                llx = if i / 2 != 0 { lx } else { -lx };
                lly = if i % 2 != 0 { ly } else { -ly };

                x = llx - pts[0][0];
                y = lly - pts[0][1];

                u = (x * d - y * b) * (1.0 / c1);
                v = (y * a - x * c) * (1.0 / c1);
                if u <= 0.0 || v <= 0.0 || u >= 1.0 || v >= 1.0 {
                    continue;
                }

                if n < mjMAXCONPAIR {
                    points[n][0] = llx;
                    points[n][1] = lly;
                    points[n][2] = pts[0][2] + u * pts[1][2] + v * pts[2][2];
                    n += 1;
                }
            }
        }

        for i in 0..(1usize << (m - 1)) {
            mji_copy3(tmp1.as_mut_ptr(), pts[if i == 0 { 0 } else { i + 2 }].as_ptr());

            if i != 0 {
                if tmp1[0] <= -lx || tmp1[0] >= lx {
                    continue;
                }
            }
            if i != 0 {
                if tmp1[1] <= -ly || tmp1[1] >= ly {
                    continue;
                }
            }

            if n < mjMAXCONPAIR {
                mji_copy3(points[n].as_mut_ptr(), tmp1.as_ptr());
                n += 1;
            }
        }

        // filter by depth
        m = n;
        n = 0;

        for i in 0..m {
            if points[i][2] > margin {
                continue;
            }
            if n != i {
                mji_copy3(points[n].as_mut_ptr(), points[i].as_ptr());
            }

            depth[n] = points[n][2];
            points[n][2] *= 0.5;

            n += 1;
        }

        mju_mul_mat_mat_t3(r.as_mut_ptr(), if q2 != 0 { mat2 } else { mat1 }, rotmore.as_ptr());
        mju_copy3(p.as_mut_ptr(), if q2 != 0 { pos2 } else { pos1 });

        tmp2[0] = (if q2 != 0 { -1.0 } else { 1.0 }) * r[2];
        tmp2[1] = (if q2 != 0 { -1.0 } else { 1.0 }) * r[5];
        tmp2[2] = (if q2 != 0 { -1.0 } else { 1.0 }) * r[8];

        mji_copy3((*con.add(0)).normal.as_mut_ptr(), tmp2.as_ptr());
        mji_zero3((*con.add(0)).tangent.as_mut_ptr());

        for i in 0..n {
            (*con.add(i)).dist = 2.0 * points[i][2];
            points[i][2] += hz;

            mji_mul_mat_vec3(tmp2.as_mut_ptr(), r.as_ptr(), points[i].as_ptr());
            mji_add3((*con.add(i)).pos.as_mut_ptr(), tmp2.as_ptr(), p.as_ptr());

            if i != 0 {
                mji_copy3((*con.add(i)).normal.as_mut_ptr(), (*con.add(0)).normal.as_ptr());
                mji_zero3((*con.add(i)).tangent.as_mut_ptr());
            }
        }

        n as i32
    }
}

