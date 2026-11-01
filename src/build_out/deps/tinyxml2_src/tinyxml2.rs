//! Port of: build_out/_deps/tinyxml2-src/tinyxml2.h
//! IR hash: cfde9898e19bc556
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: XMLConstHandle::XMLConstHandle (build_out/_deps/tinyxml2-src/tinyxml2.h:2138)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_const_handle_xml_const_handle_tinyxml2__xmlnode_ptr_8ab8562d(self_ptr: *mut tinyxml2__XMLConstHandle, node: *const tinyxml2__XMLNode) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::write(core::ptr::addr_of_mut!((*self_ptr)._node), node); }
}

/// C: XMLConstHandle::XMLConstHandle (build_out/_deps/tinyxml2-src/tinyxml2.h:2138)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_const_handle_xml_const_handle_tinyxml2__xmlnode_ptr_f7c3c026(self_ptr: *mut tinyxml2__XMLConstHandle, node: *const tinyxml2__XMLNode) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::write(core::ptr::addr_of_mut!((*self_ptr)._node), node); }
}

/// C: XMLDocument::ProcessEntities (build_out/_deps/tinyxml2-src/tinyxml2.h:1790)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_document_process_entities(self_ptr: *mut tinyxml2__XMLDocument) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr)._processEntities }
}

/// C: XMLText::ToText (build_out/_deps/tinyxml2-src/tinyxml2.h:1001)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_text_to_text(self_ptr: *mut tinyxml2__XMLText) -> *const tinyxml2__XMLText {
    // MACHINE-PROVEN: Clang AST semantics
    self_ptr as *const tinyxml2__XMLText
}
