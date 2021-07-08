use num_bigint::BigUint;
use std::str::FromStr;
use web3::types::H160;

pub struct Token {
    pub address: H160,
    min_trade_amount_wei: num_bigint::BigUint,
}
impl Token {
    pub fn new(address: &str, min_trade_amount_wei: &str) -> Token {
        Token {
            address: H160::from_str(address).unwrap(),
            min_trade_amount_wei: BigUint::from_str(&min_trade_amount_wei).unwrap(),
        }
    }

    pub fn is_above_min_trade_amount(&self, amount: String) -> bool {
        let amount_bn = BigUint::from_str(&amount).unwrap_or_default();
        amount_bn.gt(&self.min_trade_amount_wei)
    }
}
impl Clone for Token {
    fn clone(&self) -> Token {
        Token {
            address: self.address.clone(),
            min_trade_amount_wei: self.min_trade_amount_wei.clone(),
        }
    }
}
