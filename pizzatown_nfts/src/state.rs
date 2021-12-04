use cosmwasm_std::{Coin, Uint128};
use cw_storage_plus::{Item, Map}; // See: https://github.com/CosmWasm/cw-plus/tree/main/packages/storage-plus
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct NftPizzaData {
    pub background: u8,
    pub pizza: u8,
    pub topping1: u8,
    pub topping2: u8,
    pub topping3: u8,
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct NftPieData {
    pub pie: u8,
    pub id: String,
}

// key is a combination of the owner's address and the object's generated id
pub const NFT_PIZZAS: Map<(&[u8], &[u8]), DogData> = Map::new("all_pizzas");
// key is a combination of the owner's address and the object's generated id
pub const NFT_PIES: Map<(&[u8], &[u8]), AccessoryData> = Map::new("all_pies");
