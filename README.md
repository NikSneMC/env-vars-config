env-vars-config
===

A simple lib for configuring your applications via environment variables.

[![Build status](https://img.shields.io/github/actions/workflow/status/NikSneMC/env-vars-config/ci.yml?branch=master)](https://github.com/NikSneMC/env-vars-config/actions)
[![Latest version](https://img.shields.io/crates/v/env-vars-config.svg)](https://crates.io/crates/env-vars-config)
[![Documentation](https://docs.rs/env-vars-config/badge.svg)](https://docs.rs/env-vars-config)
![License](https://img.shields.io/crates/l/env-vars-config.svg)

* [`env-vars-config` documentation](https://docs.rs/env-vars-config)

## Minimum supported `rustc`

`1.80.0+`

This version is explicitly tested in CI and may be bumped in any release as needed. Maintaining compatibility with older compilers is a priority though, so the bar for bumping the minimum supported version is set very high. Any changes to the supported minimum version will be called out in the release notes.

## Usage

```toml
[dependencies]
env-vars-config = "0.5"
```

```rust
use std::env;

use env_vars_config::{env_vars_config, set_env_only};

env_vars_config! {
    SERVER_ADDRESS: String = "0.0.0.0:8080",
    WORKERS_COUNT: i32 = 32,
    OTEL_SERVICE_NAME: String = "test-service",
}

fn main() {
    config::init();

    println!("server address: {}", config::SERVER_ADDRESS.as_str());
    println!("workers count: {}", config::WORKERS_COUNT.clone());
    println!("otel service name: {}", config::OTEL_SERVICE_NAME.clone());

    unsafe {
        use config::OTEL_SERVICE_NAME;
        set_env_only!(OTEL_SERVICE_NAME);
    }

    println!(
        "otel service name (from env): {}",
        env::var("OTEL_SERVICE_NAME").unwrap()
    );

    config::test_values();
}

```

