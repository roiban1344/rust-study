use traits::Summary;

fn main() {
    let tweet = traits::Tweet {
        username: String::from("Ferris2018"),
        content: String::from("🦀Hello fellow Rustaceans!⚙"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
