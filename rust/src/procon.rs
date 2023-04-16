use std::collections::HashMap;

use aws_sdk_dynamodb as dynamodb;
use eyre::{Error, Result};
use reqwest::Client as ReqwestClient;
use soup::prelude::*;
use uuid::Uuid;

use dynamodb::types::AttributeValue;
use dynamodb::Client as DynamoClient;

#[derive(Debug, Clone)]
pub enum ArgumentType {
    Pro,
    Con,
    Other,
}

impl ToString for ArgumentType {
    fn to_string(&self) -> String {
        match self {
            ArgumentType::Pro => "Pro".to_string(),
            ArgumentType::Con => "Con".to_string(),
            ArgumentType::Other => "Other".to_string(),
        }
    }
}

impl Default for ArgumentType {
    fn default() -> Self {
        ArgumentType::Other
    }
}

#[derive(Debug, Default, Clone)]
pub struct History {
    pub content: String,
    pub more_to_explore: Vec<String>,
}

impl History {
    pub fn new(content: &str, more_to_explore: Vec<String>) -> Self {
        History {
            content: content.to_string(),
            more_to_explore,
        }
    }
}

impl From<History> for std::collections::HashMap<String, AttributeValue> {
    fn from(history: History) -> Self {
        let mut map = std::collections::HashMap::new();

        map.insert("content".to_string(), AttributeValue::S(history.content));
        if history.more_to_explore.len() > 0 {
            map.insert(
                "more_to_explore".to_string(),
                AttributeValue::Ss(history.more_to_explore),
            );
        } else {
            map.insert(
                "more_to_explore".to_string(),
                AttributeValue::Ss(vec!["none".to_string()]),
            );
        }

        map
    }
}

#[derive(Debug, Default, Clone)]
pub struct SupportingData {
    pub did_you_know: Vec<String>,
}

impl SupportingData {
    pub fn new(did_you_know: Vec<String>) -> Self {
        SupportingData { did_you_know }
    }
}

impl From<SupportingData> for std::collections::HashMap<String, AttributeValue> {
    fn from(supporting: SupportingData) -> Self {
        let mut map = std::collections::HashMap::new();

        map.insert(
            "did_you_know".to_string(),
            AttributeValue::Ss(supporting.did_you_know),
        );

        map
    }
}

#[derive(Debug, Default, Clone)]
pub struct Argument {
    pub argument_type: ArgumentType,
    pub header: String,
    pub argument_title: String,
    pub argument_content: String,
}

impl Argument {
    pub fn new(
        argument_type: ArgumentType,
        header: &str,
        argument_title: &str,
        argument_content: &str,
    ) -> Self {
        Argument {
            argument_type,
            header: header.to_string(),
            argument_title: argument_title.to_string(),
            argument_content: argument_content.to_string(),
        }
    }
}

impl From<Argument> for std::collections::HashMap<String, AttributeValue> {
    fn from(arg: Argument) -> Self {
        let mut map = std::collections::HashMap::new();

        map.insert(
            "argument_type".to_string(),
            AttributeValue::S(arg.argument_type.to_string()),
        );
        map.insert("header".to_string(), AttributeValue::S(arg.header));
        map.insert(
            "argument_title".to_string(),
            AttributeValue::S(arg.argument_title),
        );
        map.insert(
            "argument_content".to_string(),
            AttributeValue::S(arg.argument_content),
        );

        map
    }
}

#[derive(Debug, Default, Clone)]
pub struct Quote {
    pub quote_type: ArgumentType,
    pub intro: String,
    pub quote: Vec<String>,
}

impl Quote {
    pub fn new(quote_type: ArgumentType, intro: &str, quote: Vec<String>) -> Self {
        Quote {
            intro: intro.to_string(),
            quote_type,
            quote: quote,
        }
    }
}

impl From<Quote> for std::collections::HashMap<String, AttributeValue> {
    fn from(quote: Quote) -> Self {
        let mut map = std::collections::HashMap::new();

        map.insert(
            "quote_type".to_string(),
            AttributeValue::S(quote.quote_type.to_string()),
        );
        map.insert("intro".to_string(), AttributeValue::S(quote.intro));
        map.insert("quote".to_string(), AttributeValue::Ss(quote.quote));

        map
    }
}

