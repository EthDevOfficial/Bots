use crate::helpers::encoder::{tokenize_simple, tokenize_tri};
use crate::helpers::web3::{make_simple_tx, make_tri_tx, send_transaction};
use crate::types::{
    exchange::Exchange, immutable_state::ImmutableState, mutable_state::MutableState, token::Token,
};
use ethabi::ethereum_types::U256;
use ethabi::{Bytes, Token as Tokenized};
use futures::future::join_all;
use std::cmp::min;
use std::sync::Arc;
use std::vec::Vec;
use web3::types::{TransactionParameters, H160};

pub async fn make_simple_routes_firebird(
    token1: &H160,
    token2: &H160,
    gas_price: U256,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    let mut routes: Vec<Bytes> = Vec::new();

    // Find the correct pool ordering and record whether the route should be reversed
    let (token1, token2, should_reverse) = favor_outer_token(token1, token2, &immutable_state);

    // Make the routes
    // TODO: seperate into 4 blocks
    if should_reverse {
        for i in 0..immutable_state.exchanges.len() {
            for j in i + 1..immutable_state.exchanges.len() {
                let away_exchange = &immutable_state.exchanges[i];
                let return_exchange = &immutable_state.exchanges[j];
                // (T1, T2, E2) -> (T2, T1, E1)
                routes.push(tokenize_simple(
                    token1,
                    token2,
                    &away_exchange.router,
                    &return_exchange.router,
                    U256::from(away_exchange.swap_fee + return_exchange.swap_fee),
                ));
            }
        }
    } else {
        for i in 0..immutable_state.exchanges.len() {
            for j in i + 1..immutable_state.exchanges.len() {
                let away_exchange = &immutable_state.exchanges[j];
                let return_exchange = &immutable_state.exchanges[i];
                // (T1, T2, E1) -> (T2, T1. E2)
                routes.push(tokenize_simple(
                    token1,
                    token2,
                    &away_exchange.router,
                    &return_exchange.router,
                    U256::from(away_exchange.swap_fee + return_exchange.swap_fee),
                ));
            }
        }
    };

    send_routes(
        routes,
        gas_price,
        make_simple_tx,
        immutable_state,
        mutable_state,
    )
    .await;
}

pub async fn make_simple_routes(
    token1: &H160,
    token2: &H160,
    gas_price: U256,
    exchange_index: usize,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    let mut routes: Vec<Bytes> = Vec::new();

    // Find the correct pool ordering and record whether the route should be reversed
    let (token1, token2, should_reverse) = favor_outer_token(token1, token2, &immutable_state);

    // // Return the relevant exchanges to loop through
    // let (exchanges, not_primary) =
    //     if immutable_state
    //         .primary_exchanges
    //         .iter()
    //         .any(|prim_exchange| {
    //             immutable_state.exchanges[exchange_index].router == prim_exchange.router
    //         })
    //     {
    //         (&immutable_state.exchanges, false)
    //     } else {
    //         (&immutable_state.primary_exchanges, true)
    //     };

    // Make the routes
    // TODO: seperate into 4 blocks
    if should_reverse {
        for i in 0..immutable_state.exchanges.len() {
            if i != exchange_index {
                let away_exchange = &immutable_state.exchanges[i];
                let return_exchange = &immutable_state.exchanges[exchange_index];
                // (T1, T2, E2) -> (T2, T1, E1)
                routes.push(tokenize_simple(
                    token1,
                    token2,
                    &away_exchange.router,
                    &return_exchange.router,
                    U256::from(away_exchange.swap_fee + return_exchange.swap_fee),
                ));
            }
        }
    } else {
        for i in 0..immutable_state.exchanges.len() {
            if i != exchange_index {
                let away_exchange = &immutable_state.exchanges[exchange_index];
                let return_exchange = &immutable_state.exchanges[i];
                // (T1, T2, E1) -> (T2, T1. E2)
                routes.push(tokenize_simple(
                    token1,
                    token2,
                    &away_exchange.router,
                    &return_exchange.router,
                    U256::from(away_exchange.swap_fee + return_exchange.swap_fee),
                ));
            }
        }
    };

    send_routes(
        routes,
        gas_price,
        make_simple_tx,
        immutable_state,
        mutable_state,
    )
    .await;
}

