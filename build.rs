use std::env;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "macos" => env::set_var("CARGO_DEFAULT_RUN", "packages/macos"),
        "windows" => env::set_var("CARGO_DEFAULT_RUN", "packages/windows"),
        "linux" => env::set_var("CARGO_DEFAULT_RUN", "packages/linux"),
    }
}
