use log_gen::logging_gen;

#[test]
fn test_with_args() {
    #[logging_gen("{fn_name} message go here with variable {arg1}")]
    fn with_args(arg1: &str) {}

    with_args("arg1");
}
