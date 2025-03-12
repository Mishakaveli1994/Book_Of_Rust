use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

    let news_article = NewsArticle {
        author: String::from("Mishakaveli"),
        headline: String::from("Some cool headline"),
        location: String::from("Boboshevo"),
        content: String::from("Imalo edno vreme..."),
    };

    // println!("1 new news article: {}", news_article.summarize());
    notify(&news_article);

    let default_news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
           hockey team in the NHL.",
        ),
    };

    // println!("New article available! {}", default_news_article.summarize());
    notify(&default_news_article);
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}
