async fn say_world() {
    println!("world");
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        println!("start");
        let op = say_world();
        println!("initialized");
        op.await;
        println!("end");
    })
}
