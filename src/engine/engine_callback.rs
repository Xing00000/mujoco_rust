//! Port of: engine/engine_callback.h
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_resetCallbacks (engine/engine_callback.h:37)
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_callbacks() {
    todo!("C++: requires global mutable static callback function pointers (mjcb_passive, mjcb_control, mjcb_contactfilter, mjcb_sensor, mjcb_time, mjcb_act_bias, mjcb_act_gain, mjcb_act_dyn) — all set to null/0")
}

