#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, Timestamp, Uint128, WasmQuery};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, InventoryResponse, QueryMsg,
};
use crate::state::{NFT_PIES, NFT_PIZZAS, NftPizzaData, NftPieData, AIRDROP_CLAIMS};
use crate::utils::{generate_id, rand_int_between};

const CONTRACT_NAME: &str = "pizzatown:pizzatown_nfts";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const OWNER_ADDR: &str = "terra102qww57le570w0p44pw8mm6arlsekrhl7df0nk";

// Testnet address for hackathon nft contract
const HACKATHON_NFTS_CONTRACT_ADDR: &str = "terra1utd7hcq00p0grm08uyg23mdm782726y3euk8pq";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", OWNER_ADDR))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MintPizza {} => try_mint_pizza(deps, env, info),
        ExecuteMsg::MintPie { pizza_a_id, pizza_b_id } =>
            try_mint_pie(deps, env, info, pizza_a_id, pizza_b_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Inventory { address } => to_binary(&query_inventory(deps, address)?),
    }
}

fn create_random_pizza(seed_timestamp: Timestamp, id: &String) -> NftPizzaData {
    return NftPizzaData {
        id: id.clone(),
        background: rand_int_between(seed_timestamp.plus_nanos(10u64), 1, 1 + 9),
        pizza: rand_int_between(seed_timestamp.plus_nanos(11u64), 1, 1 + 7),
        topping1: rand_int_between(seed_timestamp.plus_nanos(12u64), 1, 1 + 8),
        topping2: rand_int_between(seed_timestamp.plus_nanos(13u64), 1, 1 + 8),
        topping3: rand_int_between(seed_timestamp.plus_nanos(14u64), 1, 1 + 8),
    };
}

fn create_random_pie(seed_timestamp: Timestamp, id: &String) -> NftPieData {
    return NftPieData {
        id: id.clone(),
        pie: rand_int_between(seed_timestamp.plus_nanos(5u64), 1, 1 + 10),
    };
}

fn is_airdrop_eligible(deps: Deps, owner: String) -> bool {
    let msg = hackathon_nfts::msg::QueryMsg::Balance { owner, token_id: "example".to_string() };
    let wasm = WasmQuery::Smart {
        contract_addr: HACKATHON_NFTS_CONTRACT_ADDR.to_string(),
        msg: to_binary(&msg).unwrap(),
    };
    let res: StdResult<hackathon_nfts::msg::BalanceResponse> = deps.querier.query(&wasm.into());
    match res {
        Ok(res) => res.balance.u128() == 1u128,
        _ => false,
    }
}

fn try_mint_pizza(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let id = generate_id(Timestamp::from_nanos(env.block.time.nanos() as u64));
    println!("minting pizza: #{}", id);
    let nft_data = create_random_pizza(
        Timestamp::from_nanos(env.block.time.nanos() as u64),
        &id,
    );
    let key = (info.sender.as_bytes(), id.as_bytes());
    println!("key is {:?}", key);
    NFT_PIZZAS.save(deps.storage, key, &nft_data)?;

    if !AIRDROP_CLAIMS.has(deps.storage, info.sender.as_bytes()) && is_airdrop_eligible(deps.as_ref(), info.sender.to_string()) {
        let airdrop_nft_id = generate_id(Timestamp::from_nanos(env.block.time.nanos() as u64 + 9999u64));
        let airdrop_nft_data = create_random_pizza(
            Timestamp::from_nanos(env.block.time.nanos() as u64 + 10000u64),
            &airdrop_nft_id,
        );
        let airdrop_key = (info.sender.as_bytes(), airdrop_nft_id.as_bytes());
        println!("airdrop key is {:?}", airdrop_key);
        NFT_PIZZAS.save(deps.storage, airdrop_key, &airdrop_nft_data)?;
        AIRDROP_CLAIMS.save(deps.storage, info.sender.as_bytes(), &true);
    }

    Ok(Response::new().add_attribute("method", "try_mint_pizza"))
}

