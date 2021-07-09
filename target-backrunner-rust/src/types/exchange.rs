use super::enums::Router;
use crate::helpers::abi;
use ethereum_abi::Abi;
use std::str::FromStr;
use web3::types::H160;
use std::{env, sync::Arc};

pub struct Exchange {
    pub router: H160,
    pub swap_fee: u32,
    pub router_type: Router,
    pub abi: Abi,
}
impl Exchange {
    pub fn new(router: &str, swap_fee: u32, router_type: &Router) -> Exchange {
        let this_file = file!();
        println!("defined in file: {}", this_file);
        let file_path = env::var("ABI_PATH").unwrap_or("./abis/uniswapRouter.json".to_string());
        let abi = match router_type {
            Uniswap => abi::open(&file_path),
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
