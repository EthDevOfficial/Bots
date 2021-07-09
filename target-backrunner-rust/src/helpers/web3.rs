use crate::types::{
    enums::Arb, exchange::Exchange, immutable_state::ImmutableState, mutable_state::MutableState,
    token::Token,
};
use ethabi::{ethereum_types::U256, Bytes};
use ethabi_contract::use_contract;
use ethabi_derive;
use std::sync::Arc;
use web3::{
    signing,
    transports::WebSocket,
    types::{TransactionParameters, H160},
    Error, Result, Web3,
};

use_contract!(optimizer, "./abis/optimizerExec.json");
use optimizer::functions;

pub fn make_simple_tx(
    immutable_state: Arc<ImmutableState>,
    bundle: Vec<Bytes>,
    mutable_state: Arc<MutableState>,
    gas_price: U256,
) -> (TransactionParameters, usize) {
    let wallet_index = mutable_state.increment_wallet_index();

    (
        TransactionParameters {
            to: Some(immutable_state.contract),
            gas_price: Some(gas_price),
            gas: 600_000.into(),
            nonce: Some(mutable_state.wallets[wallet_index].get_nonce()),
            chain_id: Some(immutable_state.chain_id),
            data: functions::simple_multicall::encode_input(bundle).into(),
            ..Default::default()
        },
        wallet_index,
    )
}

pub fn make_tri_tx(
    immutable_state: Arc<ImmutableState>,
    bundle: Vec<Bytes>,
    mutable_state: Arc<MutableState>,
    gas_price: U256,
) -> (TransactionParameters, usize) {
    let wallet_index = mutable_state.increment_wallet_index();
    println!("GP: {:?}", gas_price);
    (
        TransactionParameters {
            to: Some(immutable_state.contract),
            gas_price: Some(gas_price),
            gas: 600_000.into(),
            nonce: Some(mutable_state.wallets[wallet_index].get_nonce()),
            chain_id: Some(immutable_state.chain_id),
            data: functions::tri_multicall::encode_input(bundle).into(),
            ..Default::default()
        },
        wallet_index,
    )
}

pub async fn send_transaction(
    immutable_state: Arc<ImmutableState>,
    mutable_state: Arc<MutableState>,
    wallet_index: usize,
    tx: TransactionParameters,
) {
    //I dont know if we want block_on, seems to hold the thread till completion, but was used in the example.
    //let signed = tx.sign(seckey, chain_id);
    // let signed = futures::executor::block_on(web3.accounts().sign_transaction(tx, seckey)).unwrap();
    let signed = immutable_state
        .web3
        .accounts()
        .sign_transaction(tx, &mutable_state.wallets[wallet_index].private_key)
        .await
        .unwrap();
    let result = immutable_state
        .web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await;

    match result {
        Ok(response) => {
            // looks like this response may need decode to be readable
            println!("{:?}", response);
            mutable_state.wallets[wallet_index].increment_nonce();
        }
        Err(error) => {
            println!("{}", error);
            match error {
                Error::Rpc(error) => {
                    let error = error;
                    if error.message == "insufficient funds for gas * price + value".to_string() {
                        mutable_state
                            .hot_wallet
                            .send_to_wallet(
                                immutable_state.clone(),
                                Some(mutable_state.wallet_balance),
                                &mutable_state.wallets[wallet_index],
                                51.into(),
                                true,
                            )
                            .await;
                    }
                }
                _ => (),
            };
        }
    };
}

pub async fn connect_to_node(node_url: &str) -> Result<Web3<WebSocket>> {
    let transport = WebSocket::new(node_url).await?;
    let web3 = Web3::new(transport);
    Ok(web3)
}
