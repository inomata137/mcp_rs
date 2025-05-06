#[cfg(debug_assertions)]
const LOG_LEVEL: &str = "debug";
#[cfg(not(debug_assertions))]
const LOG_LEVEL: &str = "info";

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let env_filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| LOG_LEVEL.into());

    let log_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .truncate(false)
        .open("log.txt")?;

    let subscriber = tracing_subscriber::fmt::layer()
        .with_writer(log_file)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}
