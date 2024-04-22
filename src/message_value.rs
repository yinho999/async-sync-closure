use serde::{Deserialize, Serialize};

pub trait MessageValueTrait: Sized {
    fn get_value(value: &serde_json::Value) -> Self;
}
