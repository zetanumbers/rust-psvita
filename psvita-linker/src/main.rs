fn main() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("PSVITA_LINKER_LOG"))
        .init();

    psvita_linker::run();
}
