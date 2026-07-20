//! C types used in function signatures.
//! CODEGEN LOCKED — DO NOT EDIT

#![allow(non_camel_case_types, non_snake_case)]

/// Opaque C struct: Args (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Args { _opaque: [u8; 0] }

/// Opaque C struct: BVHLeafCallback (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BVHLeafCallback { _opaque: [u8; 0] }

/// Opaque C struct: ContactInfo (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContactInfo { _opaque: [u8; 0] }

/// Opaque C struct: DynArray_char__20 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DynArray_char__20 { _opaque: [u8; 0] }

/// Opaque C struct: DynArray_const_char____10_Ref (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DynArray_const_char____10_Ref { _opaque: [u8; 0] }

/// Opaque C struct: Face (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Face { _opaque: [u8; 0] }

/// Opaque C struct: GLADloadproc (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLADloadproc { _opaque: [u8; 0] }

/// Opaque C struct: Matrix (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Matrix { _opaque: [u8; 0] }

/// Opaque C struct: MeshSDFContext (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MeshSDFContext { _opaque: [u8; 0] }

/// Opaque C struct: Mutex (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mutex { _opaque: [u8; 0] }

/// Sized opaque C struct: Polytope (104 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Polytope { pub _data: [u8; 104] }

/// Opaque C struct: T (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct T { _opaque: [u8; 0] }

/// Opaque C struct: T1 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct T1 { _opaque: [u8; 0] }

/// Opaque C struct: T2 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct T2 { _opaque: [u8; 0] }

/// Opaque C struct: TableBlock_mjpPlugin (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TableBlock_mjpPlugin { _opaque: [u8; 0] }

/// Opaque C struct: Triangle (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Triangle { _opaque: [u8; 0] }

/// Opaque C struct: Vertex (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex { _opaque: [u8; 0] }

/// Opaque C struct: XMLElement_Ref (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XMLElement_Ref { _opaque: [u8; 0] }

/// Opaque C struct: anonymous_namespace___FilePath (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_namespace___FilePath { _opaque: [u8; 0] }

/// Opaque C struct: anonymous_namespace___GlobalModel (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_namespace___GlobalModel { _opaque: [u8; 0] }

/// Opaque C struct: anonymous_namespace___PNGImage (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_namespace___PNGImage { _opaque: [u8; 0] }

/// Opaque C struct: anonymous_namespace___RewriteMap (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_namespace___RewriteMap { _opaque: [u8; 0] }

/// Opaque C struct: anonymous_namespace___XMLDocument (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_namespace___XMLDocument { _opaque: [u8; 0] }

/// Opaque C struct: anonymous_namespace___XMLElement (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_namespace___XMLElement { _opaque: [u8; 0] }

/// Opaque C struct: anonymous_namespace___string (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct anonymous_namespace___string { _opaque: [u8; 0] }

/// Opaque C struct: bool_1 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bool_1 { _opaque: [u8; 0] }

/// Opaque C struct: bool_64 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bool_64 { _opaque: [u8; 0] }

/// Opaque C struct: ccd_real_t (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ccd_real_t { _opaque: [u8; 0] }

/// Opaque C struct: char__const (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct char__const { _opaque: [u8; 0] }

/// Opaque C struct: double_3 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct double_3 { _opaque: [u8; 0] }

/// Opaque C struct: double_4 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct double_4 { _opaque: [u8; 0] }

/// Opaque C struct: float_4 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct float_4 { _opaque: [u8; 0] }

/// Opaque C struct: int_3 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct int_3 { _opaque: [u8; 0] }

/// Sized opaque C struct: libcpp_mutex_t (64 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct libcpp_mutex_t { pub _data: [u8; 64] }

/// Opaque C struct: locale_t (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct locale_t { _opaque: [u8; 0] }

/// Opaque C struct: mjByteVec (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjByteVec { _opaque: [u8; 0] }

/// Opaque C struct: mjCCDConfig (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCCDConfig { _opaque: [u8; 0] }

/// Opaque C struct: mjCCDStatus (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCCDStatus { _opaque: [u8; 0] }

/// Opaque C struct: mjCDataFunc (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCDataFunc { _opaque: [u8; 0] }

/// Opaque C struct: mjCDef_1 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCDef_1 { _opaque: [u8; 0] }

/// Opaque C struct: mjDoubleVec (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjDoubleVec { _opaque: [u8; 0] }

/// Opaque C struct: mjFloatVec (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjFloatVec { _opaque: [u8; 0] }

/// Opaque C struct: mjFloatVecVec (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjFloatVecVec { _opaque: [u8; 0] }

/// Opaque C struct: mjIntVec (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjIntVec { _opaque: [u8; 0] }

/// Opaque C struct: mjIntVecVec (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjIntVecVec { _opaque: [u8; 0] }

/// Opaque C struct: mjKeyMap (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjKeyMap { _opaque: [u8; 0] }

/// Sized opaque C struct: mjListKeyMap (624 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjListKeyMap { pub _data: [u8; 624] }

/// Sized opaque C struct: mjPrimalContext (576 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjPrimalContext { pub _data: [u8; 576] }

/// Opaque C struct: mjSolverStat (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjSolverStat { _opaque: [u8; 0] }

/// Opaque C struct: mjString (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjString { _opaque: [u8; 0] }

/// Opaque C struct: mjStringVec (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjStringVec { _opaque: [u8; 0] }

/// Opaque C struct: mjTaskFunc (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjTaskFunc { _opaque: [u8; 0] }

/// Opaque C struct: mjTimerStat (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjTimerStat { _opaque: [u8; 0] }

/// Opaque C struct: mjTreeIter (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjTreeIter { _opaque: [u8; 0] }

/// Opaque C struct: mjWarningStat (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjWarningStat { _opaque: [u8; 0] }

/// Opaque C struct: mjcPair (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjcPair { _opaque: [u8; 0] }

/// Sized opaque C struct: mjfCanDecode (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfCanDecode { pub _data: [u8; 8] }

/// Sized opaque C struct: mjfCloseResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfCloseResource { pub _data: [u8; 8] }

/// Sized opaque C struct: mjfDecode (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfDecode { pub _data: [u8; 8] }

/// Sized opaque C struct: mjfEncode (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfEncode { pub _data: [u8; 8] }

/// Sized opaque C struct: mjfItemEnable (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfItemEnable { pub _data: [u8; 8] }

/// Sized opaque C struct: mjfLogHandler (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfLogHandler { pub _data: [u8; 8] }

/// Sized opaque C struct: mjfMountResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfMountResource { pub _data: [u8; 8] }

/// Sized opaque C struct: mjfOpenResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfOpenResource { pub _data: [u8; 8] }

/// Opaque C struct: mjfPluginLibraryLoadCallback (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjfPluginLibraryLoadCallback { _opaque: [u8; 0] }

/// Sized opaque C struct: mjfReadResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfReadResource { pub _data: [u8; 8] }

/// Sized opaque C struct: mjfResourceModified (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfResourceModified { pub _data: [u8; 8] }

/// Sized opaque C struct: mjfUnmountResource (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjfUnmountResource { pub _data: [u8; 8] }

/// Sized opaque C struct: mjsAuthored (72 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsAuthored { pub _data: [u8; 72] }

/// Sized opaque C struct: mjsExclude (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsExclude { pub _data: [u8; 32] }

/// Opaque C struct: mjtCompShape_3 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjtCompShape_3 { _opaque: [u8; 0] }

/// Opaque C struct: mjtCompType (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjtCompType { _opaque: [u8; 0] }

/// Opaque C struct: mjtDof (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjtDof { _opaque: [u8; 0] }

/// Opaque C struct: mjtFcompType (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjtFcompType { _opaque: [u8; 0] }

/// Sized opaque C struct: mjuiItem____anonymous_union_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjui_h_256_3 (1408 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiItem____anonymous_union_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjui_h_256_3 { pub _data: [u8; 1408] }

/// Sized opaque C struct: mjuiThemeColor (336 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiThemeColor { pub _data: [u8; 336] }

/// Sized opaque C struct: mjuiThemeSpacing (52 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiThemeSpacing { pub _data: [u8; 52] }

/// Opaque C struct: mujoco__GlobalTable__ErrorMessage (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mujoco__GlobalTable__ErrorMessage { _opaque: [u8; 0] }

/// Opaque C struct: mujoco__Mutex (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mujoco__Mutex { _opaque: [u8; 0] }

/// Opaque C struct: mujoco___anonymous_namespace___Resolver (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mujoco___anonymous_namespace___Resolver { _opaque: [u8; 0] }

/// Opaque C struct: mujoco__user__VFS__ResourcePtr (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mujoco__user__VFS__ResourcePtr { _opaque: [u8; 0] }

/// Opaque C struct: mz_zip_archive (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mz_zip_archive { _opaque: [u8; 0] }

/// Opaque C struct: pcg32_state (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pcg32_state { _opaque: [u8; 0] }

/// Opaque C struct: std__atomic_int (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__atomic_int { _opaque: [u8; 0] }

/// Opaque C struct: std__condition_variable (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__condition_variable { _opaque: [u8; 0] }

/// Opaque C struct: std__map_std__string__std__vector_mjsMesh_Ref (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__map_std__string__std__vector_mjsMesh_Ref { _opaque: [u8; 0] }

/// Opaque C struct: std__queue_std__function_void (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__queue_std__function_void { _opaque: [u8; 0] }

/// Sized opaque C struct: std__string (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__string { pub _data: [u8; 24] }

/// Opaque C struct: std__string_view (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__string_view { _opaque: [u8; 0] }

/// Opaque C struct: std__stringstream (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__stringstream { _opaque: [u8; 0] }

/// Opaque C struct: std__unordered_map_mjtCompKind__std__vector_mjCDef (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__unordered_map_mjtCompKind__std__vector_mjCDef { _opaque: [u8; 0] }

/// Opaque C struct: std__unordered_map_std__string__FileInfo (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__unordered_map_std__string__FileInfo { _opaque: [u8; 0] }

/// Opaque C struct: std__unordered_set_std__string (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__unordered_set_std__string { _opaque: [u8; 0] }

/// Opaque C struct: std__vector_mjRGBA (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__vector_mjRGBA { _opaque: [u8; 0] }

/// Opaque C struct: std__vector_std__thread (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__vector_std__thread { _opaque: [u8; 0] }

/// Opaque C struct: std__vector_std__uint8_t (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__vector_std__uint8_t { _opaque: [u8; 0] }

/// Sized opaque C struct: string_type (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct string_type { pub _data: [u8; 24] }

/// Opaque C struct: struct___sFILEX (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct struct___sFILEX { _opaque: [u8; 0] }

/// Sized opaque C struct: struct___sbuf (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct struct___sbuf { pub _data: [u8; 16] }

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_132_3 (52 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_132_3 { pub _data: [u8; 52] }

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_148_3 (20 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_148_3 { pub _data: [u8; 20] }

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_156_3 (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_156_3 { pub _data: [u8; 40] }

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_163_3 (52 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_163_3 { pub _data: [u8; 52] }

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_179_3 (68 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_179_3 { pub _data: [u8; 68] }

/// Sized opaque C struct: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_199_3 (400 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_199_3 { pub _data: [u8; 400] }

/// Opaque C struct: struct_mjpResourceProvider (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct struct_mjpResourceProvider { _opaque: [u8; 0] }

/// Opaque C struct: type_parameter_0_0 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct type_parameter_0_0 { _opaque: [u8; 0] }

/// Sized opaque C struct: union__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_src_engine_engine_collision_convex_h_52_3 (160 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct union__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_src_engine_engine_collision_convex_h_52_3 { pub _data: [u8; 160] }

/// Opaque C struct: unsigned_char_64 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct unsigned_char_64 { _opaque: [u8; 0] }

/// Opaque C struct: void_____T____const_char (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct void_____T____const_char { _opaque: [u8; 0] }

/// Opaque C struct: xlocale (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xlocale { _opaque: [u8; 0] }

/// C enum: LodePNGColorType
pub type LodePNGColorType = u32;
pub const LodePNGColorType_LCT_GREY: LodePNGColorType = 0;
pub const LodePNGColorType_LCT_RGB: LodePNGColorType = 2;
pub const LodePNGColorType_LCT_PALETTE: LodePNGColorType = 3;
pub const LodePNGColorType_LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LodePNGColorType_LCT_RGBA: LodePNGColorType = 6;
pub const LodePNGColorType_LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;

/// C enum: mjtBias
pub type mjtBias = u32;
pub const mjtBias_mjBIAS_NONE: mjtBias = 0;
pub const mjtBias_mjBIAS_AFFINE: mjtBias = 1;
pub const mjtBias_mjBIAS_MUSCLE: mjtBias = 2;
pub const mjtBias_mjBIAS_DCMOTOR: mjtBias = 3;
pub const mjtBias_mjBIAS_USER: mjtBias = 4;

/// C enum: mjtCamLight
pub type mjtCamLight = u32;
pub const mjtCamLight_mjCAMLIGHT_FIXED: mjtCamLight = 0;
pub const mjtCamLight_mjCAMLIGHT_TRACK: mjtCamLight = 1;
pub const mjtCamLight_mjCAMLIGHT_TRACKCOM: mjtCamLight = 2;
pub const mjtCamLight_mjCAMLIGHT_TARGETBODY: mjtCamLight = 3;
pub const mjtCamLight_mjCAMLIGHT_TARGETBODYCOM: mjtCamLight = 4;

/// C enum: mjtColorSpace
pub type mjtColorSpace = u32;
pub const mjtColorSpace_mjCOLORSPACE_AUTO: mjtColorSpace = 0;
pub const mjtColorSpace_mjCOLORSPACE_LINEAR: mjtColorSpace = 1;
pub const mjtColorSpace_mjCOLORSPACE_SRGB: mjtColorSpace = 2;

/// C enum: mjtConflict
pub type mjtConflict = u32;
pub const mjtConflict_mjCONFLICT_WARNING: mjtConflict = 0;
pub const mjtConflict_mjCONFLICT_MERGE: mjtConflict = 1;
pub const mjtConflict_mjCONFLICT_ERROR: mjtConflict = 2;

/// C enum: mjtDataType
pub type mjtDataType = u32;
pub const mjtDataType_mjDATATYPE_REAL: mjtDataType = 0;
pub const mjtDataType_mjDATATYPE_POSITIVE: mjtDataType = 1;
pub const mjtDataType_mjDATATYPE_AXIS: mjtDataType = 2;
pub const mjtDataType_mjDATATYPE_QUATERNION: mjtDataType = 3;

/// C enum: mjtDyn
pub type mjtDyn = u32;
pub const mjtDyn_mjDYN_NONE: mjtDyn = 0;
pub const mjtDyn_mjDYN_INTEGRATOR: mjtDyn = 1;
pub const mjtDyn_mjDYN_FILTER: mjtDyn = 2;
pub const mjtDyn_mjDYN_FILTEREXACT: mjtDyn = 3;
pub const mjtDyn_mjDYN_MUSCLE: mjtDyn = 4;
pub const mjtDyn_mjDYN_DCMOTOR: mjtDyn = 5;
pub const mjtDyn_mjDYN_USER: mjtDyn = 6;

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

/// C enum: mjtGain
pub type mjtGain = u32;
pub const mjtGain_mjGAIN_FIXED: mjtGain = 0;
pub const mjtGain_mjGAIN_AFFINE: mjtGain = 1;
pub const mjtGain_mjGAIN_MUSCLE: mjtGain = 2;
pub const mjtGain_mjGAIN_DCMOTOR: mjtGain = 3;
pub const mjtGain_mjGAIN_USER: mjtGain = 4;

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

/// C enum: mjtMeshBuiltin
pub type mjtMeshBuiltin = u32;
pub const mjtMeshBuiltin_mjMESH_BUILTIN_NONE: mjtMeshBuiltin = 0;
pub const mjtMeshBuiltin_mjMESH_BUILTIN_SPHERE: mjtMeshBuiltin = 1;
pub const mjtMeshBuiltin_mjMESH_BUILTIN_HEMISPHERE: mjtMeshBuiltin = 2;
pub const mjtMeshBuiltin_mjMESH_BUILTIN_CONE: mjtMeshBuiltin = 3;
pub const mjtMeshBuiltin_mjMESH_BUILTIN_SUPERSPHERE: mjtMeshBuiltin = 4;
pub const mjtMeshBuiltin_mjMESH_BUILTIN_SUPERTORUS: mjtMeshBuiltin = 5;
pub const mjtMeshBuiltin_mjMESH_BUILTIN_WEDGE: mjtMeshBuiltin = 6;
pub const mjtMeshBuiltin_mjMESH_BUILTIN_PLATE: mjtMeshBuiltin = 7;

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

/// C enum: mjtSDFType
pub type mjtSDFType = u32;
pub const mjtSDFType_mjSDFTYPE_SINGLE: mjtSDFType = 0;
pub const mjtSDFType_mjSDFTYPE_INTERSECTION: mjtSDFType = 1;
pub const mjtSDFType_mjSDFTYPE_MIDSURFACE: mjtSDFType = 2;
pub const mjtSDFType_mjSDFTYPE_COLLISION: mjtSDFType = 3;

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

/// C enum: mjtSleepPolicy
pub type mjtSleepPolicy = u32;
pub const mjtSleepPolicy_mjSLEEP_AUTO: mjtSleepPolicy = 0;
pub const mjtSleepPolicy_mjSLEEP_AUTO_NEVER: mjtSleepPolicy = 1;
pub const mjtSleepPolicy_mjSLEEP_AUTO_ALLOWED: mjtSleepPolicy = 2;
pub const mjtSleepPolicy_mjSLEEP_NEVER: mjtSleepPolicy = 3;
pub const mjtSleepPolicy_mjSLEEP_ALLOWED: mjtSleepPolicy = 4;
pub const mjtSleepPolicy_mjSLEEP_INIT: mjtSleepPolicy = 5;

/// C enum: mjtSleepState
pub type mjtSleepState = i32;
pub const mjtSleepState_mjS_STATIC: mjtSleepState = -1;
pub const mjtSleepState_mjS_ASLEEP: mjtSleepState = 0;
pub const mjtSleepState_mjS_AWAKE: mjtSleepState = 1;

/// C enum: mjtStage
pub type mjtStage = u32;
pub const mjtStage_mjSTAGE_NONE: mjtStage = 0;
pub const mjtStage_mjSTAGE_POS: mjtStage = 1;
pub const mjtStage_mjSTAGE_VEL: mjtStage = 2;
pub const mjtStage_mjSTAGE_ACC: mjtStage = 3;

/// C enum: mjtState
pub type mjtState = u32;
pub const mjtState_mjSTATE_TIME: mjtState = 1;
pub const mjtState_mjSTATE_QPOS: mjtState = 2;
pub const mjtState_mjSTATE_QVEL: mjtState = 4;
pub const mjtState_mjSTATE_ACT: mjtState = 8;
pub const mjtState_mjSTATE_HISTORY: mjtState = 16;
pub const mjtState_mjSTATE_WARMSTART: mjtState = 32;
pub const mjtState_mjSTATE_CTRL: mjtState = 64;
pub const mjtState_mjSTATE_QFRC_APPLIED: mjtState = 128;
pub const mjtState_mjSTATE_XFRC_APPLIED: mjtState = 256;
pub const mjtState_mjSTATE_EQ_ACTIVE: mjtState = 512;
pub const mjtState_mjSTATE_MOCAP_POS: mjtState = 1024;
pub const mjtState_mjSTATE_MOCAP_QUAT: mjtState = 2048;
pub const mjtState_mjSTATE_USERDATA: mjtState = 4096;
pub const mjtState_mjSTATE_PLUGIN: mjtState = 8192;
pub const mjtState_mjNSTATE: mjtState = 14;
pub const mjtState_mjSTATE_PHYSICS: mjtState = 30;
pub const mjtState_mjSTATE_FULLPHYSICS: mjtState = 8223;
pub const mjtState_mjSTATE_USER: mjtState = 8128;
pub const mjtState_mjSTATE_INTEGRATION: mjtState = 16383;

/// C enum: mjtTexture
pub type mjtTexture = u32;
pub const mjtTexture_mjTEXTURE_2D: mjtTexture = 0;
pub const mjtTexture_mjTEXTURE_CUBE: mjtTexture = 1;
pub const mjtTexture_mjTEXTURE_SKYBOX: mjtTexture = 2;

/// C enum: mjtTrn
pub type mjtTrn = u32;
pub const mjtTrn_mjTRN_JOINT: mjtTrn = 0;
pub const mjtTrn_mjTRN_JOINTINPARENT: mjtTrn = 1;
pub const mjtTrn_mjTRN_SLIDERCRANK: mjtTrn = 2;
pub const mjtTrn_mjTRN_TENDON: mjtTrn = 3;
pub const mjtTrn_mjTRN_SITE: mjtTrn = 4;
pub const mjtTrn_mjTRN_BODY: mjtTrn = 5;
pub const mjtTrn_mjTRN_UNDEFINED: mjtTrn = 1000;

/// C enum: mjtWrap
pub type mjtWrap = u32;
pub const mjtWrap_mjWRAP_NONE: mjtWrap = 0;
pub const mjtWrap_mjWRAP_JOINT: mjtWrap = 1;
pub const mjtWrap_mjWRAP_PULLEY: mjtWrap = 2;
pub const mjtWrap_mjWRAP_SITE: mjtWrap = 3;
pub const mjtWrap_mjWRAP_SPHERE: mjtWrap = 4;
pub const mjtWrap_mjWRAP_CYLINDER: mjtWrap = 5;

/// C enum: mujoco::user::VFS::Status
pub type mujoco_user_VFS_Status = i32;
pub const mujoco_user_VFS_Status_kSuccess: mujoco_user_VFS_Status = 0;
pub const mujoco_user_VFS_Status_kFailedToLoad: mujoco_user_VFS_Status = -1;
pub const mujoco_user_VFS_Status_kFailedToRead: mujoco_user_VFS_Status = -1;
pub const mujoco_user_VFS_Status_kNotFound: mujoco_user_VFS_Status = -1;
pub const mujoco_user_VFS_Status_kRepeatedName: mujoco_user_VFS_Status = 2;
pub const mujoco_user_VFS_Status_kInvalidVfs: mujoco_user_VFS_Status = -1;
pub const mujoco_user_VFS_Status_kInvalidResource: mujoco_user_VFS_Status = -1;
pub const mujoco_user_VFS_Status_kInvalidResourceProvider: mujoco_user_VFS_Status = -1;

/// C enum: std::byte
pub type std_byte = u8;

/// C enum: tinyxml2::Whitespace
pub type tinyxml2_Whitespace = u32;
pub const tinyxml2_Whitespace_PRESERVE_WHITESPACE: tinyxml2_Whitespace = 0;
pub const tinyxml2_Whitespace_COLLAPSE_WHITESPACE: tinyxml2_Whitespace = 1;
pub const tinyxml2_Whitespace_PEDANTIC_WHITESPACE: tinyxml2_Whitespace = 2;

/// C enum: tinyxml2::XMLElement::ElementClosingType
pub type tinyxml2_XMLElement_ElementClosingType = u32;
pub const tinyxml2_XMLElement_ElementClosingType_OPEN: tinyxml2_XMLElement_ElementClosingType = 0;
pub const tinyxml2_XMLElement_ElementClosingType_CLOSED: tinyxml2_XMLElement_ElementClosingType = 1;
pub const tinyxml2_XMLElement_ElementClosingType_CLOSING: tinyxml2_XMLElement_ElementClosingType = 2;

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

/// C struct: BufferProvider (120 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct BufferProvider {
    pub prefix: *const i8,
    pub open: mjfOpenResource,
    pub read: mjfReadResource,
    pub close: mjfCloseResource,
    pub mount: mjfMountResource,
    pub unmount: mjfUnmountResource,
    pub modified: mjfResourceModified,
    pub data: *mut (),
    pub path_: mujoco__user__FilePath,
    pub contents_: [u8; 24],
    pub hash_: u64,
}
const _: () = assert!(std::mem::size_of::<BufferProvider>() == 120);

/// C struct: FILE (152 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct FILE {
    pub _p: *mut u8,
    pub _r: i32,
    pub _w: i32,
    pub _flags: i16,
    pub _file: i16,
    pub _pad_0: [u8; 4],
    pub _bf: struct___sbuf,
    pub _lbfsize: i32,
    pub _pad_1: [u8; 4],
    pub _cookie: *mut (),
    pub _close: Option<unsafe extern "C" fn()>,
    pub _read: Option<unsafe extern "C" fn()>,
    pub _seek: Option<unsafe extern "C" fn()>,
    pub _write: Option<unsafe extern "C" fn()>,
    pub _ub: struct___sbuf,
    pub _extra: *mut struct___sFILEX,
    pub _ur: i32,
    pub _ubuf: [u8; 3],
    pub _nbuf: [u8; 1],
    pub _lb: struct___sbuf,
    pub _blksize: i32,
    pub _pad_2: [u8; 4],
    pub _offset: i64,
}
const _: () = assert!(std::mem::size_of::<FILE>() == 152);

/// C struct: GlobalTable (2560 bytes, align 256)
#[repr(C, align(256))]
#[derive(Clone, Copy)]
pub struct GlobalTable {
    pub first_block_: [u8; 2304],
    pub count_: [u8; 8],
    pub mutex_: [u8; 248],
}
const _: () = assert!(std::mem::size_of::<GlobalTable>() == 2560);

/// C struct: LocaleOverride (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct LocaleOverride {
    pub old_locale_: [u8; 8],
}
const _: () = assert!(std::mem::size_of::<LocaleOverride>() == 8);

/// C struct: tinyxml2::MemPool (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct MemPool {
    pub _vtable: *mut (),
}
const _: () = assert!(std::mem::size_of::<MemPool>() == 8);

/// C struct: MeshPolygon (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct MeshPolygon {
    pub edges_: *const (),
    pub _pad_0: [u8; 16],
    pub islands_: *const (),
    pub _pad_1: [u8; 16],
    pub nisland_: i32,
    pub _pad_2: [u8; 4],
    pub normal_: [u8; 24],
}
const _: () = assert!(std::mem::size_of::<MeshPolygon>() == 80);

