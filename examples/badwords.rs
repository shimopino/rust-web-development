#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let api_key = std::env::var("API_KEY")?;

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_charactor=*")
        .header("apikey", api_key)
        .body("a list with shit words")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", res);

    Ok(())
}
