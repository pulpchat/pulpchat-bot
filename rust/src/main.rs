use aws_sdk_dynamodb::Client as DynamoClient;
use eyre::{Error, Result};
use procon::ProconColumn;
use reqwest::Client as ReqwestClient;
use tokio;

mod aws;
mod data;
mod procon;
mod transcribe;
mod twitter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = DynamoClient::new(&config);

    let c = ReqwestClient::new();
    let col = ProconColumn::scrape("https://deathpenalty.procon.org/", &c).await?;
    println!("Arguments {:#?}", col.arguments);
    println!("Did you know? {:#?}", col.supporting.did_you_know);
    println!("History {:#?}", col.history);
    println!("Quotes {:#?}", col.quotes);

    col.write_to_dynamo("pulpscrape", &client).await?;

    Ok(())
}
