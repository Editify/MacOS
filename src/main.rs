mod macos;

#[cfg(target_os = "macos")]
fn main() {
    macos::main();
}