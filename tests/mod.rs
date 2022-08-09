use sourcegen::sourcegen;

#[test]
fn test_with_args() {
    #[sourcegen("\"{fn_name}\" was called with variable \"arg1 = {arg1}\"")]
    fn with_args(arg1: &str) {}

    with_args("first argument");
}

#[test]
#[sourcegen("Function was called without fn name as an arg")]
fn test_no_fn_name() {}

#[test]
fn test_async() {
    #[sourcegen("\"async\" fn was called with variable \"arg1 = {arg1}\"")]
    async fn with_args(arg1: &str) {}
}
