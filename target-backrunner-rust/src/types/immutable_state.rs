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
    // pub web3_quick_node: Web3<Http>,
    // pub web3_infura: Web3<Http>,
    pub primary_exchanges: Vec<Exchange>,
    pub secondary_exchanges: Vec<Exchange>,
    pub exchanges: Vec<Exchange>,
    pub routers: Vec<Exchange>,
    pub inner_tokens: Vec<Token>,
    // pub inner_token_addresses: Vec<String>,
    pub outer_tokens: Vec<Token>,
    pub tokens: Vec<Token>,
    // pub outer_token_addresses: Vec<String>,
    pub ignore_addresses: Vec<String>,
    pub contract: H160,
    pub run_tris: bool,
    pub run_simples: bool,
    pub bundle_size: usize,
    pub gas_limit: usize,
    pub simple_multicall: fn(Vec<Vec<u8>>) -> Vec<u8>,
    pub tri_multicall: fn(Vec<Vec<u8>>) -> Vec<u8>,
    pub gas_price_limit: U256,
}
impl ImmutableState {
    pub async fn new(
        chain: Chain,
        primary_exchanges: Vec<Exchange>,
        secondary_exchanges: Vec<Exchange>,
        aggregators: Vec<Exchange>,
        outer_tokens: Vec<Token>,
        inner_tokens: Vec<Token>,
        ignore_addresses: Vec<&str>,
    ) -> Self {
        // Web3
        let ws_url = env::var("WS_URL").unwrap_or("ws://34.204.203.210:8546".to_string());
        let web3 = connect_to_node(&ws_url).await.unwrap();

        // let web3_infura = connect_to_node_http(
        //     "https://polygon-mainnet.infura.io/v3/8883e83b5ecc4d15837b55a135609ed9",
        // )
        // .await
        // .unwrap();

        // let web3_quick_node = connect_to_node_http("https://green-falling-forest.matic.quiknode.pro/").await.unwrap();

        // Contracts
        let contract = H160::from_str(
            &env::var("CONTRACT")
                .unwrap_or("0x6E8a22e28A92f47CE1CE76a26dE691802A25ca85".to_string()),
        )
        .unwrap();

        let mut tokens: Vec<Token> = Vec::new();

        outer_tokens
            .iter()
            .for_each(|token| tokens.push(token.clone()));
        inner_tokens
            .iter()
            .for_each(|token| tokens.push(token.clone()));

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
        let mut exchanges: Vec<Exchange> = Vec::new();

        primary_exchanges
            .iter()
            .for_each(|primary: &Exchange| exchanges.push(primary.clone()));
        secondary_exchanges
            .iter()
            .for_each(|secondary: &Exchange| exchanges.push(secondary.clone()));

        let mut routers: Vec<Exchange> = Vec::new();

        primary_exchanges
            .iter()
            .for_each(|primary: &Exchange| routers.push(primary.clone()));
        secondary_exchanges
            .iter()
            .for_each(|secondary: &Exchange| routers.push(secondary.clone()));
        aggregators
            .iter()
            .for_each(|agg: &Exchange| routers.push(agg.clone()));

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
            // web3_quick_node,
            // web3_infura,
            primary_exchanges: primary_exchanges.clone(),
            secondary_exchanges: secondary_exchanges.clone(),
            exchanges,
            routers,
            outer_tokens: outer_tokens.clone(),
            // outer_token_addresses: outer_tokens.clone().into_iter().map(|token| token.address).collect(),
            inner_tokens: inner_tokens.clone(),
            tokens,
            // inner_token_addresses: inner_tokens.clone().into_iter().map(|token| token.address).collect(),
            ignore_addresses: ignore_addresses
                .into_iter()
                .map(|ignore: &str| ignore.to_string())
                .collect(),
            contract,
            run_simples,
            run_tris,
            bundle_size,
            gas_limit,
            simple_multicall,
            tri_multicall,
            gas_price_limit: U256::from_dec_str(
                &env::var("GAS_PRICE_LIMIT").unwrap_or("500000000000".to_string()),
            )
            .unwrap(),
        }
    }
}
