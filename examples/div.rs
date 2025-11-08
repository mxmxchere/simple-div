// examples/rtic_hello.rs

#![no_main]
#![no_std]

use nrf52840_hal as _;
use panic_rtt_target as _;

#[rtic::app(device = nrf52840_hal::pac)]
mod app {

    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        div(50, 70);
        rtt_init_print!();
        rprintln!("\n--- init ---\n");
        for a in 1..=18 {
            rprintln!("RTIC Says Hello, to all students!! {}", a);
        }

        (Shared {}, Local {}, init::Monotonics())
    }

    #[inline(never)]
    #[no_mangle]
    fn div(n: u32, d: u32) -> u32 {
        // d could be anything, we can assume 1 leading zero 
        // this leads to twelve cycles
        //(n >> 1) / d 
        
        // d could be anything, we can assume 10 leading zeros
        // this leads to ten cycles
        (n >> 10) / d 
        
        // d could be anything, we can assume 20 leading zeros
        // this leads to seven cycles
        //(n >> 20) / d 
    }
}
