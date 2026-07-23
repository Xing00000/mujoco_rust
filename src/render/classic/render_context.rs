//! Port of: render/classic/render_context.c
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

use crate :: render :: classic :: render_util :: mjr_normalize_vec ;

/// C: listAllocate (render/classic/render_context.c:61)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn list_allocate(base: *mut u32, range: *mut i32, newrange: i32) {
    todo!() // listAllocate
}

/// C: makePlane (render/classic/render_context.c:75)
/// Calls: listAllocate, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn make_plane(m: *const mjModel, con: *mut mjrContext) {
    todo!() // makePlane
}

/// C: makeMesh (render/classic/render_context.c:198)
/// Calls: listAllocate, mjr_uploadMesh
#[allow(unused_variables, non_snake_case)]
pub fn make_mesh(m: *const mjModel, con: *mut mjrContext) {
    todo!() // makeMesh
}

/// C: makeHField (render/classic/render_context.c:389)
/// Calls: listAllocate, mjr_uploadHField
#[allow(unused_variables, non_snake_case)]
pub fn make_h_field(m: *const mjModel, con: *mut mjrContext) {
    todo!() // makeHField
}

/// C: setVertexSphere (render/classic/render_context.c:500)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_sphere(v: *mut f32, n: *mut f32, az: f32, el: f32, sign: i32) {
    // SAFETY: caller guarantees v and n point to at least 3 f32 elements
    unsafe {
        *v.add(0) = az.cos() * el.cos();
        *v.add(1) = az.sin() * el.cos();
        *v.add(2) = sign as f32 + el.sin();

        *n.add(0) = *v.add(0);
        *n.add(1) = *v.add(1);
        *n.add(2) = *v.add(2) - sign as f32;
    }
}

/// C: halfSphere (render/classic/render_context.c:512)
/// Calls: setVertexSphere
#[allow(unused_variables, non_snake_case)]
pub fn half_sphere(sign: i32, nSlice: i32, nStack: i32) {
    const GL_TRIANGLES: u32 = 0x0004;
    const GL_QUADS: u32 = 0x0007;
    const MJ_PI: f32 = std::f32::consts::PI;

    extern "C" {
        fn glBegin(mode: u32);
        fn glEnd();
        fn glNormal3fv(v: *const f32);
        fn glVertex3fv(v: *const f32);
    }

    let mut az1: f32;
    let mut az2: f32;
    let mut el1: f32;
    let mut el2: f32;
    let mut v1 = [0.0f32; 3];
    let mut v2 = [0.0f32; 3];
    let mut v3 = [0.0f32; 3];
    let mut v4 = [0.0f32; 3];
    let mut n1 = [0.0f32; 3];
    let mut n2 = [0.0f32; 3];
    let mut n3 = [0.0f32; 3];
    let mut n4 = [0.0f32; 3];

    // SAFETY: GL functions are linked from mujoco C library; all array pointers are stack-local
    unsafe {
        // pole: use triangles
        glBegin(GL_TRIANGLES);
        el1 = (MJ_PI / 2.0f32 * sign as f32 * (nStack - 1) as f32) / nStack as f32;
        for j in 0..nSlice {
            az1 = (2.0f32 * MJ_PI * (j as f32 + 0.0f32)) / nSlice as f32;
            az2 = (2.0f32 * MJ_PI * (j as f32 + 1.0f32)) / nSlice as f32;

            // compute triangle vertices
            set_vertex_sphere(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, el1, sign);
            set_vertex_sphere(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, el1, sign);
            v3[0] = 0.0f32;
            v3[1] = 0.0f32;
            v3[2] = 2.0f32 * sign as f32;
            n3[0] = 0.0f32;
            n3[1] = 0.0f32;
            n3[2] = sign as f32;

            // make triangle
            if sign > 0 {
                glNormal3fv(n1.as_ptr());
                glVertex3fv(v1.as_ptr());
                glNormal3fv(n2.as_ptr());
                glVertex3fv(v2.as_ptr());
                glNormal3fv(n3.as_ptr());
                glVertex3fv(v3.as_ptr());
            } else {
                glNormal3fv(n3.as_ptr());
                glVertex3fv(v3.as_ptr());
                glNormal3fv(n2.as_ptr());
                glVertex3fv(v2.as_ptr());
                glNormal3fv(n1.as_ptr());
                glVertex3fv(v1.as_ptr());
            }
        }
        glEnd();

        // the rest: use quads
        glBegin(GL_QUADS);
        for i in 0..(nStack - 1) {
            el1 = (MJ_PI / 2.0f32 * sign as f32 * (i + 0) as f32) / nStack as f32;
            el2 = (MJ_PI / 2.0f32 * sign as f32 * (i + 1) as f32) / nStack as f32;

            for j in 0..nSlice {
                az1 = (2.0f32 * MJ_PI * (j + 0) as f32) / nSlice as f32;
                az2 = (2.0f32 * MJ_PI * (j + 1) as f32) / nSlice as f32;

                // compute quad vertices
                set_vertex_sphere(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, el1, sign);
                set_vertex_sphere(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, el1, sign);
                set_vertex_sphere(v3.as_mut_ptr(), n3.as_mut_ptr(), az2, el2, sign);
                set_vertex_sphere(v4.as_mut_ptr(), n4.as_mut_ptr(), az1, el2, sign);

                // make quad
                if sign > 0 {
                    glNormal3fv(n1.as_ptr());
                    glVertex3fv(v1.as_ptr());
                    glNormal3fv(n2.as_ptr());
                    glVertex3fv(v2.as_ptr());
                    glNormal3fv(n3.as_ptr());
                    glVertex3fv(v3.as_ptr());
                    glNormal3fv(n4.as_ptr());
                    glVertex3fv(v4.as_ptr());
                } else {
                    glNormal3fv(n4.as_ptr());
                    glVertex3fv(v4.as_ptr());
                    glNormal3fv(n3.as_ptr());
                    glVertex3fv(v3.as_ptr());
                    glNormal3fv(n2.as_ptr());
                    glVertex3fv(v2.as_ptr());
                    glNormal3fv(n1.as_ptr());
                    glVertex3fv(v1.as_ptr());
                }
            }
        }
        glEnd();
    }
}

