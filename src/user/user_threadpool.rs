//! Port of: user/user_threadpool.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ThreadPool::NumThreads (user/user_threadpool.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_num_threads(self_ptr: *mut ThreadPool) -> i32 {
    extern "C" { fn ThreadPool_NumThreads(self_ptr: *mut ThreadPool) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { ThreadPool_NumThreads(self_ptr) }
}

/// C: ThreadPool::WorkerId (user/user_threadpool.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_worker_id() -> i32 {
    extern "C" { fn ThreadPool_WorkerId() -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { ThreadPool_WorkerId() }
}

/// C: ThreadPool::Schedule (user/user_threadpool.h:46)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_schedule(self_ptr: *mut ThreadPool, task: *const ()) {
    extern "C" { fn thread_pool_schedule(self_ptr: *mut ThreadPool, task: *const ()); }
    // SAFETY: delegates to C implementation
    unsafe { thread_pool_schedule(self_ptr, task) }
}

/// C: ThreadPool::GetCount (user/user_threadpool.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_get_count(self_ptr: *mut ThreadPool) -> std__uint64_t {
    extern "C" { fn ThreadPool_GetCount(self_ptr: *mut ThreadPool) -> std__uint64_t; }
    // SAFETY: delegates to C implementation
    unsafe { ThreadPool_GetCount(self_ptr) }
}

/// C: ThreadPool::ResetCount (user/user_threadpool.h:55)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_reset_count(self_ptr: *mut ThreadPool) {
    extern "C" { fn ThreadPool_ResetCount(self_ptr: *mut ThreadPool); }
    // SAFETY: delegates to C implementation
    unsafe { ThreadPool_ResetCount(self_ptr) }
}

/// C: ThreadPool::WaitCount (user/user_threadpool.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_wait_count(self_ptr: *mut ThreadPool, value: i32) {
    extern "C" { fn ThreadPool_WaitCount(self_ptr: *mut ThreadPool, value: i32); }
    // SAFETY: delegates to C implementation
    unsafe { ThreadPool_WaitCount(self_ptr, value) }
}

/// C: ThreadPool::WorkerThread (user/user_threadpool.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_worker_thread(self_ptr: *mut ThreadPool, i: i32) {
    extern "C" { fn ThreadPool_WorkerThread(self_ptr: *mut ThreadPool, i: i32); }
    // SAFETY: delegates to C implementation
    unsafe { ThreadPool_WorkerThread(self_ptr, i) }
}

