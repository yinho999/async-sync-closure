use crate::message_handler::{BoxedMessageHandler, MessageHandler, private};
use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard};
use std::future::Future;
use crate::message_value::MessageValueTrait;

// BoxAsyncFunctionStorage is a struct that stores a map of message handlers
// The storage is generic over T, which can be either private::Sync or private::Async
pub struct BoxAsyncFunctionStorage<T> {
    message_handlers: RwLock<HashMap<String, BoxedMessageHandler<T>>>,
    _type: std::marker::PhantomData<T>,
}

pub trait MessageStorage<T> {
    fn get_message_handler(&self) -> RwLockReadGuard<HashMap<String,BoxedMessageHandler<T>>>;
    fn call_message_handler(&self, name: &str, value: serde_json::Value);
}

// Implementations for BoxAsyncFunctionStorage
impl<T> BoxAsyncFunctionStorage<T>
where
    T: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            message_handlers: Default::default(),
            _type: std::marker::PhantomData,
        }
    }

    pub fn add_message_handler<H>(&mut self, name: &str, handler: H)
    where
        H: MessageHandler<T> + Send + Sync + Clone + 'static,
    {
        self.message_handlers
            .write()
            .unwrap()
            .insert(name.to_string(), Box::new(handler));
    }

}

// Call the message handler without any arguments
impl <T> MessageStorage<T> for BoxAsyncFunctionStorage<T> where
    T: Send + Sync + 'static, {
    fn get_message_handler(&self) ->RwLockReadGuard<HashMap<String,BoxedMessageHandler<T>>> {
        self.message_handlers.read().unwrap()
    }

    fn call_message_handler(&self, name: &str, value: serde_json::Value) {
        let hash_map = self.get_message_handler();
        let handler = hash_map.get(name).unwrap();
        handler.call(value);
    }
}
