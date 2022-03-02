use crate::*;

impl Contract {
    pub fn get_prices_for_assets(&self, assets: Vec<AccountId>) -> LookupMap<AccountId, Balance> {
        let mut result = LookupMap::new(b"t".to_vec());
        for asset in assets {
            if self.prices.contains_key(&asset) {
                let price = self.get_price(asset).unwrap();
                result.insert(&price.asset_id, &price.value);
            }
        }
        return result;
    }

    pub fn get_price(&self, asset_id: AccountId) -> Option<Price> {
        return self.prices.get(&asset_id);
    }

    pub fn upsert_price(&mut self, price: &Price) {
        // Update & insert operation
        self.prices.insert(&price.asset_id, &price);
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use near_sdk::test_utils::test_env::{alice, bob};

    use super::*;

    pub fn init_test_env() -> (Contract, AccountId) {
        let (owner_account, oracle_account) = (alice(), bob());

        let eth_contract = Contract::new(Config {
            owner_id: owner_account,
            oracle_account_id: oracle_account,
        });

        let token_address: AccountId = "near".parse().unwrap();

        return (eth_contract, token_address);
    }

    #[test]
    fn test_add_get_price() {
        let (mut near_contract, token_address) = init_test_env();

        let price = Price {
            // adding price of Near
            asset_id: token_address.clone(),
            value: 20,
            volatility: 100,
        };

        near_contract.upsert_price(&price);

        let gotten_price = near_contract.get_price(token_address).unwrap();

        assert_matches!(&gotten_price, _price, "Get price format check has been failed");

        assert_eq!(&gotten_price.value, &price.value, "Get price values check has been failed");
        assert_eq!(&gotten_price.volatility, &price.volatility, "Get price volatility check has been failed");
        assert_eq!(&gotten_price.asset_id, &price.asset_id, "Get price asset_id check has been failed");
    }
}
