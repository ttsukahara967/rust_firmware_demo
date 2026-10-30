#![no_std] // No OS, so no standard library
#![no_main] // Entry point comes from cortex-m-rt's reset handler, not a normal main

use cortex_m::peripheral::{syst::SystClkSource, Peripherals};
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_halt as _;

#[entry]
fn main() -> ! {
    hprintln!("Hello from bare-metal Rust!");

    // Peripherals::take() returns Some only once: ownership guarantees a single owner of the hardware
    let cp = Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(12_000_000 / 10); // ~100 ms at 12 MHz
    syst.clear_current();
    syst.enable_counter();

    for tick in 1..=5 {
        while !syst.has_wrapped() {}
        hprintln!("SysTick #{}", tick);
    }

    let cpuid = cp.CPUID.base.read();
    hprintln!("CPUID = {:#010x}", cpuid);

    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
