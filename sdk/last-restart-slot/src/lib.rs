//! Information about the last restart slot (hard fork).
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "sysvar")]
pub mod sysvar;

use solana_sdk_macro::CloneZeroed;

#[repr(C)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
#[derive(Debug, CloneZeroed, PartialEq, Eq, Default, bincode::Encode, bincode::Decode)]
pub struct LastRestartSlot {
    /// The last restart `Slot`.
    pub last_restart_slot: u64,
}