/// C struct: OctNode (192 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct OctNode {
    pub level: i32,
    pub parent_index: i32,
    pub child_slot: i32,
    pub child: [u8; 32],
    pub vertid: [u8; 36],
    pub aamm: *const (),
    pub _pad_0: [u8; 40],
    pub coeff: *const (),
    pub _pad_1: [u8; 56],
}
const _: () = assert!(std::mem::size_of::<OctNode>() == 192);

/// C struct: OctreeTask (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct OctreeTask {
    pub elements: *const (),
    pub _pad_0: [u8; 16],
    pub lev: i32,
    pub parent_index: i32,
    pub child_slot: i32,
    pub node_index: i32,
}
const _: () = assert!(std::mem::size_of::<OctreeTask>() == 40);

/// C struct: Reader (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Reader {
    pub xml_node_: [u8; 8],
    pub elem_: *const (),
    pub text_: std__string,
}
const _: () = assert!(std::mem::size_of::<Reader>() == 40);

/// C struct: tinyxml2::StrPair (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct StrPair {
    pub _flags: i32,
    pub _pad_0: [u8; 4],
    pub _start: *mut i8,
    pub _end: *mut i8,
}
const _: () = assert!(std::mem::size_of::<StrPair>() == 24);

/// C struct: ThreadPool (240 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct ThreadPool {
    pub threads_: [u8; 24],
    pub m_: std__mutex,
    pub cv_in_: [u8; 48],
    pub cv_ext_: [u8; 48],
    pub queue_: [u8; 48],
    pub ctr_: u64,
}
const _: () = assert!(std::mem::size_of::<ThreadPool>() == 240);

/// C struct: ThreadPoolContext (128 bytes, align 64)
#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct ThreadPoolContext {
    pub model_: *const mjModel,
    pub data_: *mut mjData,
    pub func_: [u8; 8],
    pub arg_: *mut (),
    pub ntask_: i32,
    pub next_: [u8; 28],
    pub ndone_: [u8; 4],
    pub signal_: [u8; 4],
    pub threads_: [u8; 56],
}
const _: () = assert!(std::mem::size_of::<ThreadPoolContext>() == 128);

/// C struct: tinyxml2::XMLAttribute (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct XMLAttribute {
    pub _vtable: *mut (),
    pub _name: StrPair,
    pub _value: StrPair,
    pub _parseLineNum: i32,
    pub _pad_0: [u8; 4],
    pub _next: *mut XMLAttribute,
    pub _memPool: *mut MemPool,
}
const _: () = assert!(std::mem::size_of::<XMLAttribute>() == 80);

/// C struct: tinyxml2::XMLDocument (880 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct XMLDocument {
    pub _vtable: *mut (),
    pub _document: *mut XMLDocument,
    pub _parent: *mut XMLNode,
    pub _value: StrPair,
    pub _parseLineNum: i32,
    pub _pad_0: [u8; 4],
    pub _firstChild: *mut XMLNode,
    pub _lastChild: *mut XMLNode,
    pub _prev: *mut XMLNode,
    pub _next: *mut XMLNode,
    pub _userData: *mut (),
    pub _memPool: *mut MemPool,
    pub _writeBOM: bool,
    pub _processEntities: bool,
    pub _pad_1: [u8; 2],
    pub _errorID: [u8; 4],
    pub _whitespaceMode: [u8; 8],
    pub _errorStr: StrPair,
    pub _errorLineNum: i32,
    pub _pad_2: [u8; 4],
    pub _charBuffer: *mut i8,
    pub _parseCurLineNum: i32,
    pub _parsingDepth: i32,
    pub _unlinked: *const (),
    pub _pad_3: [u8; 96],
    pub _elementPool: *const (),
    pub _pad_4: [u8; 144],
    pub _attributePool: *const (),
    pub _pad_5: [u8; 144],
    pub _textPool: *const (),
    pub _pad_6: [u8; 144],
    pub _commentPool: *const (),
    pub _pad_7: [u8; 144],
}
const _: () = assert!(std::mem::size_of::<XMLDocument>() == 880);

/// C struct: tinyxml2::XMLElement (120 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct XMLElement {
    pub _vtable: *mut (),
    pub _document: *mut XMLDocument,
    pub _parent: *mut XMLNode,
    pub _value: StrPair,
    pub _parseLineNum: i32,
    pub _pad_0: [u8; 4],
    pub _firstChild: *mut XMLNode,
    pub _lastChild: *mut XMLNode,
    pub _prev: *mut XMLNode,
    pub _next: *mut XMLNode,
    pub _userData: *mut (),
    pub _memPool: *mut MemPool,
    pub _closingType: [u8; 8],
    pub _rootAttribute: *mut XMLAttribute,
}
const _: () = assert!(std::mem::size_of::<XMLElement>() == 120);

/// C struct: tinyxml2::XMLNode (104 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct XMLNode {
    pub _vtable: *mut (),
    pub _document: *mut XMLDocument,
    pub _parent: *mut XMLNode,
    pub _value: StrPair,
    pub _parseLineNum: i32,
    pub _pad_0: [u8; 4],
    pub _firstChild: *mut XMLNode,
    pub _lastChild: *mut XMLNode,
    pub _prev: *mut XMLNode,
    pub _next: *mut XMLNode,
    pub _userData: *mut (),
    pub _memPool: *mut MemPool,
}
const _: () = assert!(std::mem::size_of::<XMLNode>() == 104);

/// C struct: ZipArchiveProvider (352 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct ZipArchiveProvider {
    pub prefix: *const i8,
    pub open: mjfOpenResource,
    pub read: mjfReadResource,
    pub close: mjfCloseResource,
    pub mount: mjfMountResource,
    pub unmount: mjfUnmountResource,
    pub modified: mjfResourceModified,
    pub data: *mut (),
    pub name_: std__string,
    pub root_model_: std__string,
    pub archive_: [u8; 112],
    pub buffer_: *const (),
    pub _pad_0: [u8; 16],
    pub files_: [u8; 40],
    pub mutex_: std__mutex,
}
const _: () = assert!(std::mem::size_of::<ZipArchiveProvider>() == 352);

/// C struct: ccd_vec3_t (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct ccd_vec3_t {
    pub v: [u8; 24],
}
const _: () = assert!(std::mem::size_of::<ccd_vec3_t>() == 24);

/// C struct: mjCActuator (1688 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCActuator {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub trnid: [i32; 2],
    pub actadr_: i32,
    pub actdim_: i32,
    pub act_: *const (),
    pub _pad_1: [u8; 16],
    pub ctrl_: *const (),
    pub _pad_2: [u8; 16],
    pub plugin_name: std__string,
    pub plugin_instance_name: std__string,
    pub target_: std__string,
    pub slidersite_: std__string,
    pub refsite_: std__string,
    pub userdata_: *const (),
    pub _pad_3: [u8; 16],
    pub spec_target_: std__string,
    pub spec_slidersite_: std__string,
    pub spec_refsite_: std__string,
    pub spec_userdata_: *const (),
    pub _pad_4: [u8; 16],
    pub element: *mut mjsElement,
    pub gaintype: [u8; 8],
    pub gainprm: [f64; 10],
    pub biastype: [u8; 8],
    pub biasprm: [f64; 10],
    pub dyntype: [u8; 8],
    pub dynprm: [f64; 10],
    pub actdim: i32,
    pub actearly: u8,
    pub _pad_5: [u8; 3],
    pub trntype: [u8; 8],
    pub gear: [f64; 6],
    pub target: *mut mjString,
    pub refsite: *mut mjString,
    pub slidersite: *mut mjString,
    pub cranklength: f64,
    pub lengthrange: [f64; 2],
    pub inheritrange: f64,
    pub damping: [f64; 3],
    pub armature: f64,
    pub ctrllimited: i32,
    pub _pad_6: [u8; 4],
    pub ctrlrange: [f64; 2],
    pub forcelimited: i32,
    pub _pad_7: [u8; 4],
    pub forcerange: [f64; 2],
    pub actlimited: i32,
    pub _pad_8: [u8; 4],
    pub actrange: [f64; 2],
    pub group: i32,
    pub nsample: i32,
    pub interp: i32,
    pub _pad_9: [u8; 4],
    pub delay: f64,
    pub userdata: *mut mjDoubleVec,
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
    pub spec: mjsActuator,
    pub ptarget: *mut mjCBase,
}
const _: () = assert!(std::mem::size_of::<mjCActuator>() == 1688);

/// C struct: mjCAsset (112 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCAsset {
    pub id_: std__string,
    pub timestamp_: std__string,
    pub insert_num_: u64,
    pub access_count_: u64,
    pub size_: u64,
    pub data_: *const (),
    pub _pad_0: [u8; 8],
    pub references_: *const (),
    pub _pad_1: [u8; 16],
}
const _: () = assert!(std::mem::size_of::<mjCAsset>() == 112);

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
    pub user_payload_: *const (),
    pub _pad_3: [u8; 32],
}
const _: () = assert!(std::mem::size_of::<mjCBase>() == 224);

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
    pub userdata_: *const (),
    pub _pad_2: [u8; 16],
    pub spec_userdata_: *const (),
    pub _pad_3: [u8; 16],
    pub mpos_: *const (),
    pub _pad_4: [u8; 16],
    pub mquat_: *const (),
    pub _pad_5: [u8; 16],
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
    pub mocap: u8,
    pub _pad_6: [u8; 7],
    pub gravcomp: f64,
    pub sleep: [u8; 8],
    pub userdata: *mut mjDoubleVec,
    pub explicitinertial: u8,
    pub _pad_7: [u8; 7],
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
    pub spec: mjsBody,
    pub last_attached: *mut mjsFrame,
    pub bodies: *const (),
    pub _pad_8: [u8; 16],
    pub geoms: *const (),
    pub _pad_9: [u8; 16],
    pub frames: *const (),
    pub _pad_10: [u8; 16],
    pub joints: *const (),
    pub _pad_11: [u8; 16],
    pub sites: *const (),
    pub _pad_12: [u8; 16],
    pub cameras: *const (),
    pub _pad_13: [u8; 16],
    pub lights: *const (),
    pub _pad_14: [u8; 16],
}
const _: () = assert!(std::mem::size_of::<mjCBody>() == 2016);

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

/// C struct: mjCBoundingVolume (128 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCBoundingVolume {
    pub contype_: i32,
    pub conaffinity_: i32,
    pub aabb_: *const (),
    pub _pad_0: [u8; 40],
    pub pos_: *const (),
    pub _pad_1: [u8; 16],
    pub quat_: *const (),
    pub _pad_2: [u8; 24],
    pub quat_set_: bool,
    pub _pad_3: [u8; 3],
    pub idval_: i32,
    pub id_: *const i32,
}
const _: () = assert!(std::mem::size_of::<mjCBoundingVolume>() == 128);

/// C struct: mjCBoundingVolumeHierarchy (232 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCBoundingVolumeHierarchy {
    pub nbvh_: i32,
    pub _pad_0: [u8; 4],
    pub bvh_: *const (),
    pub _pad_1: [u8; 16],
    pub child_: *const (),
    pub _pad_2: [u8; 16],
    pub nodeid_: *const (),
    pub _pad_3: [u8; 16],
    pub nodeidptr_: *const (),
    pub _pad_4: [u8; 16],
    pub level_: *const (),
    pub _pad_5: [u8; 16],
    pub bvleaf_: *const (),
    pub _pad_6: [u8; 16],
    pub name_: std__string,
    pub ipos_: [f64; 3],
    pub iquat_: [f64; 4],
}
const _: () = assert!(std::mem::size_of::<mjCBoundingVolumeHierarchy>() == 232);

/// C struct: mjCCDObj (376 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCCDObj {
    pub geom: i32,
    pub geom_type: i32,
    pub size: [f64; 4],
    pub pos: [f64; 3],
    pub mat: [f64; 9],
    pub vertindex: i32,
    pub meshindex: i32,
    pub flex: i32,
    pub elem: i32,
    pub vert: i32,
    pub _pad_0: [u8; 4],
    pub margin: f64,
    pub rotate: [f64; 4],
    pub data: union__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_src_engine_engine_collision_convex_h_52_3,
    pub center: Option<unsafe extern "C" fn()>,
    pub support: Option<unsafe extern "C" fn()>,
}
const _: () = assert!(std::mem::size_of::<mjCCDObj>() == 376);

/// C struct: mjCCache (192 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCCache {
    pub mutex_: std__mutex,
    pub insert_num_: u64,
    pub size_: u64,
    pub capacity_: u64,
    pub lookup_: *const (),
    pub _pad_0: [u8; 32],
    pub entries_: *const (),
    pub _pad_1: [u8; 16],
    pub models_: *const (),
    pub _pad_2: [u8; 32],
}
const _: () = assert!(std::mem::size_of::<mjCCache>() == 192);

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
    pub userdata_: *const (),
    pub _pad_2: [u8; 16],
    pub spec_userdata_: *const (),
    pub _pad_3: [u8; 16],
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub mode: [u8; 8],
    pub targetbody: *mut mjString,
    pub proj: [u8; 4],
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

/// C struct: mjCComposite (14768 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCComposite {
    pub prefix: std__string,
    pub r#type: [u8; 4],
    pub count: [u8; 12],
    pub offset: [u8; 24],
    pub quat: [u8; 32],
    pub initial: std__string,
    pub uservert: *const (),
    pub _pad_0: [u8; 16],
    pub size: [u8; 24],
    pub curve: [u8; 16],
    pub frame: *mut mjsFrame,
    pub username: *const (),
    pub _pad_1: [u8; 16],
    pub plugin_name: std__string,
    pub plugin_instance_name: std__string,
    pub plugin: mjsPlugin,
    pub skin: bool,
    pub skintexcoord: bool,
    pub _pad_2: [u8; 6],
    pub skinmaterial: std__string,
    pub skinrgba: [u8; 16],
    pub skininflate: f32,
    pub skinsubgrid: i32,
    pub skingroup: i32,
    pub add: [u8; 4],
    pub def: [u8; 14184],
    pub defjoint: [u8; 40],
    pub dim: i32,
    pub _pad_3: [u8; 4],
    pub face: *const (),
    pub _pad_4: [u8; 16],
    pub vert: *const (),
    pub _pad_5: [u8; 16],
    pub bindpos: *const (),
    pub _pad_6: [u8; 16],
    pub bindquat: *const (),
    pub _pad_7: [u8; 16],
    pub texcoord: *const (),
    pub _pad_8: [u8; 16],
    pub vertid: *const (),
    pub _pad_9: [u8; 16],
    pub vertweight: *const (),
    pub _pad_10: [u8; 16],
}
const _: () = assert!(std::mem::size_of::<mjCComposite>() == 14768);

/// C struct: mjCDef (14184 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCDef {
    pub elemtype: [u8; 8],
    pub signature: u64,
    pub name: std__string,
    pub id: i32,
    pub _pad_0: [u8; 4],
    pub parent: *mut mjCDef,
    pub child: *const (),
    pub _pad_1: [u8; 16],
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
const _: () = assert!(std::mem::size_of::<mjCDef>() == 14184);

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
    pub r#type: [u8; 8],
    pub data: [f64; 11],
    pub active: u8,
    pub _pad_1: [u8; 7],
    pub name1: *mut mjString,
    pub name2: *mut mjString,
    pub objtype: [u8; 8],
    pub solref: [f64; 2],
    pub solimp: [f64; 5],
    pub info: *mut mjString,
    pub spec: mjsEquality,
}
const _: () = assert!(std::mem::size_of::<mjCEquality>() == 728);

/// C struct: mjCError (500 bytes, align 1)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCError {
    pub message: [i8; 500],
}
const _: () = assert!(std::mem::size_of::<mjCError>() == 500);

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
    pub vertbodyid: *const (),
    pub _pad_2: [u8; 16],
    pub nodebodyid: *const (),
    pub _pad_3: [u8; 16],
    pub edge: *const (),
    pub _pad_4: [u8; 16],
    pub shell: *const (),
    pub _pad_5: [u8; 16],
    pub elemlayer: *const (),
    pub _pad_6: [u8; 16],
    pub evpair: *const (),
    pub _pad_7: [u8; 16],
    pub flaps: *const (),
    pub _pad_8: [u8; 16],
    pub vertxpos: *const (),
    pub _pad_9: [u8; 16],
    pub tree: mjCBoundingVolumeHierarchy,
    pub elemaabb_: *const (),
    pub _pad_10: [u8; 16],
    pub edgeidx_: *const (),
    pub _pad_11: [u8; 16],
    pub stiffness: *const (),
    pub _pad_12: [u8; 16],
    pub bending: *const (),
    pub _pad_13: [u8; 16],
    pub has_strain_eq: bool,
    pub _pad_14: [u8; 7],
    pub cell_empty: *const (),
    pub _pad_15: [u8; 16],
    pub vertbody_: *const (),
    pub _pad_16: [u8; 16],
    pub nodebody_: *const (),
    pub _pad_17: [u8; 16],
    pub vert_: *const (),
    pub _pad_18: [u8; 16],
    pub node_: *const (),
    pub _pad_19: [u8; 16],
    pub elem_: *const (),
    pub _pad_20: [u8; 16],
    pub texcoord_: *const (),
    pub _pad_21: [u8; 16],
    pub elemtexcoord_: *const (),
    pub _pad_22: [u8; 16],
    pub material_: std__string,
    pub spec_material_: std__string,
    pub spec_vertbody_: *const (),
    pub _pad_23: [u8; 16],
    pub spec_nodebody_: *const (),
    pub _pad_24: [u8; 16],
    pub spec_vert_: *const (),
    pub _pad_25: [u8; 16],
    pub spec_node_: *const (),
    pub _pad_26: [u8; 16],
    pub spec_elem_: *const (),
    pub _pad_27: [u8; 16],
    pub spec_texcoord_: *const (),
    pub _pad_28: [u8; 16],
    pub spec_elemtexcoord_: *const (),
    pub _pad_29: [u8; 16],
    pub cached_stiffness_: *const (),
    pub _pad_30: [u8; 16],
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
    pub _pad_31: [u8; 4],
    pub radius: f64,
    pub size: [f64; 3],
    pub internal: u8,
    pub flatskin: u8,
    pub _pad_32: [u8; 2],
    pub selfcollide: i32,
    pub passive: i32,
    pub activelayers: i32,
    pub group: i32,
    pub _pad_33: [u8; 4],
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
    pub _pad_34: [u8; 4],
    pub nodebody: *mut mjStringVec,
    pub vertbody: *mut mjStringVec,
    pub node: *mut mjDoubleVec,
    pub vert: *mut mjDoubleVec,
    pub elem: *mut mjIntVec,
    pub texcoord: *mut mjFloatVec,
    pub elemtexcoord: *mut mjIntVec,
    pub info: *mut mjString,
    pub spec: mjsFlex,
    pub vert0_: *const (),
    pub _pad_35: [u8; 16],
    pub node0_: *const (),
    pub _pad_36: [u8; 16],
}
const _: () = assert!(std::mem::size_of::<mjCFlex>() == 1960);

/// C struct: mjCFlexcomp (14896 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCFlexcomp {
    pub name: std__string,
    pub r#type: [u8; 4],
    pub count: [u8; 12],
    pub cellcount: [u8; 16],
    pub spacing: [u8; 24],
    pub scale: [u8; 24],
    pub origin: [u8; 24],
    pub mass: f64,
    pub inertiabox: f64,
    pub equality: i32,
    pub _pad_0: [u8; 4],
    pub file: std__string,
    pub doftype: [u8; 8],
    pub pinid: *const (),
    pub _pad_1: [u8; 16],
    pub pinrange: *const (),
    pub _pad_2: [u8; 16],
    pub pingrid: *const (),
    pub _pad_3: [u8; 16],
    pub pingridrange: *const (),
    pub _pad_4: [u8; 16],
    pub def: mjCDef,
    pub pos: [u8; 24],
    pub quat: [u8; 32],
    pub alt: mjsOrientation,
    pub rigid: bool,
    pub centered: bool,
    pub _pad_5: [u8; 6],
    pub point: *const (),
    pub _pad_6: [u8; 16],
    pub pinned: *const (),
    pub _pad_7: [u8; 16],
    pub used: *const (),
    pub _pad_8: [u8; 16],
    pub element: *const (),
    pub _pad_9: [u8; 16],
    pub texcoord: *const (),
    pub _pad_10: [u8; 16],
    pub elemtexcoord: *const (),
    pub _pad_11: [u8; 16],
    pub plugin_name: std__string,
    pub plugin_instance_name: std__string,
    pub plugin: mjsPlugin,
}
const _: () = assert!(std::mem::size_of::<mjCFlexcomp>() == 14896);

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

/// C struct: mjCGeom (1840 bytes, align 8)
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
    pub userdata_: *const (),
    pub _pad_2: [u8; 16],
    pub spec_hfieldname_: std__string,
    pub spec_meshname_: std__string,
    pub spec_material_: std__string,
    pub spec_userdata_: *const (),
    pub _pad_3: [u8; 16],
    pub element: *mut mjsElement,
    pub r#type: [u8; 8],
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
    pub mass: f64,
    pub density: f64,
    pub typeinertia: [u8; 8],
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
const _: () = assert!(std::mem::size_of::<mjCGeom>() == 1840);

