#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "twitter")]
    #[tokio::test]
    async fn get_tweet() {
        use twitter_v2::TwitterApi;
        use twitter_v2::authorization::BearerToken;
        use twitter_v2::query::TweetField;

        let auth = BearerToken::new(std::env::var("APP_BEARER_TOKEN").unwrap());
        let tweet = TwitterApi::new(auth)
            .get_tweet(1261326399320715264)
            .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
            .send()
            .await
            .unwrap()
            .into_data()
            .expect("this tweet should exist");
        dbg!(&tweet);
        assert_eq!(tweet.id, 1261326399320715264);
        assert_eq!(tweet.author_id.unwrap(), 2244994945);
    }
}
