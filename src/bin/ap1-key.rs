#![no_main]
#![no_std]

#[cfg(not(feature = "use_semihosting"))]
extern crate panic_abort;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

extern crate rtfm4 as rtfm;

use hal::gpio::GpioExt;
use rtfm::app;

use anne_key::clock;

const _BUFFER_SIZE: usize = 0x80;

#[app(device = stm32l1::stm32l151)]
const APP: () = {
    static BLUETOOTH_BUFFERS: [[u8; _BUFFER_SIZE]; 2] = [[0; _BUFFER_SIZE]; 2];
    static LED_BUFFERS: [[u8; _BUFFER_SIZE]; 2] = [[0; _BUFFER_SIZE]; 2];

    #[init]
    fn init() {
        // re-locate vector table to 0x80004000 because bootloader uses 0x80000000
        unsafe { core.SCB.vtor.write(0x4000) };
        anne_key::heprintln!("init").unwrap();
        //        hprintln!("vector table relocated").ok();
        clock::init_clock(&device);
        clock::enable_tick(&mut core.SYST, 100_000);
        anne_key::heprintln!("clocked").unwrap();

        let gpioc = device.GPIOC.split();
        gpioc.pc15;
    }

    #[idle]
    fn idle() -> ! {
        anne_key::heprintln!("idle").unwrap();
        loop {}
    }
};
