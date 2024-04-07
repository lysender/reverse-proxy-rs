mod error;
mod run;

use std::process;

use run::run;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "web_server=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    if let Err(e) = run().await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
