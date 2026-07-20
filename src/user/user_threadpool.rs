//! Port of: user/user_threadpool.h
//! IR hash: 73a9f665ec0ecfc0
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ThreadPool::NumThreads (user/user_threadpool.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_num_threads(self_ptr: *mut ThreadPool) -> i32 {
    todo!() // ThreadPool::NumThreads
}

/// C: ThreadPool::WorkerId (user/user_threadpool.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_worker_id() -> i32 {
    todo ! ()
}

/// C: ThreadPool::Schedule (user/user_threadpool.h:46)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_schedule(self_ptr: *mut ThreadPool, task: *const ()) {
    todo!() // ThreadPool::Schedule
}

/// C: ThreadPool::GetCount (user/user_threadpool.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_get_count(self_ptr: *mut ThreadPool) -> u64 {
    todo!() // ThreadPool::GetCount
}

/// C: ThreadPool::ResetCount (user/user_threadpool.h:55)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_reset_count(self_ptr: *mut ThreadPool) {
    todo!() // ThreadPool::ResetCount
}

/// C: ThreadPool::WaitCount (user/user_threadpool.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_wait_count(self_ptr: *mut ThreadPool, value: i32) {
    todo!() // ThreadPool::WaitCount
}

/// C: ThreadPool::WorkerThread (user/user_threadpool.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_worker_thread(self_ptr: *mut ThreadPool, i: i32) {
    todo!() // ThreadPool::WorkerThread
}

