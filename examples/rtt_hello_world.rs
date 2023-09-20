#![no_main]
#![no_std]

// core components
use stm32l4xx_hal::prelude::*;
use cortex_m_rt::entry;


// dev
use panic_rtt_target as _;
use rtt_target::rprintln;


#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let lion10 = "Lion10";
    rprintln!("Hello, {} !!", lion10);

    loop {}
}