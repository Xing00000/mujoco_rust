//! Port of: engine/engine_passive.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GradSquaredLengths (engine/engine_passive.c:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn grad_squared_lengths(gradient: [[[f64; 6]; 2]; 3], xpos: *const f64, vert: [i32; 4], edge: [[i32; 6]; 2], nedge: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (gradient : [[[f64 ; 6] ; 2] ; 3], xpos : * const f64, vert : [i32 ; 4], edge : [[i32 ; 6] ; 2], nedge : i32)
    // Previous return: ()
    unsafe { let grad_ptr = & gradient as * const _ as * mut f64 ; let vert_ptr = vert . as_ptr () ; let edge_ptr = & edge as * const _ as * const i32 ; for e in 0 .. nedge as usize { for d in 0 .. 3usize { let e0 = * edge_ptr . add (e * 2 + 0) as usize ; let e1 = * edge_ptr . add (e * 2 + 1) as usize ; let v0 = * vert_ptr . add (e0) as usize ; let v1 = * vert_ptr . add (e1) as usize ; let val = * xpos . add (3 * v0 + d) - * xpos . add (3 * v1 + d) ; * grad_ptr . add (e * 6 + 0 * 3 + d) = val ; * grad_ptr . add (e * 6 + 1 * 3 + d) = - val ; } } }
}

/// C: mj_flexPassiveInterp (engine/engine_passive.c:63)
/// Calls: mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_addScl3, mji_addTo3, mji_rotVecQuat, mju_flexGatherCellState, mju_flexGatherFaceState, mju_flexGatherState, mju_mulMatVec, mju_negQuat, mju_rotVecQuat, mju_scl3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    extern "C" { fn mj_flexPassiveInterp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_flexPassiveInterp(m, d, f, enbl_spring, enbl_damper) }
}

/// C: mju_dphi2D (engine/engine_passive.c:211)
/// Calls: mju_flexDphi, mju_flexPhi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dphi2d(s0: f64, l0: i32, s1: f64, l1: i32, order: i32, dir: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (s0 : f64, l0 : i32, s1 : f64, l1 : i32, order : i32, dir : i32)
    // Previous return: f64
    use crate :: engine :: engine_util_misc :: { mju_flex_phi , mju_flex_dphi } ; if dir == 0 { mju_flex_dphi (s0 , l0 , order) * mju_flex_phi (s1 , l1 , order) } else { mju_flex_phi (s0 , l0 , order) * mju_flex_dphi (s1 , l1 , order) }
}

/// C: mj_flexPassiveBendInterp (engine/engine_passive.c:236)
/// Calls: mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_addTo3, mji_cross, mji_sub3, mju_add, mju_copy, mju_copyInt, mju_dot, mju_dot3, mju_dphi2D, mju_flexFaceNormal2D, mju_flexGatherFaceState, mju_flexGatherState, mju_message, mju_negQuat, mju_norm3, mju_normalize, mju_rotVecQuat, mju_scl, mju_warning, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    extern "C" { fn mj_flexPassiveBendInterp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_flexPassiveBendInterp(m, d, f, enbl_spring, enbl_damper) }
}

