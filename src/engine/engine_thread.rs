//! Port of: engine/engine_thread.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ThreadPoolContext::Dispatch (engine/engine_thread.cc:51)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_dispatch(self_ptr: *mut ThreadPoolContext, model: *const mjModel, data: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32) {
    extern "C" {
        fn ThreadPoolContext_Dispatch_impl(self_ptr: *mut ThreadPoolContext, model: *const mjModel, data: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { ThreadPoolContext_Dispatch_impl(self_ptr, model, data, func, arg, ntask) }
}

/// C: ThreadPoolContext::ThreadCount (engine/engine_thread.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_thread_count(self_ptr: *mut ThreadPoolContext) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut ThreadPoolContext)
    // Previous return: i32
    extern "C" { fn ThreadPoolContext_ThreadCount_impl (self_ptr : * mut ThreadPoolContext) -> i32 ; } unsafe { ThreadPoolContext_ThreadCount_impl (self_ptr) }
}

/// C: ThreadPoolContext::Worker (engine/engine_thread.cc:83)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_worker(self_ptr: *mut ThreadPoolContext, threadId: i32) {
    extern "C" { fn ThreadPoolContext_Worker_impl(self_ptr: *mut ThreadPoolContext, threadId: i32); }
    // SAFETY: delegates to C implementation
    unsafe { ThreadPoolContext_Worker_impl(self_ptr, threadId) }
}

/// C: mju_threadpool (engine/engine_thread.h:30)
/// Calls: ThreadPoolContext::ThreadCount
#[allow(unused_variables, non_snake_case)]
pub fn mju_threadpool(d: *mut mjData, nthread: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, nthread : i32)
    // Previous return: ()
    extern "C" { fn mju_threadpool_impl (d : * mut mjData , nthread : i32) ; } unsafe { mju_threadpool_impl (d , nthread) }
}

/// C: mju_numThread (engine/engine_thread.h:33)
/// Calls: ThreadPoolContext::ThreadCount
#[allow(unused_variables, non_snake_case)]
pub fn mju_num_thread(d: *const mjData) -> i32 {
    extern "C" {
        fn mju_numThread_impl(d: *const mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_numThread_impl(d) }
}

/// C: mju_dispatch (engine/engine_thread.h:36)
/// Calls: ThreadPoolContext::Dispatch, mj_freeStack, mj_markStack
#[allow(unused_variables, non_snake_case)]
pub fn mju_dispatch(m: *const mjModel, d: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32) {
    extern "C" { fn mju_dispatch_impl(m: *const mjModel, d: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_dispatch_impl(m, d, func, arg, ntask) }
}