/// C: sphere (render/classic/render_context.c:595)
/// Calls: setVertexSphere
#[allow(unused_variables, non_snake_case)]
pub fn sphere(nSlice: i32, nStack: i32) {
    const GL_TRIANGLES: u32 = 0x0004;
    const GL_QUADS: u32 = 0x0007;
    const MJ_PI: f32 = std::f32::consts::PI;

    extern "C" {
        fn glBegin(mode: u32);
        fn glEnd();
        fn glNormal3fv(v: *const f32);
        fn glVertex3fv(v: *const f32);
    }

    let mut az1: f32;
    let mut az2: f32;
    let mut el1: f32;
    let mut el2: f32;
    let mut v1 = [0.0f32; 3];
    let mut v2 = [0.0f32; 3];
    let mut v3 = [0.0f32; 3];
    let mut v4 = [0.0f32; 3];
    let mut n1 = [0.0f32; 3];
    let mut n2 = [0.0f32; 3];
    let mut n3 = [0.0f32; 3];
    let mut n4 = [0.0f32; 3];

    // SAFETY: GL functions are linked from mujoco C library; all array pointers are stack-local
    unsafe {
        // poles: use triangles
        glBegin(GL_TRIANGLES);
        let mut sign: i32 = -1;
        while sign <= 1 {
            el1 = (0.5 * MJ_PI * sign as f32 * (nStack / 2 - 1) as f32) / (nStack / 2) as f32;
            for j in 0..nSlice {
                az1 = (2.0f32 * MJ_PI * (j as f32 + 0.0f32)) / nSlice as f32;
                az2 = (2.0f32 * MJ_PI * (j as f32 + 1.0f32)) / nSlice as f32;

                // compute triangle vertices
                set_vertex_sphere(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, el1, 0);
                set_vertex_sphere(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, el1, 0);
                v3[0] = 0.0f32;
                v3[1] = 0.0f32;
                v3[2] = sign as f32;
                n3[0] = 0.0f32;
                n3[1] = 0.0f32;
                n3[2] = sign as f32;

                // make triangle
                if sign > 0 {
                    glNormal3fv(n1.as_ptr());
                    glVertex3fv(v1.as_ptr());
                    glNormal3fv(n2.as_ptr());
                    glVertex3fv(v2.as_ptr());
                    glNormal3fv(n3.as_ptr());
                    glVertex3fv(v3.as_ptr());
                } else {
                    glNormal3fv(n3.as_ptr());
                    glVertex3fv(v3.as_ptr());
                    glNormal3fv(n2.as_ptr());
                    glVertex3fv(v2.as_ptr());
                    glNormal3fv(n1.as_ptr());
                    glVertex3fv(v1.as_ptr());
                }
            }
            sign += 2;
        }
        glEnd();

        // the rest: use quads
        glBegin(GL_QUADS);
        sign = -1;
        while sign <= 1 {
            for i in 0..(nStack / 2 - 1) {
                el1 = (0.5 * MJ_PI * sign as f32 * (i + 0) as f32) / (nStack / 2) as f32;
                el2 = (0.5 * MJ_PI * sign as f32 * (i + 1) as f32) / (nStack / 2) as f32;

                for j in 0..nSlice {
                    az1 = (2.0f32 * MJ_PI * (j + 0) as f32) / nSlice as f32;
                    az2 = (2.0f32 * MJ_PI * (j + 1) as f32) / nSlice as f32;

                    // compute quad vertices
                    set_vertex_sphere(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, el1, 0);
                    set_vertex_sphere(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, el1, 0);
                    set_vertex_sphere(v3.as_mut_ptr(), n3.as_mut_ptr(), az2, el2, 0);
                    set_vertex_sphere(v4.as_mut_ptr(), n4.as_mut_ptr(), az1, el2, 0);

                    // make quad
                    if sign > 0 {
                        glNormal3fv(n1.as_ptr());
                        glVertex3fv(v1.as_ptr());
                        glNormal3fv(n2.as_ptr());
                        glVertex3fv(v2.as_ptr());
                        glNormal3fv(n3.as_ptr());
                        glVertex3fv(v3.as_ptr());
                        glNormal3fv(n4.as_ptr());
                        glVertex3fv(v4.as_ptr());
                    } else {
                        glNormal3fv(n4.as_ptr());
                        glVertex3fv(v4.as_ptr());
                        glNormal3fv(n3.as_ptr());
                        glVertex3fv(v3.as_ptr());
                        glNormal3fv(n2.as_ptr());
                        glVertex3fv(v2.as_ptr());
                        glNormal3fv(n1.as_ptr());
                        glVertex3fv(v1.as_ptr());
                    }
                }
            }
            sign += 2;
        }
        glEnd();
    }
}

