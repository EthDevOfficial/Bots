#[derive(PartialEq)]
pub enum Chain {
    Mainnet,
    XDai,
    Polygon,
    Fantom,
}

pub enum Arb {
    Simple,
    Tri,
}
#[derive(Clone, PartialEq)]
pub enum Router {
    Uniswap,
}
