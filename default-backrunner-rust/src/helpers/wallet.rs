use crate::types::{immutable_state::ImmutableState, wallet::Wallet};
use secp256k1::SecretKey;
use serde_json::Error;
use std::fs::{read, write};
use std::sync::Arc;
use std::vec::Vec;
use web3::{
    types::{Address, TransactionParameters, U256},
    Result,
};

pub fn save_to_file(wallets: &Vec<Wallet>, wallet_path: &String) {
    let to_save: Vec<String> = wallets
        .iter()
        .map(|wallet| wallet.private_key.to_string())
        .collect();
    let json = serde_json::to_string(&to_save);
    match json {
        Ok(json) => write(wallet_path, json).unwrap(),
        Err(err) => println!("wallets serialization failed: {:?}", err),
    };
}

pub fn read_private_keys_from_file(wallet_path: &String) -> Option<Vec<String>> {
    let contents = read(wallet_path);
    match contents {
        Ok(contents) => Some(serde_json::from_slice(&contents).unwrap()),
        Err(err) => None,
    }
}
