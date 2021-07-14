use crate::types::{
    enums::Arb, exchange::Exchange, immutable_state::ImmutableState, mutable_state::MutableState,
    token::Token,
};
use ethabi::{ethereum_types::U256, Bytes};
use ethabi_contract::use_contract;
use ethabi_derive;
use futures::join;
use std::sync::Arc;
use web3::{
    signing,
    transports::WebSocket,
    types::{TransactionParameters, H160},
    Error, Result, Web3,
};

pub fn make_simple_tx(
    immutable_state: &Arc<ImmutableState>,
    bundle: Vec<Bytes>,
) -> TransactionParameters {
    TransactionParameters {
        to: Some(immutable_state.contract),
        gas_price: Some(immutable_state.gas_price),
        gas: immutable_state.gas_limit.into(),
        chain_id: Some(immutable_state.chain_id),
        data: (immutable_state.simple_multicall)(bundle).into(),
        ..Default::default()
    }
}

pub fn make_tri_tx(
    immutable_state: &Arc<ImmutableState>,
    bundle: Vec<Bytes>,
) -> TransactionParameters {
    TransactionParameters {
        to: Some(immutable_state.contract),
        gas_price: Some(immutable_state.gas_price),
        gas: immutable_state.gas_limit.into(),
        chain_id: Some(immutable_state.chain_id),
        data: (immutable_state.tri_multicall)(bundle).into(),
        ..Default::default()
    }
}

#[allow(unused_must_use)]
pub async fn send_transaction(
    immutable_state: &Arc<ImmutableState>,
    mutable_state: &Arc<MutableState>,
    wallet_index: usize,
    tx: TransactionParameters,
) {
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
            // mutable_state.wallets[wallet_index].increment_nonce();
        }
        Err(error) => {
            println!("{}", error);
            match error {
                Error::Rpc(error) => {
                    let error = error;
                    if error.message.contains("funds") {
                        mutable_state
                            .hot_wallet
                            .send_to_wallet(
                                immutable_state,
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

// pub async fn connect_to_node_http(node_url: &str) -> Result<Web3<Http>> {
//     let transport = Http::new(node_url)?;
//     let web3 = Web3::new(transport);
//     Ok(web3)
// }
