//! C types used in function signatures.
//! CODEGEN LOCKED — DO NOT EDIT

#![allow(non_camel_case_types, non_snake_case)]


/// C struct: mjCModel (5936 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCModel {
    pub _pad_0: [u8; 16],
    pub prefix: std__string,
    pub suffix: std__string,
    pub compiled: bool,
    pub _pad_1: [u8; 7],
    pub nbody: i64,
    pub njnt: i64,
    pub ngeom: i64,
    pub nsite: i64,
    pub ncam: i64,
    pub nlight: i64,
    pub nflex: i64,
    pub nmesh: i64,
    pub nskin: i64,
    pub nhfield: i64,
    pub ntex: i64,
    pub nmat: i64,
    pub npair: i64,
    pub nexclude: i64,
    pub neq: i64,
    pub ntendon: i64,
    pub nJten: i64,
    pub nsensor: i64,
    pub nnumeric: i64,
    pub ntext: i64,
    pub ntuple: i64,
    pub nmocap: i64,
    pub nplugin: i64,
    pub nq: i64,
    pub nv: i64,
    pub nu: i64,
    pub nactuator: i64,
    pub nout: i64,
    pub na: i64,
    pub ntree: i64,
    pub nbvh: i64,
    pub nbvhstatic: i64,
    pub nbvhdynamic: i64,
    pub noct: i64,
    pub nflexnode: i64,
    pub nflexvert: i64,
    pub nflexedge: i64,
    pub nflexelem: i64,
    pub nflexelemdata: i64,
    pub nflexstiffness: i64,
    pub nflexbending: i64,
    pub nefm0dof: i64,
    pub nefm0L: i64,
    pub nflexelemedge: i64,
    pub nflexshelldata: i64,
    pub nflexevpair: i64,
    pub nflextexcoord: i64,
    pub nJfe: i64,
    pub nJfv: i64,
    pub nmeshvert: i64,
    pub nmeshnormal: i64,
    pub nmeshtexcoord: i64,
    pub nmeshface: i64,
    pub nmeshpoly: i64,
    pub nmeshgraph: i64,
    pub nmeshpolyvert: i64,
    pub nmeshpolymap: i64,
    pub nskinvert: i64,
    pub nskintexvert: i64,
    pub nskinface: i64,
    pub nskinbone: i64,
    pub nskinbonevert: i64,
    pub nhfielddata: i64,
    pub ntexdata: i64,
    pub nwrap: i64,
    pub nsensordata: i64,
    pub nhistory: i64,
    pub nnumericdata: i64,
    pub ntextdata: i64,
    pub ntupledata: i64,
    pub npluginattr: i64,
    pub nnames: i64,
    pub npaths: i64,
    pub nM: i64,
    pub nB: i64,
    pub nC: i64,
    pub nD: i64,
    pub nJmom: i64,
    pub meaninertia_auto: f64,
    pub meanmass_auto: f64,
    pub meansize_auto: f64,
    pub extent_auto: f64,
    pub center_auto: [f64; 3],
    pub qpos0: std__vector_mjtNum,
    pub body_pos0: std__vector_mjtNum,
    pub body_quat0: std__vector_mjtNum,
    pub comment_: std__string,
    pub modelfiledir_: std__string,
    pub modelname_: std__string,
    pub meshdir_: std__string,
    pub texturedir_: std__string,
    pub spec_comment_: std__string,
    pub spec_modelfiledir_: std__string,
    pub spec_modelname_: std__string,
    pub element: *mut mjsElement,
    pub modelname: *mut mjString,
    pub compiler: mjsCompiler,
    pub strippath: bool,
    pub _pad_2: [u8; 7],
    pub option: mjOption,
    pub visual: mjVisual,
    pub stat: mjStatistic,
    pub memory: i64,
    pub nemax: i32,
    pub nuserdata: i32,
    pub nuser_body: i32,
    pub nuser_jnt: i32,
    pub nuser_geom: i32,
    pub nuser_site: i32,
    pub nuser_cam: i32,
    pub nuser_tendon: i32,
    pub nuser_actuator: i32,
    pub nuser_sensor: i32,
    pub nkey: i32,
    pub njmax: i32,
    pub nconmax: i32,
    pub _pad_3: [u8; 4],
    pub nstack: i64,
    pub comment: *mut mjString,
    pub modelfiledir: *mut mjString,
    pub hasImplicitPluginElem: bool,
    pub _pad_4: [u8; 7],
    pub authored: mjsAuthored,
    pub spec: mjSpec,
    pub timer: [f64; 9],
    pub def_map: std__unordered_map_std__string__mjCDef,
    pub refcount: i32,
    pub _pad_5: [u8; 4],
    pub defaults_: std__vector_mjCDef,
    pub active_plugins_: std__vector_std__pair_const_mjpPlugin____int,
    pub flexes_: std__vector_mjCFlex,
    pub meshes_: std__vector_mjCMesh,
    pub skins_: std__vector_mjCSkin,
    pub hfields_: std__vector_mjCHField,
    pub textures_: std__vector_mjCTexture,
    pub materials_: std__vector_mjCMaterial,
    pub pairs_: std__vector_mjCPair,
    pub excludes_: std__vector_mjCBodyPair,
    pub equalities_: std__vector_mjCEquality,
    pub tendons_: std__vector_mjCTendon,
    pub actuators_: std__vector_mjCActuator,
    pub sensors_: std__vector_mjCSensor,
    pub numerics_: std__vector_mjCNumeric,
    pub texts_: std__vector_mjCText,
    pub tuples_: std__vector_mjCTuple,
    pub keys_: std__vector_mjCKey,
    pub plugins_: std__vector_mjCPlugin,
    pub specs_: std__vector_mjSpec,
    pub bodies_: std__vector_mjCBody,
    pub joints_: std__vector_mjCJoint,
    pub geoms_: std__vector_mjCGeom,
    pub sites_: std__vector_mjCSite,
    pub cameras_: std__vector_mjCCamera,
    pub lights_: std__vector_mjCLight,
    pub frames_: std__vector_mjCFrame,
    pub object_lists_: std__array_std__vector_mjCBase_______mjNOBJECT,
    pub ids: mjListKeyMap,
    pub errInfo: [u8; 504],
    pub warnings_: std__vector_std__string,
    pub num_attach_warnings_: i32,
    pub compiling_: bool,
    pub _pad_6: [u8; 3],
    pub key_pending_: std__vector_mjKeyInfo,
    pub deepcopy_: bool,
    pub attached_: bool,
    pub _pad_7: [u8; 6],
    pub compiler2spec_: std__unordered_map_const_mjsCompiler____mjSpec,
    pub detached_: std__vector_mjCBase,
}

/// Sized opaque C struct: mjListKeyMap (624 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjListKeyMap { pub _data: [u8; 624] }

/// C struct: mjOption (304 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjOption {
    pub timestep: f64,
    pub impratio: f64,
    pub tolerance: f64,
    pub ls_tolerance: f64,
    pub noslip_tolerance: f64,
    pub ccd_tolerance: f64,
    pub sleep_tolerance: f64,
    pub gravity: [f64; 3],
    pub wind: [f64; 3],
    pub magnetic: [f64; 3],
    pub density: f64,
    pub viscosity: f64,
    pub o_margin: f64,
    pub o_solref: [f64; 2],
    pub o_solimp: [f64; 5],
    pub o_friction: [f64; 5],
    pub integrator: i32,
    pub cone: i32,
    pub jacobian: i32,
    pub solver: i32,
    pub iterations: i32,
    pub ls_iterations: i32,
    pub noslip_iterations: i32,
    pub ccd_iterations: i32,
    pub disableflags: i32,
    pub enableflags: i32,
    pub disableactuator: i32,
    pub sdf_initpoints: i32,
    pub sdf_iterations: i32,
    pub _pad_0: [u8; 4],
}

/// C struct: mjSpec (1344 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjSpec {
    pub element: *mut mjsElement,
    pub modelname: *mut mjString,
    pub compiler: mjsCompiler,
    pub strippath: bool,
    pub _pad_0: [u8; 7],
    pub option: mjOption,
    pub visual: mjVisual,
    pub stat: mjStatistic,
    pub memory: i64,
    pub nemax: i32,
    pub nuserdata: i32,
    pub nuser_body: i32,
    pub nuser_jnt: i32,
    pub nuser_geom: i32,
    pub nuser_site: i32,
    pub nuser_cam: i32,
    pub nuser_tendon: i32,
    pub nuser_actuator: i32,
    pub nuser_sensor: i32,
    pub nkey: i32,
    pub njmax: i32,
    pub nconmax: i32,
    pub _pad_1: [u8; 4],
    pub nstack: i64,
    pub comment: *mut mjString,
    pub modelfiledir: *mut mjString,
    pub hasImplicitPluginElem: bool,
    pub _pad_2: [u8; 7],
    pub authored: mjsAuthored,
}

/// C struct: mjStatistic (56 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjStatistic {
    pub meaninertia: f64,
    pub meanmass: f64,
    pub meansize: f64,
    pub extent: f64,
    pub center: [f64; 3],
}

/// Sized opaque C struct: mjString (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjString { pub _data: [u8; 24] }

/// C struct: mjVisual (632 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjVisual {
    pub global: [u8; 52],
    pub quality: [u8; 20],
    pub headlight: [u8; 40],
    pub map: [u8; 52],
    pub scale: [u8; 68],
    pub rgba: [u8; 400],
}

/// C struct: mjsAuthored (72 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsAuthored {
    pub option: u64,
    pub disableflags: i32,
    pub enableflags: i32,
    pub disableactuator: i32,
    pub _pad_0: [u8; 4],
    pub visual_global: u64,
    pub visual_quality: u64,
    pub visual_headlight: u64,
    pub visual_map: u64,
    pub visual_scale: u64,
    pub visual_rgba: u64,
}

/// C struct: mjsCompiler (160 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsCompiler {
    pub autolimits: bool,
    pub _pad_0: [u8; 7],
    pub boundmass: f64,
    pub boundinertia: f64,
    pub settotalmass: f64,
    pub balanceinertia: bool,
    pub fitaabb: bool,
    pub degree: bool,
    pub eulerseq: [i8; 3],
    pub discardvisual: bool,
    pub usethread: bool,
    pub fusestatic: bool,
    pub _pad_1: [u8; 3],
    pub inertiafromgeom: u32,
    pub inertiagrouprange: [i32; 2],
    pub saveinertial: bool,
    pub alignfree: bool,
    pub _pad_2: [u8; 2],
    pub conflict: u32,
    pub LRopt: mjLROpt,
    pub meshdir: *mut mjString,
    pub texturedir: *mut mjString,
    pub authored: u64,
}

/// C struct: mjsElement (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsElement {
    pub elemtype: u32,
    pub _pad_0: [u8; 4],
    pub signature: u64,
}

/// Sized opaque C struct: std__array_std__vector_mjCBase_______mjNOBJECT (208 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__array_std__vector_mjCBase_______mjNOBJECT { pub _data: [u8; 208] }

/// Sized opaque C struct: std__string (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__string { pub _data: [u8; 24] }

/// Sized opaque C struct: std__unordered_map_const_mjsCompiler____mjSpec (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__unordered_map_const_mjsCompiler____mjSpec { pub _data: [u8; 40] }

/// Sized opaque C struct: std__unordered_map_std__string__mjCDef (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__unordered_map_std__string__mjCDef { pub _data: [u8; 40] }

/// Sized opaque C struct: std__vector_mjCActuator (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCActuator { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCBase (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCBase { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCBody (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCBody { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCBodyPair (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCBodyPair { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCCamera (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCCamera { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCDef (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCDef { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCEquality (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCEquality { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCFlex (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCFlex { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCFrame (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCFrame { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCGeom (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCGeom { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCHField (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCHField { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCJoint (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCJoint { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCKey (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCKey { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCLight (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCLight { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCMaterial (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCMaterial { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCMesh (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCMesh { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCNumeric (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCNumeric { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCPair (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCPair { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCPlugin (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCPlugin { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCSensor (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCSensor { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCSite (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCSite { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCSkin (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCSkin { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCTendon (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCTendon { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCText (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCText { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCTexture (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCTexture { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCTuple (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCTuple { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjKeyInfo (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjKeyInfo { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjSpec (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjSpec { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjtNum (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjtNum { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_std__pair_const_mjpPlugin____int (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_std__pair_const_mjpPlugin____int { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_std__string (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_std__string { pub _data: [u8; 24] }

/// C enum: mjtAlignFree
pub type mjtAlignFree = u32;

pub const mjtAlignFree_mjALIGNFREE_FALSE: mjtAlignFree = 0;

pub const mjtAlignFree_mjALIGNFREE_TRUE: mjtAlignFree = 1;

pub const mjtAlignFree_mjALIGNFREE_AUTO: mjtAlignFree = 2;

/// C enum: mjtBias
pub type mjtBias = u32;

pub const mjtBias_mjBIAS_NONE: mjtBias = 0;

pub const mjtBias_mjBIAS_AFFINE: mjtBias = 1;

pub const mjtBias_mjBIAS_MUSCLE: mjtBias = 2;

pub const mjtBias_mjBIAS_DCMOTOR: mjtBias = 3;

pub const mjtBias_mjBIAS_SO3: mjtBias = 4;

pub const mjtBias_mjBIAS_USER: mjtBias = 5;

/// C enum: mjtCamLight
pub type mjtCamLight = u32;

pub const mjtCamLight_mjCAMLIGHT_FIXED: mjtCamLight = 0;

pub const mjtCamLight_mjCAMLIGHT_TRACK: mjtCamLight = 1;

pub const mjtCamLight_mjCAMLIGHT_TRACKCOM: mjtCamLight = 2;

pub const mjtCamLight_mjCAMLIGHT_TARGETBODY: mjtCamLight = 3;

pub const mjtCamLight_mjCAMLIGHT_TARGETBODYCOM: mjtCamLight = 4;

/// C enum: mjtConflict
pub type mjtConflict = u32;

pub const mjtConflict_mjCONFLICT_WARNING: mjtConflict = 0;

pub const mjtConflict_mjCONFLICT_MERGE: mjtConflict = 1;

pub const mjtConflict_mjCONFLICT_ERROR: mjtConflict = 2;

/// C enum: mjtDyn
pub type mjtDyn = u32;

pub const mjtDyn_mjDYN_NONE: mjtDyn = 0;

pub const mjtDyn_mjDYN_INTEGRATOR: mjtDyn = 1;

pub const mjtDyn_mjDYN_FILTER: mjtDyn = 2;

pub const mjtDyn_mjDYN_FILTEREXACT: mjtDyn = 3;

pub const mjtDyn_mjDYN_MUSCLE: mjtDyn = 4;

pub const mjtDyn_mjDYN_DCMOTOR: mjtDyn = 5;

pub const mjtDyn_mjDYN_PID: mjtDyn = 6;

pub const mjtDyn_mjDYN_USER: mjtDyn = 7;

/// C enum: mjtEq
pub type mjtEq = u32;

pub const mjtEq_mjEQ_CONNECT: mjtEq = 0;

pub const mjtEq_mjEQ_WELD: mjtEq = 1;

pub const mjtEq_mjEQ_JOINT: mjtEq = 2;

pub const mjtEq_mjEQ_TENDON: mjtEq = 3;

pub const mjtEq_mjEQ_FLEX: mjtEq = 4;

pub const mjtEq_mjEQ_FLEXVERT: mjtEq = 5;

pub const mjtEq_mjEQ_FLEXSTRAIN: mjtEq = 6;

pub const mjtEq_mjEQ_DISTANCE: mjtEq = 7;

/// C enum: mjtFlexSelf
pub type mjtFlexSelf = u32;

pub const mjtFlexSelf_mjFLEXSELF_NONE: mjtFlexSelf = 0;

pub const mjtFlexSelf_mjFLEXSELF_NARROW: mjtFlexSelf = 1;

pub const mjtFlexSelf_mjFLEXSELF_BVH: mjtFlexSelf = 2;

pub const mjtFlexSelf_mjFLEXSELF_SAP: mjtFlexSelf = 3;

pub const mjtFlexSelf_mjFLEXSELF_AUTO: mjtFlexSelf = 4;

/// C enum: mjtGain
pub type mjtGain = u32;

pub const mjtGain_mjGAIN_FIXED: mjtGain = 0;

pub const mjtGain_mjGAIN_AFFINE: mjtGain = 1;

pub const mjtGain_mjGAIN_MUSCLE: mjtGain = 2;

pub const mjtGain_mjGAIN_DCMOTOR: mjtGain = 3;

pub const mjtGain_mjGAIN_SO3: mjtGain = 4;

pub const mjtGain_mjGAIN_PID: mjtGain = 5;

pub const mjtGain_mjGAIN_USER: mjtGain = 6;

