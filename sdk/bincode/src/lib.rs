//! Contains a single utility function for deserializing from [bincode].
//!
//! [bincode]: https://docs.rs/bincode
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use bincode::serde::Compat;
use bincode::{
    config::{Configuration, Fixint, LittleEndian},
    error,
};

lazy_static::lazy_static! {
    pub static ref CONFIG_DEFAULT: Configuration<LittleEndian, Fixint> = bincode::config::legacy();
    // pub static ref CONFIG_DEFAULT: Configuration = bincode::config::standard();
}

pub fn serialize_into<T: serde::Serialize>(
    entity: &T,
    dst: &mut [u8],
) -> Result<usize, error::EncodeError> {
    let entity = Compat(entity);
    bincode::encode_into_slice(entity, dst, CONFIG_DEFAULT.clone())
}

pub fn serialize_original<T: serde::Serialize>(
    entity: &T,
) -> Result<(Vec<u8>, usize), error::EncodeError> {
    let mut buf = vec![];
    let bytes_written = serialize_into(entity, &mut buf)?;
    Ok((buf, bytes_written))
}

pub fn serialize_config<T: serde::Serialize, C: bincode::config::Config>(
    entity: &T,
    config: C,
) -> Result<Vec<u8>, error::EncodeError> {
    let entity = Compat(entity);
    Ok(bincode::encode_to_vec(entity, config)?)
}

pub fn serialize<T: serde::Serialize>(entity: &T) -> Result<Vec<u8>, error::EncodeError> {
    serialize_config(entity, CONFIG_DEFAULT.clone())
}

pub fn serialized_size<T: serde::Serialize>(entity: &T) -> Result<usize, error::EncodeError> {
    // TODO need more efficient way to extract serialized size
    Ok(serialize(entity)?.len())
}

pub fn deserialize_config<T: serde::de::DeserializeOwned, C: bincode::config::Config>(
    src: &[u8],
    config: C,
) -> Result<T, error::DecodeError> {
    let entity: Compat<T> = bincode::decode_from_slice(src, config)?.0;
    Ok(entity.0)
}

pub fn deserialize<T: serde::de::DeserializeOwned>(src: &[u8]) -> Result<T, error::DecodeError> {
    Ok(deserialize_config(src, CONFIG_DEFAULT.clone())?)
}

/// Deserialize with a limit based the maximum amount of data a program can expect to get.
/// This function should be used in place of direct deserialization to help prevent OOM errors
pub fn limited_deserialize<const LIMIT: usize, T>(
    instruction_data: &[u8],
) -> Result<T, error::DecodeError>
where
    T: serde::de::DeserializeOwned, // serde::de::DeserializeOwned,
{
    let result = deserialize_config(
        instruction_data,
        CONFIG_DEFAULT
            .with_limit::<LIMIT>()
            .with_fixed_int_encoding(),
    )?;
    Ok(result)
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
