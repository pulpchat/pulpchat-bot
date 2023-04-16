use aws_sdk_dynamodb as dynamodb;

use dynamodb::types::AttributeValue;
use dynamodb::{Client, Error};
use tokio_stream::StreamExt;

pub async fn list_tables(client: &Client) -> Result<(), Error> {
    let paginator = client.list_tables().into_paginator().items().send();
    let table_names = paginator.collect::<Result<Vec<_>, _>>().await?;

    println!("Tables:");

    for name in &table_names {
        println!("  {}", name);
    }

    println!("Found {} tables", table_names.len());
    Ok(())
}

async fn add_item(
    client: &Client,
    table: &str,
    username: &str,
    p_type: &str,
    age: &str,
    first: &str,
    last: &str,
) -> Result<(), Error> {
    let user_av = AttributeValue::S(username.into());
    let type_av = AttributeValue::S(p_type.into());
    let age_av = AttributeValue::S(age.into());
    let first_av = AttributeValue::S(first.into());
    let last_av = AttributeValue::S(last.into());

    let request = client
        .put_item()
        .table_name(table)
        .item("username", user_av)
        .item("account_type", type_av)
        .item("age", age_av)
        .item("first_name", first_av)
        .item("last_name", last_av);

    println!("Executing request [{:?}] to add item...", request);

    request.send().await?;

    println!(
        "Added user {}, {} {}, age {} as {} user",
        username, first, last, age, p_type
    );

    Ok(())
}