/// C enum: mjtGeom
pub type mjtGeom = u32;

pub const mjtGeom_mjGEOM_PLANE: mjtGeom = 0;

pub const mjtGeom_mjGEOM_HFIELD: mjtGeom = 1;

pub const mjtGeom_mjGEOM_SPHERE: mjtGeom = 2;

pub const mjtGeom_mjGEOM_CAPSULE: mjtGeom = 3;

pub const mjtGeom_mjGEOM_ELLIPSOID: mjtGeom = 4;

pub const mjtGeom_mjGEOM_CYLINDER: mjtGeom = 5;

pub const mjtGeom_mjGEOM_BOX: mjtGeom = 6;

pub const mjtGeom_mjGEOM_MESH: mjtGeom = 7;

pub const mjtGeom_mjGEOM_SDF: mjtGeom = 8;

pub const mjtGeom_mjNGEOMTYPES: mjtGeom = 9;

pub const mjtGeom_mjGEOM_ARROW: mjtGeom = 100;

pub const mjtGeom_mjGEOM_ARROW1: mjtGeom = 101;

pub const mjtGeom_mjGEOM_ARROW2: mjtGeom = 102;

pub const mjtGeom_mjGEOM_LINE: mjtGeom = 103;

pub const mjtGeom_mjGEOM_LINEBOX: mjtGeom = 104;

pub const mjtGeom_mjGEOM_FLEX: mjtGeom = 105;

pub const mjtGeom_mjGEOM_SKIN: mjtGeom = 106;

pub const mjtGeom_mjGEOM_LABEL: mjtGeom = 107;

pub const mjtGeom_mjGEOM_TRIANGLE: mjtGeom = 108;

pub const mjtGeom_mjGEOM_NONE: mjtGeom = 1001;

/// C enum: mjtGeomInertia
pub type mjtGeomInertia = u32;

pub const mjtGeomInertia_mjINERTIA_VOLUME: mjtGeomInertia = 0;

pub const mjtGeomInertia_mjINERTIA_SHELL: mjtGeomInertia = 1;

/// C enum: mjtInertiaFromGeom
pub type mjtInertiaFromGeom = u32;

pub const mjtInertiaFromGeom_mjINERTIAFROMGEOM_FALSE: mjtInertiaFromGeom = 0;

pub const mjtInertiaFromGeom_mjINERTIAFROMGEOM_TRUE: mjtInertiaFromGeom = 1;

pub const mjtInertiaFromGeom_mjINERTIAFROMGEOM_AUTO: mjtInertiaFromGeom = 2;

/// C enum: mjtJoint
pub type mjtJoint = u32;

pub const mjtJoint_mjJNT_FREE: mjtJoint = 0;

pub const mjtJoint_mjJNT_BALL: mjtJoint = 1;

pub const mjtJoint_mjJNT_SLIDE: mjtJoint = 2;

pub const mjtJoint_mjJNT_HINGE: mjtJoint = 3;

/// C enum: mjtLightType
pub type mjtLightType = u32;

pub const mjtLightType_mjLIGHT_SPOT: mjtLightType = 0;

pub const mjtLightType_mjLIGHT_DIRECTIONAL: mjtLightType = 1;

pub const mjtLightType_mjLIGHT_POINT: mjtLightType = 2;

pub const mjtLightType_mjLIGHT_IMAGE: mjtLightType = 3;

/// C enum: mjtLimited
pub type mjtLimited = u32;

pub const mjtLimited_mjLIMITED_FALSE: mjtLimited = 0;

pub const mjtLimited_mjLIMITED_TRUE: mjtLimited = 1;

pub const mjtLimited_mjLIMITED_AUTO: mjtLimited = 2;

/// C enum: mjtMeshInertia
pub type mjtMeshInertia = u32;

pub const mjtMeshInertia_mjMESH_INERTIA_CONVEX: mjtMeshInertia = 0;

pub const mjtMeshInertia_mjMESH_INERTIA_EXACT: mjtMeshInertia = 1;

pub const mjtMeshInertia_mjMESH_INERTIA_LEGACY: mjtMeshInertia = 2;

pub const mjtMeshInertia_mjMESH_INERTIA_SHELL: mjtMeshInertia = 3;

/// C enum: mjtObj
pub type mjtObj = u32;

pub const mjtObj_mjOBJ_UNKNOWN: mjtObj = 0;

pub const mjtObj_mjOBJ_BODY: mjtObj = 1;

pub const mjtObj_mjOBJ_XBODY: mjtObj = 2;

pub const mjtObj_mjOBJ_JOINT: mjtObj = 3;

pub const mjtObj_mjOBJ_DOF: mjtObj = 4;

pub const mjtObj_mjOBJ_GEOM: mjtObj = 5;

pub const mjtObj_mjOBJ_SITE: mjtObj = 6;

pub const mjtObj_mjOBJ_CAMERA: mjtObj = 7;

pub const mjtObj_mjOBJ_LIGHT: mjtObj = 8;

pub const mjtObj_mjOBJ_FLEX: mjtObj = 9;

pub const mjtObj_mjOBJ_MESH: mjtObj = 10;

pub const mjtObj_mjOBJ_SKIN: mjtObj = 11;

pub const mjtObj_mjOBJ_HFIELD: mjtObj = 12;

pub const mjtObj_mjOBJ_TEXTURE: mjtObj = 13;

pub const mjtObj_mjOBJ_MATERIAL: mjtObj = 14;

pub const mjtObj_mjOBJ_PAIR: mjtObj = 15;

pub const mjtObj_mjOBJ_EXCLUDE: mjtObj = 16;

pub const mjtObj_mjOBJ_EQUALITY: mjtObj = 17;

pub const mjtObj_mjOBJ_TENDON: mjtObj = 18;

pub const mjtObj_mjOBJ_ACTUATOR: mjtObj = 19;

pub const mjtObj_mjOBJ_SENSOR: mjtObj = 20;

pub const mjtObj_mjOBJ_NUMERIC: mjtObj = 21;

pub const mjtObj_mjOBJ_TEXT: mjtObj = 22;

pub const mjtObj_mjOBJ_TUPLE: mjtObj = 23;

pub const mjtObj_mjOBJ_KEY: mjtObj = 24;

pub const mjtObj_mjOBJ_PLUGIN: mjtObj = 25;

pub const mjtObj_mjNOBJECT: mjtObj = 26;

pub const mjtObj_mjOBJ_FRAME: mjtObj = 100;

pub const mjtObj_mjOBJ_DEFAULT: mjtObj = 101;

pub const mjtObj_mjOBJ_MODEL: mjtObj = 102;

/// C enum: mjtOrientation
pub type mjtOrientation = u32;

pub const mjtOrientation_mjORIENTATION_QUAT: mjtOrientation = 0;

pub const mjtOrientation_mjORIENTATION_AXISANGLE: mjtOrientation = 1;

pub const mjtOrientation_mjORIENTATION_XYAXES: mjtOrientation = 2;

pub const mjtOrientation_mjORIENTATION_ZAXIS: mjtOrientation = 3;

pub const mjtOrientation_mjORIENTATION_EULER: mjtOrientation = 4;

/// C enum: mjtProjection
pub type mjtProjection = u32;

pub const mjtProjection_mjPROJ_PERSPECTIVE: mjtProjection = 0;

pub const mjtProjection_mjPROJ_ORTHOGRAPHIC: mjtProjection = 1;

/// C enum: mjtSleepPolicy
pub type mjtSleepPolicy = u32;

pub const mjtSleepPolicy_mjSLEEP_AUTO: mjtSleepPolicy = 0;

pub const mjtSleepPolicy_mjSLEEP_AUTO_NEVER: mjtSleepPolicy = 1;

pub const mjtSleepPolicy_mjSLEEP_AUTO_ALLOWED: mjtSleepPolicy = 2;

pub const mjtSleepPolicy_mjSLEEP_NEVER: mjtSleepPolicy = 3;

pub const mjtSleepPolicy_mjSLEEP_ALLOWED: mjtSleepPolicy = 4;

pub const mjtSleepPolicy_mjSLEEP_INIT: mjtSleepPolicy = 5;

/// C enum: mjtTrn
pub type mjtTrn = u32;

pub const mjtTrn_mjTRN_JOINT: mjtTrn = 0;

pub const mjtTrn_mjTRN_JOINTINPARENT: mjtTrn = 1;

pub const mjtTrn_mjTRN_SLIDERCRANK: mjtTrn = 2;

pub const mjtTrn_mjTRN_TENDON: mjtTrn = 3;

pub const mjtTrn_mjTRN_SITE: mjtTrn = 4;

pub const mjtTrn_mjTRN_BODY: mjtTrn = 5;

pub const mjtTrn_mjTRN_SO3: mjtTrn = 6;

pub const mjtTrn_mjTRN_UNDEFINED: mjtTrn = 1000;

/// C struct: mjCError (500 bytes, align 1)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCError {
    pub message: [i8; 500],
}

const _: () = assert!(std::mem::size_of::<mjCError>() == 500);

const _: () = assert!(std::mem::size_of::<mjCModel>() == 5936);

/// Sized opaque C struct: mjDoubleVec (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjDoubleVec { pub _data: [u8; 24] }

/// Sized opaque C struct: mjFloatVec (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjFloatVec { pub _data: [u8; 24] }

/// Sized opaque C struct: mjIntVec (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjIntVec { pub _data: [u8; 24] }

/// C struct: mjResource (552 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjResource {
    pub name: *mut i8,
    pub data: *mut (),
    pub vfs: *mut mjVFS,
    pub timestamp: [i8; 512],
    pub provider: *const struct_mjpResourceProvider,
    pub args: *const i8,
}

/// C struct: mjsBody (568 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsBody {
    pub element: *mut mjsElement,
    pub childclass: *mut mjString,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub mass: f64,
    pub ipos: [f64; 3],
    pub iquat: [f64; 4],
    pub inertia: [f64; 3],
    pub ialt: mjsOrientation,
    pub fullinertia: [f64; 6],
    pub mocap: bool,
    pub _pad_0: [u8; 7],
    pub gravcomp: f64,
    pub sleep: u32,
    pub simple: u8,
    pub _pad_1: [u8; 3],
    pub userdata: *mut mjDoubleVec,
    pub explicitinertial: bool,
    pub _pad_2: [u8; 7],
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
}

/// C struct: mjsFrame (216 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsFrame {
    pub element: *mut mjsElement,
    pub childclass: *mut mjString,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub info: *mut mjString,
}

/// C struct: mjsMesh (232 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsMesh {
    pub element: *mut mjsElement,
    pub content_type: *mut mjString,
    pub file: *mut mjString,
    pub refpos: [f64; 3],
    pub refquat: [f64; 4],
    pub scale: [f64; 3],
    pub inertia: u32,
    pub smoothnormal: bool,
    pub needsdf: bool,
    pub _pad_0: [u8; 2],
    pub maxhullvert: i32,
    pub _pad_1: [u8; 4],
    pub uservert: *mut mjFloatVec,
    pub usernormal: *mut mjFloatVec,
    pub usertexcoord: *mut mjFloatVec,
    pub userface: *mut mjIntVec,
    pub userfacenormal: *mut mjIntVec,
    pub userfacetexcoord: *mut mjIntVec,
    pub plugin: mjsPlugin,
    pub material: *mut mjString,
    pub octree_maxdepth: i32,
    pub _pad_2: [u8; 4],
    pub info: *mut mjString,
}

/// C struct: mjsOrientation (136 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsOrientation {
    pub r#type: u32,
    pub _pad_0: [u8; 4],
    pub axisangle: [f64; 4],
    pub xyaxes: [f64; 6],
    pub zaxis: [f64; 3],
    pub euler: [f64; 3],
}

/// C struct: mjsPlugin (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsPlugin {
    pub element: *mut mjsElement,
    pub name: *mut mjString,
    pub plugin_name: *mut mjString,
    pub active: bool,
    pub _pad_0: [u8; 7],
    pub info: *mut mjString,
}

/// C struct: mjsSite (336 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsSite {
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub fromto: [f64; 6],
    pub size: [f64; 3],
    pub r#type: u32,
    pub _pad_0: [u8; 4],
    pub material: *mut mjString,
    pub group: i32,
    pub rgba: [u8; 20],
    pub meshname: *mut mjString,
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
}

/// Sized opaque C struct: std__map_std__string__std__array_mjtNum__3 (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__map_std__string__std__array_mjtNum__3 { pub _data: [u8; 24] }

/// Sized opaque C struct: std__map_std__string__std__array_mjtNum__4 (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__map_std__string__std__array_mjtNum__4 { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_OctNode (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_OctNode { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_Point (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_Point { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_Triangle (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_Triangle { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_double (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_double { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_float (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_float { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_int (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_int { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_mjCBoundingVolume (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCBoundingVolume { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_std__pair_int__int (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_std__pair_int__int { pub _data: [u8; 24] }

/// Sized opaque C struct: std__vector_std__vector_int (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_std__vector_int { pub _data: [u8; 24] }

/// C struct: mjCBody (2016 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCBody {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub parent: *mut mjCBody,
    pub weldid: i32,
    pub dofnum: i32,
    pub mocapid: i32,
    pub contype: i32,
    pub conaffinity: i32,
    pub _pad_1: [u8; 4],
    pub margin: f64,
    pub xpos0: [f64; 3],
    pub xquat0: [f64; 4],
    pub lastdof: i32,
    pub subtreedofs: i32,
    pub tree: mjCBoundingVolumeHierarchy,
    pub plugin_name: std__string,
    pub plugin_instance_name: std__string,
    pub userdata_: std__vector_double,
    pub spec_userdata_: std__vector_double,
    pub mpos_: std__map_std__string__std__array_mjtNum__3,
    pub mquat_: std__map_std__string__std__array_mjtNum__4,
    pub element: *mut mjsElement,
    pub childclass: *mut mjString,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub mass: f64,
    pub ipos: [f64; 3],
    pub iquat: [f64; 4],
    pub inertia: [f64; 3],
    pub ialt: mjsOrientation,
    pub fullinertia: [f64; 6],
    pub mocap: bool,
    pub _pad_2: [u8; 7],
    pub gravcomp: f64,
    pub sleep: u32,
    pub simple: u8,
    pub _pad_3: [u8; 3],
    pub userdata: *mut mjDoubleVec,
    pub explicitinertial: bool,
    pub _pad_4: [u8; 7],
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
    pub spec: mjsBody,
    pub last_attached: *mut mjsFrame,
    pub bodies: std__vector_mjCBody,
    pub geoms: std__vector_mjCGeom,
    pub frames: std__vector_mjCFrame,
    pub joints: std__vector_mjCJoint,
    pub sites: std__vector_mjCSite,
    pub cameras: std__vector_mjCCamera,
    pub lights: std__vector_mjCLight,
}

const _: () = assert!(std::mem::size_of::<mjCBody>() == 2016);

/// C struct: mjCBoundingVolumeHierarchy (232 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCBoundingVolumeHierarchy {
    pub nbvh_: i32,
    pub _pad_0: [u8; 4],
    pub bvh_: std__vector_mjtNum,
    pub child_: std__vector_int,
    pub nodeid_: std__vector_int,
    pub nodeidptr_: std__vector_int,
    pub level_: std__vector_int,
    pub bvleaf_: std__vector_mjCBoundingVolume,
    pub name_: std__string,
    pub ipos_: [f64; 3],
    pub iquat_: [f64; 4],
}

const _: () = assert!(std::mem::size_of::<mjCBoundingVolumeHierarchy>() == 232);

