//! Port of: engine/engine_callback.c
//! IR hash: fd5288ad7242f001
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

/// C: mj_resetCallbacks (engine/engine_callback.c:33)
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_callbacks() {
    todo!() // cxx:_mj_resetCallbacks
}

