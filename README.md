# Simple Clap Logger

A simple cli logger, which aims to mimic the [clap](https://github.com/clap-rs/clap) format of error reporting in order to create a seamless cli experience without formatting inconsistencies.

![example][./example.PNG]

## Getting started
```rust
use simple_clap_logger::{Logger, info};
use log::Level;

// Initialize the logger with Info logging level
Logger::init_with_level(Level::Info);

// Start logging...
info!("Some info");
```