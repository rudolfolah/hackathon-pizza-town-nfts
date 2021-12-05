#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TokenInfoResponse,
};

const CONTRACT_NAME: &str = "pizzatown:hackathon_nfts";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // println!("{:?}", _info.funds);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    return Err(ContractError::Unauthorized {});
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { .. } => to_binary(&query_balance(deps)?),
        QueryMsg::TokenInfo { .. } => to_binary(&query_token_info(deps)?),
    }
}

fn query_balance(_deps: Deps,) -> StdResult<BalanceResponse> {
    return Ok(BalanceResponse {
        balance: Uint128::from(1u128),
    });
}

fn query_token_info(_deps: Deps) -> StdResult<TokenInfoResponse> {
    return Ok(TokenInfoResponse {
        url: "https://www.terra.money/".to_string(),
    });
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{
        coins, from_binary, Addr, Api, BlockInfo, ContractInfo, OwnedDeps, Querier, Storage,
    };
    // use crate::utils::print_type_of;

    fn mock_env(timestamp_nanos: u64) -> Env {
        let nanos = 1_571_797_419_879_305_533 + timestamp_nanos;
        Env {
            block: BlockInfo {
                height: 12_345,
                time: Timestamp::from_nanos(nanos),
                chain_id: "cosmos-testnet-14002".to_string(),
            },
            contract: ContractInfo {
                address: Addr::unchecked(MOCK_CONTRACT_ADDR),
            },
        }
    }

    /// checks the `address` to ensure it has the correct `expected` balance
    fn assert_balance_is<S: Storage, A: Api, Q: Querier>(
        deps: &OwnedDeps<S, A, Q>,
        address: &str,
        expected: u128,
    ) {
        let res = query(
            deps.as_ref(),
            mock_env(0),
            QueryMsg::Balance {
                owner: address.to_string(),
                token_id: "any_token_id".to_string(),
            },
        )
        .unwrap();
        let balance: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(
            expected,
            balance.balance.u128(),
            "address '{}' should have {} tokens",
            address,
            expected
        );
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let res = instantiate(
            deps.as_mut(),
            mock_env(0),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {},
        )
        .unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(
            deps.as_ref(),
            mock_env(1),
            QueryMsg::TokenInfo { token_id: "example_token_id".to_string() }
        ).unwrap();
        let token_info: TokenInfoResponse = from_binary(&res).unwrap();
        assert_eq!("https://www.terra.money/", token_info.url);

        assert_balance_is(&deps, "any_address", 1u128);
        assert_balance_is(&deps, "another_address", 1u128);
    }
}