/// C struct: mjCMesh (1968 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCMesh {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub plugin_name: std__string,
    pub plugin_instance_name: std__string,
    pub content_type_: std__string,
    pub file_: std__string,
    pub resource_: *mut mjResource,
    pub vert_: std__vector_float,
    pub normal_: std__vector_float,
    pub texcoord_: std__vector_float,
    pub face_: std__vector_int,
    pub facenormal_: std__vector_int,
    pub facetexcoord_: std__vector_int,
    pub material_: std__string,
    pub spec_content_type_: std__string,
    pub spec_file_: std__string,
    pub spec_vert_: std__vector_float,
    pub spec_normal_: std__vector_float,
    pub spec_texcoord_: std__vector_float,
    pub spec_face_: std__vector_int,
    pub spec_facenormal_: std__vector_int,
    pub spec_facetexcoord_: std__vector_int,
    pub spec_material_: std__string,
    pub needreorient_: bool,
    pub visual_: bool,
    pub _pad_1: [u8; 6],
    pub halfedge_: std__vector_std__pair_int__int,
    pub processed_: bool,
    pub transformed_: bool,
    pub _pad_2: [u8; 6],
    pub pos_: [f64; 3],
    pub quat_: [f64; 4],
    pub boxsz_: [f64; 3],
    pub aamm_: [f64; 6],
    pub volume_: f64,
    pub surface_: f64,
    pub szgraph_: i32,
    pub needhull_: bool,
    pub _pad_3: [u8; 3],
    pub maxhullvert_: i32,
    pub _pad_4: [u8; 4],
    pub tree_: mjCBoundingVolumeHierarchy,
    pub face_aabb_: std__vector_double,
    pub octree_: mjCOctree,
    pub mesh_timer_: [f64; 9],
    pub element: *mut mjsElement,
    pub content_type: *mut mjString,
    pub file: *mut mjString,
    pub refpos: [f64; 3],
    pub refquat: [f64; 4],
    pub scale: [f64; 3],
    pub inertia: u32,
    pub smoothnormal: bool,
    pub needsdf: bool,
    pub _pad_5: [u8; 2],
    pub maxhullvert: i32,
    pub _pad_6: [u8; 4],
    pub uservert: *mut mjFloatVec,
    pub usernormal: *mut mjFloatVec,
    pub usertexcoord: *mut mjFloatVec,
    pub userface: *mut mjIntVec,
    pub userfacenormal: *mut mjIntVec,
    pub userfacetexcoord: *mut mjIntVec,
    pub plugin: mjsPlugin,
    pub material: *mut mjString,
    pub octree_maxdepth: i32,
    pub _pad_7: [u8; 4],
    pub info: *mut mjString,
    pub spec: mjsMesh,
    pub center_: *mut f64,
    pub graph_: *mut i32,
    pub polygons_: std__vector_std__vector_int,
    pub polygon_normals_: std__vector_double,
    pub polygon_map_: std__vector_std__vector_int,
}

const _: () = assert!(std::mem::size_of::<mjCMesh>() == 1968);

/// C struct: mjCOctree (176 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCOctree {
    pub nnode_: i32,
    pub nvert_: i32,
    pub max_depth_: i32,
    pub _pad_0: [u8; 4],
    pub node_: std__vector_OctNode,
    pub face_: std__vector_Triangle,
    pub vert_: std__vector_Point,
    pub hang_: std__vector_std__vector_int,
    pub ipos_: [f64; 3],
    pub iquat_: [f64; 4],
    pub smoothing_iterations_: i32,
    pub _pad_1: [u8; 4],
}

const _: () = assert!(std::mem::size_of::<mjCOctree>() == 176);

/// C struct: mjCSite (1064 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCSite {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub material_: std__string,
    pub meshname_: std__string,
    pub userdata_: std__vector_double,
    pub spec_material_: std__string,
    pub spec_meshname_: std__string,
    pub spec_userdata_: std__vector_double,
    pub body: *mut mjCBody,
    pub matid: i32,
    pub _pad_1: [u8; 4],
    pub mesh: *mut mjCMesh,
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub fromto: [f64; 6],
    pub size: [f64; 3],
    pub r#type: u32,
    pub _pad_2: [u8; 4],
    pub material: *mut mjString,
    pub group: i32,
    pub rgba: [u8; 20],
    pub meshname: *mut mjString,
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
    pub spec: mjsSite,
}

const _: () = assert!(std::mem::size_of::<mjCSite>() == 1064);

/// Sized opaque C struct: mjFloatVecVec (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjFloatVecVec { pub _data: [u8; 24] }

/// Sized opaque C struct: mjIntVecVec (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjIntVecVec { pub _data: [u8; 24] }

/// Sized opaque C struct: mjStringVec (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjStringVec { pub _data: [u8; 24] }

/// C struct: mjsSkin (120 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsSkin {
    pub element: *mut mjsElement,
    pub file: *mut mjString,
    pub material: *mut mjString,
    pub rgba: [f32; 4],
    pub inflate: f32,
    pub group: i32,
    pub vert: *mut mjFloatVec,
    pub texcoord: *mut mjFloatVec,
    pub face: *mut mjIntVec,
    pub bodyname: *mut mjStringVec,
    pub bindpos: *mut mjFloatVec,
    pub bindquat: *mut mjFloatVec,
    pub vertid: *mut mjIntVecVec,
    pub vertweight: *mut mjFloatVecVec,
    pub info: *mut mjString,
}

/// Sized opaque C struct: std__vector_std__vector_float (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_std__vector_float { pub _data: [u8; 24] }

/// C struct: mjCSkin (976 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCSkin {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub file_: std__string,
    pub material_: std__string,
    pub vert_: std__vector_float,
    pub texcoord_: std__vector_float,
    pub face_: std__vector_int,
    pub bodyname_: std__vector_std__string,
    pub bindpos_: std__vector_float,
    pub bindquat_: std__vector_float,
    pub vertid_: std__vector_std__vector_int,
    pub vertweight_: std__vector_std__vector_float,
    pub spec_file_: std__string,
    pub spec_material_: std__string,
    pub spec_vert_: std__vector_float,
    pub spec_texcoord_: std__vector_float,
    pub spec_face_: std__vector_int,
    pub spec_bodyname_: std__vector_std__string,
    pub spec_bindpos_: std__vector_float,
    pub spec_bindquat_: std__vector_float,
    pub spec_vertid_: std__vector_std__vector_int,
    pub spec_vertweight_: std__vector_std__vector_float,
    pub matid: i32,
    pub _pad_1: [u8; 4],
    pub bodyid: std__vector_int,
    pub element: *mut mjsElement,
    pub file: *mut mjString,
    pub material: *mut mjString,
    pub rgba: [f32; 4],
    pub inflate: f32,
    pub group: i32,
    pub vert: *mut mjFloatVec,
    pub texcoord: *mut mjFloatVec,
    pub face: *mut mjIntVec,
    pub bodyname: *mut mjStringVec,
    pub bindpos: *mut mjFloatVec,
    pub bindquat: *mut mjFloatVec,
    pub vertid: *mut mjIntVecVec,
    pub vertweight: *mut mjFloatVecVec,
    pub info: *mut mjString,
    pub spec: mjsSkin,
}

const _: () = assert!(std::mem::size_of::<mjCSkin>() == 976);

/// C struct: mjsCamera_ (320 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsCamera {
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub mode: u32,
    pub _pad_0: [u8; 4],
    pub targetbody: *mut mjString,
    pub proj: u32,
    pub resolution: [i32; 2],
    pub output: i32,
    pub fovy: f64,
    pub ipd: f64,
    pub intrinsic: [f32; 4],
    pub sensor_size: [f32; 2],
    pub focal_length: [f32; 2],
    pub focal_pixel: [f32; 2],
    pub principal_length: [f32; 2],
    pub principal_pixel: [f32; 2],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjsCamera>() == 320);

const _: () = assert!(std::mem::size_of::<mjsElement>() == 16);

/// Sized opaque C struct: DynArray_XMLNode____10 (104 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct DynArray_XMLNode____10 { pub _data: [u8; 104] }

/// Sized opaque C struct: MemPoolT_sizeof_XMLAttribute (152 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct MemPoolT_sizeof_XMLAttribute { pub _data: [u8; 152] }

/// Sized opaque C struct: MemPoolT_sizeof_XMLComment (152 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct MemPoolT_sizeof_XMLComment { pub _data: [u8; 152] }

/// Sized opaque C struct: MemPoolT_sizeof_XMLElement (152 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct MemPoolT_sizeof_XMLElement { pub _data: [u8; 152] }

/// Sized opaque C struct: MemPoolT_sizeof_XMLText (152 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct MemPoolT_sizeof_XMLText { pub _data: [u8; 152] }

/// C enum: tinyxml2::Whitespace
pub type tinyxml2_Whitespace = u32;

pub const tinyxml2_Whitespace_PRESERVE_WHITESPACE: tinyxml2_Whitespace = 0;

pub const tinyxml2_Whitespace_COLLAPSE_WHITESPACE: tinyxml2_Whitespace = 1;

pub const tinyxml2_Whitespace_PEDANTIC_WHITESPACE: tinyxml2_Whitespace = 2;

/// C enum: tinyxml2::XMLError
pub type tinyxml2_XMLError = u32;

pub const tinyxml2_XMLError_XML_SUCCESS: tinyxml2_XMLError = 0;

pub const tinyxml2_XMLError_XML_NO_ATTRIBUTE: tinyxml2_XMLError = 1;

pub const tinyxml2_XMLError_XML_WRONG_ATTRIBUTE_TYPE: tinyxml2_XMLError = 2;

pub const tinyxml2_XMLError_XML_ERROR_FILE_NOT_FOUND: tinyxml2_XMLError = 3;

pub const tinyxml2_XMLError_XML_ERROR_FILE_COULD_NOT_BE_OPENED: tinyxml2_XMLError = 4;

pub const tinyxml2_XMLError_XML_ERROR_FILE_READ_ERROR: tinyxml2_XMLError = 5;

pub const tinyxml2_XMLError_XML_ERROR_PARSING_ELEMENT: tinyxml2_XMLError = 6;

pub const tinyxml2_XMLError_XML_ERROR_PARSING_ATTRIBUTE: tinyxml2_XMLError = 7;

pub const tinyxml2_XMLError_XML_ERROR_PARSING_TEXT: tinyxml2_XMLError = 8;

pub const tinyxml2_XMLError_XML_ERROR_PARSING_CDATA: tinyxml2_XMLError = 9;

pub const tinyxml2_XMLError_XML_ERROR_PARSING_COMMENT: tinyxml2_XMLError = 10;

pub const tinyxml2_XMLError_XML_ERROR_PARSING_DECLARATION: tinyxml2_XMLError = 11;

pub const tinyxml2_XMLError_XML_ERROR_PARSING_UNKNOWN: tinyxml2_XMLError = 12;

pub const tinyxml2_XMLError_XML_ERROR_EMPTY_DOCUMENT: tinyxml2_XMLError = 13;

pub const tinyxml2_XMLError_XML_ERROR_MISMATCHED_ELEMENT: tinyxml2_XMLError = 14;

pub const tinyxml2_XMLError_XML_ERROR_PARSING: tinyxml2_XMLError = 15;

pub const tinyxml2_XMLError_XML_CAN_NOT_CONVERT_TEXT: tinyxml2_XMLError = 16;

pub const tinyxml2_XMLError_XML_NO_TEXT_NODE: tinyxml2_XMLError = 17;

pub const tinyxml2_XMLError_XML_ELEMENT_DEPTH_EXCEEDED: tinyxml2_XMLError = 18;

pub const tinyxml2_XMLError_XML_ERROR_COUNT: tinyxml2_XMLError = 19;

/// C struct: tinyxml2::MemPool (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__MemPool {
    pub _vtable: *mut (),
}

const _: () = assert!(std::mem::size_of::<tinyxml2__MemPool>() == 8);

/// C struct: tinyxml2::StrPair (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__StrPair {
    pub _flags: i32,
    pub _pad_0: [u8; 4],
    pub _start: *mut i8,
    pub _end: *mut i8,
}

const _: () = assert!(std::mem::size_of::<tinyxml2__StrPair>() == 24);

/// C struct: tinyxml2::XMLConstHandle (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLConstHandle {
    pub _node: *const tinyxml2__XMLNode,
}

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLConstHandle>() == 8);

/// C struct: tinyxml2::XMLDocument (880 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLDocument {
    pub _vtable: *mut (),
    pub _document: *mut tinyxml2__XMLDocument,
    pub _parent: *mut tinyxml2__XMLNode,
    pub _value: tinyxml2__StrPair,
    pub _parseLineNum: i32,
    pub _pad_0: [u8; 4],
    pub _firstChild: *mut tinyxml2__XMLNode,
    pub _lastChild: *mut tinyxml2__XMLNode,
    pub _prev: *mut tinyxml2__XMLNode,
    pub _next: *mut tinyxml2__XMLNode,
    pub _userData: *mut (),
    pub _memPool: *mut tinyxml2__MemPool,
    pub _writeBOM: bool,
    pub _processEntities: bool,
    pub _pad_1: [u8; 2],
    pub _errorID: u32,
    pub _whitespaceMode: u32,
    pub _pad_2: [u8; 4],
    pub _errorStr: tinyxml2__StrPair,
    pub _errorLineNum: i32,
    pub _pad_3: [u8; 4],
    pub _charBuffer: *mut i8,
    pub _parseCurLineNum: i32,
    pub _parsingDepth: i32,
    pub _unlinked: DynArray_XMLNode____10,
    pub _elementPool: MemPoolT_sizeof_XMLElement,
    pub _attributePool: MemPoolT_sizeof_XMLAttribute,
    pub _textPool: MemPoolT_sizeof_XMLText,
    pub _commentPool: MemPoolT_sizeof_XMLComment,
}

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLDocument>() == 880);

/// C struct: tinyxml2::XMLNode (104 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLNode {
    pub _vtable: *mut (),
    pub _document: *mut tinyxml2__XMLDocument,
    pub _parent: *mut tinyxml2__XMLNode,
    pub _value: tinyxml2__StrPair,
    pub _parseLineNum: i32,
    pub _pad_0: [u8; 4],
    pub _firstChild: *mut tinyxml2__XMLNode,
    pub _lastChild: *mut tinyxml2__XMLNode,
    pub _prev: *mut tinyxml2__XMLNode,
    pub _next: *mut tinyxml2__XMLNode,
    pub _userData: *mut (),
    pub _memPool: *mut tinyxml2__MemPool,
}

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLNode>() == 104);

/// C struct: tinyxml2::XMLText (112 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLText {
    pub _vtable: *mut (),
    pub _document: *mut tinyxml2__XMLDocument,
    pub _parent: *mut tinyxml2__XMLNode,
    pub _value: tinyxml2__StrPair,
    pub _parseLineNum: i32,
    pub _pad_0: [u8; 4],
    pub _firstChild: *mut tinyxml2__XMLNode,
    pub _lastChild: *mut tinyxml2__XMLNode,
    pub _prev: *mut tinyxml2__XMLNode,
    pub _next: *mut tinyxml2__XMLNode,
    pub _userData: *mut (),
    pub _memPool: *mut tinyxml2__MemPool,
    pub _isCData: bool,
    pub _pad_1: [u8; 7],
}

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLText>() == 112);

/// C struct: tinyxml2::XMLComment (104 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLComment {
    pub _vtable: *mut (),
    pub _document: *mut tinyxml2__XMLDocument,
    pub _parent: *mut tinyxml2__XMLNode,
    pub _value: tinyxml2__StrPair,
    pub _parseLineNum: i32,
    pub _pad_0: [u8; 4],
    pub _firstChild: *mut tinyxml2__XMLNode,
    pub _lastChild: *mut tinyxml2__XMLNode,
    pub _prev: *mut tinyxml2__XMLNode,
    pub _next: *mut tinyxml2__XMLNode,
    pub _userData: *mut (),
    pub _memPool: *mut tinyxml2__MemPool,
}

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLComment>() == 104);

/// Sized opaque C struct: tinyxml2__XMLUtil (1 bytes, align 1)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLUtil { pub _data: [u8; 1] }

/// C struct: mjsSensor (168 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsSensor {
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub objtype: u32,
    pub objname: *mut mjString,
    pub reftype: u32,
    pub _pad_0: [u8; 4],
    pub refname: *mut mjString,
    pub intprm: [i32; 3],
    pub datatype: u32,
    pub needstage: u32,
    pub dim: i32,
    pub cutoff: f64,
    pub noise: f64,
    pub nsample: i32,
    pub interp: i32,
    pub delay: f64,
    pub interval: [f64; 2],
    pub userdata: *mut mjDoubleVec,
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
}

/// Sized opaque C struct: std__unordered_map_std__string__UserValue (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__unordered_map_std__string__UserValue { pub _data: [u8; 40] }

/// C enum: mjtDataType
pub type mjtDataType = u32;

pub const mjtDataType_mjDATATYPE_REAL: mjtDataType = 0;

pub const mjtDataType_mjDATATYPE_POSITIVE: mjtDataType = 1;

pub const mjtDataType_mjDATATYPE_AXIS: mjtDataType = 2;

pub const mjtDataType_mjDATATYPE_QUATERNION: mjtDataType = 3;

/// C enum: mjtSensor
pub type mjtSensor = u32;

pub const mjtSensor_mjSENS_TOUCH: mjtSensor = 0;

pub const mjtSensor_mjSENS_ACCELEROMETER: mjtSensor = 1;

pub const mjtSensor_mjSENS_VELOCIMETER: mjtSensor = 2;

pub const mjtSensor_mjSENS_GYRO: mjtSensor = 3;

pub const mjtSensor_mjSENS_FORCE: mjtSensor = 4;

pub const mjtSensor_mjSENS_TORQUE: mjtSensor = 5;

pub const mjtSensor_mjSENS_MAGNETOMETER: mjtSensor = 6;

pub const mjtSensor_mjSENS_RANGEFINDER: mjtSensor = 7;

pub const mjtSensor_mjSENS_CAMPROJECTION: mjtSensor = 8;