/// C: mj_flexPassiveBend (engine/engine_passive.c:444)
/// Calls: mji_cross, mji_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    // SAFETY: m, d valid per caller. All pointer arithmetic within allocated model/data arrays.
    unsafe {
        if *(*m).flex_dim.add(f as usize) != 2 {
            return;
        }

        let bendingadr: i32 = *(*m).flex_bendingadr.add(f as usize);
        if bendingadr < 0 {
            return;
        }

        let edgenum: i32 = *(*m).flex_edgenum.add(f as usize);
        let xpos: *mut f64 = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(f as usize) as usize);
        let bodyid: *const i32 = (*m).flex_vertbodyid.add(*(*m).flex_vertadr.add(f as usize) as usize);
        let b: *const f64 = (*m).flex_bending.add(bendingadr as usize);

        let mut e: i32 = 0;
        while e < edgenum {
            let edge: *const i32 = (*m).flex_edge.add(2 * (e + *(*m).flex_edgeadr.add(f as usize)) as usize);
            let flap: *const i32 = (*m).flex_edgeflap.add(2 * (e + *(*m).flex_edgeadr.add(f as usize)) as usize);
            let v: [i32; 4] = [*edge.add(0), *edge.add(1), *flap.add(0), *flap.add(1)];
            if v[3] == -1 {
                // skip boundary edges
                e += 1;
                continue;
            }

            // flap edges
            let mut ed: [[f64; 3]; 3] = [[0.0; 3]; 3];
            crate::engine::engine_inline::mji_sub3(ed[0].as_mut_ptr(), xpos.add(3 * v[1] as usize), xpos.add(3 * v[0] as usize));
            crate::engine::engine_inline::mji_sub3(ed[1].as_mut_ptr(), xpos.add(3 * v[2] as usize), xpos.add(3 * v[0] as usize));
            crate::engine::engine_inline::mji_sub3(ed[2].as_mut_ptr(), xpos.add(3 * v[3] as usize), xpos.add(3 * v[0] as usize));

            // forces at the vertices due to curved reference
            let mut frc: [[f64; 3]; 4] = [[0.0; 3]; 4];
            crate::engine::engine_inline::mji_cross(frc[1].as_mut_ptr(), ed[1].as_ptr(), ed[2].as_ptr());
            crate::engine::engine_inline::mji_cross(frc[2].as_mut_ptr(), ed[2].as_ptr(), ed[0].as_ptr());
            crate::engine::engine_inline::mji_cross(frc[3].as_mut_ptr(), ed[0].as_ptr(), ed[1].as_ptr());
            frc[0][0] = -(frc[1][0] + frc[2][0] + frc[3][0]);
            frc[0][1] = -(frc[1][1] + frc[2][1] + frc[3][1]);
            frc[0][2] = -(frc[1][2] + frc[2][2] + frc[3][2]);

            // velocities
            let mut vel: [*mut f64; 4] = [core::ptr::null_mut(); 4];
            let mut i: i32 = 0;
            while i < 4 {
                vel[i as usize] = (*d).qvel.add(*(*m).body_dofadr.add(*bodyid.add(v[i as usize] as usize) as usize) as usize);
                i += 1;
            }

            // force
            let mut spring: [f64; 12] = [0.0; 12];
            let mut damper: [f64; 12] = [0.0; 12];
            let mut i: i32 = 0;
            while i < 4 {
                let mut x: i32 = 0;
                while x < 3 {
                    let mut j: i32 = 0;
                    while j < 4 {
                        // thin plate bending force
                        if enbl_spring != 0 {
                            spring[(3 * i + x) as usize] += *b.add((17 * e + 4 * i + j) as usize) * *xpos.add((3 * v[j as usize] + x) as usize);
                        }

                        // thin plate damping force
                        if enbl_damper != 0 {
                            damper[(3 * i + x) as usize] += *b.add((17 * e + 4 * i + j) as usize) * *vel[j as usize].add(x as usize);
                        }
                        j += 1;
                    }

                    // curved reference contribution
                    if enbl_spring != 0 {
                        spring[(3 * i + x) as usize] += *b.add((17 * e + 16) as usize) * frc[i as usize][x as usize];
                    }
                    x += 1;
                }
                i += 1;
            }

            // insert into global force
            let mut i: i32 = 0;
            while i < 4 {
                let bid: i32 = *bodyid.add(v[i as usize] as usize);
                let body_dofnum: i32 = *(*m).body_dofnum.add(bid as usize);
                let body_dofadr: i32 = *(*m).body_dofadr.add(bid as usize);
                let mut x: i32 = 0;
                while x < body_dofnum {
                    if enbl_spring != 0 {
                        *(*d).qfrc_spring.add((body_dofadr + x) as usize) -= spring[(3 * i + x) as usize];
                    }
                    if enbl_damper != 0 {
                        *(*d).qfrc_damper.add((body_dofadr + x) as usize) -= damper[(3 * i + x) as usize] * *(*m).flex_damping.add(f as usize);
                    }
                    x += 1;
                }
                i += 1;
            }

            e += 1;
        }
    }
}

