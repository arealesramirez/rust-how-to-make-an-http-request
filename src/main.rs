use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CatFact {
    fact: String,
    length: i32,
}

#[derive(Deserialize, Debug)]
struct DifferentCatFact {
    fact: String,
    length: i32,
    id: String,
}

#[tokio::main]
async fn main() {
    let fact = get_cat_fact().await;

    println!("fact = {:#?}", fact);

    // The code below will panic as it fails to deserialize the response as JSON
    // from the GET Cat Fact Ninja API endpoint using struct DifferentCatFact as this 
    // struct doesn't match the JSON structure of the response.
    let different_cat_fact = get_different_cat_fact().await;

    println!("different cat fact = {:#?}", different_cat_fact);
}

async fn get_cat_fact() -> Result<CatFact, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get("https://catfact.ninja/fact")
        .send()
        .await?
        .json::<CatFact>()
        .await?;

    Ok(body)
}

async fn get_different_cat_fact() -> Result<DifferentCatFact, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get("https://catfact.ninja/fact")
        .send()
        .await?
        .json::<DifferentCatFact>()
        .await?;

    Ok(body)
}