/// C struct: mjCHField (552 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCHField {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub data: *const (),
    pub _pad_1: [u8; 16],
    pub file_: std__string,
    pub content_type_: std__string,
    pub userdata_: *const (),
    pub _pad_2: [u8; 16],
    pub spec_file_: std__string,
    pub spec_content_type_: std__string,
    pub spec_userdata_: *const (),
    pub _pad_3: [u8; 16],
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

/// C struct: mjCJoint (1056 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCJoint {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub body: *mut mjCBody,
    pub qpos_: *const (),
    pub _pad_1: [u8; 16],
    pub qvel_: *const (),
    pub _pad_2: [u8; 16],
    pub userdata_: *const (),
    pub _pad_3: [u8; 16],
    pub spec_userdata_: *const (),
    pub _pad_4: [u8; 16],
    pub element: *mut mjsElement,
    pub r#type: [u8; 8],
    pub pos: [f64; 3],
    pub axis: [f64; 3],
    pub r#ref: f64,
    pub align: i32,
    pub _pad_5: [u8; 4],
    pub stiffness: [f64; 3],
    pub springref: f64,
    pub springdamper: [f64; 2],
    pub limited: i32,
    pub _pad_6: [u8; 4],
    pub range: [f64; 2],
    pub margin: f64,
    pub solref_limit: [f64; 2],
    pub solimp_limit: [f64; 5],
    pub actfrclimited: i32,
    pub _pad_7: [u8; 4],
    pub actfrcrange: [f64; 2],
    pub armature: f64,
    pub damping: [f64; 3],
    pub frictionloss: f64,
    pub solref_friction: [f64; 2],
    pub solimp_friction: [f64; 5],
    pub group: i32,
    pub actgravcomp: u8,
    pub _pad_8: [u8; 3],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
    pub spec: mjsJoint,
    pub qposadr_: i32,
    pub dofadr_: i32,
}
const _: () = assert!(std::mem::size_of::<mjCJoint>() == 1056);

/// C struct: mjCKey (656 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCKey {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub qpos_: *const (),
    pub _pad_1: [u8; 16],
    pub qvel_: *const (),
    pub _pad_2: [u8; 16],
    pub act_: *const (),
    pub _pad_3: [u8; 16],
    pub mpos_: *const (),
    pub _pad_4: [u8; 16],
    pub mquat_: *const (),
    pub _pad_5: [u8; 16],
    pub ctrl_: *const (),
    pub _pad_6: [u8; 16],
    pub spec_qpos_: *const (),
    pub _pad_7: [u8; 16],
    pub spec_qvel_: *const (),
    pub _pad_8: [u8; 16],
    pub spec_act_: *const (),
    pub _pad_9: [u8; 16],
    pub spec_mpos_: *const (),
    pub _pad_10: [u8; 16],
    pub spec_mquat_: *const (),
    pub _pad_11: [u8; 16],
    pub spec_ctrl_: *const (),
    pub _pad_12: [u8; 16],
    pub element: *mut mjsElement,
    pub time: f64,
    pub qpos: *mut mjDoubleVec,
    pub qvel: *mut mjDoubleVec,
    pub act: *mut mjDoubleVec,
    pub mpos: *mut mjDoubleVec,
    pub mquat: *mut mjDoubleVec,
    pub ctrl: *mut mjDoubleVec,
    pub info: *mut mjString,
    pub spec: mjsKey,
}
const _: () = assert!(std::mem::size_of::<mjCKey>() == 656);

/// C struct: mjCLight (672 bytes, align 8)
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
    pub mode: [u8; 8],
    pub targetbody: *mut mjString,
    pub active: u8,
    pub _pad_1: [u8; 3],
    pub r#type: [u8; 4],
    pub texture: *mut mjString,
    pub castshadow: u8,
    pub _pad_2: [u8; 3],
    pub bulbradius: f32,
    pub intensity: f32,
    pub range: f32,
    pub attenuation: [f32; 3],
    pub cutoff: f32,
    pub exponent: f32,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub info: *mut mjString,
    pub spec: mjsLight,
}
const _: () = assert!(std::mem::size_of::<mjCLight>() == 672);

/// C struct: mjCMaterial (472 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCMaterial {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub texid: [i32; 10],
    pub textures_: *const (),
    pub _pad_1: [u8; 16],
    pub spec_textures_: *const (),
    pub _pad_2: [u8; 16],
    pub element: *mut mjsElement,
    pub textures: *mut mjStringVec,
    pub texuniform: u8,
    pub _pad_3: [u8; 3],
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
    pub vert_: *const (),
    pub _pad_1: [u8; 16],
    pub normal_: *const (),
    pub _pad_2: [u8; 16],
    pub texcoord_: *const (),
    pub _pad_3: [u8; 16],
    pub face_: *const (),
    pub _pad_4: [u8; 16],
    pub facenormal_: *const (),
    pub _pad_5: [u8; 16],
    pub facetexcoord_: *const (),
    pub _pad_6: [u8; 16],
    pub material_: std__string,
    pub spec_content_type_: std__string,
    pub spec_file_: std__string,
    pub spec_vert_: *const (),
    pub _pad_7: [u8; 16],
    pub spec_normal_: *const (),
    pub _pad_8: [u8; 16],
    pub spec_texcoord_: *const (),
    pub _pad_9: [u8; 16],
    pub spec_face_: *const (),
    pub _pad_10: [u8; 16],
    pub spec_facenormal_: *const (),
    pub _pad_11: [u8; 16],
    pub spec_facetexcoord_: *const (),
    pub _pad_12: [u8; 16],
    pub spec_material_: std__string,
    pub needreorient_: bool,
    pub visual_: bool,
    pub _pad_13: [u8; 6],
    pub halfedge_: *const (),
    pub _pad_14: [u8; 16],
    pub processed_: bool,
    pub transformed_: bool,
    pub _pad_15: [u8; 6],
    pub pos_: [f64; 3],
    pub quat_: [f64; 4],
    pub boxsz_: [f64; 3],
    pub aamm_: [f64; 6],
    pub volume_: f64,
    pub surface_: f64,
    pub szgraph_: i32,
    pub needhull_: bool,
    pub _pad_16: [u8; 3],
    pub maxhullvert_: i32,
    pub _pad_17: [u8; 4],
    pub tree_: mjCBoundingVolumeHierarchy,
    pub face_aabb_: *const (),
    pub _pad_18: [u8; 16],
    pub octree_: mjCOctree,
    pub mesh_timer_: [f64; 9],
    pub element: *mut mjsElement,
    pub content_type: *mut mjString,
    pub file: *mut mjString,
    pub refpos: [f64; 3],
    pub refquat: [f64; 4],
    pub scale: [f64; 3],
    pub inertia: [u8; 4],
    pub smoothnormal: u8,
    pub needsdf: u8,
    pub _pad_19: [u8; 2],
    pub maxhullvert: i32,
    pub _pad_20: [u8; 4],
    pub uservert: *mut mjFloatVec,
    pub usernormal: *mut mjFloatVec,
    pub usertexcoord: *mut mjFloatVec,
    pub userface: *mut mjIntVec,
    pub userfacenormal: *mut mjIntVec,
    pub userfacetexcoord: *mut mjIntVec,
    pub plugin: mjsPlugin,
    pub material: *mut mjString,
    pub octree_maxdepth: i32,
    pub _pad_21: [u8; 4],
    pub info: *mut mjString,
    pub spec: mjsMesh,
    pub center_: *mut f64,
    pub graph_: *mut i32,
    pub polygons_: *const (),
    pub _pad_22: [u8; 16],
    pub polygon_normals_: *const (),
    pub _pad_23: [u8; 16],
    pub polygon_map_: *const (),
    pub _pad_24: [u8; 16],
}
const _: () = assert!(std::mem::size_of::<mjCMesh>() == 1968);

/// C struct: mjCModel (5920 bytes, align 8)
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
    pub qpos0: *const (),
    pub _pad_2: [u8; 16],
    pub body_pos0: *const (),
    pub _pad_3: [u8; 16],
    pub body_quat0: *const (),
    pub _pad_4: [u8; 16],
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
    pub strippath: u8,
    pub _pad_5: [u8; 7],
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
    pub _pad_6: [u8; 4],
    pub nstack: i64,
    pub comment: *mut mjString,
    pub modelfiledir: *mut mjString,
    pub hasImplicitPluginElem: u8,
    pub _pad_7: [u8; 7],
    pub authored: mjsAuthored,
    pub spec: mjSpec,
    pub timer: [f64; 9],
    pub def_map: *const (),
    pub _pad_8: [u8; 32],
    pub refcount: i32,
    pub _pad_9: [u8; 4],
    pub defaults_: *const (),
    pub _pad_10: [u8; 16],
    pub active_plugins_: *const (),
    pub _pad_11: [u8; 16],
    pub flexes_: *const (),
    pub _pad_12: [u8; 16],
    pub meshes_: *const (),
    pub _pad_13: [u8; 16],
    pub skins_: *const (),
    pub _pad_14: [u8; 16],
    pub hfields_: *const (),
    pub _pad_15: [u8; 16],
    pub textures_: *const (),
    pub _pad_16: [u8; 16],
    pub materials_: *const (),
    pub _pad_17: [u8; 16],
    pub pairs_: *const (),
    pub _pad_18: [u8; 16],
    pub excludes_: *const (),
    pub _pad_19: [u8; 16],
    pub equalities_: *const (),
    pub _pad_20: [u8; 16],
    pub tendons_: *const (),
    pub _pad_21: [u8; 16],
    pub actuators_: *const (),
    pub _pad_22: [u8; 16],
    pub sensors_: *const (),
    pub _pad_23: [u8; 16],
    pub numerics_: *const (),
    pub _pad_24: [u8; 16],
    pub texts_: *const (),
    pub _pad_25: [u8; 16],
    pub tuples_: *const (),
    pub _pad_26: [u8; 16],
    pub keys_: *const (),
    pub _pad_27: [u8; 16],
    pub plugins_: *const (),
    pub _pad_28: [u8; 16],
    pub specs_: *const (),
    pub _pad_29: [u8; 16],
    pub bodies_: *const (),
    pub _pad_30: [u8; 16],
    pub joints_: *const (),
    pub _pad_31: [u8; 16],
    pub geoms_: *const (),
    pub _pad_32: [u8; 16],
    pub sites_: *const (),
    pub _pad_33: [u8; 16],
    pub cameras_: *const (),
    pub _pad_34: [u8; 16],
    pub lights_: *const (),
    pub _pad_35: [u8; 16],
    pub frames_: *const (),
    pub _pad_36: [u8; 16],
    pub object_lists_: *const (),
    pub _pad_37: [u8; 200],
    pub ids: mjListKeyMap,
    pub errInfo: [u8; 504],
    pub warnings_: *const (),
    pub _pad_38: [u8; 16],
    pub num_attach_warnings_: i32,
    pub compiling_: bool,
    pub _pad_39: [u8; 3],
    pub key_pending_: *const (),
    pub _pad_40: [u8; 16],
    pub deepcopy_: bool,
    pub attached_: bool,
    pub _pad_41: [u8; 6],
    pub compiler2spec_: *const (),
    pub _pad_42: [u8; 32],
    pub detached_: *const (),
    pub _pad_43: [u8; 16],
}
const _: () = assert!(std::mem::size_of::<mjCModel>() == 5920);

/// C struct: mjCNumeric (336 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCNumeric {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub data_: *const (),
    pub _pad_1: [u8; 16],
    pub spec_data_: *const (),
    pub _pad_2: [u8; 16],
    pub element: *mut mjsElement,
    pub data: *mut mjDoubleVec,
    pub size: i32,
    pub _pad_3: [u8; 4],
    pub info: *mut mjString,
    pub spec: mjsNumeric,
}
const _: () = assert!(std::mem::size_of::<mjCNumeric>() == 336);

/// C struct: mjCOctree (176 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCOctree {
    pub nnode_: i32,
    pub nvert_: i32,
    pub max_depth_: i32,
    pub _pad_0: [u8; 4],
    pub node_: *const (),
    pub _pad_1: [u8; 16],
    pub face_: *const (),
    pub _pad_2: [u8; 16],
    pub vert_: *const (),
    pub _pad_3: [u8; 16],
    pub hang_: *const (),
    pub _pad_4: [u8; 16],
    pub ipos_: [f64; 3],
    pub iquat_: [f64; 4],
    pub smoothing_iterations_: i32,
    pub _pad_5: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<mjCOctree>() == 176);

/// C struct: mjCPair (680 bytes, align 8)
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
    pub friction: [f64; 5],
    pub info: *mut mjString,
    pub spec: mjsPair,
    pub geom1: *mut mjCGeom,
    pub geom2: *mut mjCGeom,
}
const _: () = assert!(std::mem::size_of::<mjCPair>() == 680);

/// C struct: mjCPlugin (360 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCPlugin {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub nstate: i32,
    pub _pad_1: [u8; 4],
    pub config_attribs: *const (),
    pub _pad_2: [u8; 16],
    pub flattened_attributes: *const (),
    pub _pad_3: [u8; 16],
    pub plugin_name: std__string,
    pub spec: mjsPlugin,
    pub parent: *mut mjCBase,
    pub plugin_slot: i32,
    pub _pad_4: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<mjCPlugin>() == 360);

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
    pub userdata_: *const (),
    pub _pad_1: [u8; 16],
    pub spec_objname_: std__string,
    pub spec_refname_: std__string,
    pub spec_userdata_: *const (),
    pub _pad_2: [u8; 16],
    pub element: *mut mjsElement,
    pub r#type: [u8; 4],
    pub objtype: [u8; 4],
    pub objname: *mut mjString,
    pub reftype: [u8; 8],
    pub refname: *mut mjString,
    pub intprm: [i32; 3],
    pub datatype: [u8; 4],
    pub needstage: [u8; 4],
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

/// C struct: mjCSite (992 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCSite {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub material_: std__string,
    pub userdata_: *const (),
    pub _pad_1: [u8; 16],
    pub spec_material_: std__string,
    pub spec_userdata_: *const (),
    pub _pad_2: [u8; 16],
    pub body: *mut mjCBody,
    pub matid: i32,
    pub _pad_3: [u8; 4],
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub fromto: [f64; 6],
    pub size: [f64; 3],
    pub r#type: [u8; 8],
    pub material: *mut mjString,
    pub group: i32,
    pub rgba: [u8; 20],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
    pub spec: mjsSite,
}
const _: () = assert!(std::mem::size_of::<mjCSite>() == 992);

/// C struct: mjCSkin (976 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCSkin {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub file_: std__string,
    pub material_: std__string,
    pub vert_: *const (),
    pub _pad_1: [u8; 16],
    pub texcoord_: *const (),
    pub _pad_2: [u8; 16],
    pub face_: *const (),
    pub _pad_3: [u8; 16],
    pub bodyname_: *const (),
    pub _pad_4: [u8; 16],
    pub bindpos_: *const (),
    pub _pad_5: [u8; 16],
    pub bindquat_: *const (),
    pub _pad_6: [u8; 16],
    pub vertid_: *const (),
    pub _pad_7: [u8; 16],
    pub vertweight_: *const (),
    pub _pad_8: [u8; 16],
    pub spec_file_: std__string,
    pub spec_material_: std__string,
    pub spec_vert_: *const (),
    pub _pad_9: [u8; 16],
    pub spec_texcoord_: *const (),
    pub _pad_10: [u8; 16],
    pub spec_face_: *const (),
    pub _pad_11: [u8; 16],
    pub spec_bodyname_: *const (),
    pub _pad_12: [u8; 16],
    pub spec_bindpos_: *const (),
    pub _pad_13: [u8; 16],
    pub spec_bindquat_: *const (),
    pub _pad_14: [u8; 16],
    pub spec_vertid_: *const (),
    pub _pad_15: [u8; 16],
    pub spec_vertweight_: *const (),
    pub _pad_16: [u8; 16],
    pub matid: i32,
    pub _pad_17: [u8; 4],
    pub bodyid: *const (),
    pub _pad_18: [u8; 16],
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
    pub userdata_: *const (),
    pub _pad_2: [u8; 16],
    pub spec_userdata_: *const (),
    pub _pad_3: [u8; 16],
    pub element: *mut mjsElement,
    pub stiffness: [f64; 3],
    pub springlength: [f64; 2],
    pub damping: [f64; 3],
    pub frictionloss: f64,
    pub solref_friction: [f64; 2],
    pub solimp_friction: [f64; 5],
    pub armature: f64,
    pub limited: i32,
    pub actfrclimited: i32,
    pub range: [f64; 2],
    pub actfrcrange: [f64; 2],
    pub margin: f64,
    pub solref_limit: [f64; 2],
    pub solimp_limit: [f64; 5],
    pub material: *mut mjString,
    pub width: f64,
    pub rgba: [f32; 4],
    pub group: i32,
    pub _pad_4: [u8; 4],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
    pub spec: mjsTendon,
    pub path: *const (),
    pub _pad_5: [u8; 16],
}
const _: () = assert!(std::mem::size_of::<mjCTendon>() == 960);

/// C struct: mjCText (320 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCText {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub data_: std__string,
    pub spec_data_: std__string,
    pub element: *mut mjsElement,
    pub data: *mut mjString,
    pub info: *mut mjString,
    pub spec: mjsText,
}
const _: () = assert!(std::mem::size_of::<mjCText>() == 320);

/// C struct: mjCTexture (792 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCTexture {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub data_: *const (),
    pub _pad_1: [u8; 16],
    pub file_: std__string,
    pub content_type_: std__string,
    pub cubefiles_: *const (),
    pub _pad_2: [u8; 16],
    pub spec_file_: std__string,
    pub spec_content_type_: std__string,
    pub spec_cubefiles_: *const (),
    pub _pad_3: [u8; 16],
    pub element: *mut mjsElement,
    pub r#type: [u8; 4],
    pub colorspace: [u8; 4],
    pub builtin: i32,
    pub mark: i32,
    pub rgb1: [f64; 3],
    pub rgb2: [f64; 3],
    pub markrgb: [f64; 3],
    pub random: f64,
    pub height: i32,
    pub width: i32,
    pub nchannel: i32,
    pub _pad_4: [u8; 4],
    pub content_type: *mut mjString,
    pub file: *mut mjString,
    pub gridsize: [i32; 2],
    pub gridlayout: [u8; 16],
    pub cubefiles: *mut mjStringVec,
    pub data: *mut mjByteVec,
    pub hflip: u8,
    pub vflip: u8,
    pub _pad_5: [u8; 6],
    pub info: *mut mjString,
    pub spec: mjsTexture,
    pub texture_time_: f64,
    pub clear_data_: bool,
    pub _pad_6: [u8; 7],
}
const _: () = assert!(std::mem::size_of::<mjCTexture>() == 792);

/// C struct: mjCTuple (472 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCTuple {
    pub _vtable: *mut (),
    pub _pad_0: [u8; 216],
    pub obj: *const (),
    pub _pad_1: [u8; 16],
    pub objtype_: *const (),
    pub _pad_2: [u8; 16],
    pub objname_: *const (),
    pub _pad_3: [u8; 16],
    pub objprm_: *const (),
    pub _pad_4: [u8; 16],
    pub spec_objtype_: *const (),
    pub _pad_5: [u8; 16],
    pub spec_objname_: *const (),
    pub _pad_6: [u8; 16],
    pub spec_objprm_: *const (),
    pub _pad_7: [u8; 16],
    pub element: *mut mjsElement,
    pub objtype: *mut mjIntVec,
    pub objname: *mut mjStringVec,
    pub objprm: *mut mjDoubleVec,
    pub info: *mut mjString,
    pub spec: mjsTuple,
}
const _: () = assert!(std::mem::size_of::<mjCTuple>() == 472);

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
    pub r#type: [u8; 8],
    pub info: *mut mjString,
    pub spec: mjsWrap,
    pub obj: *mut mjCBase,
    pub tendon: *mut mjCTendon,
}
const _: () = assert!(std::mem::size_of::<mjCWrap>() == 328);

/// C struct: mjCache (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCache {
    pub impl_: *mut (),
}
const _: () = assert!(std::mem::size_of::<mjCache>() == 8);

/// C struct: mjContact (576 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjContact {
    pub dist: f64,
    pub pos: [f64; 3],
    pub frame: [f64; 9],
    pub includemargin: f64,
    pub friction: [f64; 5],
    pub solref: [f64; 2],
    pub solreffriction: [f64; 2],
    pub solimp: [f64; 5],
    pub mu: f64,
    pub H: [f64; 36],
    pub dim: i32,
    pub geom1: i32,
    pub geom2: i32,
    pub geom: [i32; 2],
    pub flex: [i32; 2],
    pub elem: [i32; 2],
    pub vert: [i32; 2],
    pub exclude: i32,
    pub efc_address: i32,
    pub _pad_0: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<mjContact>() == 576);

/// C struct: mjDCMotorSlots (24 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjDCMotorSlots {
    pub slew: i32,
    pub integral: i32,
    pub temperature: i32,
    pub bristle: i32,
    pub current: i32,
    pub num_slots: i32,
}
const _: () = assert!(std::mem::size_of::<mjDCMotorSlots>() == 24);

