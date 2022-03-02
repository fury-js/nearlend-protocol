use near_sdk::{AccountId, Balance, BorshStorageKey, env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap};
#[allow(unused_imports)]
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use percentage::Percentage;

#[allow(unused_imports)]
use general::*;

pub use crate::borrows_supplies::*;
pub use crate::config::*;
pub use crate::oraclehook::*;
pub use crate::prices::*;
pub use crate::repay::*;

#[allow(unused_imports)]
mod config;
mod oraclehook;
mod prices;
pub mod borrows_supplies;
pub mod repay;
mod healthfactor;
mod admin;



#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKeys {
    Markets,
    Supplies,
    SuppliesToken,
    BorrowsToken,
    Prices,
    Config,
    Borrows,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    /// Market name [Underlying asset name] -> Dtoken contract address
    pub markets: LookupMap<AccountId, AccountId>,

    /// User Account ID -> Dtoken address -> Supplies balance
    pub account_supplies: LookupMap<AccountId, UnorderedMap<AccountId, Balance>>,

    /// User Account ID -> Dtoken address -> Borrow balance
    pub account_borrows: LookupMap<AccountId, UnorderedMap<AccountId, Balance>>,

    /// Asset ID -> Price value
    pub prices: LookupMap<AccountId, Price>,

    /// Contract configuration object
    pub config: LazyOption<Config>,

    /// Contract admin account (controller itself by default)
    pub admin: AccountId,

    /// Configuration for pausing/proceeding controller processes (false by default)
    pub is_action_paused: ActionStatus,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic_str("Controller contract should be initialized before usage")
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceJsonList {
    /// Block number
    pub block_height: u64,

    /// Vector of asset prices
    pub price_list: Vec<Price>,
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ActionStatus {
    pub withdraw: bool,
    pub repay: bool,
    pub supply: bool,
    pub liquidate: bool,
    pub borrow: bool,
}

pub trait OraclePriceHandlerHook {
    fn oracle_on_data(&mut self, price_data: PriceJsonList);
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given config. Needs to be called once.
    #[init]
    pub fn new(config: Config) -> Self {
        Self {
            markets: LookupMap::new(StorageKeys::Markets),
            account_supplies: LookupMap::new(StorageKeys::Supplies),
            account_borrows: LookupMap::new(StorageKeys::Borrows),
            prices: LookupMap::new(StorageKeys::Prices),
            config: LazyOption::new(StorageKeys::Config, Some(&config)),
            admin: config.owner_id,
            is_action_paused: ActionStatus {
                withdraw: false,
                repay: false,
                supply: false,
                liquidate: false,
                borrow: false,
            },
        }
    }
}