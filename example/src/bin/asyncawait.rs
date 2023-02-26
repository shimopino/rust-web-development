// asyncラベルが付与された関数は、コンパイル時に async fn を非同期処理を行うルーチンに変換される
// await 演算子を使用するとルーチンが実行され、バックグラウンドで処理をしている間、スレッドは他の仕事ができる
async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    // この段階では処理を起動しない
    let op = say_world();

    // バックグラウンドで処理を行なっている間に、最初にマクロが実行される
    println!("hello");

    // awaitを呼び出したときに関数の中身が実行される
    op.await;
}
