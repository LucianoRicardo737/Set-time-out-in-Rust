use std::{thread::sleep,time::Duration};

pub fn set_time_out(miliseconds: u64) {
    sleep(Duration::from_millis(miliseconds));
}

pub fn set_time_out_callback(miliseconds: u64, callback: Option<fn()>)  {
    sleep(Duration::from_millis(miliseconds));
    if let Some(callback) = callback {
        callback();
    }
}