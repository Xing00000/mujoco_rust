//! Port of: user/user_threadpool.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ThreadPool::Schedule (user/user_threadpool.cc:45)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_schedule(self_ptr: *mut mujoco__user__ThreadPool, task: *const ()) {
    todo!() // cxx:__ZN6mujoco4user10ThreadPool8ScheduleENSt3__18functionIFvvEEE
}

/// C: ThreadPool::WorkerThread (user/user_threadpool.cc:52)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_worker_thread(self_ptr: *mut mujoco__user__ThreadPool, i: i32) {
    todo!() // cxx:__ZN6mujoco4user10ThreadPool12WorkerThreadEi
}

/// C: ThreadPool::NumThreads (user/user_threadpool.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_num_threads(self_ptr: *mut mujoco__user__ThreadPool) -> i32 {
    todo!() // cxx:__ZNK6mujoco4user10ThreadPool10NumThreadsEv
}

/// C: ThreadPool::WorkerId (user/user_threadpool.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_worker_id() -> i32 {
    todo!() // cxx:__ZN6mujoco4user10ThreadPool8WorkerIdEv
}

/// C: ThreadPool::GetCount (user/user_threadpool.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_get_count(self_ptr: *mut mujoco__user__ThreadPool) -> u64 {
    todo!() // cxx:__ZN6mujoco4user10ThreadPool8GetCountEv
}

/// C: ThreadPool::ResetCount (user/user_threadpool.h:55)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_reset_count(self_ptr: *mut mujoco__user__ThreadPool) {
    todo!() // cxx:__ZN6mujoco4user10ThreadPool10ResetCountEv
}

/// C: ThreadPool::WaitCount (user/user_threadpool.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_wait_count(self_ptr: *mut mujoco__user__ThreadPool, value: i32) {
    todo!() // cxx:__ZN6mujoco4user10ThreadPool9WaitCountEi
}

