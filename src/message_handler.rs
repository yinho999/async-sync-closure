use std::future::Future;

// Create distinct types for sync and async closures for individual implementations
mod private {
    #[derive(Debug, Clone, Copy)]
    pub enum Sync {}

    #[derive(Debug, Clone, Copy)]
    pub enum Async {}
}

// MessageHandler is a trait that defines the call method
pub trait MessageHandler<T>: Send + Sync + 'static {
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

