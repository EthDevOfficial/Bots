use crate::types::immutable_state::ImmutableState;
use clarity::PrivateKey;
use rand;
use secp256k1::SecretKey;
use std::ops::Add;
use std::sync::Arc;
use std::{str, usize};
use std::{str::FromStr, sync::Mutex};
use web3::{
    ethabi::ethereum_types::U256,
    types::{Address, TransactionParameters},
    Result as Web3Result,
};

pub struct Wallet {
    pub public_key: Address,
    pub private_key: SecretKey,
    pub nonce: Mutex<U256>,
}

impl Wallet {
    pub fn create_new() -> Result<Wallet, clarity::Error> {
        let key_bytes: [u8; 32] = rand::random();
        let private_key: PrivateKey = PrivateKey::from_slice(&key_bytes).unwrap();
        let public_key = private_key.to_public_key();
        match public_key {
            Ok(public_key) => Ok(Wallet {
                public_key: Address::from_str(&public_key.to_string()).unwrap(),
                private_key: SecretKey::from_str(&private_key.to_string()[2..]).unwrap(),
                nonce: Mutex::new(U256::zero()),
            }),
            Err(error) => Err(error),
        }
    }

    pub async fn load_from_pk(private_key: String, immutable_state: Arc<ImmutableState>) -> Self {
        let private_key = PrivateKey::from_str(&private_key).unwrap();
        let public_key =
            Address::from_str(&private_key.to_public_key().unwrap().to_string()).unwrap();
        let nonce = Wallet::get_nonce_from_chain(&public_key, &immutable_state).await;

        Wallet {
            public_key,
            private_key: SecretKey::from_str(&private_key.to_string()[2..]).unwrap(),
            nonce: Mutex::new(nonce),
        }
    }

    pub async fn send_to_wallet(
        &self,
        immutable_state: Arc<ImmutableState>,
        optional_amount: Option<U256>,
        to: &Wallet,
        gas_price: U256,
        get_nonce_from_chain: bool
    ) -> Web3Result {
        let gas_price = U256::exp10(9).saturating_mul(gas_price);
        let gas_limit: U256 = 21_000.into();

        let amount = match optional_amount {
            Some(amount) => U256::exp10(16).saturating_mul(amount),
            None => {
                let wallet_balance_wei: U256 = immutable_state
                    .web3
                    .eth()
                    .balance(self.public_key, None)
                    .await
                    .unwrap();

                wallet_balance_wei.saturating_sub(gas_price.saturating_mul(gas_limit))
            }
        };

        let tx_object = {
            let nonce = if get_nonce_from_chain {
                Some(Wallet::get_nonce_from_chain(&self.public_key, &immutable_state).await)
            }
            else
            {
                Some(Wallet::get_nonce(self))
            };

            match optional_amount {
                Some(amount) => println!("Sent from wallet with nonce: {:?} to: {:?}", nonce.unwrap(), to.public_key),
                None => println!("Pulled from wallet with nonce: {:?}", nonce.unwrap()),
            }

            TransactionParameters {
                to: Some(to.public_key),
                value: amount,
                gas_price: Some(gas_price),
                gas: gas_limit,
                nonce: nonce,
                ..Default::default()
            }
        };

        if !get_nonce_from_chain {self.increment_nonce();}

        // Sign the tx (can be done offline)
        let signed = immutable_state
            .web3
            .accounts()
            .sign_transaction(tx_object, &self.private_key)
            .await
            .unwrap();

        // Send the tx to our node
        let result = immutable_state
            .web3
            .eth()
            .send_raw_transaction(signed.raw_transaction)
            .await;

        Ok(())
    }

    pub fn increment_nonce(&self) {
        let mut nonce = self.nonce.lock().unwrap();
        *nonce += U256::one();
    }

    pub fn get_nonce(&self) -> U256 {
        self.nonce.lock().unwrap().clone()
    }

    async fn get_nonce_from_chain(
        public_key: &Address,
        immutable_state: &Arc<ImmutableState>,
    ) -> U256 {
        immutable_state
            .web3
            .eth()
            .transaction_count(public_key.clone(), None)
            .await
            .unwrap()
    }
}
