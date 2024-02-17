mod emulator;
mod macos;

fn main() {
    #[cfg(target_os = "macos")]
    macos::main();
}
