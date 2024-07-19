//! Low-level implementation of backdoor and sync backdoor calls,
//! involving much inline assembly, which is separated into
//! different modules for different architectures.

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "arm")]
mod arm;

#[cfg(target_arch = "arm")]
pub use arm::*;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub use riscv::*;

/// The word size of the target architecture.
#[cfg(target_os = "windows")]
pub type LibaflWord = u64;

/// The word size of the target architecture.
#[cfg(not(target_os = "windows"))]
pub type LibaflWord = usize;
