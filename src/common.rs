#[cfg(all(feature = "backdoor", feature = "sync_backdoor"))]
compile_error!(
    "feature \"backdoor\" and feature \"sync_backdoor\" cannot be enabled at the same time"
);

#[cfg(target_endian = "little")]
pub const BACKDOOR_OPCODE: u32 = 0x44f23a0f;
#[cfg(target_endian = "big")]
pub const BACKDOOR_OPCODE: u32 = 0x0f3af244;
#[cfg(target_endian = "little")]
pub const SYNC_BACKDOOR_OPCODE: u32 = 0x66f23a0f;
#[cfg(target_endian = "big")]
pub const SYNC_BACKDOOR_OPCODE: u32 = 0x0f3af266;

pub enum Command {
    StartVirt = 0,
    StartPhys = 1,
    InputVirt = 2,
    InputPhys = 3,
    End = 4,
    Save = 5,
    Load = 6,
    Version = 7,
    // VaddrFilterFollow = 8,
}

pub enum EndStatus {
    Unknown = 0,
    Ok = 1,
    Crash = 2,
}
