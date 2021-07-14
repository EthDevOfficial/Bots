use crate::types::immutable_state::ImmutableState;
use crate::types::mutable_state::MutableState;
use std::{env, sync::Arc};
// mod fantom;
mod polygon;

pub async fn load_immutable_state() -> Arc<ImmutableState> {
    let chain = env::var("CHAIN").unwrap_or("polygon".to_string());

    if chain == "polygon" {
        polygon::load_immutable_state().await
    } else {
        panic!("Loader couldn't interpret the correct chain. Check the env")
    }
}

pub async fn load_mutable_state(immutable_state: &Arc<ImmutableState>) -> Arc<MutableState> {
    MutableState::new(immutable_state).await
}
