use core::arch::asm;

use crate::common::{BACKDOOR_OPCODE, SYNC_BACKDOOR_OPCODE};

pub type LibaflWord = u32;

pub fn backdoor_call() {
    unsafe {
        asm!(
            ".word {opcode}",
            opcode = const BACKDOOR_OPCODE,
        );
    }
}

#[inline(never)]
pub fn sync_backdoor_call0(action: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".word {opcode}",
            opcode = const SYNC_BACKDOOR_OPCODE,
            inlateout("r0") action => ret,
        );
    }

    ret
}

#[inline(never)]
pub fn sync_backdoor_call1(action: LibaflWord, arg1: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".word {opcode}",
            opcode = const SYNC_BACKDOOR_OPCODE,
            inlateout("r0") action => ret,
            in("r1") arg1,
        );
    }

    ret
}

#[inline(never)]
pub fn sync_backdoor_call2(action: LibaflWord, arg1: LibaflWord, arg2: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".word {opcode}",
            opcode = const SYNC_BACKDOOR_OPCODE,
            inlateout("r0") action => ret,
            in("r1") arg1,
            in("r2") arg2,
        );
    }

    ret
}
