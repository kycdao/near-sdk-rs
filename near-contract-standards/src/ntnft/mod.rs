/// The [core NTNFT standard](https://nomicon.io/Standards/NTNFT/Core.html). This can be though of as the base standard, with the others being extension standards.
pub mod core;
/// Common implementation of the [core NTNFT standard](https://nomicon.io/Standards/NTNFT/Core.html).
/// Trait for the [NFT enumeration standard](https://nomicon.io/Standards/NTNFT/Enumeration.html).
/// This provides useful view-only methods returning token supply, tokens by owner, etc.
pub mod enumeration;
/// Macros typically used by a contract wanting to take advantage of the non-fungible
/// token NEAR contract standard approach.
mod macros;
/// Metadata traits and implementation according to the [NFT enumeration standard](https://nomicon.io/Standards/NTNFT/Metadata.html).
/// This covers both the contract metadata and the individual token metadata.
pub mod metadata;
/// The Token struct for the NTNFT.
mod token;
pub use self::token::{Token, TokenId};

/// NFT utility functions
mod utils;
pub use utils::*;

pub use self::core::NTNFT;
pub use macros::*;
