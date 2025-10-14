#![no_std]

//! Pinocchio-based SDK for Metaplex Token Metadata program
//!
//! This crate provides a minimal, zero-copy implementation for interacting with
//! Metaplex Token Metadata program from Solana programs using Pinocchio.

pub mod constant;
pub use constant::*;

pub mod types;
pub use types::*;

pub mod create_metadata_account_v3;
pub use create_metadata_account_v3::*;
