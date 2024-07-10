use core::arch::asm;

use crate::common::OPCODE;

#[cfg(target_arch = "riscv32")]
pub type LibaflWord = u32;

#[cfg(target_arch = "riscv64")]
pub type LibaflWord = u64;

#[no_mangle]
#[naked]
extern "C" fn libafl_qemu_backdoor() {
    unsafe {
        asm!(
            ".4byte {opcode}",
            "ret",
            opcode = const OPCODE,
            options(noreturn),
        );
    }
}

pub fn libafl_qemu_call0(action: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            "call libafl_qemu_backdoor",
            inlateout("a0") action => ret,
        );
    }

    ret
}

pub fn libafl_qemu_call1(action: LibaflWord, arg1: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            "call libafl_qemu_backdoor",
            inlateout("a0") action => ret,
            in("a1") arg1,
        );
    }
    
    ret
}

pub fn libafl_qemu_call2(action: LibaflWord, arg1: LibaflWord, arg2: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            "call libafl_qemu_backdoor",
            inlateout("a0") action => ret,
            in("a1") arg1,
            in("a2") arg2,
        );
    }

    ret
}
