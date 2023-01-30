#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_time::{Delay, Timer, Duration};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut delay = Delay;
    let mut adc = Adc::new(p.ADC1, &mut delay);

    loop {
        let r1 = adc.read(&mut p.PA0);
        let r2 = adc.read(&mut p.PA1);
        info!("ADC outputs: 0x{:x} 0x{:x}", r1, r2);
        Timer::after(Duration::from_secs(1)).await;
    }
}
