use std::env;

// 使用 trait 来优化代码
pub trait FormEnv {
    fn form_env() -> Self;
}

#[derive(Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub env: String,
}

impl FormEnv for Server {
    fn form_env() -> Self {
        let host = env::var("HOST").expect("HOST is not set in .env file");
        let port = env::var("PORT").expect("PORT is not set in .env file");
        let env = env::var("ENV").expect("ENV is not set in .env file");

        Self {
            host,
            port: port.parse::<u16>().unwrap(),
            env,
        }
    }
}

#[derive(Debug)]
pub struct Log {
    pub level: String,
    pub path: String,
    pub filename: String,
}

impl FormEnv for Log {
    fn form_env() -> Self {
        let level = env::var("LEVEL").expect("level is not set in .env file");
        let path = env::var("PATH").expect("path is not set in .env file");
        let filename = env::var("FILENAME").expect("filename is not set in .env file");

        Self {
            level,
            path,
            filename,
        }
    }
}

#[derive(Debug)]
pub struct Db {
    pub debug: bool,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub dbname: String,
}

impl Db {
    pub fn url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}

impl FormEnv for Db {
    fn form_env() -> Self {
        let debug = env::var("DATABASE_DEBUG").expect("DEBUG is not set in .env file");
        let host = env::var("DATABASE_HOST").expect("DATABASE_HOST is not set in .env file");
        let port = env::var("DATABASE_PORT").expect("DATABASE_PORT is not set in .env file");
        let username =
            env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME is not set in .env file");
        let password =
            env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD is not set in .env file");
        let dbname = env::var("DATABASE_DBNAME").expect("DATABASE_DBNAME is not set in .env file");

        Self {
            debug: debug.parse::<bool>().unwrap_or(false),
            host,
            port: port.parse::<u16>().unwrap(),
            username,
            password,
            dbname,
        }
    }
}

#[derive(Debug)]
pub struct Configs {
    pub server: Server,
    pub log: Log,
    pub db: Db,
}

impl FormEnv for Configs {
    fn form_env() -> Self {
        let server = Server::form_env();
        let log = Log::form_env();
        let db = Db::form_env();

        Self { server, log, db }
    }
}
