use bigdecimal::{BigDecimal, ToPrimitive};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::require;
use near_sdk::{
    env, ext_contract, log, near_bindgen, serializer, AccountId, Balance, Gas, PanicOnDefault,
    PromiseResult,
};
use std::str::FromStr;

const NO_DEPOSIT: Balance = 0;
// Need to atach --gas=200000000000000 to 'borrow' call (80TGas here and 200TGas for call)
const CONTROLLER_ACCOUNT_ID: &str = "ctrl.nearlend.testnet";
const WETH_TOKEN_ACCOUNT_ID: &str = "dev-1639659058556-60126760016852";
const WNEAR_TOKEN_ACCOUNT_ID: &str = "wnear.nearlend.testnet";

#[ext_contract(weth_token)]
trait Erc20Interface {
    fn internal_transfer_with_registration(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: Balance,
        memo: Option<String>,
    );
}

#[ext_contract(ext_controller)]
trait ControllerInterface {
    fn borrow_allowed(
        &mut self,
        dtoken_address: AccountId,
        user_address: AccountId,
        amount: u128,
    ) -> bool;
}

#[ext_contract(ext_self)]
trait DtokenInterface {
    fn borrow_callback(amount: Balance);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Dtoken {
    initial_exchange_rate: u128,
    total_supply: u128,
    total_reserve: u128,
    total_borrows: u128,
    balance_of: UnorderedMap<AccountId, BigDecimal>,
    debt_of: UnorderedMap<AccountId, BigDecimal>,
    token: FungibleToken,
    // TODO: Add underlying token address as field
}

impl Default for Dtoken {
    fn default() -> Self {
        Self {
            initial_exchange_rate: 0,
            total_supply: 0,
            total_reserve: 0,
            total_borrows: 0,
            balance_of: UnorderedMap::new(b"s".to_vec()),
            debt_of: UnorderedMap::new(b"s".to_vec()),
            token: FungibleToken::new(b"a".to_vec()),
        }
    }
}

#[near_bindgen]
impl Dtoken {
    pub fn borrow_callback(amount: Balance) {
        // Borrow allowed response
        let is_allowed: bool = match env::promise_result(0) {
            PromiseResult::NotReady => {
                unreachable!()
            }
            PromiseResult::Failed => env::panic(b"Unable to make comparison"),
            PromiseResult::Successful(result) => near_sdk::serde_json::from_slice::<bool>(&result)
                .unwrap()
                .into(),
        };

        assert!(is_allowed, "You are not allowed to borrow");

        let weth_account_id: AccountId =
            AccountId::new_unchecked(WETH_TOKEN_ACCOUNT_ID.to_string());

        weth_token::internal_transfer_with_registration(
            env::current_account_id(),
            env::predecessor_account_id(),
            amount,
            None,
            weth_account_id.clone(), // Attention here!
            NO_DEPOSIT,
            Gas(10_000_000_000_000),
        );
    }

    pub fn supply(amount: Balance) {
        //TODO: supply implementation
    }

    pub fn withdraw(amount: Balance) {
        //TODO: withdraw implementation
    }

    pub fn borrow(amount: Balance) {
        let controller_account_id: AccountId =
            AccountId::new_unchecked(CONTROLLER_ACCOUNT_ID.to_string());

        ext_controller::borrow_allowed(
            env::current_account_id(),
            env::predecessor_account_id(),
            amount,
            controller_account_id.clone(),
            NO_DEPOSIT,
            Gas(10_000_000_000_000),
        )
        .then(ext_self::borrow_callback(
            amount,
            env::current_account_id(),
            NO_DEPOSIT,
            Gas(20_000_000_000_000),
        ));
    }

    pub fn repay() {
        //TODO: repay implementation
    }

    pub fn add_reserve(amount: Balance) {
        //TODO: add_reserve implementation
    }

    pub fn get_exchange_rate() -> u128 {
        //TODO: get exchange rate by formula
        BigDecimal::from_str("1.0").unwrap().to_u128().unwrap()
    }

    pub fn get_total_reserve() -> u128 {
        Dtoken::default().total_reserve
    }

    pub fn get_total_borrows() -> u128 {
        Dtoken::default().total_borrows
    }

    pub fn get_underlying_balance() -> u128 {
        BigDecimal::from_str("1.2").unwrap().to_u128().unwrap()
    }

    pub fn internal_unwrap_balance_of(&self, account_id: &AccountId) -> Balance {
        self.token
            .internal_unwrap_balance_of(&account_id.to_string())
    }

    pub fn internal_deposit(&mut self, account_id: &AccountId, amount: Balance) {
        self.token.internal_deposit(&account_id.to_string(), amount);
    }

    pub fn internal_withdraw(&mut self, account_id: &AccountId, amount: Balance) {
        self.token
            .internal_withdraw(&account_id.to_string(), amount);
    }

    pub fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amount: Balance,
        memo: Option<String>,
    ) {
        self.token.internal_transfer(
            &sender_id.to_string(),
            &receiver_id.to_string(),
            amount,
            memo,
        );
    }

    pub fn internal_register_account(&mut self, account_id: &AccountId) {
        self.token
            .internal_register_account(&account_id.to_string());
    }

    fn mint(&mut self, account_id: &AccountId, amount: Balance) {
        self.internal_deposit(account_id, amount);
    }

    fn burn(&mut self, account_id: &AccountId, amount: Balance) {
        self.internal_withdraw(account_id, amount);
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
