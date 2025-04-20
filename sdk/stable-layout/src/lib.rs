//! Types with stable memory layouts
//!
//! Internal use only; here be dragons!

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod stable_instruction;
pub mod stable_rc;
pub mod stable_ref_cell;
pub mod stable_slice;
pub mod stable_vec;
