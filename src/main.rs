use matrix_openai_bot::bot;

#[tokio::main]
async fn main() -> Result<(), matrix_sdk::Error> {
    tracing_subscriber::fmt::init();

    bot::start().await?;

    Ok(())
}
