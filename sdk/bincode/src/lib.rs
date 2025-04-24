//! Contains a single utility function for deserializing from [bincode].
//!
//! [bincode]: https://docs.rs/bincode
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use bincode::{
    config::{Configuration, Fixint, LittleEndian, NoLimit},
    enc, error,
};

lazy_static::lazy_static! {
    pub static ref BINCODE_CONFIG_DEFAULT: Configuration<LittleEndian, Fixint> = bincode::config::legacy();
    // pub static ref BINCODE_CONFIG_DEFAULT: Configuration = bincode::config::standard();
}

pub fn bincode_serialize_into<T: enc::Encode>(
    entity: &T,
    dst: &mut [u8],
) -> Result<usize, error::EncodeError> {
    bincode::encode_into_slice(entity, dst, BINCODE_CONFIG_DEFAULT.clone())
}

pub fn bincode_serialize_original<T: enc::Encode>(
    entity: &T,
) -> Result<(Vec<u8>, usize), error::EncodeError> {
    let mut buf = vec![];
    let bytes_written = bincode_serialize_into(entity, &mut buf)?;
    Ok((buf, bytes_written))
}

pub fn bincode_serialize_config<T: enc::Encode, C: bincode::config::Config>(
    entity: &T,
    config: C,
) -> Result<Vec<u8>, error::EncodeError> {
    Ok(bincode::encode_to_vec(entity, config)?)
}

pub fn bincode_serialize<T: enc::Encode>(entity: &T) -> Result<Vec<u8>, error::EncodeError> {
    bincode_serialize_config(entity, BINCODE_CONFIG_DEFAULT.clone())
}

pub fn bincode_serialized_size<T: enc::Encode>(entity: &T) -> Result<usize, error::EncodeError> {
    // TODO need more efficient way to extract serialized size
    Ok(bincode_serialize(entity)?.len())
}

pub fn bincode_deserialize_config<T: bincode::de::Decode<()>, C: bincode::config::Config>(
    src: &[u8],
    config: C,
) -> Result<T, error::DecodeError> {
    Ok(bincode::decode_from_slice(src, config)?.0)
}

pub fn bincode_deserialize<T: bincode::de::Decode<()>>(
    src: &[u8],
) -> Result<T, error::DecodeError> {
    Ok(bincode_deserialize_config(
        src,
        BINCODE_CONFIG_DEFAULT.clone(),
    )?)
}

/// Deserialize with a limit based the maximum amount of data a program can expect to get.
/// This function should be used in place of direct deserialization to help prevent OOM errors
pub fn limited_deserialize<const LIMIT: usize, T>(
    instruction_data: &[u8],
) -> Result<T, error::DecodeError>
where
    T: bincode::de::Decode<()>, // serde::de::DeserializeOwned,
{
    Ok(bincode_deserialize_config(
        instruction_data,
        BINCODE_CONFIG_DEFAULT
            .with_limit::<LIMIT>()
            .with_fixed_int_encoding(),
    )?)
    // .allow_trailing_bytes() // to retain the behavior of bincode::deserialize with the new `options()` method
    // .deserialize_from(instruction_data)
    // .map_err(|_| InstructionError::InvalidInstructionData)
}

#[cfg(test)]
pub mod tests {
    use {super::*, solana_program::system_instruction::SystemInstruction};

    #[test]
    fn test_limited_deserialize_advance_nonce_account() {
        let item = SystemInstruction::AdvanceNonceAccount;
        let mut serialized = bincode::serialize(&item).unwrap();

        assert_eq!(
            serialized.len(),
            4,
            "`SanitizedMessage::get_durable_nonce()` may need a change"
        );

        assert_eq!(
            limited_deserialize::<4, SystemInstruction>(&serialized).as_ref(),
            Ok(&item)
        );
        assert!(limited_deserialize::<3, SystemInstruction>(&serialized).is_err());

        serialized.push(0);
        assert_eq!(
            limited_deserialize::<4, SystemInstruction>(&serialized).as_ref(),
            Ok(&item)
        );
    }
}
