#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TrackRequest {
    pub track_id: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchTrackRequest {
    pub query: String,
}
