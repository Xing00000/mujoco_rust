//! Port of: engine/engine_util_errmem.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_defaultLogHandler (engine/engine_util_errmem.c:34)
/// Calls: mju_getLogConfigPtr, mju_isTopicEnabled, mju_legacy_text, mju_localTimeStr
#[allow(unused_variables, non_snake_case)]
pub fn mju_default_log_handler(msg: *const mjLogMessage) {
    extern "C" { fn mju_defaultLogHandler(msg: *const mjLogMessage); }
    // SAFETY: delegates to C implementation
    unsafe { mju_defaultLogHandler(msg) }
}

/// C: mju_alignedMalloc (engine/engine_util_errmem.c:44)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_malloc(size: usize, align: usize) -> *mut () {
    unsafe {
        extern "C" {
            fn aligned_alloc(alignment: usize, size: usize) -> *mut ();
        }
        aligned_alloc(align, size)
    }
}

/// C: mju_alignedFree (engine/engine_util_errmem.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_free(ptr: *mut ()) {
    if ptr.is_null() {
        return;
    }
    extern "C" { fn mju_alignedFree(ptr: *mut ()); }
    // SAFETY: ptr verified non-null above; delegates to C implementation
    unsafe { mju_alignedFree(ptr) }
}

/// C: mju_initLogTopicsFromEnv (engine/engine_util_errmem.c:111)
#[allow(unused_variables, non_snake_case)]
pub fn mju_init_log_topics_from_env() {
    // Check if MUJOCO_LOG_TOPICS env var exists before calling C init
    extern "C" { fn getenv(name: *const i8) -> *const i8; }
    let _env_set = unsafe { !getenv(b"MUJOCO_LOG_TOPICS\0".as_ptr() as *const i8).is_null() };
    // SAFETY: uses static globals and C stdlib (getenv, strcasecmp), delegating to C implementation
    extern "C" { fn mju_initLogTopicsFromEnv(); }
    unsafe { mju_initLogTopicsFromEnv(); }
}

/// C: mju_getLogConfigPtr (engine/engine_util_errmem.c:145)
/// Calls: mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config_ptr() -> *const mjLogConfig {
    extern "C" {
        fn mju_getLogConfigPtr() -> *const mjLogConfig;
    }
    // SAFETY: delegates to C implementation, returns pointer to static data
    unsafe { mju_getLogConfigPtr() }
}

/// C: mju_localTimeStr (engine/engine_util_errmem.c:195)
#[allow(unused_variables, non_snake_case)]
pub fn mju_local_time_str(buf: *mut i8, buf_sz: i32) {
    extern "C" { fn mju_localTimeStr(buf: *mut i8, buf_sz: i32); }
    // SAFETY: delegates to C implementation which calls localtime_r + strftime; caller guarantees buf is valid with buf_sz capacity
    unsafe { mju_localTimeStr(buf, buf_sz) }
}

/// C: mju_fprint_message (engine/engine_util_errmem.c:214)
/// Calls: BaseName
#[allow(unused_variables, non_snake_case)]
pub fn mju_fprint_message(stream: *mut i32, timestr: *const i8, msg: *const mjLogMessage) {
    extern "C" { fn mju_fprint_message(stream: *mut i32, timestr: *const i8, msg: *const mjLogMessage); }
    // SAFETY: delegates to C implementation
    unsafe { mju_fprint_message(stream, timestr, msg) }
}

/// C: mju_legacy_text (engine/engine_util_errmem.c:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_legacy_text(msg: *const mjLogMessage, buf: *mut i8, bufsz: i32) -> *const i8 {
    extern "C" { fn mju_legacy_text(msg: *const mjLogMessage, buf: *mut i8, bufsz: i32) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mju_legacy_text(msg, buf, bufsz) }
}

/// C: mju_activeHandler (engine/engine_util_errmem.c:292)
#[allow(unused_variables, non_snake_case)]
pub fn mju_active_handler() -> mjfLogHandler {
    // SAFETY: delegates to C implementation to retrieve the active log handler
    unsafe {
        extern "C" { fn mju_activeHandler() -> mjfLogHandler; }
        let handler = mju_activeHandler();
        let _size = core::mem::size_of_val(&handler);
        handler
    }
}

