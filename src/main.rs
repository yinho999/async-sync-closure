use serde_json::json;
use async_sync_closure_testing::message_handler;
use async_sync_closure_testing::message_storage::{BoxAsyncFunctionStorage, MessageStorage};
use async_sync_closure_testing::message_value::MessageValueTrait;

pub struct MessageValue(serde_json::Value);
impl From<serde_json::Value> for MessageValue {
    fn from(value: serde_json::Value) -> Self {
        MessageValue(value)
    }
}
impl MessageValueTrait for MessageValue {
    fn get_value(value: &serde_json::Value) -> Self {
        MessageValue(value.clone())
    }
}

#[tokio::main]
async fn main() {
    // Create a new BoxAsyncFunctionStorage
    let mut sync_storage = BoxAsyncFunctionStorage::new();
    // Either add a sync message handler
    sync_storage.add_message_handler("hello", || {
        println!("Hello, World!");
    });
    sync_storage.call_message_handler("hello", json!({}));

    let mut async_storage = BoxAsyncFunctionStorage::new();
    // Or add an async message handler
    async_storage.add_message_handler("goodbye", || async move {
        println!("Goodbye, World!");
    });

    async_storage.call_message_handler("goodbye", json!("{}"));
    
    // multiple arguments
    let mut sync_storage = BoxAsyncFunctionStorage::new();
    sync_storage.add_message_handler("hello", |msg_value: MessageValue| {
        println!("Hello, World! {:?}", msg_value.0);
    });
    sync_storage.call_message_handler("hello", json!([5,5]));
}
