use std::error::Error;
use rss::Channel;

async fn nhk_news_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("www3.nhk.or.jp/rss/news/cat0.xml")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() {
    let feed = nhk_news_feed();
}
