//! news-related trends using custom implementation.

use squid::squid_client::SquidClient;
use squid::{AddRequest, LeaderboardRequest};
use tonic::transport::{Channel, Error};
use tonic::Status;

const DAY_IN_SECONDS: u64 = 86400;

mod squid {
    tonic::include_proto!("squid");
}

/// Squid client structure.
#[derive(Clone, Debug)]
pub struct Squid {
    client: SquidClient<Channel>,
}

impl Squid {
    /// Start a new squid client over gRPC.
    pub async fn init(url: String) -> Result<Self, Error> {
        let client = SquidClient::connect(url).await?;

        Ok(Squid { client })
    }

    /// Add new sentence on squid.
    pub async fn add_entry(&mut self, text: String) -> Result<(), Status> {
        let request = tonic::Request::new(AddRequest {
            sentence: text,
            lifetime: DAY_IN_SECONDS,
        });

        self.client.add(request).await?;

        Ok(())
    }

    /// Get top 10 most used words in news articles.
    pub async fn leaderboard(&mut self) -> Result<Vec<String>, Status> {
        let request = tonic::Request::new(LeaderboardRequest { length: 10 });

        let leaderboard = self
            .client
            .leaderboard(request)
            .await?
            .into_inner()
            .word
            .iter()
            .map(|w| w.word.clone())
            .collect();

        Ok(leaderboard)
    }
}