/// C: setVertexDisk (render/classic/render_context.c:682)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_disk(v: *mut f32, az: f32, r: f32, sign: i32) {
    // SAFETY: caller guarantees v points to at least 3 f32 elements
    unsafe {
        *v.add(0) = az.cos() * r;
        *v.add(1) = az.sin() * r;
        *v.add(2) = sign as f32;
    }
}

/// C: disk (render/classic/render_context.c:690)
/// Calls: setVertexDisk
#[allow(unused_variables, non_snake_case)]
pub fn disk(sign: i32, nSlice: i32, nStack: i32) {
    const GL_TRIANGLES: u32 = 0x0004;
    const GL_QUADS: u32 = 0x0007;
    const MJ_PI: f32 = std::f32::consts::PI;

    extern "C" {
        fn glBegin(mode: u32);
        fn glEnd();
        fn glNormal3fv(v: *const f32);
        fn glVertex3fv(v: *const f32);
    }

    let mut az1: f32;
    let mut az2: f32;
    let mut r1: f32;
    let mut r2: f32;
    let mut v1 = [0.0f32; 3];
    let mut v2 = [0.0f32; 3];
    let mut v3 = [0.0f32; 3];
    let mut v4 = [0.0f32; 3];
    let normal: [f32; 3] = [0.0, 0.0, if sign == 0 { -1.0f32 } else { sign as f32 }];

    // SAFETY: GL functions linked from mujoco C library; all array pointers are stack-local
    unsafe {
        // pole: use triangles
        glBegin(GL_TRIANGLES);
        glNormal3fv(normal.as_ptr());
        r1 = 1.0f32 / nStack as f32;
        for j in 0..nSlice {
            az1 = (2.0f32 * MJ_PI * (j as f32 + 0.0f32)) / nSlice as f32;
            az2 = (2.0f32 * MJ_PI * (j as f32 + 1.0f32)) / nSlice as f32;

            // compute triangle vertices
            set_vertex_disk(v1.as_mut_ptr(), az1, r1, sign);
            set_vertex_disk(v2.as_mut_ptr(), az2, r1, sign);
            v3[0] = 0.0f32;
            v3[1] = 0.0f32;
            v3[2] = sign as f32;

            // make triangle
            if sign > 0 {
                glVertex3fv(v1.as_ptr());
                glVertex3fv(v2.as_ptr());
                glVertex3fv(v3.as_ptr());
            } else {
                glVertex3fv(v3.as_ptr());
                glVertex3fv(v2.as_ptr());
                glVertex3fv(v1.as_ptr());
            }
        }
        glEnd();

        // the rest: use quads
        glBegin(GL_QUADS);
        glNormal3fv(normal.as_ptr());
        for i in 0..(nStack - 1) {
            r1 = (i + 1) as f32 / nStack as f32;
            r2 = (i + 2) as f32 / nStack as f32;

            for j in 0..nSlice {
                az1 = (2.0f32 * MJ_PI * (j as f32 + 0.0f32)) / nSlice as f32;
                az2 = (2.0f32 * MJ_PI * (j as f32 + 1.0f32)) / nSlice as f32;

                // compute quad vertices
                set_vertex_disk(v1.as_mut_ptr(), az1, r2, sign);
                set_vertex_disk(v2.as_mut_ptr(), az2, r2, sign);
                set_vertex_disk(v3.as_mut_ptr(), az2, r1, sign);
                set_vertex_disk(v4.as_mut_ptr(), az1, r1, sign);

                // make quad
                if sign > 0 {
                    glVertex3fv(v1.as_ptr());
                    glVertex3fv(v2.as_ptr());
                    glVertex3fv(v3.as_ptr());
                    glVertex3fv(v4.as_ptr());
                } else {
                    glVertex3fv(v4.as_ptr());
                    glVertex3fv(v3.as_ptr());
                    glVertex3fv(v2.as_ptr());
                    glVertex3fv(v1.as_ptr());
                }
            }
        }
        glEnd();
    }
}

