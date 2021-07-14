use ethereum_abi::{Abi, DecodedParams, Function};
use std::fs::File;

pub fn open(file: &str) -> Abi {
    let file = File::open(file).expect("failed to open ABI file");
    Abi::from_reader(file).expect("failed to parse ABI")
}

pub fn decode<'a>(encoded_param: &Vec<u8>, abi: &'a Abi) -> Option<(&'a Function, DecodedParams)> {
    match abi.decode_input_from_slice(encoded_param) {
        Ok(abi) => Some(abi),
        _ => None,
    }
}
