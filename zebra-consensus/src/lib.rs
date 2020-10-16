//! Implementation of Zcash consensus checks.
//!
//! More specifically, this crate implements *semantic* validity checks,
//! as defined below.
//!
//! ## Verification levels.
//!
//! Zebra's implementation of the Zcash consensus rules is oriented
//! around three telescoping notions of validity:
//!
//! 1. *Structural Validity*, or whether the format and structure of the
//!    object are valid.  For instance, Sprout-on-BCTV14 proofs are not
//!    allowed in version 4 transactions, and a transaction with a spend
//!    or output description must include a binding signature.
//!
//! 2. *Semantic Validity*, or whether the object could potentially be
//!    valid, depending on the chain state.  For instance, a transaction
//!    that spends a UTXO must supply a valid unlock script; a shielded
//!    transaction must have valid proofs, etc.
//!
//! 3. *Contextual Validity*, or whether a semantically valid
//!    transaction is actually valid in the context of a particular
//!    chain state.  For instance, a transaction that spends a
//!    UTXO is only valid if the UTXO remains unspent; a
//!    shielded transaction spending some note must reveal a nullifier
//!    not already in the nullifier set, etc.
//!
//! *Structural validity* is enforced by the definitions of data
//! structures in `zebra-chain`.  *Semantic validity* is enforced by the
//! code in this crate.  *Contextual validity* is enforced in
//! `zebra-state` when objects are committed to the chain state.

#![doc(html_favicon_url = "https://www.zfnd.org/images/zebra-favicon-128.png")]
#![doc(html_logo_url = "https://www.zfnd.org/images/zebra-icon.png")]
#![doc(html_root_url = "https://doc.zebra.zfnd.org/zebra_consensus")]
// Re-enable this after cleaning the API surface.
//#![deny(missing_docs)]
#![allow(clippy::try_err)]

pub mod block;
pub mod chain;
pub mod checkpoint;
pub mod config;
pub mod error;
pub mod mempool;
pub mod parameters;
pub mod script;

#[allow(dead_code)] // Remove this once transaction verification is implemented
mod primitives;
mod transaction;

pub use crate::config::Config;

/// A boxed [`std::error::Error`].
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
