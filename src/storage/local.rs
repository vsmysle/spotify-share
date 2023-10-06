use async_trait::async_trait;

use tokio::sync::Mutex;
use crate::track::Track;

use super::AsyncStorage;

#[derive(Default)]
pub struct LocalStorage {
    inner: Mutex<Vec<Track>>,
    current_index: Mutex<u64>
}

#[async_trait]
impl AsyncStorage for LocalStorage {
    async fn add(&self, track: Track) -> anyhow::Result<()> {
        let mut queue = self.inner.lock().await;
        queue.push(track.clone());
        Ok(())
    }

    async fn next(&self) -> anyhow::Result<Option<Track>> {
        let mut idx_guard = self.current_index.lock().await;
        let next_idx = *idx_guard as usize + 1;            
        let queue = self.inner.lock().await;

        tracing::debug!("Number of song in a queue: {}, next_idx: {}", queue.len(), next_idx);
        if let Some(track) = queue.get(next_idx) {
            *idx_guard += 1;
            Ok(Some(track.clone()))
        } else {
            Ok(None)
        }
    }
    
    async fn next_track_id(&self) -> anyhow::Result<Option<String>> {
        let idx_guard = self.current_index.lock().await;
        let next_idx = *idx_guard as usize + 1;            
        let queue = self.inner.lock().await;

        tracing::debug!("Number of song in a queue: {}, next_idx: {}", queue.len(), next_idx);
        if let Some(track) = queue.get(next_idx) {
            let track_id = &track.id;
            if let Some(track_id) = track_id {
                return Ok(Some(track_id.to_string()))
            } 
        }
        Ok(None)
    }

    async fn prev(&self) -> anyhow::Result<Option<Track>> {
        let mut idx_guard = self.current_index.lock().await;
        let prev_idx = *idx_guard as usize - 1;            
        let queue = self.inner.lock().await;
            
        tracing::debug!("Number of song in a queue: {}, prev_idx: {}", queue.len(), prev_idx);
        if let Some(track) = queue.get(prev_idx) {
            *idx_guard -= 1;
            Ok(Some(track.clone()))
        } else {
            Ok(None)
        }
    }
    
    async fn current(&self) -> anyhow::Result<Option<Track>> {
        let idx = *self.current_index.lock().await as usize;
        let queue = self.inner.lock().await;
            
        tracing::debug!("Number of song in a queue: {}, current_idx: {}", queue.len(), idx);
        if let Some(track) = queue.get(idx) {
            Ok(Some(track.clone()))
        } else {
            Ok(None)
        }
    }

    
    async fn list(&self) -> anyhow::Result<Vec<Track>> {
        let vec = self.inner.lock().await;
        Ok(vec.clone())
    }
}