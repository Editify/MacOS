use serde::Deserialize;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;


fn read_repo(url: &str) -> io::Result<String> {
    let mut stream = TcpStream::connect("api.github.com:443")?;
    let request = format!("GET {} HTTP/1.1\r\nHost: api.github.com\r\nConnection: close\r\n\r\n", url);
    stream.write_all(request.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("response: {}", response);
    Ok(response)
}

fn parse_json(response: &str) -> Result<String, &'static str> {
    let start = response.find("[").unwrap_or(0);
    let end = response.find("]").unwrap_or_else(|| response.len());
    let json = &response[start..end];
    let releases: Releases = serde_json::from_str(json).map_err(|_| "Failed to parse JSON")?;
    Ok(releases.releases[0].zipball_url.clone())
}

pub fn download_repo(url: &str, dest_path: Option<&str>) -> io::Result<()> {
    let response = read_repo(url)?;
    std::fs::write(dest_path.unwrap_or("$HOME/.editify"), response)?;
    Ok(())
}