use super::enums::{Router, Router::Uniswap, Router::Firebird};
use crate::helpers::abi;
use ethereum_abi::Abi;
use std::str::FromStr;
use std::{env, sync::Arc};
use web3::types::H160;

pub struct Exchange {
    pub router: H160,
    pub swap_fee: u32,
    pub router_type: Router,
    pub abi: Abi,
}
impl Exchange {
    pub fn new(router: &str, swap_fee: u32, router_type: &Router) -> Exchange {
        let this_file = file!();
        let file_path = env::var("ABI_PATH").unwrap_or("./abis".to_string());
        let abi = match router_type {
            Uniswap => abi::open(&format!("{}/uniswapRouter.json",file_path)),
            Firebird => abi::open(&&format!("{}/firebirdRouter.json",file_path)),
        };

        Exchange {
            router: H160::from_str(router).unwrap(),
            swap_fee,
            router_type: router_type.clone(),
            abi,
        }
    }
}
impl Clone for Exchange {
    fn clone(&self) -> Exchange {
        Exchange {
            router: self.router.clone(),
            swap_fee: self.swap_fee.clone(),
            router_type: self.router_type.clone(),
            abi: self.abi.clone(),
        }
    }
}
