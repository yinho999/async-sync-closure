use std::future::Future;
use crate::message_value::MessageValueTrait;

// BoxedMessageHandler is a trait object that can be used to store any type that implements MessageHandler
// T is either private::Sync or private::Async
pub(crate) type BoxedMessageHandler<T> = Box<dyn MessageHandler<T>>;

// Create distinct types for sync and async closures for individual implementations
pub mod private {
    #[derive(Debug, Clone, Copy)]
    pub enum Sync {}

    #[derive(Debug, Clone, Copy)]
    pub enum Async {}
}

// MessageHandler is a trait that defines the call method
pub trait MessageHandler<T>: Send + Sync + 'static {
    fn call(&self, value: serde_json::Value);
    #[doc(hidden)]
    fn phantom(&self) -> std::marker::PhantomData<T> {
        std::marker::PhantomData
    }
}

// impl<F, > MessageHandler<private::Sync> for F - F is sync closure. No arguments are passed to the closure
impl<F> MessageHandler<private::Sync> for F
    where
        F: FnOnce() + Send + Sync + Clone + 'static,
{
    fn call(&self, _value: serde_json::Value) {
        (self.clone())();
    }
}

// impl<F, Fut, > MessageHandler<private::Sync> for F - F is async closure. No arguments are passed to the closure
impl<F, Fut> MessageHandler<private::Async> for F
    where
        F: FnOnce() -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output=()> + Send + 'static,
{
    fn call(&self, _value: serde_json::Value) {
        let fut = (self.clone())();
        tokio::spawn(async move {
            fut.await;
        });
    }
}

// implement one or more of the arguments for the sync trait
macro_rules! impl_message_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused)]
        impl<F, $($ty,)* $last> MessageHandler<(private::Sync,  $($ty,)* $last,)> for F
        where
            F: FnOnce($($ty,)* $last,) + Send + Sync + Clone + 'static,
            $( $ty: MessageValueTrait + Send,)*
            $last: MessageValueTrait + Send,
        {
            fn call(&self, value: serde_json::Value) {
                $(
                    let $ty = $ty::get_value(&value);
                )*
                let last = $last::get_value(&value);
                (self.clone())($($ty,)* last);
            }
        }
    };
}

// implement one or more of the arguments for the async trait
macro_rules! impl_async_message_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused)]
        impl<F, $($ty,)* $last, Fut> MessageHandler<(private::Async, $($ty,)* $last,)> for F
        where
            F: FnOnce($($ty,)* $last) -> Fut + Send + Sync + Clone + 'static,
            Fut: Future<Output = ()> + Send + 'static,
            $( $ty: MessageValueTrait + Send,)*
            $last: MessageValueTrait + Send,
        {
            fn call(&self,value: serde_json::Value) {
                $(
                    let $ty = $ty::get_value(&value);
                )*
                let last = $last::get_value(&value);
                let fut = (self.clone())($($ty,)* last);
                tokio::spawn(fut);
            }
        }
    };
}

all_the_tuples!(impl_message_handler);
all_the_tuples!(impl_async_message_handler);