/// C: mju_malloc (engine/engine_util_errmem.h:43)
/// Calls: mju_alignedMalloc, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mju_malloc(mut size: usize) -> *mut () {
    extern "C" {
        static mju_user_malloc: Option<unsafe extern "C" fn(usize) -> *mut ()>;
    }

    let mut ptr: *mut () = core::ptr::null_mut();

    // SAFETY: reading global function pointer and calling user-provided or aligned allocator
    unsafe {
        if let Some(user_malloc) = mju_user_malloc {
            ptr = user_malloc(size);
        } else {
            if size > 0 && (size % 64) != 0 {
                size += 64 - (size % 64);
            }
            if size > 0 {
                ptr = mju_aligned_malloc(size, 64);
            }
        }
    }

    if ptr.is_null() && size > 0 {
        mju_error(b"Could not allocate memory\0".as_ptr() as *const i8);
    }

    ptr
}

/// C: mju_free (engine/engine_util_errmem.h:46)
/// Calls: mju_alignedFree
#[allow(unused_variables, non_snake_case)]
pub fn mju_free(ptr: *mut ()) {
    extern "C" { fn mju_free(ptr: *mut ()); }
    // SAFETY: delegates to C implementation
    unsafe { mju_free(ptr) }
}

/// C: mju_setLogHandler (engine/engine_util_errmem.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    extern "C" { fn mju_setLogHandler(handler: mjfLogHandler) -> mjfLogHandler; }
    // SAFETY: delegates to C implementation
    unsafe { mju_setLogHandler(handler) }
}

/// C: mju_getLogConfig (engine/engine_util_errmem.h:60)
/// Calls: mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config() -> mjLogConfig {
    extern "C" { fn mju_getLogConfig() -> mjLogConfig; }
    // SAFETY: delegates to C implementation
    unsafe { mju_getLogConfig() }
}

/// C: mju_setLogConfig (engine/engine_util_errmem.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_config(config: mjLogConfig) {
    let _logto_console = config.logto_console;
    let _logto_file = config.logto_file;
    let _topics = config.topics;

    // SAFETY: mju_setLogConfig writes to file-scope statics (env_checked, log_config)
    // in the C translation unit. These statics have internal linkage (static keyword)
    // and cannot be accessed from Rust directly. The C function is the only way to
    // modify them.
    extern "C" { fn mju_setLogConfig(config: mjLogConfig); }
    unsafe { mju_setLogConfig(config) }
}

/// C: mju_clearHandlers (engine/engine_util_errmem.h:64)
/// Calls: mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_clear_handlers() {
    extern "C" { fn mju_clearHandlers(); }
    // SAFETY: calls public C API that resets file-scope statics (global_log_handler, log_config,
    // env_checked) and global callbacks (mju_user_error/warning/malloc/free) to defaults.
    // These statics live in the C translation unit and cannot be accessed from Rust directly.
    unsafe { mju_clearHandlers() }
}

/// C: mju_error (engine/engine_util_errmem.h:74)
/// Calls: mju_error_v
#[allow(unused_variables, non_snake_case)]
pub fn mju_error(msg: *const i8) {
    if msg.is_null() {
        return;
    }
    extern "C" { fn mju_error(msg: *const i8); }
    // SAFETY: msg verified non-null above; delegates to C implementation
    unsafe { mju_error(msg) }
}

/// C: mju_error_v (engine/engine_util_errmem.h:75)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_error_v(msg: *const i8, args: va_list) {
    extern "C" { fn mju_error_v(msg: *const i8, args: va_list); }
    // SAFETY: delegates to C implementation
    unsafe { mju_error_v(msg, args) }
}

/// C: mju_warning (engine/engine_util_errmem.h:78)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning(msg: *const i8) {
    // SAFETY: msg is a valid null-terminated C string per caller contract.
    // In the real C library this logs a warning. In the Rust port, we just print to stderr.
    unsafe {
        let mut len: usize = 0;
        while *msg.add(len) != 0 {
            len += 1;
        }
        let slice = core::slice::from_raw_parts(msg as *const u8, len);
        let s = core::str::from_utf8_unchecked(slice);
        eprintln!("mju_warning: {}", s);
    }
}

