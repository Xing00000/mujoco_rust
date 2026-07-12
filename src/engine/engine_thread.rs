//! Port of: engine/engine_thread.cc
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ThreadPoolContext::Dispatch (engine/engine_thread.cc:51)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_dispatch(self_ptr: *mut ThreadPoolContext, model: *const mjModel, data: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut ThreadPoolContext, model : * const mjModel, data : * mut mjData, func : mjTaskFunc, arg : * mut (), ntask : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: ThreadPoolContext::ThreadCount (engine/engine_thread.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_thread_count(self_ptr: *mut ThreadPoolContext) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut ThreadPoolContext)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: ThreadPoolContext::Worker (engine/engine_thread.cc:83)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_worker(self_ptr: *mut ThreadPoolContext, threadId: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut ThreadPoolContext, threadId : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_threadpool (engine/engine_thread.h:30)
/// Calls: ThreadPoolContext::ThreadCount
#[allow(unused_variables, non_snake_case)]
pub fn mju_threadpool(d: *mut mjData, nthread: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * mut mjData, nthread : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_numThread (engine/engine_thread.h:33)
/// Calls: ThreadPoolContext::ThreadCount
#[allow(unused_variables, non_snake_case)]
pub fn mju_num_thread(d: *const mjData) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * const mjData)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_dispatch (engine/engine_thread.h:36)
/// Calls: ThreadPoolContext::Dispatch, mj_freeStack, mj_markStack
#[allow(unused_variables, non_snake_case)]
pub fn mju_dispatch(m: *const mjModel, d: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, func : mjTaskFunc, arg : * mut (), ntask : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

