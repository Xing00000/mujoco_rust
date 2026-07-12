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

/// Opaque C struct: BufferProvider (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BufferProvider { _opaque: [u8; 0] }

/// Opaque C struct: ContactInfo (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContactInfo { _opaque: [u8; 0] }

/// Opaque C struct: Face (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Face { _opaque: [u8; 0] }

/// Opaque C struct: FilePath (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FilePath { _opaque: [u8; 0] }

/// Opaque C struct: GLADloadproc (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLADloadproc { _opaque: [u8; 0] }

/// Opaque C struct: GLchar (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLchar { _opaque: [u8; 0] }

/// Opaque C struct: GLenum (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLenum { _opaque: [u8; 0] }

/// Opaque C struct: GLsizei (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLsizei { _opaque: [u8; 0] }

/// Opaque C struct: GLuint (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLuint { _opaque: [u8; 0] }

/// Opaque C struct: GlobalTable (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GlobalTable { _opaque: [u8; 0] }

/// Opaque C struct: LocaleOverride (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LocaleOverride { _opaque: [u8; 0] }

/// Opaque C struct: Matrix (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Matrix { _opaque: [u8; 0] }

/// Sized opaque C struct: MemPool (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct MemPool { pub _data: [u8; 8] }

/// Opaque C struct: MeshPolygon (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MeshPolygon { _opaque: [u8; 0] }

/// Opaque C struct: MeshSDFContext (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MeshSDFContext { _opaque: [u8; 0] }

/// Opaque C struct: Mutex (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mutex { _opaque: [u8; 0] }

/// Opaque C struct: OctNode (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OctNode { _opaque: [u8; 0] }

/// Opaque C struct: OctreeTask (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OctreeTask { _opaque: [u8; 0] }

/// Opaque C struct: PNGImage (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNGImage { _opaque: [u8; 0] }

/// Opaque C struct: Polytope (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Polytope { _opaque: [u8; 0] }

/// Opaque C struct: Reader (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Reader { _opaque: [u8; 0] }

/// Opaque C struct: Resolver (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Resolver { _opaque: [u8; 0] }

/// Opaque C struct: ResourcePtr (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ResourcePtr { _opaque: [u8; 0] }

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

/// Opaque C struct: ThreadPool (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ThreadPool { _opaque: [u8; 0] }

/// Opaque C struct: ThreadPoolContext (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ThreadPoolContext { _opaque: [u8; 0] }

/// Opaque C struct: Triangle (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Triangle { _opaque: [u8; 0] }

/// Opaque C struct: VFS (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VFS { _opaque: [u8; 0] }

/// Opaque C struct: Vertex (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex { _opaque: [u8; 0] }

/// Opaque C struct: XMLDocument (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XMLDocument { _opaque: [u8; 0] }

/// Opaque C struct: ZipArchiveProvider (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ZipArchiveProvider { _opaque: [u8; 0] }

/// Opaque C struct: ccd_real_t (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ccd_real_t { _opaque: [u8; 0] }

/// Opaque C struct: char__const (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct char__const { _opaque: [u8; 0] }

/// Opaque C struct: mjByteVec (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjByteVec { _opaque: [u8; 0] }

/// Opaque C struct: mjCActuator (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCActuator { _opaque: [u8; 0] }

/// Opaque C struct: mjCAsset (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCAsset { _opaque: [u8; 0] }

/// Opaque C struct: mjCBase (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCBase { _opaque: [u8; 0] }

/// Opaque C struct: mjCBody (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCBody { _opaque: [u8; 0] }

/// Opaque C struct: mjCBodyPair (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCBodyPair { _opaque: [u8; 0] }

/// Opaque C struct: mjCBoundingVolume (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCBoundingVolume { _opaque: [u8; 0] }

/// Opaque C struct: mjCBoundingVolumeHierarchy (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCBoundingVolumeHierarchy { _opaque: [u8; 0] }

/// Opaque C struct: mjCCDConfig (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCCDConfig { _opaque: [u8; 0] }

/// Opaque C struct: mjCCDStatus (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCCDStatus { _opaque: [u8; 0] }

/// Opaque C struct: mjCCache (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCCache { _opaque: [u8; 0] }

/// Opaque C struct: mjCCamera (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCCamera { _opaque: [u8; 0] }

/// Opaque C struct: mjCComposite (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCComposite { _opaque: [u8; 0] }

/// Opaque C struct: mjCDataFunc (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCDataFunc { _opaque: [u8; 0] }

/// Opaque C struct: mjCDef (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCDef { _opaque: [u8; 0] }

/// Opaque C struct: mjCEquality (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCEquality { _opaque: [u8; 0] }

