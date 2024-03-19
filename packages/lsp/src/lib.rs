use std::{io::{BufRead, Read, Write}, process::{Command, Stdio}};
use serde_json::json;
use anyhow::Result;

pub struct Server {
    lspserver: String,
}


pub(crate) const JSONRPCVERSION: &str = "2.0";

impl Server {
    pub fn new(file_type: &str) -> Self {
        let lspserver = match file_type {
            "rs" => "rust-analyzer".to_string(),
            "ts" => "typescript-language-server".to_string(),
            "c" => "ccls".to_string(),
            "cpp" => "ccls".to_string(),
            _ => String::new(), 
        };
        Self { lspserver }
    }

    pub fn start_lsp(&self, project_path: &str) -> Result<()> {
        let process = Command::new(&self.lspserver)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = process.stdin.unwrap();
        let stdout = process.stdout.unwrap();


        let init_request = json!({
            "jsonrpc": JSONRPCVERSION,
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": format!("file://{}", project_path),
                "capabilities": {
                    "workspaceFolders": true,
                    "textdocument": {
                        "completion": {
                            "snippetSupport": true,
                            "deprecatedSupport": true
                        }
                    }
                }
            }
        });

        let body = serde_json::to_string(&init_request)?;
        let header = format!("Content-Length: {}\r\n\r\n", body.len());
        let req = format!("{}{}", header, body);
        
        stdin.write_all(req.as_bytes())?;
        
        let mut reader = std::io::BufReader::new(stdout);
        let mut content_length = String::new();
        reader.read_line(&mut content_length)?;
        let content_length = content_length.strip_prefix("Content-Length: ").unwrap().trim().parse::<usize>()?;
        
        let mut body = vec![0; content_length];
        reader.read_exact(&mut body)?;
        
        println!("{}", String::from_utf8_lossy(&body));

        Ok(())
    }
}



#[cfg(test)]
mod tests {

    use std::env;

    use super::*;

    #[test]
    fn start_lsp() {
        let lsp = Server::new("rs");


        let current_dir = env::current_dir().expect("Failed to get current directory");
        let current_dir_str = current_dir.to_str().expect("Failed to convert path to string");
        let lsp_result = lsp.start_lsp(current_dir_str);
        assert!(lsp_result.is_ok())
    }
}