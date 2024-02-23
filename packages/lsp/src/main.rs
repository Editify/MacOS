mod git;

use git::download_repo;
fn main() {
    lsp::Server::new("rust-analyzer");

    match download_repo("eveeifyeve/cli", Some("~/projects/editify/editify")) {
        Ok(result) => {
            println!("{:?}", result);
            // You can use the result here
        },
        Err(e) => {
            println!("Error: {}", e);
            // Handle the error case here
        }
    }
}