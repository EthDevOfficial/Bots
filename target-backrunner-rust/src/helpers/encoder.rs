use ethabi::ethereum_types::U256;
use ethabi::token::Token::{Address, Array, Uint};
use ethabi::{encode, Bytes, Token};
use std::str::FromStr;
use web3::{
    signing,
    transports::WebSocket,
    types::{TransactionParameters, H160},
    Result, Web3,
};

pub fn tokenize_simple(
    token1: &H160,
    token2: &H160,
    ex1: &H160,
    ex2: &H160,
    swap_fee_sum: U256,
) -> Bytes {
    encode(&[
        Address(token1.clone()),
        Address(token2.clone()),
        Address(ex1.clone()),
        Address(ex2.clone()),
        Uint(swap_fee_sum),
    ])
}

pub fn tokenize_tri(
    token1: &H160,
    token2: &H160,
    token3: &H160,
    ex1: &H160,
    ex2: &H160,
    ex3: &H160,
    swap_fee_sum: U256,
) -> Bytes {
    encode(&[
        Address(token1.clone()),
        Address(token2.clone()),
        Address(token3.clone()),
        Address(ex1.clone()),
        Address(ex2.clone()),
        Address(ex3.clone()),
        Uint(swap_fee_sum),
    ])
}
