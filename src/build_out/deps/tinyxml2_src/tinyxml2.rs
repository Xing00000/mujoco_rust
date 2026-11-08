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

/// C: XMLComment::ToComment (build_out/_deps/tinyxml2-src/tinyxml2.h:1036)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_comment_to_comment(self_ptr: *mut tinyxml2__XMLComment) -> *mut tinyxml2__XMLComment {
    // MACHINE-PROVEN: Clang AST semantics
    self_ptr as *mut tinyxml2__XMLComment
}

pub fn xml_util_is_utf8continuation(p: i8) -> bool {
    ((p as i32) & 0x80) != 0
}

/// C: XMLNode::NoChildren (build_out/_deps/tinyxml2-src/tinyxml2.h:765)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_node_no_children(self_ptr: *mut tinyxml2__XMLNode) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { ((*self_ptr)._firstChild == ::core::ptr::null_mut()) }
}

/// C: XMLConstHandle::XMLConstHandle (build_out/_deps/tinyxml2-src/tinyxml2.h:2142)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_const_handle_xml_const_handle_tinyxml2__xmlconsthandle_ptr_6d5f4cca(self_ptr: *mut tinyxml2__XMLConstHandle, r#ref: *const tinyxml2__XMLConstHandle) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::write(core::ptr::addr_of_mut!((*self_ptr)._node), (*r#ref)._node); }
}

/// C: XMLConstHandle::XMLConstHandle (build_out/_deps/tinyxml2-src/tinyxml2.h:2142)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_const_handle_xml_const_handle_tinyxml2__xmlconsthandle_ptr_31ff2366(self_ptr: *mut tinyxml2__XMLConstHandle, r#ref: *const tinyxml2__XMLConstHandle) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::write(core::ptr::addr_of_mut!((*self_ptr)._node), (*r#ref)._node); }
}

/// C: StrPair::Empty (build_out/_deps/tinyxml2-src/tinyxml2.h:163)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn str_pair_empty(self_ptr: *mut tinyxml2__StrPair) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { ((*self_ptr)._start == (*self_ptr)._end) }
}

/// C: XMLUnknown::ToUnknown (build_out/_deps/tinyxml2-src/tinyxml2.h:1113)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_unknown_to_unknown(self_ptr: *mut tinyxml2__XMLUnknown) -> *const tinyxml2__XMLUnknown {
    // MACHINE-PROVEN: Clang AST semantics
    self_ptr as *const tinyxml2__XMLUnknown
}

/// C: XMLHandle::XMLHandle (build_out/_deps/tinyxml2-src/tinyxml2.h:2058)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_handle_xml_handle_tinyxml2__xmlnode_ptr_90b56a4d(self_ptr: *mut tinyxml2__XMLHandle, node: *mut tinyxml2__XMLNode) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::write(core::ptr::addr_of_mut!((*self_ptr)._node), node); }
}

/// C: XMLHandle::XMLHandle (build_out/_deps/tinyxml2-src/tinyxml2.h:2058)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn xml_handle_xml_handle_tinyxml2__xmlnode_ptr_af481ba9(self_ptr: *mut tinyxml2__XMLHandle, node: *mut tinyxml2__XMLNode) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::write(core::ptr::addr_of_mut!((*self_ptr)._node), node); }
}

/// C: XMLDocument::ShallowClone (build_out/_deps/tinyxml2-src/tinyxml2.h:1925)
#[allow(unused_variables, non_snake_case)]
pub fn xml_document_shallow_clone(self_ptr: *mut tinyxml2__XMLDocument, arg0: *mut tinyxml2__XMLDocument) -> *mut tinyxml2__XMLNode {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { ::core::ptr::null_mut() }
}

/// C: XMLPrinter::VisitExit (build_out/_deps/tinyxml2-src/tinyxml2.h:2297)
#[allow(unused_variables, non_snake_case)]
pub fn xml_printer_visit_exit(self_ptr: *mut tinyxml2__XMLPrinter, arg0: *const tinyxml2__XMLDocument) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { true }
}
