use std::{net::SocketAddr, str::FromStr as _};

use env_vars_config::{env_vars_config, set_env_only};

env_vars_config! {
    SERVER_ADDRESS: SocketAddr = SocketAddr::from_str("0.0.0.0:8080").expect("this is a valid SocketAddr"),
    WORKER_COUNT: i32 = 32,
    OTEL_SERVICE_NAME: String = "test-service",
}

fn main() {
    config::init();

    println!("server address: {}", config::SERVER_ADDRESS.clone());
    println!("worker count: {}", *config::WORKER_COUNT);
    println!("otel service name: {}", config::OTEL_SERVICE_NAME.clone());

    unsafe {
        use config::OTEL_SERVICE_NAME;
        set_env_only!(OTEL_SERVICE_NAME);
    }

    println!(
        "otel service name (from env): {}",
        std::env::var("OTEL_SERVICE_NAME").unwrap()
    );

    assert!(!config::check_values());
}