/// C: setVertexCone (render/classic/render_context.c:759)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cone(v: *mut f32, n: *mut f32, az: f32, r: f32) {
    let scale: f32 = 1.0f32 / 2.0f32.sqrt();

    // SAFETY: caller guarantees v and n point to at least 3 f32 elements
    unsafe {
        // vertex
        *v.add(0) = az.cos() * r;
        *v.add(1) = az.sin() * r;
        *v.add(2) = 1.0f32 - r;

        // normal
        *n.add(0) = az.cos() * scale;
        *n.add(1) = az.sin() * scale;
        *n.add(2) = scale;
    }
}

/// C: cone (render/classic/render_context.c:775)
/// Calls: mjr_normalizeVec, setVertexCone
#[allow(unused_variables, non_snake_case)]
pub fn cone(nSlice: i32, nStack: i32) {
    const GL_TRIANGLES: u32 = 0x0004;
    const GL_QUADS: u32 = 0x0007;
    const MJ_PI: f32 = std::f32::consts::PI;

    extern "C" {
        fn glBegin(mode: u32);
        fn glEnd();
        fn glNormal3fv(v: *const f32);
        fn glVertex3fv(v: *const f32);
    }

    let mut az1: f32;
    let mut az2: f32;
    let mut r1: f32;
    let mut r2: f32;
    let mut v1 = [0.0f32; 3];
    let mut v2 = [0.0f32; 3];
    let mut v3 = [0.0f32; 3];
    let mut v4 = [0.0f32; 3];
    let mut n1 = [0.0f32; 3];
    let mut n2 = [0.0f32; 3];
    let mut n3 = [0.0f32; 3];
    let mut n4 = [0.0f32; 3];

    // SAFETY: GL functions linked from mujoco C library; all array pointers are stack-local
    unsafe {
        // pole: use triangles
        glBegin(GL_TRIANGLES);
        r1 = 1.0f32 / nStack as f32;
        for j in 0..nSlice {
            az1 = (2.0f32 * MJ_PI * (j as f32 + 0.0f32)) / nSlice as f32;
            az2 = (2.0f32 * MJ_PI * (j as f32 + 1.0f32)) / nSlice as f32;

            // compute triangle vertices
            set_vertex_cone(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, r1);
            set_vertex_cone(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, r1);
            v3[0] = 0.0f32;
            v3[1] = 0.0f32;
            v3[2] = 1.0f32;
            n3[0] = n1[0] + n2[0];
            n3[1] = n1[1] + n2[1];
            n3[2] = n1[2] + n2[2];
            mjr_normalize_vec(n3.as_mut_ptr());

            // make triangle
            glNormal3fv(n1.as_ptr());
            glVertex3fv(v1.as_ptr());
            glNormal3fv(n2.as_ptr());
            glVertex3fv(v2.as_ptr());
            glNormal3fv(n3.as_ptr());
            glVertex3fv(v3.as_ptr());
        }
        glEnd();

        // the rest: use quads
        glBegin(GL_QUADS);
        for i in 0..(nStack - 1) {
            r1 = (i + 1) as f32 / nStack as f32;
            r2 = (i + 2) as f32 / nStack as f32;

            for j in 0..nSlice {
                az1 = (2.0f32 * MJ_PI * (j as f32 + 0.0f32)) / nSlice as f32;
                az2 = (2.0f32 * MJ_PI * (j as f32 + 1.0f32)) / nSlice as f32;

                // compute quad vertices
                set_vertex_cone(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, r2);
                set_vertex_cone(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, r2);
                set_vertex_cone(v3.as_mut_ptr(), n3.as_mut_ptr(), az2, r1);
                set_vertex_cone(v4.as_mut_ptr(), n4.as_mut_ptr(), az1, r1);

                // make quad
                glNormal3fv(n1.as_ptr());
                glVertex3fv(v1.as_ptr());
                glNormal3fv(n2.as_ptr());
                glVertex3fv(v2.as_ptr());
                glNormal3fv(n3.as_ptr());
                glVertex3fv(v3.as_ptr());
                glNormal3fv(n4.as_ptr());
                glVertex3fv(v4.as_ptr());
            }
        }
        glEnd();
    }
}

