use cw_storage_plus::Map; // See: https://github.com/CosmWasm/cw-plus/tree/main/packages/storage-plus
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

// consider using an integer vector to allow future airdrops
// simplifying lets us check just for the existence of the key
pub const AIRDROP_CLAIMS: Map<&[u8], bool> = Map::new("airdrop_claims");

// key is a combination of the owner's address and the object's generated id
pub const NFT_PIZZAS: Map<(&[u8], &[u8]), NftPizzaData> = Map::new("all_pizzas");
// key is a combination of the owner's address and the object's generated id
pub const NFT_PIES: Map<(&[u8], &[u8]), NftPieData> = Map::new("all_pies");