/// C: mj_flexPassiveStretch (engine/engine_passive.c:524)
/// Calls: GradSquaredLengths, mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_stretch(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    // edges[dim-2]: for dim=2 (tri) → edges[0], for dim=3 (tet) → edges[1]
    // C layout: int edges[2][6][2], accessed via pointer as edge_ptr[e*2+{0,1}]
    // Rust signature expects [[i32; 6]; 2], but data is accessed as raw pointer
    // Flat memory for dim=2: [1,2, 2,0, 0,1, 0,0, 0,0, 0,0]
    // Flat memory for dim=3: [0,1, 1,2, 2,0, 2,3, 0,3, 1,3]
    const EDGES: [[[i32; 6]; 2]; 2] = [
        [[1, 2, 2, 0, 0, 1], [0, 0, 0, 0, 0, 0]],
        [[0, 1, 1, 2, 2, 0], [2, 3, 0, 3, 1, 3]],
    ];

    // SAFETY: m, d valid per caller. All pointer arithmetic within allocated model/data arrays.
    unsafe {
        let stiffnessadr: i32 = *(*m).flex_stiffnessadr.add(f as usize);
        if stiffnessadr < 0 {
            return;
        }
        let k: *const f64 = (*m).flex_stiffness.add(stiffnessadr as usize);
        if *k.add(0) == 0.0 {
            return;
        }

        let dim: i32 = *(*m).flex_dim.add(f as usize);
        let nedge: i32 = if dim == 2 { 3 } else { 6 };
        let nvert: i32 = if dim == 2 { 3 } else { 4 };
        let elem: *const i32 = (*m).flex_elem.add(*(*m).flex_elemdataadr.add(f as usize) as usize);
        let edgeelem: *const i32 = (*m).flex_elemedge.add(*(*m).flex_elemedgeadr.add(f as usize) as usize);
        let xpos: *const f64 = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(f as usize) as usize);
        let vel: *const f64 = (*d).flexedge_velocity.add(*(*m).flex_edgeadr.add(f as usize) as usize);
        let deformed: *const f64 = (*d).flexedge_length.add(*(*m).flex_edgeadr.add(f as usize) as usize);
        let reference: *const f64 = (*m).flexedge_length0.add(*(*m).flex_edgeadr.add(f as usize) as usize);
        let bodyid: *const i32 = (*m).flex_vertbodyid.add(*(*m).flex_vertadr.add(f as usize) as usize);
        let kD: f64 = if (*m).opt.timestep > 0.0 { *(*m).flex_damping.add(f as usize) / (*m).opt.timestep } else { 0.0 };

        crate::engine::engine_memory::mj_mark_stack(d);
        let qfrc: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, 3 * *(*m).flex_vertnum.add(f as usize) as usize);
        crate::engine::engine_util_blas::mju_zero(qfrc, 3 * *(*m).flex_vertnum.add(f as usize));

        // compute force element-by-element
        let elemnum: i32 = *(*m).flex_elemnum.add(f as usize);
        let mut t: i32 = 0;
        while t < elemnum {
            let vert: *const i32 = elem.add(((dim + 1) * t) as usize);

            // compute length gradient with respect to dofs
            let mut gradient: [[[f64; 6]; 2]; 3] = [[[0.0; 6]; 2]; 3];
            let vert_arr: [i32; 4] = [*vert.add(0), *vert.add(1), *vert.add(2), if nvert > 3 { *vert.add(3) } else { 0 }];
            crate::engine::engine_passive::grad_squared_lengths(gradient, xpos, vert_arr, EDGES[(dim - 2) as usize], nedge);
            let grad_ptr: *const f64 = &gradient as *const _ as *const f64;

            // extract elongation of edges belonging to this element
            let mut elongation: [f64; 6] = [0.0; 6];
            let mut e: i32 = 0;
            while e < nedge {
                let idx: i32 = *edgeelem.add((t * nedge + e) as usize);
                let previous: f64 = *deformed.add(idx as usize) - *vel.add(idx as usize) * (*m).opt.timestep;
                elongation[e as usize] = *deformed.add(idx as usize) * *deformed.add(idx as usize)
                    - *reference.add(idx as usize) * *reference.add(idx as usize)
                    + (*deformed.add(idx as usize) * *deformed.add(idx as usize) - previous * previous) * kD;
                e += 1;
            }

            // unpack triangular representation
            let mut metric: [f64; 36] = [0.0; 36];
            let mut id: i32 = 0;
            let mut ed1: i32 = 0;
            while ed1 < nedge {
                let mut ed2: i32 = ed1;
                while ed2 < nedge {
                    metric[(nedge * ed1 + ed2) as usize] = *k.add((21 * t + id) as usize);
                    metric[(nedge * ed2 + ed1) as usize] = *k.add((21 * t + id) as usize);
                    id += 1;
                    ed2 += 1;
                }
                ed1 += 1;
            }

            // compute local force
            let mut force: [f64; 12] = [0.0; 12];
            let mut ed1: i32 = 0;
            while ed1 < nedge {
                let mut ed2: i32 = 0;
                while ed2 < nedge {
                    let mut i: i32 = 0;
                    while i < 2 {
                        let mut x: i32 = 0;
                        while x < 3 {
                            // C access: gradient[ed2][i][x] at offset ed2*6 + i*3 + x
                            let edge_vert: i32 = *(&EDGES[(dim - 2) as usize] as *const _ as *const i32).add((ed2 * 2 + i) as usize);
                            force[(3 * edge_vert + x) as usize] -=
                                elongation[ed1 as usize] * *grad_ptr.add((ed2 * 6 + i * 3 + x) as usize)
                                * metric[(nedge * ed1 + ed2) as usize];
                            x += 1;
                        }
                        i += 1;
                    }
                    ed2 += 1;
                }
                ed1 += 1;
            }

            // insert into global force
            let mut i: i32 = 0;
            while i < nvert {
                let mut x: i32 = 0;
                while x < 3 {
                    *qfrc.add((3 * *vert.add(i as usize) + x) as usize) += force[(3 * i + x) as usize];
                    x += 1;
                }
                i += 1;
            }

            t += 1;
        }

        // insert force into qfrc_passive
        let mut v: i32 = 0;
        while v < *(*m).flex_vertnum.add(f as usize) {
            let bid: i32 = *bodyid.add(v as usize);
            if *(*m).body_simple.add(bid as usize) != 2 {
                // this should only occur for pinned flex vertices
                crate::engine::engine_support::mj_apply_ft(m, d, qfrc.add(3 * v as usize), core::ptr::null(), xpos.add(3 * v as usize), bid, (*d).qfrc_spring);
            } else {
                let body_dofnum: i32 = *(*m).body_dofnum.add(bid as usize);
                let body_dofadr: i32 = *(*m).body_dofadr.add(bid as usize);
                let mut x: i32 = 0;
                while x < body_dofnum {
                    *(*d).qfrc_spring.add((body_dofadr + x) as usize) += *qfrc.add((3 * v + x) as usize);
                    x += 1;
                }
            }
            v += 1;
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_springdamper (engine/engine_passive.c:626)
/// Calls: mj_actuatorDamping, mj_flexPassiveBend, mj_flexPassiveBendInterp, mj_flexPassiveInterp, mj_flexPassiveStretch, mj_sleepState, mji_addToScl3, mji_copy4, mji_sub3, mji_subQuat, mju_copy, mju_isZero, mju_norm3, mju_normalize4, mju_polyForce
#[allow(unused_variables, non_snake_case)]
pub fn mj_springdamper(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_springdamper(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_springdamper(m, d) }
}

/// C: mj_gravcomp (engine/engine_passive.c:817)
/// Calls: mj_applyFT, mji_scl3, mju_norm3
#[allow(unused_variables, non_snake_case)]
pub fn mj_gravcomp(m: *const mjModel, d: *mut mjData) -> i32 {
    const mjDSBL_GRAVITY: i32 = 1 << 7;
    const mjENBL_SLEEP: i32 = 1 << 4;

    // SAFETY: m, d valid per caller contract. All field accesses use valid model/data arrays.
    unsafe {
        if (*m).ngravcomp == 0
            || ((*m).opt.disableflags & mjDSBL_GRAVITY) != 0
            || crate::engine::engine_util_blas::mju_norm3((*m).opt.gravity.as_ptr()) == 0.0
        {
            return 0;
        }

        let mut has_gravcomp: i32 = 0;
        let mut force: [f64; 3] = [0.0; 3];
        let torque: [f64; 3] = [0.0; 3];
        let sleep_filter: i32 = (((*m).opt.enableflags & mjENBL_SLEEP) != 0
            && (*d).nbody_awake < (*m).nbody as i32) as i32;
        let nbody: i32 = if sleep_filter != 0 { (*d).nbody_awake } else { (*m).nbody as i32 };

        let mut b: i32 = 1;
        while b < nbody {
            let i: i32 = if sleep_filter != 0 {
                *(*d).body_awake_ind.add(b as usize)
            } else {
                b
            };
            if *(*m).body_gravcomp.add(i as usize) != 0.0 {
                has_gravcomp = 1;
                crate::engine::engine_inline::mji_scl3(
                    force.as_mut_ptr(),
                    (*m).opt.gravity.as_ptr(),
                    -(*(*m).body_mass.add(i as usize) * *(*m).body_gravcomp.add(i as usize)),
                );
                crate::engine::engine_support::mj_apply_ft(
                    m, d,
                    force.as_ptr(), torque.as_ptr(),
                    (*d).xipos.add(3 * i as usize),
                    i, (*d).qfrc_gravcomp,
                );
            }
            b += 1;
        }

        has_gravcomp
    }
}

/// C: mj_fluid (engine/engine_passive.c:842)
/// Calls: mj_ellipsoidFluidModel, mj_inertiaBoxFluidModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_fluid(m: *const mjModel, d: *mut mjData) -> i32 {
    const mjENBL_SLEEP: i32 = 1 << 4;
    const mjNFLUID: usize = 12;
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m, d valid per caller contract. All field accesses use valid model/data arrays.
    unsafe {
        // no fluid forces: early return
        if (*m).opt.viscosity == 0.0 && (*m).opt.density == 0.0 {
            return 0;
        }

        let sleep_filter: i32 = (((*m).opt.enableflags & mjENBL_SLEEP) != 0
            && (*d).nbody_awake < (*m).nbody as i32) as i32;
        let nbody: i32 = if sleep_filter != 0 { (*d).nbody_awake } else { (*m).nbody as i32 };

        let mut b: i32 = 0;
        while b < nbody {
            let i: i32 = if sleep_filter != 0 {
                *(*d).body_awake_ind.add(b as usize)
            } else {
                b
            };

            if *(*m).body_mass.add(i as usize) < MJMINVAL {
                b += 1;
                continue;
            }

            // if any child geom uses the ellipsoid model, inertia-box model is disabled
            let mut use_ellipsoid_model: i32 = 0;
            let geomnum: i32 = *(*m).body_geomnum.add(i as usize);
            let mut j: i32 = 0;
            while j < geomnum && use_ellipsoid_model == 0 {
                let geomid: i32 = *(*m).body_geomadr.add(i as usize) + j;
                use_ellipsoid_model += (*(*m).geom_fluid.add(mjNFLUID * geomid as usize) > 0.0) as i32;
                j += 1;
            }

            if use_ellipsoid_model != 0 {
                mj_ellipsoid_fluid_model(m, d, i);
            } else {
                mj_inertia_box_fluid_model(m, d, i);
            }

            b += 1;
        }

        1
    }
}