/// C: setVertexCylinder (render/classic/render_context.c:840)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cylinder(v: *mut f32, n: *mut f32, az: f32, h: f32) {
    // SAFETY: caller guarantees v and n point to at least 3 f32 elements
    unsafe {
        *v.add(0) = az.cos();
        *v.add(1) = az.sin();
        *v.add(2) = h;

        let len: f32 = (*v.add(0) * *v.add(0) + *v.add(1) * *v.add(1)).sqrt();
        *n.add(0) = *v.add(0) / len;
        *n.add(1) = *v.add(1) / len;
        *n.add(2) = 0.0f32;
    }
}

/// C: cylinder (render/classic/render_context.c:852)
/// Calls: setVertexCylinder
#[allow(unused_variables, non_snake_case)]
pub fn cylinder(nSlice: i32, nStack: i32) {
    const GL_QUADS: u32 = 0x0007;
    const MJ_PI: f32 = std::f32::consts::PI;

    extern "C" {
        fn glBegin(mode: u32);
        fn glEnd();
        fn glNormal3fv(v: *const f32);
        fn glVertex3fv(v: *const f32);
    }

    let mut az1: f32;
    let mut az2: f32;
    let mut h1: f32;
    let mut h2: f32;
    let mut v1 = [0.0f32; 3];
    let mut v2 = [0.0f32; 3];
    let mut v3 = [0.0f32; 3];
    let mut v4 = [0.0f32; 3];
    let mut n1 = [0.0f32; 3];
    let mut n2 = [0.0f32; 3];
    let mut n3 = [0.0f32; 3];
    let mut n4 = [0.0f32; 3];

    // SAFETY: GL functions linked from mujoco C library; all array pointers are stack-local
    unsafe {
        // use quads everywhere
        glBegin(GL_QUADS);
        for i in 0..nStack {
            h1 = 2.0f32 * (i as f32 + 0.0f32) / nStack as f32 - 1.0f32;
            h2 = 2.0f32 * (i as f32 + 1.0f32) / nStack as f32 - 1.0f32;

            for j in 0..nSlice {
                az1 = (2.0f32 * MJ_PI * (j as f32 + 0.0f32)) / nSlice as f32;
                az2 = (2.0f32 * MJ_PI * (j as f32 + 1.0f32)) / nSlice as f32;

                // compute quad vertices
                set_vertex_cylinder(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, h1);
                set_vertex_cylinder(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, h1);
                set_vertex_cylinder(v3.as_mut_ptr(), n3.as_mut_ptr(), az2, h2);
                set_vertex_cylinder(v4.as_mut_ptr(), n4.as_mut_ptr(), az1, h2);

                // make quad
                glNormal3fv(n1.as_ptr());
                glVertex3fv(v1.as_ptr());
                glNormal3fv(n2.as_ptr());
                glVertex3fv(v2.as_ptr());
                glNormal3fv(n3.as_ptr());
                glVertex3fv(v3.as_ptr());
                glNormal3fv(n4.as_ptr());
                glVertex3fv(v4.as_ptr());
            }
        }
        glEnd();
    }
}

