#![feature(never_type)]
#![feature(non_exhaustive)]
#![feature(unsize)]
#![no_std]

// #[macro_use]
// pub mod action;
// pub mod bluetooth;
pub mod clock;
pub mod debug;
// pub mod hidreport;
// pub mod keyboard;
// pub mod keycodes;
// pub mod keymatrix;
// pub mod layout;
// pub mod led;
// pub mod protocol;
// pub mod serial;
// pub mod usb;

pub use debug::heprintln;

enum Threshold {}
