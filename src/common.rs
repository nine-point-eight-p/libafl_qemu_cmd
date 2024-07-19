//! Common values for all architectures

/// Opcode for backdoor calls
#[cfg(target_endian = "little")]
pub const BACKDOOR_OPCODE: u32 = 0x44f23a0f;
#[cfg(target_endian = "big")]
pub const BACKDOOR_OPCODE: u32 = 0x0f3af244;

/// Opcode for sync backdoor calls
#[cfg(target_endian = "little")]
pub const SYNC_BACKDOOR_OPCODE: u32 = 0x66f23a0f;
#[cfg(target_endian = "big")]
pub const SYNC_BACKDOOR_OPCODE: u32 = 0x0f3af266;

/// Command codes for sync backdoor calls
pub enum Command {
    StartVirt = 0,
    StartPhys = 1,
    InputVirt = 2,
    InputPhys = 3,
    End = 4,
    Save = 5,
    Load = 6,
    Version = 7,
    VaddrFilterAllow = 8,
}

/// Execution status when fuzzing ends
pub enum EndStatus {
    /// Execution status is unknown
    Unknown = 0,
    /// Execution succeeded
    Ok = 1,
    /// Execution crashed
    Crash = 2,
}
