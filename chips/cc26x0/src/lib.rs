#![feature(asm, concat_idents, const_fn, const_cell_new, try_from)]
#![no_std]
#![crate_name = "cc26x0"]
#![crate_type = "rlib"]
extern crate cortexm3;
#[allow(unused_imports)]
#[macro_use(debug)]
extern crate kernel;
extern crate cc26xx;

extern crate bitfield;

pub mod chip;
pub mod crt1;
pub mod i2c;

pub use crt1::init;
