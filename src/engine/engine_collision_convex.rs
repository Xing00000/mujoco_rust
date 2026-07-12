//! Port of: engine/engine_collision_convex.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: prism_firstdir (engine/engine_collision_convex.c:47)
#[allow(unused_variables, non_snake_case)]
pub fn prism_firstdir(o1: *const (), o2: *const (), vec: *mut ccd_vec3_t) {
    // WARNING: signature changed — verify body
    // Previous params: (o1 : * const (), o2 : * const (), vec : * mut ccd_vec3_t)
    // Previous return: ()
    todo ! ()
}

/// C: _libccd_wrapper (engine/engine_collision_convex.c:52)
/// Calls: mji_copy3, mji_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn libccd_wrapper(m: *const mjModel, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj, con : * mut mjPreContact, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_penetration (engine/engine_collision_convex.c:87)
/// Calls: _libccd_wrapper, mj_freeStack, mj_markStack, mj_stackAllocByte, mjc_ccd, mjc_ccdSize, mji_sub3, mji_zero3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_penetration(m: *const mjModel, d: *mut mjData, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, ncon: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj, con : * mut mjPreContact, ncon : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mulMatTVec3 (engine/engine_collision_convex.c:174)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_mat_t_vec3(res: *mut f64, mat: *const f64, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: localToGlobal (engine/engine_collision_convex.c:183)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn local_to_global(res: *mut f64, mat: *const f64, dir: *const f64, pos: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, dir : * const f64, pos : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_sphereSupport (engine/engine_collision_convex.c:202)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_capsuleSupport (engine/engine_collision_convex.c:231)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_ellipsoidSupport (engine/engine_collision_convex.c:256)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_cylinderSupport (engine/engine_collision_convex.c:293)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_cylinder_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_boxSupport (engine/engine_collision_convex.c:317)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_box_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: dot3f (engine/engine_collision_convex.c:343)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3f(a: *const f64, b: *const f32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (a : * const f64, b : * const f32)
    // Previous return: f64
    todo ! ()
}

/// C: mjc_meshSupport (engine/engine_collision_convex.c:349)
/// Calls: dot3f, localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_mesh_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_hillclimbSupport (engine/engine_collision_convex.c:391)
/// Calls: dot3f, localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_hillclimb_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_prism_support (engine/engine_collision_convex.c:436)
/// Calls: mji_copy3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_prism_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_flexSupport (engine/engine_collision_convex.c:458)
/// Calls: mji_addScl3, mji_addToScl3, mji_copy3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_flex_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_setCCDObjFlex (engine/engine_collision_convex.c:790)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_obj_flex(obj: *mut mjCCDObj, flex: i32, elem: i32, vert: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (obj : * mut mjCCDObj, flex : i32, elem : i32, vert : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_isDistinctContact (engine/engine_collision_convex.c:798)
/// Calls: mju_dist3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_is_distinct_contact(con: *const mjPreContact, ncon: i32, tolerance: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * const mjPreContact, ncon : i32, tolerance : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mju_rotateFrame (engine/engine_collision_convex.c:810)
/// Calls: mji_sub3, mji_subFrom3, mju_copy, mju_mulMatMat3, mju_mulMatVec3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rotate_frame(origin: *const f64, rot: *const f64, xmat: *mut f64, xpos: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (origin : * const f64, rot : * const f64, xmat : * mut f64, xpos : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: maxContacts (engine/engine_collision_convex.c:831)
#[allow(unused_variables, non_snake_case)]
pub fn max_contacts(m: *const mjModel, obj1: *const mjCCDObj, obj2: *const mjCCDObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, obj1 : * const mjCCDObj, obj2 : * const mjCCDObj)
    // Previous return: i32
    todo ! ()
}

/// C: addplanemesh (engine/engine_collision_convex.c:946)
/// Calls: mji_addToScl3, mji_copy3, mji_sub3, mji_zero3, mju_addTo3, mju_dist3, mju_dot3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn addplanemesh(con: *mut mjPreContact, vertex: *const f32, pos1: *const f64, normal1: *const f64, pos2: *const f64, mat2: *const f64, first: *const f64, rbound: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * mut mjPreContact, vertex : * const f32, pos1 : * const f64, normal1 : * const f64, pos2 : * const f64, mat2 : * const f64, first : * const f64, rbound : f64)
    // Previous return: i32
    todo ! ()
}

/// C: addVert (engine/engine_collision_convex.c:1085)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_vert(obj: *mut mjCCDObj, x: f64, y: f64, z: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (obj : * mut mjCCDObj, x : f64, y : f64, z : f64)
    // Previous return: ()
    todo ! ()
}

