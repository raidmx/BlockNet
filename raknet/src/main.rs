use std::time::Duration;
use tokio::time::sleep;
use crate::listener::RakListener;

pub mod packet;
pub mod types;
pub mod conn;
pub mod listener;

#[tokio::main]
async fn main() {
    let listener = RakListener::new("0.0.0.0:19132");
    listener.listen();

    loop {
        sleep(Duration::from_secs(100)).await;
    }
}