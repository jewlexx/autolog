use log_gen::logging_gen;

#[test]
fn test_with_args() {
    #[logging_gen("\"{fn_name}\" was called with variable \"arg1 = {arg1}\"")]
    fn with_args(arg1: &str) {}

    with_args("first argument");
}
