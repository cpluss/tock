#![feature(asm, concat_idents, const_fn, const_cell_new, try_from)]
#![no_std]
#![crate_name = "cc26x0"]
#![crate_type = "rlib"]
extern crate cc26xx;
extern crate cortexm3;
#[allow(unused_imports)]
#[macro_use]
extern crate kernel;

pub mod chip;
pub mod crt1;
pub mod i2c;
pub mod sensor;
pub mod hdc;
pub mod mpu;

// Since the setup code is converted from C -> Rust, we
// ignore side effects from the conversion (unused vars & muts).
#[allow(unused, unused_mut)]
mod setup;

pub use crt1::init;
