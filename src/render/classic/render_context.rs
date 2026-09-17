//! Port of: render/classic/render_context.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

use crate :: render :: classic :: render_util :: mjr_normalizeVec ;

/// C: mjr_defaultContext (render/classic/render_context.c:54)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_default_context(con: *mut mjrContext) {
    todo!() // cxx:_mjr_defaultContext
}

/// C: contextHasResources (render/classic/render_context.c:60)
#[allow(unused_variables, non_snake_case)]
pub fn context_has_resources(con: *const mjrContext) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_contextHasResources
}

/// C: mjr_defaultRendererInfo (render/classic/render_context.c:67)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_default_renderer_info(info: *mut mjrRendererInfo) {
    todo!() // cxx:_mjr_defaultRendererInfo
}

/// C: mjr_getRendererInfo (render/classic/render_context.c:74)
/// Calls: cxx:_mjr_defaultRendererInfo
#[allow(unused_variables, non_snake_case)]
pub fn mjr_get_renderer_info(info: *mut mjrRendererInfo) {
    todo!() // cxx:_mjr_getRendererInfo
}

/// C: listAllocate (render/classic/render_context.c:84)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn list_allocate(base: *mut u32, range: *mut i32, newrange: i32) {
    todo!() // cxx-internal:render_context.c.o:_listAllocate
}

/// C: makePlane (render/classic/render_context.c:98)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_listAllocate, cxx:_mju_max
#[allow(unused_variables, non_snake_case)]
pub fn make_plane(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makePlane
}

/// C: makeMesh (render/classic/render_context.c:246)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_listAllocate, cxx:_mjr_uploadMesh
#[allow(unused_variables, non_snake_case)]
pub fn make_mesh(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeMesh
}

/// C: mjr_uploadMesh (render/classic/render_context.c:259)
/// Calls: cxx:_mjr_makeNormal, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_mesh(m: *const mjModel, con: *const mjrContext, meshid: i32) {
    todo!() // cxx:_mjr_uploadMesh
}

/// C: addVert (render/classic/render_context.c:404)
/// Calls: cxx:_mjr_makeNormal
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_vert(x: f32, y: f32, z: f32, sclz: f32, buf: *mut struct_vertbuf) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_addVert
}

/// C: makeHField (render/classic/render_context.c:437)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_listAllocate, cxx:_mjr_uploadHField
#[allow(unused_variables, non_snake_case)]
pub fn make_h_field(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeHField
}

/// C: mjr_uploadHField (render/classic/render_context.c:450)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_h_field(m: *const mjModel, con: *const mjrContext, hfieldid: i32) {
    todo!() // cxx:_mjr_uploadHField
}

/// C: setVertexSphere (render/classic/render_context.c:548)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_sphere(v: *mut f32, n: *mut f32, az: f32, el: f32, sign: i32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexSphere
}

/// C: halfSphere (render/classic/render_context.c:560)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_setVertexSphere
#[allow(unused_variables, non_snake_case)]
pub fn half_sphere(sign: i32, nSlice: i32, nStack: i32) {
    todo!() // cxx-internal:render_context.c.o:_halfSphere
}

/// C: sphere (render/classic/render_context.c:651)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_setVertexSphere
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
                setVertexSphere(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, el1, 0);
                setVertexSphere(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, el1, 0);
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
                    setVertexSphere(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, el1, 0);
                    setVertexSphere(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, el1, 0);
                    setVertexSphere(v3.as_mut_ptr(), n3.as_mut_ptr(), az2, el2, 0);
                    setVertexSphere(v4.as_mut_ptr(), n4.as_mut_ptr(), az1, el2, 0);

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

/// C: setVertexDisk (render/classic/render_context.c:737)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_disk(v: *mut f32, az: f32, r: f32, sign: i32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexDisk
}

/// C: disk (render/classic/render_context.c:745)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_setVertexDisk
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
            setVertexDisk(v1.as_mut_ptr(), az1, r1, sign);
            setVertexDisk(v2.as_mut_ptr(), az2, r1, sign);
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
                setVertexDisk(v1.as_mut_ptr(), az1, r2, sign);
                setVertexDisk(v2.as_mut_ptr(), az2, r2, sign);
                setVertexDisk(v3.as_mut_ptr(), az2, r1, sign);
                setVertexDisk(v4.as_mut_ptr(), az1, r1, sign);

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

/// C: setVertexCone (render/classic/render_context.c:830)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cone(v: *mut f32, n: *mut f32, az: f32, r: f32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexCone
}

/// C: cone (render/classic/render_context.c:846)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_setVertexCone, cxx:_mjr_normalizeVec
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
            setVertexCone(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, r1);
            setVertexCone(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, r1);
            v3[0] = 0.0f32;
            v3[1] = 0.0f32;
            v3[2] = 1.0f32;
            n3[0] = n1[0] + n2[0];
            n3[1] = n1[1] + n2[1];
            n3[2] = n1[2] + n2[2];
            mjr_normalizeVec(n3.as_mut_ptr());

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
                setVertexCone(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, r2);
                setVertexCone(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, r2);
                setVertexCone(v3.as_mut_ptr(), n3.as_mut_ptr(), az2, r1);
                setVertexCone(v4.as_mut_ptr(), n4.as_mut_ptr(), az1, r1);

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

