//! Port of: xml/xml_native_reader.cc
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

pub use crate::types::Reader;
impl crate::types::Reader {
    /// C: Reader::set_node (xml/xml_native_reader.cc:121)
    #[allow(unused_variables, non_snake_case)]
    pub fn set_node(self_ptr: *mut Reader, node: *mut tinyxml2__XMLElement) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).xml_node_ = node; }
    }
    

}

