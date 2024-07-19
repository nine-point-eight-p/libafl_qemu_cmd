//! High-level APIs for calling backdoor hooks and sync backdoor commands.

use core::unreachable;

use crate::call::*;
use crate::common::{Command, EndStatus};

/// Invoke backdoor hooks
pub fn backdoor() {
    backdoor_call();
}

/// Start fuzzing with input buffer in virtual address space
pub fn start_virt(buf_vaddr: *mut u8, max_len: usize) -> LibaflWord {
    sync_backdoor_call2(
        Command::StartVirt as LibaflWord,
        buf_vaddr as LibaflWord,
        max_len as LibaflWord,
    )
}

/// Start fuzzing with input buffer in physical address space
pub fn start_phys(buf_paddr: *mut u8, max_len: usize) -> LibaflWord {
    sync_backdoor_call2(
        Command::StartPhys as LibaflWord,
        buf_paddr as LibaflWord,
        max_len as LibaflWord,
    )
}

/// Get input from buffer in virtual address space
pub fn input_virt(buf_vaddr: *mut u8, max_len: usize) -> LibaflWord {
    sync_backdoor_call2(
        Command::InputVirt as LibaflWord,
        buf_vaddr as LibaflWord,
        max_len as LibaflWord,
    )
}

/// Get input from buffer in physical address space
pub fn input_phys(buf_paddr: *mut u8, max_len: usize) -> LibaflWord {
    sync_backdoor_call2(
        Command::InputPhys as LibaflWord,
        buf_paddr as LibaflWord,
        max_len as LibaflWord,
    )
}

/// End fuzzing with status
pub fn end(status: EndStatus) -> ! {
    sync_backdoor_call1(Command::End as LibaflWord, status as LibaflWord);
    unreachable!("QEMU should have been terminated");
}

/// Create a new snapshot
pub fn save() -> LibaflWord {
    sync_backdoor_call0(Command::Save as LibaflWord)
}

/// Load a snapshot
pub fn load() -> LibaflWord {
    sync_backdoor_call0(Command::Load as LibaflWord)
}

/// Check compatibility with the specified LibAFL QEMU version
pub fn version(version: u32) -> LibaflWord {
    sync_backdoor_call1(Command::Version as LibaflWord, version as LibaflWord)
}

/// Allow a range of virtual addresses
pub fn vaddr_filter_allow_range(start: *const u8, end: *const u8) -> LibaflWord {
    sync_backdoor_call2(
        Command::VaddrFilterAllow as LibaflWord,
        start as LibaflWord,
        end as LibaflWord,
    )
}
