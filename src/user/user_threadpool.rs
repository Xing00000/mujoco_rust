//! Port of: user/user_threadpool.cc
//! IR hash: b692318cb377f559
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


pub unsafe fn __c2rust_thread_pool__thread_pool_e5c5ce9b2056a76f38607e0ad39b481aefc22a761d2e539d943e48a3030d4a10(self_ptr: *mut mujoco__user__ThreadPool) -> () {
    unsafe { __c2rust_thread_pool__thread_pool_42534a244cf5acb980cfac88f4fdb92ecac7ec5102c25f593d772d5527ce97f1(self_ptr); }
}

pub unsafe fn __c2rust_thread_pool__thread_pool_42534a244cf5acb980cfac88f4fdb92ecac7ec5102c25f593d772d5527ce97f1(self_ptr: *mut mujoco__user__ThreadPool) -> () {
    #[repr(C, align(8))]
    struct Scratch16([u8; 16]);
    #[repr(C, align(8))]
    struct Scratch32([u8; 32]);

    extern "C" {
        fn c2rust_library_call_345629260c10827e095766ac37730eb8232cc72da3e44b550552e613d14cc1a0(arg_0: *mut core::ffi::c_void, arg_1: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_c1b98303b46cdda76367dcffafc9a02f9cdcf747466ab357e094a303499aaf80(arg_0: *mut core::ffi::c_void) -> u64;
        fn c2rust_library_call_d311ec6461988c2b8578e524917e8bae8a0457d3dc1126bbfb5b804d429cd7ea(arg_0: *mut core::ffi::c_void, arg_1: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_dc46084967cdf97bb6af31193acc21ff0e234687bbc53e05491714439328511e(arg_0: *mut core::ffi::c_void, arg_1: *mut core::ffi::c_void) -> ();
        fn c2rust_library_call_a892a5b4b10c294b44be1783c466dd075ce80ddd848403520c4b5255ef62e26c(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_f84ad0cb952b7a3251e871ebeab574dc063641b9f041095352389fca2c0d4c51(arg_0: *mut core::ffi::c_void) -> ();
        fn c2rust_library_call_55663a7a67ecf35fc3a7b337611872cb097efdd7a1f844324e0d3376881042db(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_30a8d44d0a77e204bc81e42201a8db4a38cf2a5c6ed0677dc961bf0bb4065c85(arg_0: *mut core::ffi::c_void) -> u64;
        fn c2rust_library_call_6a3d74374123a7b23b51d54e0ce7c1e9d553c581b93c6013792e069656df8e65(arg_0: *mut core::ffi::c_void) -> u64;
        fn c2rust_library_call_796045d8f3b86671ef17599e90f2bf2505b818ecf6663316509c8635d39dbb66(arg_0: *mut core::ffi::c_void, arg_1: *mut core::ffi::c_void) -> u64;
        fn c2rust_library_call_7e05e05deddac3472377e96b06123dccd67e969e83bb817de352c81c8fe31676(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_6082a29f95fd0b6d359b51efe16774dc345c0753142cc5756e469ef7baf223cf(arg_0: *mut core::ffi::c_void) -> ();
        fn c2rust_library_call_eeb9cb64fda98ec2bc6b3ad737f43e50d5780597687fd39549da8ccc66d4a689(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_4e329bba7681da9d273f2bf4de410ccb1f48f81ba004530c51f5f1fe275cd813(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_c1954d599c59e4f0faaab7a1f7070104e82362050fc434162ad0d1cb874d6ae8(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_6badf988275b77422a7c447fa7d6b43d0a45a46e06fa1d10659af06759bf485b(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_8f01498b81cee3321c485a35260a76307b0274968efe494df9cae9ffbbf3874b(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    }

    let threads = unsafe { self_ptr.cast::<u8>().add(0).cast::<core::ffi::c_void>() };
    let mutex = unsafe { self_ptr.cast::<u8>().add(24).cast::<core::ffi::c_void>() };
    let cv_in = unsafe { self_ptr.cast::<u8>().add(88).cast::<core::ffi::c_void>() };
    let cv_ext = unsafe { self_ptr.cast::<u8>().add(136).cast::<core::ffi::c_void>() };
    let queue = unsafe { self_ptr.cast::<u8>().add(184).cast::<core::ffi::c_void>() };

    let mut lock_storage = core::mem::MaybeUninit::<Scratch16>::uninit();
    let lock_ptr = lock_storage.as_mut_ptr().cast::<core::ffi::c_void>();
    unsafe { c2rust_library_call_345629260c10827e095766ac37730eb8232cc72da3e44b550552e613d14cc1a0(lock_ptr, mutex); }

    let mut counter: i32 = 0;
    while ((counter as i64) as u64) < unsafe { c2rust_library_call_c1b98303b46cdda76367dcffafc9a02f9cdcf747466ab357e094a303499aaf80(threads) } {
        let mut temporary = core::mem::MaybeUninit::<Scratch32>::uninit();
        let temporary_ptr = temporary.as_mut_ptr().cast::<core::ffi::c_void>();
        unsafe {
            c2rust_library_call_d311ec6461988c2b8578e524917e8bae8a0457d3dc1126bbfb5b804d429cd7ea(temporary_ptr, core::ptr::null_mut());
            c2rust_library_call_dc46084967cdf97bb6af31193acc21ff0e234687bbc53e05491714439328511e(queue, temporary_ptr);
            c2rust_library_call_a892a5b4b10c294b44be1783c466dd075ce80ddd848403520c4b5255ef62e26c(temporary_ptr);
        }
        counter = counter.wrapping_add(1);
    }

    unsafe { c2rust_library_call_f84ad0cb952b7a3251e871ebeab574dc063641b9f041095352389fca2c0d4c51(cv_in); }
    unsafe { c2rust_library_call_55663a7a67ecf35fc3a7b337611872cb097efdd7a1f844324e0d3376881042db(lock_ptr); }

    let begin_bits = unsafe { c2rust_library_call_30a8d44d0a77e204bc81e42201a8db4a38cf2a5c6ed0677dc961bf0bb4065c85(threads) };
    let end_bits = unsafe { c2rust_library_call_6a3d74374123a7b23b51d54e0ce7c1e9d553c581b93c6013792e069656df8e65(threads) };
    let mut iterator = core::ptr::with_exposed_provenance_mut::<core::ffi::c_void>(begin_bits as usize);
    let mut end = core::ptr::with_exposed_provenance_mut::<core::ffi::c_void>(end_bits as usize);
    let iterator_address = core::ptr::addr_of_mut!(iterator).cast::<core::ffi::c_void>();
    let end_address = core::ptr::addr_of_mut!(end).cast::<core::ffi::c_void>();
    while unsafe { c2rust_library_call_796045d8f3b86671ef17599e90f2bf2505b818ecf6663316509c8635d39dbb66(iterator_address, end_address) } == 0 {
        let thread = unsafe { c2rust_library_call_7e05e05deddac3472377e96b06123dccd67e969e83bb817de352c81c8fe31676(iterator_address) };
        unsafe { c2rust_library_call_6082a29f95fd0b6d359b51efe16774dc345c0753142cc5756e469ef7baf223cf(thread); }
        unsafe { c2rust_library_call_eeb9cb64fda98ec2bc6b3ad737f43e50d5780597687fd39549da8ccc66d4a689(iterator_address); }
    }

    unsafe {
        c2rust_library_call_4e329bba7681da9d273f2bf4de410ccb1f48f81ba004530c51f5f1fe275cd813(queue);
        c2rust_library_call_c1954d599c59e4f0faaab7a1f7070104e82362050fc434162ad0d1cb874d6ae8(cv_ext);
        c2rust_library_call_c1954d599c59e4f0faaab7a1f7070104e82362050fc434162ad0d1cb874d6ae8(cv_in);
        c2rust_library_call_6badf988275b77422a7c447fa7d6b43d0a45a46e06fa1d10659af06759bf485b(mutex);
        c2rust_library_call_8f01498b81cee3321c485a35260a76307b0274968efe494df9cae9ffbbf3874b(threads);
    }
}
