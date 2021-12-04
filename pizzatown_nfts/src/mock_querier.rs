use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Coin, ContractResult, CustomQuery, from_slice, OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError, SystemResult, to_binary, Uint128, WasmQuery};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};

// based on https://github.com/terra-money/terra-cosmwasm/blob/main/packages/terra-cosmwasm/src/query.rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TerraQueryWrapper {
    // pub route: TerraRoute,
    pub query_data: TerraQuery,
}

// implement custom query
impl CustomQuery for TerraQueryWrapper {}

/// TerraQuery is defines available query datas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerraQuery {
    ContractInfo {
        contract_address: String,
    },
}

/// ContractInfoResponse is data format returned from WasmRequest::ContractInfo query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfoResponse {
    pub address: String,
    pub creator: String,
    pub code_id: u64,
    pub admin: Option<String>,
}

// based on https://github.com/LoTerra/loterra-contract/blob/master/src/mock_querier.rs
pub fn mock_dependencies_custom(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier = WasmMockQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]));
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<TerraQueryWrapper>,
}

#[derive(Clone, Default, Serialize)]
pub struct HackathonNftsBalanceResponse {
    pub balance: Uint128,
}

impl HackathonNftsBalanceResponse {
    pub fn new(balance: Uint128) -> Self {
        HackathonNftsBalanceResponse { balance }
    }
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<TerraQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                if contract_addr == &"terra1utd7hcq00p0grm08uyg23mdm782726y3euk8pq".to_string() {
                    let msg_hackathon_nfts_balance = HackathonNftsBalanceResponse {
                        balance: Uint128::from(1u128)
                    };
                    return SystemResult::Ok(ContractResult::from(to_binary(&msg_hackathon_nfts_balance)));
                }
                panic!("DO NOT ENTER HERE");
            },
            _ => self.base.handle_query(request),
        }
    }

    pub fn new(base: MockQuerier<TerraQueryWrapper>) -> Self {
        WasmMockQuerier { base }
    }
}
