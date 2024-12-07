use std::env;
use reqwest::Error;

pub async fn get_input(year: &str, day: &str) -> Result<String, Error> {
    let session = env::var("SESSION")
        .expect("Please specify the session cookie");
    let cookie = format!("session={}", session);
    let client = reqwest::Client::builder()
        .build()?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let response = client
        .get(url)
        .header("Cookie", cookie)
        .send()
        .await?;
    let input = response
        .text()
        .await?;
    Ok(input)
}


