use async_trait::async_trait;

use crate::track::Track;

#[async_trait]
pub trait AsyncStorage {
    async fn add(&self, track: Track) -> anyhow::Result<()>;
    async fn list(&self) -> anyhow::Result<Vec<Track>>;
    async fn next(&self) -> anyhow::Result<Option<Track>>;
    async fn next_track_id(&self) -> anyhow::Result<Option<String>>;
    async fn prev(&self) -> anyhow::Result<Option<Track>>;
    async fn current(&self) -> anyhow::Result<Option<Track>>;
}


pub mod local;
