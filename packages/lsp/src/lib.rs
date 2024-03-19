use std::{fmt::format, io::{BufRead, Read, Write}, process::{Command, Stdio}};
use serde_json::json;
use anyhow::Result;

pub struct Server {
    lspserver: String,
}

impl Server {
    pub fn new(file_type: &str) -> Self {
        let lspserver = match file_type {
            "rs" => "rust-analyzer".to_string(),
            "ts" => "typescript-language-server".to_string(),
            "c" => "ccls".to_string(),
            "cpp" => "ccls".to_string(),
            _ => String::new(), 
        };
        println!("LSP is Type is Set to: {} based on the filetype of {}", lspserver, file_type);
        Self { lspserver }
    }

    pub fn start_lsp(&self, projectPath: &str) -> Result<()> {
        let process = Command::new(&self.lspserver)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = process.stdin.unwrap();
        let stdout = process.stdout.unwrap();

        let init_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": format!("file://{}", projectPath), 
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
        let mut empty_line = String::new();
        reader.read_line(&mut empty_line)?;

        let content_length = content_length.strip_prefix("Content-Length: ").unwrap().trim().parse::<usize>()?;

        let mut body = vec![0; content_length];
        reader.read_exact(&mut body)?;

        println!("{}", String::from_utf8_lossy(&body));

        Ok(())
    }
}