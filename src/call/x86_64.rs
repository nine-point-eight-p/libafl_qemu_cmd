use core::arch::asm;

use crate::common::OPCODE;

pub type LibaflWord = u64;

#[inline(never)]
pub fn libafl_qemu_call0(action: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".4byte {opcode}",
            opcode = const OPCODE,
            inlateout("rax") action => ret,
        );
    }

    ret
}

#[inline(never)]
pub fn libafl_qemu_call1(action: LibaflWord, arg1: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".4byte {opcode}",
            opcode = const OPCODE,
            inlateout("rax") action => ret,
            in("rdi") arg1,
        );
    }

    ret
}

#[inline(never)]
pub fn libafl_qemu_call2(action: LibaflWord, arg1: LibaflWord, arg2: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".4byte {opcode}",
            opcode = const OPCODE,
            inlateout("rax") action => ret,
            in("rdi") arg1,
            in("rsi") arg2,
        );
    }

    ret
}
