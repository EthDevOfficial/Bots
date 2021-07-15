use std::sync::Arc;

use crate::helpers::web3::connect_to_node;
use crate::types::enums::Router;
use crate::types::immutable_state::ImmutableState;
use crate::types::token::Token;
use crate::types::{enums::Chain, exchange::Exchange};

pub async fn load_immutable_state() -> Arc<ImmutableState> {
    let tokens = Tokens::new();
    let exchanges = Exchanges::new();

    let outer_tokens = vec![tokens.wftm.clone()];
    let inner_tokens = vec![
        tokens.fusd,
        tokens.usdc,
        tokens.weth,
        tokens.fbtc,
        tokens.aave,
        tokens.dai,
        tokens.curve,
    ];

    let primary_exchanges = vec![exchanges.spirit, exchanges.spooky];
    let secondary_exchanges = vec![exchanges.sushi];
    let aggregators = vec![];

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
    wftm: Token,
    weth: Token,
    usdc: Token,
    fusd: Token,
    fbtc: Token,
    aave: Token,
    dai: Token,
    curve: Token,
}
impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
            wftm: Token::new("0x21be370D5312f44cB42ce377BC9b8a0cEF1A4C83", 3000, 1, 18),
            weth: Token::new("0x74b23882a30290451A17c44f4F05243b6b58C76d", 5, 10, 18),
            usdc: Token::new("0x04068DA6C83AFCFA0e13ba15A6696662335D5B75", 1000, 1, 6),
            fusd: Token::new("0xAd84341756Bf337f5a0164515b1f6F993D194E1f", 1000, 1, 18),
            fbtc: Token::new("0xe1146b9AC456fCbB60644c36Fd3F868A9072fc6E", 3, 100, 18),
            aave: Token::new("0x6a07A792ab2965C72a5B8088d3a069A7aC3a993B", 3, 1, 18),
            dai: Token::new("0x8D11eC38a3EB5E956B052f67Da8Bdc9bef8Abf3E", 1000, 1, 18),
            curve: Token::new("0x1E4F97b9f9F913c46F1632781732927B9019C68b", 700, 1, 18),
        }
    }
}

struct Exchanges {
    spirit: Exchange,
    spooky: Exchange,
    sushi: Exchange,
}
impl Exchanges {
    pub fn new() -> Exchanges {
        Exchanges {
            spirit: Exchange::new(
                "0x16327E3FbDaCA3bcF7E38F5Af2599D2DDc33aE52",
                30,
                &Router::Uniswap,
            ),
            spooky: Exchange::new(
                "0xF491e7B69E4244ad4002BC14e878a34207E38c29",
                20,
                &Router::Uniswap,
            ),
            sushi: Exchange::new(
                "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506",
                30,
                &Router::Uniswap,
            ),
        }
    }
}
