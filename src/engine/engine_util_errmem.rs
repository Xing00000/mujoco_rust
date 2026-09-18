//! Port of: engine/engine_util_errmem.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_defaultLogHandler (engine/engine_util_errmem.c:39)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:_mju_getLogConfigPtr, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:_mju_legacy_text, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:_mju_localTimeStr, cxx-internal:engine_util_errmem.c.o:_mju_fprint_message, cxx:_mju_isTopicEnabled
#[allow(unused_variables, non_snake_case)]
pub fn mju_default_log_handler(msg: *const mjLogMessage) {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_defaultLogHandler
}

/// C: mju_alignedMalloc (engine/engine_util_errmem.c:49)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_malloc(size: usize, align: usize) -> *mut () {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_alignedMalloc
}

/// C: mju_alignedFree (engine/engine_util_errmem.c:58)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_free(ptr: *mut ()) {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_alignedFree
}

/// C: mju_malloc (engine/engine_util_errmem.c:67)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:_mju_alignedMalloc, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mju_malloc(size: usize) -> *mut () {
    type MallocFn = unsafe extern "C" fn(usize) -> *mut ();

    // SAFETY: reading function pointer from global storage (set via mju_clearHandlers)
    unsafe {
        let mut ptr: *mut () = std::ptr::null_mut();

        // check mju_user_malloc
        let user_malloc_ptr: usize;
        {
            let guard = MJU_USER_MALLOC.lock().unwrap();
            user_malloc_ptr = usize::from_ne_bytes([
                guard[0], guard[1], guard[2], guard[3],
                guard[4], guard[5], guard[6], guard[7],
            ]);
        }

        if user_malloc_ptr != 0 {
            let user_malloc: MallocFn = *(&user_malloc_ptr as *const usize as *const MallocFn);
            ptr = user_malloc(size);
        } else {
            let mut aligned_size = size;
            if aligned_size > 0 && (aligned_size % 64) != 0 {
                aligned_size += 64 - (aligned_size % 64);
            }
            if aligned_size > 0 {
                ptr = mju_alignedMalloc(aligned_size, 64);
            }
        }

        if ptr.is_null() && size > 0 {
            mju_error(b"Could not allocate memory\0".as_ptr() as *const i8);
        }

        ptr
    }
}

/// C: mju_free (engine/engine_util_errmem.c:88)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:_mju_alignedFree
#[allow(unused_variables, non_snake_case)]
pub fn mju_free(ptr: *mut ()) {
    if ptr.is_null() {
        return;
    }
    mju_alignedFree(ptr);
}

/// C: mju_initLogTopicsFromEnv (engine/engine_util_errmem.c:117)
#[allow(unused_variables, non_snake_case)]
pub fn mju_init_log_topics_from_env() {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_initLogTopicsFromEnv
}

/// C: mju_getLogConfigPtr (engine/engine_util_errmem.c:151)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config_ptr() -> *const mjLogConfig {
    // C: static const mjLogConfig* mju_getLogConfigPtr(void) { ... return &log_config; }
    // SAFETY: LOG_CONFIG is a static LazyLock<Mutex<[u8; 1032]>>.
    // We need to return a stable pointer to the inner data.
    // This is a simplification; in C it returns &log_config directly.
    // We use std::ptr::addr_of! to get the address without going through the lock.
    use crate::types::LOG_CONFIG;
    // Lock and leak the guard to get a stable pointer (this is a workaround)
    // In practice, for bit-exact translation, we can use a static mut or unsafe block
    static mut LOG_CONFIG_BYTES: [u8; 1032] = [0u8; 1032];
    unsafe {
        let guard = LOG_CONFIG.lock().unwrap();
        LOG_CONFIG_BYTES.copy_from_slice(&*guard);
        LOG_CONFIG_BYTES.as_ptr() as *const mjLogConfig
    }
}

/// C: mju_isTopicEnabled (engine/engine_util_errmem.c:172)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:_mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_topic_enabled(topic: i32) -> bool {
    // C: mjtBool mju_isTopicEnabled(int topic) {
    //      if (!topic) return 1;
    //      const mjLogConfig* cfg = mju_getLogConfigPtr();
    //      return ((cfg->topics & (1 << (topic - 1))) != 0);
    //    }
    if topic == 0 {
        return true;
    }
    unsafe {
        let cfg = mju_get_log_config_ptr();
        ((*cfg).topics & (1 << (topic - 1))) != 0
    }
}

