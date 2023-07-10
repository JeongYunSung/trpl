use std::env;
use self::net_kqueue::*;

mod poller;
mod net_kqueue;

#[cfg(target_os = "macos")]
fn main() {
    if let Some(pid) = env::args().nth(1) {
        if let Ok(npid) = pid.parse::<libc::pid_t>() {
            if let Err(err) = watch_pid(npid) {
                println!("Error: {}", err);
            }
        }
    } else {
        println!("Usage: cargo run --example pid <pid>");
    }
}