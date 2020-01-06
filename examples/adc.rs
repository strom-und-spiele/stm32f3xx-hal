#![no_std]
#![no_main]

//! Example usage for ADC.

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f3xx_hal::{adc, prelude::*, stm32};

#[entry]
/// Main Thread
fn main() -> ! {
    // get peripherals, clocks and freeze them
    let mut dp = stm32::Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut peripherals.FLASH.constrain().acr);

    // set up adc1
    let mut adc1 = adc::Adc::adc1(
        peripherals.ADC1,
        &mut peripherals.ADC1_2,
        &mut rcc.ahb,
        clocks,
    );

    // set up pin pa0 as analog pin
    let mut gpio_a = peripherals.GPIOA.split(&mut rcc.ahb);
    let mut adc1_in1_pin = gpio_a.pa0.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);

    loop {
        let adc1_in1_data: u16 = adc1.read(&mut adc1_in1_pin).unwrap();
        hprintln!("pa0 reads {}", adc1_in1_data).expect("error in read");
    }
}
