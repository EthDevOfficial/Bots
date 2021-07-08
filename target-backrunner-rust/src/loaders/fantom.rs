// use std::sync::Arc;

// use crate::helpers::web3::connect_to_node;
// use crate::types::enums::Router;
// use crate::types::immutable_state::ImmutableState;
// use crate::types::token::Token;
// use crate::types::{enums::Chain, exchange::Exchange};

// pub async fn load_immutable_state(ws: &str) -> Arc<ImmutableState> {
//     // let web3 = connect(ws).await.unwrap();
//     let web3 = connect_to_node("ws://52.59.188.80:8546").await.unwrap();

//     let tokens = Tokens::new();
//     let exchanges = Exchanges::new();

//     let outer_tokens = vec![tokens.weth, tokens.wftm];
//     let inner_tokens = vec![tokens.fusd, tokens.fbtc, tokens.aave, tokens.curve];

//     let primary_exchanges = vec![exchanges.spooky];
//     let secondary_exchanges = vec![exchanges.spirit, exchanges.sushi];

//     let ignore_tokens = vec![];

//     Arc::new(ImmutableState::new(
//         Chain::Fantom,
//         web3,
//         primary_exchanges,
//         secondary_exchanges,
//         outer_tokens,
//         inner_tokens,
//         ignore_tokens,
//     ))
// }

// struct Tokens {
//     wftm: Token,
//     weth: Token,
//     usdc: Token,
//     fusd: Token,
//     fbtc: Token,
//     aave: Token,
//     dai: Token,
//     curve: Token,
// }
// impl Tokens {
//     pub fn new() -> Tokens {
//         Tokens {
//             wftm: Token::new(
//                 "0x21be370D5312f44cB42ce377BC9b8a0cEF1A4C83",
//                 "100000000000000000000",
//             ),
//             weth: Token::new(
//                 "0x74b23882a30290451A17c44f4F05243b6b58C76d",
//                 "50000000000000000",
//             ),
//             usdc: Token::new("0x71850b7E9Ee3f13Ab46d67167341E4bDc905Eef9", "100000000"),
//             fusd: Token::new("0x3a97704a1b25F08aa230ae53B352e2e72ef52843", "0"),
//             fbtc: Token::new("0xDDAfbb505ad214D7b80b1f830fcCc89B60fb7A83", "0"),
//             aave: Token::new("0xb7D311E2Eb55F2f68a9440da38e7989210b9A05e", "0"),
//             dai: Token::new("0xb7D311E2Eb55F2f68a9440da38e7989210b9A05e", "0"),
//             curve: Token::new("0xb7D311E2Eb55F2f68a9440da38e7989210b9A05e", "0"),
//         }
//     }
// }

// struct Exchanges {
//     spirit: Exchange,
//     spooky: Exchange,
//     sushi: Exchange,
// }
// impl Exchanges {
//     pub fn new() -> Exchanges {
//         Exchanges {
//             spirit: Exchange::new(
//                 "0x16327E3FbDaCA3bcF7E38F5Af2599D2DDc33aE52",
//                 30,
//                 Router::Uniswap,
//             ),
//             spooky: Exchange::new(
//                 "0xF491e7B69E4244ad4002BC14e878a34207E38c29",
//                 20,
//                 Router::Uniswap,
//             ),
//             sushi: Exchange::new(
//                 "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506",
//                 25,
//                 Router::Uniswap,
//             ),
//         }
//     }
// }
