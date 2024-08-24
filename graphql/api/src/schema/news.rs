use crate::models::{image::Image, news::News, source::Media};
use crate::schema::Date;
use crate::Context;
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

/// Implement GraphQL on News structure.
#[graphql_object(context = Context, description = "A media article.")]
impl News {
    /// News article title.
    fn title(&self) -> &str {
        &self.title
    }

    /// Description of the article written by its authors.
    fn description(&self) -> &str {
        &self.description
    }

    /// Article publication date with `dd/mm/yyyy` format.
    fn published_at(&self) -> Date {
        self.published_at
    }

    /// The image chosen by the editorial team to illustrate the subject.
    fn image(&self) -> &Image {
        &self.image
    }

    /// Similar articles.
    fn similar(&self) -> &Vec<News> {
        &self.similar
    }

    /// The media that published the article.
    fn source(&self) -> &Media {
        &self.source
    }

    /// ML-generated summary of news article.
    fn summary(&self) -> &String {
        &self.summary
    }
}

/// Define the news query object.
#[derive(Clone, Copy, Debug)]
pub struct NewsQuery;

/// Implement the GraphQL object for the news query.
#[graphql_object(context = Context)]
impl NewsQuery {
    /// Get the *three* most relevant news of the day.
    async fn get_top_news(
        ctx: &Context,
        #[graphql(description = "ISO 3166-1 alpha-2 country code.")]
        country: String,
        #[graphql(description = "Maximum number of articles sent.")] limit: i32,
    ) -> FieldResult<Vec<News>> {
        let rank = ctx.ranker.get_rank(limit.try_into()?).await?;
        let mut most_revelant_news = Vec::new();

        for word in rank {
            let news = ctx
                .meilisearch
                .write()
                .await
                .index
                .as_ref()
                .ok_or_else(|| FieldError::new("Index not found", graphql_value!({ "internal_error": "Meilisearch index is not selected" })))?
                .search()
                .with_query(&word)
                .with_limit(1)
                .with_filter(&format!("source.country={:?}", country))
                .with_attributes_to_search_on(&["title"])
                .execute::<News>()
                .await?;

            most_revelant_news.push(news.hits[0].result.clone())
        }

        Ok(most_revelant_news)
    }

    /// Get news of the day.
    async fn get_news(
        ctx: &Context,
        #[graphql(description = "ISO 3166-1 alpha-2 country code.")]
        country: String,
        #[graphql(description = "Maximum number of articles sent.")] limit: i32,
    ) -> FieldResult<Vec<News>> {
        let news = ctx
            .meilisearch
            .write()
            .await
            .index
            .as_ref()
            .ok_or_else(|| FieldError::new("Index not found", graphql_value!({ "internal_error": "Meilisearch index is not selected" })))?
            .search()
            .with_query("*")
            .with_limit(limit.try_into()?)
            .with_filter(&format!("source.country={:?}", country))
            .execute::<News>()
            .await?;

        Ok(news.hits.iter().map(|r| r.result.clone()).collect())
    }
}
