use std::env;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "macos" {
        env::set_var("CARGO_DEFAULT_RUN", "packages/macos");
    }
}