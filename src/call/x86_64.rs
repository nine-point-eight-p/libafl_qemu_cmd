use core::arch::asm;

use crate::common::OPCODE;

pub type LibaflWord = u64;

#[naked]
extern "C" fn backdoor() {
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
            "mov rax, {1}",
            "call {2}",
            "mov {0}, rax",
            lateout(reg) ret,
            in(reg) action,
            in(reg) backdoor,
            // clobbered registers
            out("rax") _,
        );
    }

    ret
}

pub fn libafl_qemu_call1(action: LibaflWord, arg1: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            "mov rax, {1}",
            "mov rdi, {2}",
            "call {3}",
            "mov {0}, rax",
            lateout(reg) ret,
            in(reg) action,
            in(reg) arg1,
            in(reg) backdoor,
            // clobbered registers
            out("rax") _,
            out("rdi") _,
        );
    }
    
    ret
}

pub fn libafl_qemu_call2(action: LibaflWord, arg1: LibaflWord, arg2: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            "mov rax, {1}",
            "mov rdi, {2}",
            "mov rsi, {3}",
            "call {4}",
            "mov {0}, rax",
            lateout(reg) ret,
            in(reg) action,
            in(reg) arg1,
            in(reg) arg2,
            in(reg) backdoor,
            // clobbered registers
            out("rax") _,
            out("rdi") _,
            out("rsi") _,
        );
    }

    ret
}