/// C: mj_contactPassive (engine/engine_passive.c:878)
/// Calls: mj_contactJacobian, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_mulMatMat, mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_passive(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" { fn mj_contactPassive(m: *const mjModel, d: *mut mjData) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_contactPassive(m, d) }
}

/// C: mji_pow4 (engine/engine_passive.c:1215)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow4(val: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (val : f64)
    // Previous return: f64
    (val * val) * (val * val)
}

/// C: mji_pow2 (engine/engine_passive.c:1219)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow2(val: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (val : f64)
    // Previous return: f64
    val * val
}

/// C: mji_ellipsoid_max_moment (engine/engine_passive.c:1223)
/// Calls: mji_pow4, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_ellipsoid_max_moment(size: *const f64, dir: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (size : * const f64, dir : i32)
    // Previous return: f64
    unsafe { let d0 = * size . add (dir as usize) ; let d1 = * size . add (((dir + 1) % 3) as usize) ; let d2 = * size . add (((dir + 2) % 3) as usize) ; let max_d = if d1 > d2 { d1 } else { d2 } ; 8.0 / 15.0 * std :: f64 :: consts :: PI * d0 * mji_pow4 (max_d) }
}

/// C: mj_passive (engine/engine_passive.h:29)
/// Calls: mj_contactPassive, mj_fluid, mj_gravcomp, mj_springdamper, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_add, mju_addInd, mju_addTo, mju_addToInd, mju_message, mju_zero, mju_zeroInd
#[allow(unused_variables, non_snake_case)]
pub fn mj_passive(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_passive(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_passive(m, d) }
}