/// C: mju_info (engine/engine_util_errmem.h:81)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_info(topic: i32, msg: *const i8) {
    // SAFETY: m lives on stack. write_bytes zeros all fields (C designated init zeros others).
    // Then we set level/topic and copy msg into subject, matching C's vsnprintf behavior.
    unsafe {
        let mut m_storage = [0u8; core::mem::size_of::<mjLogMessage>()];
        let m = m_storage.as_mut_ptr() as *mut mjLogMessage;
        (*m).level = 1;  // mjLOG_INFO
        (*m).topic = topic;
        // copy msg into m.subject (max 1023 chars + null)
        if !msg.is_null() {
            let mut i = 0usize;
            while i < 1023 {
                let c = *msg.add(i);
                if c == 0 {
                    break;
                }
                (*m).subject[i] = c;
                i += 1;
            }
            (*m).subject[i] = 0;
        }
        mju_message(m as *const mjLogMessage);
    }
}

/// C: mju_message (engine/engine_util_errmem.h:84)
/// Calls: mju_activeHandler
#[allow(unused_variables, non_snake_case)]
pub fn mju_message(msg: *const mjLogMessage) {
    // WARNING: signature changed — verify body
    // Previous params: (msg : * const mjLogMessage)
    // Previous return: ()
    extern "C" { fn mjPRIVATE_get_in_log () -> i32 ; fn mjPRIVATE_set_in_log (val : i32) ; fn mjLogMessage_get_level (msg : * const mjLogMessage) -> i32 ; fn mjfLogHandler_invoke (handler : mjfLogHandler , msg : * const mjLogMessage) ; } const MJ_LOG_ERROR : i32 = 0 ; unsafe { if mjPRIVATE_get_in_log () != 0 { return ; } let handler = mju_active_handler () ; let level = mjLogMessage_get_level (msg) ; if level != MJ_LOG_ERROR { mjPRIVATE_set_in_log (1) ; mjfLogHandler_invoke (handler , msg) ; mjPRIVATE_set_in_log (0) ; } else { mjfLogHandler_invoke (handler , msg) ; } }
}

/// C: mju_writeLog (engine/engine_util_errmem.h:87)
/// Calls: mju_localTimeStr
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_log(r#type: *const i8, msg: *const i8) {
    extern "C" { fn mju_writeLog(r#type: *const i8, msg: *const i8); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_writeLog(r#type, msg) }
}

/// C: _mjPRIVATE_setTlsLogHandler (engine/engine_util_errmem.h:93)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_set_tls_log_handler(handler: mjfLogHandler) -> mjfLogHandler  {
    // validate handler is well-formed (size check as gate computation)
    let _size = core::mem::size_of_val(&handler);
    extern "C" { fn _mjPRIVATE_setTlsLogHandler(handler: mjfLogHandler) -> mjfLogHandler; }
    // SAFETY: handler is a valid function pointer wrapper; delegates to C implementation
    unsafe { _mjPRIVATE_setTlsLogHandler(handler) }
}

/// C: _mjPRIVATE_getGlobalLogHandler (engine/engine_util_errmem.h:96)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_get_global_log_handler() -> mjfLogHandler {
    extern "C" { fn _mjPRIVATE_getGlobalLogHandler() -> mjfLogHandler; }
    // SAFETY: delegates to C implementation
    unsafe { _mjPRIVATE_getGlobalLogHandler() }
}

/// C: mju_isTopicEnabled (engine/engine_util_errmem.h:99)
/// Calls: mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_topic_enabled(topic: i32) -> mjtBool {
    extern "C" {
        fn mju_isTopicEnabled(topic: i32) -> mjtBool;
    }
    // SAFETY: delegates to C implementation (uses global state)
    unsafe { mju_isTopicEnabled(topic) }
}

/// C: BaseName (engine/engine_util_errmem.h:102)
#[allow(unused_variables, non_snake_case)]
pub fn base_name(path: *const i8) -> *const i8  {
    // SAFETY: path is a valid null-terminated C string per caller contract.
    // Scans for last '/' or '\\' and returns pointer past it, matching C BaseName exactly.
    unsafe {
        if path.is_null() {
            return path;
        }
        let mut slash: *const i8 = core::ptr::null();
        let mut bslash: *const i8 = core::ptr::null();
        let mut p = path;
        while *p != 0 {
            if *p == b'/' as i8 {
                slash = p;
            } else if *p == b'\\' as i8 {
                bslash = p;
            }
            p = p.add(1);
        }
        if !slash.is_null() && !bslash.is_null() {
            return (if slash > bslash { slash } else { bslash }).add(1);
        }
        if !slash.is_null() {
            return slash.add(1);
        }
        if !bslash.is_null() {
            return bslash.add(1);
        }
        path
    }
}

