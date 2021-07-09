use crate::helpers::wallet::{read_private_keys_from_file, save_to_file};
use crate::loaders::load_immutable_state;
use crate::types::immutable_state::ImmutableState;
use crate::types::wallet::Wallet;
use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use web3::types::{Address, U256};

pub struct MutableState {
    pub wallets: Vec<Wallet>,
    pub wallet_index: Mutex<usize>,
    pub hot_wallet: Wallet,
    pub wallet_balance: U256,
}
impl MutableState {
    pub async fn new(immutable_state: Arc<ImmutableState>) -> Arc<Self> {
        // Wallets
        let wallet_path: String = env::var("WALLET_PATH").unwrap_or("./wallets.json".to_string());
        let gen_new_wallets: bool = env::var("GEN_NEW_WALLETS")
            .unwrap_or("true".to_string())
            .eq("true");

        let num_wallets: usize = env::var("NUM_WALLETS")
            .unwrap_or("10".to_string())
            .parse()
            .unwrap();

        let wallet_balance: U256 =
            U256::from_dec_str(&env::var("WALLET_BALANCE").unwrap_or("100".to_string())).unwrap();

        let wl_gas_price: U256 =
            U256::from_dec_str(&env::var("WL_GAS_PRICE").unwrap_or("50".to_string())).unwrap();

        let hot_wallet = Wallet::load_from_pk(
            "593b7e767faafbe9d60488cc01dc748ee83ce3aef4a8c5cbff80ee94bb5ec7bf".to_string(),
            immutable_state.clone(),
        )
        .await;

        println!("Hot wallet address: {:?}", hot_wallet.public_key);

        let mut loaded_wallets: Vec<Wallet> = Vec::new();

        if gen_new_wallets {
            let prev_wallets_pk = read_private_keys_from_file(&wallet_path);

            // If there were any previously loaded wallets, pull
            match prev_wallets_pk {
                Some(prev_wallets_pk) => {
                    for (wallet_id, private_key) in prev_wallets_pk.into_iter().enumerate() {
                        let prev_wallet =
                            Wallet::load_from_pk(private_key, immutable_state.clone()).await;
                        let pull_result = prev_wallet
                            .send_to_wallet(
                                immutable_state.clone(),
                                None,
                                &hot_wallet,
                                wl_gas_price,
                                false,
                            )
                            .await;
                        match pull_result {
                            Ok(()) => (),
                            Err(err) => println!("wallet {} pull error: {:?}", wallet_id, err),
                        }
                    }
                }
                None => (),
            };

            // let ten_sec = std::time::Duration::from_secs(30);
            // println!("Waiting {:?} for wallet cooldown", ten_sec);
            // std::thread::sleep(ten_sec);

            // Create, send balance, and save wallets
            for i in 0..num_wallets {
                let wallet = Wallet::create_new();
                match wallet {
                    Ok(wallet) => {
                        let send_result = hot_wallet
                            .send_to_wallet(
                                immutable_state.clone(),
                                Some(wallet_balance),
                                &wallet,
                                wl_gas_price,
                                false,
                            )
                            .await;
                        match send_result {
                            Ok(()) => {
                                loaded_wallets.push(wallet);
                                save_to_file(&loaded_wallets, &wallet_path);
                            }
                            Err(err) => println!("failed wallet send: {:?}", err),
                        }
                        let ten_millis = std::time::Duration::from_millis(6000);

                        std::thread::sleep(ten_millis);
                    }
                    Err(error) => println!("wallet creation error: {:?}", error),
                }
            }
        }

        let ten_sec = std::time::Duration::from_secs(20);
        println!("Waiting {:?} for wallet cooldown", ten_sec);
        std::thread::sleep(ten_sec);

        let loaded_wallets_pk = read_private_keys_from_file(&wallet_path).unwrap();
        let mut loaded_wallets: Vec<Wallet> = Vec::new();

        for (wallet_id, private_key) in loaded_wallets_pk.into_iter().enumerate() {
            loaded_wallets.push(Wallet::load_from_pk(private_key, immutable_state.clone()).await);
            println!("loaded wallet {}", wallet_id);
        }

        Arc::new(MutableState {
            wallets: loaded_wallets,
            wallet_index: Mutex::new(0),
            hot_wallet,
            wallet_balance,
        })
    }

    pub fn increment_wallet_index(&self) -> usize {
        let mut wallet_index = self.wallet_index.lock().unwrap();
        if *wallet_index + 1 == self.wallets.len() {
            *wallet_index = 0;
            wallet_index.clone()
        } else {
            *wallet_index += 1;
            wallet_index.clone()
        }
    }
}
