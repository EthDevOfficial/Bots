use std::sync::Arc;

use crate::helpers::web3::connect_to_node;
use crate::types::enums::Router;
use crate::types::immutable_state::ImmutableState;
use crate::types::token::Token;
use crate::types::{enums::Chain, exchange::Exchange};

pub async fn load_immutable_state() -> Arc<ImmutableState> {
    let tokens = Tokens::new();
    let exchanges = Exchanges::new();

    let outer_tokens = vec![tokens.wmatic.clone()];
    let inner_tokens = vec![
        tokens.pwings,
        tokens.weth,
        tokens.wmatic,
        tokens.bone,
        tokens.usdc,
        tokens.pup,
    ];

    let primary_exchanges = vec![exchanges.quickswap, exchanges.jetswap, exchanges.sushiswap];
    let secondary_exchanges = vec![exchanges.dfyn, exchanges.apeswap];
    let aggregators = vec![exchanges.firebird];

    let ignore_tokens = vec![];

    Arc::new(
        ImmutableState::new(
            Chain::Polygon,
            primary_exchanges,
            secondary_exchanges,
            aggregators,
            outer_tokens,
            inner_tokens,
            ignore_tokens,
        )
        .await,
    )
}

struct Tokens {
    quick: Token,
    wmatic: Token,
    weth: Token,
    pwings: Token,
    wbtc: Token,
    usdt: Token,
    dai: Token,
    usdc: Token,
    pup: Token,
    dfyn: Token,
    bone: Token,
    route: Token,
    titan: Token,
}
impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
            wmatic: Token::new("0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270", 1000, 1, 18),
            weth: Token::new("0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619", 5, 10, 18),
            pwings: Token::new("0x845e76a8691423fbc4ecb8dd77556cb61c09ee25", 10000, 1, 18),
            quick: Token::new("0x831753DD7087CaC61aB5644b308642cc1c33Dc13", 3, 1, 18),
            wbtc: Token::new("0x1BFD67037B42Cf73acF2047067bd4F2C47D9BfD6", 3, 100, 8),
            usdt: Token::new("0xc2132D05D31c914a87C6611C10748AEb04B58e8F", 1000, 1, 6),
            dai: Token::new("0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063", 0, 1, 18),
            usdc: Token::new("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174", 1000, 1, 6),
            pup: Token::new("0xcFe2cF35D2bDDE84967e67d00aD74237e234CE59", 20, 1, 18),
            dfyn: Token::new("0xC168E40227E4ebD8C1caE80F7a55a4F0e6D66C97", 500, 1, 18),
            bone: Token::new("0x6bb45cEAC714c52342Ef73ec663479da35934bf7", 20, 1, 18),
            route: Token::new("0x16ECCfDbb4eE1A85A33f3A9B21175Cd7Ae753dB4", 100, 1, 18),
            titan: Token::new(
                "0xaAa5B9e6c589642f98a1cDA99B9D024B8407285A",
                100000000,
                1,
                18,
            ),
        }
    }
}

struct Exchanges {
    quickswap: Exchange,
    sushiswap: Exchange,
    dfyn: Exchange,
    apeswap: Exchange,
    jetswap: Exchange,
    firebird: Exchange,
}
impl Exchanges {
    pub fn new() -> Exchanges {
        Exchanges {
            quickswap: Exchange::new(
                "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff",
                30,
                &Router::Uniswap,
            ),
            sushiswap: Exchange::new(
                "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506",
                30,
                &Router::Uniswap,
            ),
            dfyn: Exchange::new(
                "0xA102072A4C07F06EC3B4900FDC4C7B80b6c57429",
                30,
                &Router::Dfyn,
            ),
            apeswap: Exchange::new(
                "0xC0788A3aD43d79aa53B09c2EaCc313A787d1d607",
                20,
                &Router::Uniswap,
            ),
            jetswap: Exchange::new(
                "0x5C6EC38fb0e2609672BDf628B1fD605A523E5923",
                10,
                &Router::Uniswap,
            ),
            firebird: Exchange::new(
                "0xF6fa9Ea1f64f1BBfA8d71f7f43fAF6D45520bfac",
                0,
                &Router::Firebird,
            ),
        }
    }
}
