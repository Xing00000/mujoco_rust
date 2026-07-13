//! Port of: user/user_threadpool.h
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ThreadPool::NumThreads (user/user_threadpool.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_num_threads(self_ptr: *mut ThreadPool) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut ThreadPool)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: ThreadPool::WorkerId (user/user_threadpool.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_worker_id() -> i32 {
    todo ! ()
}

/// C: ThreadPool::Schedule (user/user_threadpool.h:46)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_schedule(self_ptr: *mut ThreadPool, task: *const ()) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut ThreadPool, task : * const ())
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: ThreadPool::GetCount (user/user_threadpool.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_get_count(self_ptr: *mut ThreadPool) -> std__uint64_t {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut ThreadPool)
    // Previous return: std__uint64_t
    todo!("re-translate: params renamed")
}

/// C: ThreadPool::ResetCount (user/user_threadpool.h:55)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_reset_count(self_ptr: *mut ThreadPool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut ThreadPool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: ThreadPool::WaitCount (user/user_threadpool.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_wait_count(self_ptr: *mut ThreadPool, value: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut ThreadPool, value : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: ThreadPool::WorkerThread (user/user_threadpool.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_worker_thread(self_ptr: *mut ThreadPool, i: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut ThreadPool, i : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

