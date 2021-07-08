// use std::sync::Arc;

// use crate::helpers::web3::connect_to_node;
// use crate::types::enums::Router;
// use crate::types::immutable_state::ImmutableState;
// use crate::types::token::Token;
// use crate::types::{enums::Chain, exchange::Exchange};

// pub async fn load_immutable_state(ws: &str) -> Arc<ImmutableState> {
//     // let web3 = connect(ws).await.unwrap();
//     let web3 = connect_to_node("ws://35.168.113.135:8546").await.unwrap();

//     let tokens = Tokens::new();
//     let exchanges = Exchanges::new();

//     let outer_tokens = vec![tokens.weth, tokens.wmatic];
//     let inner_tokens = vec![tokens.bone, tokens.titan, tokens.usdc, tokens.pup];

//     let primary_exchanges = vec![exchanges.quickswap];
//     let secondary_exchanges = vec![exchanges.sushiswap, exchanges.dfyn, exchanges.apeswap];

//     let ignore_tokens = vec![];

//     Arc::new(ImmutableState::new(
//         Chain::Polygon,
//         web3,
//         primary_exchanges,
//         secondary_exchanges,
//         outer_tokens,
//         inner_tokens,
//         ignore_tokens,
//     ))
// }

// struct Tokens {
//     quick: Token,
//     wmatic: Token,
//     weth: Token,
//     wbtc: Token,
//     usdt: Token,
//     dai: Token,
//     usdc: Token,
//     pup: Token,
//     dfyn: Token,
//     bone: Token,
//     route: Token,
//     titan: Token,
// }
// impl Tokens {
//     pub fn new() -> Tokens {
//         Tokens {
//             quick: Token::new("0x831753DD7087CaC61aB5644b308642cc1c33Dc13", "0"),
//             wmatic: Token::new(
//                 "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
//                 "1000000000000000000000",
//             ),
//             weth: Token::new(
//                 "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619",
//                 "500000000000000000",
//             ),
//             wbtc: Token::new("0x1BFD67037B42Cf73acF2047067bd4F2C47D9BfD6", "0"),
//             usdt: Token::new("0xc2132D05D31c914a87C6611C10748AEb04B58e8F", "1000000000"),
//             dai: Token::new("0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063", "0"),
//             usdc: Token::new("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174", "1000000000"),
//             pup: Token::new("0xcFe2cF35D2bDDE84967e67d00aD74237e234CE59", "0"),
//             dfyn: Token::new("0xC168E40227E4ebD8C1caE80F7a55a4F0e6D66C97", "0"),
//             bone: Token::new("0x6bb45cEAC714c52342Ef73ec663479da35934bf7", "0"),
//             route: Token::new("0x16ECCfDbb4eE1A85A33f3A9B21175Cd7Ae753dB4", "0"),
//             titan: Token::new("0xaAa5B9e6c589642f98a1cDA99B9D024B8407285A", "0"),
//         }
//     }
// }

// struct Exchanges {
//     quickswap: Exchange,
//     sushiswap: Exchange,
//     dfyn: Exchange,
//     apeswap: Exchange,
// }
// impl Exchanges {
//     pub fn new() -> Exchanges {
//         Exchanges {
//             quickswap: Exchange::new(
//                 "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff",
//                 30,
//                 Router::Uniswap,
//             ),
//             sushiswap: Exchange::new(
//                 "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506",
//                 30,
//                 Router::Uniswap,
//             ),
//             dfyn: Exchange::new(
//                 "0xA102072A4C07F06EC3B4900FDC4C7B80b6c57429",
//                 30,
//                 Router::Uniswap,
//             ),
//             apeswap: Exchange::new(
//                 "0xC0788A3aD43d79aa53B09c2EaCc313A787d1d607",
//                 30,
//                 Router::Uniswap,
//             ),
//         }
//     }
// }
