use sourcegen::sourcegen;

#[cfg(feature = "tracing")]
use parking_lot::Mutex;

#[cfg(feature = "tracing")]
static TRACING_ENABLED: Mutex<bool> = Mutex::new(false);

fn init_sub() {
    #[cfg(feature = "tracing")]
    {
        if !*TRACING_ENABLED.lock() {
            *TRACING_ENABLED.lock() = true;

            use tracing::Level;
            use tracing_subscriber::*;

            // a builder for `FmtSubscriber`.
            let subscriber = FmtSubscriber::builder()
                // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
                // will be written to stdout.
                .with_max_level(Level::TRACE)
                // completes the builder.
                .finish();

            tracing::subscriber::set_global_default(subscriber)
                .expect("setting default subscriber failed");
        }
    }
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
    init_sub();

    #[sourcegen("Function was called without fn name as an arg")]
    fn no_fn_name() {}

    no_fn_name();
}

#[test]
fn test_async() {
    init_sub();

    #[sourcegen("\"async\" fn was called with variable \"arg1 = {arg1}\"")]
    async fn with_args(arg1: &str) {}

    smol::block_on(with_args("first argument"));
}
