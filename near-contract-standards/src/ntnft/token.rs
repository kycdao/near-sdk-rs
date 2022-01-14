use crate::ntnft::metadata::TokenMetadata;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

/// Note that token IDs for NFTs are strings on NEAR. It's still fine to use autoincrementing numbers as unique IDs if desired, but they should be stringified. This is to make IDs more future-proof as chain-agnostic conventions and standards arise, and allows for more flexibility with considerations like bridging NFTs across chains, etc.
pub type TokenId = String;

/// In this implementation, the Token struct takes one extensions standard (metadata) as optional field, as it is requently used in modern NFTs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: Option<TokenMetadata>,
}
