

pub struct Server {
    name: String,
}


impl Server {
    pub fn new(name: &str) -> Self {
        
        Self { name: name.to_string() }
    }
} 