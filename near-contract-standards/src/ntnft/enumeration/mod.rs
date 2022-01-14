mod enumeration_impl;

use crate::ntnft::token::Token;
use near_sdk::json_types::U128;
use near_sdk::AccountId;

/// Offers methods helpful in determining account ownership of NFTs and provides a way to page through NFTs per owner, determine total supply, etc.
pub trait NTNFTEnumeration {
    /// Returns the total supply of NTNFTs as a string representing an
    /// unsigned 128-bit integer to avoid JSON number limit of 2^53.
    fn ntnft_total_supply(&self) -> U128;

    /// Get a list of all tokens
    ///
    /// Arguments:
    /// * `from_index`: a string representing an unsigned 128-bit integer,
    ///    representing the starting index of tokens to return
    /// * `limit`: the maximum number of tokens to return
    ///
    /// Returns an array of Token objects, as described in Core standard
    fn ntnft_tokens(
        &self,
        from_index: Option<U128>, // default: "0"
        limit: Option<u64>,       // default: unlimited (could fail due to gas limit)
    ) -> Vec<Token>;

    /// Get number of tokens owned by a given account
    ///
    /// Arguments:
    /// * `account_id`: a valid NEAR account
    ///
    /// Returns the number of NTNFTs owned by given `account_id` as
    /// a string representing the value as an unsigned 128-bit integer to avoid JSON
    /// number limit of 2^53.
    fn ntnft_supply_for_owner(&self, account_id: AccountId) -> U128;

    /// Get list of all tokens owned by a given account
    ///
    /// Arguments:
    /// * `account_id`: a valid NEAR account
    /// * `from_index`: a string representing an unsigned 128-bit integer,
    ///    representing the starting index of tokens to return
    /// * `limit`: the maximum number of tokens to return
    ///
    /// Returns a paginated list of all tokens owned by this account
    fn ntnft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>, // default: "0"
        limit: Option<u64>,       // default: unlimited (could fail due to gas limit)
    ) -> Vec<Token>;
}