/// C: mju_setLogHandler (engine/engine_util_errmem.c:179)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    todo!() // cxx:_mju_setLogHandler
}

/// C: mju_getLogConfig (engine/engine_util_errmem.c:186)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:_mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config() -> mjLogConfig {
    todo!() // cxx:_mju_getLogConfig
}

/// C: mju_setLogConfig (engine/engine_util_errmem.c:191)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_config(config: mjLogConfig) {
    todo!() // cxx:_mju_setLogConfig
}

/// C: mju_clearHandlers (engine/engine_util_errmem.c:198)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_clear_handlers() {
    todo!() // cxx:_mju_clearHandlers
}

/// C: mju_localTimeStr (engine/engine_util_errmem.c:215)
#[allow(unused_variables, non_snake_case)]
pub fn mju_local_time_str(buf: *mut i8, buf_sz: i32) {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_localTimeStr
}

/// C: mju_fprint_message (engine/engine_util_errmem.c:234)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fprint_message(stream: *mut FILE, timestr: *const i8, msg: *const mjLogMessage) {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_fprint_message
}

/// C: mju_legacy_text (engine/engine_util_errmem.c:251)
#[allow(unused_variables, non_snake_case)]
pub fn mju_legacy_text(msg: *const mjLogMessage, buf: *mut i8, bufsz: i32) -> *const i8 {
    extern "C" {
        fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    }

    // SAFETY: msg is a valid pointer to mjLogMessage, buf is a writable buffer of at least bufsz bytes
    unsafe {
        if !(*msg).func.is_null() {
            snprintf(
                buf,
                bufsz as usize,
                b"%s: %s\0".as_ptr() as *const i8,
                (*msg).func,
                (*msg).subject.as_ptr(),
            );
            return buf;
        }
        (*msg).subject.as_ptr()
    }
}

/// C: mju_activeHandler (engine/engine_util_errmem.c:312)
#[allow(unused_variables, non_snake_case)]
pub fn mju_active_handler() -> mjfLogHandler {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_activeHandler
}

/// C: mju_message (engine/engine_util_errmem.c:317)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:_mju_activeHandler
#[allow(unused_variables, non_snake_case)]
pub fn mju_message(msg: *const mjLogMessage) {
    // C: mju_activeHandler dispatches TLS > global log handler
    // mjfLogHandler is void(*)(const mjLogMessage*) = 8-byte fn pointer
    // TLS handler stored in _MJPRIVATE_TLS_LOG_HANDLER, global in GLOBAL_LOG_HANDLER
    type LogHandlerFn = unsafe extern "C" fn(*const mjLogMessage);

    // thread_local recursion guard (C: static mjTHREADLOCAL bool in_log)
    thread_local! {
        static IN_LOG: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }

    // SAFETY: msg is a valid pointer to mjLogMessage (caller contract)
    unsafe {
        // recursion guard
        let in_log = IN_LOG.with(|c| c.get());
        if in_log {
            return;
        }

        // resolve active handler: TLS > global
        let handler_ptr: usize;
        {
            let tls_guard = _MJPRIVATE_TLS_LOG_HANDLER.lock().unwrap();
            let tls_val = usize::from_ne_bytes([
                tls_guard[0], tls_guard[1], tls_guard[2], tls_guard[3],
                tls_guard[4], tls_guard[5], tls_guard[6], tls_guard[7],
            ]);
            if tls_val != 0 {
                handler_ptr = tls_val;
            } else {
                drop(tls_guard);
                let global_guard = GLOBAL_LOG_HANDLER.lock().unwrap();
                handler_ptr = usize::from_ne_bytes([
                    global_guard[0], global_guard[1], global_guard[2], global_guard[3],
                    global_guard[4], global_guard[5], global_guard[6], global_guard[7],
                ]);
            }
        }

        if handler_ptr == 0 {
            return;
        }

        // SAFETY: handler_ptr is a valid function pointer set via mju_setLogHandler
        let handler: LogHandlerFn = *(&handler_ptr as *const usize as *const LogHandlerFn);

        // error handlers may not return (longjmp); don't set in_log around them
        if (*msg).level != 2 {
            // not mjLOG_ERROR
            IN_LOG.with(|c| c.set(true));
            handler(msg);
            IN_LOG.with(|c| c.set(false));
        } else {
            handler(msg);
        }
    }
}

