use near_sdk::{env, require, AccountId, Balance, CryptoHash, Promise};

pub fn refund_deposit_to_account(storage_used: u64, account_id: AccountId, deposit_used: Option<Balance>) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let deposite_left = env::attached_deposit() - deposit_used.unwrap_or(0);

    require!(
        required_cost <= deposite_left,
        format!("Must attach {} yoctoNEAR to cover storage", required_cost)
    );

    let refund = deposite_left - required_cost;
    if refund > 1 {
        Promise::new(account_id).transfer(refund);
    }
}

/// Assumes that the predecessor will be refunded
pub fn refund_deposit(storage_used: u64, deposit_used: Option<Balance>) {
    refund_deposit_to_account(storage_used, env::predecessor_account_id(), deposit_used)
}

pub fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}