pub const mjtSensor_mjSENS_JOINTPOS: mjtSensor = 9;

pub const mjtSensor_mjSENS_JOINTVEL: mjtSensor = 10;

pub const mjtSensor_mjSENS_TENDONPOS: mjtSensor = 11;

pub const mjtSensor_mjSENS_TENDONVEL: mjtSensor = 12;

pub const mjtSensor_mjSENS_ACTUATORPOS: mjtSensor = 13;

pub const mjtSensor_mjSENS_ACTUATORVEL: mjtSensor = 14;

pub const mjtSensor_mjSENS_ACTUATORFRC: mjtSensor = 15;

pub const mjtSensor_mjSENS_JOINTACTFRC: mjtSensor = 16;

pub const mjtSensor_mjSENS_TENDONACTFRC: mjtSensor = 17;

pub const mjtSensor_mjSENS_BALLQUAT: mjtSensor = 18;

pub const mjtSensor_mjSENS_BALLANGVEL: mjtSensor = 19;

pub const mjtSensor_mjSENS_JOINTLIMITPOS: mjtSensor = 20;

pub const mjtSensor_mjSENS_JOINTLIMITVEL: mjtSensor = 21;

pub const mjtSensor_mjSENS_JOINTLIMITFRC: mjtSensor = 22;

pub const mjtSensor_mjSENS_TENDONLIMITPOS: mjtSensor = 23;

pub const mjtSensor_mjSENS_TENDONLIMITVEL: mjtSensor = 24;

pub const mjtSensor_mjSENS_TENDONLIMITFRC: mjtSensor = 25;

pub const mjtSensor_mjSENS_FRAMEPOS: mjtSensor = 26;

pub const mjtSensor_mjSENS_FRAMEQUAT: mjtSensor = 27;

pub const mjtSensor_mjSENS_FRAMEXAXIS: mjtSensor = 28;

pub const mjtSensor_mjSENS_FRAMEYAXIS: mjtSensor = 29;

pub const mjtSensor_mjSENS_FRAMEZAXIS: mjtSensor = 30;

pub const mjtSensor_mjSENS_FRAMELINVEL: mjtSensor = 31;

pub const mjtSensor_mjSENS_FRAMEANGVEL: mjtSensor = 32;

pub const mjtSensor_mjSENS_FRAMELINACC: mjtSensor = 33;

pub const mjtSensor_mjSENS_FRAMEANGACC: mjtSensor = 34;

pub const mjtSensor_mjSENS_SUBTREECOM: mjtSensor = 35;

pub const mjtSensor_mjSENS_SUBTREELINVEL: mjtSensor = 36;

pub const mjtSensor_mjSENS_SUBTREEANGMOM: mjtSensor = 37;

pub const mjtSensor_mjSENS_INSIDESITE: mjtSensor = 38;

pub const mjtSensor_mjSENS_GEOMDIST: mjtSensor = 39;

pub const mjtSensor_mjSENS_GEOMNORMAL: mjtSensor = 40;

pub const mjtSensor_mjSENS_GEOMFROMTO: mjtSensor = 41;

pub const mjtSensor_mjSENS_CONTACT: mjtSensor = 42;

pub const mjtSensor_mjSENS_E_POTENTIAL: mjtSensor = 43;

pub const mjtSensor_mjSENS_E_KINETIC: mjtSensor = 44;

pub const mjtSensor_mjSENS_CLOCK: mjtSensor = 45;

pub const mjtSensor_mjSENS_TACTILE: mjtSensor = 46;

pub const mjtSensor_mjSENS_PLUGIN: mjtSensor = 47;

pub const mjtSensor_mjSENS_USER: mjtSensor = 48;

/// C enum: mjtStage
pub type mjtStage = u32;

pub const mjtStage_mjSTAGE_NONE: mjtStage = 0;

pub const mjtStage_mjSTAGE_POS: mjtStage = 1;

pub const mjtStage_mjSTAGE_VEL: mjtStage = 2;

pub const mjtStage_mjSTAGE_ACC: mjtStage = 3;

/// C struct: mjCBase (224 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCBase {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 16],
    pub id: i32,
    pub _pad_1: [u8; 4],
    pub name: std__string,
    pub classname: std__string,
    pub info: std__string,
    pub prefix: std__string,
    pub suffix: std__string,
    pub frame: *mut mjCFrame,
    pub model: *mut mjCModel,
    pub compiler: *mut mjsCompiler,
    pub refcount: i32,
    pub _pad_2: [u8; 4],
    pub user_payload_: std__unordered_map_std__string__UserValue,
}

const _: () = assert!(std::mem::size_of::<mjCBase>() == 224);

/// C struct: mjCFrame (680 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCFrame {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub compiled: bool,
    pub _pad_1: [u8; 7],
    pub element: *mut mjsElement,
    pub childclass: *mut mjString,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub info: *mut mjString,
    pub spec: mjsFrame,
    pub last_attached: *mut mjsBody,
    pub body: *mut mjCBody,
}

const _: () = assert!(std::mem::size_of::<mjCFrame>() == 680);

/// C struct: mjCSensor (768 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCSensor {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub plugin_name: std__string,
    pub plugin_instance_name: std__string,
    pub objname_: std__string,
    pub refname_: std__string,
    pub userdata_: std__vector_double,
    pub spec_objname_: std__string,
    pub spec_refname_: std__string,
    pub spec_userdata_: std__vector_double,
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub objtype: u32,
    pub objname: *mut mjString,
    pub reftype: u32,
    pub _pad_1: [u8; 4],
    pub refname: *mut mjString,
    pub intprm: [i32; 3],
    pub datatype: u32,
    pub needstage: u32,
    pub dim: i32,
    pub cutoff: f64,
    pub noise: f64,
    pub nsample: i32,
    pub interp: i32,
    pub delay: f64,
    pub interval: [f64; 2],
    pub userdata: *mut mjDoubleVec,
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
    pub spec: mjsSensor,
    pub obj: *mut mjCBase,
    pub r#ref: *mut mjCBase,
}

const _: () = assert!(std::mem::size_of::<mjCSensor>() == 768);

/// C struct: mjsGeom (640 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsGeom {
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub _pad_0: [u8; 4],
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub fromto: [f64; 6],
    pub size: [f64; 3],
    pub contype: i32,
    pub conaffinity: i32,
    pub condim: i32,
    pub priority: i32,
    pub friction: [f64; 3],
    pub solmix: f64,
    pub solref: [f64; 2],
    pub solimp: [f64; 5],
    pub margin: f64,
    pub gap: f64,
    pub surfacevel: [f64; 6],
    pub adhesion: f64,
    pub mass: f64,
    pub density: f64,
    pub typeinertia: u32,
    pub _pad_1: [u8; 4],
    pub fluid_ellipsoid: f64,
    pub fluid_coefs: [f64; 5],
    pub material: *mut mjString,
    pub rgba: [f32; 4],
    pub group: i32,
    pub _pad_2: [u8; 4],
    pub hfieldname: *mut mjString,
    pub meshname: *mut mjString,
    pub fitscale: f64,
    pub userdata: *mut mjDoubleVec,
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
}

/// C struct: mjsHField (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsHField {
    pub element: *mut mjsElement,
    pub content_type: *mut mjString,
    pub file: *mut mjString,
    pub size: [f64; 4],
    pub nrow: i32,
    pub ncol: i32,
    pub userdata: *mut mjFloatVec,
    pub info: *mut mjString,
}

/// C struct: mjsPair (176 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsPair {
    pub element: *mut mjsElement,
    pub geomname1: *mut mjString,
    pub geomname2: *mut mjString,
    pub condim: i32,
    pub _pad_0: [u8; 4],
    pub solref: [f64; 2],
    pub solreffriction: [f64; 2],
    pub solimp: [f64; 5],
    pub margin: f64,
    pub gap: f64,
    pub adhesion: f64,
    pub friction: [f64; 5],
    pub info: *mut mjString,
}

/// C struct: mjCGeom (1952 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCGeom {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub inferinertia: bool,
    pub visual_: bool,
    pub _pad_1: [u8; 2],
    pub matid: i32,
    pub mesh: *mut mjCMesh,
    pub hfield: *mut mjCHField,
    pub mass_: f64,
    pub inertia: [f64; 3],
    pub aabb: [f64; 6],
    pub body: *mut mjCBody,
    pub fluid: [f64; 12],
    pub plugin_name: std__string,
    pub plugin_instance_name: std__string,
    pub hfieldname_: std__string,
    pub meshname_: std__string,
    pub material_: std__string,
    pub userdata_: std__vector_double,
    pub spec_hfieldname_: std__string,
    pub spec_meshname_: std__string,
    pub spec_material_: std__string,
    pub spec_userdata_: std__vector_double,
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub _pad_2: [u8; 4],
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub fromto: [f64; 6],
    pub size: [f64; 3],
    pub contype: i32,
    pub conaffinity: i32,
    pub condim: i32,
    pub priority: i32,
    pub friction: [f64; 3],
    pub solmix: f64,
    pub solref: [f64; 2],
    pub solimp: [f64; 5],
    pub margin: f64,
    pub gap: f64,
    pub surfacevel: [f64; 6],
    pub adhesion: f64,
    pub mass: f64,
    pub density: f64,
    pub typeinertia: u32,
    pub _pad_3: [u8; 4],
    pub fluid_ellipsoid: f64,
    pub fluid_coefs: [f64; 5],
    pub material: *mut mjString,
    pub rgba: [f32; 4],
    pub group: i32,
    pub _pad_4: [u8; 4],
    pub hfieldname: *mut mjString,
    pub meshname: *mut mjString,
    pub fitscale: f64,
    pub userdata: *mut mjDoubleVec,
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
    pub spec: mjsGeom,
}

const _: () = assert!(std::mem::size_of::<mjCGeom>() == 1952);

/// C struct: mjCHField (552 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCHField {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub data: std__vector_float,
    pub file_: std__string,
    pub content_type_: std__string,
    pub userdata_: std__vector_float,
    pub spec_file_: std__string,
    pub spec_content_type_: std__string,
    pub spec_userdata_: std__vector_float,
    pub element: *mut mjsElement,
    pub content_type: *mut mjString,
    pub file: *mut mjString,
    pub size: [f64; 4],
    pub nrow: i32,
    pub ncol: i32,
    pub userdata: *mut mjFloatVec,
    pub info: *mut mjString,
    pub spec: mjsHField,
}

const _: () = assert!(std::mem::size_of::<mjCHField>() == 552);

/// C struct: mjCPair (696 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCPair {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub signature: i32,
    pub _pad_1: [u8; 4],
    pub geomname1_: std__string,
    pub geomname2_: std__string,
    pub spec_geomname1_: std__string,
    pub spec_geomname2_: std__string,
    pub element: *mut mjsElement,
    pub geomname1: *mut mjString,
    pub geomname2: *mut mjString,
    pub condim: i32,
    pub _pad_2: [u8; 4],
    pub solref: [f64; 2],
    pub solreffriction: [f64; 2],
    pub solimp: [f64; 5],
    pub margin: f64,
    pub gap: f64,
    pub adhesion: f64,
    pub friction: [f64; 5],
    pub info: *mut mjString,
    pub spec: mjsPair,
    pub geom1: *mut mjCGeom,
    pub geom2: *mut mjCGeom,
}

const _: () = assert!(std::mem::size_of::<mjCPair>() == 696);

const _: () = assert!(std::mem::size_of::<mjListKeyMap>() == 624);

const _: () = assert!(std::mem::align_of::<mjListKeyMap>() == 8);

/// Sized opaque C struct: mjfCloseResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfCloseResource { pub _data: [u8; 8] }

const _: () = assert!(std::mem::size_of::<mjfCloseResource>() == 8);

const _: () = assert!(std::mem::align_of::<mjfCloseResource>() == 8);

/// Sized opaque C struct: mjfMountResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfMountResource { pub _data: [u8; 8] }

const _: () = assert!(std::mem::size_of::<mjfMountResource>() == 8);

const _: () = assert!(std::mem::align_of::<mjfMountResource>() == 8);

/// Sized opaque C struct: mjfOpenResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfOpenResource { pub _data: [u8; 8] }

const _: () = assert!(std::mem::size_of::<mjfOpenResource>() == 8);

const _: () = assert!(std::mem::align_of::<mjfOpenResource>() == 8);

/// Sized opaque C struct: mjfReadResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfReadResource { pub _data: [u8; 8] }

const _: () = assert!(std::mem::size_of::<mjfReadResource>() == 8);

const _: () = assert!(std::mem::align_of::<mjfReadResource>() == 8);

/// Sized opaque C struct: mjfResourceModified (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfResourceModified { pub _data: [u8; 8] }

const _: () = assert!(std::mem::size_of::<mjfResourceModified>() == 8);

const _: () = assert!(std::mem::align_of::<mjfResourceModified>() == 8);

/// Sized opaque C struct: mjfUnmountResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfUnmountResource { pub _data: [u8; 8] }

const _: () = assert!(std::mem::size_of::<mjfUnmountResource>() == 8);

const _: () = assert!(std::mem::align_of::<mjfUnmountResource>() == 8);

/// Sized opaque C struct: mjfWriteResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfWriteResource { pub _data: [u8; 8] }

const _: () = assert!(std::mem::size_of::<mjfWriteResource>() == 8);

const _: () = assert!(std::mem::align_of::<mjfWriteResource>() == 8);

const _: () = assert!(std::mem::size_of::<std__array_std__vector_mjCBase_______mjNOBJECT>() == 208);

const _: () = assert!(std::mem::align_of::<std__array_std__vector_mjCBase_______mjNOBJECT>() == 8);

/// Sized opaque C struct: std__map_std__string__mjtNum (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__map_std__string__mjtNum { pub _data: [u8; 24] }

const _: () = assert!(std::mem::size_of::<std__map_std__string__mjtNum>() == 24);

const _: () = assert!(std::mem::align_of::<std__map_std__string__mjtNum>() == 8);

const _: () = assert!(std::mem::size_of::<std__map_std__string__std__array_mjtNum__3>() == 24);

const _: () = assert!(std::mem::align_of::<std__map_std__string__std__array_mjtNum__3>() == 8);

const _: () = assert!(std::mem::size_of::<std__map_std__string__std__array_mjtNum__4>() == 24);

const _: () = assert!(std::mem::align_of::<std__map_std__string__std__array_mjtNum__4>() == 8);

/// Sized opaque C struct: std__map_std__string__std__array_mjtNum__6 (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__map_std__string__std__array_mjtNum__6 { pub _data: [u8; 24] }

const _: () = assert!(std::mem::size_of::<std__map_std__string__std__array_mjtNum__6>() == 24);

const _: () = assert!(std::mem::align_of::<std__map_std__string__std__array_mjtNum__6>() == 8);

/// Sized opaque C struct: std__map_std__string__std__array_mjtNum__7 (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__map_std__string__std__array_mjtNum__7 { pub _data: [u8; 24] }

const _: () = assert!(std::mem::size_of::<std__map_std__string__std__array_mjtNum__7>() == 24);

const _: () = assert!(std::mem::align_of::<std__map_std__string__std__array_mjtNum__7>() == 8);

/// Sized opaque C struct: std__map_std__string__std__vector_mjtNum (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__map_std__string__std__vector_mjtNum { pub _data: [u8; 24] }

const _: () = assert!(std::mem::size_of::<std__map_std__string__std__vector_mjtNum>() == 24);

const _: () = assert!(std::mem::align_of::<std__map_std__string__std__vector_mjtNum>() == 8);

const _: () = assert!(std::mem::size_of::<std__string>() == 24);

const _: () = assert!(std::mem::align_of::<std__string>() == 8);

const _: () = assert!(std::mem::size_of::<std__unordered_map_const_mjsCompiler____mjSpec>() == 40);

const _: () = assert!(std::mem::align_of::<std__unordered_map_const_mjsCompiler____mjSpec>() == 8);

const _: () = assert!(std::mem::size_of::<std__unordered_map_std__string__UserValue>() == 40);

const _: () = assert!(std::mem::align_of::<std__unordered_map_std__string__UserValue>() == 8);

const _: () = assert!(std::mem::size_of::<std__unordered_map_std__string__mjCDef>() == 40);

const _: () = assert!(std::mem::align_of::<std__unordered_map_std__string__mjCDef>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_OctNode>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_OctNode>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_Point>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_Point>() == 8);

/// Sized opaque C struct: std__vector_StencilFlap (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_StencilFlap { pub _data: [u8; 24] }

const _: () = assert!(std::mem::size_of::<std__vector_StencilFlap>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_StencilFlap>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_Triangle>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_Triangle>() == 8);

/// Sized opaque C struct: std__vector_bool (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_bool { pub _data: [u8; 24] }