pub async fn make_outer_tri_routes_firebird(
    token1: &H160,
    token2: &H160,
    gas_price: U256,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    // Find the correct pool ordering and record whether the route should be reversed
    let (token1, token2, should_reverse) = favor_outer_token(token1, token2, &immutable_state);

    // Token1 is preferred
    let mut routes: Vec<Bytes> = Vec::new();
    for other_exchange in immutable_state.exchanges.iter() {
        for inner_token in immutable_state.inner_tokens.iter() {
            if token1.ne(&inner_token.address) && token2.ne(&inner_token.address) {
                if should_reverse {
                    // (T1, InnerT, E3) -> (InnerT, T2, E2) -> (T2, T1, E1)
                    routes.push(tokenize_tri(
                        token1,
                        &inner_token.address,
                        token2,
                        &other_exchange.router,
                        &other_exchange.router,
                        &other_exchange.router,
                        U256::from(other_exchange.swap_fee * 3),
                    ));
                } else {
                    // (T1, T2, E1) -> (T2, InnerT, E2) -> (InnerT, T1, E3)
                    routes.push(tokenize_tri(
                        token1,
                        token2,
                        &inner_token.address,
                        &other_exchange.router,
                        &other_exchange.router,
                        &other_exchange.router,
                        U256::from(other_exchange.swap_fee * 3),
                    ));
                }
            }
        }
    }

    send_routes(
        routes,
        gas_price,
        make_tri_tx,
        immutable_state,
        mutable_state,
    )
    .await;
}

pub async fn make_outer_tri_routes(
    token1: &H160,
    token2: &H160,
    gas_price: U256,
    exchange_index: usize,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    // Find the correct pool ordering and record whether the route should be reversed
    let (token1, token2, should_reverse) = favor_outer_token(token1, token2, &immutable_state);

    // Token1 is preferred
    let mut routes: Vec<Bytes> = Vec::new();
    for other_exchange in immutable_state.exchanges.iter() {
        for inner_token in immutable_state.inner_tokens.iter() {
            if token1.ne(&inner_token.address) && token2.ne(&inner_token.address) {
                if should_reverse {
                    // (T1, InnerT, E3) -> (InnerT, T2, E2) -> (T2, T1, E1)
                    routes.push(tokenize_tri(
                        token1,
                        &inner_token.address,
                        token2,
                        &other_exchange.router,
                        &other_exchange.router,
                        &immutable_state.exchanges[exchange_index].router,
                        U256::from(
                            other_exchange.swap_fee
                                + other_exchange.swap_fee
                                + immutable_state.exchanges[exchange_index].swap_fee,
                        ),
                    ));
                } else {
                    // (T1, T2, E1) -> (T2, InnerT, E2) -> (InnerT, T1, E3)
                    routes.push(tokenize_tri(
                        token1,
                        token2,
                        &inner_token.address,
                        &immutable_state.exchanges[exchange_index].router,
                        &other_exchange.router,
                        &other_exchange.router,
                        U256::from(
                            other_exchange.swap_fee
                                + other_exchange.swap_fee
                                + immutable_state.exchanges[exchange_index].swap_fee,
                        ),
                    ));
                }
            }
        }
    }

    send_routes(
        routes,
        gas_price,
        make_tri_tx,
        immutable_state,
        mutable_state,
    )
    .await;
}

