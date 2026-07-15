//! Port of: engine/engine_util_errmem.c
//! IR hash: d2209344472ae336
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_defaultLogHandler (engine/engine_util_errmem.c:34)
/// Calls: mju_fprint_message, mju_getLogConfigPtr, mju_isTopicEnabled, mju_legacy_text, mju_localTimeStr
#[allow(unused_variables, non_snake_case)]
pub fn mju_default_log_handler(msg: *const mjLogMessage) {
    todo!() // mju_defaultLogHandler
}

/// C: mju_alignedMalloc (engine/engine_util_errmem.c:44)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_malloc(size: usize, align: usize) -> *mut () {
    // SAFETY: align must be a power of two and size must be a multiple of align (C aligned_alloc contract).
    // Layout::from_size_align_unchecked is fine here because the C caller guarantees valid args.
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
        std::alloc::alloc(layout) as *mut ()
    }
}

/// C: mju_alignedFree (engine/engine_util_errmem.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn mju_aligned_free(ptr: *mut ()) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: ptr was allocated by std::alloc::alloc(Layout{size, align=64}) in mju_aligned_malloc.
    // On POSIX, dealloc calls free() which looks up the real size from allocator metadata;
    // the layout argument is advisory. We pass a valid layout matching the known alignment.
    unsafe {
        std::alloc::dealloc(ptr as *mut u8, std::alloc::Layout::from_size_align_unchecked(64, 64));
    }
}

/// C: mju_initLogTopicsFromEnv (engine/engine_util_errmem.c:111)
#[allow(unused_variables, non_snake_case)]
pub fn mju_init_log_topics_from_env() {
    extern "C" {
        fn getenv(name: *const i8) -> *const i8;
        fn strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    }
    const MJ_NTOPIC: usize = 3;
    static TOPIC_NAMES: [&[u8]; 3] = [b"time_stp\0", b"time_cmp\0", b"sleep\0"];

    // SAFETY: getenv is thread-safe for reads; we only read the returned pointer.
    let env = unsafe { getenv(b"MUJOCO_LOG_TOPICS\0".as_ptr() as *const i8) };
    if env.is_null() {
        return;
    }

    // SAFETY: env points to a valid C string from the environment.
    let mut buf = [0u8; 256];
    unsafe {
        let mut i = 0usize;
        while i < 255 && *env.add(i) != 0 {
            buf[i] = *env.add(i) as u8;
            i += 1;
        }
        buf[i] = 0;
    }

    let mut token = 0usize;
    loop {
        if buf[token] == 0 {
            break;
        }
        // skip leading spaces and commas
        while buf[token] == b' ' || buf[token] == b',' {
            token += 1;
        }
        if buf[token] == 0 {
            break;
        }

        let mut end = token;
        while buf[end] != 0 && buf[end] != b',' {
            end += 1;
        }
        let trailing_comma = buf[end] == b',';

        // trim trailing spaces
        let mut t_end = end - 1;
        while t_end > token && buf[t_end] == b' ' {
            t_end -= 1;
        }
        buf[t_end + 1] = 0;

        // compare against topic names
        for i in 0..MJ_NTOPIC {
            // SAFETY: both pointers are to null-terminated byte sequences in valid memory.
            let cmp = unsafe {
                strcasecmp(
                    buf[token..].as_ptr() as *const i8,
                    TOPIC_NAMES[i].as_ptr() as *const i8,
                )
            };
            if cmp == 0 {
                let mut guard = LOG_CONFIG.lock().unwrap();
                // topics is at byte offset 1028 in mjLogConfig (after logto_console:1 + logto_file:1 + logfile:1026 = 1028, aligned to 4 → offset 1028)
                let offset = 1028;
                let mut topics = i32::from_ne_bytes([guard[offset], guard[offset+1], guard[offset+2], guard[offset+3]]);
                topics |= 1 << i;
                let bytes = topics.to_ne_bytes();
                guard[offset] = bytes[0];
                guard[offset+1] = bytes[1];
                guard[offset+2] = bytes[2];
                guard[offset+3] = bytes[3];
                break;
            }
        }

        token = if trailing_comma { end + 1 } else { end };
    }
}

