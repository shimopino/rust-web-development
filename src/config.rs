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
#[derive(Parser, Debug, PartialEq)]
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
    #[clap(long, default_value = "rustwebdev")]
    pub database_password: String,
    #[clap(short, long, default_value = "3030")]
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self, handle_errors::Error> {
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

#[cfg(test)]
mod config_test {
    use super::*;

    fn set_env() {
        env::set_var("BAD_WORDS_API_KEY", "yes");
        env::set_var("PASETO_KEY", "yes");
        env::set_var("POSTGRES_USER", "user");
        env::set_var("POSTGRES_PASSWORD", "pass");
        env::set_var("POSTGRES_HOST", "localhost");
        env::set_var("POSTGRES_PORT", "5432");
        env::set_var("POSTGRES_DB", "rustwebdev");
    }

    #[test]
    fn set_api_key() {
        // 環境変数を設定すると他のテストケースにも影響を与えてしまう
        // 簡単だけど副作用の大きい解決策は `cargo test -- --test-threads=1` で実行して直列実行
        // あるいは2つのテストケースを1つのケースで実行する
        let result = std::panic::catch_unwind(|| Config::new());

        assert!(result.is_err());

        set_env();

        let expected = Config {
            log_level: "warn".to_string(),
            port: 3030,
            database_user: "user".to_string(),
            database_password: "pass".to_string(),
            database_host: "localhost".to_string(),
            database_port: 5432,
            database_name: "rustwebdev".to_string(),
        };

        let config = Config::new().unwrap();

        assert_eq!(config, expected);
    }
}
