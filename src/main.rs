use async_sync_closure_testing::message_storage::BoxAsyncFunctionStorage;

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
