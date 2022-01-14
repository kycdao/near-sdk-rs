mod core_impl;

pub use self::core_impl::*;

use crate::ntnft::token::{Token, TokenId};

/// Used for all NTNFTs. The specification for the
/// [core NTNFT standard] lays out the reasoning for each method.
/// It's important to check out [NTNFTReceiver](crate::ntnft::core::NTNFTReceiver)
/// and [NTNFTResolver](crate::ntnft::core::NTNFTResolver) to
/// understand how the cross-contract call work.
///
/// [core NTNFT standard]: https://nomicon.io/Standards/NTNFT/Core.html
pub trait NTNFTCore {
    /// Returns the token with the given `token_id` or `null` if no such token.
    fn ntnft_token(&self, token_id: TokenId) -> Option<Token>;
}