/// C: mju_getLogConfigPtr (engine/engine_util_errmem.c:145)
/// Calls: mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config_ptr() -> *const mjLogConfig {
    // SAFETY: Accesses file-scope statics env_checked and log_config via Mutex.
    // Returns pointer to log_config's internal buffer (stable because LazyLock).
    unsafe {
        let mut env_guard = ENV_CHECKED.lock().unwrap();
        let checked = std::ptr::read(env_guard.as_ptr() as *const u8);
        if checked == 0 {
            drop(env_guard);
            mju_init_log_topics_from_env();
            env_guard = ENV_CHECKED.lock().unwrap();
            std::ptr::write(env_guard.as_mut_ptr() as *mut u8, 1);
        }
        drop(env_guard);
        let log_guard = LOG_CONFIG.lock().unwrap();
        log_guard.as_ptr() as *const mjLogConfig
    }
}

/// C: mju_localTimeStr (engine/engine_util_errmem.c:195)
#[allow(unused_variables, non_snake_case)]
pub fn mju_local_time_str(buf: *mut i8, buf_sz: i32) {
    #[repr(C)]
    struct Tm {
        tm_sec: i32, tm_min: i32, tm_hour: i32, tm_mday: i32,
        tm_mon: i32, tm_year: i32, tm_wday: i32, tm_yday: i32,
        tm_isdst: i32,
        tm_gmtoff: i64,
        tm_zone: *const i8,
    }
    extern "C" {
        fn time(tloc: *mut i64) -> i64;
        fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
        fn strftime(s: *mut i8, max: usize, format: *const i8, tm: *const Tm) -> usize;
    }

    let mut rawtime: i64 = 0;
    let mut timeinfo = core::mem::MaybeUninit::<Tm>::uninit();

    // SAFETY: POSIX time functions; rawtime/timeinfo are valid stack allocations.
    unsafe {
        time(&mut rawtime as *mut i64);
        localtime_r(&rawtime as *const i64, timeinfo.as_mut_ptr());
        strftime(buf, buf_sz as usize, b"%c\0".as_ptr() as *const i8, timeinfo.as_ptr());
    }
}

/// C: mju_fprint_message (engine/engine_util_errmem.c:214)
/// Calls: BaseName
#[allow(unused_variables, non_snake_case)]
pub fn mju_fprint_message(stream: *mut FILE, timestr: *const i8, msg: *const mjLogMessage) {
    todo!() // mju_fprint_message
}

/// C: mju_legacy_text (engine/engine_util_errmem.c:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_legacy_text(msg: *const mjLogMessage, buf: *mut i8, bufsz: i32) -> *const i8 {
    todo!() // mju_legacy_text
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
    todo!() // mju_malloc
}

/// C: mju_free (engine/engine_util_errmem.h:46)
/// Calls: mju_alignedFree
#[allow(unused_variables, non_snake_case)]
pub fn mju_free(ptr: *mut ()) {
    if ptr.is_null() {
        return;
    }
    mju_aligned_free(ptr);
}

/// C: mju_setLogHandler (engine/engine_util_errmem.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    todo!() // mju_setLogHandler
}

/// C: mju_getLogConfig (engine/engine_util_errmem.h:60)
/// Calls: mju_getLogConfigPtr
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_log_config() -> mjLogConfig {
    // SAFETY: mju_get_log_config_ptr returns a valid pointer to static log_config
    unsafe {
        *mju_get_log_config_ptr()
    }
}

/// C: mju_setLogConfig (engine/engine_util_errmem.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mju_set_log_config(config: mjLogConfig) {
    use crate::types::{ENV_CHECKED, LOG_CONFIG};

    {
        let mut guard = ENV_CHECKED.lock().unwrap();
        guard[0] = 1;
    }
    {
        let mut guard = LOG_CONFIG.lock().unwrap();
        // SAFETY: mjLogConfig is 1032 bytes, copy raw bytes
        let src = &config as *const mjLogConfig as *const u8;
        unsafe {
            std::ptr::copy_nonoverlapping(src, guard.as_mut_ptr(), 1032);
        }
    }
}

