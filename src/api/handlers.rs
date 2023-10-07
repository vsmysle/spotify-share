use std::sync::Arc;
use axum::http::StatusCode;
use axum::extract::{Json, State};

use super::errors::AppError;

use crate::state::AppState;
use crate::storage::local::LocalStorage;
use crate::track::Track;

pub mod tracks {
    use crate::request::{TrackRequest, SearchTrackRequest};
    use super::*;

    pub async fn add(
        State(state): State<Arc<AppState<LocalStorage>>>,
        Json(track_request): Json<TrackRequest>
    ) -> Result<Json<Track>, AppError> {
        let track_id = track_request.track_id.clone();        
        tracing::info!("received a request to add a new track to a playlist - {}", track_id); 
        let track = state.api_client.get_by_id(&track_id).await?;
        state.interface.add_track(track.clone()).await?;
        Ok(Json(track))
    }
    
    pub async fn list(
        State(state): State<Arc<AppState<LocalStorage>>>
    ) -> Result<Json<Vec<Track>>, AppError> {
        let tracks = state.interface.list_tracks().await?;
        Ok(Json(tracks))
    }

    pub async fn search(
        State(state): State<Arc<AppState<LocalStorage>>>,
        Json(search_payload): Json<SearchTrackRequest>
    ) -> Result<Json<Vec<Track>>, AppError> {
        let tracks = state.api_client.search(&search_payload.query).await?;
        Ok(Json(tracks))
    }
}

pub mod control {
    use crate::player::action::PlayerAction;
    use super::*;

    pub async fn next(
        State(state): State<Arc<AppState<LocalStorage>>>,
    ) -> Result<StatusCode, AppError> {
        state.tx.send_async(PlayerAction::Next).await?;
        Ok(StatusCode::OK) 
    }
    
    pub async fn prev(
        State(state): State<Arc<AppState<LocalStorage>>>,
    ) -> Result<StatusCode, AppError> {
        state.tx.send_async(PlayerAction::Prev).await?;
        Ok(StatusCode::OK)
    }
    
    pub async fn stop(
        State(state): State<Arc<AppState<LocalStorage>>>,
    ) -> Result<StatusCode, AppError> {
        state.tx.send_async(PlayerAction::Stop).await?;
        Ok(StatusCode::OK) 
    }

    pub async fn play(
        State(state): State<Arc<AppState<LocalStorage>>>,
    ) -> Result<StatusCode, AppError> {
        state.tx.send_async(PlayerAction::Play).await?;
        Ok(StatusCode::OK) 
    }
}
