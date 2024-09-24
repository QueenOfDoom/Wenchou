use std::error::Error;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::{Block, List}
};
use ratatui::layout::Alignment;
use rss::Channel;

async fn nhk_news_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://www3.nhk.or.jp/rss/news/cat0.xml")
        .await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let feed = nhk_news_feed().await?;
    let feed_items = feed.items();
    let articles: Vec<String> = feed_items.iter()
        .map(|i| i.clone().title.unwrap()).collect();
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {
            frame.render_widget(
                List::new(articles.clone())
                    .block(Block::bordered().title("NHK Feed")
                        .title_alignment(Alignment::Center)),
                frame.area(),
            );
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    ratatui::restore();
    Ok(())
}
