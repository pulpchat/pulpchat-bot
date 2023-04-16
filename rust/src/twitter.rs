// #[derive(Debug, Deserialize)]
// struct Tweet {
//     pub user_name: String,
//     pub user_location: String,
//     pub user_description: String,
//     pub user_created: String,
//     pub user_followers: u64,
//     pub user_friends: u64,
//     pub user_favourites: u64,
//     pub user_verified: String,
//     pub date: String,
//     pub text: String,
//     pub hashtags: String,
//     pub source: String,
// }

// fn read_tweets_csv() -> Result<(), Box<dyn Error>> {
//     let file = File::open("tweets.csv")?;
//     let reader = BufReader::new(file);
//     let mut csv_reader = csv::Reader::from_reader(reader);

//     for result in csv_reader.deserialize() {
//         let tweet: Tweet = result?;
//         println!("{:#?}", tweet.user_name);
//     }

//     Ok(())
// }