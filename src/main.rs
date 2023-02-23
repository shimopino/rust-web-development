#![warn(clippy::all)]

use clap::Parser;
use handle_errors::return_error;
use std::env;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

mod profanity;
mod routes;
mod store;
mod types;

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
struct Args {
    #[clap(short, long, default_value = "warn")]
    log_level: String,
    #[clap(long, default_value = "localhost")]
    database_host: String,
    #[clap(long, default_value = "5432")]
    database_port: u16,
    #[clap(long, default_value = "rustwebdev")]
    database_name: String,
    #[clap(long, default_value = "rustwebdev")]
    database_user: String,
    #[clap(long, default_value = "rustwebdev")]
    database_password: String,
}

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv::dotenv().ok();

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

    let args = Args::parse();

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rust-web-dev={},warp={}",
            args.log_level, args.log_level, args.log_level
        )
    });

    let store = store::Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        args.database_user,
        args.database_password,
        args.database_host,
        args.database_port,
        args.database_name
    ))
    .await
    .map_err(handle_errors::Error::DatabaseQueryError)?;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .map_err(handle_errors::Error::MigrationError)?;

    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // どのトレースを保存するのかを決定する
        .with_env_filter(log_filter)
        // イベントはSpanが終了したことも併せて保存する
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::PUT,
            Method::DELETE,
            Method::GET,
            Method::POST,
        ]);

    // ルートハンドラー
    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    // 質問周りのハンドラー
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    tracing::info!(
        "Q&A service build ID {}",
        env!("RUST_WEB_DEV_VERSION")
    );

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    Ok(())
}