/// C: setVertexHaze (render/classic/render_context.c:890)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_haze(v: *mut f32, az: f32, h: f32, r: f32) {
    // SAFETY: caller guarantees v points to at least 3 writable f32 elements
    unsafe {
        *v.add(0) = az.cos() * (1.0 - r * (1.0 - h));
        *v.add(1) = az.sin() * (1.0 - r * (1.0 - h));
        *v.add(2) = h;
    }
}

/// C: haze (render/classic/render_context.c:898)
/// Calls: setVertexHaze
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn haze(nSlice: i32, r: f32, rgba: *const f32) {
    const GL_QUADS: u32 = 0x0007;
    const MJ_PI: f32 = std::f32::consts::PI;

    extern "C" {
        fn glBegin(mode: u32);
        fn glEnd();
        fn glNormal3f(nx: f32, ny: f32, nz: f32);
        fn glVertex3fv(v: *const f32);
        fn glColor4f(r: f32, g: f32, b: f32, a: f32);
    }

    // SAFETY: GL functions linked from mujoco C library; rgba is caller-guaranteed valid
    unsafe {
        // compute elevation h for transparency transition point
        let alpha: f32 = (1.0f32).atan2(r);
        let beta: f32 = (0.75f32 * MJ_PI) - alpha;
        let h: f32 = (0.5f32).sqrt() * r * alpha.sin() / beta.sin();

        // use quads everywhere
        glBegin(GL_QUADS);

        // normal not needed (always rendered with lighting off)
        glNormal3f(0.0f32, 0.0f32, 1.0f32);

        // stacks = 2
        for i in 0..2_i32 {
            let h1: f32 = if i == 0 { 0.0f32 } else { h };
            let h2: f32 = if i == 0 { h } else { 1.0f32 };

            for j in 0..nSlice {
                let az1: f32 = (2.0f32 * MJ_PI * (j as f32 + 0.0f32)) / nSlice as f32;
                let az2: f32 = (2.0f32 * MJ_PI * (j as f32 + 1.0f32)) / nSlice as f32;

                // compute quad vertices
                let mut v1 = [0.0f32; 3];
                let mut v2 = [0.0f32; 3];
                let mut v3 = [0.0f32; 3];
                let mut v4 = [0.0f32; 3];
                set_vertex_haze(v1.as_mut_ptr(), az1, h1, r);
                set_vertex_haze(v2.as_mut_ptr(), az2, h1, r);
                set_vertex_haze(v3.as_mut_ptr(), az2, h2, r);
                set_vertex_haze(v4.as_mut_ptr(), az1, h2, r);

                // colors at elevation h1 and h2
                let c1: f32 = if i == 1 { 1.0f32 } else { 0.0f32 };
                let c2: f32 = if i == 0 { 1.0f32 } else { 0.0f32 };

                // make quad, with colors
                glColor4f(*rgba.add(0), *rgba.add(1), *rgba.add(2), c1);
                glVertex3fv(v1.as_ptr());
                glVertex3fv(v2.as_ptr());
                glColor4f(*rgba.add(0), *rgba.add(1), *rgba.add(2), c2);
                glVertex3fv(v3.as_ptr());
                glVertex3fv(v4.as_ptr());
            }
        }
        glEnd();
    }
}

/// C: makeBuiltin (render/classic/render_context.c:945)
/// Calls: cone, cylinder, disk, halfSphere, haze, listAllocate, sphere
#[allow(unused_variables, non_snake_case)]
pub fn make_builtin(m: *const mjModel, con: *mut mjrContext) {
    todo!() // makeBuiltin
}

/// C: makeShadow (render/classic/render_context.c:1041)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_shadow(m: *const mjModel, con: *mut mjrContext) {
    todo!() // makeShadow
}

/// C: makeOff (render/classic/render_context.c:1094)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_off(con: *mut mjrContext) {
    todo!() // makeOff
}

