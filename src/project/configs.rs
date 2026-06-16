use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16,
    pub env: String,
}

impl App {
    pub fn is_prod(&self) -> bool {
        self.env == "prod"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub level: String,
    pub path: String,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub debug: bool,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwt {
    pub secret: String,
    pub expires_in_hours: u64,
    pub refresh_secret: String,
    pub refresh_expires_in_days: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub db: Option<u16>,
}

impl Redis {
    pub fn url(&self) -> String {
        let base = format!("redis://{}:{}", self.host, self.port);
        if let Some(ref pw) = self.password {
            let encoded =
                percent_encoding::utf8_percent_encode(pw, percent_encoding::NON_ALPHANUMERIC)
                    .to_string();
            let mut url = format!("redis://:{}@{}:{}", encoded, self.host, self.port);
            if let Some(db) = self.db {
                url = format!("{}/{}", url, db);
            }
            url
        } else if let Some(db) = self.db {
            format!("{}/{}", base, db)
        } else {
            base
        }
    }
}

impl Database {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configs {
    pub app: App,
    pub log: Log,
    pub database: Database,
    pub jwt: Jwt,
    pub redis: Redis,
}

impl Configs {
    pub fn new() -> Self {
        let cfg = Config::builder()
            .add_source(File::with_name("config.toml").required(false))
            .build()
            .unwrap_or_else(|e| panic!("配置文件加载失败：{}", e));

        cfg.try_deserialize()
            .unwrap_or_else(|e1| panic!("配置文件转换失败: {}", e1))
    }
}