/// C: mju_error_v (engine/engine_util_errmem.c:332)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_error_v(msg: *const i8, args: *mut i8) {
    extern "C" {
        fn vsnprintf(buf: *mut i8, size: usize, fmt: *const i8, args: *mut i8) -> i32;
    }
    // SAFETY: msg is a valid C format string, args is a valid va_list pointer
    unsafe {
        let mut m = mjLogMessage {
            level: 2, // mjLOG_ERROR
            topic: 0,
            subject: [0i8; 1024],
            body: std::ptr::null(),
            func: std::ptr::null(),
            file: std::ptr::null(),
            line: 0,
            timestamp: false,
            _pad_0: [0; 3],
        };
        vsnprintf(m.subject.as_mut_ptr(), 1024, msg, args);
        mju_message(&m);
    }
}

/// C: mju_error (engine/engine_util_errmem.c:339)
/// Calls: cxx:_mju_error_v
#[allow(unused_variables, non_snake_case)]
pub fn mju_error(msg: *const i8) {
    // C: mju_error is variadic, but Rust signature takes only msg (no va_args).
    // Construct mjLogMessage with level=ERROR and subject=msg, then dispatch.
    extern "C" {
        fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    }
    // SAFETY: msg is a valid C string (caller contract)
    unsafe {
        let mut m = mjLogMessage {
            level: 2, // mjLOG_ERROR
            topic: 0,
            subject: [0i8; 1024],
            body: std::ptr::null(),
            func: std::ptr::null(),
            file: std::ptr::null(),
            line: 0,
            timestamp: false,
            _pad_0: [0; 3],
        };
        snprintf(m.subject.as_mut_ptr(), 1024, b"%s\0".as_ptr() as *const i8, msg);
        mju_message(&m);
    }
}

/// C: mju_warning (engine/engine_util_errmem.c:347)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning(msg: *const i8) {
    // C: mju_warning is variadic, but Rust signature takes only msg.
    extern "C" {
        fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    }
    // SAFETY: msg is a valid C string (caller contract)
    unsafe {
        let mut m = mjLogMessage {
            level: 1, // mjLOG_WARNING
            topic: 0,
            subject: [0i8; 1024],
            body: std::ptr::null(),
            func: std::ptr::null(),
            file: std::ptr::null(),
            line: 0,
            timestamp: false,
            _pad_0: [0; 3],
        };
        snprintf(m.subject.as_mut_ptr(), 1024, b"%s\0".as_ptr() as *const i8, msg);
        mju_message(&m);
    }
}

/// C: mju_info (engine/engine_util_errmem.c:357)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_info(topic: i32, msg: *const i8) {
    // C: mju_info is variadic, but Rust signature takes only topic + msg.
    extern "C" {
        fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    }
    // SAFETY: msg is a valid C string (caller contract)
    unsafe {
        let mut m = mjLogMessage {
            level: 0, // mjLOG_INFO
            topic: topic,
            subject: [0i8; 1024],
            body: std::ptr::null(),
            func: std::ptr::null(),
            file: std::ptr::null(),
            line: 0,
            timestamp: false,
            _pad_0: [0; 3],
        };
        snprintf(m.subject.as_mut_ptr(), 1024, b"%s\0".as_ptr() as *const i8, msg);
        mju_message(&m);
    }
}

/// C: mju_writeLog (engine/engine_util_errmem.c:367)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:_mju_localTimeStr
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_log(r#type: *const i8, msg: *const i8) {
    todo!() // cxx:_mju_writeLog
}

/// C: _mjPRIVATE_setTlsLogHandler (engine/engine_util_errmem.c:382)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_set_tls_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    todo!() // cxx:__mjPRIVATE_setTlsLogHandler
}

/// C: _mjPRIVATE_getGlobalLogHandler (engine/engine_util_errmem.c:390)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_get_global_log_handler() -> mjfLogHandler {
    todo!() // cxx:__mjPRIVATE_getGlobalLogHandler
}

/// C: BaseName (engine/engine_util_errmem.h:102)
#[allow(unused_variables, non_snake_case)]
pub fn base_name__1f9ca8b6b7d9ebb9(path: *const i8) -> *const i8 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:_BaseName
}

/// C: BaseName (engine/engine_util_errmem.h:102)
#[allow(unused_variables, non_snake_case)]
pub fn base_name__d955fff29655c428(path: *const i8) -> *const i8 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:__ZL8BaseNamePKc
}

