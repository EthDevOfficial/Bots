use crate::helpers::routes::{make_inner_tri_routes, make_outer_tri_routes, make_simple_routes, make_outer_quad_routes, make_inner_quad_routes};
use crate::types::immutable_state::ImmutableState;
use crate::types::mutable_state::MutableState;
use ethereum_abi::{
    DecodedParams, Function, Value,
    Value::{Address, Array, Uint},
};
use futures;
use primitive_types::U256;
use std::sync::Arc;
use web3::types::{H160, U256 as Web3U256};

async fn process_token_path(
    token_path: &Vec<Value>,
    gas_price: Web3U256,
    exchange_index: usize,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    for i in 0..(token_path.len() - 1) {
        match token_path[i] {
            Address(token1) => match token_path[i + 1] {
                Address(token2) => {
                    if immutable_state.outer_tokens.iter().any(|token| {
                        token.address.as_bytes() == token1.as_bytes()
                            || token.address.as_bytes() == token2.as_bytes()
                    }) {
                        let token1_h160 = H160::from_slice(token1.as_bytes());
                        let token2_h160 = H160::from_slice(token2.as_bytes());

                        let immutable_state_clone = immutable_state.clone();
                        let mutable_state_clone = mutable_state.clone();
                        tokio::spawn(async move {
                            make_simple_routes(
                                &token1_h160,
                                &token2_h160,
                                gas_price,
                                exchange_index,
                                &immutable_state_clone,
                                &mutable_state_clone,
                            )
                            .await;
                        });

                        let immutable_state_clone = immutable_state.clone();
                        let mutable_state_clone = mutable_state.clone();
                        tokio::spawn(async move {
                            make_outer_tri_routes(
                                &token1_h160,
                                &token2_h160,
                                gas_price,
                                exchange_index,
                                &immutable_state_clone,
                                &mutable_state_clone,
                            )
                            .await;
                        });

                        let immutable_state_clone = immutable_state.clone();
                        let mutable_state_clone = mutable_state.clone();
                        tokio::spawn(async move {
                            make_outer_quad_routes(
                                &token1_h160,
                                &token2_h160,
                                gas_price,
                                exchange_index,
                                &immutable_state_clone,
                                &mutable_state_clone,
                            )
                            .await;
                        });
                        // let simple_future = make_simple_routes(
                        //     &token1_h160,
                        //     &token2_h160,
                        //     gas_price,
                        //     exchange_index,
                        //     immutable_state,
                        //     mutable_state,
                        // );

                        // let tri_future = make_outer_tri_routes(
                        //     &token1_h160,
                        //     &token2_h160,
                        //     gas_price,
                        //     exchange_index,
                        //     immutable_state,
                        //     mutable_state,
                        // );
                        // futures::join!(tri_future, simple_future);
                    } else {
                        make_inner_tri_routes(
                            &H160::from_slice(token1.as_bytes()),
                            &H160::from_slice(token2.as_bytes()),
                            gas_price,
                            exchange_index,
                            &immutable_state,
                            &mutable_state,
                        )
                        .await;
                        if i + 2 <= token_path.len() - 1 {
                            match token_path[i + 2] {
                                Address(token3) => {
                                    if immutable_state.outer_tokens.iter().any(|token| {
                                        token.address.as_bytes() == token3.as_bytes()
                                    }) {
                                        make_inner_quad_routes(
                                            &H160::from_slice(token1.as_bytes()),
                                            &H160::from_slice(token2.as_bytes()),
                                            &H160::from_slice(token3.as_bytes()),
                                            gas_price,
                                            exchange_index,
                                            &immutable_state,
                                            &mutable_state,
                                        )
                                        .await;
                                    }
                                },
                                _ => ()
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub async fn process_uniswap_router_params(
    function_headers: &Function,
    decoded_parameters: DecodedParams,
    tx_value: Web3U256,
    gas_price: Web3U256,
    exchange_index: usize,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    if function_headers.name == "swapExactTokensForTokens"
        || function_headers.name == "swapExactTokensForETH"
    {
        let token_path = &decoded_parameters[2].value;
        match token_path {
            Array(token_path, _) => {
                if above_trade_threshold(
                    &token_path[0],
                    &token_path[token_path.len() - 1],
                    &decoded_parameters[0].value,
                    &decoded_parameters[1].value,
                    immutable_state,
                ) {
                    process_token_path(
                        token_path,
                        gas_price,
                        exchange_index,
                        immutable_state,
                        mutable_state,
                    )
                    .await;
                }
            }
            _ => (),
        }
    } else if function_headers.name == "swapExactETHForTokens"
        || function_headers.name == "swapETHForExactTokens"
    {
        let token_path = &decoded_parameters[1].value;
        match token_path {
            Array(token_path, _) => {
                if above_trade_threshold_web3(
                    &token_path[0],
                    &token_path[token_path.len() - 1],
                    &tx_value,
                    &decoded_parameters[0].value,
                    immutable_state,
                ) {
                    process_token_path(
                        token_path,
                        gas_price,
                        exchange_index,
                        immutable_state,
                        mutable_state,
                    )
                    .await;
                }
            }
            _ => (),
        }
    } else if function_headers.name == "swapTokensForExactTokens"
        || function_headers.name == "swapTokensForExactETH"
    {
        let token_path = &decoded_parameters[2].value;
        match token_path {
            Array(token_path, _) => {
                if above_trade_threshold(
                    &token_path[0],
                    &token_path[token_path.len() - 1],
                    &decoded_parameters[1].value,
                    &decoded_parameters[0].value,
                    immutable_state,
                ) {
                    process_token_path(
                        token_path,
                        gas_price,
                        exchange_index,
                        immutable_state,
                        mutable_state,
                    )
                    .await;
                }
            }
            _ => (),
        }
    } else {
        // println!("missed all swap func names: {}", function_headers.name)
    }
}

pub fn above_trade_threshold(
    in_token: &Value,
    out_token: &Value,
    in_amount: &Value,
    out_amount: &Value,
    immutable_state: &Arc<ImmutableState>,
) -> bool {
    above_one_trade_threshold(in_token, in_amount, immutable_state)
        || above_one_trade_threshold(out_token, out_amount, immutable_state)
}

pub fn above_trade_threshold_web3(
    in_token: &Value,
    out_token: &Value,
    in_amount: &Web3U256,
    out_amount: &Value,
    immutable_state: &Arc<ImmutableState>,
) -> bool {
    above_one_trade_threshold(out_token, out_amount, immutable_state)
        || above_one_trade_threshold(
            in_token,
            &Value::Uint(U256::from_dec_str(&in_amount.to_string()).unwrap(), 0),
            immutable_state,
        )
}

fn above_one_trade_threshold(
    token: &Value,
    amount: &Value,
    immutable_state: &Arc<ImmutableState>,
) -> bool {
    match token {
        Address(token) => {
            let token_index = immutable_state
                .tokens
                .iter()
                .position(|other_token| other_token.address.as_bytes() == token.as_bytes());
            match token_index {
                Some(token_index) => {
                    let token = &immutable_state.tokens[token_index];
                    match amount {
                        Uint(amount, _) => token.above_trade_threshold(amount),
                        _ => false,
                    }
                }
                None => false,
            }
        }
        _ => false,
    }
}

// =============================== //

// pub async fn process_firebird_router_params(
//     function_headers: &Function,
//     decoded_parameters: DecodedParams,
//     tx_value: Web3U256,
//     gas_price: Web3U256,
//     immutable_state: &Arc<ImmutableState>,
//     mutable_state: &Arc<MutableState>,
// ) {
//     if function_headers.name == "multihopBatchSwapExactIn" {
//         println!("{:?} \n", function_headers);
//         println!("{:?}", decoded_parameters);
//         // let token_path = &decoded_parameters[0].value;
//         // match token_path {
//         //     Array(token_path, _) => {
//         //         if above_trade_threshold(
//         //             &token_path[0],
//         //             &token_path[token_path.len() - 1],
//         //             &decoded_parameters[0].value,
//         //             &decoded_parameters[1].value,
//         //             immutable_state,
//         //         ) {
//         //             process_token_path_firebird(
//         //                 token_path,
//         //                 gas_price,
//         //                 immutable_state,
//         //                 mutable_state,
//         //             )
//         //             .await;
//         //         }
//         //     }
//         //     _ => (),
//         // }
//     } else if function_headers.name == "multihopBatchSwapExactOut" {
//         println!("{:?} \n", function_headers);
//         println!("{:?}", decoded_parameters);
//     }
// }

// async fn process_token_path_firebird(
//     token_path: &Vec<Value>,
//     gas_price: Web3U256,
//     immutable_state: &Arc<ImmutableState>,
//     mutable_state: &Arc<MutableState>,
// ) {
//     for i in 0..(token_path.len() - 1) {
//         match token_path[i] {
//             Address(token1) => match token_path[i + 1] {
//                 Address(token2) => {
//                     if immutable_state.outer_tokens.iter().any(|token| {
//                         token.address.as_bytes() == token1.as_bytes()
//                             || token.address.as_bytes() == token2.as_bytes()
//                     }) {
//                         let token1_h160 = H160::from_slice(token1.as_bytes());
//                         let token2_h160 = H160::from_slice(token2.as_bytes());

//                         let immutable_state_clone = immutable_state.clone();
//                         let mutable_state_clone = mutable_state.clone();
//                         tokio::spawn(async move {
//                             make_simple_routes_firebird(
//                                 &token1_h160,
//                                 &token2_h160,
//                                 gas_price,
//                                 &immutable_state_clone,
//                                 &mutable_state_clone,
//                             )
//                             .await;
//                         });

//                         let immutable_state_clone = immutable_state.clone();
//                         let mutable_state_clone = mutable_state.clone();
//                         tokio::spawn(async move {
//                             make_outer_tri_routes_firebird(
//                                 &token1_h160,
//                                 &token2_h160,
//                                 gas_price,
//                                 &immutable_state_clone,
//                                 &mutable_state_clone,
//                             )
//                             .await;
//                         });
//                     } else {
//                         make_inner_tri_routes_firebird(
//                             &H160::from_slice(token1.as_bytes()),
//                             &H160::from_slice(token2.as_bytes()),
//                             gas_price,
//                             &immutable_state,
//                             &mutable_state,
//                         )
//                         .await;
//                     }
//                 }
//                 _ => {}
//             },
//             _ => {}
//         }
//     }
// }
