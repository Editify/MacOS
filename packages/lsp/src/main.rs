use std::env;


fn main() {
    let lsp = lsp::Server::new("rs");


    let current_dir = env::current_dir().expect("Failed to get current directory");
    let current_dir_str = current_dir.to_str().expect("Failed to convert path to string");
    let _ = lsp.start_lsp(current_dir_str);
}