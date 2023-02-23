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

## リクエスト間でのデータ共有

- `Mutex`
  - 読み取りも書き込みも合わせた排他制御
- `RwLock`
  - 読み取りは共有参照、書き込みは排他制御を行う

## トレーシング

- Span
  - 開始と終了をもつ期間であり、大抵は HTTP リクエストで開始され、HTTP レスポンスを返したときに終了する
  - Span はネスとして設定することも可能
- Event
  - Span の内部で発生しているログ
  - データベースへのクエリ実行結果のレスポンスや各種関数の成功・失敗など
- Subscribers
  - 全てのイベントを収集し、どのように扱うのかを決定する
  - ロガーの初期化の時と同じように設定する

## 最初のテーブル

今回の型に従うテーブル定義を作成する

```sql
-- 解答テーブル
CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 質問テーブル
CREATE TABLE IF NOT EXISTS answers (
    id serial PRIMARY KEY,
    content TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    corresponding_question integer REFERENCES questions
);
```

マイグレーションファイルを初期化することで、マイグレーションを適用する時とロールバックするときに動かす処理を記載する

```bash
$ sqlx migrate add -r questions_table
$ sqlx migrate add -r answers_table
```

## 外部 API

apilayer の bad words api を使用する

```bash
$ curl --request POST \
  --url 'https://api.apilayer.com/bad_words?censor_charactor=*' \
  --header 'apikey: 35CID42ET40KTouKLhGPGRdBYEEhWr4d' \
  --data-raw '{ "title": "fuck" }'

{
  "content": "{ \"title\": \"fuck\" }",
  "bad_words_total": 1,
  "bad_words_list": [
    {
      "original": "fuck",
      "word": "fuck",
      "deviations": 0,
      "info": 2,
      "start": 9,
      "end": 13,
      "replacedLen": 4
    }
  ],
  "censored_content": "{ \"title\": \"****\" }"}
```

## 外部ライブラリの検証

[Secure Rust Guidelines](https://anssi-fr.github.io/rust-guide/04_language.html)

```bash
# https://github.com/crev-dev/cargo-crev
$ cargo install cargo-crev
$ cargo crev trust --level high https://github.com/dpc/crev-proofs
$ cargo crev repo fetch all

# 検証を実施
$ cargo crev verify --show-all

# 個別のライブラリの詳細を検証
cargo crev repo query issue <crate> <version>
```

```bash
$ cargo install cargo-audit
$ cargo audit
```
