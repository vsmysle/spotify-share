use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}