/// C: mj_inertiaBoxFluidModel (engine/engine_passive.h:37)
/// Calls: mj_applyFT, mj_objectVelocity, mji_copy3, mji_mulMatVec3, mji_scl3, mji_subFrom3, mju_max, mju_transformSpatial, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_inertia_box_fluid_model(m: *const mjModel, d: *mut mjData, i: i32) {
    // SAFETY: m, d valid. i is valid body index.
    unsafe {
        const MJ_MINVAL: f64 = 1e-15;
        const MJ_PI: f64 = std::f64::consts::PI;
        const mjOBJ_BODY: i32 = 1;

        let inertia: *const f64 = (*m).body_inertia.add(3 * i as usize);
        let mass: f64 = *(*m).body_mass.add(i as usize);
        let mut box_: [f64; 3] = [0.0; 3];
        box_[0] = (crate::engine::engine_util_misc::mju_max(MJ_MINVAL,
            (*inertia.add(1) + *inertia.add(2) - *inertia.add(0))) / mass * 6.0).sqrt();
        box_[1] = (crate::engine::engine_util_misc::mju_max(MJ_MINVAL,
            (*inertia.add(0) + *inertia.add(2) - *inertia.add(1))) / mass * 6.0).sqrt();
        box_[2] = (crate::engine::engine_util_misc::mju_max(MJ_MINVAL,
            (*inertia.add(0) + *inertia.add(1) - *inertia.add(2))) / mass * 6.0).sqrt();

        // map from CoM-centered to local body-centered 6D velocity
        let mut lvel: [f64; 6] = [0.0; 6];
        crate::engine::engine_core_util::mj_object_velocity(
            m, d as *const mjData, mjOBJ_BODY, i, lvel.as_mut_ptr(), 1);

        // compute wind in local coordinates
        let mut wind: [f64; 6] = [0.0; 6];
        crate::engine::engine_inline::mji_copy3(wind.as_mut_ptr().add(3), (*m).opt.wind.as_ptr());
        let mut lwind: [f64; 6] = [0.0; 6];
        crate::engine::engine_util_spatial::mju_transform_spatial(
            lwind.as_mut_ptr(), wind.as_ptr(), 0,
            (*d).xipos.add(3 * i as usize),
            (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize),
            (*d).ximat.add(9 * i as usize));

        // subtract translational component from body velocity
        crate::engine::engine_inline::mji_sub_from3(lvel.as_mut_ptr().add(3), lwind.as_ptr().add(3));
        let mut lfrc: [f64; 6] = [0.0; 6];

        // set viscous force and torque
        if (*m).opt.viscosity > 0.0 {
            // diameter of sphere approximation
            let diam: f64 = (box_[0] + box_[1] + box_[2]) / 3.0;

            // angular viscosity
            crate::engine::engine_inline::mji_scl3(
                lfrc.as_mut_ptr(), lvel.as_ptr(), -MJ_PI * diam * diam * diam * (*m).opt.viscosity);

            // linear viscosity
            crate::engine::engine_inline::mji_scl3(
                lfrc.as_mut_ptr().add(3), lvel.as_ptr().add(3), -3.0 * MJ_PI * diam * (*m).opt.viscosity);
        }

        // add lift and drag force and torque
        if (*m).opt.density > 0.0 {
            // force
            lfrc[3] -= 0.5 * (*m).opt.density * box_[1] * box_[2] * lvel[3].abs() * lvel[3];
            lfrc[4] -= 0.5 * (*m).opt.density * box_[0] * box_[2] * lvel[4].abs() * lvel[4];
            lfrc[5] -= 0.5 * (*m).opt.density * box_[0] * box_[1] * lvel[5].abs() * lvel[5];

            // torque
            lfrc[0] -= (*m).opt.density * box_[0]
                * (box_[1] * box_[1] * box_[1] * box_[1] + box_[2] * box_[2] * box_[2] * box_[2])
                * lvel[0].abs() * lvel[0] / 64.0;
            lfrc[1] -= (*m).opt.density * box_[1]
                * (box_[0] * box_[0] * box_[0] * box_[0] + box_[2] * box_[2] * box_[2] * box_[2])
                * lvel[1].abs() * lvel[1] / 64.0;
            lfrc[2] -= (*m).opt.density * box_[2]
                * (box_[0] * box_[0] * box_[0] * box_[0] + box_[1] * box_[1] * box_[1] * box_[1])
                * lvel[2].abs() * lvel[2] / 64.0;
        }

        // rotate to global orientation: lfrc -> bfrc
        let mut bfrc: [f64; 6] = [0.0; 6];
        crate::engine::engine_inline::mji_mul_mat_vec3(
            bfrc.as_mut_ptr(), (*d).ximat.add(9 * i as usize), lfrc.as_ptr());
        crate::engine::engine_inline::mji_mul_mat_vec3(
            bfrc.as_mut_ptr().add(3), (*d).ximat.add(9 * i as usize), lfrc.as_ptr().add(3));

        // apply force and torque to body com
        crate::engine::engine_support::mj_apply_ft(
            m, d, bfrc.as_ptr().add(3), bfrc.as_ptr(), (*d).xipos.add(3 * i as usize), i, (*d).qfrc_fluid);
    }
}

