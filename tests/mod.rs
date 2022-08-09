use log_gen::logging_gen;

#[logging_gen("{fn_name} message go here", print_macro = "print")]
fn this_was_called(yo: &str) {
    let mut x = 5;
}
