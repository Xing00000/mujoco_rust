//! Port of: engine/engine_thread.cc
//! IR hash: 699b5f0da57e8d78
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ThreadPoolContext::Dispatch (engine/engine_thread.cc:51)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_dispatch(model: *const mjModel, data: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (model : * const mjModel, data : * mut mjData, func : mjTaskFunc, arg : * mut (), ntask : i32)
    // Previous return: ()
    todo ! ()
}

/// C: ThreadPoolContext::ThreadCount (engine/engine_thread.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_thread_count() -> i32 {
    todo ! ()
}

/// C: ThreadPoolContext::Worker (engine/engine_thread.cc:83)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_worker(threadId: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (threadId : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_threadpool (engine/engine_thread.h:30)
/// Calls: ThreadPoolContext::ThreadCount
#[allow(unused_variables, non_snake_case)]
pub fn mju_threadpool(d: *mut mjData, nthread: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, nthread : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_numThread (engine/engine_thread.h:33)
/// Calls: ThreadPoolContext::ThreadCount
#[allow(unused_variables, non_snake_case)]
pub fn mju_num_thread(d: *const mjData) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (d : * const mjData)
    // Previous return: i32
    todo ! ()
}

/// C: mju_dispatch (engine/engine_thread.h:36)
/// Calls: ThreadPoolContext::Dispatch, mj_freeStack, mj_markStack
#[allow(unused_variables, non_snake_case)]
pub fn mju_dispatch(m: *const mjModel, d: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, func : mjTaskFunc, arg : * mut (), ntask : i32)
    // Previous return: ()
    todo ! ()
}

