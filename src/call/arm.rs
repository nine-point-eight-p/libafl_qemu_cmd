use std::arch::asm;

pub type LibaflWord = u64;

#[naked]
fn backdoor() {
    unsafe {
        asm!(
            ".word {opcode}",
            "bx lr",
            opcode = const OPCODE,
        );
    }
}

pub fn libafl_qemu_call0(action: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            "mov r0, {1}",
            "bl {2}",
            "mov {0}, r0",
            out(reg) ret,
            in(reg) action,
            in(reg) backdoor,
        );
    }

    ret
}

pub fn libafl_qemu_call1(action: LibaflWord, arg1: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            "mov r0, {1}",
            "mov r1, {2}",
            "bl {3}",
            "mov {0}, r0",
            out(reg) ret,
            in(reg) action,
            in(reg) arg1,
            in(reg) backdoor,
        );
    }

    ret
}

pub fn libafl_qemu_call2(action: LibaflWord, arg1: LibaflWord, arg2: LibaflWord) -> LibaflWord {
    let ret: LibaflWord;

    unsafe {
        asm!(
            "mov r0, {1}",
            "mov r1, {2}",
            "mov r2, {3}",
            "bl {4}",
            "mov {0}, r0",
            out(reg) ret,
            in(reg) action,
            in(reg) arg1,
            in(reg) arg2,
            in(reg) backdoor,
        );
    }

    ret
}
