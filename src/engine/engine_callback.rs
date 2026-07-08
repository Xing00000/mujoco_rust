//! Port of: engine/engine_callback.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_resetCallbacks (engine/engine_callback.h:37)
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_callbacks() {
    // SAFETY: Sets global C callback function pointers to NULL.
    // These are extern globals provided by the linked C library (libmujoco).
    // Setting them to None/null matches the C implementation which sets them to 0.
    unsafe {
        extern "C" {
            static mut mjcb_passive: Option<unsafe extern "C" fn(*const mjModel, *mut mjData)>;
            static mut mjcb_control: Option<unsafe extern "C" fn(*const mjModel, *mut mjData)>;
            static mut mjcb_contactfilter: Option<unsafe extern "C" fn(*const mjModel, *mut mjData, i32, i32) -> i32>;
            static mut mjcb_sensor: Option<unsafe extern "C" fn(*const mjModel, *mut mjData, i32)>;
            static mut mjcb_time: Option<unsafe extern "C" fn() -> f64>;
            static mut mjcb_act_bias: Option<unsafe extern "C" fn(*const mjModel, *const mjData, i32) -> f64>;
            static mut mjcb_act_gain: Option<unsafe extern "C" fn(*const mjModel, *const mjData, i32) -> f64>;
            static mut mjcb_act_dyn: Option<unsafe extern "C" fn(*const mjModel, *const mjData, i32) -> f64>;
        }
        mjcb_passive = None;
        mjcb_control = None;
        mjcb_contactfilter = None;
        mjcb_sensor = None;
        mjcb_time = None;
        mjcb_act_bias = None;
        mjcb_act_gain = None;
        mjcb_act_dyn = None;
    }
}

