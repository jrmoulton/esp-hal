#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]

use core::{fmt::Write, sync::atomic::Ordering};

use esp32s2_hal::{pac::Peripherals, prelude::*, RtcCntl, Serial, Timer};
use nb::block;
use panic_halt as _;
use xtensa_lx_rt::entry;

use xtensa_atomic_emulation_trap as _;
use xtensa_lx_rt::exception::{Context, ExceptionCause};

extern crate xtensa_atomic_emulation_trap;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let mut timer0 = Timer::new(peripherals.TIMG0);
    let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);
    let mut serial0 = Serial::new(peripherals.UART0).unwrap();

    // Disable MWDT and RWDT (Watchdog) flash boot protection
    timer0.disable();
    rtc_cntl.set_wdt_global_enable(false);

    timer0.start(40_000_000u64);

    loop {
        writeln!(serial0, "Hello world!").unwrap();
        block!(timer0.wait()).unwrap();

        let x = core::sync::atomic::AtomicUsize::new(0);

        let old = x.compare_exchange(0, 16, Ordering::SeqCst, Ordering::SeqCst).unwrap();

        writeln!(serial0, "Old value: {}, current: {}", old, x.load(Ordering::Acquire)).unwrap();

        unsafe { core::arch::asm!("quos {0},{0},{0}",in(reg) 0) } // trigger divide by zero
    }
}

#[xtensa_lx_rt::exception]
unsafe fn ruh_roh(cause: ExceptionCause, frame: &mut Context) -> ! {
    let peripherals = Peripherals::steal();
    let mut serial0 = Serial::new(peripherals.UART0).unwrap();

    writeln!(serial0, "Exception Reason: {:?}", cause).unwrap();

    loop {}
}
