use std::env;

use env_vars_config::env_vars_config;

env_vars_config! {
    SERVER_ADDRESS: String = "0.0.0.0:8080",
    WORKERS_COUNT: i32 = 32,
    OTEL_SERVICE_NAME: String = "test-service",
}

fn main() {
    println!("server address: {}", config::SERVER_ADDRESS.as_str());
    println!("workers count: {}", config::WORKERS_COUNT.clone());
    println!("otel service name: {}", config::OTEL_SERVICE_NAME.clone());
    println!(
        "otel service name (from env): {}",
        env::var("OTEL_SERVICE_NAME").unwrap()
    );
}