/// Opaque C struct: mjCFlex (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCFlex { _opaque: [u8; 0] }

/// Opaque C struct: mjCFlexcomp (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCFlexcomp { _opaque: [u8; 0] }

/// Opaque C struct: mjCFrame (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCFrame { _opaque: [u8; 0] }

/// Opaque C struct: mjCGeom (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCGeom { _opaque: [u8; 0] }

/// Opaque C struct: mjCHField (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCHField { _opaque: [u8; 0] }

/// Opaque C struct: mjCJoint (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCJoint { _opaque: [u8; 0] }

/// Opaque C struct: mjCKey (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCKey { _opaque: [u8; 0] }

/// Opaque C struct: mjCLight (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCLight { _opaque: [u8; 0] }

/// Opaque C struct: mjCMaterial (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCMaterial { _opaque: [u8; 0] }

/// Opaque C struct: mjCMesh (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCMesh { _opaque: [u8; 0] }

/// Opaque C struct: mjCModel (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCModel { _opaque: [u8; 0] }

/// Opaque C struct: mjCNumeric (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCNumeric { _opaque: [u8; 0] }

/// Opaque C struct: mjCOctree (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCOctree { _opaque: [u8; 0] }

/// Opaque C struct: mjCPair (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCPair { _opaque: [u8; 0] }

/// Opaque C struct: mjCPlugin (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCPlugin { _opaque: [u8; 0] }

/// Opaque C struct: mjCSensor (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCSensor { _opaque: [u8; 0] }

/// Opaque C struct: mjCSite (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCSite { _opaque: [u8; 0] }

/// Opaque C struct: mjCSkin (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCSkin { _opaque: [u8; 0] }

/// Opaque C struct: mjCTendon (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCTendon { _opaque: [u8; 0] }

/// Opaque C struct: mjCText (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCText { _opaque: [u8; 0] }

/// Opaque C struct: mjCTexture (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCTexture { _opaque: [u8; 0] }

/// Opaque C struct: mjCTuple (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCTuple { _opaque: [u8; 0] }

/// Opaque C struct: mjCWrap (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCWrap { _opaque: [u8; 0] }

/// Opaque C struct: mjDCMotorSlots (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjDCMotorSlots { _opaque: [u8; 0] }

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

/// Opaque C struct: mjListKeyMap (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjListKeyMap { _opaque: [u8; 0] }

/// Opaque C struct: mjPrimalContext (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjPrimalContext { _opaque: [u8; 0] }

/// Opaque C struct: mjSolverStat (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjSolverStat { _opaque: [u8; 0] }

/// Opaque C struct: mjStackInfo (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjStackInfo { _opaque: [u8; 0] }

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

/// Opaque C struct: mjXReader (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjXReader { _opaque: [u8; 0] }

/// Opaque C struct: mjXSchema (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjXSchema { _opaque: [u8; 0] }

/// Opaque C struct: mjXURDF (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjXURDF { _opaque: [u8; 0] }

/// Sized opaque C struct: mjXUtil (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjXUtil { pub _data: [u8; 8] }

/// Opaque C struct: mj_XMLPrinter (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mj_XMLPrinter { _opaque: [u8; 0] }

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

/// Opaque C struct: mjfLogHandler (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjfLogHandler { _opaque: [u8; 0] }

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

/// Opaque C struct: mjsExclude (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjsExclude { _opaque: [u8; 0] }

/// Opaque C struct: mjsWrap (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjsWrap { _opaque: [u8; 0] }

/// Sized opaque C struct: mjtBool (1 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjtBool { pub _data: [u8; 1] }

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

/// Opaque C struct: mujoco__user__FilePath (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mujoco__user__FilePath { _opaque: [u8; 0] }

/// Opaque C struct: pcg32_state (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pcg32_state { _opaque: [u8; 0] }

/// Opaque C struct: std__byte (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__byte { _opaque: [u8; 0] }

/// Opaque C struct: std__exception_ptr (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__exception_ptr { _opaque: [u8; 0] }

/// Opaque C struct: std__mutex (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__mutex { _opaque: [u8; 0] }

/// Opaque C struct: std__size_t (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__size_t { _opaque: [u8; 0] }

/// Opaque C struct: std__string (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__string { _opaque: [u8; 0] }

/// Opaque C struct: std__stringstream (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__stringstream { _opaque: [u8; 0] }

/// Opaque C struct: std__uint64_t (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct std__uint64_t { _opaque: [u8; 0] }

/// Opaque C struct: string (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct string { _opaque: [u8; 0] }

/// Sized opaque C struct: string_type (4 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct string_type { pub _data: [u8; 4] }

/// Opaque C struct: string_view (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct string_view { _opaque: [u8; 0] }

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

