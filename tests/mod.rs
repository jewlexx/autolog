use log_gen::logging_gen;

#[logging_gen("{fn_name} message go here with variable {yo}")]
fn this_was_called(yo: &str) {
    let mut x = 5;
}

#[logging_gen]
async fn this_wasnt_called() {}
