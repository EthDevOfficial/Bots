use primitive_types::U256;
use std::{cmp::min, str::FromStr};
use web3::types::H160;

pub struct Token {
    pub address: H160,
    min_trade_amount_wei: U256,
}
impl Token {
    // pub fn new(address: &str, min_trade_amount_wei: &str) -> Token {
    //     Token {
    //         address: H160::from_str(address).unwrap(),
    //         min_trade_amount_wei: PrimU256::from_dec_str(min_trade_amount_wei).unwrap(),
    //     }
    // }

    pub fn new(address: &str, min_trade_amount: usize, divisor: usize, decimals: usize) -> Token {
        Token {
            address: H160::from_str(address).unwrap(),
            min_trade_amount_wei: U256::from(min_trade_amount)
                .saturating_mul(U256::exp10(decimals))
                / divisor,
        }
    }

    pub fn above_trade_threshold(&self, amount: &U256) -> bool {
        amount.gt(&self.min_trade_amount_wei)
        // if self.min_trade_amount_wei != U256::zero() {
        //     amount.gt(&self.min_trade_amount_wei)
        // } else {
        //     false
        // }
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
