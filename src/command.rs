use core::unreachable;

use crate::call::*;
use crate::common::{Command, EndStatus};

pub fn start_virt(buf_vaddr: *mut u8, max_len: usize) -> LibaflWord {
    libafl_qemu_call2(
        Command::StartVirt as LibaflWord,
        buf_vaddr as LibaflWord,
        max_len as LibaflWord,
    )
}

pub fn start_phys(buf_paddr: *mut u8, max_len: usize) -> LibaflWord {
    libafl_qemu_call2(
        Command::StartPhys as LibaflWord,
        buf_paddr as LibaflWord,
        max_len as LibaflWord,
    )
}

pub fn input_virt(buf_vaddr: *mut u8, max_len: usize) -> LibaflWord {
    libafl_qemu_call2(
        Command::InputVirt as LibaflWord,
        buf_vaddr as LibaflWord,
        max_len as LibaflWord,
    )
}

pub fn input_phys(buf_paddr: *mut u8, max_len: usize) -> LibaflWord {
    libafl_qemu_call2(
        Command::InputPhys as LibaflWord,
        buf_paddr as LibaflWord,
        max_len as LibaflWord,
    )
}

pub fn end(status: EndStatus) -> ! {
    libafl_qemu_call1(Command::End as LibaflWord, status as LibaflWord);
    unreachable!("QEMU should have been terminated");
}

pub fn save() -> LibaflWord {
    libafl_qemu_call0(Command::Save as LibaflWord)
}

pub fn load() -> LibaflWord {
    libafl_qemu_call0(Command::Load as LibaflWord)
}

pub fn version() -> LibaflWord {
    libafl_qemu_call0(Command::Version as LibaflWord)
}
