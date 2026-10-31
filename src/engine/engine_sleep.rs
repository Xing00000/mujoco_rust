//! Port of: engine/engine_sleep.c
//! IR hash: ed97cb44a3b1517e
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

#[allow(unused_variables, non_snake_case)]
pub fn mj_sleep_cycle(tree_asleep: *const i32, ntree: i32, i: i32) -> i32 {
    if i < 0 || i >= ntree {
        return -1;
    }

    let mut smallest = i;
    let mut current = i;
    let mut count: i32 = 0;

    loop {
        if count > ntree {
            return -1;
        }

        let next = unsafe { *tree_asleep.add(current as usize) };

        if next < 0 || next >= ntree {
            return -1;
        }

        if next < smallest {
            smallest = next;
        }

        current = next;
        count += 1;

        if current == i {
            break;
        }
    }

    smallest
}