const _: () = assert!(std::mem::size_of::<std__vector_bool>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_bool>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_double>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_double>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_float>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_float>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_int>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_int>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCActuator>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCActuator>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCBase>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCBase>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCBody>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCBody>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCBodyPair>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCBodyPair>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCBoundingVolume>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCBoundingVolume>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCCamera>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCCamera>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCDef>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCDef>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCEquality>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCEquality>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCFlex>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCFlex>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCFrame>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCFrame>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCGeom>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCGeom>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCHField>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCHField>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCJoint>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCJoint>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCKey>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCKey>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCLight>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCLight>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCMaterial>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCMaterial>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCMesh>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCMesh>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCNumeric>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCNumeric>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCPair>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCPair>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCPlugin>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCPlugin>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCSensor>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCSensor>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCSite>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCSite>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCSkin>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCSkin>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCTendon>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCTendon>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCText>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCText>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCTexture>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCTexture>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjCTuple>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCTuple>() == 8);

/// Sized opaque C struct: std__vector_mjCWrap (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__vector_mjCWrap { pub _data: [u8; 24] }

const _: () = assert!(std::mem::size_of::<std__vector_mjCWrap>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjCWrap>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjKeyInfo>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjKeyInfo>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjSpec>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjSpec>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_mjtNum>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_mjtNum>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_std__pair_const_mjpPlugin____int>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_std__pair_const_mjpPlugin____int>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_std__pair_int__int>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_std__pair_int__int>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_std__string>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_std__string>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_std__vector_int>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_std__vector_int>() == 8);

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_130_3 (52 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_130_3 { pub _data: [u8; 52] }

const _: () = assert!(std::mem::size_of::<struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_130_3>() == 52);

const _ : () = assert ! (std :: mem :: align_of ::< struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_130_3 > () == 4) ;

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_146_3 (20 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_146_3 { pub _data: [u8; 20] }

const _: () = assert!(std::mem::size_of::<struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_146_3>() == 20);

const _ : () = assert ! (std :: mem :: align_of ::< struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_146_3 > () == 4) ;

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_154_3 (40 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_154_3 { pub _data: [u8; 40] }

const _: () = assert!(std::mem::size_of::<struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_154_3>() == 40);

const _ : () = assert ! (std :: mem :: align_of ::< struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_154_3 > () == 4) ;

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_161_3 (52 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_161_3 { pub _data: [u8; 52] }

const _: () = assert!(std::mem::size_of::<struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_161_3>() == 52);

const _ : () = assert ! (std :: mem :: align_of ::< struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_161_3 > () == 4) ;

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_177_3 (68 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_177_3 { pub _data: [u8; 68] }

const _: () = assert!(std::mem::size_of::<struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_177_3>() == 68);

const _ : () = assert ! (std :: mem :: align_of ::< struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_177_3 > () == 4) ;

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_197_3 (400 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_197_3 { pub _data: [u8; 400] }

const _: () = assert!(std::mem::size_of::<struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_197_3>() == 400);

const _ : () = assert ! (std :: mem :: align_of ::< struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_197_3 > () == 4) ;

/// C struct: mjCActuator (1848 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCActuator {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub trnid: [i32; 2],
    pub actadr_: i32,
    pub actdim_: i32,
    pub ctrladr_: i32,
    pub ctrlnum_: i32,
    pub ctrlspec_: i32,
    pub outadr_: i32,
    pub outnum_: i32,
    pub so3_: bool,
    pub _pad_1: [u8; 3],
    pub ctrlranges_: [[f64; 2]; 4],
    pub ctrllimiteds_: [u8; 4],
    pub _pad_2: [u8; 4],
    pub act_: std__map_std__string__std__vector_mjtNum,
    pub ctrl_: std__map_std__string__mjtNum,
    pub plugin_name: std__string,
    pub plugin_instance_name: std__string,
    pub target_: std__string,
    pub slidersite_: std__string,
    pub refsite_: std__string,
    pub userdata_: std__vector_double,
    pub spec_target_: std__string,
    pub spec_slidersite_: std__string,
    pub spec_refsite_: std__string,
    pub spec_userdata_: std__vector_double,
    pub element: *mut mjsElement,
    pub gaintype: u32,
    pub _pad_3: [u8; 4],
    pub gainprm: [f64; 10],
    pub biastype: u32,
    pub _pad_4: [u8; 4],
    pub biasprm: [f64; 10],
    pub dyntype: u32,
    pub _pad_5: [u8; 4],
    pub dynprm: [f64; 10],
    pub actdim: i32,
    pub ctrlspec: i32,
    pub velrange: [f64; 2],
    pub ffrange: [f64; 2],
    pub actearly: bool,
    pub _pad_6: [u8; 3],
    pub trntype: u32,
    pub gear: [f64; 6],
    pub target: *mut mjString,
    pub refsite: *mut mjString,
    pub slidersite: *mut mjString,
    pub cranklength: f64,
    pub lengthrange: [f64; 2],
    pub inheritrange: f64,
    pub damping: [f64; 3],
    pub armature: f64,
    pub ctrllimited: u32,
    pub _pad_7: [u8; 4],
    pub ctrlrange: [f64; 2],
    pub forcelimited: u32,
    pub _pad_8: [u8; 4],
    pub forcerange: [f64; 2],
    pub actlimited: u32,
    pub _pad_9: [u8; 4],
    pub actrange: [f64; 2],
    pub group: i32,
    pub nsample: i32,
    pub interp: i32,
    pub _pad_10: [u8; 4],
    pub delay: f64,
    pub userdata: *mut mjDoubleVec,
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
    pub spec: mjsActuator,
    pub ptarget: *mut mjCBase,
}

const _: () = assert!(std::mem::size_of::<mjCActuator>() == 1848);

const _: () = assert!(std::mem::align_of::<mjCActuator>() == 8);

const _: () = assert!(std::mem::align_of::<mjCBase>() == 8);

const _: () = assert!(std::mem::align_of::<mjCBody>() == 8);

const _: () = assert!(std::mem::align_of::<mjCBoundingVolumeHierarchy>() == 8);

/// C struct: mjCCamera (976 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCCamera {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub body: *mut mjCBody,
    pub targetbodyid: i32,
    pub _pad_1: [u8; 4],
    pub targetbody_: std__string,
    pub spec_targetbody_: std__string,
    pub userdata_: std__vector_double,
    pub spec_userdata_: std__vector_double,
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub mode: u32,
    pub _pad_2: [u8; 4],
    pub targetbody: *mut mjString,
    pub proj: u32,
    pub resolution: [i32; 2],
    pub output: i32,
    pub fovy: f64,
    pub ipd: f64,
    pub intrinsic: [f32; 4],
    pub sensor_size: [f32; 2],
    pub focal_length: [f32; 2],
    pub focal_pixel: [f32; 2],
    pub principal_length: [f32; 2],
    pub principal_pixel: [f32; 2],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
    pub spec: mjsCamera,
}

const _: () = assert!(std::mem::size_of::<mjCCamera>() == 976);

const _: () = assert!(std::mem::align_of::<mjCCamera>() == 8);

/// C struct: mjCDef (14560 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCDef {
    pub elemtype: u32,
    pub _pad_0: [u8; 4],
    pub signature: u64,
    pub name: std__string,
    pub id: i32,
    pub _pad_1: [u8; 4],
    pub parent: *mut mjCDef,
    pub child: std__vector_mjCDef,
    pub spec: mjsDefault,
    pub model: *mut mjCModel,
    pub joint_: mjCJoint,
    pub geom_: mjCGeom,
    pub site_: mjCSite,
    pub camera_: mjCCamera,
    pub light_: mjCLight,
    pub flex_: mjCFlex,
    pub mesh_: mjCMesh,
    pub material_: mjCMaterial,
    pub pair_: mjCPair,
    pub equality_: mjCEquality,
    pub tendon_: mjCTendon,
    pub actuator_: mjCActuator,
}

const _: () = assert!(std::mem::size_of::<mjCDef>() == 14560);

const _: () = assert!(std::mem::align_of::<mjCDef>() == 8);

/// C struct: mjCEquality (728 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCEquality {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub obj1id: i32,
    pub obj2id: i32,
    pub name1_: std__string,
    pub name2_: std__string,
    pub spec_name1_: std__string,
    pub spec_name2_: std__string,
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub _pad_1: [u8; 4],
    pub data: [f64; 11],
    pub active: bool,
    pub _pad_2: [u8; 7],
    pub name1: *mut mjString,
    pub name2: *mut mjString,
    pub objtype: u32,
    pub _pad_3: [u8; 4],
    pub solref: [f64; 2],
    pub solimp: [f64; 5],
    pub info: *mut mjString,
    pub spec: mjsEquality,
}

const _: () = assert!(std::mem::size_of::<mjCEquality>() == 728);

const _: () = assert!(std::mem::align_of::<mjCEquality>() == 8);

const _: () = assert!(std::mem::align_of::<mjCError>() == 1);

/// C struct: mjCFlex (1960 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCFlex {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub nvert: i32,
    pub nnode: i32,
    pub nedge: i32,
    pub nelem: i32,
    pub matid: i32,
    pub rigid: bool,
    pub centered: bool,
    pub interpolated: bool,
    pub _pad_1: [u8; 1],
    pub vertbodyid: std__vector_int,
    pub nodebodyid: std__vector_int,
    pub edge: std__vector_std__pair_int__int,
    pub shell: std__vector_int,
    pub elemlayer: std__vector_int,
    pub evpair: std__vector_int,
    pub flaps: std__vector_StencilFlap,
    pub vertxpos: std__vector_double,
    pub tree: mjCBoundingVolumeHierarchy,
    pub elemaabb_: std__vector_double,
    pub edgeidx_: std__vector_int,
    pub stiffness: std__vector_double,
    pub bending: std__vector_double,
    pub has_strain_eq: bool,
    pub _pad_2: [u8; 7],
    pub cell_empty: std__vector_bool,
    pub vertbody_: std__vector_std__string,
    pub nodebody_: std__vector_std__string,
    pub vert_: std__vector_double,
    pub node_: std__vector_double,
    pub elem_: std__vector_int,
    pub texcoord_: std__vector_float,
    pub elemtexcoord_: std__vector_int,
    pub material_: std__string,
    pub spec_material_: std__string,
    pub spec_vertbody_: std__vector_std__string,
    pub spec_nodebody_: std__vector_std__string,
    pub spec_vert_: std__vector_double,
    pub spec_node_: std__vector_double,
    pub spec_elem_: std__vector_int,
    pub spec_texcoord_: std__vector_float,
    pub spec_elemtexcoord_: std__vector_int,
    pub cached_stiffness_: std__vector_double,
    pub element: *mut mjsElement,
    pub contype: i32,
    pub conaffinity: i32,
    pub condim: i32,
    pub priority: i32,
    pub friction: [f64; 3],
    pub solmix: f64,
    pub solref: [f64; 2],
    pub solimp: [f64; 5],
    pub margin: f64,
    pub gap: f64,
    pub dim: i32,
    pub _pad_3: [u8; 4],
    pub radius: f64,
    pub size: [f64; 3],
    pub internal: bool,
    pub flatskin: bool,
    pub _pad_4: [u8; 2],
    pub selfcollide: u32,
    pub passive: i32,
    pub activelayers: i32,
    pub group: i32,
    pub _pad_5: [u8; 4],
    pub edgestiffness: f64,
    pub edgedamping: f64,
    pub rgba: [f32; 4],
    pub material: *mut mjString,
    pub young: f64,
    pub poisson: f64,
    pub damping: f64,
    pub thickness: f64,
    pub elastic2d: i32,
    pub cellcount: [i32; 3],
    pub order: i32,
    pub _pad_6: [u8; 4],
    pub nodebody: *mut mjStringVec,
    pub vertbody: *mut mjStringVec,
    pub node: *mut mjDoubleVec,
    pub vert: *mut mjDoubleVec,
    pub elem: *mut mjIntVec,
    pub texcoord: *mut mjFloatVec,
    pub elemtexcoord: *mut mjIntVec,
    pub info: *mut mjString,
    pub spec: mjsFlex,
    pub vert0_: std__vector_double,
    pub node0_: std__vector_double,
}

const _: () = assert!(std::mem::size_of::<mjCFlex>() == 1960);

const _: () = assert!(std::mem::align_of::<mjCFlex>() == 8);

const _: () = assert!(std::mem::align_of::<mjCFrame>() == 8);

const _: () = assert!(std::mem::align_of::<mjCGeom>() == 8);

const _: () = assert!(std::mem::align_of::<mjCHField>() == 8);

/// C struct: mjCJoint (1056 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCJoint {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub body: *mut mjCBody,
    pub qpos_: std__map_std__string__std__array_mjtNum__7,
    pub qvel_: std__map_std__string__std__array_mjtNum__6,
    pub userdata_: std__vector_double,
    pub spec_userdata_: std__vector_double,
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub _pad_1: [u8; 4],
    pub pos: [f64; 3],
    pub axis: [f64; 3],
    pub r#ref: f64,
    pub align: u32,
    pub _pad_2: [u8; 4],
    pub stiffness: [f64; 3],
    pub springref: f64,
    pub springdamper: [f64; 2],
    pub limited: u32,
    pub _pad_3: [u8; 4],
    pub range: [f64; 2],
    pub margin: f64,
    pub solref_limit: [f64; 2],
    pub solimp_limit: [f64; 5],
    pub actfrclimited: u32,
    pub _pad_4: [u8; 4],
    pub actfrcrange: [f64; 2],
    pub armature: f64,
    pub damping: [f64; 3],
    pub frictionloss: f64,
    pub solref_friction: [f64; 2],
    pub solimp_friction: [f64; 5],
    pub group: i32,
    pub actgravcomp: bool,
    pub _pad_5: [u8; 3],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
    pub spec: mjsJoint,
    pub qposadr_: i32,
    pub dofadr_: i32,
}

const _: () = assert!(std::mem::size_of::<mjCJoint>() == 1056);

const _: () = assert!(std::mem::align_of::<mjCJoint>() == 8);

/// C struct: mjCLight (688 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCLight {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub body: *mut mjCBody,
    pub targetbodyid: i32,
    pub texid: i32,
    pub texture_: std__string,
    pub spec_texture_: std__string,
    pub targetbody_: std__string,
    pub spec_targetbody_: std__string,
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub dir: [f64; 3],
    pub mode: u32,
    pub _pad_1: [u8; 4],
    pub targetbody: *mut mjString,
    pub active: bool,
    pub _pad_2: [u8; 3],
    pub r#type: u32,
    pub texture: *mut mjString,
    pub castshadow: bool,
    pub _pad_3: [u8; 3],
    pub bulbradius: f32,
    pub intensity: f32,
    pub range: f32,
    pub attenuation: [f32; 3],
    pub cutoff: f32,
    pub softness: f32,
    pub exponent: f32,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [u8; 16],
    pub info: *mut mjString,
    pub spec: mjsLight,
}

const _: () = assert!(std::mem::size_of::<mjCLight>() == 688);

const _: () = assert!(std::mem::align_of::<mjCLight>() == 8);

/// C struct: mjCMaterial (472 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCMaterial {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub texid: [i32; 10],
    pub textures_: std__vector_std__string,
    pub spec_textures_: std__vector_std__string,
    pub element: *mut mjsElement,
    pub textures: *mut mjStringVec,
    pub texuniform: bool,
    pub _pad_1: [u8; 3],
    pub texrepeat: [f32; 2],
    pub emission: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectance: f32,
    pub metallic: f32,
    pub roughness: f32,
    pub rgba: [u8; 20],
    pub info: *mut mjString,
    pub spec: mjsMaterial,
}

const _: () = assert!(std::mem::size_of::<mjCMaterial>() == 472);

const _: () = assert!(std::mem::align_of::<mjCMaterial>() == 8);

const _: () = assert!(std::mem::align_of::<mjCMesh>() == 8);

const _: () = assert!(std::mem::align_of::<mjCModel>() == 8);

const _: () = assert!(std::mem::align_of::<mjCOctree>() == 8);

const _: () = assert!(std::mem::align_of::<mjCPair>() == 8);

const _: () = assert!(std::mem::align_of::<mjCSite>() == 8);

/// C struct: mjCTendon (960 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCTendon {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub matid: i32,
    pub _pad_1: [u8; 4],
    pub material_: std__string,
    pub spec_material_: std__string,
    pub userdata_: std__vector_double,
    pub spec_userdata_: std__vector_double,
    pub element: *mut mjsElement,
    pub stiffness: [f64; 3],
    pub springlength: [f64; 2],
    pub damping: [f64; 3],
    pub frictionloss: f64,
    pub solref_friction: [f64; 2],
    pub solimp_friction: [f64; 5],
    pub armature: f64,
    pub limited: u32,
    pub actfrclimited: u32,
    pub range: [f64; 2],
    pub actfrcrange: [f64; 2],
    pub margin: f64,
    pub solref_limit: [f64; 2],
    pub solimp_limit: [f64; 5],
    pub material: *mut mjString,
    pub width: f64,
    pub rgba: [f32; 4],
    pub group: i32,
    pub _pad_2: [u8; 4],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
    pub spec: mjsTendon,
    pub path: std__vector_mjCWrap,
}

