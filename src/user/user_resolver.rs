//! Port of: user/user_resolver.cc
//! IR hash: 71dbc8f1e7b9706a
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


pub unsafe fn __c2rust_conflict_subject_1942dc2fc1c77996b39c9f30cb3c2faf67d9573958dbdfdf3e9aae0cc8b4f114(c2rust_result_out: *mut std__string, parent: *const mjSpec, child: *const mjSpec) -> () { unsafe {
extern "C" {
fn c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(destination: *mut core::ffi::c_void, left: *mut core::ffi::c_void, text: *const core::ffi::c_char);
fn c2rust_string_append_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(destination: *mut core::ffi::c_void, left: *mut core::ffi::c_void, right: *const core::ffi::c_void);
fn c2rust_string_construct_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(destination: *mut core::ffi::c_void, text: *const core::ffi::c_char);
fn c2rust_string_copy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(destination: *mut core::ffi::c_void, source: *const core::ffi::c_void);
fn c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(value: *mut core::ffi::c_void);
fn c2rust_string_empty_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(value: *const core::ffi::c_void) -> bool;
fn c2rust_string_equal_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(value: *const core::ffi::c_void, text: *const core::ffi::c_char) -> bool;
fn c2rust_string_prepend_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(destination: *mut core::ffi::c_void, text: *const core::ffi::c_char, right: *const core::ffi::c_void);
}
let conflict = (parent as *const u8).add(76).cast::<u32>().read();
let mode = if conflict == 1 { c"merge" } else if conflict == 2 { c"error" } else { c"warning" };
let mut pname = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
let mut cname = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_copy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(pname.as_mut_ptr().cast(), (parent as *const u8).add(8).cast::<*const core::ffi::c_void>().read());
c2rust_string_copy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(cname.as_mut_ptr().cast(), (child as *const u8).add(8).cast::<*const core::ffi::c_void>().read());
let has_parent = !c2rust_string_equal_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(pname.as_ptr().cast(), c"MuJoCo Model".as_ptr()) && !c2rust_string_empty_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(pname.as_ptr().cast());
let has_child = !c2rust_string_equal_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(cname.as_ptr().cast(), c"MuJoCo Model".as_ptr()) && !c2rust_string_empty_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(cname.as_ptr().cast());
if has_child && has_parent {
let mut t1 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_prepend_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t1.as_mut_ptr().cast(), c"Attach conflict when attaching '".as_ptr(), cname.as_ptr().cast());
let mut t2 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t2.as_mut_ptr().cast(), t1.as_mut_ptr().cast(), c"' to '".as_ptr());
let mut t3 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_append_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t3.as_mut_ptr().cast(), t2.as_mut_ptr().cast(), pname.as_ptr().cast());
let mut t4 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t4.as_mut_ptr().cast(), t3.as_mut_ptr().cast(), c"', policy is '".as_ptr());
let mut t5 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t5.as_mut_ptr().cast(), t4.as_mut_ptr().cast(), mode.as_ptr());
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(c2rust_result_out.cast(), t5.as_mut_ptr().cast(), c"'".as_ptr());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t5.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t4.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t3.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t2.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t1.as_mut_ptr().cast());
}
else if has_child {
let mut t1 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_prepend_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t1.as_mut_ptr().cast(), c"Attach conflict when attaching '".as_ptr(), cname.as_ptr().cast());
let mut t2 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t2.as_mut_ptr().cast(), t1.as_mut_ptr().cast(), c"', policy is '".as_ptr());
let mut t3 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t3.as_mut_ptr().cast(), t2.as_mut_ptr().cast(), mode.as_ptr());
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(c2rust_result_out.cast(), t3.as_mut_ptr().cast(), c"'".as_ptr());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t3.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t2.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t1.as_mut_ptr().cast());
}
else if has_parent {
let mut t1 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_prepend_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t1.as_mut_ptr().cast(), c"Attach conflict when attaching to '".as_ptr(), pname.as_ptr().cast());
let mut t2 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t2.as_mut_ptr().cast(), t1.as_mut_ptr().cast(), c"', policy is '".as_ptr());
let mut t3 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t3.as_mut_ptr().cast(), t2.as_mut_ptr().cast(), mode.as_ptr());
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(c2rust_result_out.cast(), t3.as_mut_ptr().cast(), c"'".as_ptr());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t3.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t2.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t1.as_mut_ptr().cast());
}
else {
let mut t1 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_construct_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t1.as_mut_ptr().cast(), c"Attach conflict on attach, policy is '".as_ptr());
let mut t2 = core::mem::MaybeUninit::<crate::types::std__string>::uninit();
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t2.as_mut_ptr().cast(), t1.as_mut_ptr().cast(), mode.as_ptr());
c2rust_string_append_c_string_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(c2rust_result_out.cast(), t2.as_mut_ptr().cast(), c"'".as_ptr());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t2.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(t1.as_mut_ptr().cast());
}
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(cname.as_mut_ptr().cast());
c2rust_string_destroy_1e259bb31cd35d03d0c34349ece756cccff8225ee5ea9dc48c991425959ebdcd(pname.as_mut_ptr().cast());
} }
