use crate::helpers::web3::connect_to_node;
use crate::types::enums::Chain;
use crate::types::exchange::Exchange;
use crate::types::token::Token;
use ethabi::ethereum_types::U256;
use std::{env, str::FromStr, vec::Vec};
use web3::types::H160;
use web3::{transports::WebSocket, Web3};

pub struct ImmutableState {
    pub chain: Chain,
    pub chain_id: u64,
    pub web3: Web3<WebSocket>,
    pub primary_exchanges: Vec<Exchange>,
    pub secondary_exchanges: Vec<Exchange>,
    pub exchanges: Vec<Exchange>,
    pub inner_tokens: Vec<Token>,
    // pub inner_token_addresses: Vec<String>,
    pub outer_tokens: Vec<Token>,
    // pub outer_token_addresses: Vec<String>,
    pub ignore_addresses: Vec<String>,
    pub contract: H160,
    pub run_tris: bool,
    pub run_simples: bool,
    pub bundle_size: usize,
}
impl ImmutableState {
    pub async fn new(
        chain: Chain,
        primary_exchanges: Vec<Exchange>,
        secondary_exchanges: Vec<Exchange>,
        outer_tokens: Vec<Token>,
        inner_tokens: Vec<Token>,
        ignore_addresses: Vec<&str>,
    ) -> Self {
        // Web3
        let ws_url = env::var("WS_URL").unwrap_or("ws://35.168.113.135:8546".to_string());
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
        let mut exchanges: Vec<Exchange> = Vec::new();

        primary_exchanges
            .iter()
            .for_each(|primary: &Exchange| exchanges.push(primary.clone()));
        secondary_exchanges
            .iter()
            .for_each(|secondary: &Exchange| exchanges.push(secondary.clone()));

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
            outer_tokens: outer_tokens.clone(),
            // outer_token_addresses: outer_tokens.clone().into_iter().map(|token| token.address).collect(),
            inner_tokens: inner_tokens.clone(),
            // inner_token_addresses: inner_tokens.clone().into_iter().map(|token| token.address).collect(),
            ignore_addresses: ignore_addresses
                .into_iter()
                .map(|ignore: &str| ignore.to_string())
                .collect(),
            contract,
            run_simples,
            run_tris,
            bundle_size,
        }
    }
}
