//! This example shows how to use the DAC on PIN 25 and 26
//! You can connect an LED (with a suitable resistor) or check the changing
//! voltage using a voltmeter on those pins.

#![no_std]
#![no_main]

use esp32_hal::{
    clock::ClockControl,
    dac,
    gpio::IO,
    pac::Peripherals,
    prelude::*,
    Delay,
    RtcCntl,
    Timer,
};
use panic_halt as _;
use xtensa_lx_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut timer0 = Timer::new(peripherals.TIMG0, clocks.apb_clock);
    let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);

    // Disable MWDT and RWDT (Watchdog) flash boot protection
    timer0.disable();
    rtc_cntl.set_wdt_global_enable(false);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pin25 = io.pins.gpio25.into_analog();
    let pin26 = io.pins.gpio26.into_analog();

    // Create DAC instances
    let analog = peripherals.SENS.split();
    let mut dac1 = dac::DAC1::dac(analog.dac1, pin25).unwrap();
    let mut dac2 = dac::DAC2::dac(analog.dac2, pin26).unwrap();

    let mut delay = Delay::new(&clocks);

    let mut voltage_dac1: u8 = 200;
    let mut voltage_dac2: u8 = 255;
    loop {
        // Change voltage on the pins using write function
        voltage_dac1 = voltage_dac1.wrapping_add(1);
        dac1.write(voltage_dac1);

        voltage_dac2 = voltage_dac2.wrapping_sub(1);
        dac2.write(voltage_dac2);
        delay.delay_ms(50u32);
    }
}