/// Opaque C struct: tinyxml2__XMLElement (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tinyxml2__XMLElement { _opaque: [u8; 0] }

/// Opaque C struct: type_parameter_0_0 (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct type_parameter_0_0 { _opaque: [u8; 0] }

/// Sized opaque C struct: union__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_src_engine_engine_collision_convex_h_52_3 (160 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct union__unnamed_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_src_engine_engine_collision_convex_h_52_3 { pub _data: [u8; 160] }

/// Opaque C struct: va_list (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct va_list { _opaque: [u8; 0] }

/// Opaque C struct: void_____T____const_char (unsized, pointer-only)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct void_____T____const_char { _opaque: [u8; 0] }

/// C enum: ElementClosingType
pub type ElementClosingType = u32;

/// C enum: LodePNGColorType
pub type LodePNGColorType = u32;

/// C enum: Status
pub type Status = i32;

/// C enum: mjtBias
pub type mjtBias = u32;

/// C enum: mjtCamLight
pub type mjtCamLight = u32;

/// C enum: mjtColorSpace
pub type mjtColorSpace = u32;

/// C enum: mjtConflict
pub type mjtConflict = u32;

/// C enum: mjtDataType
pub type mjtDataType = u32;

/// C enum: mjtDyn
pub type mjtDyn = u32;

/// C enum: mjtEq
pub type mjtEq = u32;

/// C enum: mjtGain
pub type mjtGain = u32;

/// C enum: mjtGeom
pub type mjtGeom = u32;

/// C enum: mjtGeomInertia
pub type mjtGeomInertia = u32;

/// C enum: mjtJoint
pub type mjtJoint = u32;

/// C enum: mjtLightType
pub type mjtLightType = u32;

/// C enum: mjtMeshBuiltin
pub type mjtMeshBuiltin = u32;

/// C enum: mjtMeshInertia
pub type mjtMeshInertia = u32;

/// C enum: mjtObj
pub type mjtObj = u32;

/// C enum: mjtOrientation
pub type mjtOrientation = u32;

/// C enum: mjtProjection
pub type mjtProjection = u32;

/// C enum: mjtSDFType
pub type mjtSDFType = u32;

/// C enum: mjtSensor
pub type mjtSensor = u32;

/// C enum: mjtSleepPolicy
pub type mjtSleepPolicy = u32;

/// C enum: mjtSleepState
pub type mjtSleepState = i32;

/// C enum: mjtStage
pub type mjtStage = u32;

/// C enum: mjtState
pub type mjtState = u32;

/// C enum: mjtTexture
pub type mjtTexture = u32;

/// C enum: mjtTrn
pub type mjtTrn = u32;

/// C enum: mjtWrap
pub type mjtWrap = u32;

/// C struct: GlobalModel (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct GlobalModel { pub _data: [u8; 16] }

/// C struct: ReentrantWriteLock (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct ReentrantWriteLock { pub _data: [u8; 8] }

/// C struct: StrPair (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct StrPair { pub _data: [u8; 24] }

/// C struct: XMLAttribute (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct XMLAttribute { pub _data: [u8; 80] }

/// C struct: XMLElement (120 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct XMLElement { pub _data: [u8; 120] }

/// C struct: ccd_vec3_t (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct ccd_vec3_t { pub _data: [u8; 24] }

/// C struct: fs::path (4 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct fs__path { pub _data: [u8; 4] }

/// C struct: mjCCDObj (376 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCCDObj { pub _data: [u8; 376] }

/// C struct: mjCError (500 bytes, align 1)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mjCError { pub _data: [u8; 500] }

/// C struct: mjCache (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjCache { pub _data: [u8; 8] }

/// C struct: mjContact (576 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjContact { pub _data: [u8; 576] }

/// C struct: mjData (161872 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjData { pub _data: [u8; 161872] }

/// C struct: mjLROpt (72 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjLROpt { pub _data: [u8; 72] }

/// C struct: mjLogConfig (1032 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjLogConfig { pub _data: [u8; 1032] }

/// C struct: mjLogMessage (1064 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjLogMessage { pub _data: [u8; 1064] }

/// C struct: mjMap (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjMap { pub _data: [u8; 16] }

/// C struct: mjModel (5512 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjModel { pub _data: [u8; 5512] }

/// C struct: mjOption (304 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjOption { pub _data: [u8; 304] }

/// C struct: mjPreContact (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjPreContact { pub _data: [u8; 80] }

/// C struct: mjPrimalPnt (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjPrimalPnt { pub _data: [u8; 32] }

/// C struct: mjResource (544 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjResource { pub _data: [u8; 544] }

/// C struct: mjSDF (48 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjSDF { pub _data: [u8; 48] }

