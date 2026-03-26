/// Initialize tracing subscriber using `BADAL_LOG` env var (defaults to `warn`).
pub fn init() {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_env("BADAL_LOG").unwrap_or_else(|_| EnvFilter::new("warn"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
