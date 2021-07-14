use crate::types::enums::Router;
use crate::types::immutable_state::ImmutableState;
use crate::types::token::Token;
use crate::types::{enums::Chain, exchange::Exchange};
use std::sync::Arc;

pub async fn load_immutable_state() -> Arc<ImmutableState> {
    let tokens = Tokens::new();
    let exchanges = Exchanges::new();

    let outer_tokens = vec![tokens.wxdai.clone()];
    let inner_tokens = vec![
        tokens.usdc,
        tokens.weth,
        tokens.hny,
        tokens.stake,
        tokens.agve,
    ];

    let primary_exchanges = vec![exchanges.honeyswap];
    let secondary_exchanges = vec![exchanges.swapper, exchanges.baoswap];
    let aggregators = vec![];

    let ignore_tokens = vec![
        "0x4609e9b9c2912dd5b954cbf3a5d7d89ab6c8979d",
        "0x43bf77e8c21b0A57774fedD90Ca8791B58C457D1",
        "0xec07b6E321014B3093101C8296944a7C56354B3f",
    ];

    Arc::new(
        ImmutableState::new(
            Chain::XDai,
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
    wxdai: Token,
    weth: Token,
    hny: Token,
    agve: Token,
    usdc: Token,
    stake: Token,
}
impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
            wxdai: Token::new(
                "0xe91D153E0b41518A2Ce8Dd3D7944Fa863463a97d",
                100, // amount
                1,   // divisor
                18,  // decimals
            ),
            weth: Token::new("0x6A023CCd1ff6F2045C3309768eAd9E68F978f6e1", 5, 100, 18),
            hny: Token::new("0x71850b7E9Ee3f13Ab46d67167341E4bDc905Eef9", 3, 10, 18), // .3
            agve: Token::new("0x3a97704a1b25F08aa230ae53B352e2e72ef52843", 5, 10, 18), // .5
            usdc: Token::new("0xDDAfbb505ad214D7b80b1f830fcCc89B60fb7A83", 100, 1, 6), // 100
            stake: Token::new("0xb7D311E2Eb55F2f68a9440da38e7989210b9A05e", 13, 1, 18), // 13
        }
    }
}

struct Exchanges {
    baoswap: Exchange,
    honeyswap: Exchange,
    swapper: Exchange,
}
impl Exchanges {
    pub fn new() -> Exchanges {
        Exchanges {
            baoswap: Exchange::new(
                "0x6093AeBAC87d62b1A5a4cEec91204e35020E38bE",
                30,
                &Router::Uniswap,
            ),
            honeyswap: Exchange::new(
                "0x1C232F01118CB8B424793ae03F870aa7D0ac7f77",
                30,
                &Router::Uniswap,
            ),
            swapper: Exchange::new(
                "0xE43e60736b1cb4a75ad25240E2f9a62Bff65c0C0",
                25,
                &Router::Uniswap,
            ),
        }
    }
}
