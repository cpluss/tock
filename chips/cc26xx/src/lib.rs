#![feature(asm, concat_idents, const_fn, const_cell_new, try_from)]
#![no_std]
#![crate_name = "cc26xx"]
#![crate_type = "rlib"]
extern crate bitfield;
#[allow(unused_imports)]
#[macro_use]
extern crate kernel;

pub mod aon;
pub mod aux;
pub mod rtc;
pub mod gpio;
pub mod ioc;
pub mod osc;
pub mod prcm;
pub mod ccfg;
pub mod trng;
pub mod uart;
pub mod timer;
pub mod peripheral_interrupts;

// Since the setup code is converted from C -> Rust, we
// ignore side effects from the conversion (unused vars & muts).
#[allow(unused_variables, unused_mut)]
pub mod setup;
