//! Port of: engine/engine_util_errmem.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_defaultLogHandler (engine/engine_util_errmem.c:34)
/// Calls: mju_getLogConfigPtr, mju_isTopicEnabled, mju_legacy_text, mju_localTimeStr
#[allow(unused_variables, non_snake_case)]
pub fn mju_default_log_handler(msg: *const mjLogMessage) {
    // WARNING: signature changed — verify body
    // Previous params: (msg : * const mjLogMessage)
    // Previous return: ()
    todo ! ()
}

/// C: mju_alignedMalloc (engine/engine_util_errmem.c:44)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_malloc(size: usize, align: usize) -> *mut () {
    extern "C" { fn mju_alignedMalloc_impl(size: usize, align: usize) -> *mut (); }
    // SAFETY: delegates to C implementation
    unsafe { mju_alignedMalloc_impl(size, align) }
}

/// C: mju_alignedFree (engine/engine_util_errmem.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_free(ptr: *mut ()) {
    // WARNING: signature changed — verify body
    // Previous params: (ptr : * mut ())
    // Previous return: ()
    extern "C" { fn free (ptr : * mut ()) ; } unsafe { free (ptr) ; }
}

/// C: mju_initLogTopicsFromEnv (engine/engine_util_errmem.c:111)
#[allow(unused_variables, non_snake_case)]
pub fn mju_init_log_topics_from_env() {
    // SAFETY: uses static globals and C stdlib (getenv, strcasecmp), delegating to C implementation
    extern "C" { fn mju_initLogTopicsFromEnv(); }
    unsafe { mju_initLogTopicsFromEnv(); }
}

/// C: mju_getLogConfigPtr (engine/engine_util_errmem.c:145)
/// Calls: mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config_ptr() -> *const mjLogConfig {
    extern "C" {
        fn mju_getLogConfigPtr_impl() -> *const mjLogConfig;
    }
    // SAFETY: delegates to C implementation, returns pointer to static data
    unsafe { mju_getLogConfigPtr_impl() }
}

/// C: mju_localTimeStr (engine/engine_util_errmem.c:195)
#[allow(unused_variables, non_snake_case)]
pub fn mju_local_time_str(buf: *mut i8, buf_sz: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (buf : * mut i8, buf_sz : i32)
    // Previous return: ()
    extern "C" { fn mju_localTimeStr_impl(buf: *mut i8, buf_sz: i32); }
    // SAFETY: delegates to C implementation which calls localtime_r + strftime; caller guarantees buf is valid with buf_sz capacity
    unsafe { mju_localTimeStr_impl(buf, buf_sz) }
}

/// C: mju_fprint_message (engine/engine_util_errmem.c:214)
/// Calls: BaseName
#[allow(unused_variables, non_snake_case)]
pub fn mju_fprint_message(stream: *mut i32, timestr: *const i8, msg: *const mjLogMessage) {
    extern "C" { fn mju_fprint_message_impl(stream: *mut i32, timestr: *const i8, msg: *const mjLogMessage); }
    // SAFETY: delegates to C implementation
    unsafe { mju_fprint_message_impl(stream, timestr, msg) }
}

/// C: mju_legacy_text (engine/engine_util_errmem.c:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_legacy_text(msg: *const mjLogMessage, buf: *mut i8, bufsz: i32) -> *const i8 {
    extern "C" { fn mju_legacy_text_impl(msg: *const mjLogMessage, buf: *mut i8, bufsz: i32) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mju_legacy_text_impl(msg, buf, bufsz) }
}

/// C: mju_activeHandler (engine/engine_util_errmem.c:292)
#[allow(unused_variables, non_snake_case)]
pub fn mju_active_handler() -> mjfLogHandler {
    unsafe { extern "C" { fn mju_activeHandler () -> mjfLogHandler ; } mju_activeHandler () }
}

/// C: mju_malloc (engine/engine_util_errmem.h:43)
/// Calls: mju_alignedMalloc, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mju_malloc(size: usize) -> *mut () {
    extern "C" { fn mju_malloc_impl(size: usize) -> *mut (); }
    // SAFETY: delegates to C implementation
    unsafe { mju_malloc_impl(size) }
}

/// C: mju_free (engine/engine_util_errmem.h:46)
/// Calls: mju_alignedFree
#[allow(unused_variables, non_snake_case)]
pub fn mju_free(ptr: *mut ()) {
    // WARNING: signature changed — verify body
    // Previous params: (ptr : * mut ())
    // Previous return: ()
    if ptr . is_null () { return ; } unsafe { extern "C" { static mju_user_free : Option < unsafe extern "C" fn (* mut ()) > ; fn free (ptr : * mut ()) ; } if let Some (user_free) = mju_user_free { user_free (ptr) ; } else { free (ptr) ; } }
}