/// C struct: mjSpec (1352 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjSpec { pub _data: [u8; 1352] }

/// C struct: mjStatistic (56 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjStatistic { pub _data: [u8; 56] }

/// C struct: mjUI (3009832 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjUI { pub _data: [u8; 3009832] }

/// C struct: mjVFS (8 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjVFS { pub _data: [u8; 8] }

/// C struct: mjVisual (632 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjVisual { pub _data: [u8; 632] }

/// C struct: mjXBase (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjXBase { pub _data: [u8; 16] }

/// C struct: mjXWriter (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjXWriter { pub _data: [u8; 32] }

/// C struct: mjpDecoder (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjpDecoder { pub _data: [u8; 32] }

/// C struct: mjpEncoder (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjpEncoder { pub _data: [u8; 32] }

/// C struct: mjpPlugin (152 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjpPlugin { pub _data: [u8; 152] }

/// C struct: mjpResourceProvider (64 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjpResourceProvider { pub _data: [u8; 64] }

/// C struct: mjrContext (61512 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjrContext { pub _data: [u8; 61512] }

/// C struct: mjrRect (16 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjrRect { pub _data: [u8; 16] }

/// C struct: mjsActuator (576 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsActuator { pub _data: [u8; 576] }

/// C struct: mjsBody (568 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsBody { pub _data: [u8; 568] }

/// C struct: mjsCamera (320 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsCamera { pub _data: [u8; 320] }

/// C struct: mjsCompiler (168 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsCompiler { pub _data: [u8; 168] }

/// C struct: mjsDefault (104 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsDefault { pub _data: [u8; 104] }

/// C struct: mjsElement (16 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsElement { pub _data: [u8; 16] }

/// C struct: mjsEquality (200 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsEquality { pub _data: [u8; 200] }

/// C struct: mjsFlex (352 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsFlex { pub _data: [u8; 352] }

/// C struct: mjsFrame (216 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsFrame { pub _data: [u8; 216] }

/// C struct: mjsGeom (584 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsGeom { pub _data: [u8; 584] }

/// C struct: mjsHField (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsHField { pub _data: [u8; 80] }

/// C struct: mjsJoint (360 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsJoint { pub _data: [u8; 360] }

/// C struct: mjsKey (72 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsKey { pub _data: [u8; 72] }

/// C struct: mjsLight (168 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsLight { pub _data: [u8; 168] }

/// C struct: mjsMaterial (80 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsMaterial { pub _data: [u8; 80] }

/// C struct: mjsMesh (232 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsMesh { pub _data: [u8; 232] }

/// C struct: mjsNumeric (32 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsNumeric { pub _data: [u8; 32] }

/// C struct: mjsOrientation (136 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsOrientation { pub _data: [u8; 136] }

/// C struct: mjsPair (168 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsPair { pub _data: [u8; 168] }

/// C struct: mjsPlugin (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsPlugin { pub _data: [u8; 40] }

/// C struct: mjsSensor (168 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsSensor { pub _data: [u8; 168] }

/// C struct: mjsSite (328 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsSite { pub _data: [u8; 328] }

/// C struct: mjsSkin (120 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsSkin { pub _data: [u8; 120] }

/// C struct: mjsTendon (304 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsTendon { pub _data: [u8; 304] }

/// C struct: mjsText (24 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsText { pub _data: [u8; 24] }

/// C struct: mjsTexture (192 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsTexture { pub _data: [u8; 192] }

/// C struct: mjsTuple (40 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjsTuple { pub _data: [u8; 40] }

/// C struct: mjtSAP (8 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjtSAP { pub _data: [u8; 8] }

/// C struct: mjuiDef (360 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiDef { pub _data: [u8; 360] }

/// C struct: mjuiItem (1504 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiItem { pub _data: [u8; 1504] }

/// C struct: mjuiSection (300904 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiSection { pub _data: [u8; 300904] }

/// C struct: mjuiState (544 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjuiState { pub _data: [u8; 544] }

/// C struct: mjvCamera (72 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjvCamera { pub _data: [u8; 72] }

/// C struct: mjvFigure (813748 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvFigure { pub _data: [u8; 813748] }

/// C struct: mjvGLCamera (64 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvGLCamera { pub _data: [u8; 64] }

/// C struct: mjvGeom (236 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvGeom { pub _data: [u8; 236] }

/// C struct: mjvLight (108 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvLight { pub _data: [u8; 108] }

/// C struct: mjvOption (92 bytes, align 4)
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct mjvOption { pub _data: [u8; 92] }

/// C struct: mjvPerturb (144 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjvPerturb { pub _data: [u8; 144] }

/// C struct: mjvScene (11184 bytes, align 8)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct mjvScene { pub _data: [u8; 11184] }