fn try_mint_pie(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    pizza_a_id: String,
    pizza_b_id: String,
) -> Result<Response, ContractError> {
    let pizza_a_key = (info.sender.as_bytes(), pizza_a_id.as_bytes());
    let pizza_b_key = (info.sender.as_bytes(), pizza_b_id.as_bytes());

    if !NFT_PIZZAS.has(deps.storage, pizza_a_key) || !NFT_PIZZAS.has(deps.storage, pizza_b_key) {
        return Err(ContractError::Unauthorized {});
    }

    let id = generate_id(Timestamp::from_nanos(env.block.time.nanos() as u64));
    println!("minting pie: #{}", id);
    let nft_data = create_random_pie(
        Timestamp::from_nanos(env.block.time.nanos() as u64),
        &id,
    );
    let key = (info.sender.as_bytes(), id.as_bytes());
    println!("key is {:?}", key);
    NFT_PIES.save(deps.storage, key, &nft_data)?;

    NFT_PIZZAS.remove(deps.storage, pizza_a_key);
    NFT_PIZZAS.remove(deps.storage, pizza_b_key);
    Ok(Response::new().add_attribute("method", "try_mint_pie"))
}
//
// fn try_spin_the_wheel(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
// ) -> Result<Response, ContractError> {
//     let denom = PRICE_DENOM.to_string();
//     let price = Uint128::from(SPIN_THE_WHEEL_PRICE_UUSD);
//     let fund = info
//         .funds
//         .iter()
//         .find(|&x| x.denom == denom && x.amount == price);
//     if fund.is_none() {
//         return Err(ContractError::InsufficientFunds {});
//     }
//     let choice = rand_int_between(env.block.time, 1u8, 100u8);
//     let prize_id = generate_id(env.block.time);
//     let mut prize = "none";
//     println!("WHEEL SPUN: {}", choice);
//     if choice <= 10u8 {
//         prize = "dog";
//         let class_choice = rand_int_between(env.block.time, 1u8, 100u8);
//         let mut class = 0;
//         if class_choice <= 20 {
//             class = 0;
//         } else if class_choice <= 50 {
//             class = 1;
//         } else if class_choice <= 77 {
//             class = 2;
//         } else if class_choice <= 97 {
//             class = 3;
//         } else if class_choice == 98 {
//             class = 4;
//         } else if class_choice == 99 {
//             class = 5;
//         } else if class_choice == 100 {
//             class = 6;
//         }
//         let dog_name = format!("Dog #{}", prize_id.clone());
//         let dog_data = create_random_dog(env.block.time, &prize_id, &dog_name, class);
//         DOGS.save(
//             deps.storage,
//             (info.sender.as_bytes(), prize_id.as_bytes()),
//             &dog_data,
//         )?;
//     } else {
//         prize = "accessory";
//         let accessory_name_index = usize::from(rand_int_between(env.block.time, 1u8, 3u8) - 1u8);
//         let accessory_name = ACCESSORY_NAMES[accessory_name_index];
//         let accessory_data = AccessoryData {
//             name: accessory_name.to_string(),
//             id: prize_id.clone(),
//         };
//         ACCESSORIES.save(
//             deps.storage,
//             (info.sender.as_bytes(), prize_id.as_bytes()),
//             &accessory_data,
//         )?;
//     }
//     println!("PRIZE IS {}", prize);
//
//     Ok(Response::new()
//         .add_attribute("method", "try_spin_the_wheel")
//         .add_attribute("prize", prize)
//         .add_attribute("prize_id", prize_id))
// }

