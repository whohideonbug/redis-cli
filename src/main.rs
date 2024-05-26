use clap::Parser;
use redis_wrapper::RedisClient;

#[derive(Parser, Debug, Clone)]
struct CLI {
    /// The address of redis server to connect
    #[clap(long, default_value = "redis://localhost:6379")]
    pub redis: String,
}

#[tokio::main]
async fn main() {
    let CLI { redis } = CLI::parse();
    let client = RedisClient::connect(&redis, "db_name").unwrap();
    let _ = client.set_str("Alice", "100", 30).await;
    let get_result = client.get_str("Alice").await.unwrap();
    println!("Alice's score is {}", get_result);
}

