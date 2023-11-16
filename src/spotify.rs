use rspotify::{
    model::{Country, Market, SearchType, TrackId},
    prelude::*,
    ClientCredsSpotify,
};

use crate::track::Track;

#[derive(Clone)]
pub struct SpotifyAPIClient {
    inner: ClientCredsSpotify,
}

impl SpotifyAPIClient {
    pub async fn new(spotify_client_id: &str, spotify_client_secret: &str) -> anyhow::Result<Self> {
        let spotify_api_client = rspotify::ClientCredsSpotify::with_config(
            rspotify::Credentials::new(spotify_client_id, spotify_client_secret),
            rspotify::Config {
                token_refreshing: true,
                ..rspotify::Config::default()
            },
        );
        if let Err(err) = spotify_api_client.request_token().await {
            Err(anyhow::anyhow!(err))
        } else {
            Ok(Self {
                inner: spotify_api_client,
            })
        }
    }

    pub async fn get_by_id(&self, track_id: &str) -> anyhow::Result<Track> {
        let track_id = TrackId::from_id(track_id)?;
        let track = &self.inner.track(track_id, None).await?;
        Ok(Track::from(track))
    }

    pub async fn search(&self, track_query: &str) -> anyhow::Result<Vec<Track>> {
        tracing::info!("searching for \"{}\"...", track_query);
        let result = &self
            .inner
            .search(
                track_query,
                SearchType::Track,
                Some(Market::Country(Country::Ukraine)),
                None,
                Some(10),
                None,
            )
            .await?;

        if let rspotify::model::SearchResult::Tracks(tracks) = result {
            let tracks = tracks.items.iter().map(Track::from).collect::<Vec<Track>>();
            return Ok(tracks);
        }
        Ok(vec![])
    }
}
