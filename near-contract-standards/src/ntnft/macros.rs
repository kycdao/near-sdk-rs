/// The core methods for a basic NTNFT. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_ntnft_core {
    ($contract: ident, $token: ident) => {
        use $crate::ntnft::core::NTNFTCore;

        #[near_bindgen]
        impl NTNFTCore for $contract {
            fn ntnft_token(&self, token_id: TokenId) -> Option<Token> {
                self.$token.ntnft_token(token_id)
            }
        }
    };
}

/// Non-fungible enumeration adds the extension standard offering several
/// view-only methods to get token supply, tokens per owner, etc.
#[macro_export]
macro_rules! impl_ntnft_enumeration {
    ($contract: ident, $token: ident) => {
        use $crate::ntnft::enumeration::NTNFTEnumeration;

        #[near_bindgen]
        impl NTNFTEnumeration for $contract {
            fn ntnft_total_supply(&self) -> near_sdk::json_types::U128 {
                self.$token.ntnft_total_supply()
            }

            fn ntnft_tokens(
                &self,
                from_index: Option<near_sdk::json_types::U128>,
                limit: Option<u64>,
            ) -> Vec<Token> {
                self.$token.ntnft_tokens(from_index, limit)
            }

            fn ntnft_supply_for_owner(&self, account_id: AccountId) -> near_sdk::json_types::U128 {
                self.$token.ntnft_supply_for_owner(account_id)
            }

            fn ntnft_tokens_for_owner(
                &self,
                account_id: AccountId,
                from_index: Option<near_sdk::json_types::U128>,
                limit: Option<u64>,
            ) -> Vec<Token> {
                self.$token.ntnft_tokens_for_owner(account_id, from_index, limit)
            }
        }
    };
}
