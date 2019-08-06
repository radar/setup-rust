use std::process::Command;

pub fn optional_action(command: &str, args: &[&str], success: fn(), fail: fn()) {
    let output = Command::new(command)
                           .args(args)
                           .output()
                           .expect("output");

    if output.status.success() {
        success()
    } else {
        fail()
    }
}