/// C: addPrismVert (engine/engine_collision_convex.c:1100)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_prism_vert(obj: *mut mjCCDObj, r: i32, c: i32, i: i32, dx: f64, dy: f64, margin: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (obj : * mut mjCCDObj, r : i32, c : i32, i : i32, dx : f64, dy : f64, margin : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_ellipsoidInside (engine/engine_collision_convex.c:1282)
/// Calls: mji_addScl3, mji_copy3, mju_dist3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_inside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (nrm : * mut f64, pos : * const f64, size : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_ellipsoidOutside (engine/engine_collision_convex.c:1337)
/// Calls: mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_outside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (nrm : * mut f64, pos : * const f64, size : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_initCCDObj (engine/engine_collision_convex.h:94)
/// Calls: mju_copy, mju_zero4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_init_ccd_obj(obj: *mut mjCCDObj, m: *const mjModel, d: *const mjData, g: i32, margin: f64) {
    use crate::engine::engine_util_blas::{mju_zero4, mju_copy};

    // Union data field offsets (from C struct layout, all relative to &obj.data):
    // mesh variant:
    //   nvert: i32          @ offset 0
    //   mesh_polynum: i32   @ offset 4
    //   vert: *const f32    @ offset 8
    //   mpolymapadr: *const i32 @ offset 16
    //   mpolymapnum: *const i32 @ offset 24
    //   polymap: *const i32 @ offset 32
    //   polyvertadr: *const i32 @ offset 40
    //   polyvertnum: *const i32 @ offset 48
    //   polyvert: *const i32 @ offset 56
    //   polynormal: *const f64 @ offset 64
    //   graph: *const i32   @ offset 72
    // hfield variant:
    //   prism: [f64;18]     @ offset 0  (144 bytes)
    //   hfield_data: *const f32 @ offset 144
    //   hfield_nrow: i32    @ offset 152
    //   hfield_ncol: i32    @ offset 156
    // flex variant:
    //   elem: *const i32    @ offset 0
    //   dim: *const i32     @ offset 8
    //   aabb: *const f64    @ offset 16
    //   elemadr: *const i32 @ offset 24
    //   elemdataadr: *const i32 @ offset 32
    //   vert_xpos: *const f64 @ offset 40
    //   vertadr: *const i32 @ offset 48
    //   xradius: *const f64 @ offset 56

    // mjtGeom enum values
    const MJGEOM_PLANE: i32 = 0;
    const MJGEOM_HFIELD: i32 = 1;
    const MJGEOM_SPHERE: i32 = 2;
    const MJGEOM_CAPSULE: i32 = 3;
    const MJGEOM_ELLIPSOID: i32 = 4;
    const MJGEOM_CYLINDER: i32 = 5;
    const MJGEOM_BOX: i32 = 6;
    const MJGEOM_MESH: i32 = 7;
    const MJGEOM_SDF: i32 = 8;
    const MJGEOM_FLEX: i32 = 105;

    const MJ_MESH_HILLCLIMB_MIN: i32 = 10;

    // SAFETY: all pointer dereferences follow C semantics exactly.
    // obj, m, d are valid structs from MuJoCo runtime.
    // Union fields accessed via raw byte offsets matching C ABI layout.
    unsafe {
        let data_ptr = &mut (*obj).data as *mut _ as *mut u8;

        (*obj).geom = g;
        (*obj).margin = margin;
        (*obj).center = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_center as *const ()));
        (*obj).vertindex = -1;
        (*obj).meshindex = -1;
        (*obj).flex = -1;
        (*obj).elem = -1;
        (*obj).vert = -1;
        mju_zero4((*obj).rotate.as_mut_ptr());
        (*obj).rotate[0] = 1.0;

        if g >= 0 {
            let g_usize = g as usize;
            mju_copy((*obj).size.as_mut_ptr(), (*m).geom_size.add(3 * g_usize), 3);
            mju_copy((*obj).pos.as_mut_ptr(), (*d).geom_xpos.add(3 * g_usize), 3);
            mju_copy((*obj).mat.as_mut_ptr(), (*d).geom_xmat.add(9 * g_usize), 9);
            (*obj).geom_type = *(*m).geom_type.add(g_usize);

            match (*obj).geom_type {
                MJGEOM_ELLIPSOID => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_ellipsoid_support as *const ()));
                }
                MJGEOM_MESH | MJGEOM_SDF => {
                    let dataid = *(*m).geom_dataid.add(g_usize) as usize;
                    let graphadr = *(*m).mesh_graphadr.add(dataid);
                    let vertadr = *(*m).mesh_vertadr.add(dataid) as usize;
                    let polyadr = *(*m).mesh_polyadr.add(dataid) as usize;

                    if graphadr < 0 || *(*m).mesh_vertnum.add(dataid) < MJ_MESH_HILLCLIMB_MIN {
                        // graph = NULL
                        *(data_ptr.add(72) as *mut *const i32) = std::ptr::null();
                        (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_mesh_support as *const ()));
                    } else {
                        // graph = m->mesh_graph + graphadr
                        *(data_ptr.add(72) as *mut *const i32) =
                            (*m).mesh_graph.add(graphadr as usize) as *const i32;
                        (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_hillclimb_support as *const ()));
                    }
                    // vert = m->mesh_vert + 3*vertadr
                    *(data_ptr.add(8) as *mut *const f32) =
                        (*m).mesh_vert.add(3 * vertadr) as *const f32;
                    // nvert
                    *(data_ptr.add(0) as *mut i32) = *(*m).mesh_vertnum.add(dataid);
                    // mpolymapadr
                    *(data_ptr.add(16) as *mut *const i32) =
                        (*m).mesh_polymapadr.add(vertadr) as *const i32;
                    // mpolymapnum
                    *(data_ptr.add(24) as *mut *const i32) =
                        (*m).mesh_polymapnum.add(vertadr) as *const i32;
                    // polymap
                    *(data_ptr.add(32) as *mut *const i32) =
                        (*m).mesh_polymap as *const i32;
                    // polynormal
                    *(data_ptr.add(64) as *mut *const f64) =
                        (*m).mesh_polynormal.add(3 * polyadr) as *const f64;
                    // polyvertadr
                    *(data_ptr.add(40) as *mut *const i32) =
                        (*m).mesh_polyvertadr.add(polyadr) as *const i32;
                    // polyvertnum
                    *(data_ptr.add(48) as *mut *const i32) =
                        (*m).mesh_polyvertnum.add(polyadr) as *const i32;
                    // polyvert
                    *(data_ptr.add(56) as *mut *const i32) =
                        (*m).mesh_polyvert as *const i32;
                    // mesh_polynum
                    *(data_ptr.add(4) as *mut i32) = *(*m).mesh_polynum.add(dataid);
                }
                MJGEOM_SPHERE => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_sphere_support as *const ()));
                }
                MJGEOM_CAPSULE => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_capsule_support as *const ()));
                }
                MJGEOM_CYLINDER => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_cylinder_support as *const ()));
                }
                MJGEOM_BOX => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_box_support as *const ()));
                }
                MJGEOM_HFIELD => {
                    (*obj).center = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_center as *const ()));
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_prism_support as *const ()));

                    let hid = *(*m).geom_dataid.add(g_usize) as usize;
                    // hfield_nrow
                    *(data_ptr.add(152) as *mut i32) = *(*m).hfield_nrow.add(hid);
                    // hfield_ncol
                    *(data_ptr.add(156) as *mut i32) = *(*m).hfield_ncol.add(hid);
                    // size = m->hfield_size + 4*hid (copy 4 doubles)
                    mju_copy((*obj).size.as_mut_ptr(), (*m).hfield_size.add(4 * hid), 4);
                    // hfield_data
                    *(data_ptr.add(144) as *mut *const f32) =
                        (*m).hfield_data.add(*(*m).hfield_adr.add(hid) as usize) as *const f32;
                }
                _ => {
                    (*obj).support = None;
                }
            }
        } else {
            // g < 0: flex geom
            (*obj).geom_type = MJGEOM_FLEX;
            // flex.dim
            *(data_ptr.add(8) as *mut *const i32) = (*m).flex_dim as *const i32;
            (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_flex_support as *const ()));
            // flex.aabb
            *(data_ptr.add(16) as *mut *const f64) = (*d).flexelem_aabb as *const f64;
            // flex.elemadr
            *(data_ptr.add(24) as *mut *const i32) = (*m).flex_elemadr as *const i32;
            // flex.vert_xpos
            *(data_ptr.add(40) as *mut *const f64) = (*d).flexvert_xpos as *const f64;
            // flex.vertadr
            *(data_ptr.add(48) as *mut *const i32) = (*m).flex_vertadr as *const i32;
            // flex.xradius
            *(data_ptr.add(56) as *mut *const f64) = (*m).flex_radius as *const f64;
            // flex.elemdataadr
            *(data_ptr.add(32) as *mut *const i32) = (*m).flex_elemdataadr as *const i32;
            // flex.elem
            *(data_ptr.add(0) as *mut *const i32) = (*m).flex_elem as *const i32;
        }
    }
}

