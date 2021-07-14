use crate::helpers::web3::connect_to_node;
use crate::types::enums::Chain;
use crate::types::exchange::Exchange;
use crate::types::token::Token;
use ethabi::ethereum_types::U256;
use ethabi::Bytes;
use ethabi_contract::use_contract;
use std::{env, str::FromStr, vec::Vec};
use web3::types::H160;
use web3::{transports::WebSocket, Web3};

use_contract!(optimizer, "./abis/optimizerExec.json");
use optimizer::functions;

pub struct ImmutableState {
    pub chain: Chain,
    pub chain_id: u64,
    pub web3: Web3<WebSocket>,
    pub primary_exchanges: Vec<Exchange>,
    pub secondary_exchanges: Vec<Exchange>,
    pub exchanges: Vec<Exchange>,
    pub inner_tokens: Vec<Token>,
    pub outer_token: Token,
    pub contract: H160,
    pub run_tris: bool,
    pub run_simples: bool,
    pub bundle_size: usize,
    pub gas_limit: usize,
    pub gas_price: U256,
    pub simple_multicall: fn(Vec<Vec<u8>>) -> Vec<u8>,
    pub tri_multicall: fn(Vec<Vec<u8>>) -> Vec<u8>,
}
impl ImmutableState {
    pub async fn new(
        chain: Chain,
        primary_exchanges: Vec<Exchange>,
        secondary_exchanges: Vec<Exchange>,
        outer_token: Token,
        inner_tokens: Vec<Token>,
    ) -> Self {
        // Web3
        let ws_url = env::var("WS_URL").unwrap_or("ws://34.204.203.210:8546".to_string());
        let web3 = connect_to_node(&ws_url).await.unwrap();

        // Contracts
        let contract = H160::from_str(
            &env::var("CONTRACT")
                .unwrap_or("0x6E8a22e28A92f47CE1CE76a26dE691802A25ca85".to_string()),
        )
        .unwrap();

        // Routing
        let run_simples: bool = env::var("RUN_SIMPLES")
            .unwrap_or("true".to_string())
            .eq("true");
        let run_tris: bool = env::var("RUN_TRIS")
            .unwrap_or("true".to_string())
            .eq("true");

        // Bidding
        let bundle_size: usize = env::var("BUNDLE_SIZE")
            .unwrap_or("2".to_string())
            .parse()
            .unwrap();

        let gas_limit: usize = env::var("GAS_LIMIT")
            .unwrap_or("1200000".to_string())
            .parse()
            .unwrap();

        let gas_price: U256 =
            U256::from_dec_str(&env::var("GAS_PRICE").unwrap_or("50".to_string())).unwrap();

        let gas_price = U256::exp10(9).saturating_mul(gas_price);

        let mut exchanges: Vec<Exchange> = Vec::new();

        primary_exchanges
            .iter()
            .for_each(|primary: &Exchange| exchanges.push(primary.clone()));
        secondary_exchanges
            .iter()
            .for_each(|secondary: &Exchange| exchanges.push(secondary.clone()));

        let simple_multicall = if chain != Chain::Polygon {
            |bundle: Vec<Bytes>| functions::simple_multicall::encode_input(bundle)
        } else {
            |bundle: Vec<Bytes>| functions::simple_multicall_chi::encode_input(bundle)
        };

        let tri_multicall = if chain != Chain::Polygon {
            |bundle: Vec<Bytes>| functions::tri_multicall::encode_input(bundle)
        } else {
            |bundle: Vec<Bytes>| functions::tri_multicall_chi::encode_input(bundle)
        };

        ImmutableState {
            chain,
            chain_id: web3
                .eth()
                .chain_id()
                .await
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            web3,
            primary_exchanges: primary_exchanges.clone(),
            secondary_exchanges: secondary_exchanges.clone(),
            exchanges,
            outer_token: outer_token.clone(),
            inner_tokens: inner_tokens.clone(),
            contract,
            run_simples,
            run_tris,
            bundle_size,
            gas_limit,
            gas_price,
            simple_multicall,
            tri_multicall,
        }
    }
}
