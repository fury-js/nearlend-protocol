use near_sdk::{env, is_promise_success};

use crate::*;

#[near_bindgen]
impl Contract {
    pub fn borrow(&mut self, amount: Balance) -> Promise {
        return underline_token::ft_balance_of(
            env::current_account_id(),
            self.underlying_token.clone(),
            NO_DEPOSIT,
            TGAS * 20u64,
        )
            .then(ext_self::borrow_balance_of_callback(
                amount,
                env::current_account_id().clone(),
                NO_DEPOSIT,
                TGAS * 60u64,
            ));
    }

    #[allow(dead_code)]
    fn borrow_balance_of_callback(&mut self, amount: Balance) -> Promise {
        let promise_success: bool = is_promise_success();

        assert_eq!(
            promise_success,
            true,
            "borrow has failed, not enough balance of Utoken: Account {} deposits {}",
            env::predecessor_account_id(),
            amount
        );

        // let balance_of: Balance = match env::promise_result(0) {
        //     PromiseResult::NotReady => 0,
        //     PromiseResult::Failed => 0,
        //     PromiseResult::Successful(result) => near_sdk::serde_json::from_slice::<u128>(&result)
        //         .unwrap()
        //         .into(),
        // };

        underline_token::ft_transfer_call(
            env::current_account_id(),
            amount,
            Some(format!("token_amount of {} was borrowed", amount)),
            self.underlying_token.clone(),
            NO_DEPOSIT,
            TGAS * 20u64,
        )
    }
}


/*
user wants to borrow some asset giving some collateral in exchange


user_account.borrow(amount) f.e. eth -->  skip the check functionality ->
-> just transfer the requested amount of asssets

*/



#[cfg(test)]
mod tests {
    use near_sdk::{env, testing_env};
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::test_utils::test_env::{alice, bob};

    use super::*;

    fn init() -> (VMContextBuilder, AccountId, Contract) {
        // get VM builer
        let context = VMContextBuilder::new();

        // account for contract
        let _contract_account = alice();

        // init the contract
        let eth_contract = Contract::new(Config{
            initial_exchange_rate: 0,

            /// The account ID of underlying_token
            underlying_token_id: "eth".parse().unwrap(),

            /// The account ID of the contract owner that allows to modify config
            owner_id: alice(),

            /// The account ID of the controller contract
            controller_account_id: alice()
        });


        (context, _contract_account, eth_contract)
    }




    #[test]
    fn test_borrow() {
        let (context, eth_contract_account, mut eth_contract) = init();

        testing_env!(context.build());

        let borrow_amount: u128 = 100_000;

        let bob = bob();

        // FIXME how come it didnt work with increasing total_supply

        eth_contract.supply(borrow_amount);

        assert_eq!(eth_contract.total_supplies, 100_000);

        // eth_contract.borrow(borrow_amount);

    }
}