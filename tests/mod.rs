use log_gen::logging_gen;

#[test]
fn test_with_args() {
    #[logging_gen("\"{fn_name}\" was called with variable \"arg1 = {arg1}\"")]
    fn with_args(arg1: &str) {}

    with_args("first argument");
}

#[test]
#[logging_gen("Function was called without fn name as an arg")]
fn test_no_fn_name() {}

#[test]
fn test_async() {
    #[logging_gen("\"async\" fn was called with variable \"arg1 = {arg1}\"")]
    async fn with_args(arg1: &str) {}
}
