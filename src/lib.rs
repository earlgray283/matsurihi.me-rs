use once_cell::sync::Lazy;
use reqwest::Client;
use tokio::sync::Mutex;

pub mod card;
pub mod idol;
pub mod model;

static HTTP_CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| {
    let client = reqwest::ClientBuilder::default()
        .build()
        .expect("http_client building error");
    Mutex::new(client)
});
