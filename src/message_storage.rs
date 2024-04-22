use std::collections::HashMap;
use std::sync::RwLock;
use crate::message_handler::{BoxedMessageHandler, MessageHandler};

// BoxAsyncFunctionStorage is a struct that stores a map of message handlers
// The storage is generic over T, which can be either private::Sync or private::Async
pub struct BoxAsyncFunctionStorage<T> {
    message_handlers: RwLock<HashMap<String, BoxedMessageHandler<T>>>,
    _type: std::marker::PhantomData<T>,
}

// Implementations for BoxAsyncFunctionStorage
impl <T>BoxAsyncFunctionStorage<T> where T: Send + Sync + 'static{
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
        self.message_handlers.write().unwrap().insert(name.to_string(), Box::new(handler));
    }

    pub fn call_message_handler(&self, name: &str) {
        let lock = self.message_handlers.read().unwrap();
        let handler = lock.get(name).unwrap();
        handler.call();
    }
}