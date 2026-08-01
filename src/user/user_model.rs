//! Port of: user/user_model.h
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

pub use crate::types::mjCModel;
impl crate::types::mjCModel {
    /// C: mjCModel::IsCompiled (user/user_model.h:249)
    #[allow(unused_variables, non_snake_case)]
    pub fn IsCompiled(self_ptr: *mut mjCModel) -> bool {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).compiled }
    }
    

    /// C: mjCModel::GetWarnings (user/user_model.h:256)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetWarnings(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).warnings_ }
    }
    

    /// C: mjCModel::ActivePlugins (user/user_model.h:291)
    #[allow(unused_variables, non_snake_case)]
    pub fn ActivePlugins(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).active_plugins_ }
    }
    

    /// C: mjCModel::Flexes (user/user_model.h:295)
    #[allow(unused_variables, non_snake_case)]
    pub fn Flexes(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).flexes_ }
    }
    

    /// C: mjCModel::Meshes (user/user_model.h:296)
    #[allow(unused_variables, non_snake_case)]
    pub fn Meshes(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).meshes_ }
    }
    

    /// C: mjCModel::Skins (user/user_model.h:297)
    #[allow(unused_variables, non_snake_case)]
    pub fn Skins(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).skins_ }
    }
    

    /// C: mjCModel::HFields (user/user_model.h:298)
    #[allow(unused_variables, non_snake_case)]
    pub fn HFields(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).hfields_ }
    }
    

    /// C: mjCModel::Textures (user/user_model.h:299)
    #[allow(unused_variables, non_snake_case)]
    pub fn Textures(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).textures_ }
    }
    

    /// C: mjCModel::Materials (user/user_model.h:300)
    #[allow(unused_variables, non_snake_case)]
    pub fn Materials(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).materials_ }
    }
    

    /// C: mjCModel::Pairs (user/user_model.h:301)
    #[allow(unused_variables, non_snake_case)]
    pub fn Pairs(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).pairs_ }
    }
    

    /// C: mjCModel::Excludes (user/user_model.h:302)
    #[allow(unused_variables, non_snake_case)]
    pub fn Excludes(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).excludes_ }
    }
    

    /// C: mjCModel::Equalities (user/user_model.h:303)
    #[allow(unused_variables, non_snake_case)]
    pub fn Equalities(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).equalities_ }
    }
    

    /// C: mjCModel::Tendons (user/user_model.h:304)
    #[allow(unused_variables, non_snake_case)]
    pub fn Tendons(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).tendons_ }
    }
    

    /// C: mjCModel::Actuators (user/user_model.h:305)
    #[allow(unused_variables, non_snake_case)]
    pub fn Actuators(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).actuators_ }
    }
    

    /// C: mjCModel::Sensors (user/user_model.h:306)
    #[allow(unused_variables, non_snake_case)]
    pub fn Sensors(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).sensors_ }
    }
    

    /// C: mjCModel::Numerics (user/user_model.h:307)
    #[allow(unused_variables, non_snake_case)]
    pub fn Numerics(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).numerics_ }
    }
    

    /// C: mjCModel::Texts (user/user_model.h:308)
    #[allow(unused_variables, non_snake_case)]
    pub fn Texts(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).texts_ }
    }
    

    /// C: mjCModel::Tuples (user/user_model.h:309)
    #[allow(unused_variables, non_snake_case)]
    pub fn Tuples(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).tuples_ }
    }
    

    /// C: mjCModel::Keys (user/user_model.h:310)
    #[allow(unused_variables, non_snake_case)]
    pub fn Keys(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).keys_ }
    }
    

    /// C: mjCModel::Plugins (user/user_model.h:311)
    #[allow(unused_variables, non_snake_case)]
    pub fn Plugins(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).plugins_ }
    }
    

    /// C: mjCModel::Bodies (user/user_model.h:312)
    #[allow(unused_variables, non_snake_case)]
    pub fn Bodies(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).bodies_ }
    }
    

    /// C: mjCModel::Geoms (user/user_model.h:313)
    #[allow(unused_variables, non_snake_case)]
    pub fn Geoms(self_ptr: *mut mjCModel) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).geoms_ }
    }
    

    /// C: mjCModel::SetDeepCopy (user/user_model.h:347)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetDeepCopy(self_ptr: *mut mjCModel, deepcopy: bool) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).deepcopy_ = deepcopy; }
    }
    

    /// C: mjCModel::IsAttached (user/user_model.h:353)
    #[allow(unused_variables, non_snake_case)]
    pub fn IsAttached(self_ptr: *mut mjCModel) -> bool {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).attached_ }
    }
    

    /// C: mjCModel::GetRef (user/user_model.h:360)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetRef(self_ptr: *mut mjCModel) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).refcount }
    }
    

}