/// C: mjc_center (engine/engine_collision_convex.h:97)
/// Calls: mji_addTo3, mji_copy3, mju_scl3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_center(res: *mut f64, obj: *const mjCCDObj) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * const mjCCDObj)
    // Previous return: ()
    todo ! ()
}

/// C: mjccd_center (engine/engine_collision_convex.h:100)
/// Calls: mjc_center
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_center(obj: *const (), center: *mut ccd_vec3_t) {
    // WARNING: signature changed — verify body
    // Previous params: (obj : * const (), center : * mut ccd_vec3_t)
    // Previous return: ()
    todo ! ()
}

/// C: mjccd_support (engine/engine_collision_convex.h:103)
/// Calls: mjc_prism_support, mji_addScl3, mji_addTo3, mji_addToScl3, mji_copy3, mji_scl3, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_normalize3, mju_sign, mju_warning, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_support(obj: *const (), dir: *const ccd_vec3_t, vec: *mut ccd_vec3_t) {
    // WARNING: signature changed — verify body
    // Previous params: (obj : * const (), dir : * const ccd_vec3_t, vec : * mut ccd_vec3_t)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_pointSupport (engine/engine_collision_convex.h:106)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_point_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_lineSupport (engine/engine_collision_convex.h:109)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_line_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: caller guarantees res[3], obj, dir[3] are valid
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let length = (*obj).size[1];

        let dot = *mat.add(2) * *dir.add(0) + *mat.add(5) * *dir.add(1) + *mat.add(8) * *dir.add(2);
        let scl = if dot >= 0.0 { length } else { -length };

        // transform result to global frame
        *res.add(0) = *mat.add(2) * scl + *pos.add(0);
        *res.add(1) = *mat.add(5) * scl + *pos.add(1);
        *res.add(2) = *mat.add(8) * scl + *pos.add(2);
    }
}

