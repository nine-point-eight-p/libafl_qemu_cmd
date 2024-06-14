#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "arm")]
mod arm;

#[cfg(target_arch = "arm")]
pub use arm::*;