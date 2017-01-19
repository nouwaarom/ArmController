#![crate_type = "staticlib"]

extern crate libc;

use libc::{size_t};
use std::slice;

#[no_mangle]
pub extern fn get_angle(joint: i32) -> f64
{
    // Get action from host
    // Convert action to end effector path
    // * do speed control (we need to keep track of time)
    // Calculate angles using IK
    // Send the angles and timing information to the real-time arm controller
    joint as f64 * 3.0
}

#[no_mangle]
pub extern fn set_dh(dh_table: *const f64, joint_cnt: u32) -> i32
{
    let denavit_hartenberg_matrix = unsafe {
        assert!(!dh_table.is_null());
        slice::from_raw_parts(dh_table, (joint_cnt*4) as usize)
    };

    for i in 0..joint_cnt {
        let row_index: usize = (i*4) as usize;
        println!("Joint: {}", i);
        println!("theta:{}, d:{}, a:{}, alpha:{}",
                 denavit_hartenberg_matrix[row_index],
                 denavit_hartenberg_matrix[row_index+1],
                 denavit_hartenberg_matrix[row_index+2],
                 denavit_hartenberg_matrix[row_index+3],
        );
    }
    return 1;
}

#[no_mangle]
pub extern fn set_desired_end_effector_position(position: Vec<f64>, rotation: Vec<f64>)
{
   // Do stuff
}