/// C struct: mjData (161872 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjData {
    pub narena: i64,
    pub nbuffer: i64,
    pub nplugin: i32,
    pub _pad_0: [u8; 4],
    pub pstack: usize,
    pub pbase: usize,
    pub parena: usize,
    pub threadpool: usize,
    pub threadlock: bool,
    pub _pad_1: [u8; 7],
    pub maxuse_stack: i64,
    pub maxuse_arena: i64,
    pub maxuse_con: i32,
    pub maxuse_efc: i32,
    pub solver: [u8; 160000],
    pub solver_niter: [i32; 20],
    pub solver_nnz: [i32; 20],
    pub solver_fwdinv: [f64; 2],
    pub warning: [u8; 56],
    pub timer: [u8; 240],
    pub ncon: i32,
    pub ne: i32,
    pub nf: i32,
    pub nl: i32,
    pub nefc: i32,
    pub nJ: i32,
    pub nY: i32,
    pub nA: i32,
    pub nisland: i32,
    pub nidof: i32,
    pub ntree_awake: i32,
    pub nbody_awake: i32,
    pub nparent_awake: i32,
    pub nv_awake: i32,
    pub flg_energypos: bool,
    pub flg_energyvel: bool,
    pub flg_subtreevel: bool,
    pub flg_rnepost: bool,
    pub _pad_2: [u8; 4],
    pub time: f64,
    pub energy: [f64; 2],
    pub buffer: *mut (),
    pub arena: *mut (),
    pub qpos: *mut f64,
    pub qvel: *mut f64,
    pub act: *mut f64,
    pub history: *mut f64,
    pub qacc_warmstart: *mut f64,
    pub plugin_state: *mut f64,
    pub ctrl: *mut f64,
    pub qfrc_applied: *mut f64,
    pub xfrc_applied: *mut f64,
    pub eq_active: *mut bool,
    pub mocap_pos: *mut f64,
    pub mocap_quat: *mut f64,
    pub qacc: *mut f64,
    pub act_dot: *mut f64,
    pub userdata: *mut f64,
    pub sensordata: *mut f64,
    pub tree_asleep: *mut i32,
    pub plugin: *mut i32,
    pub plugin_data: *mut usize,
    pub xpos: *mut f64,
    pub xquat: *mut f64,
    pub xmat: *mut f64,
    pub xipos: *mut f64,
    pub ximat: *mut f64,
    pub xanchor: *mut f64,
    pub xaxis: *mut f64,
    pub geom_xpos: *mut f64,
    pub geom_xmat: *mut f64,
    pub site_xpos: *mut f64,
    pub site_xmat: *mut f64,
    pub cam_xpos: *mut f64,
    pub cam_xmat: *mut f64,
    pub light_xpos: *mut f64,
    pub light_xdir: *mut f64,
    pub subtree_com: *mut f64,
    pub cdof: *mut f64,
    pub cinert: *mut f64,
    pub flexvert_xpos: *mut f64,
    pub flexelem_aabb: *mut f64,
    pub flexedge_J: *mut f64,
    pub flexedge_length: *mut f64,
    pub flexvert_J: *mut f64,
    pub flexvert_length: *mut f64,
    pub bvh_aabb_dyn: *mut f64,
    pub ten_wrapadr: *mut i32,
    pub ten_wrapnum: *mut i32,
    pub ten_J: *mut f64,
    pub ten_length: *mut f64,
    pub wrap_obj: *mut i32,
    pub wrap_xpos: *mut f64,
    pub actuator_length: *mut f64,
    pub moment_rownnz: *mut i32,
    pub moment_rowadr: *mut i32,
    pub moment_colind: *mut i32,
    pub actuator_moment: *mut f64,
    pub crb: *mut f64,
    pub qM: *mut f64,
    pub M: *mut f64,
    pub qLD: *mut f64,
    pub qLDiagInv: *mut f64,
    pub bvh_active: *mut bool,
    pub tree_awake: *mut i32,
    pub body_awake: *mut i32,
    pub body_awake_ind: *mut i32,
    pub parent_awake_ind: *mut i32,
    pub dof_awake_ind: *mut i32,
    pub flexedge_velocity: *mut f64,
    pub ten_velocity: *mut f64,
    pub actuator_velocity: *mut f64,
    pub cvel: *mut f64,
    pub cdof_dot: *mut f64,
    pub qfrc_bias: *mut f64,
    pub qfrc_spring: *mut f64,
    pub qfrc_damper: *mut f64,
    pub qfrc_gravcomp: *mut f64,
    pub qfrc_fluid: *mut f64,
    pub qfrc_passive: *mut f64,
    pub subtree_linvel: *mut f64,
    pub subtree_angmom: *mut f64,
    pub qH: *mut f64,
    pub qHDiagInv: *mut f64,
    pub qDeriv: *mut f64,
    pub qLU: *mut f64,
    pub actuator_force: *mut f64,
    pub qfrc_actuator: *mut f64,
    pub qfrc_smooth: *mut f64,
    pub qacc_smooth: *mut f64,
    pub qfrc_constraint: *mut f64,
    pub qfrc_inverse: *mut f64,
    pub cacc: *mut f64,
    pub cfrc_int: *mut f64,
    pub cfrc_ext: *mut f64,
    pub contact: *mut mjContact,
    pub efc_type: *mut i32,
    pub efc_id: *mut i32,
    pub efc_J_rownnz: *mut i32,
    pub efc_J_rowadr: *mut i32,
    pub efc_J_rowsuper: *mut i32,
    pub efc_J_colind: *mut i32,
    pub efc_J: *mut f64,
    pub efc_pos: *mut f64,
    pub efc_margin: *mut f64,
    pub efc_frictionloss: *mut f64,
    pub efc_diagA: *mut f64,
    pub efc_KBIP: *mut f64,
    pub efc_D: *mut f64,
    pub efc_R: *mut f64,
    pub tendon_efcadr: *mut i32,
    pub tree_island: *mut i32,
    pub island_ntree: *mut i32,
    pub island_itreeadr: *mut i32,
    pub map_itree2tree: *mut i32,
    pub dof_island: *mut i32,
    pub island_nv: *mut i32,
    pub island_idofadr: *mut i32,
    pub island_dofadr: *mut i32,
    pub map_dof2idof: *mut i32,
    pub map_idof2dof: *mut i32,
    pub ifrc_smooth: *mut f64,
    pub iacc_smooth: *mut f64,
    pub iacc: *mut f64,
    pub efc_island: *mut i32,
    pub island_ne: *mut i32,
    pub island_nf: *mut i32,
    pub island_nefc: *mut i32,
    pub island_iefcadr: *mut i32,
    pub map_efc2iefc: *mut i32,
    pub map_iefc2efc: *mut i32,
    pub iefc_type: *mut i32,
    pub iefc_id: *mut i32,
    pub iefc_frictionloss: *mut f64,
    pub iefc_D: *mut f64,
    pub iefc_R: *mut f64,
    pub efc_Y_rownnz: *mut i32,
    pub efc_Y_rowadr: *mut i32,
    pub efc_Y_colind: *mut i32,
    pub efc_Y: *mut f64,
    pub efc_AR_rownnz: *mut i32,
    pub efc_AR_rowadr: *mut i32,
    pub efc_AR_colind: *mut i32,
    pub efc_AR: *mut f64,
    pub efc_vel: *mut f64,
    pub efc_aref: *mut f64,
    pub efc_b: *mut f64,
    pub iefc_aref: *mut f64,
    pub iefc_state: *mut i32,
    pub iefc_force: *mut f64,
    pub efc_state: *mut i32,
    pub efc_force: *mut f64,
    pub ifrc_constraint: *mut f64,
    pub signature: u64,
}
const _: () = assert!(std::mem::size_of::<mjData>() == 161872);

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

/// C struct: mjLogConfig (1032 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjLogConfig {
    pub logto_console: bool,
    pub logto_file: bool,
    pub logfile: [u8; 1026],
    pub topics: i32,
}
const _: () = assert!(std::mem::size_of::<mjLogConfig>() == 1032);

/// C struct: mjLogMessage (1064 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjLogMessage {
    pub level: i32,
    pub topic: i32,
    pub subject: [i8; 1024],
    pub body: *const i8,
    pub func: *const i8,
    pub file: *const i8,
    pub line: i32,
    pub timestamp: bool,
    pub _pad_0: [u8; 3],
}
const _: () = assert!(std::mem::size_of::<mjLogMessage>() == 1064);

/// C struct: mjMap (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjMap {
    pub key: *const i8,
    pub value: i32,
    pub _pad_0: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<mjMap>() == 16);

/// C struct: mjModel (5512 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjModel {
    pub nq: i64,
    pub nv: i64,
    pub nu: i64,
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
    pub nuserdata: i64,
    pub nsensordata: i64,
    pub npluginstate: i64,
    pub nhistory: i64,
    pub narena: i64,
    pub nbuffer: i64,
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
    pub geom_fluid: *mut f64,
    pub geom_user: *mut f64,
    pub geom_rgba: *mut f32,
    pub site_type: *mut i32,
    pub site_bodyid: *mut i32,
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
    pub actuator_trnid: *mut i32,
    pub actuator_damping: *mut f64,
    pub actuator_dampingpoly: *mut f64,
    pub actuator_armature: *mut f64,
    pub actuator_actadr: *mut i32,
    pub actuator_actnum: *mut i32,
    pub actuator_group: *mut i32,
    pub actuator_history: *mut i32,
    pub actuator_historyadr: *mut i32,
    pub actuator_delay: *mut f64,
    pub actuator_ctrllimited: *mut bool,
    pub actuator_forcelimited: *mut bool,
    pub actuator_actlimited: *mut bool,
    pub actuator_dynprm: *mut f64,
    pub actuator_gainprm: *mut f64,
    pub actuator_biasprm: *mut f64,
    pub actuator_actearly: *mut bool,
    pub actuator_ctrlrange: *mut f64,
    pub actuator_forcerange: *mut f64,
    pub actuator_actrange: *mut f64,
    pub actuator_gear: *mut f64,
    pub actuator_cranklength: *mut f64,
    pub actuator_acc0: *mut f64,
    pub actuator_length0: *mut f64,
    pub actuator_lengthrange: *mut f64,
    pub actuator_user: *mut f64,
    pub actuator_plugin: *mut i32,
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
const _: () = assert!(std::mem::size_of::<mjModel>() == 5512);

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
const _: () = assert!(std::mem::size_of::<mjOption>() == 304);

/// C struct: mjPreContact (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjPreContact {
    pub dist: f64,
    pub pos: [f64; 3],
    pub normal: [f64; 3],
    pub tangent: [f64; 3],
}
const _: () = assert!(std::mem::size_of::<mjPreContact>() == 80);

/// C struct: mjPrimalPnt (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjPrimalPnt {
    pub alpha: f64,
    pub cost: f64,
    pub deriv: [f64; 2],
}
const _: () = assert!(std::mem::size_of::<mjPrimalPnt>() == 32);

/// C struct: mjResource (544 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjResource {
    pub name: *mut i8,
    pub data: *mut (),
    pub vfs: *mut mjVFS,
    pub timestamp: [i8; 512],
    pub provider: *const struct_mjpResourceProvider,
}
const _: () = assert!(std::mem::size_of::<mjResource>() == 544);

/// C struct: mjSDF (48 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjSDF {
    pub plugin: *const *mut mjpPlugin,
    pub id: *mut i32,
    pub r#type: [u8; 8],
    pub relpos: *mut f64,
    pub relmat: *mut f64,
    pub geomtype: *mut u32,
}
const _: () = assert!(std::mem::size_of::<mjSDF>() == 48);

/// C struct: mjSpec (1352 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjSpec {
    pub element: *mut mjsElement,
    pub modelname: *mut mjString,
    pub compiler: mjsCompiler,
    pub strippath: u8,
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
    pub hasImplicitPluginElem: u8,
    pub _pad_2: [u8; 7],
    pub authored: mjsAuthored,
}
const _: () = assert!(std::mem::size_of::<mjSpec>() == 1352);

/// C struct: mjStackInfo (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjStackInfo {
    pub bottom: usize,
    pub top: usize,
    pub limit: usize,
    pub stack_base: usize,
}
const _: () = assert!(std::mem::size_of::<mjStackInfo>() == 32);

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
const _: () = assert!(std::mem::size_of::<mjStatistic>() == 56);

/// C struct: mjUI (3009832 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjUI {
    pub spacing: [u8; 52],
    pub color: [u8; 340],
    pub predicate: mjfItemEnable,
    pub userdata: *mut (),
    pub rectid: i32,
    pub auxid: i32,
    pub radiocol: i32,
    pub width: i32,
    pub height: i32,
    pub maxheight: i32,
    pub scroll: i32,
    pub mousesect: i32,
    pub mouseitem: i32,
    pub mousehelp: i32,
    pub mouseclicks: i32,
    pub mousesectcheck: i32,
    pub editsect: i32,
    pub edititem: i32,
    pub editcursor: i32,
    pub editscroll: i32,
    pub edittext: [u8; 304],
    pub editchanged: *mut mjuiItem,
    pub nsect: i32,
    pub _pad_0: [u8; 4],
    pub sect: [mjuiSection; 10],
}
const _: () = assert!(std::mem::size_of::<mjUI>() == 3009832);

/// C struct: mjVFS (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjVFS {
    pub impl_: *mut (),
}
const _: () = assert!(std::mem::size_of::<mjVFS>() == 8);

/// C struct: mjVisual (632 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjVisual {
    pub global: [u8; 52],
    pub quality: [u8; 20],
    pub headlight: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_156_3,
    pub map: [u8; 52],
    pub scale: [u8; 68],
    pub rgba: struct__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjmodel_h_199_3,
}
const _: () = assert!(std::mem::size_of::<mjVisual>() == 632);

/// C struct: mjXBase (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjXBase {
    pub _vtable: *mut (),
    pub spec: *mut mjSpec,
}
const _: () = assert!(std::mem::size_of::<mjXBase>() == 16);

/// C struct: mjXReader (232 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjXReader {
    pub _vtable: *mut (),
    pub spec: *mut mjSpec,
    pub schema: mjXSchema,
    pub readingdefaults: bool,
    pub _pad_0: [u8; 7],
    pub modelfiledir_: mujoco__user__FilePath,
    pub assetdir_: mujoco__user__FilePath,
    pub meshdir_: mujoco__user__FilePath,
    pub texturedir_: mujoco__user__FilePath,
}
const _: () = assert!(std::mem::size_of::<mjXReader>() == 232);

/// C struct: mjXSchema (112 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjXSchema {
    pub name_: std__string,
    pub type_: i8,
    pub _pad_0: [u8; 7],
    pub attr_: *const (),
    pub _pad_1: [u8; 16],
    pub subschema_: *const (),
    pub _pad_2: [u8; 16],
    pub refcnt_: i32,
    pub _pad_3: [u8; 4],
    pub error: std__string,
}
const _: () = assert!(std::mem::size_of::<mjXSchema>() == 112);

/// C struct: mjXURDF (224 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjXURDF {
    pub _vtable: *mut (),
    pub spec: *mut mjSpec,
    pub urName: *const (),
    pub _pad_0: [u8; 16],
    pub urParent: *const (),
    pub _pad_1: [u8; 16],
    pub urChildren: *const (),
    pub _pad_2: [u8; 16],
    pub urMat: *const (),
    pub _pad_3: [u8; 16],
    pub urRGBA: [u8; 24],
    pub urGeomNames: [u8; 40],
    pub meshes: [u8; 24],
    pub urPrefix: std__string,
}
const _: () = assert!(std::mem::size_of::<mjXURDF>() == 224);

/// C struct: mjXUtil (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjXUtil {
    pub _vtable: *mut (),
}
const _: () = assert!(std::mem::size_of::<mjXUtil>() == 8);

/// C struct: mjXWriter (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjXWriter {
    pub _vtable: *mut (),
    pub spec: *mut mjSpec,
    pub model: *mut mjCModel,
    pub writingdefaults: bool,
    pub _pad_0: [u8; 7],
}
const _: () = assert!(std::mem::size_of::<mjXWriter>() == 32);

/// C struct: mj_XMLPrinter (328 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mj_XMLPrinter {
    pub _vtable: *mut (),
    pub _elementJustOpened: bool,
    pub _pad_0: [u8; 7],
    pub _stack: [u8; 104],
    pub _firstElement: bool,
    pub _pad_1: [u8; 7],
    pub _fp: *mut FILE,
    pub _depth: i32,
    pub _textDepth: i32,
    pub _processEntities: bool,
    pub _compactMode: bool,
    pub _entityFlag: [u8; 64],
    pub _restrictedEntityFlag: [u8; 70],
    pub _buffer: [u8; 48],
}
const _: () = assert!(std::mem::size_of::<mj_XMLPrinter>() == 328);

/// C struct: mjpDecoder (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjpDecoder {
    pub content_type: *const i8,
    pub extension: *const i8,
    pub can_decode: mjfCanDecode,
    pub decode: mjfDecode,
}
const _: () = assert!(std::mem::size_of::<mjpDecoder>() == 32);

/// C struct: mjpEncoder (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjpEncoder {
    pub content_type: *const i8,
    pub extension: *const i8,
    pub encode: mjfEncode,
    pub close_resource: mjfCloseResource,
}
const _: () = assert!(std::mem::size_of::<mjpEncoder>() == 32);

/// C struct: mjpPlugin_ (152 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjpPlugin {
    pub name: *const i8,
    pub nattribute: i32,
    pub _pad_0: [u8; 4],
    pub attributes: *const char__const,
    pub capabilityflags: i32,
    pub needstage: i32,
    pub nstate: Option<unsafe extern "C" fn()>,
    pub nsensordata: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn()>,
    pub destroy: Option<unsafe extern "C" fn()>,
    pub copy: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub compute: Option<unsafe extern "C" fn()>,
    pub advance: Option<unsafe extern "C" fn()>,
    pub visualize: Option<unsafe extern "C" fn()>,
    pub actuator_act_dot: Option<unsafe extern "C" fn()>,
    pub sdf_distance: Option<unsafe extern "C" fn()>,
    pub sdf_gradient: Option<unsafe extern "C" fn()>,
    pub sdf_staticdistance: Option<unsafe extern "C" fn()>,
    pub sdf_attribute: Option<unsafe extern "C" fn()>,
    pub sdf_aabb: Option<unsafe extern "C" fn()>,
}
const _: () = assert!(std::mem::size_of::<mjpPlugin>() == 152);

/// C struct: mjpResourceProvider (64 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjpResourceProvider {
    pub prefix: *const i8,
    pub open: mjfOpenResource,
    pub read: mjfReadResource,
    pub close: mjfCloseResource,
    pub mount: mjfMountResource,
    pub unmount: mjfUnmountResource,
    pub modified: mjfResourceModified,
    pub data: *mut (),
}
const _: () = assert!(std::mem::size_of::<mjpResourceProvider>() == 64);

/// C struct: mjrContext (61512 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjrContext {
    pub lineWidth: f32,
    pub shadowClip: f32,
    pub shadowScale: f32,
    pub fogStart: f32,
    pub fogEnd: f32,
    pub fogRGBA: [f32; 4],
    pub shadowSize: i32,
    pub offWidth: i32,
    pub offHeight: i32,
    pub offSamples: i32,
    pub fontScale: i32,
    pub auxWidth: [i32; 10],
    pub auxHeight: [i32; 10],
    pub auxSamples: [i32; 10],
    pub offFBO: u32,
    pub offFBO_r: u32,
    pub offColor: u32,
    pub offColor_r: u32,
    pub offDepthStencil: u32,
    pub offDepthStencil_r: u32,
    pub shadowFBO: u32,
    pub shadowTex: u32,
    pub auxFBO: [u32; 10],
    pub auxFBO_r: [u32; 10],
    pub auxColor: [u32; 10],
    pub auxColor_r: [u32; 10],
    pub mat_texid: [i32; 10000],
    pub mat_texuniform: [i32; 1000],
    pub mat_texrepeat: [f32; 2000],
    pub ntexture: i32,
    pub textureType: [i32; 1000],
    pub texture: [u32; 1000],
    pub basePlane: u32,
    pub baseMesh: u32,
    pub baseHField: u32,
    pub baseBuiltin: u32,
    pub baseFontNormal: u32,
    pub baseFontShadow: u32,
    pub baseFontBig: u32,
    pub rangePlane: i32,
    pub rangeMesh: i32,
    pub rangeHField: i32,
    pub rangeBuiltin: i32,
    pub rangeFont: i32,
    pub nskin: i32,
    pub skinvertVBO: *mut u32,
    pub skinnormalVBO: *mut u32,
    pub skintexcoordVBO: *mut u32,
    pub skinfaceVBO: *mut u32,
    pub charWidth: [i32; 127],
    pub charWidthBig: [i32; 127],
    pub charHeight: i32,
    pub charHeightBig: i32,
    pub glInitialized: i32,
    pub windowAvailable: i32,
    pub windowSamples: i32,
    pub windowStereo: i32,
    pub windowDoublebuffer: i32,
    pub currentBuffer: i32,
    pub readPixelFormat: i32,
    pub readDepthMap: i32,
}
const _: () = assert!(std::mem::size_of::<mjrContext>() == 61512);

/// C struct: mjrRect (16 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjrRect {
    pub left: i32,
    pub bottom: i32,
    pub width: i32,
    pub height: i32,
}
const _: () = assert!(std::mem::size_of::<mjrRect>() == 16);

/// C struct: mjsActuator (576 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsActuator {
    pub element: *mut mjsElement,
    pub gaintype: [u8; 8],
    pub gainprm: [f64; 10],
    pub biastype: [u8; 8],
    pub biasprm: [f64; 10],
    pub dyntype: [u8; 8],
    pub dynprm: [f64; 10],
    pub actdim: i32,
    pub actearly: u8,
    pub _pad_0: [u8; 3],
    pub trntype: [u8; 8],
    pub gear: [f64; 6],
    pub target: *mut mjString,
    pub refsite: *mut mjString,
    pub slidersite: *mut mjString,
    pub cranklength: f64,
    pub lengthrange: [f64; 2],
    pub inheritrange: f64,
    pub damping: [f64; 3],
    pub armature: f64,
    pub ctrllimited: i32,
    pub _pad_1: [u8; 4],
    pub ctrlrange: [f64; 2],
    pub forcelimited: i32,
    pub _pad_2: [u8; 4],
    pub forcerange: [f64; 2],
    pub actlimited: i32,
    pub _pad_3: [u8; 4],
    pub actrange: [f64; 2],
    pub group: i32,
    pub nsample: i32,
    pub interp: i32,
    pub _pad_4: [u8; 4],
    pub delay: f64,
    pub userdata: *mut mjDoubleVec,
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsActuator>() == 576);

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
    pub mocap: u8,
    pub _pad_0: [u8; 7],
    pub gravcomp: f64,
    pub sleep: [u8; 8],
    pub userdata: *mut mjDoubleVec,
    pub explicitinertial: u8,
    pub _pad_1: [u8; 7],
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsBody>() == 568);

/// C struct: mjsCamera (320 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsCamera {
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub mode: [u8; 8],
    pub targetbody: *mut mjString,
    pub proj: [u8; 4],
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

/// C struct: mjsCompiler (168 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsCompiler {
    pub autolimits: u8,
    pub _pad_0: [u8; 7],
    pub boundmass: f64,
    pub boundinertia: f64,
    pub settotalmass: f64,
    pub balanceinertia: u8,
    pub fitaabb: u8,
    pub degree: u8,
    pub eulerseq: [i8; 3],
    pub discardvisual: u8,
    pub usethread: u8,
    pub fusestatic: u8,
    pub _pad_1: [u8; 3],
    pub inertiafromgeom: i32,
    pub inertiagrouprange: [i32; 2],
    pub saveinertial: u8,
    pub _pad_2: [u8; 3],
    pub alignfree: i32,
    pub conflict: i32,
    pub _pad_3: [u8; 4],
    pub LRopt: mjLROpt,
    pub meshdir: *mut mjString,
    pub texturedir: *mut mjString,
    pub authored: u64,
}
const _: () = assert!(std::mem::size_of::<mjsCompiler>() == 168);

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

/// C struct: mjsElement (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsElement {
    pub elemtype: [u8; 8],
    pub signature: u64,
}
const _: () = assert!(std::mem::size_of::<mjsElement>() == 16);

/// C struct: mjsEquality (200 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsEquality {
    pub element: *mut mjsElement,
    pub r#type: [u8; 8],
    pub data: [f64; 11],
    pub active: u8,
    pub _pad_0: [u8; 7],
    pub name1: *mut mjString,
    pub name2: *mut mjString,
    pub objtype: [u8; 8],
    pub solref: [f64; 2],
    pub solimp: [f64; 5],
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsEquality>() == 200);

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
    pub internal: u8,
    pub flatskin: u8,
    pub _pad_1: [u8; 2],
    pub selfcollide: i32,
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
const _: () = assert!(std::mem::size_of::<mjsFrame>() == 216);

/// C struct: mjsGeom (584 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsGeom {
    pub element: *mut mjsElement,
    pub r#type: [u8; 8],
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
    pub mass: f64,
    pub density: f64,
    pub typeinertia: [u8; 8],
    pub fluid_ellipsoid: f64,
    pub fluid_coefs: [f64; 5],
    pub material: *mut mjString,
    pub rgba: [f32; 4],
    pub group: i32,
    pub _pad_0: [u8; 4],
    pub hfieldname: *mut mjString,
    pub meshname: *mut mjString,
    pub fitscale: f64,
    pub userdata: *mut mjDoubleVec,
    pub plugin: mjsPlugin,
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsGeom>() == 584);

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
const _: () = assert!(std::mem::size_of::<mjsHField>() == 80);

/// C struct: mjsJoint (360 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsJoint {
    pub element: *mut mjsElement,
    pub r#type: [u8; 8],
    pub pos: [f64; 3],
    pub axis: [f64; 3],
    pub r#ref: f64,
    pub align: i32,
    pub _pad_0: [u8; 4],
    pub stiffness: [f64; 3],
    pub springref: f64,
    pub springdamper: [f64; 2],
    pub limited: i32,
    pub _pad_1: [u8; 4],
    pub range: [f64; 2],
    pub margin: f64,
    pub solref_limit: [f64; 2],
    pub solimp_limit: [f64; 5],
    pub actfrclimited: i32,
    pub _pad_2: [u8; 4],
    pub actfrcrange: [f64; 2],
    pub armature: f64,
    pub damping: [f64; 3],
    pub frictionloss: f64,
    pub solref_friction: [f64; 2],
    pub solimp_friction: [f64; 5],
    pub group: i32,
    pub actgravcomp: u8,
    pub _pad_3: [u8; 3],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsJoint>() == 360);

