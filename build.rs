use std::process::Command;

fn main() {
    println!("cargo:rustc-env=GIT_HASH={}", "");
    match Command::new("echo").args(&["$GITHUB_SHA"]).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(hash) => println!("cargo:rustc-env=GIT_HASH={}", hash),
            Err(_) => (),
        },
        Err(_) => (),
    };
}
