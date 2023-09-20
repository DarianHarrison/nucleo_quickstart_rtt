//! Blinks an LED

#![no_main]
#![no_std]

// core components
use stm32l4xx_hal::prelude::*;
use cortex_m_rt::entry;
use cortex_m;

// rng
use stm32l4xx_hal::delay::Delay;
use stm32l4xx_hal::hal::blocking::rng::Read;

// dev
use panic_rtt_target as _;
use rtt_target::rprintln;



#[entry]
fn main() -> ! {

    // rtt config
    rtt_target::rtt_init_default!();
    
    /////////////////////
    // BOARD SETUP
    /////////////////////

    let cp = cortex_m::Peripherals::take().unwrap(); // processor peripherals
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap(); // board peripherals

    let mut flash = dp.FLASH.constrain(); // .constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .freeze(&mut flash.acr, &mut pwr);

    /////////////////////

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut timer = Delay::new(cp.SYST, clocks);

    loop {
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        led.set_high();
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        led.set_low();
    }
}