/// C: makeFont (render/classic/render_context.c:1195)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_font(con: *mut mjrContext, fontscale: i32) {
    todo!() // makeFont
}

/// C: makeMaterial (render/classic/render_context.c:1303)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_material(m: *const mjModel, con: *mut mjrContext) {
    todo!() // makeMaterial
}

/// C: makeTexture (render/classic/render_context.c:1341)
/// Calls: mjr_uploadTexture, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_texture(m: *const mjModel, con: *mut mjrContext) {
    todo!() // makeTexture
}

/// C: makeSkin (render/classic/render_context.c:1457)
/// Calls: mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn make_skin(m: *const mjModel, con: *mut mjrContext) {
    todo!() // makeSkin
}

/// C: debugCallback (render/classic/render_context.c:1504)
#[allow(unused_variables, non_snake_case)]
pub fn debug_callback(source: u32, r#type: u32, id: u32, severity: u32, length: i32, message: *const i8, userParam: *const ()) {
    todo!() // debugCallback
}

/// C: glDebugEnabled (render/classic/render_context.c:1518)
#[allow(unused_variables, non_snake_case)]
pub fn gl_debug_enabled() -> i32 {
    extern "C" {
        fn getenv(name: *const i8) -> *mut i8;
        fn strcmp(s1: *const i8, s2: *const i8) -> i32;
    }

    // SAFETY: getenv and strcmp are standard C library functions; string literals are null-terminated
    unsafe {
        let debug = getenv(b"MUJOCO_GL_DEBUG\0".as_ptr() as *const i8);
        if !debug.is_null() && strcmp(debug, b"1\0".as_ptr() as *const i8) == 0 {
            1
        } else {
            0
        }
    }
}

/// C: mjr_makeContext_offSize (render/classic/render_context.c:1525)
/// Calls: glDebugEnabled, makeBuiltin, makeFont, makeHField, makeMaterial, makeMesh, makeOff, makePlane, makeShadow, makeSkin, makeTexture, mjGladLoadGL, mjr_freeContext, mjr_setBuffer, mju_error, mju_round, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context_off_size(m: *const mjModel, con: *mut mjrContext, fontscale: i32, default_offwidth: i32, default_offheight: i32) {
    todo!() // mjr_makeContext_offSize
}

/// C: mjr_defaultContext (render/classic/render_context.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_default_context(con: *mut mjrContext) {
    todo!() // mjr_defaultContext
}

/// C: mjr_makeContext (render/classic/render_context.h:45)
/// Calls: mjr_makeContext_offSize
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context(m: *const mjModel, con: *mut mjrContext, fontscale: i32) {
    todo!() // mjr_makeContext
}

/// C: mjr_changeFont (render/classic/render_context.h:48)
/// Calls: makeFont
#[allow(unused_variables, non_snake_case)]
pub fn mjr_change_font(fontscale: i32, con: *mut mjrContext) {
    todo!() // mjr_changeFont
}

/// C: mjr_addAux (render/classic/render_context.h:51)
/// Calls: mjr_restoreBuffer, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_add_aux(index: i32, width: i32, height: i32, samples: i32, con: *mut mjrContext) {
    todo!() // mjr_addAux
}

/// C: mjr_freeContext (render/classic/render_context.h:54)
/// Calls: mjr_defaultContext, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mjr_free_context(con: *mut mjrContext) {
    todo!() // mjr_freeContext
}

/// C: mjr_resizeOffscreen (render/classic/render_context.h:57)
/// Calls: makeOff
#[allow(unused_variables, non_snake_case)]
pub fn mjr_resize_offscreen(offwidth: i32, offheight: i32, con: *mut mjrContext) {
    todo!() // mjr_resizeOffscreen
}

/// C: mjr_uploadTexture (render/classic/render_context.h:60)
/// Calls: mjr_setf4, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_texture(m: *const mjModel, con: *const mjrContext, texid: i32) {
    todo!() // mjr_uploadTexture
}

/// C: mjr_uploadMesh (render/classic/render_context.h:63)
/// Calls: mjr_makeNormal, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_mesh(m: *const mjModel, con: *const mjrContext, meshid: i32) {
    todo!() // mjr_uploadMesh
}

/// C: mjr_uploadHField (render/classic/render_context.h:66)
/// Calls: addVert, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_h_field(m: *const mjModel, con: *const mjrContext, hfieldid: i32) {
    todo!() // mjr_uploadHField
}

