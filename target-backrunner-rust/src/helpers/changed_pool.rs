use crate::helpers::routes::{make_simple_routes, make_tri_routes};
use crate::types::immutable_state::ImmutableState;
use crate::types::mutable_state::MutableState;
use ethabi::ethereum_types::U256;
use ethereum_abi::{
    DecodedParams, Function, Value,
    Value::{Address, Array},
};
use std::sync::Arc;
use web3::types::H160;

async fn process_token_path(
    token_path: &Value,
    gas_price: U256,
    exchange_index: usize,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    match token_path {
        Array(path, _) => {
            for i in 0..(path.len() - 1) {
                match path[i] {
                    Address(token1) => match path[i + 1] {
                        Address(token2) => {
                            if immutable_state.outer_tokens.iter().any(|token| {
                                token.address.as_bytes() == token1.as_bytes()
                                    || token.address.as_bytes() == token2.as_bytes()
                            }) {
                                if immutable_state.run_simples {
                                    make_simple_routes(
                                        &H160::from_slice(token1.as_bytes()),
                                        &H160::from_slice(token2.as_bytes()),
                                        gas_price,
                                        exchange_index,
                                        immutable_state,
                                        mutable_state,
                                    )
                                    .await;
                                }
                                if immutable_state.run_tris {
                                    make_tri_routes(
                                        &H160::from_slice(token1.as_bytes()),
                                        &H160::from_slice(token2.as_bytes()),
                                        gas_price,
                                        exchange_index,
                                        immutable_state,
                                        mutable_state,
                                    )
                                    .await;
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub async fn process_router_params(
    function_headers: &Function,
    decoded_parameters: DecodedParams,
    gas_price: U256,
    exchange_index: usize,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    if function_headers.name == "swapExactTokensForTokens"
        || function_headers.name == "swapExactTokensForETH"
    {
        let token_path = &decoded_parameters[2].value;
        process_token_path(
            token_path,
            gas_price,
            exchange_index,
            immutable_state,
            mutable_state,
        )
        .await;
    } else if function_headers.name == "swapExactETHForTokens"
        || function_headers.name == "swapETHForExactTokens"
    {
        let token_path = &decoded_parameters[1].value;
        process_token_path(
            token_path,
            gas_price,
            exchange_index,
            immutable_state,
            mutable_state,
        )
        .await;
    } else if function_headers.name == "swapTokensForExactTokens"
        || function_headers.name == "swapTokensForExactETH"
    {
        let token_path = &decoded_parameters[2].value;
        process_token_path(
            token_path,
            gas_price,
            exchange_index,
            immutable_state,
            mutable_state,
        )
        .await;
    } else {
        // println!("missed all swap func names: {}", function_headers.name)
    }
}
