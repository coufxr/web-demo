use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16,
    pub env: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub level: String,
    pub path: String,
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub debug: bool,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub dbname: String,
}

impl Database {
    pub fn url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    pub app: App,
    pub log: Log,
    pub database: Database,
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
