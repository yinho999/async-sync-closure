use std::collections::HashMap;
use std::future::Future;
use std::sync::RwLock;

// Create distinct types for sync and async closures for individual implementations
mod private {
    #[derive(Debug, Clone, Copy)]
    pub enum Sync {}

    #[derive(Debug, Clone, Copy)]
    pub enum Async {}
}

// MessageHandler is a trait that defines the call method
trait MessageHandler<T>: Send + Sync + 'static {
    fn call(&self);
    #[doc(hidden)]
    fn phantom(&self) -> std::marker::PhantomData<T> {
        std::marker::PhantomData
    }
}

// impl<F, > MessageHandler<private::Sync> for F - F is sync closure
impl<F, > MessageHandler<private::Sync> for F
    where
        F: FnOnce() + Send + Sync + Clone + 'static,
{
    fn call(&self) {
        (self.clone())();
    }
}

// impl<F, Fut, > MessageHandler<private::Sync> for F - F is async closure
impl<F, Fut, > MessageHandler<private::Async> for F
    where
        F: FnOnce() -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output=()> + Send + 'static,
{
    fn call(&self) {
        let fut = (self.clone())();
        tokio::spawn(async move {
            fut.await;
        });
    }
}

// BoxedMessageHandler is a trait object that can be used to store any type that implements MessageHandler
// T is either private::Sync or private::Async
pub(crate) type BoxedMessageHandler<T> = Box<dyn MessageHandler<T>>;

// BoxAsyncFunctionStorage is a struct that stores a map of message handlers
// The storage is generic over T, which can be either private::Sync or private::Async
struct BoxAsyncFunctionStorage<T> {
    message_handlers: RwLock<HashMap<String, BoxedMessageHandler<T>>>,
    _type: std::marker::PhantomData<T>,
}

// Implementations for BoxAsyncFunctionStorage
impl <T>BoxAsyncFunctionStorage<T> where T: Send + Sync + 'static{
    fn new() -> Self {
        Self {
            message_handlers: Default::default(),
            _type: std::marker::PhantomData,
        }
    }

    fn add_message_handler<H>(&mut self, name: &str, handler: H)
        where
            H: MessageHandler<T> + Send + Sync + Clone + 'static,
    {
        self.message_handlers.write().unwrap().insert(name.to_string(), Box::new(handler));
    }

    fn call_message_handler(&self, name: &str) {
        let lock = self.message_handlers.read().unwrap();
        let handler = lock.get(name).unwrap();
        handler.call();
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
    sync_storage.call_message_handler("hello");
    
    let mut async_storage = BoxAsyncFunctionStorage::new();
    // Or add an async message handler
    async_storage.add_message_handler("goodbye", || async move {
        println!("Goodbye, World!");
    });
    async_storage.call_message_handler("goodbye");
}