/// C struct: mjsKey (72 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsKey {
    pub element: *mut mjsElement,
    pub time: f64,
    pub qpos: *mut mjDoubleVec,
    pub qvel: *mut mjDoubleVec,
    pub act: *mut mjDoubleVec,
    pub mpos: *mut mjDoubleVec,
    pub mquat: *mut mjDoubleVec,
    pub ctrl: *mut mjDoubleVec,
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsKey>() == 72);

/// C struct: mjsLight (168 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsLight {
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub dir: [f64; 3],
    pub mode: [u8; 8],
    pub targetbody: *mut mjString,
    pub active: u8,
    pub _pad_0: [u8; 3],
    pub r#type: [u8; 4],
    pub texture: *mut mjString,
    pub castshadow: u8,
    pub _pad_1: [u8; 3],
    pub bulbradius: f32,
    pub intensity: f32,
    pub range: f32,
    pub attenuation: [f32; 3],
    pub cutoff: f32,
    pub exponent: f32,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsLight>() == 168);

/// C struct: mjsMaterial (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsMaterial {
    pub element: *mut mjsElement,
    pub textures: *mut mjStringVec,
    pub texuniform: u8,
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
    pub inertia: [u8; 4],
    pub smoothnormal: u8,
    pub needsdf: u8,
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
const _: () = assert!(std::mem::size_of::<mjsMesh>() == 232);

/// C struct: mjsNumeric (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsNumeric {
    pub element: *mut mjsElement,
    pub data: *mut mjDoubleVec,
    pub size: i32,
    pub _pad_0: [u8; 4],
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsNumeric>() == 32);

/// C struct: mjsOrientation (136 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsOrientation {
    pub r#type: [u8; 8],
    pub axisangle: [f64; 4],
    pub xyaxes: [f64; 6],
    pub zaxis: [f64; 3],
    pub euler: [f64; 3],
}
const _: () = assert!(std::mem::size_of::<mjsOrientation>() == 136);

/// C struct: mjsPair (168 bytes, align 8)
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
    pub friction: [f64; 5],
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsPair>() == 168);

/// C struct: mjsPlugin (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsPlugin {
    pub element: *mut mjsElement,
    pub name: *mut mjString,
    pub plugin_name: *mut mjString,
    pub active: u8,
    pub _pad_0: [u8; 7],
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsPlugin>() == 40);

/// C struct: mjsSensor (168 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsSensor {
    pub element: *mut mjsElement,
    pub r#type: [u8; 4],
    pub objtype: [u8; 4],
    pub objname: *mut mjString,
    pub reftype: [u8; 8],
    pub refname: *mut mjString,
    pub intprm: [i32; 3],
    pub datatype: [u8; 4],
    pub needstage: [u8; 4],
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
const _: () = assert!(std::mem::size_of::<mjsSensor>() == 168);

/// C struct: mjsSite (328 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsSite {
    pub element: *mut mjsElement,
    pub pos: [f64; 3],
    pub quat: [f64; 4],
    pub alt: mjsOrientation,
    pub fromto: [f64; 6],
    pub size: [f64; 3],
    pub r#type: [u8; 8],
    pub material: *mut mjString,
    pub group: i32,
    pub rgba: [u8; 20],
    pub userdata: *mut mjDoubleVec,
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsSite>() == 328);

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
const _: () = assert!(std::mem::size_of::<mjsSkin>() == 120);

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
    pub limited: i32,
    pub actfrclimited: i32,
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

/// C struct: mjsText (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsText {
    pub element: *mut mjsElement,
    pub data: *mut mjString,
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsText>() == 24);

/// C struct: mjsTexture (192 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsTexture {
    pub element: *mut mjsElement,
    pub r#type: [u8; 4],
    pub colorspace: [u8; 4],
    pub builtin: i32,
    pub mark: i32,
    pub rgb1: [f64; 3],
    pub rgb2: [f64; 3],
    pub markrgb: [f64; 3],
    pub random: f64,
    pub height: i32,
    pub width: i32,
    pub nchannel: i32,
    pub _pad_0: [u8; 4],
    pub content_type: *mut mjString,
    pub file: *mut mjString,
    pub gridsize: [i32; 2],
    pub gridlayout: [u8; 16],
    pub cubefiles: *mut mjStringVec,
    pub data: *mut mjByteVec,
    pub hflip: u8,
    pub vflip: u8,
    pub _pad_1: [u8; 6],
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsTexture>() == 192);

/// C struct: mjsTuple (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsTuple {
    pub element: *mut mjsElement,
    pub objtype: *mut mjIntVec,
    pub objname: *mut mjStringVec,
    pub objprm: *mut mjDoubleVec,
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsTuple>() == 40);

/// C struct: mjsWrap (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsWrap {
    pub element: *mut mjsElement,
    pub r#type: [u8; 8],
    pub info: *mut mjString,
}
const _: () = assert!(std::mem::size_of::<mjsWrap>() == 24);

/// C struct: mjtSAP (8 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjtSAP {
    pub value: f32,
    pub id_ismax: i32,
}
const _: () = assert!(std::mem::size_of::<mjtSAP>() == 8);

/// C struct: mjuiDef (360 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiDef {
    pub r#type: i32,
    pub name: [i8; 40],
    pub state: i32,
    pub pdata: *mut (),
    pub other: [i8; 300],
    pub otherint: i32,
}
const _: () = assert!(std::mem::size_of::<mjuiDef>() == 360);

/// C struct: mjuiItem (1504 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiItem {
    pub r#type: i32,
    pub name: [i8; 40],
    pub state: i32,
    pub pdata: *mut (),
    pub sectionid: i32,
    pub itemid: i32,
    pub userid: i32,
    pub _pad_0: [u8; 4],
    pub __anon_7: mjuiItem____anonymous_union_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_include_mujoco_mjui_h_256_3,
    pub rect: mjrRect,
    pub skip: i32,
    pub _pad_1: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<mjuiItem>() == 1504);

/// C struct: mjuiSection (300904 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiSection {
    pub name: [i8; 40],
    pub state: i32,
    pub modifier: i32,
    pub shortcut: i32,
    pub checkbox: i32,
    pub nitem: i32,
    pub _pad_0: [u8; 4],
    pub item: [mjuiItem; 200],
    pub rtitle: mjrRect,
    pub rcontent: mjrRect,
    pub lastclick: i32,
    pub _pad_1: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<mjuiSection>() == 300904);

/// C struct: mjuiState (544 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiState {
    pub nrect: i32,
    pub rect: [u8; 404],
    pub userdata: *mut (),
    pub r#type: i32,
    pub left: i32,
    pub right: i32,
    pub middle: i32,
    pub doubleclick: i32,
    pub button: i32,
    pub buttontime: f64,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub sx: f64,
    pub sy: f64,
    pub control: i32,
    pub shift: i32,
    pub alt: i32,
    pub key: i32,
    pub keytime: f64,
    pub mouserect: i32,
    pub dragrect: i32,
    pub dragbutton: i32,
    pub dropcount: i32,
    pub droppaths: *const *mut i8,
}
const _: () = assert!(std::mem::size_of::<mjuiState>() == 544);

/// C struct: mjvCamera (72 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjvCamera {
    pub r#type: i32,
    pub fixedcamid: i32,
    pub trackbodyid: i32,
    pub _pad_0: [u8; 4],
    pub lookat: [f64; 3],
    pub distance: f64,
    pub azimuth: f64,
    pub elevation: f64,
    pub orthographic: i32,
    pub _pad_1: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<mjvCamera>() == 72);

/// C struct: mjvFigure (813748 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvFigure {
    pub flg_legend: i32,
    pub flg_ticklabel: [i32; 2],
    pub flg_extend: i32,
    pub flg_barplot: i32,
    pub flg_selection: i32,
    pub flg_symmetric: i32,
    pub linewidth: f32,
    pub gridwidth: f32,
    pub gridsize: [i32; 2],
    pub gridrgb: [f32; 3],
    pub figurergba: [f32; 4],
    pub panergba: [f32; 4],
    pub legendrgba: [f32; 4],
    pub textrgb: [f32; 3],
    pub linergb: [[f32; 100]; 3],
    pub range: [[f32; 2]; 2],
    pub xformat: [i8; 20],
    pub yformat: [i8; 20],
    pub minwidth: [i8; 20],
    pub title: [i8; 1000],
    pub xlabel: [i8; 100],
    pub linename: [[i8; 100]; 100],
    pub legendoffset: i32,
    pub subplot: i32,
    pub highlight: [i32; 2],
    pub highlightid: i32,
    pub selection: f32,
    pub linepnt: [i32; 100],
    pub linedata: [[f32; 100]; 2002],
    pub xaxispixel: [i32; 2],
    pub yaxispixel: [i32; 2],
    pub xaxisdata: [f32; 2],
    pub yaxisdata: [f32; 2],
}
const _: () = assert!(std::mem::size_of::<mjvFigure>() == 813748);

/// C struct: mjvGLCamera (64 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvGLCamera {
    pub pos: [f32; 3],
    pub forward: [f32; 3],
    pub up: [f32; 3],
    pub frustum_center: f32,
    pub frustum_width: f32,
    pub frustum_bottom: f32,
    pub frustum_top: f32,
    pub frustum_near: f32,
    pub frustum_far: f32,
    pub orthographic: i32,
}
const _: () = assert!(std::mem::size_of::<mjvGLCamera>() == 64);

/// C struct: mjvGeom (236 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvGeom {
    pub r#type: i32,
    pub dataid: i32,
    pub objtype: i32,
    pub objid: i32,
    pub category: i32,
    pub matid: i32,
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
    pub label: [i8; 100],
    pub camdist: f32,
    pub modelrbound: f32,
    pub transparent: u8,
    pub _pad_0: [u8; 3],
}
const _: () = assert!(std::mem::size_of::<mjvGeom>() == 236);

/// C struct: mjvLight (108 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvLight {
    pub id: i32,
    pub pos: [f32; 3],
    pub dir: [f32; 3],
    pub r#type: i32,
    pub texid: i32,
    pub attenuation: [f32; 3],
    pub cutoff: f32,
    pub exponent: f32,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub headlight: u8,
    pub castshadow: u8,
    pub _pad_0: [u8; 2],
    pub bulbradius: f32,
    pub intensity: f32,
    pub range: f32,
}
const _: () = assert!(std::mem::size_of::<mjvLight>() == 108);

/// C struct: mjvOption (92 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvOption {
    pub label: i32,
    pub frame: i32,
    pub geomgroup: [u8; 6],
    pub sitegroup: [u8; 6],
    pub jointgroup: [u8; 6],
    pub tendongroup: [u8; 6],
    pub actuatorgroup: [u8; 6],
    pub flexgroup: [u8; 6],
    pub skingroup: [u8; 6],
    pub flags: [u8; 34],
    pub bvh_depth: i32,
    pub flex_layer: i32,
}
const _: () = assert!(std::mem::size_of::<mjvOption>() == 92);

/// C struct: mjvPerturb (144 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjvPerturb {
    pub select: i32,
    pub flexselect: i32,
    pub skinselect: i32,
    pub active: i32,
    pub active2: i32,
    pub _pad_0: [u8; 4],
    pub refpos: [f64; 3],
    pub refquat: [f64; 4],
    pub refselpos: [f64; 3],
    pub localpos: [f64; 3],
    pub localmass: f64,
    pub scale: f64,
}
const _: () = assert!(std::mem::size_of::<mjvPerturb>() == 144);

/// C struct: mjvScene (11184 bytes, align 8)
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
    pub lights: [mjvLight; 100],
    pub camera: [mjvGLCamera; 2],
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
const _: () = assert!(std::mem::size_of::<mjvScene>() == 11184);

/// C struct: mujoco::ReentrantWriteLock (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mujoco__ReentrantWriteLock {
    pub mutex_: *mut Mutex,
}
const _: () = assert!(std::mem::size_of::<mujoco__ReentrantWriteLock>() == 8);

/// C struct: mujoco::user::FilePath (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mujoco__user__FilePath {
    pub path_: std__string,
}
const _: () = assert!(std::mem::size_of::<mujoco__user__FilePath>() == 24);

/// C struct: mujoco::user::VFS (792 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mujoco__user__VFS {
    pub wrapped_vfs_: mjVFS,
    pub mutex_: std__mutex,
    pub open_resources_: *const (),
    pub _pad_0: [u8; 32],
    pub mounts_: *const (),
    pub _pad_1: [u8; 32],
    pub default_mount_: mjResource,
    pub default_provider_: mjpResourceProvider,
    pub destructor_: *const (),
    pub _pad_2: [u8; 24],
}
const _: () = assert!(std::mem::size_of::<mujoco__user__VFS>() == 792);

/// C struct: std::exception_ptr (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__exception_ptr {
    pub __ptr_: *mut (),
}
const _: () = assert!(std::mem::size_of::<std__exception_ptr>() == 8);

/// C struct: std::filesystem::path (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__filesystem__path {
    pub __pn_: string_type,
}
const _: () = assert!(std::mem::size_of::<std__filesystem__path>() == 24);

/// C struct: std::mutex (64 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct std__mutex {
    pub __m_: libcpp_mutex_t,
}
const _: () = assert!(std::mem::size_of::<std__mutex>() == 64);

/// C struct: tinyxml2::XMLElement (120 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLElement {
    pub _vtable: *mut (),
    pub _document: *mut XMLDocument,
    pub _parent: *mut XMLNode,
    pub _value: StrPair,
    pub _parseLineNum: i32,
    pub _pad_0: [u8; 4],
    pub _firstChild: *mut XMLNode,
    pub _lastChild: *mut XMLNode,
    pub _prev: *mut XMLNode,
    pub _next: *mut XMLNode,
    pub _userData: *mut (),
    pub _memPool: *mut MemPool,
    pub _closingType: [u8; 8],
    pub _rootAttribute: *mut XMLAttribute,
}
const _: () = assert!(std::mem::size_of::<tinyxml2__XMLElement>() == 120);

// ═══════════════════════════════════════════════════════════
// Static variables (class statics / file-scope globals)
// ═══════════════════════════════════════════════════════════

/// C static: ID (const int)
pub static ID: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: IDENTITY (const mjtNum[9])
pub static IDENTITY: std::sync::LazyLock<std::sync::Mutex<[u8; 72]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 72]));

/// C static: MAX_ARRAY_SIZE (const int)
pub static MAX_ARRAY_SIZE: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: RK4_A (const mjtNum[9])
pub static RK4_A: std::sync::LazyLock<std::sync::Mutex<[u8; 72]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 72]));

/// C static: RK4_B (const mjtNum[4])
pub static RK4_B: std::sync::LazyLock<std::sync::Mutex<[u8; 32]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 32]));

/// C static: Stencil2D::edge (const int[3][2])
pub static STENCIL2D_EDGE: std::sync::LazyLock<std::sync::Mutex<[u8; 24]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 24]));

/// C static: Stencil2D::edge2face (const int[3][2])
pub static STENCIL2D_EDGE2FACE: std::sync::LazyLock<std::sync::Mutex<[u8; 24]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 24]));

/// C static: Stencil2D::face (const int[3][2])
pub static STENCIL2D_FACE: std::sync::LazyLock<std::sync::Mutex<[u8; 24]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 24]));

/// C static: Stencil2D::kNumEdges (const int)
pub static STENCIL2D_KNUMEDGES: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: Stencil2D::kNumFaces (const int)
pub static STENCIL2D_KNUMFACES: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: Stencil2D::kNumVerts (const int)
pub static STENCIL2D_KNUMVERTS: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: Stencil3D::edge (const int[6][2])
pub static STENCIL3D_EDGE: std::sync::LazyLock<std::sync::Mutex<[u8; 48]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 48]));

/// C static: Stencil3D::edge2face (const int[6][2])
pub static STENCIL3D_EDGE2FACE: std::sync::LazyLock<std::sync::Mutex<[u8; 48]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 48]));

/// C static: Stencil3D::face (const int[4][3])
pub static STENCIL3D_FACE: std::sync::LazyLock<std::sync::Mutex<[u8; 48]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 48]));

/// C static: Stencil3D::kNumEdges (const int)
pub static STENCIL3D_KNUMEDGES: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: Stencil3D::kNumFaces (const int)
pub static STENCIL3D_KNUMFACES: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: Stencil3D::kNumVerts (const int)
pub static STENCIL3D_KNUMVERTS: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: StencilFlap::kNumVerts (const int)
pub static STENCILFLAP_KNUMVERTS: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: TableBlock::kBlockSize (const int)
pub static TABLEBLOCK_KBLOCKSIZE: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: ThreadPool::worker_id_ (int)
pub static THREADPOOL_WORKER_ID_: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: _linergb (const float[8][3])
pub static _LINERGB: std::sync::LazyLock<std::sync::Mutex<[u8; 96]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 96]));

/// C static: _mjPRIVATE_tls_log_handler (mjfLogHandler)
pub static _MJPRIVATE_TLS_LOG_HANDLER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: bias_sz (const int)
pub static BIAS_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: bodysleep_sz (const int)
pub static BODYSLEEP_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: builtin_sz (const int)
pub static BUILTIN_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: camlight_sz (const int)
pub static CAMLIGHT_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: camout_sz (const int)
pub static CAMOUT_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: ccd_buffer (void *)
pub static CCD_BUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: collision_sz (const int)
pub static COLLISION_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: colorspace_sz (const int)
pub static COLORSPACE_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: comp_map (const mjMap[6])
pub static COMP_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 96]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 96]));

/// C static: cone_sz (const int)
pub static CONE_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: conflict_sz (const int)
pub static CONFLICT_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: coordinate_map (const mjMap[2])
pub static COORDINATE_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 32]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 32]));

/// C static: datatype_sz (const int)
pub static DATATYPE_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: dcmotorinput_map (const mjMap[3])
pub static DCMOTORINPUT_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 48]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 48]));

/// C static: dcmotorinput_sz (const int)
pub static DCMOTORINPUT_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: dyn_sz (const int)
pub static DYN_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: edge (const int[3][2])
pub static EDGE: std::sync::LazyLock<std::sync::Mutex<[u8; 24]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 24]));

/// C static: edges (const int[2][6][2])
pub static EDGES: std::sync::LazyLock<std::sync::Mutex<[u8; 96]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 96]));

/// C static: eigEPS (const mjtNum)
pub static EIGEPS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: eledge (const int[3][6][2])
pub static ELEDGE: std::sync::LazyLock<std::sync::Mutex<[u8; 144]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 144]));

/// C static: env_checked (mjtBool)
pub static ENV_CHECKED: std::sync::LazyLock<std::sync::Mutex<[u8; 1]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 1]));

/// C static: equality_sz (const int)
pub static EQUALITY_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: error_jmp_buf (std::jmp_buf)
pub static ERROR_JMP_BUF: std::sync::LazyLock<std::sync::Mutex<[u8; 192]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 192]));

/// C static: errortext (char[500])
pub static ERRORTEXT: std::sync::LazyLock<std::sync::Mutex<[u8; 500]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 500]));

/// C static: fcomp_map (const mjMap[10])
pub static FCOMP_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 160]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 160]));

/// C static: fdof_map (const mjMap[5])
pub static FDOF_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 80]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 80]));

/// C static: flexeq_map (const mjMap[4])
pub static FLEXEQ_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 64]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 64]));

/// C static: font_back100 (const unsigned char[2662])
pub static FONT_BACK100: std::sync::LazyLock<std::sync::Mutex<[u8; 2662]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 2662]));

/// C static: font_back150 (const unsigned char[5102])
pub static FONT_BACK150: std::sync::LazyLock<std::sync::Mutex<[u8; 5102]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 5102]));

/// C static: font_back200 (const unsigned char[8056])
pub static FONT_BACK200: std::sync::LazyLock<std::sync::Mutex<[u8; 8056]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8056]));

/// C static: font_back250 (const unsigned char[12616])
pub static FONT_BACK250: std::sync::LazyLock<std::sync::Mutex<[u8; 12616]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 12616]));

/// C static: font_back300 (const unsigned char[16378])
pub static FONT_BACK300: std::sync::LazyLock<std::sync::Mutex<[u8; 16378]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 16378]));

/// C static: font_back50 (const unsigned char[1375])
pub static FONT_BACK50: std::sync::LazyLock<std::sync::Mutex<[u8; 1375]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 1375]));

/// C static: font_big100 (const unsigned char[8056])
pub static FONT_BIG100: std::sync::LazyLock<std::sync::Mutex<[u8; 8056]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8056]));

/// C static: font_big150 (const unsigned char[16378])
pub static FONT_BIG150: std::sync::LazyLock<std::sync::Mutex<[u8; 16378]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 16378]));

/// C static: font_big200 (const unsigned char[26982])
pub static FONT_BIG200: std::sync::LazyLock<std::sync::Mutex<[u8; 26982]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 26982]));

/// C static: font_big250 (const unsigned char[42205])
pub static FONT_BIG250: std::sync::LazyLock<std::sync::Mutex<[u8; 42205]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 42205]));

/// C static: font_big300 (const unsigned char[59010])
pub static FONT_BIG300: std::sync::LazyLock<std::sync::Mutex<[u8; 59010]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 59010]));

/// C static: font_big50 (const unsigned char[2662])
pub static FONT_BIG50: std::sync::LazyLock<std::sync::Mutex<[u8; 2662]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 2662]));

/// C static: font_normal100 (const unsigned char[2662])
pub static FONT_NORMAL100: std::sync::LazyLock<std::sync::Mutex<[u8; 2662]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 2662]));

/// C static: font_normal150 (const unsigned char[5102])
pub static FONT_NORMAL150: std::sync::LazyLock<std::sync::Mutex<[u8; 5102]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 5102]));

/// C static: font_normal200 (const unsigned char[8056])
pub static FONT_NORMAL200: std::sync::LazyLock<std::sync::Mutex<[u8; 8056]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8056]));

/// C static: font_normal250 (const unsigned char[12616])
pub static FONT_NORMAL250: std::sync::LazyLock<std::sync::Mutex<[u8; 12616]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 12616]));

/// C static: font_normal300 (const unsigned char[16378])
pub static FONT_NORMAL300: std::sync::LazyLock<std::sync::Mutex<[u8; 16378]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 16378]));

/// C static: font_normal50 (const unsigned char[1375])
pub static FONT_NORMAL50: std::sync::LazyLock<std::sync::Mutex<[u8; 1375]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 1375]));

/// C static: gain_sz (const int)
pub static GAIN_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: global_log_handler (mjfLogHandler)
pub static GLOBAL_LOG_HANDLER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: in_log (bool)
pub static IN_LOG: std::sync::LazyLock<std::sync::Mutex<[u8; 1]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 1]));

/// C static: integrator_sz (const int)
pub static INTEGRATOR_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: interp_sz (const int)
pub static INTERP_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: jac_sz (const int)
pub static JAC_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: jkind_map (const mjMap[1])
pub static JKIND_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 16]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 16]));

/// C static: joint_sz (const int)
pub static JOINT_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: kAwake (int)
pub static KAWAKE: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: kCacheLineBytes (const int)
pub static KCACHELINEBYTES: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: kContactStiffness (const mjtNum)
pub static KCONTACTSTIFFNESS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: kEigEPS (const double)
pub static KEIGEPS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: kErrorBufferSize (const int)
pub static KERRORBUFFERSIZE: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: kFrameEps (const double)
pub static KFRAMEEPS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: kGlobalCacheSize (const std::size_t)
pub static KGLOBALCACHESIZE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: kMaxAttributes (const int)
pub static KMAXATTRIBUTES: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: kMaxNameLength (const int)
pub static KMAXNAMELENGTH: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: kNumEdges (const int)
pub static KNUMEDGES: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: kNumVerts (const int)
pub static KNUMVERTS: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: lighttype_sz (const int)
pub static LIGHTTYPE_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: local_warningtext_ptr (std::string *)
pub static LOCAL_WARNINGTEXT_PTR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: log_config (mjLogConfig)
pub static LOG_CONFIG: std::sync::LazyLock<std::sync::Mutex<[u8; 1032]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 1032]));

