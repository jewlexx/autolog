use log_gen::logging_gen;

#[test]
#[logging_gen(yo = "message slay")]
fn this_was_called() {
    let mut x = 5;
}