/// C: mj_ellipsoidFluidModel (engine/engine_passive.h:40)
/// Calls: mj_addedMassForces, mj_applyFT, mj_objectVelocity, mj_viscousForces, mji_copy3, mji_mulMatVec3, mji_subFrom3, mju_geomSemiAxes, mju_scl, mju_transformSpatial, mju_zero, readFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mj_ellipsoid_fluid_model(m: *const mjModel, d: *mut mjData, bodyid: i32) {
    extern "C" {
        fn mj_ellipsoidFluidModel(m: *const mjModel, d: *mut mjData, bodyid: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_ellipsoidFluidModel(m, d, bodyid) }
}

/// C: mj_addedMassForces (engine/engine_passive.h:43)
/// Calls: mji_addTo3, mji_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_added_mass_forces(local_vels: *const f64, local_accels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64, local_force: *mut f64) {
    // SAFETY: local_vels has 6 elements, local_accels has 6 (may be null),
    // virtual_mass has 3, virtual_inertia has 3, local_force has 6.
    unsafe {
        let lin_vel: [f64; 3] = [*local_vels.add(3), *local_vels.add(4), *local_vels.add(5)];
        let ang_vel: [f64; 3] = [*local_vels.add(0), *local_vels.add(1), *local_vels.add(2)];
        let virtual_lin_mom: [f64; 3] = [
            fluid_density * *virtual_mass.add(0) * lin_vel[0],
            fluid_density * *virtual_mass.add(1) * lin_vel[1],
            fluid_density * *virtual_mass.add(2) * lin_vel[2],
        ];
        let virtual_ang_mom: [f64; 3] = [
            fluid_density * *virtual_inertia.add(0) * ang_vel[0],
            fluid_density * *virtual_inertia.add(1) * ang_vel[1],
            fluid_density * *virtual_inertia.add(2) * ang_vel[2],
        ];

        // acceleration-dependent terms (disabled in practice but included)
        if !local_accels.is_null() {
            *local_force.add(0) -= fluid_density * *virtual_inertia.add(0) * *local_accels.add(0);
            *local_force.add(1) -= fluid_density * *virtual_inertia.add(1) * *local_accels.add(1);
            *local_force.add(2) -= fluid_density * *virtual_inertia.add(2) * *local_accels.add(2);
            *local_force.add(3) -= fluid_density * *virtual_mass.add(0) * *local_accels.add(3);
            *local_force.add(4) -= fluid_density * *virtual_mass.add(1) * *local_accels.add(4);
            *local_force.add(5) -= fluid_density * *virtual_mass.add(2) * *local_accels.add(5);
        }

        let mut added_mass_force: [f64; 3] = [0.0; 3];
        let mut added_mass_torque1: [f64; 3] = [0.0; 3];
        let mut added_mass_torque2: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(added_mass_force.as_mut_ptr(), virtual_lin_mom.as_ptr(), ang_vel.as_ptr());
        crate::engine::engine_inline::mji_cross(added_mass_torque1.as_mut_ptr(), virtual_lin_mom.as_ptr(), lin_vel.as_ptr());
        crate::engine::engine_inline::mji_cross(added_mass_torque2.as_mut_ptr(), virtual_ang_mom.as_ptr(), ang_vel.as_ptr());

        crate::engine::engine_inline::mji_add_to3(local_force, added_mass_torque1.as_ptr());
        crate::engine::engine_inline::mji_add_to3(local_force, added_mass_torque2.as_ptr());
        crate::engine::engine_inline::mji_add_to3(local_force.add(3), added_mass_force.as_ptr());
    }
}

