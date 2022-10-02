export interface Token {
    name: string | null;
    symbol: string | null;
    total_supply: string | null;
    address: string | null;
    decimals: number | null;
}

export interface Denom {
    native: string | null;
    cw20: string | null;
}

export enum Chain {
    JUNO = "juno",
    OSMOSIS = "osmosis",
}

export interface Asset {
    token: {
        denom: string;
        amount: string;
        native_name: string;
    };
    weight: string;
}

export interface PoolParams {
    exit_fee: string;
    swap_fee: string;
}

export interface TotalShares {
    denom: string;
    amount: string;
    native_name: null;
}

export interface Pool {
    chain?: Chain;
    pool_address: string | null;
    lp_token_address: string | null;
    lp_token_supply: string | null;
    token1: Token | null;
    token1_denom: Denom | null;
    token1_reserve: string | null;
    token2: Token | null;
    token2_denom: Denom | null;
    token2_reserve: string | null;

    //Osomsis
    future_pool_governor?: string;
    id?: string;
    pool_assets?: Asset[];
    pool_params?: PoolParams;
    total_shares?: TotalShares;
    total_weight?: string;
}