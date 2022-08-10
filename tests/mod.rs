use sourcegen::sourcegen;

fn init_sub() {
    use tracing::Level;
    use tracing_subscriber::*;

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[test]
fn test_with_args() {
    init_sub();

    #[sourcegen("\"{fn_name}\" was called with variable \"arg1 = {arg1}\"")]
    fn with_args(arg1: &str) {}

    with_args("first argument");
}

#[test]
fn test_no_fn_name() {
    #[sourcegen("Function was called without fn name as an arg")]
    fn no_fn_name() {}

    no_fn_name();
}

#[test]
fn test_async() {
    #[sourcegen("\"async\" fn was called with variable \"arg1 = {arg1}\"")]
    async fn with_args(arg1: &str) {}

    with_args("first argument");
}