/// C static: lrmode_map (const mjMap[4])
pub static LRMODE_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 64]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 64]));

/// C static: lrmode_sz (const int)
pub static LRMODE_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mark_sz (const int)
pub static MARK_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: maxplanemesh (const int)
pub static MAXPLANEMESH: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: meshbuiltin_map (const mjMap[8])
pub static MESHBUILTIN_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 128]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 128]));

/// C static: meshbuiltin_sz (const int)
pub static MESHBUILTIN_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjCFlex::kNumEdges (const int[3])
pub static MJCFLEX_KNUMEDGES: std::sync::LazyLock<std::sync::Mutex<[u8; 12]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 12]));

/// C static: mjCOLLISIONFUNC (mjfCollision[9][9])
pub static MJCOLLISIONFUNC: std::sync::LazyLock<std::sync::Mutex<[u8; 648]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 648]));

/// C static: mjCONDATA_SIZE (const int[7])
pub static MJCONDATA_SIZE: std::sync::LazyLock<std::sync::Mutex<[u8; 28]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 28]));

/// C static: mjDISABLESTRING (const char *[20])
pub static MJDISABLESTRING: std::sync::LazyLock<std::sync::Mutex<[u8; 160]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 160]));

/// C static: mjENABLESTRING (const char *[6])
pub static MJENABLESTRING: std::sync::LazyLock<std::sync::Mutex<[u8; 48]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 48]));

/// C static: mjEPS (const double)
pub static MJEPS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjFRAMESTRING (const char *[8])
pub static MJFRAMESTRING: std::sync::LazyLock<std::sync::Mutex<[u8; 64]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 64]));

/// C static: mjGEOMINFO (const int[9])
pub static MJGEOMINFO: std::sync::LazyLock<std::sync::Mutex<[u8; 36]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 36]));

