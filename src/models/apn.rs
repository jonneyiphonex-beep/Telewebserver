use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApnConfig {
    pub name: String,
    pub apn: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    pub os_target: String, // Android, iOS, Windows Mobile
    pub pdn_type: String,  // IPv4, IPv6, IPv4v6
}