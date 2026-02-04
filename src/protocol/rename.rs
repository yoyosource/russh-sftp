use super::{impl_packet_for, impl_request_id, Packet, RequestId};

/// Implementation for `SSH_FXP_RENAME`
#[derive(Debug, Serialize, Deserialize)]
pub struct Rename {
    pub id: u32,
    pub oldpath: String,
    pub newpath: String,
    pub flags: RenameFlags,
}

impl_request_id!(Rename);
impl_packet_for!(Rename);

/// Attributes flags according to the specification
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenameFlags(u32);

bitflags! {
    impl RenameFlags: u32 {
        const OVERWRITE = 0x00000001;
        const REQUEST_ATOMIC = 0x00000002;
        const REQUEST_NATIVE = 0x00000004;
    }
}
