mod app;

use tokio::runtime::Runtime;

use crate::app::{start_bevy_app, start_http_server};

/// Maximum of players can play concurrently.
pub const SERVER_CAPACITY: usize = 5500;

#[tokio::main]
async fn main() {
    // Start the HTTP server in a separate thread
    std::thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            start_http_server().await;
        });
    });

    start_bevy_app();
}