const _: () = assert!(std::mem::size_of::<mjCTendon>() == 960);

const _: () = assert!(std::mem::align_of::<mjCTendon>() == 8);

/// C struct: mjLROpt (72 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjLROpt {
    pub mode: i32,
    pub useexisting: i32,
    pub uselimit: i32,
    pub _pad_0: [u8; 4],
    pub accel: f64,
    pub maxforce: f64,
    pub timeconst: f64,
    pub timestep: f64,
    pub inttotal: f64,
    pub interval: f64,
    pub tolrange: f64,
}

const _: () = assert!(std::mem::size_of::<mjLROpt>() == 72);

const _: () = assert!(std::mem::align_of::<mjLROpt>() == 8);

const _: () = assert!(std::mem::size_of::<mjOption>() == 304);

const _: () = assert!(std::mem::align_of::<mjOption>() == 8);

const _: () = assert!(std::mem::size_of::<mjResource>() == 552);

const _: () = assert!(std::mem::align_of::<mjResource>() == 8);

const _: () = assert!(std::mem::size_of::<mjSpec>() == 1344);

const _: () = assert!(std::mem::align_of::<mjSpec>() == 8);

const _: () = assert!(std::mem::size_of::<mjStatistic>() == 56);

const _: () = assert!(std::mem::align_of::<mjStatistic>() == 8);

/// C struct: mjVFS (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjVFS {
    pub impl_: *mut (),
}

const _: () = assert!(std::mem::size_of::<mjVFS>() == 8);

const _: () = assert!(std::mem::align_of::<mjVFS>() == 8);

const _: () = assert!(std::mem::size_of::<mjVisual>() == 632);

const _: () = assert!(std::mem::align_of::<mjVisual>() == 4);

/// C struct: mjsActuator (608 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsActuator {
    pub element: *mut mjsElement,
    pub gaintype: u32,
    pub _pad_0: [u8; 4],
    pub gainprm: [f64; 10],
    pub biastype: u32,
    pub _pad_1: [u8; 4],
    pub biasprm: [f64; 10],
    pub dyntype: u32,
    pub _pad_2: [u8; 4],
    pub dynprm: [f64; 10],
    pub actdim: i32,
    pub ctrlspec: i32,
    pub velrange: [f64; 2],
    pub ffrange: [f64; 2],
    pub actearly: bool,
    pub _pad_3: [u8; 3],
    pub trntype: u32,
    pub gear: [f64; 6],
    pub target: *mut mjString,
    pub refsite: *mut mjString,
    pub slidersite: *mut mjString,
    pub cranklength: f64,
    pub lengthrange: [f64; 2],
    pub inheritrange: f64,
    pub damping: [f64; 3],
    pub armature: f64,
    pub ctrllimited: u32,
    pub _pad_4: [u8; 4],
    pub ctrlrange: [f64; 2],
    pub forcelimited: u32,
    pub _pad_5: [u8; 4],
    pub forcerange: [f64; 2],
    pub actlimited: u32,
    pub _pad_6: [u8; 4],
    pub actrange: [f64; 2],
    pub group: i32,
    pub nsample: i32,
    pub interp: i32,
    pub _pad_7: [u8; 4],
    pub delay: f64,
    pub userdata: *mut mjDoubleVec,
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjsActuator>() == 608);

const _: () = assert!(std::mem::align_of::<mjsActuator>() == 8);

const _: () = assert!(std::mem::size_of::<mjsAuthored>() == 72);

const _: () = assert!(std::mem::align_of::<mjsAuthored>() == 8);

const _: () = assert!(std::mem::size_of::<mjsBody>() == 568);

const _: () = assert!(std::mem::align_of::<mjsBody>() == 8);

const _: () = assert!(std::mem::align_of::<mjsCamera>() == 8);

const _: () = assert!(std::mem::size_of::<mjsCompiler>() == 160);

const _: () = assert!(std::mem::align_of::<mjsCompiler>() == 8);

/// C struct: mjsDefault (104 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsDefault {
    pub element: *mut mjsElement,
    pub joint: *mut mjsJoint,
    pub geom: *mut mjsGeom,
    pub site: *mut mjsSite,
    pub camera: *mut mjsCamera,
    pub light: *mut mjsLight,
    pub flex: *mut mjsFlex,
    pub mesh: *mut mjsMesh,
    pub material: *mut mjsMaterial,
    pub pair: *mut mjsPair,
    pub equality: *mut mjsEquality,
    pub tendon: *mut mjsTendon,
    pub actuator: *mut mjsActuator,
}

const _: () = assert!(std::mem::size_of::<mjsDefault>() == 104);

const _: () = assert!(std::mem::align_of::<mjsDefault>() == 8);

const _: () = assert!(std::mem::align_of::<mjsElement>() == 8);

/// C struct: mjsEquality (200 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsEquality {
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub _pad_0: [u8; 4],
    pub data: [f64; 11],
    pub active: bool,
    pub _pad_1: [u8; 7],
    pub name1: *mut mjString,
    pub name2: *mut mjString,
    pub objtype: u32,
    pub _pad_2: [u8; 4],
    pub solref: [f64; 2],
    pub solimp: [f64; 5],
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjsEquality>() == 200);

const _: () = assert!(std::mem::align_of::<mjsEquality>() == 8);

/// C struct: mjsFlex (352 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsFlex {
    pub element: *mut mjsElement,
    pub contype: i32,
    pub conaffinity: i32,
    pub condim: i32,
    pub priority: i32,
    pub friction: [f64; 3],
    pub solmix: f64,
    pub solref: [f64; 2],
    pub solimp: [f64; 5],
    pub margin: f64,
    pub gap: f64,
    pub dim: i32,
    pub _pad_0: [u8; 4],
    pub radius: f64,
    pub size: [f64; 3],
    pub internal: bool,
    pub flatskin: bool,
    pub _pad_1: [u8; 2],
    pub selfcollide: u32,
    pub passive: i32,
    pub activelayers: i32,
    pub group: i32,
    pub _pad_2: [u8; 4],
    pub edgestiffness: f64,
    pub edgedamping: f64,
    pub rgba: [f32; 4],
    pub material: *mut mjString,
    pub young: f64,
    pub poisson: f64,
    pub damping: f64,
    pub thickness: f64,
    pub elastic2d: i32,
    pub cellcount: [i32; 3],
    pub order: i32,
    pub _pad_3: [u8; 4],
    pub nodebody: *mut mjStringVec,
    pub vertbody: *mut mjStringVec,
    pub node: *mut mjDoubleVec,
    pub vert: *mut mjDoubleVec,
    pub elem: *mut mjIntVec,
    pub texcoord: *mut mjFloatVec,
    pub elemtexcoord: *mut mjIntVec,
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjsFlex>() == 352);

const _: () = assert!(std::mem::align_of::<mjsFlex>() == 8);

const _: () = assert!(std::mem::size_of::<mjsFrame>() == 216);

const _: () = assert!(std::mem::align_of::<mjsFrame>() == 8);

const _: () = assert!(std::mem::size_of::<mjsGeom>() == 640);

const _: () = assert!(std::mem::align_of::<mjsGeom>() == 8);

const _: () = assert!(std::mem::size_of::<mjsHField>() == 80);

const _: () = assert!(std::mem::align_of::<mjsHField>() == 8);

/// C struct: mjsJoint (360 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsJoint {
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub _pad_0: [u8; 4],
    pub pos: [f64; 3],
    pub axis: [f64; 3],
    pub r#ref: f64,
    pub align: u32,
    pub _pad_1: [u8; 4],
    pub stiffness: [f64; 3],
    pub springref: f64,
    pub springdamper: [f64; 2],
    pub limited: u32,
    pub _pad_2: [u8; 4],
    pub range: [f64; 2],
    pub margin: f64,
    pub solref_limit: [f64; 2],
    pub solimp_limit: [f64; 5],
    pub actfrclimited: u32,
    pub _pad_3: [u8; 4],
    pub actfrcrange: [f64; 2],
    pub armature: f64,
    pub damping: [f64; 3],
    pub frictionloss: f64,
    pub solref_friction: [f64; 2],
    pub solimp_friction: [f64; 5],
    pub group: i32,
    pub actgravcomp: bool,
    pub _pad_4: [u8; 3],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjsJoint>() == 360);

const _: () = assert!(std::mem::align_of::<mjsJoint>() == 8);

/// C struct: mjsLight (176 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsLight {
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub dir: [f64; 3],
    pub mode: u32,
    pub _pad_0: [u8; 4],
    pub targetbody: *mut mjString,
    pub active: bool,
    pub _pad_1: [u8; 3],
    pub r#type: u32,
    pub texture: *mut mjString,
    pub castshadow: bool,
    pub _pad_2: [u8; 3],
    pub bulbradius: f32,
    pub intensity: f32,
    pub range: f32,
    pub attenuation: [f32; 3],
    pub cutoff: f32,
    pub softness: f32,
    pub exponent: f32,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [u8; 16],
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjsLight>() == 176);

const _: () = assert!(std::mem::align_of::<mjsLight>() == 8);

/// C struct: mjsMaterial (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsMaterial {
    pub element: *mut mjsElement,
    pub textures: *mut mjStringVec,
    pub texuniform: bool,
    pub _pad_0: [u8; 3],
    pub texrepeat: [f32; 2],
    pub emission: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectance: f32,
    pub metallic: f32,
    pub roughness: f32,
    pub rgba: [u8; 20],
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjsMaterial>() == 80);

const _: () = assert!(std::mem::align_of::<mjsMaterial>() == 8);

const _: () = assert!(std::mem::size_of::<mjsMesh>() == 232);

const _: () = assert!(std::mem::align_of::<mjsMesh>() == 8);

const _: () = assert!(std::mem::size_of::<mjsOrientation>() == 136);

const _: () = assert!(std::mem::align_of::<mjsOrientation>() == 8);

const _: () = assert!(std::mem::size_of::<mjsPair>() == 176);

const _: () = assert!(std::mem::align_of::<mjsPair>() == 8);

const _: () = assert!(std::mem::size_of::<mjsPlugin>() == 40);

const _: () = assert!(std::mem::align_of::<mjsPlugin>() == 8);

const _: () = assert!(std::mem::size_of::<mjsSite>() == 336);

const _: () = assert!(std::mem::align_of::<mjsSite>() == 8);

/// C struct: mjsTendon (304 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsTendon {
    pub element: *mut mjsElement,
    pub stiffness: [f64; 3],
    pub springlength: [f64; 2],
    pub damping: [f64; 3],
    pub frictionloss: f64,
    pub solref_friction: [f64; 2],
    pub solimp_friction: [f64; 5],
    pub armature: f64,
    pub limited: u32,
    pub actfrclimited: u32,
    pub range: [f64; 2],
    pub actfrcrange: [f64; 2],
    pub margin: f64,
    pub solref_limit: [f64; 2],
    pub solimp_limit: [f64; 5],
    pub material: *mut mjString,
    pub width: f64,
    pub rgba: [f32; 4],
    pub group: i32,
    pub _pad_0: [u8; 4],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjsTendon>() == 304);

const _: () = assert!(std::mem::align_of::<mjsTendon>() == 8);

/// C struct: struct mjpResourceProvider (72 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct struct_mjpResourceProvider {
    pub prefix: *const i8,
    pub open: mjfOpenResource,
    pub read: mjfReadResource,
    pub close: mjfCloseResource,
    pub mount: mjfMountResource,
    pub unmount: mjfUnmountResource,
    pub modified: mjfResourceModified,
    pub write: mjfWriteResource,
    pub data: *mut (),
}

const _: () = assert!(std::mem::size_of::<struct_mjpResourceProvider>() == 72);

const _: () = assert!(std::mem::align_of::<struct_mjpResourceProvider>() == 8);

const _: () = assert!(std::mem::size_of::<std__vector_std__vector_float>() == 24);

const _: () = assert!(std::mem::align_of::<std__vector_std__vector_float>() == 8);

const _: () = assert!(std::mem::align_of::<mjCSkin>() == 8);

const _: () = assert!(std::mem::size_of::<mjsSkin>() == 120);

const _: () = assert!(std::mem::align_of::<mjsSkin>() == 8);

const _: () = assert!(std::mem::size_of::<DynArray_XMLNode____10>() == 104);

const _: () = assert!(std::mem::align_of::<DynArray_XMLNode____10>() == 8);

const _: () = assert!(std::mem::size_of::<MemPoolT_sizeof_XMLAttribute>() == 152);

const _: () = assert!(std::mem::align_of::<MemPoolT_sizeof_XMLAttribute>() == 8);

const _: () = assert!(std::mem::size_of::<MemPoolT_sizeof_XMLComment>() == 152);

const _: () = assert!(std::mem::align_of::<MemPoolT_sizeof_XMLComment>() == 8);

const _: () = assert!(std::mem::size_of::<MemPoolT_sizeof_XMLElement>() == 152);

const _: () = assert!(std::mem::align_of::<MemPoolT_sizeof_XMLElement>() == 8);

const _: () = assert!(std::mem::size_of::<MemPoolT_sizeof_XMLText>() == 152);

const _: () = assert!(std::mem::align_of::<MemPoolT_sizeof_XMLText>() == 8);

const _: () = assert!(std::mem::align_of::<tinyxml2__MemPool>() == 8);

const _: () = assert!(std::mem::align_of::<tinyxml2__StrPair>() == 8);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLConstHandle>() == 8);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLDocument>() == 8);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLNode>() == 8);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLText>() == 8);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLComment>() == 8);

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLUtil>() == 1);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLUtil>() == 1);

const _: () = assert!(std::mem::align_of::<mjCSensor>() == 8);

const _: () = assert!(std::mem::size_of::<mjsSensor>() == 168);

const _: () = assert!(std::mem::align_of::<mjsSensor>() == 8);

/// C struct: ccd_vec3_t (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct ccd_vec3_t {
    pub v: [f64; 3],
}

const _: () = assert!(std::mem::size_of::<ccd_vec3_t>() == 24);

const _: () = assert!(std::mem::align_of::<ccd_vec3_t>() == 8);

/// C struct: mjCBodyPair (400 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCBodyPair {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub body1: i32,
    pub body2: i32,
    pub signature: i32,
    pub _pad_1: [u8; 4],
    pub bodyname1_: std__string,
    pub bodyname2_: std__string,
    pub spec_bodyname1_: std__string,
    pub spec_bodyname2_: std__string,
    pub element: *mut mjsElement,
    pub bodyname1: *mut mjString,
    pub bodyname2: *mut mjString,
    pub info: *mut mjString,
    pub spec: mjsExclude,
}

const _: () = assert!(std::mem::size_of::<mjCBodyPair>() == 400);

const _: () = assert!(std::mem::align_of::<mjCBodyPair>() == 8);

/// C struct: mjsExclude (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsExclude {
    pub element: *mut mjsElement,
    pub bodyname1: *mut mjString,
    pub bodyname2: *mut mjString,
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjsExclude>() == 32);

const _: () = assert!(std::mem::align_of::<mjsExclude>() == 8);

const _: () = assert!(std::mem::size_of::<mjFloatVec>() == 24);

const _: () = assert!(std::mem::align_of::<mjFloatVec>() == 8);

const _: () = assert!(std::mem::size_of::<mjFloatVecVec>() == 24);

const _: () = assert!(std::mem::align_of::<mjFloatVecVec>() == 8);

const _: () = assert!(std::mem::size_of::<mjIntVec>() == 24);

const _: () = assert!(std::mem::align_of::<mjIntVec>() == 8);

const _: () = assert!(std::mem::size_of::<mjIntVecVec>() == 24);

const _: () = assert!(std::mem::align_of::<mjIntVecVec>() == 8);

const _: () = assert!(std::mem::size_of::<mjStringVec>() == 24);

const _: () = assert!(std::mem::align_of::<mjStringVec>() == 8);

const _: () = assert!(std::mem::size_of::<mjDoubleVec>() == 24);

const _: () = assert!(std::mem::align_of::<mjDoubleVec>() == 8);

/// C struct: mujoco::user::FilePath (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mujoco__user__FilePath {
    pub path_: std__string,
}

const _: () = assert!(std::mem::size_of::<mujoco__user__FilePath>() == 24);

const _: () = assert!(std::mem::align_of::<mujoco__user__FilePath>() == 8);