/// C: mj_viscousForces (engine/engine_passive.h:49)
/// Calls: mji_cross, mji_ellipsoid_max_moment, mji_pow2, mji_pow4, mju_max, mju_min, mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_viscous_forces(local_vels: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, magnus_lift_coef: f64, kutta_lift_coef: f64, blunt_drag_coef: f64, slender_drag_coef: f64, ang_drag_coef: f64, local_force: *mut f64) {
    // SAFETY: local_vels has 6 elements, size has 3, local_force has 6. All valid.
    unsafe {
        const MJPI: f64 = 3.14159265358979323846;
        const MJMINVAL: f64 = 1e-15;

        let lin_vel: [f64; 3] = [*local_vels.add(3), *local_vels.add(4), *local_vels.add(5)];
        let ang_vel: [f64; 3] = [*local_vels.add(0), *local_vels.add(1), *local_vels.add(2)];
        let volume: f64 = 4.0 / 3.0 * MJPI * *size.add(0) * *size.add(1) * *size.add(2);
        let d_max: f64 = crate::engine::engine_util_misc::mju_max(
            crate::engine::engine_util_misc::mju_max(*size.add(0), *size.add(1)), *size.add(2));
        let d_min: f64 = crate::engine::engine_util_misc::mju_min(
            crate::engine::engine_util_misc::mju_min(*size.add(0), *size.add(1)), *size.add(2));
        let d_mid: f64 = *size.add(0) + *size.add(1) + *size.add(2) - d_max - d_min;
        let A_max: f64 = MJPI * d_max * d_mid;

        let mut magnus_force: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(magnus_force.as_mut_ptr(), ang_vel.as_ptr(), lin_vel.as_ptr());
        magnus_force[0] *= magnus_lift_coef * fluid_density * volume;
        magnus_force[1] *= magnus_lift_coef * fluid_density * volume;
        magnus_force[2] *= magnus_lift_coef * fluid_density * volume;

        // projected area computation
        let proj_denom: f64 = mji_pow4(*size.add(1) * *size.add(2)) * mji_pow2(lin_vel[0])
            + mji_pow4(*size.add(2) * *size.add(0)) * mji_pow2(lin_vel[1])
            + mji_pow4(*size.add(0) * *size.add(1)) * mji_pow2(lin_vel[2]);
        let proj_num: f64 = mji_pow2(*size.add(1) * *size.add(2) * lin_vel[0])
            + mji_pow2(*size.add(2) * *size.add(0) * lin_vel[1])
            + mji_pow2(*size.add(0) * *size.add(1) * lin_vel[2]);

        let A_proj: f64 = MJPI * (proj_denom / crate::engine::engine_util_misc::mju_max(MJMINVAL, proj_num)).sqrt();

        // not-unit normal to ellipsoid's projected area
        let norm: [f64; 3] = [
            mji_pow2(*size.add(1) * *size.add(2)) * lin_vel[0],
            mji_pow2(*size.add(2) * *size.add(0)) * lin_vel[1],
            mji_pow2(*size.add(0) * *size.add(1)) * lin_vel[2],
        ];

        // cosine between velocity and normal
        let cos_alpha: f64 = proj_num / crate::engine::engine_util_misc::mju_max(
            MJMINVAL, crate::engine::engine_util_blas::mju_norm3(lin_vel.as_ptr()) * proj_denom);

        let mut kutta_circ: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(kutta_circ.as_mut_ptr(), norm.as_ptr(), lin_vel.as_ptr());
        kutta_circ[0] *= kutta_lift_coef * fluid_density * cos_alpha * A_proj;
        kutta_circ[1] *= kutta_lift_coef * fluid_density * cos_alpha * A_proj;
        kutta_circ[2] *= kutta_lift_coef * fluid_density * cos_alpha * A_proj;
        let mut kutta_force: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(kutta_force.as_mut_ptr(), kutta_circ.as_ptr(), lin_vel.as_ptr());

        // viscous force and torque in Stokes flow
        let eq_sphere_D: f64 = 2.0 / 3.0 * (*size.add(0) + *size.add(1) + *size.add(2));
        let lin_visc_force_coef: f64 = 3.0 * MJPI * eq_sphere_D;
        let lin_visc_torq_coef: f64 = MJPI * eq_sphere_D * eq_sphere_D * eq_sphere_D;

        // moments of inertia for angular quadratic drag
        let I_max: f64 = 8.0 / 15.0 * MJPI * d_mid * mji_pow4(d_max);
        let II: [f64; 3] = [
            mji_ellipsoid_max_moment(size, 0),
            mji_ellipsoid_max_moment(size, 1),
            mji_ellipsoid_max_moment(size, 2),
        ];
        let mom_visc: [f64; 3] = [
            ang_vel[0] * (ang_drag_coef * II[0] + slender_drag_coef * (I_max - II[0])),
            ang_vel[1] * (ang_drag_coef * II[1] + slender_drag_coef * (I_max - II[1])),
            ang_vel[2] * (ang_drag_coef * II[2] + slender_drag_coef * (I_max - II[2])),
        ];

        let drag_lin_coef: f64 = fluid_viscosity * lin_visc_force_coef
            + fluid_density * crate::engine::engine_util_blas::mju_norm3(lin_vel.as_ptr())
            * (A_proj * blunt_drag_coef + slender_drag_coef * (A_max - A_proj));
        let drag_ang_coef: f64 = fluid_viscosity * lin_visc_torq_coef
            + fluid_density * crate::engine::engine_util_blas::mju_norm3(mom_visc.as_ptr());

        *local_force.add(0) -= drag_ang_coef * ang_vel[0];
        *local_force.add(1) -= drag_ang_coef * ang_vel[1];
        *local_force.add(2) -= drag_ang_coef * ang_vel[2];
        *local_force.add(3) += magnus_force[0] + kutta_force[0] - drag_lin_coef * lin_vel[0];
        *local_force.add(4) += magnus_force[1] + kutta_force[1] - drag_lin_coef * lin_vel[1];
        *local_force.add(5) += magnus_force[2] + kutta_force[2] - drag_lin_coef * lin_vel[2];
    }
}

