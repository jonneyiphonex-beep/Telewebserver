pub struct ServerConfig {
    pub host: &'static str,
    pub port: u16,
}

impl ServerConfig {
    pub fn default() -> Self {
        Self {
            host: "0.0.0.0",
            port: 8080,
        }
    }
}