fn query_inventory(deps: Deps, address: String) -> StdResult<InventoryResponse> {
    let pizzas: Vec<_> = NFT_PIZZAS
        .prefix(address.as_bytes())
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let mut pizzas_response = Vec::new();
    for pizza in pizzas {
        let (_key, pizza_data) = pizza.unwrap();
        pizzas_response.push(pizza_data);
    }

    let pies: Vec<_> = NFT_PIES
        .prefix(address.as_bytes())
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let mut pies_response = Vec::new();
    for pie in pies {
        let (_key, pie_data) = pie.unwrap();
        pies_response.push(pie_data);
    }

    return Ok(InventoryResponse {
        address,
        pizzas: pizzas_response,
        pies: pies_response,
    });
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_querier::mock_dependencies_custom;
    use cosmwasm_std::testing::{mock_dependencies, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{
        coins, from_binary, Addr, BlockInfo, ContractInfo,
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
    }

    #[test]
    // fn spin_the_wheel() {
    //     let mut deps = mock_dependencies(&[]);
    //     let res = instantiate(
    //         deps.as_mut(),
    //         mock_env(0),
    //         mock_info("creator_address", &coins(1000, "earth")),
    //         InstantiateMsg {},
    //     )
    //     .unwrap();
    //     assert_eq!(0, res.messages.len());
    //
    //     let res = execute(
    //         deps.as_mut(),
    //         mock_env(1),
    //         mock_info(
    //             "player_address",
    //             &[Coin {
    //                 amount: Uint128::from(999_999u128),
    //                 denom: "uusd".to_string(),
    //             }],
    //         ),
    //         ExecuteMsg::SpinTheWheel {},
    //     );
    //     match res {
    //         Err(ContractError::InsufficientFunds {}) => {}
    //         _ => panic!("must return insufficient funds error"),
    //     }
    //
    //     // spin the wheel
    //     let res = execute(
    //         deps.as_mut(),
    //         mock_env(2),
    //         mock_info(
    //             "player_address",
    //             &[Coin {
    //                 amount: Uint128::from(1_000_000u128),
    //                 denom: "uusd".to_string(),
    //             }],
    //         ),
    //         ExecuteMsg::SpinTheWheel {},
    //     )
    //     .unwrap();
    //     assert_eq!(0, res.messages.len());
    //
    //     // check the inventory of the player, they should receive an item
    //     let res = query(
    //         deps.as_ref(),
    //         mock_env(3),
    //         QueryMsg::Inventory {
    //             address: "player_address".to_string(),
    //         },
    //     )
    //     .unwrap();
    //     let inventory: InventoryResponse = from_binary(&res).unwrap();
    //     println!("SPIN THE WHEEL");
    //     println!("{:?}", inventory);
    //     if inventory.dogs.len() == 0 {
    //         assert_eq!(1, inventory.accessories.len());
    //     } else if inventory.accessories.len() == 0 {
    //         assert_eq!(1, inventory.dogs.len());
    //     }
    //
    //     // check the inventory of contract owner address, should receive nothing
    //     let res = query(
    //         deps.as_ref(),
    //         mock_env(4),
    //         QueryMsg::Inventory {
    //             address: OWNER_ADDR.to_string(),
    //         },
    //     )
    //     .unwrap();
    //     let inventory: InventoryResponse = from_binary(&res).unwrap();
    //     println!("SPIN THE WHEEL");
    //     println!("{:?}", inventory);
    //     assert_eq!(0, inventory.dogs.len());
    //     assert_eq!(0, inventory.accessories.len());
    // }

    #[test]
    fn test_is_airdrop_eligible() {
        let mut deps = mock_dependencies_custom(&[]);
        assert_eq!(is_airdrop_eligible(deps.as_ref(), "example_address".to_string()), true);
    }

    #[test]
    fn mint_pizza_and_pie() {
        let mut deps = mock_dependencies_custom(&[]);
        let res = instantiate(
            deps.as_mut(),
            mock_env(0),
            mock_info("creator_address", &coins(1000, "earth")),
            InstantiateMsg {},
        ).unwrap();
        assert_eq!(0, res.messages.len());

        // regular mint before aidrop is claim gives two pizzas
        let res = execute(
            deps.as_mut(),
            mock_env(1),
            mock_info(
                "player_address",
                &[Coin {
                    amount: Uint128::from(1_000_000u128),
                    denom: "uusd".to_string(),
                }],
            ),
            ExecuteMsg::MintPizza {},
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(
            deps.as_ref(),
            mock_env(2),
            QueryMsg::Inventory {
                address: "player_address".to_string(),
            },
        ).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        println!("{:?}", inventory);
        assert_eq!(2, inventory.pizzas.len(), "there should be two pizzas from mint + airdrop");
        assert_eq!(0, inventory.pies.len(), "there should be no pies, no pizzas have been combined yet");

        // regular mint after airdrop is claimed gives one pizza
        let res = execute(
            deps.as_mut(),
            mock_env(3),
            mock_info(
                "player_address",
                &[Coin {
                    amount: Uint128::from(1_000_000u128),
                    denom: "uusd".to_string(),
                }],
            ),
            ExecuteMsg::MintPizza {},
        ).unwrap();
        assert_eq!(0, res.messages.len());
        let res = query(
            deps.as_ref(),
            mock_env(4),
            QueryMsg::Inventory {
                address: "player_address".to_string(),
            },
        ).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        println!("{:?}", inventory);
        assert_eq!(3, inventory.pizzas.len());
        assert_eq!(0, inventory.pies.len());

        let pizza_a_id = String::from(&inventory.pizzas.get(0).unwrap().id);
        let pizza_b_id = String::from(&inventory.pizzas.get(1).unwrap().id);

        // combine two pizzas and mint a pie
        let res = execute(
            deps.as_mut(),
            mock_env(5),
            mock_info(
                "player_address",
                &[Coin {
                    amount: Uint128::from(1_000_000u128),
                    denom: "uusd".to_string(),
                }],
            ),
            ExecuteMsg::MintPie { pizza_a_id, pizza_b_id },
        ).unwrap();
        assert_eq!(0, res.messages.len());
        let res = query(
            deps.as_ref(),
            mock_env(6),
            QueryMsg::Inventory {
                address: "player_address".to_string(),
            },
        ).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        println!("{:?}", inventory);
        assert_eq!(1, inventory.pizzas.len(), "after pizzas are combined, they are removed from inventory");
        assert_eq!(1, inventory.pies.len(), "combined pizzas result in a new pie in the inventory");
    }
}
