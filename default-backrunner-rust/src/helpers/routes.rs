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

pub fn make_simples_defaults(immutable_state: Arc<ImmutableState>) -> Vec<TransactionParameters>{
    let mut routes: Vec<Bytes> = Vec::new();

    for i in 0..immutable_state.inner_tokens.len() {
        for j in 0..immutable_state.exchanges.len() {
            for k in 0..immutable_state.exchanges.len() {
                if j != k {
                    routes.push(tokenize_simple(
                        &immutable_state.outer_token.address,
                        &immutable_state.inner_tokens[i].address,
                        &immutable_state.exchanges[j].router,
                        &immutable_state.exchanges[k].router,
                        U256::from(
                            immutable_state.exchanges[j].swap_fee
                                + immutable_state.exchanges[k].swap_fee,
                        ),
                    ));
                }
            }
        }
    }

    let mut txs: Vec<TransactionParameters> = Vec::new();
    for i in (0..routes.len()).step_by(immutable_state.bundle_size) {
        let bundle = Vec::from(&routes[i..min(i + immutable_state.bundle_size, routes.len())]);
        let immutable_state = immutable_state.clone();
        txs.push(make_simple_tx(&immutable_state, bundle));
    }
    txs
}

pub fn make_tris_defaults(immutable_state: Arc<ImmutableState>) -> Vec<TransactionParameters>{
    let mut routes: Vec<Bytes> = Vec::new();

    for i in 0..immutable_state.inner_tokens.len() {
        for j in 0..immutable_state.inner_tokens.len() {
            if i != j {
                for k in 0..immutable_state.exchanges.len() {
                    for m in 0..immutable_state.exchanges.len() {
                        for l in 0..immutable_state.exchanges.len() {
                            routes.push(tokenize_tri(
                                &immutable_state.outer_token.address,
                                &immutable_state.inner_tokens[i].address,
                                &immutable_state.inner_tokens[j].address,
                                &immutable_state.exchanges[k].router,
                                &immutable_state.exchanges[m].router,
                                &immutable_state.exchanges[l].router,
                                U256::from(
                                    immutable_state.exchanges[k].swap_fee
                                        + immutable_state.exchanges[m].swap_fee
                                        + immutable_state.exchanges[l].swap_fee,
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }

    let mut txs: Vec<TransactionParameters> = Vec::new();
    for i in (0..routes.len()).step_by(immutable_state.bundle_size) {
        let bundle = Vec::from(&routes[i..min(i + immutable_state.bundle_size, routes.len())]);
        let immutable_state = immutable_state.clone();
        txs.push(make_tri_tx(&immutable_state, bundle));
    }
    txs
}
