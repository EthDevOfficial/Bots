use crate::helpers::{abi::decode, changed_pool::process_router_params};
use crate::types::immutable_state::ImmutableState;
use crate::types::mutable_state::MutableState;
use std::sync::Arc;
use web3::{
    types::{Bytes, Transaction, TransactionId, H256},
    Result,
};

async fn process_transaction(
    transaction: Transaction,
    immutable_state: Arc<ImmutableState>,
    mutable_state: Arc<MutableState>,
) {
    match transaction.to {
        Some(to_address) => {
            let optional_exchange_index = immutable_state
                .exchanges
                .iter()
                .position(|exchange| exchange.router == to_address);
            match optional_exchange_index {
                Some(exchange_index) => match &transaction.input {
                    Bytes(encoded_tx) => {
                        let (func, params) =
                            decode(encoded_tx, &immutable_state.exchanges[exchange_index].abi);
                        process_router_params(
                            func,
                            params,
                            transaction.gas_price,
                            exchange_index,
                            immutable_state.clone(),
                            mutable_state,
                        )
                        .await;
                    }
                },
                None => {}
            }
        }
        None => {}
    };
}

pub async fn process_hash(
    tx_hash: Result<H256>,
    immutable_state: Arc<ImmutableState>,
    mutable_state: Arc<MutableState>,
) {
    match tx_hash {
        Ok(hash) => {
            tokio::spawn(async move {
                let result_tx_data = immutable_state
                    .web3
                    .eth()
                    .transaction(TransactionId::Hash(hash))
                    .await;
                match result_tx_data {
                    Ok(optional_tx_data) => match optional_tx_data {
                        Some(tx_data) => {
                            process_transaction(tx_data, immutable_state, mutable_state).await;
                        }
                        None => {}
                    },
                    Err(error) => println!("SPAWN TX HANDLER ERROR: {:?}", error),
                };
            })
            .await
            .unwrap();
        }
        Err(error) => println!("{:?}", error),
    }
}