/// C static: mjGLAD_GL_ARB_clip_control (int)
pub static MJGLAD_GL_ARB_CLIP_CONTROL: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_ARB_depth_buffer_float (int)
pub static MJGLAD_GL_ARB_DEPTH_BUFFER_FLOAT: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_ARB_framebuffer_object (int)
pub static MJGLAD_GL_ARB_FRAMEBUFFER_OBJECT: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_ARB_seamless_cube_map (int)
pub static MJGLAD_GL_ARB_SEAMLESS_CUBE_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_ARB_vertex_buffer_object (int)
pub static MJGLAD_GL_ARB_VERTEX_BUFFER_OBJECT: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_EXT_texture_sRGB (int)
pub static MJGLAD_GL_EXT_TEXTURE_SRGB: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_KHR_debug (int)
pub static MJGLAD_GL_KHR_DEBUG: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_VERSION_1_0 (int)
pub static MJGLAD_GL_VERSION_1_0: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_VERSION_1_1 (int)
pub static MJGLAD_GL_VERSION_1_1: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_VERSION_1_2 (int)
pub static MJGLAD_GL_VERSION_1_2: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_VERSION_1_3 (int)
pub static MJGLAD_GL_VERSION_1_3: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_VERSION_1_4 (int)
pub static MJGLAD_GL_VERSION_1_4: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLAD_GL_VERSION_1_5 (int)
pub static MJGLAD_GL_VERSION_1_5: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGLVersion (struct gladGLversionStruct)
pub static MJGLVERSION: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_exts (const char *)
pub static MJGLAD_EXTS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_exts_i (char **)
pub static MJGLAD_EXTS_I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glAccum (PFNGLACCUMPROC)
pub static MJGLAD_GLACCUM: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glActiveTexture (PFNGLACTIVETEXTUREPROC)
pub static MJGLAD_GLACTIVETEXTURE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glAlphaFunc (PFNGLALPHAFUNCPROC)
pub static MJGLAD_GLALPHAFUNC: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glAreTexturesResident (PFNGLARETEXTURESRESIDENTPROC)
pub static MJGLAD_GLARETEXTURESRESIDENT: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glArrayElement (PFNGLARRAYELEMENTPROC)
pub static MJGLAD_GLARRAYELEMENT: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBegin (PFNGLBEGINPROC)
pub static MJGLAD_GLBEGIN: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBeginQuery (PFNGLBEGINQUERYPROC)
pub static MJGLAD_GLBEGINQUERY: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBindBuffer (PFNGLBINDBUFFERPROC)
pub static MJGLAD_GLBINDBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBindBufferARB (PFNGLBINDBUFFERARBPROC)
pub static MJGLAD_GLBINDBUFFERARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBindFramebuffer (PFNGLBINDFRAMEBUFFERPROC)
pub static MJGLAD_GLBINDFRAMEBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBindRenderbuffer (PFNGLBINDRENDERBUFFERPROC)
pub static MJGLAD_GLBINDRENDERBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBindTexture (PFNGLBINDTEXTUREPROC)
pub static MJGLAD_GLBINDTEXTURE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBitmap (PFNGLBITMAPPROC)
pub static MJGLAD_GLBITMAP: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBlendColor (PFNGLBLENDCOLORPROC)
pub static MJGLAD_GLBLENDCOLOR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBlendEquation (PFNGLBLENDEQUATIONPROC)
pub static MJGLAD_GLBLENDEQUATION: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBlendFunc (PFNGLBLENDFUNCPROC)
pub static MJGLAD_GLBLENDFUNC: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBlendFuncSeparate (PFNGLBLENDFUNCSEPARATEPROC)
pub static MJGLAD_GLBLENDFUNCSEPARATE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBlitFramebuffer (PFNGLBLITFRAMEBUFFERPROC)
pub static MJGLAD_GLBLITFRAMEBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBufferData (PFNGLBUFFERDATAPROC)
pub static MJGLAD_GLBUFFERDATA: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBufferDataARB (PFNGLBUFFERDATAARBPROC)
pub static MJGLAD_GLBUFFERDATAARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBufferSubData (PFNGLBUFFERSUBDATAPROC)
pub static MJGLAD_GLBUFFERSUBDATA: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glBufferSubDataARB (PFNGLBUFFERSUBDATAARBPROC)
pub static MJGLAD_GLBUFFERSUBDATAARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCallList (PFNGLCALLLISTPROC)
pub static MJGLAD_GLCALLLIST: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCallLists (PFNGLCALLLISTSPROC)
pub static MJGLAD_GLCALLLISTS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCheckFramebufferStatus (PFNGLCHECKFRAMEBUFFERSTATUSPROC)
pub static MJGLAD_GLCHECKFRAMEBUFFERSTATUS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glClear (PFNGLCLEARPROC)
pub static MJGLAD_GLCLEAR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glClearAccum (PFNGLCLEARACCUMPROC)
pub static MJGLAD_GLCLEARACCUM: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glClearColor (PFNGLCLEARCOLORPROC)
pub static MJGLAD_GLCLEARCOLOR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glClearDepth (PFNGLCLEARDEPTHPROC)
pub static MJGLAD_GLCLEARDEPTH: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glClearIndex (PFNGLCLEARINDEXPROC)
pub static MJGLAD_GLCLEARINDEX: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glClearStencil (PFNGLCLEARSTENCILPROC)
pub static MJGLAD_GLCLEARSTENCIL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glClientActiveTexture (PFNGLCLIENTACTIVETEXTUREPROC)
pub static MJGLAD_GLCLIENTACTIVETEXTURE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glClipControl (PFNGLCLIPCONTROLPROC)
pub static MJGLAD_GLCLIPCONTROL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glClipPlane (PFNGLCLIPPLANEPROC)
pub static MJGLAD_GLCLIPPLANE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3b (PFNGLCOLOR3BPROC)
pub static MJGLAD_GLCOLOR3B: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3bv (PFNGLCOLOR3BVPROC)
pub static MJGLAD_GLCOLOR3BV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3d (PFNGLCOLOR3DPROC)
pub static MJGLAD_GLCOLOR3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3dv (PFNGLCOLOR3DVPROC)
pub static MJGLAD_GLCOLOR3DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3f (PFNGLCOLOR3FPROC)
pub static MJGLAD_GLCOLOR3F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3fv (PFNGLCOLOR3FVPROC)
pub static MJGLAD_GLCOLOR3FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3i (PFNGLCOLOR3IPROC)
pub static MJGLAD_GLCOLOR3I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3iv (PFNGLCOLOR3IVPROC)
pub static MJGLAD_GLCOLOR3IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3s (PFNGLCOLOR3SPROC)
pub static MJGLAD_GLCOLOR3S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3sv (PFNGLCOLOR3SVPROC)
pub static MJGLAD_GLCOLOR3SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3ub (PFNGLCOLOR3UBPROC)
pub static MJGLAD_GLCOLOR3UB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3ubv (PFNGLCOLOR3UBVPROC)
pub static MJGLAD_GLCOLOR3UBV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3ui (PFNGLCOLOR3UIPROC)
pub static MJGLAD_GLCOLOR3UI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3uiv (PFNGLCOLOR3UIVPROC)
pub static MJGLAD_GLCOLOR3UIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3us (PFNGLCOLOR3USPROC)
pub static MJGLAD_GLCOLOR3US: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor3usv (PFNGLCOLOR3USVPROC)
pub static MJGLAD_GLCOLOR3USV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4b (PFNGLCOLOR4BPROC)
pub static MJGLAD_GLCOLOR4B: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4bv (PFNGLCOLOR4BVPROC)
pub static MJGLAD_GLCOLOR4BV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4d (PFNGLCOLOR4DPROC)
pub static MJGLAD_GLCOLOR4D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4dv (PFNGLCOLOR4DVPROC)
pub static MJGLAD_GLCOLOR4DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4f (PFNGLCOLOR4FPROC)
pub static MJGLAD_GLCOLOR4F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4fv (PFNGLCOLOR4FVPROC)
pub static MJGLAD_GLCOLOR4FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4i (PFNGLCOLOR4IPROC)
pub static MJGLAD_GLCOLOR4I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4iv (PFNGLCOLOR4IVPROC)
pub static MJGLAD_GLCOLOR4IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4s (PFNGLCOLOR4SPROC)
pub static MJGLAD_GLCOLOR4S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4sv (PFNGLCOLOR4SVPROC)
pub static MJGLAD_GLCOLOR4SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4ub (PFNGLCOLOR4UBPROC)
pub static MJGLAD_GLCOLOR4UB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4ubv (PFNGLCOLOR4UBVPROC)
pub static MJGLAD_GLCOLOR4UBV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4ui (PFNGLCOLOR4UIPROC)
pub static MJGLAD_GLCOLOR4UI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4uiv (PFNGLCOLOR4UIVPROC)
pub static MJGLAD_GLCOLOR4UIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4us (PFNGLCOLOR4USPROC)
pub static MJGLAD_GLCOLOR4US: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColor4usv (PFNGLCOLOR4USVPROC)
pub static MJGLAD_GLCOLOR4USV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColorMask (PFNGLCOLORMASKPROC)
pub static MJGLAD_GLCOLORMASK: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColorMaterial (PFNGLCOLORMATERIALPROC)
pub static MJGLAD_GLCOLORMATERIAL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glColorPointer (PFNGLCOLORPOINTERPROC)
pub static MJGLAD_GLCOLORPOINTER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCompressedTexImage1D (PFNGLCOMPRESSEDTEXIMAGE1DPROC)
pub static MJGLAD_GLCOMPRESSEDTEXIMAGE1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCompressedTexImage2D (PFNGLCOMPRESSEDTEXIMAGE2DPROC)
pub static MJGLAD_GLCOMPRESSEDTEXIMAGE2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCompressedTexImage3D (PFNGLCOMPRESSEDTEXIMAGE3DPROC)
pub static MJGLAD_GLCOMPRESSEDTEXIMAGE3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCompressedTexSubImage1D (PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC)
pub static MJGLAD_GLCOMPRESSEDTEXSUBIMAGE1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCompressedTexSubImage2D (PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC)
pub static MJGLAD_GLCOMPRESSEDTEXSUBIMAGE2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCompressedTexSubImage3D (PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC)
pub static MJGLAD_GLCOMPRESSEDTEXSUBIMAGE3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCopyPixels (PFNGLCOPYPIXELSPROC)
pub static MJGLAD_GLCOPYPIXELS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCopyTexImage1D (PFNGLCOPYTEXIMAGE1DPROC)
pub static MJGLAD_GLCOPYTEXIMAGE1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCopyTexImage2D (PFNGLCOPYTEXIMAGE2DPROC)
pub static MJGLAD_GLCOPYTEXIMAGE2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCopyTexSubImage1D (PFNGLCOPYTEXSUBIMAGE1DPROC)
pub static MJGLAD_GLCOPYTEXSUBIMAGE1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCopyTexSubImage2D (PFNGLCOPYTEXSUBIMAGE2DPROC)
pub static MJGLAD_GLCOPYTEXSUBIMAGE2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCopyTexSubImage3D (PFNGLCOPYTEXSUBIMAGE3DPROC)
pub static MJGLAD_GLCOPYTEXSUBIMAGE3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glCullFace (PFNGLCULLFACEPROC)
pub static MJGLAD_GLCULLFACE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDebugMessageCallback (PFNGLDEBUGMESSAGECALLBACKPROC)
pub static MJGLAD_GLDEBUGMESSAGECALLBACK: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDebugMessageCallbackKHR (PFNGLDEBUGMESSAGECALLBACKKHRPROC)
pub static MJGLAD_GLDEBUGMESSAGECALLBACKKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDebugMessageControl (PFNGLDEBUGMESSAGECONTROLPROC)
pub static MJGLAD_GLDEBUGMESSAGECONTROL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDebugMessageControlKHR (PFNGLDEBUGMESSAGECONTROLKHRPROC)
pub static MJGLAD_GLDEBUGMESSAGECONTROLKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDebugMessageInsert (PFNGLDEBUGMESSAGEINSERTPROC)
pub static MJGLAD_GLDEBUGMESSAGEINSERT: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDebugMessageInsertKHR (PFNGLDEBUGMESSAGEINSERTKHRPROC)
pub static MJGLAD_GLDEBUGMESSAGEINSERTKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDeleteBuffers (PFNGLDELETEBUFFERSPROC)
pub static MJGLAD_GLDELETEBUFFERS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDeleteBuffersARB (PFNGLDELETEBUFFERSARBPROC)
pub static MJGLAD_GLDELETEBUFFERSARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDeleteFramebuffers (PFNGLDELETEFRAMEBUFFERSPROC)
pub static MJGLAD_GLDELETEFRAMEBUFFERS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDeleteLists (PFNGLDELETELISTSPROC)
pub static MJGLAD_GLDELETELISTS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDeleteQueries (PFNGLDELETEQUERIESPROC)
pub static MJGLAD_GLDELETEQUERIES: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDeleteRenderbuffers (PFNGLDELETERENDERBUFFERSPROC)
pub static MJGLAD_GLDELETERENDERBUFFERS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDeleteTextures (PFNGLDELETETEXTURESPROC)
pub static MJGLAD_GLDELETETEXTURES: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDepthFunc (PFNGLDEPTHFUNCPROC)
pub static MJGLAD_GLDEPTHFUNC: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDepthMask (PFNGLDEPTHMASKPROC)
pub static MJGLAD_GLDEPTHMASK: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDepthRange (PFNGLDEPTHRANGEPROC)
pub static MJGLAD_GLDEPTHRANGE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDisable (PFNGLDISABLEPROC)
pub static MJGLAD_GLDISABLE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDisableClientState (PFNGLDISABLECLIENTSTATEPROC)
pub static MJGLAD_GLDISABLECLIENTSTATE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDrawArrays (PFNGLDRAWARRAYSPROC)
pub static MJGLAD_GLDRAWARRAYS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDrawBuffer (PFNGLDRAWBUFFERPROC)
pub static MJGLAD_GLDRAWBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDrawElements (PFNGLDRAWELEMENTSPROC)
pub static MJGLAD_GLDRAWELEMENTS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDrawPixels (PFNGLDRAWPIXELSPROC)
pub static MJGLAD_GLDRAWPIXELS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glDrawRangeElements (PFNGLDRAWRANGEELEMENTSPROC)
pub static MJGLAD_GLDRAWRANGEELEMENTS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEdgeFlag (PFNGLEDGEFLAGPROC)
pub static MJGLAD_GLEDGEFLAG: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEdgeFlagPointer (PFNGLEDGEFLAGPOINTERPROC)
pub static MJGLAD_GLEDGEFLAGPOINTER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEdgeFlagv (PFNGLEDGEFLAGVPROC)
pub static MJGLAD_GLEDGEFLAGV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEnable (PFNGLENABLEPROC)
pub static MJGLAD_GLENABLE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEnableClientState (PFNGLENABLECLIENTSTATEPROC)
pub static MJGLAD_GLENABLECLIENTSTATE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEnd (PFNGLENDPROC)
pub static MJGLAD_GLEND: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEndList (PFNGLENDLISTPROC)
pub static MJGLAD_GLENDLIST: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEndQuery (PFNGLENDQUERYPROC)
pub static MJGLAD_GLENDQUERY: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalCoord1d (PFNGLEVALCOORD1DPROC)
pub static MJGLAD_GLEVALCOORD1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalCoord1dv (PFNGLEVALCOORD1DVPROC)
pub static MJGLAD_GLEVALCOORD1DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalCoord1f (PFNGLEVALCOORD1FPROC)
pub static MJGLAD_GLEVALCOORD1F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalCoord1fv (PFNGLEVALCOORD1FVPROC)
pub static MJGLAD_GLEVALCOORD1FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalCoord2d (PFNGLEVALCOORD2DPROC)
pub static MJGLAD_GLEVALCOORD2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalCoord2dv (PFNGLEVALCOORD2DVPROC)
pub static MJGLAD_GLEVALCOORD2DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalCoord2f (PFNGLEVALCOORD2FPROC)
pub static MJGLAD_GLEVALCOORD2F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalCoord2fv (PFNGLEVALCOORD2FVPROC)
pub static MJGLAD_GLEVALCOORD2FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalMesh1 (PFNGLEVALMESH1PROC)
pub static MJGLAD_GLEVALMESH1: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalMesh2 (PFNGLEVALMESH2PROC)
pub static MJGLAD_GLEVALMESH2: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalPoint1 (PFNGLEVALPOINT1PROC)
pub static MJGLAD_GLEVALPOINT1: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glEvalPoint2 (PFNGLEVALPOINT2PROC)
pub static MJGLAD_GLEVALPOINT2: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFeedbackBuffer (PFNGLFEEDBACKBUFFERPROC)
pub static MJGLAD_GLFEEDBACKBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFinish (PFNGLFINISHPROC)
pub static MJGLAD_GLFINISH: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFlush (PFNGLFLUSHPROC)
pub static MJGLAD_GLFLUSH: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFogCoordPointer (PFNGLFOGCOORDPOINTERPROC)
pub static MJGLAD_GLFOGCOORDPOINTER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFogCoordd (PFNGLFOGCOORDDPROC)
pub static MJGLAD_GLFOGCOORDD: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFogCoorddv (PFNGLFOGCOORDDVPROC)
pub static MJGLAD_GLFOGCOORDDV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFogCoordf (PFNGLFOGCOORDFPROC)
pub static MJGLAD_GLFOGCOORDF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFogCoordfv (PFNGLFOGCOORDFVPROC)
pub static MJGLAD_GLFOGCOORDFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFogf (PFNGLFOGFPROC)
pub static MJGLAD_GLFOGF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFogfv (PFNGLFOGFVPROC)
pub static MJGLAD_GLFOGFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFogi (PFNGLFOGIPROC)
pub static MJGLAD_GLFOGI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFogiv (PFNGLFOGIVPROC)
pub static MJGLAD_GLFOGIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFramebufferRenderbuffer (PFNGLFRAMEBUFFERRENDERBUFFERPROC)
pub static MJGLAD_GLFRAMEBUFFERRENDERBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFramebufferTexture1D (PFNGLFRAMEBUFFERTEXTURE1DPROC)
pub static MJGLAD_GLFRAMEBUFFERTEXTURE1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFramebufferTexture2D (PFNGLFRAMEBUFFERTEXTURE2DPROC)
pub static MJGLAD_GLFRAMEBUFFERTEXTURE2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFramebufferTexture3D (PFNGLFRAMEBUFFERTEXTURE3DPROC)
pub static MJGLAD_GLFRAMEBUFFERTEXTURE3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFramebufferTextureLayer (PFNGLFRAMEBUFFERTEXTURELAYERPROC)
pub static MJGLAD_GLFRAMEBUFFERTEXTURELAYER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFrontFace (PFNGLFRONTFACEPROC)
pub static MJGLAD_GLFRONTFACE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glFrustum (PFNGLFRUSTUMPROC)
pub static MJGLAD_GLFRUSTUM: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGenBuffers (PFNGLGENBUFFERSPROC)
pub static MJGLAD_GLGENBUFFERS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGenBuffersARB (PFNGLGENBUFFERSARBPROC)
pub static MJGLAD_GLGENBUFFERSARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGenFramebuffers (PFNGLGENFRAMEBUFFERSPROC)
pub static MJGLAD_GLGENFRAMEBUFFERS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGenLists (PFNGLGENLISTSPROC)
pub static MJGLAD_GLGENLISTS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGenQueries (PFNGLGENQUERIESPROC)
pub static MJGLAD_GLGENQUERIES: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGenRenderbuffers (PFNGLGENRENDERBUFFERSPROC)
pub static MJGLAD_GLGENRENDERBUFFERS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGenTextures (PFNGLGENTEXTURESPROC)
pub static MJGLAD_GLGENTEXTURES: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGenerateMipmap (PFNGLGENERATEMIPMAPPROC)
pub static MJGLAD_GLGENERATEMIPMAP: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetBooleanv (PFNGLGETBOOLEANVPROC)
pub static MJGLAD_GLGETBOOLEANV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetBufferParameteriv (PFNGLGETBUFFERPARAMETERIVPROC)
pub static MJGLAD_GLGETBUFFERPARAMETERIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetBufferParameterivARB (PFNGLGETBUFFERPARAMETERIVARBPROC)
pub static MJGLAD_GLGETBUFFERPARAMETERIVARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetBufferPointerv (PFNGLGETBUFFERPOINTERVPROC)
pub static MJGLAD_GLGETBUFFERPOINTERV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetBufferPointervARB (PFNGLGETBUFFERPOINTERVARBPROC)
pub static MJGLAD_GLGETBUFFERPOINTERVARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetBufferSubData (PFNGLGETBUFFERSUBDATAPROC)
pub static MJGLAD_GLGETBUFFERSUBDATA: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetBufferSubDataARB (PFNGLGETBUFFERSUBDATAARBPROC)
pub static MJGLAD_GLGETBUFFERSUBDATAARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetClipPlane (PFNGLGETCLIPPLANEPROC)
pub static MJGLAD_GLGETCLIPPLANE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetCompressedTexImage (PFNGLGETCOMPRESSEDTEXIMAGEPROC)
pub static MJGLAD_GLGETCOMPRESSEDTEXIMAGE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetDebugMessageLog (PFNGLGETDEBUGMESSAGELOGPROC)
pub static MJGLAD_GLGETDEBUGMESSAGELOG: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetDebugMessageLogKHR (PFNGLGETDEBUGMESSAGELOGKHRPROC)
pub static MJGLAD_GLGETDEBUGMESSAGELOGKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetDoublev (PFNGLGETDOUBLEVPROC)
pub static MJGLAD_GLGETDOUBLEV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetError (PFNGLGETERRORPROC)
pub static MJGLAD_GLGETERROR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetFloatv (PFNGLGETFLOATVPROC)
pub static MJGLAD_GLGETFLOATV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetFramebufferAttachmentParameteriv (PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC)
pub static MJGLAD_GLGETFRAMEBUFFERATTACHMENTPARAMETERIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetIntegerv (PFNGLGETINTEGERVPROC)
pub static MJGLAD_GLGETINTEGERV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetLightfv (PFNGLGETLIGHTFVPROC)
pub static MJGLAD_GLGETLIGHTFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetLightiv (PFNGLGETLIGHTIVPROC)
pub static MJGLAD_GLGETLIGHTIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetMapdv (PFNGLGETMAPDVPROC)
pub static MJGLAD_GLGETMAPDV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetMapfv (PFNGLGETMAPFVPROC)
pub static MJGLAD_GLGETMAPFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetMapiv (PFNGLGETMAPIVPROC)
pub static MJGLAD_GLGETMAPIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetMaterialfv (PFNGLGETMATERIALFVPROC)
pub static MJGLAD_GLGETMATERIALFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetMaterialiv (PFNGLGETMATERIALIVPROC)
pub static MJGLAD_GLGETMATERIALIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetObjectLabel (PFNGLGETOBJECTLABELPROC)
pub static MJGLAD_GLGETOBJECTLABEL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetObjectLabelKHR (PFNGLGETOBJECTLABELKHRPROC)
pub static MJGLAD_GLGETOBJECTLABELKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetObjectPtrLabel (PFNGLGETOBJECTPTRLABELPROC)
pub static MJGLAD_GLGETOBJECTPTRLABEL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetObjectPtrLabelKHR (PFNGLGETOBJECTPTRLABELKHRPROC)
pub static MJGLAD_GLGETOBJECTPTRLABELKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetPixelMapfv (PFNGLGETPIXELMAPFVPROC)
pub static MJGLAD_GLGETPIXELMAPFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetPixelMapuiv (PFNGLGETPIXELMAPUIVPROC)
pub static MJGLAD_GLGETPIXELMAPUIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetPixelMapusv (PFNGLGETPIXELMAPUSVPROC)
pub static MJGLAD_GLGETPIXELMAPUSV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetPointerv (PFNGLGETPOINTERVPROC)
pub static MJGLAD_GLGETPOINTERV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetPointervKHR (PFNGLGETPOINTERVKHRPROC)
pub static MJGLAD_GLGETPOINTERVKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetPolygonStipple (PFNGLGETPOLYGONSTIPPLEPROC)
pub static MJGLAD_GLGETPOLYGONSTIPPLE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetQueryObjectiv (PFNGLGETQUERYOBJECTIVPROC)
pub static MJGLAD_GLGETQUERYOBJECTIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetQueryObjectuiv (PFNGLGETQUERYOBJECTUIVPROC)
pub static MJGLAD_GLGETQUERYOBJECTUIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetQueryiv (PFNGLGETQUERYIVPROC)
pub static MJGLAD_GLGETQUERYIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetRenderbufferParameteriv (PFNGLGETRENDERBUFFERPARAMETERIVPROC)
pub static MJGLAD_GLGETRENDERBUFFERPARAMETERIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetString (PFNGLGETSTRINGPROC)
pub static MJGLAD_GLGETSTRING: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexEnvfv (PFNGLGETTEXENVFVPROC)
pub static MJGLAD_GLGETTEXENVFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexEnviv (PFNGLGETTEXENVIVPROC)
pub static MJGLAD_GLGETTEXENVIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexGendv (PFNGLGETTEXGENDVPROC)
pub static MJGLAD_GLGETTEXGENDV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexGenfv (PFNGLGETTEXGENFVPROC)
pub static MJGLAD_GLGETTEXGENFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexGeniv (PFNGLGETTEXGENIVPROC)
pub static MJGLAD_GLGETTEXGENIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexImage (PFNGLGETTEXIMAGEPROC)
pub static MJGLAD_GLGETTEXIMAGE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexLevelParameterfv (PFNGLGETTEXLEVELPARAMETERFVPROC)
pub static MJGLAD_GLGETTEXLEVELPARAMETERFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexLevelParameteriv (PFNGLGETTEXLEVELPARAMETERIVPROC)
pub static MJGLAD_GLGETTEXLEVELPARAMETERIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexParameterfv (PFNGLGETTEXPARAMETERFVPROC)
pub static MJGLAD_GLGETTEXPARAMETERFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glGetTexParameteriv (PFNGLGETTEXPARAMETERIVPROC)
pub static MJGLAD_GLGETTEXPARAMETERIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glHint (PFNGLHINTPROC)
pub static MJGLAD_GLHINT: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexMask (PFNGLINDEXMASKPROC)
pub static MJGLAD_GLINDEXMASK: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexPointer (PFNGLINDEXPOINTERPROC)
pub static MJGLAD_GLINDEXPOINTER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexd (PFNGLINDEXDPROC)
pub static MJGLAD_GLINDEXD: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexdv (PFNGLINDEXDVPROC)
pub static MJGLAD_GLINDEXDV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexf (PFNGLINDEXFPROC)
pub static MJGLAD_GLINDEXF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexfv (PFNGLINDEXFVPROC)
pub static MJGLAD_GLINDEXFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexi (PFNGLINDEXIPROC)
pub static MJGLAD_GLINDEXI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexiv (PFNGLINDEXIVPROC)
pub static MJGLAD_GLINDEXIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexs (PFNGLINDEXSPROC)
pub static MJGLAD_GLINDEXS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexsv (PFNGLINDEXSVPROC)
pub static MJGLAD_GLINDEXSV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexub (PFNGLINDEXUBPROC)
pub static MJGLAD_GLINDEXUB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIndexubv (PFNGLINDEXUBVPROC)
pub static MJGLAD_GLINDEXUBV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glInitNames (PFNGLINITNAMESPROC)
pub static MJGLAD_GLINITNAMES: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glInterleavedArrays (PFNGLINTERLEAVEDARRAYSPROC)
pub static MJGLAD_GLINTERLEAVEDARRAYS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIsBuffer (PFNGLISBUFFERPROC)
pub static MJGLAD_GLISBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIsBufferARB (PFNGLISBUFFERARBPROC)
pub static MJGLAD_GLISBUFFERARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIsEnabled (PFNGLISENABLEDPROC)
pub static MJGLAD_GLISENABLED: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIsFramebuffer (PFNGLISFRAMEBUFFERPROC)
pub static MJGLAD_GLISFRAMEBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIsList (PFNGLISLISTPROC)
pub static MJGLAD_GLISLIST: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIsQuery (PFNGLISQUERYPROC)
pub static MJGLAD_GLISQUERY: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIsRenderbuffer (PFNGLISRENDERBUFFERPROC)
pub static MJGLAD_GLISRENDERBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glIsTexture (PFNGLISTEXTUREPROC)
pub static MJGLAD_GLISTEXTURE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLightModelf (PFNGLLIGHTMODELFPROC)
pub static MJGLAD_GLLIGHTMODELF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLightModelfv (PFNGLLIGHTMODELFVPROC)
pub static MJGLAD_GLLIGHTMODELFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLightModeli (PFNGLLIGHTMODELIPROC)
pub static MJGLAD_GLLIGHTMODELI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLightModeliv (PFNGLLIGHTMODELIVPROC)
pub static MJGLAD_GLLIGHTMODELIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLightf (PFNGLLIGHTFPROC)
pub static MJGLAD_GLLIGHTF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLightfv (PFNGLLIGHTFVPROC)
pub static MJGLAD_GLLIGHTFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLighti (PFNGLLIGHTIPROC)
pub static MJGLAD_GLLIGHTI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLightiv (PFNGLLIGHTIVPROC)
pub static MJGLAD_GLLIGHTIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLineStipple (PFNGLLINESTIPPLEPROC)
pub static MJGLAD_GLLINESTIPPLE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLineWidth (PFNGLLINEWIDTHPROC)
pub static MJGLAD_GLLINEWIDTH: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glListBase (PFNGLLISTBASEPROC)
pub static MJGLAD_GLLISTBASE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLoadIdentity (PFNGLLOADIDENTITYPROC)
pub static MJGLAD_GLLOADIDENTITY: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLoadMatrixd (PFNGLLOADMATRIXDPROC)
pub static MJGLAD_GLLOADMATRIXD: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLoadMatrixf (PFNGLLOADMATRIXFPROC)
pub static MJGLAD_GLLOADMATRIXF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLoadName (PFNGLLOADNAMEPROC)
pub static MJGLAD_GLLOADNAME: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLoadTransposeMatrixd (PFNGLLOADTRANSPOSEMATRIXDPROC)
pub static MJGLAD_GLLOADTRANSPOSEMATRIXD: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLoadTransposeMatrixf (PFNGLLOADTRANSPOSEMATRIXFPROC)
pub static MJGLAD_GLLOADTRANSPOSEMATRIXF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glLogicOp (PFNGLLOGICOPPROC)
pub static MJGLAD_GLLOGICOP: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMap1d (PFNGLMAP1DPROC)
pub static MJGLAD_GLMAP1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMap1f (PFNGLMAP1FPROC)
pub static MJGLAD_GLMAP1F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMap2d (PFNGLMAP2DPROC)
pub static MJGLAD_GLMAP2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMap2f (PFNGLMAP2FPROC)
pub static MJGLAD_GLMAP2F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMapBuffer (PFNGLMAPBUFFERPROC)
pub static MJGLAD_GLMAPBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMapBufferARB (PFNGLMAPBUFFERARBPROC)
pub static MJGLAD_GLMAPBUFFERARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMapGrid1d (PFNGLMAPGRID1DPROC)
pub static MJGLAD_GLMAPGRID1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMapGrid1f (PFNGLMAPGRID1FPROC)
pub static MJGLAD_GLMAPGRID1F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMapGrid2d (PFNGLMAPGRID2DPROC)
pub static MJGLAD_GLMAPGRID2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMapGrid2f (PFNGLMAPGRID2FPROC)
pub static MJGLAD_GLMAPGRID2F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMaterialf (PFNGLMATERIALFPROC)
pub static MJGLAD_GLMATERIALF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMaterialfv (PFNGLMATERIALFVPROC)
pub static MJGLAD_GLMATERIALFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMateriali (PFNGLMATERIALIPROC)
pub static MJGLAD_GLMATERIALI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMaterialiv (PFNGLMATERIALIVPROC)
pub static MJGLAD_GLMATERIALIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMatrixMode (PFNGLMATRIXMODEPROC)
pub static MJGLAD_GLMATRIXMODE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultMatrixd (PFNGLMULTMATRIXDPROC)
pub static MJGLAD_GLMULTMATRIXD: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultMatrixf (PFNGLMULTMATRIXFPROC)
pub static MJGLAD_GLMULTMATRIXF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultTransposeMatrixd (PFNGLMULTTRANSPOSEMATRIXDPROC)
pub static MJGLAD_GLMULTTRANSPOSEMATRIXD: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultTransposeMatrixf (PFNGLMULTTRANSPOSEMATRIXFPROC)
pub static MJGLAD_GLMULTTRANSPOSEMATRIXF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiDrawArrays (PFNGLMULTIDRAWARRAYSPROC)
pub static MJGLAD_GLMULTIDRAWARRAYS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiDrawElements (PFNGLMULTIDRAWELEMENTSPROC)
pub static MJGLAD_GLMULTIDRAWELEMENTS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord1d (PFNGLMULTITEXCOORD1DPROC)
pub static MJGLAD_GLMULTITEXCOORD1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord1dv (PFNGLMULTITEXCOORD1DVPROC)
pub static MJGLAD_GLMULTITEXCOORD1DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord1f (PFNGLMULTITEXCOORD1FPROC)
pub static MJGLAD_GLMULTITEXCOORD1F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord1fv (PFNGLMULTITEXCOORD1FVPROC)
pub static MJGLAD_GLMULTITEXCOORD1FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord1i (PFNGLMULTITEXCOORD1IPROC)
pub static MJGLAD_GLMULTITEXCOORD1I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord1iv (PFNGLMULTITEXCOORD1IVPROC)
pub static MJGLAD_GLMULTITEXCOORD1IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord1s (PFNGLMULTITEXCOORD1SPROC)
pub static MJGLAD_GLMULTITEXCOORD1S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord1sv (PFNGLMULTITEXCOORD1SVPROC)
pub static MJGLAD_GLMULTITEXCOORD1SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord2d (PFNGLMULTITEXCOORD2DPROC)
pub static MJGLAD_GLMULTITEXCOORD2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord2dv (PFNGLMULTITEXCOORD2DVPROC)
pub static MJGLAD_GLMULTITEXCOORD2DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord2f (PFNGLMULTITEXCOORD2FPROC)
pub static MJGLAD_GLMULTITEXCOORD2F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord2fv (PFNGLMULTITEXCOORD2FVPROC)
pub static MJGLAD_GLMULTITEXCOORD2FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord2i (PFNGLMULTITEXCOORD2IPROC)
pub static MJGLAD_GLMULTITEXCOORD2I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord2iv (PFNGLMULTITEXCOORD2IVPROC)
pub static MJGLAD_GLMULTITEXCOORD2IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord2s (PFNGLMULTITEXCOORD2SPROC)
pub static MJGLAD_GLMULTITEXCOORD2S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord2sv (PFNGLMULTITEXCOORD2SVPROC)
pub static MJGLAD_GLMULTITEXCOORD2SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord3d (PFNGLMULTITEXCOORD3DPROC)
pub static MJGLAD_GLMULTITEXCOORD3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord3dv (PFNGLMULTITEXCOORD3DVPROC)
pub static MJGLAD_GLMULTITEXCOORD3DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord3f (PFNGLMULTITEXCOORD3FPROC)
pub static MJGLAD_GLMULTITEXCOORD3F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord3fv (PFNGLMULTITEXCOORD3FVPROC)
pub static MJGLAD_GLMULTITEXCOORD3FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord3i (PFNGLMULTITEXCOORD3IPROC)
pub static MJGLAD_GLMULTITEXCOORD3I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord3iv (PFNGLMULTITEXCOORD3IVPROC)
pub static MJGLAD_GLMULTITEXCOORD3IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord3s (PFNGLMULTITEXCOORD3SPROC)
pub static MJGLAD_GLMULTITEXCOORD3S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord3sv (PFNGLMULTITEXCOORD3SVPROC)
pub static MJGLAD_GLMULTITEXCOORD3SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord4d (PFNGLMULTITEXCOORD4DPROC)
pub static MJGLAD_GLMULTITEXCOORD4D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord4dv (PFNGLMULTITEXCOORD4DVPROC)
pub static MJGLAD_GLMULTITEXCOORD4DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord4f (PFNGLMULTITEXCOORD4FPROC)
pub static MJGLAD_GLMULTITEXCOORD4F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord4fv (PFNGLMULTITEXCOORD4FVPROC)
pub static MJGLAD_GLMULTITEXCOORD4FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord4i (PFNGLMULTITEXCOORD4IPROC)
pub static MJGLAD_GLMULTITEXCOORD4I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord4iv (PFNGLMULTITEXCOORD4IVPROC)
pub static MJGLAD_GLMULTITEXCOORD4IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord4s (PFNGLMULTITEXCOORD4SPROC)
pub static MJGLAD_GLMULTITEXCOORD4S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glMultiTexCoord4sv (PFNGLMULTITEXCOORD4SVPROC)
pub static MJGLAD_GLMULTITEXCOORD4SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNewList (PFNGLNEWLISTPROC)
pub static MJGLAD_GLNEWLIST: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3b (PFNGLNORMAL3BPROC)
pub static MJGLAD_GLNORMAL3B: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3bv (PFNGLNORMAL3BVPROC)
pub static MJGLAD_GLNORMAL3BV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3d (PFNGLNORMAL3DPROC)
pub static MJGLAD_GLNORMAL3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3dv (PFNGLNORMAL3DVPROC)
pub static MJGLAD_GLNORMAL3DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3f (PFNGLNORMAL3FPROC)
pub static MJGLAD_GLNORMAL3F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3fv (PFNGLNORMAL3FVPROC)
pub static MJGLAD_GLNORMAL3FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3i (PFNGLNORMAL3IPROC)
pub static MJGLAD_GLNORMAL3I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3iv (PFNGLNORMAL3IVPROC)
pub static MJGLAD_GLNORMAL3IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3s (PFNGLNORMAL3SPROC)
pub static MJGLAD_GLNORMAL3S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormal3sv (PFNGLNORMAL3SVPROC)
pub static MJGLAD_GLNORMAL3SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glNormalPointer (PFNGLNORMALPOINTERPROC)
pub static MJGLAD_GLNORMALPOINTER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glObjectLabel (PFNGLOBJECTLABELPROC)
pub static MJGLAD_GLOBJECTLABEL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glObjectLabelKHR (PFNGLOBJECTLABELKHRPROC)
pub static MJGLAD_GLOBJECTLABELKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glObjectPtrLabel (PFNGLOBJECTPTRLABELPROC)
pub static MJGLAD_GLOBJECTPTRLABEL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glObjectPtrLabelKHR (PFNGLOBJECTPTRLABELKHRPROC)
pub static MJGLAD_GLOBJECTPTRLABELKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glOrtho (PFNGLORTHOPROC)
pub static MJGLAD_GLORTHO: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPassThrough (PFNGLPASSTHROUGHPROC)
pub static MJGLAD_GLPASSTHROUGH: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPixelMapfv (PFNGLPIXELMAPFVPROC)
pub static MJGLAD_GLPIXELMAPFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPixelMapuiv (PFNGLPIXELMAPUIVPROC)
pub static MJGLAD_GLPIXELMAPUIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPixelMapusv (PFNGLPIXELMAPUSVPROC)
pub static MJGLAD_GLPIXELMAPUSV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPixelStoref (PFNGLPIXELSTOREFPROC)
pub static MJGLAD_GLPIXELSTOREF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPixelStorei (PFNGLPIXELSTOREIPROC)
pub static MJGLAD_GLPIXELSTOREI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPixelTransferf (PFNGLPIXELTRANSFERFPROC)
pub static MJGLAD_GLPIXELTRANSFERF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPixelTransferi (PFNGLPIXELTRANSFERIPROC)
pub static MJGLAD_GLPIXELTRANSFERI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPixelZoom (PFNGLPIXELZOOMPROC)
pub static MJGLAD_GLPIXELZOOM: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPointParameterf (PFNGLPOINTPARAMETERFPROC)
pub static MJGLAD_GLPOINTPARAMETERF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPointParameterfv (PFNGLPOINTPARAMETERFVPROC)
pub static MJGLAD_GLPOINTPARAMETERFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPointParameteri (PFNGLPOINTPARAMETERIPROC)
pub static MJGLAD_GLPOINTPARAMETERI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPointParameteriv (PFNGLPOINTPARAMETERIVPROC)
pub static MJGLAD_GLPOINTPARAMETERIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPointSize (PFNGLPOINTSIZEPROC)
pub static MJGLAD_GLPOINTSIZE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPolygonMode (PFNGLPOLYGONMODEPROC)
pub static MJGLAD_GLPOLYGONMODE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPolygonOffset (PFNGLPOLYGONOFFSETPROC)
pub static MJGLAD_GLPOLYGONOFFSET: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPolygonStipple (PFNGLPOLYGONSTIPPLEPROC)
pub static MJGLAD_GLPOLYGONSTIPPLE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPopAttrib (PFNGLPOPATTRIBPROC)
pub static MJGLAD_GLPOPATTRIB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPopClientAttrib (PFNGLPOPCLIENTATTRIBPROC)
pub static MJGLAD_GLPOPCLIENTATTRIB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPopDebugGroup (PFNGLPOPDEBUGGROUPPROC)
pub static MJGLAD_GLPOPDEBUGGROUP: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPopDebugGroupKHR (PFNGLPOPDEBUGGROUPKHRPROC)
pub static MJGLAD_GLPOPDEBUGGROUPKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPopMatrix (PFNGLPOPMATRIXPROC)
pub static MJGLAD_GLPOPMATRIX: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPopName (PFNGLPOPNAMEPROC)
pub static MJGLAD_GLPOPNAME: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPrioritizeTextures (PFNGLPRIORITIZETEXTURESPROC)
pub static MJGLAD_GLPRIORITIZETEXTURES: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPushAttrib (PFNGLPUSHATTRIBPROC)
pub static MJGLAD_GLPUSHATTRIB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPushClientAttrib (PFNGLPUSHCLIENTATTRIBPROC)
pub static MJGLAD_GLPUSHCLIENTATTRIB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPushDebugGroup (PFNGLPUSHDEBUGGROUPPROC)
pub static MJGLAD_GLPUSHDEBUGGROUP: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPushDebugGroupKHR (PFNGLPUSHDEBUGGROUPKHRPROC)
pub static MJGLAD_GLPUSHDEBUGGROUPKHR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPushMatrix (PFNGLPUSHMATRIXPROC)
pub static MJGLAD_GLPUSHMATRIX: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glPushName (PFNGLPUSHNAMEPROC)
pub static MJGLAD_GLPUSHNAME: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos2d (PFNGLRASTERPOS2DPROC)
pub static MJGLAD_GLRASTERPOS2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos2dv (PFNGLRASTERPOS2DVPROC)
pub static MJGLAD_GLRASTERPOS2DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos2f (PFNGLRASTERPOS2FPROC)
pub static MJGLAD_GLRASTERPOS2F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos2fv (PFNGLRASTERPOS2FVPROC)
pub static MJGLAD_GLRASTERPOS2FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos2i (PFNGLRASTERPOS2IPROC)
pub static MJGLAD_GLRASTERPOS2I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos2iv (PFNGLRASTERPOS2IVPROC)
pub static MJGLAD_GLRASTERPOS2IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos2s (PFNGLRASTERPOS2SPROC)
pub static MJGLAD_GLRASTERPOS2S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos2sv (PFNGLRASTERPOS2SVPROC)
pub static MJGLAD_GLRASTERPOS2SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos3d (PFNGLRASTERPOS3DPROC)
pub static MJGLAD_GLRASTERPOS3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos3dv (PFNGLRASTERPOS3DVPROC)
pub static MJGLAD_GLRASTERPOS3DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos3f (PFNGLRASTERPOS3FPROC)
pub static MJGLAD_GLRASTERPOS3F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos3fv (PFNGLRASTERPOS3FVPROC)
pub static MJGLAD_GLRASTERPOS3FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos3i (PFNGLRASTERPOS3IPROC)
pub static MJGLAD_GLRASTERPOS3I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos3iv (PFNGLRASTERPOS3IVPROC)
pub static MJGLAD_GLRASTERPOS3IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos3s (PFNGLRASTERPOS3SPROC)
pub static MJGLAD_GLRASTERPOS3S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos3sv (PFNGLRASTERPOS3SVPROC)
pub static MJGLAD_GLRASTERPOS3SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos4d (PFNGLRASTERPOS4DPROC)
pub static MJGLAD_GLRASTERPOS4D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos4dv (PFNGLRASTERPOS4DVPROC)
pub static MJGLAD_GLRASTERPOS4DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos4f (PFNGLRASTERPOS4FPROC)
pub static MJGLAD_GLRASTERPOS4F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos4fv (PFNGLRASTERPOS4FVPROC)
pub static MJGLAD_GLRASTERPOS4FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos4i (PFNGLRASTERPOS4IPROC)
pub static MJGLAD_GLRASTERPOS4I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos4iv (PFNGLRASTERPOS4IVPROC)
pub static MJGLAD_GLRASTERPOS4IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos4s (PFNGLRASTERPOS4SPROC)
pub static MJGLAD_GLRASTERPOS4S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRasterPos4sv (PFNGLRASTERPOS4SVPROC)
pub static MJGLAD_GLRASTERPOS4SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glReadBuffer (PFNGLREADBUFFERPROC)
pub static MJGLAD_GLREADBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glReadPixels (PFNGLREADPIXELSPROC)
pub static MJGLAD_GLREADPIXELS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRectd (PFNGLRECTDPROC)
pub static MJGLAD_GLRECTD: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRectdv (PFNGLRECTDVPROC)
pub static MJGLAD_GLRECTDV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRectf (PFNGLRECTFPROC)
pub static MJGLAD_GLRECTF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRectfv (PFNGLRECTFVPROC)
pub static MJGLAD_GLRECTFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRecti (PFNGLRECTIPROC)
pub static MJGLAD_GLRECTI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRectiv (PFNGLRECTIVPROC)
pub static MJGLAD_GLRECTIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRects (PFNGLRECTSPROC)
pub static MJGLAD_GLRECTS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRectsv (PFNGLRECTSVPROC)
pub static MJGLAD_GLRECTSV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRenderMode (PFNGLRENDERMODEPROC)
pub static MJGLAD_GLRENDERMODE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRenderbufferStorage (PFNGLRENDERBUFFERSTORAGEPROC)
pub static MJGLAD_GLRENDERBUFFERSTORAGE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRenderbufferStorageMultisample (PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC)
pub static MJGLAD_GLRENDERBUFFERSTORAGEMULTISAMPLE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRotated (PFNGLROTATEDPROC)
pub static MJGLAD_GLROTATED: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glRotatef (PFNGLROTATEFPROC)
pub static MJGLAD_GLROTATEF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSampleCoverage (PFNGLSAMPLECOVERAGEPROC)
pub static MJGLAD_GLSAMPLECOVERAGE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glScaled (PFNGLSCALEDPROC)
pub static MJGLAD_GLSCALED: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glScalef (PFNGLSCALEFPROC)
pub static MJGLAD_GLSCALEF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glScissor (PFNGLSCISSORPROC)
pub static MJGLAD_GLSCISSOR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3b (PFNGLSECONDARYCOLOR3BPROC)
pub static MJGLAD_GLSECONDARYCOLOR3B: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3bv (PFNGLSECONDARYCOLOR3BVPROC)
pub static MJGLAD_GLSECONDARYCOLOR3BV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3d (PFNGLSECONDARYCOLOR3DPROC)
pub static MJGLAD_GLSECONDARYCOLOR3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3dv (PFNGLSECONDARYCOLOR3DVPROC)
pub static MJGLAD_GLSECONDARYCOLOR3DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3f (PFNGLSECONDARYCOLOR3FPROC)
pub static MJGLAD_GLSECONDARYCOLOR3F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3fv (PFNGLSECONDARYCOLOR3FVPROC)
pub static MJGLAD_GLSECONDARYCOLOR3FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3i (PFNGLSECONDARYCOLOR3IPROC)
pub static MJGLAD_GLSECONDARYCOLOR3I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3iv (PFNGLSECONDARYCOLOR3IVPROC)
pub static MJGLAD_GLSECONDARYCOLOR3IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3s (PFNGLSECONDARYCOLOR3SPROC)
pub static MJGLAD_GLSECONDARYCOLOR3S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3sv (PFNGLSECONDARYCOLOR3SVPROC)
pub static MJGLAD_GLSECONDARYCOLOR3SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3ub (PFNGLSECONDARYCOLOR3UBPROC)
pub static MJGLAD_GLSECONDARYCOLOR3UB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3ubv (PFNGLSECONDARYCOLOR3UBVPROC)
pub static MJGLAD_GLSECONDARYCOLOR3UBV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3ui (PFNGLSECONDARYCOLOR3UIPROC)
pub static MJGLAD_GLSECONDARYCOLOR3UI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3uiv (PFNGLSECONDARYCOLOR3UIVPROC)
pub static MJGLAD_GLSECONDARYCOLOR3UIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3us (PFNGLSECONDARYCOLOR3USPROC)
pub static MJGLAD_GLSECONDARYCOLOR3US: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColor3usv (PFNGLSECONDARYCOLOR3USVPROC)
pub static MJGLAD_GLSECONDARYCOLOR3USV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSecondaryColorPointer (PFNGLSECONDARYCOLORPOINTERPROC)
pub static MJGLAD_GLSECONDARYCOLORPOINTER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glSelectBuffer (PFNGLSELECTBUFFERPROC)
pub static MJGLAD_GLSELECTBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glShadeModel (PFNGLSHADEMODELPROC)
pub static MJGLAD_GLSHADEMODEL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glStencilFunc (PFNGLSTENCILFUNCPROC)
pub static MJGLAD_GLSTENCILFUNC: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glStencilMask (PFNGLSTENCILMASKPROC)
pub static MJGLAD_GLSTENCILMASK: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glStencilOp (PFNGLSTENCILOPPROC)
pub static MJGLAD_GLSTENCILOP: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord1d (PFNGLTEXCOORD1DPROC)
pub static MJGLAD_GLTEXCOORD1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord1dv (PFNGLTEXCOORD1DVPROC)
pub static MJGLAD_GLTEXCOORD1DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord1f (PFNGLTEXCOORD1FPROC)
pub static MJGLAD_GLTEXCOORD1F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord1fv (PFNGLTEXCOORD1FVPROC)
pub static MJGLAD_GLTEXCOORD1FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord1i (PFNGLTEXCOORD1IPROC)
pub static MJGLAD_GLTEXCOORD1I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord1iv (PFNGLTEXCOORD1IVPROC)
pub static MJGLAD_GLTEXCOORD1IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord1s (PFNGLTEXCOORD1SPROC)
pub static MJGLAD_GLTEXCOORD1S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord1sv (PFNGLTEXCOORD1SVPROC)
pub static MJGLAD_GLTEXCOORD1SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord2d (PFNGLTEXCOORD2DPROC)
pub static MJGLAD_GLTEXCOORD2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord2dv (PFNGLTEXCOORD2DVPROC)
pub static MJGLAD_GLTEXCOORD2DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord2f (PFNGLTEXCOORD2FPROC)
pub static MJGLAD_GLTEXCOORD2F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord2fv (PFNGLTEXCOORD2FVPROC)
pub static MJGLAD_GLTEXCOORD2FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord2i (PFNGLTEXCOORD2IPROC)
pub static MJGLAD_GLTEXCOORD2I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord2iv (PFNGLTEXCOORD2IVPROC)
pub static MJGLAD_GLTEXCOORD2IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord2s (PFNGLTEXCOORD2SPROC)
pub static MJGLAD_GLTEXCOORD2S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord2sv (PFNGLTEXCOORD2SVPROC)
pub static MJGLAD_GLTEXCOORD2SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord3d (PFNGLTEXCOORD3DPROC)
pub static MJGLAD_GLTEXCOORD3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord3dv (PFNGLTEXCOORD3DVPROC)
pub static MJGLAD_GLTEXCOORD3DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord3f (PFNGLTEXCOORD3FPROC)
pub static MJGLAD_GLTEXCOORD3F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord3fv (PFNGLTEXCOORD3FVPROC)
pub static MJGLAD_GLTEXCOORD3FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord3i (PFNGLTEXCOORD3IPROC)
pub static MJGLAD_GLTEXCOORD3I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord3iv (PFNGLTEXCOORD3IVPROC)
pub static MJGLAD_GLTEXCOORD3IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord3s (PFNGLTEXCOORD3SPROC)
pub static MJGLAD_GLTEXCOORD3S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord3sv (PFNGLTEXCOORD3SVPROC)
pub static MJGLAD_GLTEXCOORD3SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord4d (PFNGLTEXCOORD4DPROC)
pub static MJGLAD_GLTEXCOORD4D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord4dv (PFNGLTEXCOORD4DVPROC)
pub static MJGLAD_GLTEXCOORD4DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord4f (PFNGLTEXCOORD4FPROC)
pub static MJGLAD_GLTEXCOORD4F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord4fv (PFNGLTEXCOORD4FVPROC)
pub static MJGLAD_GLTEXCOORD4FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord4i (PFNGLTEXCOORD4IPROC)
pub static MJGLAD_GLTEXCOORD4I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord4iv (PFNGLTEXCOORD4IVPROC)
pub static MJGLAD_GLTEXCOORD4IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord4s (PFNGLTEXCOORD4SPROC)
pub static MJGLAD_GLTEXCOORD4S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoord4sv (PFNGLTEXCOORD4SVPROC)
pub static MJGLAD_GLTEXCOORD4SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexCoordPointer (PFNGLTEXCOORDPOINTERPROC)
pub static MJGLAD_GLTEXCOORDPOINTER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexEnvf (PFNGLTEXENVFPROC)
pub static MJGLAD_GLTEXENVF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexEnvfv (PFNGLTEXENVFVPROC)
pub static MJGLAD_GLTEXENVFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexEnvi (PFNGLTEXENVIPROC)
pub static MJGLAD_GLTEXENVI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexEnviv (PFNGLTEXENVIVPROC)
pub static MJGLAD_GLTEXENVIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexGend (PFNGLTEXGENDPROC)
pub static MJGLAD_GLTEXGEND: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexGendv (PFNGLTEXGENDVPROC)
pub static MJGLAD_GLTEXGENDV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexGenf (PFNGLTEXGENFPROC)
pub static MJGLAD_GLTEXGENF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexGenfv (PFNGLTEXGENFVPROC)
pub static MJGLAD_GLTEXGENFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexGeni (PFNGLTEXGENIPROC)
pub static MJGLAD_GLTEXGENI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexGeniv (PFNGLTEXGENIVPROC)
pub static MJGLAD_GLTEXGENIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexImage1D (PFNGLTEXIMAGE1DPROC)
pub static MJGLAD_GLTEXIMAGE1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexImage2D (PFNGLTEXIMAGE2DPROC)
pub static MJGLAD_GLTEXIMAGE2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexImage3D (PFNGLTEXIMAGE3DPROC)
pub static MJGLAD_GLTEXIMAGE3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexParameterf (PFNGLTEXPARAMETERFPROC)
pub static MJGLAD_GLTEXPARAMETERF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexParameterfv (PFNGLTEXPARAMETERFVPROC)
pub static MJGLAD_GLTEXPARAMETERFV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexParameteri (PFNGLTEXPARAMETERIPROC)
pub static MJGLAD_GLTEXPARAMETERI: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexParameteriv (PFNGLTEXPARAMETERIVPROC)
pub static MJGLAD_GLTEXPARAMETERIV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexSubImage1D (PFNGLTEXSUBIMAGE1DPROC)
pub static MJGLAD_GLTEXSUBIMAGE1D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexSubImage2D (PFNGLTEXSUBIMAGE2DPROC)
pub static MJGLAD_GLTEXSUBIMAGE2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTexSubImage3D (PFNGLTEXSUBIMAGE3DPROC)
pub static MJGLAD_GLTEXSUBIMAGE3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTranslated (PFNGLTRANSLATEDPROC)
pub static MJGLAD_GLTRANSLATED: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glTranslatef (PFNGLTRANSLATEFPROC)
pub static MJGLAD_GLTRANSLATEF: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glUnmapBuffer (PFNGLUNMAPBUFFERPROC)
pub static MJGLAD_GLUNMAPBUFFER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glUnmapBufferARB (PFNGLUNMAPBUFFERARBPROC)
pub static MJGLAD_GLUNMAPBUFFERARB: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex2d (PFNGLVERTEX2DPROC)
pub static MJGLAD_GLVERTEX2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex2dv (PFNGLVERTEX2DVPROC)
pub static MJGLAD_GLVERTEX2DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex2f (PFNGLVERTEX2FPROC)
pub static MJGLAD_GLVERTEX2F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex2fv (PFNGLVERTEX2FVPROC)
pub static MJGLAD_GLVERTEX2FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex2i (PFNGLVERTEX2IPROC)
pub static MJGLAD_GLVERTEX2I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex2iv (PFNGLVERTEX2IVPROC)
pub static MJGLAD_GLVERTEX2IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex2s (PFNGLVERTEX2SPROC)
pub static MJGLAD_GLVERTEX2S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex2sv (PFNGLVERTEX2SVPROC)
pub static MJGLAD_GLVERTEX2SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex3d (PFNGLVERTEX3DPROC)
pub static MJGLAD_GLVERTEX3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex3dv (PFNGLVERTEX3DVPROC)
pub static MJGLAD_GLVERTEX3DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex3f (PFNGLVERTEX3FPROC)
pub static MJGLAD_GLVERTEX3F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex3fv (PFNGLVERTEX3FVPROC)
pub static MJGLAD_GLVERTEX3FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex3i (PFNGLVERTEX3IPROC)
pub static MJGLAD_GLVERTEX3I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex3iv (PFNGLVERTEX3IVPROC)
pub static MJGLAD_GLVERTEX3IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex3s (PFNGLVERTEX3SPROC)
pub static MJGLAD_GLVERTEX3S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex3sv (PFNGLVERTEX3SVPROC)
pub static MJGLAD_GLVERTEX3SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex4d (PFNGLVERTEX4DPROC)
pub static MJGLAD_GLVERTEX4D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex4dv (PFNGLVERTEX4DVPROC)
pub static MJGLAD_GLVERTEX4DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex4f (PFNGLVERTEX4FPROC)
pub static MJGLAD_GLVERTEX4F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex4fv (PFNGLVERTEX4FVPROC)
pub static MJGLAD_GLVERTEX4FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex4i (PFNGLVERTEX4IPROC)
pub static MJGLAD_GLVERTEX4I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex4iv (PFNGLVERTEX4IVPROC)
pub static MJGLAD_GLVERTEX4IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex4s (PFNGLVERTEX4SPROC)
pub static MJGLAD_GLVERTEX4S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertex4sv (PFNGLVERTEX4SVPROC)
pub static MJGLAD_GLVERTEX4SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glVertexPointer (PFNGLVERTEXPOINTERPROC)
pub static MJGLAD_GLVERTEXPOINTER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glViewport (PFNGLVIEWPORTPROC)
pub static MJGLAD_GLVIEWPORT: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos2d (PFNGLWINDOWPOS2DPROC)
pub static MJGLAD_GLWINDOWPOS2D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos2dv (PFNGLWINDOWPOS2DVPROC)
pub static MJGLAD_GLWINDOWPOS2DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos2f (PFNGLWINDOWPOS2FPROC)
pub static MJGLAD_GLWINDOWPOS2F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos2fv (PFNGLWINDOWPOS2FVPROC)
pub static MJGLAD_GLWINDOWPOS2FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos2i (PFNGLWINDOWPOS2IPROC)
pub static MJGLAD_GLWINDOWPOS2I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos2iv (PFNGLWINDOWPOS2IVPROC)
pub static MJGLAD_GLWINDOWPOS2IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos2s (PFNGLWINDOWPOS2SPROC)
pub static MJGLAD_GLWINDOWPOS2S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos2sv (PFNGLWINDOWPOS2SVPROC)
pub static MJGLAD_GLWINDOWPOS2SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos3d (PFNGLWINDOWPOS3DPROC)
pub static MJGLAD_GLWINDOWPOS3D: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos3dv (PFNGLWINDOWPOS3DVPROC)
pub static MJGLAD_GLWINDOWPOS3DV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos3f (PFNGLWINDOWPOS3FPROC)
pub static MJGLAD_GLWINDOWPOS3F: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos3fv (PFNGLWINDOWPOS3FVPROC)
pub static MJGLAD_GLWINDOWPOS3FV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos3i (PFNGLWINDOWPOS3IPROC)
pub static MJGLAD_GLWINDOWPOS3I: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos3iv (PFNGLWINDOWPOS3IVPROC)
pub static MJGLAD_GLWINDOWPOS3IV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos3s (PFNGLWINDOWPOS3SPROC)
pub static MJGLAD_GLWINDOWPOS3S: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_glWindowPos3sv (PFNGLWINDOWPOS3SVPROC)
pub static MJGLAD_GLWINDOWPOS3SV: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_libGL (void *)
pub static MJGLAD_LIBGL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjGlad_max_loaded_major (int)
pub static MJGLAD_MAX_LOADED_MAJOR: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGlad_max_loaded_minor (int)
pub static MJGLAD_MAX_LOADED_MINOR: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjGlad_num_exts_i (int)
pub static MJGLAD_NUM_EXTS_I: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjLABELSTRING (const char *[17])
pub static MJLABELSTRING: std::sync::LazyLock<std::sync::Mutex<[u8; 136]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 136]));

