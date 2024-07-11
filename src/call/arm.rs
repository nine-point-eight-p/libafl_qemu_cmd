use std::arch::asm;

pub type LibaflWord = u32;

#[inline(never)]
pub fn libafl_qemu_call0(action: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".word {opcode}",
            opcode = const OPCODE,
            inlateout("r0") action => ret,
        );
    }

    ret
}

#[inline(never)]
pub fn libafl_qemu_call1(action: LibaflWord, arg1: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".word {opcode}",
            opcode = const OPCODE,
            inlateout("r0") action => ret,
            in("r1") arg1,
        );
    }

    ret
}

#[inline(never)]
pub fn libafl_qemu_call2(action: LibaflWord, arg1: LibaflWord, arg2: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".word {opcode}",
            opcode = const OPCODE,
            inlateout("r0") action => ret,
            in("r1") arg1,
            in("r2") arg2,
        );
    }

    ret
}
