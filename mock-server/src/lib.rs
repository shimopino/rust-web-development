use bytes::Bytes;
use serde_json::json;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::oneshot::{self, Sender};
use warp::{http, Filter, Reply};

#[derive(Debug, Clone)]
pub struct MockServer {
    socket: SocketAddr,
}

pub struct OneshotHandler {
    pub sender: Sender<i32>,
}

impl MockServer {
    pub fn new(bind_addr: SocketAddr) -> MockServer {
        MockServer { socket: bind_addr }
    }

    async fn check_profanity(
        _: (),
        content: Bytes,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let content =
            String::from_utf8(content.to_vec()).expect("Invalid UTF-8");

        if content.contains("shitty") {
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({
                    "bad_words_list": [
                    {
                        "deviations": 0,
                        "end": 16,
                        "info": 2,
                        "original": "shitty",
                        "replacedLen": 6,
                        "start": 10,
                        "word": "shitty"
                    }
                    ],
                    "bad_words_total": 1,
                    "censored_content": "this is a ****** sentence",
                    "content": "this is a shitty sentence"
                })),
                http::StatusCode::OK,
            ))
        } else {
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({
                    "bad_words_list": [],
                    "bad_words_total": 0,
                    "censored_content": "",
                    "content": "this is a sentence"
                })),
                http::StatusCode::OK,
            ))
        }
    }

    /// 下記のAPIルートをモックする
    /// /bad_words?censor_ character={{censor_character}}
    ///
    /// クエリパラメータを受け取っても内容に応じて処理を分けることなどはしないため、
    /// 受け取ったクエリパラメータに対しては何もしない
    fn build_routes(&self) -> impl Filter<Extract = impl Reply> + Clone {
        warp::post()
            .and(warp::path("bad_words"))
            .and(warp::query())
            .map(|_: HashMap<String, String>| ())
            .and(warp::path::end())
            .and(warp::body::bytes())
            .and_then(Self::check_profanity)
    }

    /// チャネル機能を使用してテスト起動側から、モックサーバーをダウンできる
    pub fn oneshot(&self) -> OneshotHandler {
        // oneshotを使用することで、一方通行のチャネルを提供することができる
        // ある非同期タスクから、別の非同期タスクにメッセージを送信することができる
        // 通常は、あるタスクが実行される前に別のタスクが終了するまで待機するような状況で使用される
        // テスト起動側からトリガーして、モックサーバーを停止させる
        let (tx, rx) = oneshot::channel::<i32>();
        let routes = Self::build_routes(&self);

        let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(
            self.socket,
            async {
                rx.await.ok();
            },
        );

        tokio::task::spawn(server);

        // メッセージを送信することで、安全なシャットダウンを発火する
        OneshotHandler { sender: tx }
    }
}
