use std::env;

use clap::Parser;

/// clapを使用して起動時に引数でパラメータを指定できる
///
/// ```bash
/// $ cargo run --bin rust-web-dev \
///     --database-host localhost \
///     --log-level info \
///     --database-name rustwebdev
/// ```
///
/// バイナリ起動時にも引数として設定することが可能
///
/// ```bash
/// $ ./target/debug/rustwebdev \
///     --database-host localhost \
///     --log-level info \
///     --database-name rustwebdev
/// ```
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    #[clap(long, default_value = "localhost")]
    pub database_host: String,
    #[clap(long, default_value = "5432")]
    pub database_port: u16,
    #[clap(long, default_value = "rustwebdev")]
    pub database_name: String,
    #[clap(long, default_value = "rustwebdev")]
    pub database_user: String,
    #[clap(long)]
    pub database_password: String,
    #[clap(short, long, default_value = "3030")]
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self, handle_errors::Error> {
        dotenv::dotenv().ok();
        let config = Config::parse();

        if env::var("BAD_WORDS_API_KEY").is_err() {
            panic!("BadWords API KEYが設定されていない");
        }

        if env::var("PASETO_KEY").is_err() {
            panic!("PASETO KEYが設定されていない");
        }

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(3030))
            .map_err(handle_errors::Error::ParseError)?;

        let db_user = env::var("POSTGRES_USER")
            .unwrap_or(config.database_user.to_owned());
        let db_host = env::var("POSTGRES_HOST")
            .unwrap_or(config.database_host.to_owned());
        let db_port = env::var("POSTGRES_PORT")
            .unwrap_or(config.database_port.to_string());
        let db_name = env::var("POSTGRES_DB")
            .unwrap_or(config.database_name.to_owned());
        let db_password = env::var("POSTGRES_PASSWORD").unwrap();

        Ok(Config {
            log_level: config.log_level,
            database_user: db_user,
            database_password: db_password,
            database_host: db_host,
            database_name: db_name,
            database_port: db_port
                .parse::<u16>()
                .map_err(handle_errors::Error::ParseError)?,
            port,
        })
    }
}
