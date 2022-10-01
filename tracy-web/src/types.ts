export interface Token {
    name: String | null;
    symbol: String | null;
    total_supply: String | null;
    address: String | null;
    decimals: number | null;
}

export interface Denom {
    native: String | null;
    cw20: String | null;
}

export enum Chain {
    JUNO = "juno",
    OSMOSIS = "osmosis",
}

export interface Asset {
    token: {
        denom: String;
        amount: String;
        native_name: String;
    };
    weight: String;
}

export interface PoolParams {
    exit_fee: String;
    swap_fee: String;
}

export interface TotalShares {
    denom: String;
    amount: String;
    native_name: null;
}

export interface Pool {
    chain?: Chain;
    pool_address: String | null;
    lp_token_address: String | null;
    lp_token_supply: String | null;
    token1: Token | null;
    token1_denom: Denom | null;
    token1_reserve: String | null;
    token2: Token | null;
    token2_denom: Denom | null;
    token2_reserve: String | null;

    //Osomsis
    future_pool_governor?: String;
    id?: String;
    pool_assets?: Asset[];
    pool_params?: PoolParams;
    total_shares?: TotalShares;
    total_weight?: String;
}