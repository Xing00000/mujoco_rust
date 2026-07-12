//! Port of: engine/engine_util_errmem.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_defaultLogHandler (engine/engine_util_errmem.c:34)
/// Calls: mju_getLogConfigPtr, mju_isTopicEnabled, mju_legacy_text, mju_localTimeStr
#[allow(unused_variables, non_snake_case)]
pub fn mju_default_log_handler(msg: *const mjLogMessage) {
    // NOTE: signature changed from previous IR version
    // Previous params: (msg : * const mjLogMessage)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_alignedMalloc (engine/engine_util_errmem.c:44)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_malloc(size: usize, align: usize) -> *mut () {
    // NOTE: signature changed from previous IR version
    // Previous params: (size : usize, align : usize)
    // Previous return: * mut ()
    todo!("re-translate: params renamed")
}

/// C: mju_alignedFree (engine/engine_util_errmem.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_free(ptr: *mut ()) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ptr : * mut ())
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_initLogTopicsFromEnv (engine/engine_util_errmem.c:111)
#[allow(unused_variables, non_snake_case)]
pub fn mju_init_log_topics_from_env() {
    todo ! ()
}

/// C: mju_getLogConfigPtr (engine/engine_util_errmem.c:145)
/// Calls: mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config_ptr() -> *const mjLogConfig {
    // NOTE: signature changed from previous IR version
    // Previous params: ()
    // Previous return: * const mjLogConfig
    todo!("re-translate: params renamed")
}

/// C: mju_localTimeStr (engine/engine_util_errmem.c:195)
#[allow(unused_variables, non_snake_case)]
pub fn mju_local_time_str(buf: *mut i8, buf_sz: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (buf : * mut i8, buf_sz : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_fprint_message (engine/engine_util_errmem.c:214)
/// Calls: BaseName
#[allow(unused_variables, non_snake_case)]
pub fn mju_fprint_message(stream: *mut i32, timestr: *const i8, msg: *const mjLogMessage) {
    // NOTE: signature changed from previous IR version
    // Previous params: (stream : * mut i32, timestr : * const i8, msg : * const mjLogMessage)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_legacy_text (engine/engine_util_errmem.c:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_legacy_text(msg: *const mjLogMessage, buf: *mut i8, bufsz: i32) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (msg : * const mjLogMessage, buf : * mut i8, bufsz : i32)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mju_activeHandler (engine/engine_util_errmem.c:292)
#[allow(unused_variables, non_snake_case)]
pub fn mju_active_handler() -> mjfLogHandler {
    todo ! ()
}

/// C: mju_malloc (engine/engine_util_errmem.h:43)
/// Calls: mju_alignedMalloc, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mju_malloc(size: usize) -> *mut () {
    // NOTE: signature changed from previous IR version
    // Previous params: (size : usize)
    // Previous return: * mut ()
    todo!("re-translate: params renamed")
}

/// C: mju_free (engine/engine_util_errmem.h:46)
/// Calls: mju_alignedFree
#[allow(unused_variables, non_snake_case)]
pub fn mju_free(ptr: *mut ()) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ptr : * mut ())
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_setLogHandler (engine/engine_util_errmem.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    // NOTE: signature changed from previous IR version
    // Previous params: (handler : mjfLogHandler)
    // Previous return: mjfLogHandler
    todo!("re-translate: params renamed")
}

/// C: mju_getLogConfig (engine/engine_util_errmem.h:60)
/// Calls: mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config() -> mjLogConfig {
    todo ! ()
}

/// C: mju_setLogConfig (engine/engine_util_errmem.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_config(config: mjLogConfig) {
    // NOTE: signature changed from previous IR version
    // Previous params: (config : mjLogConfig)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_clearHandlers (engine/engine_util_errmem.h:64)
/// Calls: mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_clear_handlers() {
    todo ! ()
}

/// C: mju_error (engine/engine_util_errmem.h:74)
/// Calls: mju_error_v
#[allow(unused_variables, non_snake_case)]
pub fn mju_error(msg: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (msg : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_error_v (engine/engine_util_errmem.h:75)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_error_v(msg: *const i8, args: va_list) {
    // NOTE: signature changed from previous IR version
    // Previous params: (msg : * const i8, args : va_list)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_warning (engine/engine_util_errmem.h:78)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning(msg: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (msg : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_info (engine/engine_util_errmem.h:81)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_info(topic: i32, msg: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (topic : i32, msg : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_message (engine/engine_util_errmem.h:84)
/// Calls: mju_activeHandler
#[allow(unused_variables, non_snake_case)]
pub fn mju_message(msg: *const mjLogMessage) {
    // NOTE: signature changed from previous IR version
    // Previous params: (msg : * const mjLogMessage)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_writeLog (engine/engine_util_errmem.h:87)
/// Calls: mju_localTimeStr
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_log(r#type: *const i8, msg: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (r#type : * const i8, msg : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: _mjPRIVATE_setTlsLogHandler (engine/engine_util_errmem.h:93)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_set_tls_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    // NOTE: signature changed from previous IR version
    // Previous params: (handler : mjfLogHandler)
    // Previous return: mjfLogHandler
    todo!("re-translate: params renamed")
}

/// C: _mjPRIVATE_getGlobalLogHandler (engine/engine_util_errmem.h:96)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_get_global_log_handler() -> mjfLogHandler {
    todo ! ()
}

/// C: mju_isTopicEnabled (engine/engine_util_errmem.h:99)
/// Calls: mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_topic_enabled(topic: i32) -> mjtBool {
    // NOTE: signature changed from previous IR version
    // Previous params: (topic : i32)
    // Previous return: mjtBool
    todo!("re-translate: params renamed")
}

/// C: BaseName (engine/engine_util_errmem.h:102)
#[allow(unused_variables, non_snake_case)]
pub fn base_name(path: *const i8) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (path : * const i8)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

