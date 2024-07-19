//! Harness APIs for LibAFL QEMU.
#![feature(asm_const)]
#![no_std]
#![deny(missing_docs)]
#![deny(unused)]

mod call;
mod command;
mod common;

pub use call::LibaflWord;
pub use command::*;
pub use common::EndStatus;
