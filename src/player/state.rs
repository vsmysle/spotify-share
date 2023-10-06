use librespot::{core::SpotifyId, playback::player::PlayerEvent};

#[derive(Clone, Default)]
pub struct PlayerState<LocalStorage> {
    pub storage: LocalStorage, 
    pub player_state: TrackPlayerState
}


#[derive(Default, Clone)]
pub enum TrackPlayerState {
    #[default]
    Initializing,
    Playing {
        play_request_id: u64,
        track_id: SpotifyId,
        position_ms: u32
    },
    Stopped {
        play_request_id: u64,
        track_id: SpotifyId
    },
    Paused {
        play_request_id: u64,
        track_id: SpotifyId,
        position_ms: u32
    }
}
pub struct Unsupported;

impl TryFrom<PlayerEvent> for TrackPlayerState {
    type Error = Unsupported;

    fn try_from(value: PlayerEvent) -> Result<Self, Self::Error> {
        match value {
            PlayerEvent::Stopped { play_request_id, track_id } => Ok(
                Self::Stopped { 
                    play_request_id, 
                    track_id 
                }
            ),
            PlayerEvent::Playing { 
                play_request_id, 
                track_id, 
                position_ms 
            } => Ok(
                Self::Playing { 
                    play_request_id, 
                    track_id, 
                    position_ms
                }
            ),
            PlayerEvent::Paused { 
                play_request_id, 
                track_id, 
                position_ms 
            } => Ok(
                Self::Paused { 
                    play_request_id, 
                    track_id,
                    position_ms
                }
            ),
            _ => Err(Unsupported)
        } 
    }
}