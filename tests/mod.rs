use log_gen::logging_gen;

#[test]
#[logging_gen("{fn_name} message go here")]
fn this_was_called() {
    let mut x = 5;
}
