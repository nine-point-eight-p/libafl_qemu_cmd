use core::arch::asm;

use crate::call::LibaflWord;
use crate::common::{BACKDOOR_OPCODE, SYNC_BACKDOOR_OPCODE};

pub fn backdoor_call() {
    unsafe {
        asm!(
            ".4byte {opcode}",
            opcode = const BACKDOOR_OPCODE,
        );
    }
}

#[inline(never)]
pub fn sync_backdoor_call0(action: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".4byte {opcode}",
            opcode = const SYNC_BACKDOOR_OPCODE,
            inlateout("rax") action => ret,
        );
    }

    ret
}

#[inline(never)]
pub fn sync_backdoor_call1(action: LibaflWord, arg1: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".4byte {opcode}",
            opcode = const SYNC_BACKDOOR_OPCODE,
            inlateout("rax") action => ret,
            in("rdi") arg1,
        );
    }

    ret
}

#[inline(never)]
pub fn sync_backdoor_call2(action: LibaflWord, arg1: LibaflWord, arg2: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            ".4byte {opcode}",
            opcode = const SYNC_BACKDOOR_OPCODE,
            inlateout("rax") action => ret,
            in("rdi") arg1,
            in("rsi") arg2,
        );
    }

    ret
}