/// C: mjc_PlaneConvex (engine/engine_collision_convex.h:112)
/// Calls: addplanemesh, mjc_initCCDObj, mjccd_support, mji_addToScl3, mji_copy3, mji_sub3, mji_zero3, mju_dot3, mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_ConvexHField (engine/engine_collision_convex.h:113)
/// Calls: addPrismVert, mjc_fixNormal, mjc_initCCDObj, mjc_penetration, mji_addTo3, mji_copy3, mji_copy9, mji_mulMatTMat3, mji_mulMatVec3, mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_h_field(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_Convex (engine/engine_collision_convex.h:114)
/// Calls: maxContacts, mjc_fixNormal, mjc_initCCDObj, mjc_isDistinctContact, mjc_penetration, mji_axisAngle2Quat, mji_copy3, mji_copy9, mju_makeFrame, mju_min, mju_quat2Mat, mju_rotateFrame, mju_transpose, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_ConvexElem (engine/engine_collision_convex.h:117)
/// Calls: mjc_fixNormal, mjc_initCCDObj, mjc_penetration, mjc_setCCDObjFlex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, f1: i32, e1: i32, v1: i32, f2: i32, e2: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g1 : i32, f1 : i32, e1 : i32, v1 : i32, f2 : i32, e2 : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_HFieldElem (engine/engine_collision_convex.h:121)
/// Calls: addVert, mjc_initCCDObj, mjc_penetration, mjc_setCCDObjFlex, mji_addTo3, mji_copy3, mji_mulMatTVec3, mji_sub3, mji_zero3, mju_max, mju_min, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_h_field_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g: i32, f: i32, e: i32, margin: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * mut mjPreContact, g : i32, f : i32, e : i32, margin : f64)
    // Previous return: i32
    todo ! ()
}

/// C: mjc_fixNormal (engine/engine_collision_convex.h:125)
/// Calls: mjc_ellipsoidInside, mjc_ellipsoidOutside, mji_copy3, mji_mulMatVec3, mji_scl3, mji_sub3, mju_mulMatTVec3, mju_norm, mju_normalize3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjc_fix_normal(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, g1: i32, g2: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, con : * mut mjPreContact, g1 : i32, g2 : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_setCCDBuffer (engine/engine_collision_convex.h:128)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_buffer(buffer: *mut ()) {
    use std::cell::Cell;
    thread_local! {
        pub static CCD_BUFFER: Cell<*mut ()> = Cell::new(std::ptr::null_mut());
    }
    CCD_BUFFER.with(|b| b.set(buffer));
}

