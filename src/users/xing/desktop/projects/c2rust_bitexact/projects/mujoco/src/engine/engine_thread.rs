//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_thread.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ThreadPoolContext::Dispatch (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_thread.cc:51)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_dispatch(self_ptr: *mut ThreadPoolContext, model: *const mjModel, data: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32) {
    todo!() // cxx:__ZN17ThreadPoolContext8DispatchEPK8mjModel_P7mjData_PFvS2_S4_PviiES5_i
}

/// C: ThreadPoolContext::ThreadCount (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_thread.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_thread_count(self_ptr: *mut ThreadPoolContext) -> i32 {
    todo!() // cxx:__ZNK17ThreadPoolContext11ThreadCountEv
}

/// C: ThreadPoolContext::Worker (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_thread.cc:83)
#[allow(unused_variables, non_snake_case)]
pub fn thread_pool_context_worker(self_ptr: *mut ThreadPoolContext, threadId: i32) {
    todo!() // cxx:__ZN17ThreadPoolContext6WorkerEi
}

/// C: mju_threadpool (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_thread.h:30)
/// Calls: cxx:__ZNK17ThreadPoolContext11ThreadCountEv
#[allow(unused_variables, non_snake_case)]
pub fn mju_threadpool(d: *mut mjData, nthread: i32) {
    todo!() // cxx:_mju_threadpool
}

/// C: mju_numThread (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_thread.h:33)
/// Calls: cxx:__ZNK17ThreadPoolContext11ThreadCountEv
#[allow(unused_variables, non_snake_case)]
pub fn mju_num_thread(d: *const mjData) -> i32 {
    todo!() // cxx:_mju_numThread
}

/// C: mju_dispatch (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_thread.h:36)
/// Calls: cxx:__ZN17ThreadPoolContext8DispatchEPK8mjModel_P7mjData_PFvS2_S4_PviiES5_i, cxx:_mj_freeStack, cxx:_mj_markStack
#[allow(unused_variables, non_snake_case)]
pub fn mju_dispatch(m: *const mjModel, d: *mut mjData, func: mjTaskFunc, arg: *mut (), ntask: i32) {
    todo!() // cxx:_mju_dispatch
}

