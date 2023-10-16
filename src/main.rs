#![no_main]
#![no_std]

use core::fmt::{Display, Formatter, Write};
use stm32h7xx_hal::hal::fmt;
use stm32h7xx_hal::prelude::_embedded_hal_serial_Read;
use stm32h7xx_hal::serial::{Rx, Serial, Tx};

use cortex_m::{asm, Peripherals};
use rtt_target::{rtt_init_print, rprintln, rprint};
use rtt_target::rtt::*;
use cortex_m_rt::entry;
use panic_halt as _;

use stm32h7xx_hal::{
    pac,
    prelude::*,
    delay::Delay
};
use stm32h7xx_hal::*;


#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();

    let core = cortex_m::Peripherals::take().unwrap();

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();
    dp.RCC.ahb1enr.write(|w| w.dma1en().set_bit());



    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();

    let pwrcfg = pwr.freeze();


    // let clocks = rcc.hclk(100.mhz()).freeze(pwrcfg, &dp.SYSCFG);

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    // let rcc = rcc.sys_ck(400.mhz()).use_hse(8.mhz()).bypass_hse();
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);



    // Acquire the GPIOB peripheral
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Configure gpio B pin 14 as a push-pull output.
    let mut ld3 = gpiob.pb14.into_push_pull_output();

    // Acquire the GPIOD peripheral
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    // initialize serial
    let tx = gpiod.pd8.into_alternate();
    let rx = gpiod.pd9.into_alternate();
    let serial  = dp
        .USART3
        .serial((tx, rx), 115200.bps(), ccdr.peripheral.USART3, &ccdr.clocks)
        .unwrap();

    let (mut tx, mut rx) = serial.split();

    let mut delay = Delay::new(core.SYST, ccdr.clocks);

    rprintln!("Hello, world!");

    loop {
        loop {
            if let Ok(received_byte) = rx.read() {
                let byte = received_byte as char;
                if byte == '0' {
                    break;
                } else {
                    ld3.set_high();
                    rprint!("{}",byte);
                    tx.write(received_byte).unwrap();
                }
            } else {
                ld3.set_low();
            }
        }

        asm::bkpt()

    }
}
