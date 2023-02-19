# Rust Web Development

## Web フレームワーク: Warp

どのような Web フレームワークであっても、下記のような操作をおこなっているため、対応する質問に対して該当フレームワークがどのように対処しているのか把握する必要がある

- HTTP リクエストからパスとメソッドに合致するハンドラ関数を決定
  - Q: どのように PATH と HTTP メソッドを解析しているのか？
- 共通処理として指定されているミドルウェアを実行
  - HTTP ボディから直接 JSON リクエストとしてパースできるのか
- 個別のルートハンドラを実行
  - リクエストの URI パラメータを解析できるのか
  - セッションや Cookie を導入できるのか
  - データベース接続のようなオブジェクトをルートハンドラに渡せるのか
- HTTP リクエストを返す
  - 具体的なレスポンスボディを返すにはどうすればいいのか

## HashMap

HashMap のキーに自身で作成した型を使用する際は、カスタム構造体に対して比較のための `PartialEq, Eq, Hash` を導入する必要がある

今回のように `QuestionId` は元々は `String` なので以下のように実装することが可能となる

```rs
#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct QuestionId(String);
```

## データストアの渡し方

`warp` では `any` を使用することでどのパスにもマッチするハンドラを作成することができ、 `map` で関数を実行したときにどのような処理を行うのかを指定することができる

```rust
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)       // 挟みたい参照値を追加する
        .and_then(get_questions) // 1つ先の入力の引数となる
        .recover(return_error);
```