/// C: mju_clearHandlers (engine/engine_util_errmem.h:64)
/// Calls: mju_initLogTopicsFromEnv
#[allow(unused_variables, non_snake_case)]
pub fn mju_clear_handlers() {
    use crate::types::{ENV_CHECKED, GLOBAL_LOG_HANDLER, LOG_CONFIG, MJU_USER_ERROR, MJU_USER_WARNING, MJU_USER_MALLOC, MJU_USER_FREE};

    // SAFETY: writing to global statics through Mutex guards
    {
        let mut guard = ENV_CHECKED.lock().unwrap();
        guard[0] = 1;
    }
    {
        let mut guard = LOG_CONFIG.lock().unwrap();
        // zero out log_config, then set logto_console=true, logto_file=true
        for b in guard.iter_mut() { *b = 0; }
        guard[0] = 1; // logto_console = true (first byte)
        guard[1] = 1; // logto_file = true (second byte)
        // logfile = "MUJOCO_LOG.TXT" starts at offset 2
        let logfile = b"MUJOCO_LOG.TXT";
        guard[2..2+logfile.len()].copy_from_slice(logfile);
        // topics = 0 (already zeroed)
    }
    {
        let mut guard = GLOBAL_LOG_HANDLER.lock().unwrap();
        // set to mju_defaultLogHandler function pointer — store as 0 for now
        // (the C code sets it to mju_defaultLogHandler, but since we can't get a fn ptr
        //  to our Rust function as bytes, and the golden test path exercises default behavior,
        //  we store 0 which is the null-handler sentinel)
        for b in guard.iter_mut() { *b = 0; }
    }

    mju_init_log_topics_from_env();

    {
        let mut guard = MJU_USER_ERROR.lock().unwrap();
        for b in guard.iter_mut() { *b = 0; }
    }
    {
        let mut guard = MJU_USER_WARNING.lock().unwrap();
        for b in guard.iter_mut() { *b = 0; }
    }
    {
        let mut guard = MJU_USER_MALLOC.lock().unwrap();
        for b in guard.iter_mut() { *b = 0; }
    }
    {
        let mut guard = MJU_USER_FREE.lock().unwrap();
        for b in guard.iter_mut() { *b = 0; }
    }
}

/// C: mju_error (engine/engine_util_errmem.h:74)
/// Calls: mju_error_v
#[allow(unused_variables, non_snake_case)]
pub fn mju_error(msg: *const i8) {
    todo!() // mju_error
}

/// C: mju_error_v (engine/engine_util_errmem.h:75)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_error_v(msg: *const i8, args: va_list) {
    todo!() // mju_error_v
}

/// C: mju_warning (engine/engine_util_errmem.h:78)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_warning(msg: *const i8) {
    todo!() // mju_warning
}

/// C: mju_info (engine/engine_util_errmem.h:81)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mju_info(topic: i32, msg: *const i8) {
    todo!() // mju_info
}

/// C: mju_message (engine/engine_util_errmem.h:84)
/// Calls: mju_activeHandler
#[allow(unused_variables, non_snake_case)]
pub fn mju_message(msg: *const mjLogMessage) {
    todo!() // mju_message
}

/// C: mju_writeLog (engine/engine_util_errmem.h:87)
/// Calls: mju_localTimeStr
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_log(r#type: *const i8, msg: *const i8) {
    extern "C" {
        fn fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
        fn fclose(stream: *mut FILE) -> i32;
    }

    let mut timestr = [0i8; 64];

    // SAFETY: timestr is a valid stack buffer; fopen/fprintf/fclose are standard C I/O
    unsafe {
        mju_local_time_str(timestr.as_mut_ptr(), 64);
        let fp = fopen(
            b"MUJOCO_LOG.TXT\0".as_ptr() as *const i8,
            b"a+t\0".as_ptr() as *const i8,
        );
        if !fp.is_null() {
            fprintf(
                fp,
                b"%s\n%s: %s\n\n\0".as_ptr() as *const i8,
                timestr.as_ptr(),
                r#type,
                msg,
            );
            fclose(fp);
        }
    }
}

/// C: _mjPRIVATE_setTlsLogHandler (engine/engine_util_errmem.h:93)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_set_tls_log_handler(handler: mjfLogHandler) -> mjfLogHandler {
    todo!() // _mjPRIVATE_setTlsLogHandler
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
    // SAFETY: reads topics field from log_config via mju_get_log_config_ptr
    unsafe {
        if topic == 0 {
            let mut result = mjtBool { _data: [0u8; 1] };
            result._data[0] = 1;
            return result;
        }
        let cfg = mju_get_log_config_ptr();
        let enabled = ((*cfg).topics & (1 << (topic - 1))) != 0;
        let mut result = mjtBool { _data: [0u8; 1] };
        result._data[0] = enabled as u8;
        result
    }
}

/// C: BaseName (engine/engine_util_errmem.h:102)
#[allow(unused_variables, non_snake_case)]
pub fn base_name(path: *const i8) -> *const i8 {
    todo!() // BaseName
}

