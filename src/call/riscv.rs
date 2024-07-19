use core::arch::asm;

use crate::call::LibaflWord;
use crate::common::{BACKDOOR_OPCODE, SYNC_BACKDOOR_OPCODE};

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
            inlateout("a0") action => ret,
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
            inlateout("a0") action => ret,
            in("a1") arg1,
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
            inlateout("a0") action => ret,
            in("a1") arg1,
            in("a2") arg2,
        );
    }

    ret
}
