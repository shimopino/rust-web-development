use dotenv::dotenv;
use rust_web_dev::{config, run, setup_store};

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv().ok();

    let config = config::Config::new().expect("Config can't be set");
    let store = setup_store(&config).await?;

    tracing::info!(
        "Q&A service build ID {}",
        env!("RUST_WEB_DEV_VERSION")
    );

    run(config, store).await;

    Ok(())
}