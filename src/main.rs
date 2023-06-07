use tokio;

mod data_objects;
mod errors;
mod filesystem;
mod traits;

use data_objects::sentence::Sentence;

#[tokio::main]
async fn main() {
    let s = Sentence::new("huhuhu".to_owned());
    println!("Hello, world! {}", s);
}
