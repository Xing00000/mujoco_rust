//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_defaultLogHandler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:34)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_fprint_message, cxx-internal:engine_util_errmem.c.o:_mju_getLogConfigPtr, cxx-internal:engine_util_errmem.c.o:_mju_legacy_text, cxx-internal:engine_util_errmem.c.o:_mju_localTimeStr, cxx:_mju_isTopicEnabled
#[allow(unused_variables, non_snake_case)]
pub fn mju_default_log_handler(msg: *const mjLogMessage) {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_defaultLogHandler
}

/// C: mju_alignedMalloc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:44)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_malloc(size: usize, align: usize) -> *mut () {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_alignedMalloc
}

/// C: mju_alignedFree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_free(ptr: *mut ()) {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_alignedFree
}

/// C: mju_initLogTopicsFromEnv (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:111)
#[allow(unused_variables, non_snake_case)]
pub fn mju_init_log_topics_from_env() {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_initLogTopicsFromEnv
}

/// C: mju_getLogConfigPtr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:145)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config_ptr() -> *const mjLogConfig {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_getLogConfigPtr
}

/// C: mju_localTimeStr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:195)
#[allow(unused_variables, non_snake_case)]
pub fn mju_local_time_str(buf: *mut i8, buf_sz: i32) {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_localTimeStr
}

/// C: mju_fprint_message (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:214)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fprint_message(stream: *mut FILE, timestr: *const i8, msg: *const mjLogMessage) {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_fprint_message
}

/// C: mju_legacy_text (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_legacy_text(msg: *const mjLogMessage, buf: *mut i8, bufsz: i32) -> *const i8 {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_legacy_text
}

/// C: mju_activeHandler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.c:292)
#[allow(unused_variables, non_snake_case)]
pub fn mju_active_handler() -> mjfLogHandler {
    todo!() // cxx-internal:engine_util_errmem.c.o:_mju_activeHandler
}

/// C: mju_malloc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:43)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_alignedMalloc, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mju_malloc(size: usize) -> *mut () {
    todo!() // cxx:_mju_malloc
}

/// C: mju_free (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:46)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_alignedFree
#[allow(unused_variables, non_snake_case)]
pub fn mju_free(ptr: *mut ()) {
    todo!() // cxx:_mju_free
}

/// C: mju_setLogHandler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    todo!() // cxx:_mju_setLogHandler
}

/// C: mju_getLogConfig (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:60)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config() -> mjLogConfig {
    todo!() // cxx:_mju_getLogConfig
}

/// C: mju_setLogConfig (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_config(config: mjLogConfig) {
    todo!() // cxx:_mju_setLogConfig
}

/// C: mju_clearHandlers (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:64)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_clear_handlers() {
    todo!() // cxx:_mju_clearHandlers
}

/// C: mju_error (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:74)
/// Calls: cxx:_mju_error_v
#[allow(unused_variables, non_snake_case)]
pub fn mju_error(msg: *const i8) {
    todo!() // cxx:_mju_error
}

/// C: mju_error_v (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:75)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_error_v(msg: *const i8, args: *mut i8) {
    todo!() // cxx:_mju_error_v
}

/// C: mju_warning (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:78)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning(msg: *const i8) {
    todo!() // cxx:_mju_warning
}

/// C: mju_info (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:81)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_info(topic: i32, msg: *const i8) {
    todo!() // cxx:_mju_info
}

/// C: mju_message (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:84)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_activeHandler
#[allow(unused_variables, non_snake_case)]
pub fn mju_message(msg: *const mjLogMessage) {
    todo!() // cxx:_mju_message
}

/// C: mju_writeLog (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:87)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_localTimeStr
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_log(r#type: *const i8, msg: *const i8) {
    todo!() // cxx:_mju_writeLog
}

/// C: _mjPRIVATE_setTlsLogHandler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:93)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_set_tls_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    todo!() // cxx:__mjPRIVATE_setTlsLogHandler
}

/// C: _mjPRIVATE_getGlobalLogHandler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:96)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_get_global_log_handler() -> mjfLogHandler {
    todo!() // cxx:__mjPRIVATE_getGlobalLogHandler
}

/// C: mju_isTopicEnabled (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:99)
/// Calls: cxx-internal:engine_util_errmem.c.o:_mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_topic_enabled(topic: i32) -> bool {
    todo!() // cxx:_mju_isTopicEnabled
}

/// C: BaseName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:102)
#[allow(unused_variables, non_snake_case)]
pub fn base_name__d955fff29655c428(path: *const i8) -> *const i8 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:__ZL8BaseNamePKc
}

/// C: BaseName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_errmem.h:102)
#[allow(unused_variables, non_snake_case)]
pub fn base_name__b87e40211ebca1be(path: *const i8) -> *const i8 {
    todo!() // cxx-internal:engine_util_errmem.c.o:_BaseName
}

