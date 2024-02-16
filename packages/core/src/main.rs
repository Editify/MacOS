mod macos;
mod emulator;

fn main() {
    #[cfg(target_os = "macos")]
    macos::main();
}
