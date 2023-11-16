use super::state::PlayerState;
use crate::storage::{local::LocalStorage, AsyncStorage};
use crate::track::Track;

#[derive(Clone, Default)]
pub struct PlayerInterface<LocalStorage> {
    state: PlayerState<LocalStorage>,
}

impl PlayerInterface<LocalStorage> {
    pub async fn get_current_track(&self) -> anyhow::Result<Option<Track>> {
        self.state.storage.current().await
    }

    pub async fn get_next_track(&self) -> anyhow::Result<Option<Track>> {
        self.state.storage.next().await
    }

    pub async fn get_next_track_id_for_preload(&self) -> anyhow::Result<Option<String>> {
        self.state.storage.next_track_id().await
    }

    pub async fn get_prev_track(&self) -> anyhow::Result<Option<Track>> {
        self.state.storage.prev().await
    }

    pub async fn list_tracks(&self) -> anyhow::Result<Vec<Track>> {
        self.state.storage.list().await
    }

    pub async fn add_track(&self, track: Track) -> anyhow::Result<()> {
        self.state.storage.add(track).await?;
        Ok(())
    }
}