/// C: readFluidGeomInteraction (engine/engine_passive.h:56)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn read_fluid_geom_interaction(geom_fluid_coefs: *const f64, geom_fluid_coef: *mut f64, blunt_drag_coef: *mut f64, slender_drag_coef: *mut f64, ang_drag_coef: *mut f64, kutta_lift_coef: *mut f64, magnus_lift_coef: *mut f64, virtual_mass: *mut f64, virtual_inertia: *mut f64) {
    // SAFETY: geom_fluid_coefs has mjNFLUID (12) elements. All output pointers valid.
    unsafe {
        let mut i: usize = 0;
        *geom_fluid_coef = *geom_fluid_coefs.add(i); i += 1;
        *blunt_drag_coef = *geom_fluid_coefs.add(i); i += 1;
        *slender_drag_coef = *geom_fluid_coefs.add(i); i += 1;
        *ang_drag_coef = *geom_fluid_coefs.add(i); i += 1;
        *kutta_lift_coef = *geom_fluid_coefs.add(i); i += 1;
        *magnus_lift_coef = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(0) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(1) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(2) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(0) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(1) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(2) = *geom_fluid_coefs.add(i); i += 1;
        // mjNFLUID = 12, assert i == 12
    }
}

/// C: writeFluidGeomInteraction (engine/engine_passive.h:66)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn write_fluid_geom_interaction(geom_fluid_coefs: *mut f64, geom_fluid_coef: *const f64, blunt_drag_coef: *const f64, slender_drag_coef: *const f64, ang_drag_coef: *const f64, kutta_lift_coef: *const f64, magnus_lift_coef: *const f64, virtual_mass: *const f64, virtual_inertia: *const f64) {
    const MJ_NFLUID: i32 = 12;
    // SAFETY: all pointers are valid per caller contract; geom_fluid_coefs has mjNFLUID (12) elements
    unsafe {
        let mut i: i32 = 0;
        *geom_fluid_coefs.add(i as usize) = *geom_fluid_coef.add(0); i += 1;
        *geom_fluid_coefs.add(i as usize) = *blunt_drag_coef.add(0); i += 1;
        *geom_fluid_coefs.add(i as usize) = *slender_drag_coef.add(0); i += 1;
        *geom_fluid_coefs.add(i as usize) = *ang_drag_coef.add(0); i += 1;
        *geom_fluid_coefs.add(i as usize) = *kutta_lift_coef.add(0); i += 1;
        *geom_fluid_coefs.add(i as usize) = *magnus_lift_coef.add(0); i += 1;
        *geom_fluid_coefs.add(i as usize) = *virtual_mass.add(0); i += 1;
        *geom_fluid_coefs.add(i as usize) = *virtual_mass.add(1); i += 1;
        *geom_fluid_coefs.add(i as usize) = *virtual_mass.add(2); i += 1;
        *geom_fluid_coefs.add(i as usize) = *virtual_inertia.add(0); i += 1;
        *geom_fluid_coefs.add(i as usize) = *virtual_inertia.add(1); i += 1;
        *geom_fluid_coefs.add(i as usize) = *virtual_inertia.add(2); i += 1;
        if i != MJ_NFLUID {
            crate::engine::engine_util_errmem::mju_error(b"wrong number of entries.\0".as_ptr() as *const i8);
        }
    }
}

