export interface Coin {
  denom: string;
  amount: string;
}

export interface NftPizza {
  id: string;
  attr1: number;
  attr2: number;
  attr3: number;
  attr4: number;
}

export interface NftPie {
  id: string;
  quantity: number;
}

export interface NftHackathon {
  id: string;
}

export interface BalanceResponse {
  balance: string;
}

export interface PizzaTownInfoResponse {
  total_supply_pizzas: number;
  total_supply_pies: number;
}

export interface HackathonInfoResponse {
  total_supply: number;
}

export interface PizzaTownInventoryResponse {
  // from the smart contract
  address: string;
  pizzas: NftPizza[];
  pies: NftPie[];
}

export interface HackathonInventoryResponse {
  // from the smart contract
  address: string;
  nfts: NftHackathon;
}

export interface TokenInfoResponse {
  name: string;
  symbol: string;
  decimals: number;
  total_supply: string;
}

export interface TransferRequest {
  recipient: string;
  amount: string;
}

export interface BurnRequest {
  amount: string;
}

export interface SendRequest {
  contract: string;
  amount: string;
  msg: any;
}

export interface MintRequest {
  amount: string;
}

export interface MintPizzaRequest {}

export interface MintPieRequest {
  pizza_a_id: string;
  pizza_b_id: string;
}
