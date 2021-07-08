use crate::types::immutable_state::ImmutableState;
use crate::types::mutable_state::MutableState;
use std::{env, sync::Arc};
// mod fantom;
// mod polygon;
mod xdai;

pub async fn load_immutable_state() -> Arc<ImmutableState> {
    // let chain = env::var("CHAIN").expect("Couldn't read CHAIN from env");
    let chain = "xdai";

    if chain == "xdai" {
        xdai::load_immutable_state().await
    // } else if chain == "polygon" {
    //     polygon::load_immutable_state(ws).await
    // } else if chain == "fantom" {
    //     fantom::load_immutable_state(ws).await
    } else {
        panic!("Loader couldn't interpret the correct chain. Check the env")
    }
}

pub async fn load_mutable_state(immutable_state: Arc<ImmutableState>) -> Arc<MutableState> {
    MutableState::new(immutable_state).await
}