/// C struct: mjCBase::UserValue (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCBase__UserValue {
    pub value: *const (),
    pub cleanup: Option<unsafe extern "C" fn(*const ())>,
}

const _: () = assert!(std::mem::size_of::<mjCBase__UserValue>() == 16);

const _: () = assert!(std::mem::align_of::<mjCBase__UserValue>() == 8);

/// C struct: mjModel (5696 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjModel {
    pub nq: i64,
    pub nv: i64,
    pub nu: i64,
    pub nactuator: i64,
    pub nout: i64,
    pub na: i64,
    pub nbody: i64,
    pub nbvh: i64,
    pub nbvhstatic: i64,
    pub nbvhdynamic: i64,
    pub noct: i64,
    pub njnt: i64,
    pub ntree: i64,
    pub nM: i64,
    pub nB: i64,
    pub nC: i64,
    pub nD: i64,
    pub ngeom: i64,
    pub nsite: i64,
    pub ncam: i64,
    pub nlight: i64,
    pub nflex: i64,
    pub nflexnode: i64,
    pub nflexvert: i64,
    pub nflexedge: i64,
    pub nflexelem: i64,
    pub nflexelemdata: i64,
    pub nflexstiffness: i64,
    pub nflexbending: i64,
    pub nefm0dof: i64,
    pub nefm0L: i64,
    pub nflexelemedge: i64,
    pub nflexshelldata: i64,
    pub nflexevpair: i64,
    pub nflextexcoord: i64,
    pub nJfe: i64,
    pub nJfv: i64,
    pub nmesh: i64,
    pub nmeshvert: i64,
    pub nmeshnormal: i64,
    pub nmeshtexcoord: i64,
    pub nmeshface: i64,
    pub nmeshgraph: i64,
    pub nmeshpoly: i64,
    pub nmeshpolyvert: i64,
    pub nmeshpolymap: i64,
    pub nskin: i64,
    pub nskinvert: i64,
    pub nskintexvert: i64,
    pub nskinface: i64,
    pub nskinbone: i64,
    pub nskinbonevert: i64,
    pub nhfield: i64,
    pub nhfielddata: i64,
    pub ntex: i64,
    pub ntexdata: i64,
    pub nmat: i64,
    pub npair: i64,
    pub nexclude: i64,
    pub neq: i64,
    pub ntendon: i64,
    pub nJten: i64,
    pub nwrap: i64,
    pub nsensor: i64,
    pub nnumeric: i64,
    pub nnumericdata: i64,
    pub ntext: i64,
    pub ntextdata: i64,
    pub ntuple: i64,
    pub ntupledata: i64,
    pub nkey: i64,
    pub nmocap: i64,
    pub nplugin: i64,
    pub npluginattr: i64,
    pub nuser_body: i64,
    pub nuser_jnt: i64,
    pub nuser_geom: i64,
    pub nuser_site: i64,
    pub nuser_cam: i64,
    pub nuser_tendon: i64,
    pub nuser_actuator: i64,
    pub nuser_sensor: i64,
    pub nnames: i64,
    pub npaths: i64,
    pub nnames_map: i64,
    pub nJmom: i64,
    pub ngravcomp: i64,
    pub nemax: i64,
    pub njmax: i64,
    pub nconmax: i64,
    pub npolygonmax: i64,
    pub nmeshdegmax: i64,
    pub nuserdata: i64,
    pub nsensordata: i64,
    pub npluginstate: i64,
    pub nhistory: i64,
    pub narena: i64,
    pub nbuffer: i64,
    pub flg_gravcomp: bool,
    pub flg_surfacevel: bool,
    pub flg_adhesion: bool,
    pub _pad_0: [u8; 5],
    pub opt: mjOption,
    pub vis: mjVisual,
    pub stat: mjStatistic,
    pub buffer: *mut (),
    pub qpos0: *mut f64,
    pub qpos_spring: *mut f64,
    pub body_parentid: *mut i32,
    pub body_rootid: *mut i32,
    pub body_weldid: *mut i32,
    pub body_mocapid: *mut i32,
    pub body_jntnum: *mut i32,
    pub body_jntadr: *mut i32,
    pub body_dofnum: *mut i32,
    pub body_dofadr: *mut i32,
    pub body_treeid: *mut i32,
    pub body_geomnum: *mut i32,
    pub body_geomadr: *mut i32,
    pub body_simple: *mut u8,
    pub body_sameframe: *mut u8,
    pub body_pos: *mut f64,
    pub body_quat: *mut f64,
    pub body_ipos: *mut f64,
    pub body_iquat: *mut f64,
    pub body_mass: *mut f64,
    pub body_subtreemass: *mut f64,
    pub body_inertia: *mut f64,
    pub body_invweight0: *mut f64,
    pub body_gravcomp: *mut f64,
    pub body_margin: *mut f64,
    pub body_user: *mut f64,
    pub body_plugin: *mut i32,
    pub body_contype: *mut i32,
    pub body_conaffinity: *mut i32,
    pub body_bvhadr: *mut i32,
    pub body_bvhnum: *mut i32,
    pub bvh_depth: *mut i32,
    pub bvh_child: *mut i32,
    pub bvh_nodeid: *mut i32,
    pub bvh_aabb: *mut f64,
    pub oct_depth: *mut i32,
    pub oct_child: *mut i32,
    pub oct_aabb: *mut f64,
    pub oct_coeff: *mut f64,
    pub jnt_type: *mut i32,
    pub jnt_qposadr: *mut i32,
    pub jnt_dofadr: *mut i32,
    pub jnt_bodyid: *mut i32,
    pub jnt_actuatorid: *mut i32,
    pub jnt_group: *mut i32,
    pub jnt_limited: *mut bool,
    pub jnt_actfrclimited: *mut bool,
    pub jnt_actgravcomp: *mut bool,
    pub jnt_solref: *mut f64,
    pub jnt_solimp: *mut f64,
    pub jnt_pos: *mut f64,
    pub jnt_axis: *mut f64,
    pub jnt_stiffness: *mut f64,
    pub jnt_stiffnesspoly: *mut f64,
    pub jnt_range: *mut f64,
    pub jnt_actfrcrange: *mut f64,
    pub jnt_margin: *mut f64,
    pub jnt_user: *mut f64,
    pub dof_bodyid: *mut i32,
    pub dof_jntid: *mut i32,
    pub dof_parentid: *mut i32,
    pub dof_treeid: *mut i32,
    pub dof_Madr: *mut i32,
    pub dof_simplenum: *mut i32,
    pub dof_solref: *mut f64,
    pub dof_solimp: *mut f64,
    pub dof_frictionloss: *mut f64,
    pub dof_armature: *mut f64,
    pub dof_damping: *mut f64,
    pub dof_dampingpoly: *mut f64,
    pub dof_invweight0: *mut f64,
    pub dof_M0: *mut f64,
    pub dof_length: *mut f64,
    pub tree_bodyadr: *mut i32,
    pub tree_bodynum: *mut i32,
    pub tree_dofadr: *mut i32,
    pub tree_dofnum: *mut i32,
    pub tree_sleep_policy: *mut i32,
    pub geom_type: *mut i32,
    pub geom_contype: *mut i32,
    pub geom_conaffinity: *mut i32,
    pub geom_condim: *mut i32,
    pub geom_bodyid: *mut i32,
    pub geom_dataid: *mut i32,
    pub geom_matid: *mut i32,
    pub geom_group: *mut i32,
    pub geom_priority: *mut i32,
    pub geom_plugin: *mut i32,
    pub geom_sameframe: *mut u8,
    pub geom_solmix: *mut f64,
    pub geom_solref: *mut f64,
    pub geom_solimp: *mut f64,
    pub geom_size: *mut f64,
    pub geom_aabb: *mut f64,
    pub geom_rbound: *mut f64,
    pub geom_pos: *mut f64,
    pub geom_quat: *mut f64,
    pub geom_friction: *mut f64,
    pub geom_margin: *mut f64,
    pub geom_gap: *mut f64,
    pub geom_surfacevel: *mut f64,
    pub geom_adhesion: *mut f64,
    pub geom_fluid: *mut f64,
    pub geom_user: *mut f64,
    pub geom_rgba: *mut f32,
    pub site_type: *mut i32,
    pub site_bodyid: *mut i32,
    pub site_dataid: *mut i32,
    pub site_matid: *mut i32,
    pub site_group: *mut i32,
    pub site_sameframe: *mut u8,
    pub site_size: *mut f64,
    pub site_pos: *mut f64,
    pub site_quat: *mut f64,
    pub site_user: *mut f64,
    pub site_rgba: *mut f32,
    pub cam_mode: *mut i32,
    pub cam_bodyid: *mut i32,
    pub cam_targetbodyid: *mut i32,
    pub cam_pos: *mut f64,
    pub cam_quat: *mut f64,
    pub cam_poscom0: *mut f64,
    pub cam_pos0: *mut f64,
    pub cam_mat0: *mut f64,
    pub cam_projection: *mut i32,
    pub cam_fovy: *mut f64,
    pub cam_ipd: *mut f64,
    pub cam_resolution: *mut i32,
    pub cam_output: *mut i32,
    pub cam_sensorsize: *mut f32,
    pub cam_intrinsic: *mut f32,
    pub cam_user: *mut f64,
    pub light_mode: *mut i32,
    pub light_bodyid: *mut i32,
    pub light_targetbodyid: *mut i32,
    pub light_type: *mut i32,
    pub light_texid: *mut i32,
    pub light_castshadow: *mut bool,
    pub light_bulbradius: *mut f32,
    pub light_intensity: *mut f32,
    pub light_range: *mut f32,
    pub light_active: *mut bool,
    pub light_pos: *mut f64,
    pub light_dir: *mut f64,
    pub light_poscom0: *mut f64,
    pub light_pos0: *mut f64,
    pub light_dir0: *mut f64,
    pub light_attenuation: *mut f32,
    pub light_cutoff: *mut f32,
    pub light_softness: *mut f32,
    pub light_exponent: *mut f32,
    pub light_ambient: *mut f32,
    pub light_diffuse: *mut f32,
    pub light_specular: *mut f32,
    pub flex_contype: *mut i32,
    pub flex_conaffinity: *mut i32,
    pub flex_condim: *mut i32,
    pub flex_priority: *mut i32,
    pub flex_solmix: *mut f64,
    pub flex_solref: *mut f64,
    pub flex_solimp: *mut f64,
    pub flex_friction: *mut f64,
    pub flex_margin: *mut f64,
    pub flex_gap: *mut f64,
    pub flex_internal: *mut bool,
    pub flex_selfcollide: *mut i32,
    pub flex_activelayers: *mut i32,
    pub flex_passive: *mut i32,
    pub flex_dim: *mut i32,
    pub flex_matid: *mut i32,
    pub flex_group: *mut i32,
    pub flex_interp: *mut i32,
    pub flex_cellnum: *mut i32,
    pub flex_nodeadr: *mut i32,
    pub flex_nodenum: *mut i32,
    pub flex_vertadr: *mut i32,
    pub flex_vertnum: *mut i32,
    pub flex_edgeadr: *mut i32,
    pub flex_edgenum: *mut i32,
    pub flex_elemadr: *mut i32,
    pub flex_elemnum: *mut i32,
    pub flex_elemdataadr: *mut i32,
    pub flex_stiffnessadr: *mut i32,
    pub flex_elemedgeadr: *mut i32,
    pub flex_bendingadr: *mut i32,
    pub flex_shellnum: *mut i32,
    pub flex_shelldataadr: *mut i32,
    pub flex_evpairadr: *mut i32,
    pub flex_evpairnum: *mut i32,
    pub flex_texcoordadr: *mut i32,
    pub flex_nodebodyid: *mut i32,
    pub flex_vertbodyid: *mut i32,
    pub flex_vertedgeadr: *mut i32,
    pub flex_vertedgenum: *mut i32,
    pub flex_vertedge: *mut i32,
    pub flex_edge: *mut i32,
    pub flex_edgeflap: *mut i32,
    pub flex_elem: *mut i32,
    pub flex_elemtexcoord: *mut i32,
    pub flex_elemedge: *mut i32,
    pub flex_elemlayer: *mut i32,
    pub flex_shell: *mut i32,
    pub flex_evpair: *mut i32,
    pub flex_vert: *mut f64,
    pub flex_vert0: *mut f64,
    pub flex_vertmetric: *mut f64,
    pub flex_node: *mut f64,
    pub flex_node0: *mut f64,
    pub flexedge_length0: *mut f64,
    pub flexedge_invweight0: *mut f64,
    pub flex_radius: *mut f64,
    pub flex_size: *mut f64,
    pub flex_stiffness: *mut f64,
    pub flex_bending: *mut f64,
    pub efm0_dofid: *mut i32,
    pub efm0_L_rownnz: *mut i32,
    pub efm0_L_rowadr: *mut i32,
    pub efm0_L_colind: *mut i32,
    pub efm0_L: *mut f64,
    pub flex_damping: *mut f64,
    pub flex_edgestiffness: *mut f64,
    pub flex_edgedamping: *mut f64,
    pub flex_edgeequality: *mut i32,
    pub flex_rigid: *mut bool,
    pub flexedge_rigid: *mut bool,
    pub flex_centered: *mut bool,
    pub flex_flatskin: *mut bool,
    pub flex_bvhadr: *mut i32,
    pub flex_bvhnum: *mut i32,
    pub flexedge_J_rownnz: *mut i32,
    pub flexedge_J_rowadr: *mut i32,
    pub flexedge_J_colind: *mut i32,
    pub flexvert_J_rownnz: *mut i32,
    pub flexvert_J_rowadr: *mut i32,
    pub flexvert_J_colind: *mut i32,
    pub flex_rgba: *mut f32,
    pub flex_texcoord: *mut f32,
    pub mesh_vertadr: *mut i32,
    pub mesh_vertnum: *mut i32,
    pub mesh_faceadr: *mut i32,
    pub mesh_facenum: *mut i32,
    pub mesh_bvhadr: *mut i32,
    pub mesh_bvhnum: *mut i32,
    pub mesh_octadr: *mut i32,
    pub mesh_octnum: *mut i32,
    pub mesh_normaladr: *mut i32,
    pub mesh_normalnum: *mut i32,
    pub mesh_texcoordadr: *mut i32,
    pub mesh_texcoordnum: *mut i32,
    pub mesh_graphadr: *mut i32,
    pub mesh_extrema: *mut i32,
    pub mesh_vert: *mut f32,
    pub mesh_normal: *mut f32,
    pub mesh_texcoord: *mut f32,
    pub mesh_face: *mut i32,
    pub mesh_facenormal: *mut i32,
    pub mesh_facetexcoord: *mut i32,
    pub mesh_graph: *mut i32,
    pub mesh_scale: *mut f64,
    pub mesh_pos: *mut f64,
    pub mesh_quat: *mut f64,
    pub mesh_pathadr: *mut i32,
    pub mesh_polynum: *mut i32,
    pub mesh_polyadr: *mut i32,
    pub mesh_polynormal: *mut f64,
    pub mesh_polyvertadr: *mut i32,
    pub mesh_polyvertnum: *mut i32,
    pub mesh_polyvert: *mut i32,
    pub mesh_polymapadr: *mut i32,
    pub mesh_polymapnum: *mut i32,
    pub mesh_polymap: *mut i32,
    pub skin_matid: *mut i32,
    pub skin_group: *mut i32,
    pub skin_rgba: *mut f32,
    pub skin_inflate: *mut f32,
    pub skin_vertadr: *mut i32,
    pub skin_vertnum: *mut i32,
    pub skin_texcoordadr: *mut i32,
    pub skin_faceadr: *mut i32,
    pub skin_facenum: *mut i32,
    pub skin_boneadr: *mut i32,
    pub skin_bonenum: *mut i32,
    pub skin_vert: *mut f32,
    pub skin_texcoord: *mut f32,
    pub skin_face: *mut i32,
    pub skin_bonevertadr: *mut i32,
    pub skin_bonevertnum: *mut i32,
    pub skin_bonebindpos: *mut f32,
    pub skin_bonebindquat: *mut f32,
    pub skin_bonebodyid: *mut i32,
    pub skin_bonevertid: *mut i32,
    pub skin_bonevertweight: *mut f32,
    pub skin_pathadr: *mut i32,
    pub hfield_size: *mut f64,
    pub hfield_nrow: *mut i32,
    pub hfield_ncol: *mut i32,
    pub hfield_adr: *mut i32,
    pub hfield_data: *mut f32,
    pub hfield_pathadr: *mut i32,
    pub tex_type: *mut i32,
    pub tex_colorspace: *mut i32,
    pub tex_height: *mut i32,
    pub tex_width: *mut i32,
    pub tex_nchannel: *mut i32,
    pub tex_adr: *mut i64,
    pub tex_data: *mut u8,
    pub tex_pathadr: *mut i32,
    pub mat_texid: *mut i32,
    pub mat_texuniform: *mut bool,
    pub mat_texrepeat: *mut f32,
    pub mat_emission: *mut f32,
    pub mat_specular: *mut f32,
    pub mat_shininess: *mut f32,
    pub mat_reflectance: *mut f32,
    pub mat_metallic: *mut f32,
    pub mat_roughness: *mut f32,
    pub mat_rgba: *mut f32,
    pub pair_dim: *mut i32,
    pub pair_geom1: *mut i32,
    pub pair_geom2: *mut i32,
    pub pair_signature: *mut i32,
    pub pair_solref: *mut f64,
    pub pair_solreffriction: *mut f64,
    pub pair_solimp: *mut f64,
    pub pair_margin: *mut f64,
    pub pair_gap: *mut f64,
    pub pair_adhesion: *mut f64,
    pub pair_friction: *mut f64,
    pub exclude_signature: *mut i32,
    pub eq_type: *mut i32,
    pub eq_obj1id: *mut i32,
    pub eq_obj2id: *mut i32,
    pub eq_objtype: *mut i32,
    pub eq_active0: *mut bool,
    pub eq_solref: *mut f64,
    pub eq_solimp: *mut f64,
    pub eq_data: *mut f64,
    pub tendon_adr: *mut i32,
    pub tendon_num: *mut i32,
    pub tendon_matid: *mut i32,
    pub tendon_actuatorid: *mut i32,
    pub tendon_group: *mut i32,
    pub tendon_treenum: *mut i32,
    pub tendon_treeid: *mut i32,
    pub ten_J_rownnz: *mut i32,
    pub ten_J_rowadr: *mut i32,
    pub ten_J_colind: *mut i32,
    pub tendon_limited: *mut bool,
    pub tendon_actfrclimited: *mut bool,
    pub tendon_width: *mut f64,
    pub tendon_solref_lim: *mut f64,
    pub tendon_solimp_lim: *mut f64,
    pub tendon_solref_fri: *mut f64,
    pub tendon_solimp_fri: *mut f64,
    pub tendon_range: *mut f64,
    pub tendon_actfrcrange: *mut f64,
    pub tendon_margin: *mut f64,
    pub tendon_stiffness: *mut f64,
    pub tendon_stiffnesspoly: *mut f64,
    pub tendon_damping: *mut f64,
    pub tendon_dampingpoly: *mut f64,
    pub tendon_armature: *mut f64,
    pub tendon_frictionloss: *mut f64,
    pub tendon_lengthspring: *mut f64,
    pub tendon_length0: *mut f64,
    pub tendon_invweight0: *mut f64,
    pub tendon_user: *mut f64,
    pub tendon_rgba: *mut f32,
    pub wrap_type: *mut i32,
    pub wrap_objid: *mut i32,
    pub wrap_prm: *mut f64,
    pub actuator_trntype: *mut i32,
    pub actuator_dyntype: *mut i32,
    pub actuator_gaintype: *mut i32,
    pub actuator_biastype: *mut i32,
    pub actuator_ctrladr: *mut i32,
    pub actuator_ctrlnum: *mut i32,
    pub actuator_ctrlspec: *mut i32,
    pub actuator_outadr: *mut i32,
    pub actuator_outnum: *mut i32,
    pub actuator_actadr: *mut i32,
    pub actuator_actnum: *mut i32,
    pub actuator_trnid: *mut i32,
    pub actuator_cranklength: *mut f64,
    pub actuator_dynprm: *mut f64,
    pub actuator_gainprm: *mut f64,
    pub actuator_biasprm: *mut f64,
    pub actuator_actlimited: *mut bool,
    pub actuator_actrange: *mut f64,
    pub actuator_actearly: *mut bool,
    pub actuator_history: *mut i32,
    pub actuator_historyadr: *mut i32,
    pub actuator_delay: *mut f64,
    pub actuator_damping: *mut f64,
    pub actuator_dampingpoly: *mut f64,
    pub actuator_armature: *mut f64,
    pub actuator_group: *mut i32,
    pub actuator_user: *mut f64,
    pub actuator_plugin: *mut i32,
    pub actuator_forcelimited: *mut bool,
    pub actuator_forcerange: *mut f64,
    pub actuator_ctrllimited: *mut bool,
    pub actuator_ctrlrange: *mut f64,
    pub actuator_gear: *mut f64,
    pub actuator_acc0: *mut f64,
    pub actuator_length0: *mut f64,
    pub actuator_lengthrange: *mut f64,
    pub sensor_type: *mut i32,
    pub sensor_datatype: *mut i32,
    pub sensor_needstage: *mut i32,
    pub sensor_objtype: *mut i32,
    pub sensor_objid: *mut i32,
    pub sensor_reftype: *mut i32,
    pub sensor_refid: *mut i32,
    pub sensor_intprm: *mut i32,
    pub sensor_dim: *mut i32,
    pub sensor_adr: *mut i32,
    pub sensor_cutoff: *mut f64,
    pub sensor_noise: *mut f64,
    pub sensor_history: *mut i32,
    pub sensor_historyadr: *mut i32,
    pub sensor_delay: *mut f64,
    pub sensor_interval: *mut f64,
    pub sensor_user: *mut f64,
    pub sensor_plugin: *mut i32,
    pub plugin: *mut i32,
    pub plugin_stateadr: *mut i32,
    pub plugin_statenum: *mut i32,
    pub plugin_attr: *mut i8,
    pub plugin_attradr: *mut i32,
    pub numeric_adr: *mut i32,
    pub numeric_size: *mut i32,
    pub numeric_data: *mut f64,
    pub text_adr: *mut i32,
    pub text_size: *mut i32,
    pub text_data: *mut i8,
    pub tuple_adr: *mut i32,
    pub tuple_size: *mut i32,
    pub tuple_objtype: *mut i32,
    pub tuple_objid: *mut i32,
    pub tuple_objprm: *mut f64,
    pub key_time: *mut f64,
    pub key_qpos: *mut f64,
    pub key_qvel: *mut f64,
    pub key_act: *mut f64,
    pub key_mpos: *mut f64,
    pub key_mquat: *mut f64,
    pub key_ctrl: *mut f64,
    pub name_bodyadr: *mut i32,
    pub name_jntadr: *mut i32,
    pub name_geomadr: *mut i32,
    pub name_siteadr: *mut i32,
    pub name_camadr: *mut i32,
    pub name_lightadr: *mut i32,
    pub name_flexadr: *mut i32,
    pub name_meshadr: *mut i32,
    pub name_skinadr: *mut i32,
    pub name_hfieldadr: *mut i32,
    pub name_texadr: *mut i32,
    pub name_matadr: *mut i32,
    pub name_pairadr: *mut i32,
    pub name_excludeadr: *mut i32,
    pub name_eqadr: *mut i32,
    pub name_tendonadr: *mut i32,
    pub name_actuatoradr: *mut i32,
    pub name_sensoradr: *mut i32,
    pub name_numericadr: *mut i32,
    pub name_textadr: *mut i32,
    pub name_tupleadr: *mut i32,
    pub name_keyadr: *mut i32,
    pub name_pluginadr: *mut i32,
    pub names: *mut i8,
    pub names_map: *mut i32,
    pub paths: *mut i8,
    pub B_rownnz: *mut i32,
    pub B_rowadr: *mut i32,
    pub B_colind: *mut i32,
    pub M_rownnz: *mut i32,
    pub M_rowadr: *mut i32,
    pub M_colind: *mut i32,
    pub mapM2M: *mut i32,
    pub D_rownnz: *mut i32,
    pub D_rowadr: *mut i32,
    pub D_diag: *mut i32,
    pub D_colind: *mut i32,
    pub mapM2D: *mut i32,
    pub mapD2M: *mut i32,
    pub signature: u64,
}

