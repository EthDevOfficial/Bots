#![allow(dead_code)] // comment these out for unused annotations to come back
#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod helpers;
mod loaders;
use crate::helpers::process_pending::process_hash;
pub mod types;
use web3::futures::StreamExt;

#[tokio::main]
async fn main() {
    println!("starting target backrunner");

    let immutable_state = loaders::load_immutable_state().await;
    let mutable_state = loaders::load_mutable_state(&immutable_state).await;
    println!("starting target backrunner");

    immutable_state
        .web3
        .eth_subscribe()
        .subscribe_new_pending_transactions()
        .await
        .unwrap()
        .for_each(|tx_hash| process_hash(tx_hash, immutable_state.clone(), mutable_state.clone()))
        .await;
}
