#![allow(dead_code)] // comment these out for unused annotations to come back
#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod helpers;
mod loaders;
pub mod types;
use web3::futures::StreamExt;
use web3::types::{TransactionParameters, H160};


#[tokio::main]
async fn main() {
    println!("starting default backrunner");
    let immutable_state = loaders::load_immutable_state().await;
    println!("loaded immutable state");
    let mutable_state = loaders::load_mutable_state(&immutable_state).await;
    println!("loaded mutable state");
    println!("default backrunner started");

    let mut txs: Vec<TransactionParameters> = Vec::new();

    if immutable_state.run_simples {
        let mut simple_txs = helpers::routes::make_simples_defaults(immutable_state.clone());
        txs.append(&mut simple_txs);
    }
    if immutable_state.run_tris {
        let mut tri_txs = helpers::routes::make_tris_defaults(immutable_state.clone());
        txs.append(&mut tri_txs);
    }

    helpers::web3::send_transaction(&immutable_state, &mutable_state, wallet_index, tx)
   
}
