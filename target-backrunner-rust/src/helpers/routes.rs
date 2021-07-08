use crate::helpers::encoder::{tokenize_simple, tokenize_tri};
use crate::helpers::web3::{make_simple_tx, make_tri_tx, send_transaction};
use crate::types::{
    exchange::Exchange, immutable_state::ImmutableState, mutable_state::MutableState, token::Token,
};
use ethabi::ethereum_types::U256;
use ethabi::{Bytes, Token as Tokenized};
use std::sync::Arc;
use std::vec::Vec;
use web3::types::H160;
use std::cmp::min;

pub async fn make_simple_routes(
    token1: &H160,
    token2: &H160,
    gas_price: U256,
    exchange_index: usize,
    immutable_state: Arc<ImmutableState>,
    mutable_state: Arc<MutableState>,
) {
    let mut routes: Vec<Bytes> = Vec::new();

    // Find the correct pool ordering and record whether the route should be reversed
    let (token1, token2, should_reverse) = favor_outer_token(token1, token2, &immutable_state);

    // Return the relevant exchanges to loop through
    let (exchanges, not_primary) =
        if immutable_state
            .primary_exchanges
            .iter()
            .any(|prim_exchange| {
                immutable_state.exchanges[exchange_index].router == prim_exchange.router
            })
        {
            (&immutable_state.exchanges, false)
        } else {
            (&immutable_state.primary_exchanges, true)
        };

    // Make the routes
    // TODO: seperate into 4 blocks
    if should_reverse {
        for i in 0..exchanges.len() {
            if not_primary || i != exchange_index {
                let away_exchange = &exchanges[i];
                let return_exchange = &immutable_state.exchanges[exchange_index];
                // (T1, T2, E2) -> (T2, T1, E1)
                routes.push(tokenize_simple(
                    token1,
                    token2,
                    &away_exchange.router,
                    &return_exchange.router,
                    (away_exchange.swap_fee + return_exchange.swap_fee).into(),
                ));
            }
        }
    } else {
        for i in 0..exchanges.len() {
            if not_primary || i != exchange_index {
                let away_exchange = &immutable_state.exchanges[exchange_index];
                let return_exchange = &exchanges[i];
                // (T1, T2, E1) -> (T2, T1. E2)
                routes.push(tokenize_simple(
                    token1,
                    token2,
                    &away_exchange.router,
                    &return_exchange.router,
                    (away_exchange.swap_fee + return_exchange.swap_fee).into(),
                ));
            }
        }
    };

    let bundle_size = immutable_state.bundle_size;
    while routes.len() > 0 {
        let bundle: Vec<Bytes> = routes.drain(0..min(bundle_size,routes.len())).collect();
        let (tx_obj, wallet_index) = make_simple_tx(
            immutable_state.clone(),
            bundle,
            mutable_state.clone(),
            gas_price,
        );
        send_transaction(
            immutable_state.clone(),
            mutable_state.clone(),
            wallet_index,
            tx_obj,
        )
        .await;
    }
}

pub async fn make_tri_routes(
    token1: &H160,
    token2: &H160,
    gas_price: U256,
    exchange_index: usize,
    immutable_state: Arc<ImmutableState>,
    mutable_state: Arc<MutableState>,
) {
    // Find the correct pool ordering and record whether the route should be reversed
    let (token1, token2, should_reverse) = favor_outer_token(token1, token2, &immutable_state);

    // Make routes where the changed pool is on the outside
    let mut outer_routes = outer_tri_routes(
        token1,
        token2,
        should_reverse,
        exchange_index,
        immutable_state.clone(),
    );

    // This combines the inners and outers, not sure if thats what we want
    // let tri_routes: Vec<Tokenized> = outer_routes.into_iter().chain(inner_routes).collect();

    let bundle_size = immutable_state.bundle_size;
    while outer_routes.len() > 0 {
        let bundle: Vec<Bytes> = outer_routes.drain(0..min(bundle_size,outer_routes.len())).collect();
        let (tx_obj, wallet_index) = make_tri_tx(
            immutable_state.clone(),
            bundle,
            mutable_state.clone(),
            gas_price,
        );
        send_transaction(
            immutable_state.clone(),
            mutable_state.clone(),
            wallet_index,
            tx_obj,
        )
        .await;
    }
}

fn outer_tri_routes(
    token1: &H160,
    token2: &H160,
    should_reverse: bool,
    exchange_index: usize,
    immutable_state: Arc<ImmutableState>,
) -> Vec<Bytes> {
    // Token1 is preferred
    let mut routes: Vec<Bytes> = Vec::new();
    for primary_exchange in immutable_state.primary_exchanges.iter() {
        for inner_token in immutable_state.inner_tokens.iter() {
            if token1.ne(&inner_token.address) && token2.ne(&inner_token.address) {
                if should_reverse {
                    // (T1, InnerT, E3) -> (InnerT, T2, E2) -> (T2, T1, E1)
                    routes.push(tokenize_tri(
                        token1,
                        &inner_token.address,
                        token2,
                        &primary_exchange.router,
                        &primary_exchange.router,
                        &immutable_state.exchanges[exchange_index].router,
                        (primary_exchange.swap_fee
                            + primary_exchange.swap_fee
                            + immutable_state.exchanges[exchange_index].swap_fee)
                            .into(),
                    ));
                } else {
                    // (T1, T2, E1) -> (T2, InnerT, E2) -> (InnerT, T1, E3)
                    routes.push(tokenize_tri(
                        token1,
                        token2,
                        &inner_token.address,
                        &immutable_state.exchanges[exchange_index].router,
                        &primary_exchange.router,
                        &primary_exchange.router,
                        (primary_exchange.swap_fee
                            + primary_exchange.swap_fee
                            + immutable_state.exchanges[exchange_index].swap_fee)
                            .into(),
                    ));
                }
            }
        }
    }
    routes
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

// fn inner_tri_routes(
//     token1: &String,
//     token2: &String,
//     outer_tokens: &Vec<Token>,
//     should_reverse: bool,
//     exchange_index: usize,
//     immutable_state: Arc<ImmutableState>,
// ) -> Vec<Tokenized> {
//     // Token1 is preferred
//     let mut routes: Vec<Tokenized> = Vec::new();
//     for primary_exchange in immutable_state.primary_exchanges.iter() {
//         for outer_token in outer_tokens.iter() {
//             if token1.ne(&outer_token.address) && token2.ne(&outer_token.address) {
//                 if should_reverse {
//                     // (OuterT, T2 E3) -> (T2, T1, E2) -> (T1, OuterT, E1)
//                     routes.push(tokenize_tri(
//                         &outer_token.address,
//                         token2,
//                         token1,
//                         &primary_exchange.router,
//                         &immutable_state.exchanges[exchange_index].router,
//                         &primary_exchange.router,
//                         primary_exchange.swap_fee + primary_exchange.swap_fee + immutable_state.exchanges[exchange_index].swap_fee
//                     ));
//                 } else {
//                     // (OuterT, T1, E1) -> (T1, T2, E2) -> (T2, OuterT, E3)
//                     routes.push(tokenize_tri(
//                         &outer_token.address,
//                         token1,
//                         token2,
//                         &primary_exchange.router,
//                         &immutable_state.exchanges[exchange_index].router,
//                         &primary_exchange.router,
//                         primary_exchange.swap_fee + primary_exchange.swap_fee + immutable_state.exchanges[exchange_index].swap_fee
//                     ));
//                 }
//             }
//         }
//     }
//     routes
// }