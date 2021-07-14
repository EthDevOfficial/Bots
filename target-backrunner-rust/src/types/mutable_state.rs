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
    pub async fn new(immutable_state: &Arc<ImmutableState>) -> Arc<Self> {
        // Wallets
        let wallet_path: String = env::var("WALLET_PATH").unwrap_or("./wallets.json".to_string());
        let gen_new_wallets: bool = env::var("GEN_NEW_WALLETS")
            .unwrap_or("false".to_string())
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
            immutable_state,
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
                        let prev_wallet = Wallet::load_from_pk(private_key, immutable_state).await;
                        let pull_result = prev_wallet
                            .send_to_wallet(immutable_state, None, &hot_wallet, wl_gas_price, false)
                            .await;
                        println!("Pulled from wallet [{:?}]", wallet_id);
                        match pull_result {
                            Ok(()) => (),
                            Err(err) => println!("wallet [{}] pull error: {:?}", wallet_id, err),
                        }
                    }
                }
                None => (),
            };

            // Create, send balance, and save wallets
            for i in 0..num_wallets {
                let wallet = Wallet::create_new();
                match wallet {
                    Ok(wallet) => {
                        // Get the current hot wallet nonce
                        let previous_nonce =
                            Wallet::get_nonce_from_chain(&hot_wallet.public_key, immutable_state)
                                .await;
                        let mut current_nonce = previous_nonce;
                        let mut poll_count: i32 = 0;
                        while previous_nonce == current_nonce {
                            // Send the first attempt to load the wallet
                            if poll_count == 0 {
                                let send_result = hot_wallet
                                    .send_to_wallet(
                                        immutable_state,
                                        Some(wallet_balance),
                                        &wallet,
                                        wl_gas_price,
                                        false,
                                    )
                                    .await;
                            }

                            // Poll the hot wallet nonce until it increments. If it doesn't after 10 polls, we resend
                            let poll_time = std::time::Duration::from_secs(2);
                            std::thread::sleep(poll_time);

                            current_nonce = Wallet::get_nonce_from_chain(
                                &hot_wallet.public_key,
                                immutable_state,
                            )
                            .await;

                            if poll_count == 9 {
                                poll_count = 0;
                            } else {
                                poll_count += 1;
                            }
                        }
                        println!("Loaded wallet [{:?}]", i);
                        loaded_wallets.push(wallet);
                        save_to_file(&loaded_wallets, &wallet_path);
                    }
                    Err(error) => println!("wallet creation error: {:?}", error),
                }
            }
        }

        let loaded_wallets_pk = read_private_keys_from_file(&wallet_path).unwrap();
        let mut loaded_wallets: Vec<Wallet> = Vec::new();

        for (wallet_id, private_key) in loaded_wallets_pk.into_iter().enumerate() {
            loaded_wallets.push(Wallet::load_from_pk(private_key, immutable_state).await);
            println!("loaded wallet {}", wallet_id);
        }

        Arc::new(MutableState {
            wallets: loaded_wallets,
            wallet_index: Mutex::new(0),
            hot_wallet,
            wallet_balance,
        })
    }

    pub fn increment_wallet_index(&self, inc_amount: usize) -> usize {
        let mut wallet_index = self.wallet_index.lock().unwrap();
        let to_return = wallet_index.clone();
        if *wallet_index + inc_amount >= self.wallets.len() {
            *wallet_index = inc_amount - (self.wallets.len() - *wallet_index);
        } else {
            *wallet_index += inc_amount;
        }
        to_return
    }
}
