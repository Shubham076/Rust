use std::time::Duration;
use tokio::time::sleep;
use futures::{executor, future, FutureExt};

async fn say() {
    println!("hello");
    sleep(Duration::from_secs(3)).await;
    println!("Woke up!");
}

async fn greet() {
    println!("world");
}
pub async fn run() {
    //Boxing homogenizes them, allowing you to store different types of futures in the same collection.
     let futures = vec![say().boxed(), greet().boxed()];

    // takes an iterator of futures and returns a new future that completes when all of the futures in the iterator have completed
    future::join_all(futures).await;
}