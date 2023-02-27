use mini_redis::client;

#[tokio::main]
async fn main() {
    // サーバーへのコネクションを確立する
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // 2つのタスクを spawn する。
    // タスク1 はキーによる "get" を行い、// タスク2 は値を "set" する。
    // let t1 = tokio::spawn(async {
    //     let res = client.get("hello").await;
    // });

    // let t2 = tokio::spawn(async {
    //     client.set("foo", "bar".into()).await;
    // });

    // t1.await.unwrap();
    // t2.await.unwrap();
}