/// C: mju_setLogHandler (engine/engine_util_errmem.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    extern "C" { fn mju_setLogHandler_impl(handler: mjfLogHandler) -> mjfLogHandler; }
    // SAFETY: delegates to C implementation
    unsafe { mju_setLogHandler_impl(handler) }
}

/// C: mju_getLogConfig (engine/engine_util_errmem.h:60)
/// Calls: mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config() -> mjLogConfig {
    extern "C" { fn mju_getLogConfig_impl() -> mjLogConfig; }
    // SAFETY: delegates to C implementation
    unsafe { mju_getLogConfig_impl() }
}

/// C: mju_setLogConfig (engine/engine_util_errmem.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_config(config: mjLogConfig) {
    extern "C" { fn mju_setLogConfig_impl(config: mjLogConfig); }
    // SAFETY: delegates to C implementation
    unsafe { mju_setLogConfig_impl(config) }
}

/// C: mju_clearHandlers (engine/engine_util_errmem.h:64)
/// Calls: mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_clear_handlers() {
    extern "C" { fn mju_clearHandlers_impl(); }
    // SAFETY: delegates to C implementation
    unsafe { mju_clearHandlers_impl() }
}

/// C: mju_error (engine/engine_util_errmem.h:74)
/// Calls: mju_error_v
#[allow(unused_variables, non_snake_case)]
pub fn mju_error(msg: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (msg : * const i8)
    // Previous return: ()
    unsafe { let args : va_list = core :: mem :: transmute :: < [u8 ; 0] , va_list > ([]) ; mju_error_v (msg , args) ; }
}

/// C: mju_error_v (engine/engine_util_errmem.h:75)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_error_v(msg: *const i8, args: va_list) {
    extern "C" { fn mju_error_v_impl(msg: *const i8, args: va_list); }
    // SAFETY: delegates to C implementation
    unsafe { mju_error_v_impl(msg, args) }
}

/// C: mju_warning (engine/engine_util_errmem.h:78)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning(msg: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (msg : * const i8)
    // Previous return: ()
    extern "C" { fn mju_warning_impl (msg : * const i8) ; } unsafe { mju_warning_impl (msg) }
}

/// C: mju_info (engine/engine_util_errmem.h:81)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_info(topic: i32, msg: *const i8) {
    extern "C" { fn mju_info_impl(topic: i32, msg: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { mju_info_impl(topic, msg) }
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
    extern "C" { fn mju_writeLog_impl(r#type: *const i8, msg: *const i8); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_writeLog_impl(r#type, msg) }
}

/// C: _mjPRIVATE_setTlsLogHandler (engine/engine_util_errmem.h:93)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_set_tls_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    // WARNING: signature changed — verify body
    // Previous params: (handler : mjfLogHandler)
    // Previous return: mjfLogHandler
    unsafe { extern "C" { static mut _mjPRIVATE_tls_log_handler : mjfLogHandler ; } let prev = _mjPRIVATE_tls_log_handler ; _mjPRIVATE_tls_log_handler = handler ; prev }
}

/// C: _mjPRIVATE_getGlobalLogHandler (engine/engine_util_errmem.h:96)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_get_global_log_handler() -> mjfLogHandler {
    extern "C" { fn _mjPRIVATE_getGlobalLogHandler_impl() -> mjfLogHandler; }
    // SAFETY: delegates to C implementation
    unsafe { _mjPRIVATE_getGlobalLogHandler_impl() }
}

/// C: mju_isTopicEnabled (engine/engine_util_errmem.h:99)
/// Calls: mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_topic_enabled(topic: i32) -> mjtBool {
    extern "C" {
        fn mju_isTopicEnabled_impl(topic: i32) -> mjtBool;
    }
    // SAFETY: delegates to C implementation (uses global state)
    unsafe { mju_isTopicEnabled_impl(topic) }
}

/// C: BaseName (engine/engine_util_errmem.h:102)
#[allow(unused_variables, non_snake_case)]
pub fn base_name(path: *const i8) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (path : * const i8)
    // Previous return: * const i8
    extern "C" { fn strrchr (s : * const i8 , c : i32) -> * const i8 ; } unsafe { let slash : * const i8 = strrchr (path , b'/' as i32) ; let bslash : * const i8 = strrchr (path , b'\\' as i32) ; if ! slash . is_null () && ! bslash . is_null () { if slash > bslash { return slash . add (1) ; } else { return bslash . add (1) ; } } if ! slash . is_null () { return slash . add (1) ; } if ! bslash . is_null () { return bslash . add (1) ; } return path ; }
}