/// C static: mjMINMASS (const double)
pub static MJMINMASS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjRAYDATA_SIZE (const int[6])
pub static MJRAYDATA_SIZE: std::sync::LazyLock<std::sync::Mutex<[u8; 24]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 24]));

/// C static: mjRNDSTRING (const char *[11][3])
pub static MJRNDSTRING: std::sync::LazyLock<std::sync::Mutex<[u8; 264]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 264]));

/// C static: mjTIMERSTRING (const char *[15])
pub static MJTIMERSTRING: std::sync::LazyLock<std::sync::Mutex<[u8; 120]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 120]));

/// C static: mjTOPICSTRING (const char *[3])
pub static MJTOPICSTRING: std::sync::LazyLock<std::sync::Mutex<[u8; 24]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 24]));

/// C static: mjVISSTRING (const char *[31][3])
pub static MJVISSTRING: std::sync::LazyLock<std::sync::Mutex<[u8; 744]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 744]));

/// C static: mj_nesterov_momentum (int)
pub static MJ_NESTEROV_MOMENTUM: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: mjcb_act_bias (mjfAct)
pub static MJCB_ACT_BIAS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjcb_act_dyn (mjfAct)
pub static MJCB_ACT_DYN: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjcb_act_gain (mjfAct)
pub static MJCB_ACT_GAIN: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjcb_contactfilter (mjfConFilt)
pub static MJCB_CONTACTFILTER: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjcb_control (mjfGeneric)
pub static MJCB_CONTROL: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjcb_passive (mjfGeneric)
pub static MJCB_PASSIVE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjcb_sensor (mjfSensor)
pub static MJCB_SENSOR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mjcb_time (mjfTime)
pub static MJCB_TIME: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mju_user_error (void (*)(const char *))
pub static MJU_USER_ERROR: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mju_user_free (void (*)(void *))
pub static MJU_USER_FREE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mju_user_malloc (void *(*)(size_t))
pub static MJU_USER_MALLOC: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: mju_user_warning (void (*)(const char *))
pub static MJU_USER_WARNING: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: nPOS (const int[4])
pub static NPOS: std::sync::LazyLock<std::sync::Mutex<[u8; 16]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 16]));

/// C static: nVEL (const int[4])
pub static NVEL: std::sync::LazyLock<std::sync::Mutex<[u8; 16]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 16]));

/// C static: precision (int)
pub static PRECISION: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: projection_sz (const int)
pub static PROJECTION_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: reduce_sz (const int)
pub static REDUCE_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: rotEPS (const mjtNum)
pub static ROTEPS: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: shape_map (const mjMap[4])
pub static SHAPE_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 64]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 64]));

/// C static: solver_sz (const int)
pub static SOLVER_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: stage_sz (const int)
pub static STAGE_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: subD00 (const mjtNum[88])
pub static SUBD00: std::sync::LazyLock<std::sync::Mutex<[u8; 704]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 704]));

/// C static: subD01 (const mjtNum[88])
pub static SUBD01: std::sync::LazyLock<std::sync::Mutex<[u8; 704]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 704]));

/// C static: subD02 (const mjtNum[88])
pub static SUBD02: std::sync::LazyLock<std::sync::Mutex<[u8; 704]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 704]));

/// C static: subD10 (const mjtNum[88])
pub static SUBD10: std::sync::LazyLock<std::sync::Mutex<[u8; 704]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 704]));

/// C static: subD11 (const mjtNum[88])
pub static SUBD11: std::sync::LazyLock<std::sync::Mutex<[u8; 704]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 704]));

/// C static: subD12 (const mjtNum[88])
pub static SUBD12: std::sync::LazyLock<std::sync::Mutex<[u8; 704]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 704]));

/// C static: subD20 (const mjtNum[88])
pub static SUBD20: std::sync::LazyLock<std::sync::Mutex<[u8; 704]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 704]));

/// C static: subD21 (const mjtNum[88])
pub static SUBD21: std::sync::LazyLock<std::sync::Mutex<[u8; 704]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 704]));

/// C static: subD22 (const mjtNum[88])
pub static SUBD22: std::sync::LazyLock<std::sync::Mutex<[u8; 704]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 704]));

/// C static: subW (const mjtNum[256])
pub static SUBW: std::sync::LazyLock<std::sync::Mutex<[u8; 2048]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 2048]));

/// C static: texrole_sz (const int)
pub static TEXROLE_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: texture_sz (const int)
pub static TEXTURE_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: themeColor0 (const mjuiThemeColor)
pub static THEMECOLOR0: std::sync::LazyLock<std::sync::Mutex<[u8; 336]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 336]));

/// C static: themeColor1 (const mjuiThemeColor)
pub static THEMECOLOR1: std::sync::LazyLock<std::sync::Mutex<[u8; 336]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 336]));

/// C static: themeColor2 (const mjuiThemeColor)
pub static THEMECOLOR2: std::sync::LazyLock<std::sync::Mutex<[u8; 336]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 336]));

/// C static: themeColor3 (const mjuiThemeColor)
pub static THEMECOLOR3: std::sync::LazyLock<std::sync::Mutex<[u8; 336]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 336]));

/// C static: themeSpacing0 (const mjuiThemeSpacing)
pub static THEMESPACING0: std::sync::LazyLock<std::sync::Mutex<[u8; 52]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 52]));

/// C static: themeSpacing1 (const mjuiThemeSpacing)
pub static THEMESPACING1: std::sync::LazyLock<std::sync::Mutex<[u8; 52]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 52]));

/// C static: tolerance (const mjtNum)
pub static TOLERANCE: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: tolplanemesh (const mjtNum)
pub static TOLPLANEMESH: std::sync::LazyLock<std::sync::Mutex<[u8; 8]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 8]));

/// C static: urJoint_map (const mjMap[7])
pub static URJOINT_MAP: std::sync::LazyLock<std::sync::Mutex<[u8; 112]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 112]));

/// C static: urJoint_sz (const int)
pub static URJOINT_SZ: std::sync::LazyLock<std::sync::Mutex<[u8; 4]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 4]));

/// C static: warningtext (char[500])
pub static WARNINGTEXT: std::sync::LazyLock<std::sync::Mutex<[u8; 500]>> = std::sync::LazyLock::new(|| std::sync::Mutex::new([0u8; 500]));

/// C static (override): mjCModel::Delete
pub static MJ_C_MODEL_DELETE: std::sync::LazyLock<std::sync::Mutex<Option<unsafe extern "C" fn()>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

/// C static (override): mjCModel::DeleteAll
pub static MJ_C_MODEL_DELETE_ALL: std::sync::LazyLock<std::sync::Mutex<Option<unsafe extern "C" fn()>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));


/// Compatibility: size_type (older codegen emitted as sized opaque)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct size_type { pub _data: [u8; 8] }
/// Compatibility: Mat44 = float[4][4] (OpenGL 4x4 matrix)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct Mat44 { pub _data: [u8; 64] }
/// Compatibility: Mat44Arg = Mat44 (pass-by-value alias)
pub type Mat44Arg = Mat44;
