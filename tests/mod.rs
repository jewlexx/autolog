// NOTE: Unfortunately when using tracing, the tests must be run with `-- --test-threads=1` in order for the tracing messages to show up. Otherwise only the first one will

use autolog::autolog;

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

    #[autolog("\"{fn_name}\" was called with variable \"arg1 = {arg1}\"")]
    fn with_args(arg1: &str) {}

    with_args("first arg");
}

#[test]
fn test_no_fn_name() {
    init_sub();

    #[autolog("Function was called without fn name as an arg")]
    fn no_fn_name() {}

    no_fn_name();
}

#[test]
fn test_no_fn_name_args() {
    init_sub();

    #[autolog("Function was called without fn name as an arg, but with args: {arg1}, {arg2}")]
    fn no_fn_name_args(arg1: &str, arg2: &str) {}

    no_fn_name_args("first arg", "second arg");
}

#[test]
fn test_fn_name_no_args() {
    init_sub();

    #[autolog("\"{fn_name}\" was called without args")]
    fn fn_name_no_args() {}

    fn_name_no_args();
}

#[test]
fn test_default_message() {
    init_sub();

    #[autolog]
    fn default_message() {}

    default_message();
}

#[test]
fn test_default_message_args() {
    init_sub();

    #[autolog]
    fn default_message(arg1: &str) {}

    default_message("first arg");
}

#[test]
fn test_async() {
    init_sub();

    #[autolog("\"async\" fn was called with variable \"arg1 = {arg1}\"")]
    async fn with_args(arg1: &str) {}

    smol::block_on(with_args("first arg"));
}