#[derive(Debug, Default)]
pub struct ProconColumn {
    pub topic: String,
    pub background: String,
    pub arguments: Vec<Argument>,
    pub quotes: Vec<Quote>,
    pub history: History,
    pub supporting: SupportingData,
}

impl ProconColumn {
    pub async fn scrape(target: &str, client: &ReqwestClient) -> Result<ProconColumn, Error> {
        let mut column = ProconColumn::default();
        let content = client.get(target).send().await?;
        let soup = Soup::new(content.text().await?.as_ref());

        let mut divs = soup.tag("div").find_all();

        while let Some(div) = divs.next() {
            if div.get("id") == Some("topic-question-image-wrapper".to_string()) {
                column.topic = div.tag("h1").find().unwrap().text();
            }
        }

        let mut divs = soup.tag("div").find_all();
        while let Some(div) = divs.next() {
            if div.get("class") == Some("entry-content".to_string()) {
                div.tag("p").find_all().for_each(|p| {
                    column.background = format!("{} {}", column.background, p.text());
                });

                div.tag("div").find_all().for_each(|div| {
                    if div.get("class") == Some("row no-gutters arguments-container".to_string()) {
                        div.tag("div").find_all().for_each(|div| {
                            if div.get("class")
                                == Some("col-6 arguments-column arguments-column-pro".to_string())
                            {
                                div.tag("div").find_all().for_each(|div| {
                                    if div.get("class") == Some("argument-container".to_string()) {
                                        let mut arg = Argument::new(ArgumentType::Pro, "", "", "");

                                        div.tag("blockquote").find_all().for_each(|blockquote| {
                                            arg.argument_title = format!(
                                                "{}",
                                                blockquote.tag("h3").find().unwrap().text()
                                            );

                                            arg.header = format!(
                                                "{}",
                                                blockquote.tag("h4").find().unwrap().text()
                                            );

                                            arg.argument_content = format!(
                                                "{} {}",
                                                arg.argument_content,
                                                blockquote.tag("p").find().unwrap().text()
                                            );
                                        });

                                        column.arguments.push(arg);
                                    }
                                });
                            } else if div.get("class")
                                == Some("col-6 arguments-column arguments-column-con".to_string())
                            {
                                div.tag("div").find_all().for_each(|div| {
                                    if div.get("class") == Some("argument-container".to_string()) {
                                        let mut arg = Argument::new(ArgumentType::Con, "", "", "");

                                        div.tag("blockquote").find_all().for_each(|blockquote| {
                                            arg.argument_title = format!(
                                                "{}",
                                                blockquote.tag("h3").find().unwrap().text()
                                            );

                                            arg.header = format!(
                                                "{}",
                                                blockquote.tag("h4").find().unwrap().text()
                                            );

                                            arg.argument_content = format!(
                                                "{} {}",
                                                arg.argument_content,
                                                blockquote.tag("p").find().unwrap().text()
                                            );
                                        });

                                        column.arguments.push(arg);
                                    }
                                });
                            }
                        });
                    }
                });
            }
        }

        let mut divs = soup.tag("div").find_all();
        while let Some(div) = divs.next() {
            if div.get("class") == Some("additional-content".to_string()) {
                div.children().for_each(|child| {
                    if child.get("class")
                        == Some("tablepress tablepress-id-1 did-you-know".to_string())
                    {
                        column.supporting.did_you_know.push(child.text());
                    }
                });

                let anchors = div.tag("a").find_all();

                for a in anchors {
                    let href_par = client
                        .get(format!("{}{}", target, a.get("href").unwrap()))
                        .send()
                        .await?;

                    let href = &href_par.url().to_string();

                    let soup = Soup::new(href_par.text().await?.as_ref());
                    if href.contains("history") {
                        println!("{}", href);

                        let mut divs = soup.tag("div").find_all();
                        while let Some(div) = divs.next() {
                            if div.get("class") == Some("entry-content".to_string()) {
                                div.tag("p").find_all().for_each(|p| {
                                    column.history.content =
                                        format!("{} {}", column.history.content, p.text());
                                });
                            }
                        }
                    } else if href.contains("quotes") {
                        let mut divs = soup.tag("div").find_all();
                        while let Some(div) = divs.next() {
                            if div.get("class") == Some("entry-content".to_string()) {
                                div.tag("div").find_all().for_each(|div| {
                                    if div.get("class")
                                        == Some("row no-gutters quotes-container".to_string())
                                    {
                                        div.tag("div").find_all().for_each(|div| {
                                            if div.get("class")
                                                == Some(
                                                    "col-6 quotes-column quotes-column-pro"
                                                        .to_string(),
                                                )
                                            {
                                                div.tag("div").find_all().for_each(|div| {
                                                    if div.get("class")
                                                        == Some("quote-container".to_string())
                                                    {
                                                        let mut quote = Quote::new(
                                                            ArgumentType::Pro,
                                                            "",
                                                            vec!["".to_string()],
                                                        );

                                                        div.tag("blockquote").find_all().for_each(
                                                            |blockquote| {
                                                                quote.intro = {
                                                                    if blockquote.get("class")
                                                                        == Some("intro".to_string())
                                                                    {
                                                                        blockquote.text()
                                                                    } else {
                                                                        "".to_string()
                                                                    }
                                                                };
                                                                blockquote
                                                                    .tag("p")
                                                                    .find_all()
                                                                    .for_each(|p| {
                                                                        quote.quote.push(p.text());
                                                                    });
                                                            },
                                                        );

                                                        column.quotes.push(quote);
                                                    }
                                                });
                                            } else if div.get("class")
                                                == Some(
                                                    "col-6 quotes-column quotes-column-con"
                                                        .to_string(),
                                                )
                                            {
                                                div.tag("div").find_all().for_each(|div| {
                                                    if div.get("class")
                                                        == Some("quote-container".to_string())
                                                    {
                                                        let mut quote = Quote::new(
                                                            ArgumentType::Con,
                                                            "",
                                                            vec!["".to_string()],
                                                        );

                                                        div.tag("blockquote").find_all().for_each(
                                                            |blockquote| {
                                                                quote.intro = {
                                                                    if blockquote.get("class")
                                                                        == Some("intro".to_string())
                                                                    {
                                                                        blockquote.text()
                                                                    } else {
                                                                        "".to_string()
                                                                    }
                                                                };
                                                                blockquote
                                                                    .tag("p")
                                                                    .find_all()
                                                                    .for_each(|p| {
                                                                        quote.quote.push(p.text());
                                                                    });
                                                            },
                                                        );

                                                        column.quotes.push(quote);
                                                    }
                                                });
                                            }
                                        });
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(column)
    }

    pub async fn scrape_all(
        targets: &Vec<&str>,
        client: &ReqwestClient,
    ) -> Result<Vec<ProconColumn>, Error> {
        let mut columns = vec![];

        for target in targets {
            let column = ProconColumn::scrape(target, client).await?;

            columns.push(column);
        }

        Ok(columns)
    }

    pub async fn write_to_dynamo(
        &self,
        table_name: &str,
        client: &DynamoClient,
    ) -> Result<(), Error> {
        let topic = AttributeValue::S(self.topic.clone().into());
        let background = AttributeValue::S(self.background.clone().into());
        let arguments = {
            let mut arguments = vec![];

            for arg in self.arguments.clone() {
                let av = HashMap::from(arg);
                arguments.push(av);
            }

            arguments
        };

        let quotes = {
            let mut quotes = vec![];

            for quote in self.quotes.clone() {
                let av = HashMap::from(quote);

                quotes.push(av);
            }

            quotes
        };
        let history = AttributeValue::M(self.history.clone().into());
        let supporting_data = AttributeValue::M(self.supporting.clone().into());

        let mut request = client
            .put_item()
            .table_name(table_name)
            .item("id", AttributeValue::S(Uuid::new_v4().to_string().into()))
            .item(
                "source",
                AttributeValue::S("https://procon.org".to_string().into()),
            )
            .item("topic", topic)
            .item("background", background)
            .item("history", history)
            .item("supporting_data", supporting_data);

        for i in 0..arguments.len() {
            request = request.item(
                format!("argument_{}", i),
                AttributeValue::M(arguments[i].clone()),
            );
        }

        for i in 0..quotes.len() {
            request = request.item(format!("quote_{}", i), AttributeValue::M(quotes[i].clone()));
        }

        request.send().await?;

        Ok(())
    }
}