const _: () = assert!(std::mem::size_of::<mjModel>() == 5696);

const _: () = assert!(std::mem::align_of::<mjModel>() == 8);

/// Sized opaque C struct: tinyxml2__XMLUnknown (104 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLUnknown { pub _data: [u8; 104] }

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLUnknown>() == 104);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLUnknown>() == 8);

/// C struct: tinyxml2::XMLHandle (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLHandle {
    pub _node: *mut tinyxml2__XMLNode,
}

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLHandle>() == 8);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLHandle>() == 8);

/// C struct: mjCWrap (328 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCWrap {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub sideid: i32,
    pub _pad_1: [u8; 4],
    pub prm: f64,
    pub sidesite: std__string,
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub _pad_2: [u8; 4],
    pub info: *mut mjString,
    pub spec: mjsWrap,
    pub obj: *mut mjCBase,
    pub tendon: *mut mjCTendon,
}

/// C struct: mjsWrap (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsWrap {
    pub element: *mut mjsElement,
    pub r#type: u32,
    pub _pad_0: [u8; 4],
    pub info: *mut mjString,
}

const _: () = assert!(std::mem::size_of::<mjCWrap>() == 328);

const _: () = assert!(std::mem::align_of::<mjCWrap>() == 8);

const _: () = assert!(std::mem::size_of::<mjsWrap>() == 24);

const _: () = assert!(std::mem::align_of::<mjsWrap>() == 8);

/// Sized opaque C struct: tinyxml2__XMLPrinter (328 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLPrinter { pub _data: [u8; 328] }

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLPrinter>() == 328);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLPrinter>() == 8);

/// Sized opaque C struct: tinyxml2__XMLVisitor (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLVisitor { pub _data: [u8; 8] }

const _: () = assert!(std::mem::size_of::<tinyxml2__XMLVisitor>() == 8);

const _: () = assert!(std::mem::align_of::<tinyxml2__XMLVisitor>() == 8);

/// Sized opaque C struct: mjCAsset (112 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCAsset { pub _data: [u8; 112] }

const _: () = assert!(std::mem::size_of::<mjCAsset>() == 112);

const _: () = assert!(std::mem::align_of::<mjCAsset>() == 8);

/// C struct: Polytope (104 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Polytope {
    pub verts: *mut Vertex,
    pub nverts: i32,
    pub _pad_0: [u8; 4],
    pub faces: *mut Face,
    pub nfaces: i32,
    pub maxfaces: i32,
    pub center: [f64; 3],
    pub map: *mut *mut Face,
    pub nmap: i32,
    pub _pad_1: [u8; 4],
    pub horizon: struct_Horizon,
}

/// C struct: mjCCDStatus (3184 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCCDStatus {
    pub separated: i32,
    pub _pad_0: [u8; 4],
    pub dist: [f64; 50],
    pub x1: [f64; 150],
    pub x2: [f64; 150],
    pub nx: i32,
    pub max_iterations: i32,
    pub tolerance: f64,
    pub max_contacts: i32,
    pub _pad_1: [u8; 4],
    pub dist_cutoff: f64,
    pub gjk_iterations: i32,
    pub epa_iterations: i32,
    pub epa_status: mjEPAStatus,
    pub _pad_2: [u8; 4],
    pub simplex: [Vertex; 4],
    pub nsimplex: i32,
    pub _pad_3: [u8; 4],
}

const _: () = assert!(std::mem::size_of::<Polytope>() == 104);

const _: () = assert!(std::mem::align_of::<Polytope>() == 8);

/// Sized opaque C struct: mjEPAStatus (4 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjEPAStatus { pub _data: [u8; 4] }

/// C struct: Face (56 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Face {
    pub verts: i32,
    pub adj: [i32; 3],
    pub v: [f64; 3],
    pub dist2: f64,
    pub index: i32,
    pub _pad_0: [u8; 4],
}

/// C struct: Vertex (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub vert: [f64; 3],
    pub vert1: [f64; 3],
    pub vert2: [f64; 3],
    pub index1: i32,
    pub index2: i32,
}

/// C struct: struct Horizon (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct struct_Horizon {
    pub indices: *mut i32,
    pub edges: *mut i32,
    pub nedges: i32,
    pub _pad_0: [u8; 4],
    pub w: *const f64,
}

const _: () = assert!(std::mem::size_of::<Face>() == 56);

const _: () = assert!(std::mem::align_of::<Face>() == 8);

const _: () = assert!(std::mem::size_of::<Vertex>() == 80);

const _: () = assert!(std::mem::align_of::<Vertex>() == 8);

const _: () = assert!(std::mem::size_of::<mjCCDStatus>() == 3184);

const _: () = assert!(std::mem::align_of::<mjCCDStatus>() == 8);

const _: () = assert!(std::mem::size_of::<mjEPAStatus>() == 4);

const _: () = assert!(std::mem::align_of::<mjEPAStatus>() == 4);

const _: () = assert!(std::mem::size_of::<struct_Horizon>() == 32);

const _: () = assert!(std::mem::align_of::<struct_Horizon>() == 8);

/// C struct: mjvGeom (252 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvGeom {
    pub r#type: i32,
    pub dataid: i32,
    pub objtype: i32,
    pub objid: i32,
    pub category: i32,
    pub matid: i32,
    pub texid: i32,
    pub texuniform: i32,
    pub texcoord: i32,
    pub segid: i32,
    pub size: [f32; 3],
    pub pos: [f32; 3],
    pub mat: [f32; 9],
    pub rgba: [f32; 4],
    pub emission: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectance: f32,
    pub texrepeat: [f32; 2],
    pub label: [i8; 100],
    pub camdist: f32,
    pub modelrbound: f32,
    pub transparent: u8,
    pub _pad_0: [u8; 3],
}

/// C struct: mjvScene (11584 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjvScene {
    pub maxgeom: i32,
    pub ngeom: i32,
    pub geoms: *mut mjvGeom,
    pub geomorder: *mut i32,
    pub nflex: i32,
    pub _pad_0: [u8; 4],
    pub flexedgeadr: *mut i32,
    pub flexedgenum: *mut i32,
    pub flexvertadr: *mut i32,
    pub flexvertnum: *mut i32,
    pub flexfaceadr: *mut i32,
    pub flexfacenum: *mut i32,
    pub flexfaceused: *mut i32,
    pub flexedge: *mut i32,
    pub flexvert: *mut f32,
    pub flexface: *mut f32,
    pub flexnormal: *mut f32,
    pub flextexcoord: *mut f32,
    pub flexvertopt: u8,
    pub flexedgeopt: u8,
    pub flexfaceopt: u8,
    pub flexskinopt: u8,
    pub nskin: i32,
    pub skinfacenum: *mut i32,
    pub skinvertadr: *mut i32,
    pub skinvertnum: *mut i32,
    pub skinvert: *mut f32,
    pub skinnormal: *mut f32,
    pub nlight: i32,
    pub lights: [u8; 11200],
    pub camera: [u8; 128],
    pub enabletransform: u8,
    pub _pad_1: [u8; 3],
    pub translate: [f32; 3],
    pub rotate: [f32; 4],
    pub scale: f32,
    pub stereo: i32,
    pub flags: [u8; 12],
    pub framewidth: i32,
    pub framergb: [f32; 3],
    pub status: i32,
    pub _pad_2: [u8; 4],
}

const _: () = assert!(std::mem::size_of::<mjvGeom>() == 252);

const _: () = assert!(std::mem::align_of::<mjvGeom>() == 4);

const _: () = assert!(std::mem::size_of::<mjvScene>() == 11584);

const _: () = assert!(std::mem::align_of::<mjvScene>() == 8);

const _: () = assert!(std::mem::size_of::<mjString>() == 24);

const _: () = assert!(std::mem::align_of::<mjString>() == 8);

/// Sized opaque C struct: std__array_double__3 (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__array_double__3 { pub _data: [u8; 24] }

/// Sized opaque C struct: std__array_double__4 (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__array_double__4 { pub _data: [u8; 32] }

/// Sized opaque C struct: std__array_double__6 (48 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__array_double__6 { pub _data: [u8; 48] }

/// C struct: mjCBoundingVolume (128 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCBoundingVolume {
    pub contype_: i32,
    pub conaffinity_: i32,
    pub aabb_: std__array_double__6,
    pub pos_: std__array_double__3,
    pub quat_: std__array_double__4,
    pub quat_set_: bool,
    pub _pad_0: [u8; 3],
    pub idval_: i32,
    pub id_: *const i32,
}

const _: () = assert!(std::mem::size_of::<mjCBoundingVolume>() == 128);

const _: () = assert!(std::mem::align_of::<mjCBoundingVolume>() == 8);

const _: () = assert!(std::mem::size_of::<std__array_double__3>() == 24);

const _: () = assert!(std::mem::align_of::<std__array_double__3>() == 8);

const _: () = assert!(std::mem::size_of::<std__array_double__4>() == 32);

const _: () = assert!(std::mem::align_of::<std__array_double__4>() == 8);

const _: () = assert!(std::mem::size_of::<std__array_double__6>() == 48);

const _: () = assert!(std::mem::align_of::<std__array_double__6>() == 8);
