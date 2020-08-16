//! Blockchain-related datastructures for Zebra. 🦓
//!
//! $$a^2 + b^2 = c^2$$

#![doc(html_favicon_url = "https://www.zfnd.org/images/zebra-favicon-128.png")]
#![doc(html_logo_url = "https://www.zfnd.org/images/zebra-icon.png")]
#![doc(html_root_url = "https://doc.zebra.zfnd.org/zebra_chain")]
//#![deny(missing_docs)]
#![allow(clippy::try_err)]

#[macro_use]
extern crate serde;

mod merkle_tree;

pub mod addresses;
pub mod keys;

pub mod amount;
pub mod block;
pub mod parameters;
pub mod primitives;
pub mod sapling;
pub mod serialization;
pub mod sprout;
pub mod transaction;
pub mod transparent;
pub mod work;
