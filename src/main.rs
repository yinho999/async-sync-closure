use async_sync_closure_testing::message_handler;
use async_sync_closure_testing::message_storage::{BoxAsyncFunctionStorage, MessageStorage};
use async_sync_closure_testing::message_value::MessageValueTrait;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageValue {
    x: i32,
    y: u32,
}

impl From<serde_json::Value> for MessageValue {
    fn from(value: serde_json::Value) -> Self {
        let message_value = value.get("message_value").unwrap();
        MessageValue::deserialize(message_value).unwrap()
    }
}
impl MessageValueTrait for MessageValue {
    fn get_value(value: &serde_json::Value) -> Self {
        MessageValue::from(value.clone())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SocketMessage {
    message: String,
}

impl From<serde_json::Value> for SocketMessage {
    fn from(value: serde_json::Value) -> Self {
        let socket_message = value.get("socket_message").unwrap();
        SocketMessage::deserialize(socket_message).unwrap()
    }
}

impl MessageValueTrait for SocketMessage {
    fn get_value(value: &serde_json::Value) -> Self {
        SocketMessage::from(value.clone())
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
    sync_storage.add_message_handler(
        "hello",
        |msg_value: MessageValue, socket_message: SocketMessage| {
            println!("Message value! {}", msg_value.x);
            println!("Message value! {}", msg_value.y);

            println!("Socket message! {}", socket_message.message);
        },
    );
    sync_storage.call_message_handler(
        "hello",
        json!( {
            "message_value": {
                "x": 1,
                "y": 2
            },
            "socket_message": {
                "message": "hello"
            }
        }),
    );
}
