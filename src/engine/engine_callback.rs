//! Port of: engine/engine_callback.h
//! IR hash: 73a9f665ec0ecfc0
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

# [no_mangle] pub static mut mjcb_passive : Option < unsafe extern "C" fn (* const mjModel , * mut mjData) > = None ;
# [no_mangle] pub static mut mjcb_control : Option < unsafe extern "C" fn (* const mjModel , * mut mjData) > = None ;
# [no_mangle] pub static mut mjcb_contactfilter : Option < unsafe extern "C" fn (* const mjModel , * mut mjData , i32 , i32) -> i32 > = None ;
# [no_mangle] pub static mut mjcb_sensor : Option < unsafe extern "C" fn (* const mjModel , * mut mjData , i32) > = None ;
# [no_mangle] pub static mut mjcb_time : Option < unsafe extern "C" fn () -> f64 > = None ;
# [no_mangle] pub static mut mjcb_act_bias : Option < unsafe extern "C" fn (* const mjModel , * const mjData , i32) -> f64 > = None ;
# [no_mangle] pub static mut mjcb_act_gain : Option < unsafe extern "C" fn (* const mjModel , * const mjData , i32) -> f64 > = None ;
# [no_mangle] pub static mut mjcb_act_dyn : Option < unsafe extern "C" fn (* const mjModel , * const mjData , i32) -> f64 > = None ;

/// C: mj_resetCallbacks (engine/engine_callback.h:37)
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_callbacks() {
    // SAFETY: single-threaded access to global callback pointers, matching C semantics.
    // All callbacks are set to null (None) exactly as C sets them to 0.
    unsafe {
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