pub async fn make_inner_tri_routes_firebird(
    token1: &H160,
    token2: &H160,
    gas_price: U256,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    // In original ordering (no outer token)
    let mut routes: Vec<Bytes> = Vec::new();
    for other_exchange in immutable_state.exchanges.iter() {
        for outer_token in immutable_state.outer_tokens.iter() {
            if token1.ne(&outer_token.address) && token2.ne(&outer_token.address) {
                // We want to reverse
                // (OuterT, T2 E3) -> (T2, T1, E2) -> (T1, OuterT, E1)
                routes.push(tokenize_tri(
                    &outer_token.address,
                    token2,
                    token1,
                    &other_exchange.router,
                    &other_exchange.router,
                    &other_exchange.router,
                    U256::from(other_exchange.swap_fee * 3),
                ));
            }
        }
    }

    send_routes(
        routes,
        gas_price,
        make_tri_tx,
        immutable_state,
        mutable_state,
    )
    .await;
}

pub async fn make_inner_tri_routes(
    token1: &H160,
    token2: &H160,
    gas_price: U256,
    exchange_index: usize,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    // In original ordering (no outer token)
    let mut routes: Vec<Bytes> = Vec::new();
    for other_exchange in immutable_state.exchanges.iter() {
        for outer_token in immutable_state.outer_tokens.iter() {
            if token1.ne(&outer_token.address) && token2.ne(&outer_token.address) {
                // We want to reverse
                // (OuterT, T2 E3) -> (T2, T1, E2) -> (T1, OuterT, E1)
                routes.push(tokenize_tri(
                    &outer_token.address,
                    token2,
                    token1,
                    &other_exchange.router,
                    &immutable_state.exchanges[exchange_index].router,
                    &other_exchange.router,
                    U256::from(
                        other_exchange.swap_fee
                            + other_exchange.swap_fee
                            + immutable_state.exchanges[exchange_index].swap_fee,
                    ),
                ));
            }
        }
    }

    send_routes(
        routes,
        gas_price,
        make_tri_tx,
        immutable_state,
        mutable_state,
    )
    .await;
}

async fn send_routes(
    routes: Vec<Bytes>,
    gas_price: U256,
    make_tx: fn(
        &Arc<ImmutableState>,
        Vec<Bytes>,
        &Arc<MutableState>,
        U256,
    ) -> TransactionParameters,
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
) {
    let wallet_index =
        mutable_state.increment_wallet_index(routes.len() / immutable_state.bundle_size + 1);
    for (wallet_increment, i) in (0..routes.len())
        .step_by(immutable_state.bundle_size)
        .enumerate()
    {
        let bundle = Vec::from(&routes[i..min(i + immutable_state.bundle_size, routes.len())]);
        let immutable_state = immutable_state.clone();
        let mutable_state = mutable_state.clone();
        let actual_wallet_index = wallet_index + wallet_increment;
        // let specific_wallet_index =
        tokio::spawn(async move {
            let tx_obj = make_tx(&immutable_state, bundle, &mutable_state, gas_price);
            send_transaction(
                &immutable_state,
                &mutable_state,
                actual_wallet_index,
                tx_obj,
            )
            .await;
        });
    }

    // let txs: Vec<_> = routes
    //     .iter()
    //     .enumerate()
    //     .step_by(immutable_state.bundle_size)
    //     .map(|(i, _)| {
    //         let (tx_obj, wallet_index) = make_tx(
    //             immutable_state,
    //             Vec::from(&routes[i..min(i + immutable_state.bundle_size, routes.len())]),
    //             mutable_state,
    //             gas_price,
    //         );
    //         send_transaction(immutable_state, mutable_state, wallet_index, tx_obj)
    //     })
    //     .collect();
    // join_all(txs).await;
}

fn favor_outer_token<'a>(
    token1: &'a H160,
    token2: &'a H160,
    immutable_state: &Arc<ImmutableState>,
) -> (&'a H160, &'a H160, bool) {
    if immutable_state
        .outer_tokens
        .iter()
        .any(|token| token.address.eq(token2))
    {
        // swapping here, so already in reverse change pool order
        (token2, token1, false)
    } else {
        // outer token already token1, so have to reverse the route by switching exchange ordering
        (token1, token2, true)
    }
}
