use std::{
    thread::sleep, 
    time::Duration
};

pub fn set_time_out(miliseconds: u64) {
    sleep(Duration::from_millis(miliseconds));
}