/// C: setVertexCylinder (render/classic/render_context.c:911)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cylinder(v: *mut f32, n: *mut f32, az: f32, h: f32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexCylinder
}

/// C: cylinder (render/classic/render_context.c:923)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_setVertexCylinder
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
                setVertexCylinder(v1.as_mut_ptr(), n1.as_mut_ptr(), az1, h1);
                setVertexCylinder(v2.as_mut_ptr(), n2.as_mut_ptr(), az2, h1);
                setVertexCylinder(v3.as_mut_ptr(), n3.as_mut_ptr(), az2, h2);
                setVertexCylinder(v4.as_mut_ptr(), n4.as_mut_ptr(), az1, h2);

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

/// C: setVertexHaze (render/classic/render_context.c:963)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_haze(v: *mut f32, az: f32, h: f32, r: f32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexHaze
}

/// C: haze (render/classic/render_context.c:971)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_setVertexHaze
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
                setVertexHaze(v1.as_mut_ptr(), az1, h1, r);
                setVertexHaze(v2.as_mut_ptr(), az2, h1, r);
                setVertexHaze(v3.as_mut_ptr(), az2, h2, r);
                setVertexHaze(v4.as_mut_ptr(), az1, h2, r);

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

/// C: makeBuiltin (render/classic/render_context.c:1018)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_cone, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_haze, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_listAllocate, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_sphere, cxx-internal:render_context.c.o:_cylinder, cxx-internal:render_context.c.o:_disk, cxx-internal:render_context.c.o:_halfSphere
#[allow(unused_variables, non_snake_case)]
pub fn make_builtin(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeBuiltin
}

/// C: makeShadow (render/classic/render_context.c:1125)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_shadow(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeShadow
}

/// C: makeOff (render/classic/render_context.c:1178)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_off(con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeOff
}

/// C: makeFont (render/classic/render_context.c:1279)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_font(con: *mut mjrContext, fontscale: i32) {
    todo!() // cxx-internal:render_context.c.o:_makeFont
}

/// C: makeMaterial (render/classic/render_context.c:1387)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_material(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeMaterial
}

/// C: makeTexture (render/classic/render_context.c:1425)
/// Calls: cxx:_mjr_uploadTexture, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_texture(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeTexture
}

/// C: mjr_uploadTexture (render/classic/render_context.c:1448)
/// Calls: cxx:_mjr_setf4, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_texture(m: *const mjModel, con: *const mjrContext, texid: i32) {
    todo!() // cxx:_mjr_uploadTexture
}

/// C: makeSkin (render/classic/render_context.c:1549)
/// Calls: cxx:_mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn make_skin(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeSkin
}

/// C: debugCallback (render/classic/render_context.c:1596)
#[allow(unused_variables, non_snake_case)]
pub fn debug_callback(source: u32, r#type: u32, id: u32, severity: u32, length: i32, message: *const i8, userParam: *const ()) {
    todo!() // cxx:_debugCallback
}

/// C: glDebugEnabled (render/classic/render_context.c:1610)
#[allow(unused_variables, non_snake_case)]
pub fn gl_debug_enabled() -> i32 {
    todo!() // cxx-internal:render_context.c.o:_glDebugEnabled
}

/// C: mjr_makeContext_offSize (render/classic/render_context.c:1617)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_glDebugEnabled, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_makeBuiltin, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_makeHField, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_makeMaterial, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_makeMesh, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_makePlane, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_makeShadow, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_makeSkin, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_makeTexture, cxx-internal:render_context.c.o:_makeFont, cxx-internal:render_context.c.o:_makeOff, cxx:_mjGladLoadGL, cxx:_mjr_freeContext, cxx:_mjr_setBuffer, cxx:_mju_error, cxx:_mju_round, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context_off_size(m: *const mjModel, con: *mut mjrContext, fontscale: i32, default_offwidth: i32, default_offheight: i32) {
    todo!() // cxx:_mjr_makeContext_offSize
}

/// C: mjr_makeContext (render/classic/render_context.c:1769)
/// Calls: cxx:_mjr_makeContext_offSize
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context(m: *const mjModel, con: *mut mjrContext, fontscale: i32) {
    todo!() // cxx:_mjr_makeContext
}

/// C: mjr_changeFont (render/classic/render_context.c:1776)
/// Calls: cxx-internal:render_context.c.o:_makeFont
#[allow(unused_variables, non_snake_case)]
pub fn mjr_change_font(fontscale: i32, con: *mut mjrContext) {
    todo!() // cxx:_mjr_changeFont
}

/// C: mjr_addAux (render/classic/render_context.c:1795)
/// Calls: cxx:_mjr_restoreBuffer, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_add_aux(index: i32, width: i32, height: i32, samples: i32, con: *mut mjrContext) {
    todo!() // cxx:_mjr_addAux
}

/// C: mjr_freeContext (render/classic/render_context.c:1907)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_contextHasResources, cxx:_mjr_defaultContext, cxx:_mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mjr_free_context(con: *mut mjrContext) {
    todo!() // cxx:_mjr_freeContext
}

/// C: mjr_resizeOffscreen (render/classic/render_context.c:1975)
/// Calls: cxx-internal:render_context.c.o:_makeOff
#[allow(unused_variables, non_snake_case)]
pub fn mjr_resize_offscreen(offwidth: i32, offheight: i32, con: *mut mjrContext) {
    todo!() // cxx:_mjr_resizeOffscreen
}

