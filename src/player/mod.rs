use std::sync::Arc;

use librespot::{
    core::{cache::Cache, config::SessionConfig, session::Session, SpotifyId},
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer::NoOpVolume,
        player::{Player, PlayerEvent},
    },
};

use crate::{storage::local::LocalStorage, track::Track};

pub mod action;
pub mod interface;
mod state;

async fn get_session_with_cached_credentials(
    cache_location: std::path::PathBuf,
) -> anyhow::Result<Session> {
    let session_config = SessionConfig::default();
    let cache = Cache::new(Some(cache_location), None, None, None)?;

    let session = Session::new(session_config, Some(cache.clone()));
    if let Some(cached_cred) = cache.credentials() {
        match session.connect(cached_cred, true).await {
            Ok(_) => Ok(session),
            Err(err) => {
                tracing::error!("{}", err);
                Err(anyhow::anyhow!("invalid session credentials"))
            }
        }
    } else {
        Err(anyhow::anyhow!(
            "invalid credentials in cache, please regenerate them with get_token"
        ))
    }
}

pub async fn get_player(cache_location: std::path::PathBuf) -> anyhow::Result<Arc<Player>> {
    let session = get_session_with_cached_credentials(cache_location).await?;

    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let backend = audio_backend::find(None).unwrap();
    let player = Player::new(player_config, session, Box::new(NoOpVolume), move || {
        backend(None, audio_format)
    });
    Ok(player)
}

pub struct SpotifyPlayer<LocalStorage> {
    player: Arc<Player>,
    interface: Arc<interface::PlayerInterface<LocalStorage>>,
    rx: flume::Receiver<action::PlayerAction>,
}

impl SpotifyPlayer<LocalStorage> {
    pub async fn new(
        player: Arc<Player>,
        interface: Arc<interface::PlayerInterface<LocalStorage>>,
        rx: flume::Receiver<action::PlayerAction>,
    ) -> Self {
        Self {
            player,
            interface,
            rx,
        }
    }

    async fn play_track(&self, track: Track) {
        let track_id = track.id;
        if let Some(track_id) = track_id {
            let spotify_track_id = SpotifyId::from_uri(&track_id.to_string()).unwrap();
            self.player.load(spotify_track_id, true, 0);
        }
    }

    pub async fn handle_api_actions(&self) -> anyhow::Result<()> {
        tracing::info!("Initializing API events handler.");
        while let Ok(event) = self.rx.recv_async().await {
            match event {
                action::PlayerAction::Play => {
                    tracing::info!("Received play signal");
                    let track = self.interface.get_current_track().await?;
                    if let Some(track) = track {
                        self.play_track(track).await;
                    }
                }
                action::PlayerAction::Stop => {
                    tracing::debug!("Received stop signal");
                    self.player.stop()
                }
                action::PlayerAction::Next => {
                    tracing::debug!("Received next signal");
                    let next_track = self.interface.get_next_track().await;
                    if let Ok(Some(next_track)) = next_track {
                        self.play_track(next_track).await;
                    } else {
                        tracing::warn!("No next track or error: {:?}", next_track);
                    }
                }
                action::PlayerAction::Prev => {
                    tracing::debug!("Received prev signal");
                    let prev_track = self.interface.get_prev_track().await;
                    if let Ok(Some(prev_track)) = prev_track {
                        self.play_track(prev_track).await;
                    } else {
                        tracing::warn!("No prev track or error: {:?}", prev_track);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn handle_player_events(&self) -> anyhow::Result<()> {
        tracing::info!("Initializing player events handler.");
        let mut channel = self.player.get_player_event_channel();
        loop {
            if let Some(event) = channel.recv().await {
                match event {
                    PlayerEvent::Stopped { track_id, .. } => {
                        tracing::info!("stopped playing track with id: {}", track_id);
                    }
                    PlayerEvent::Loading { track_id, .. } => {
                        tracing::info!("loading track with id: {}", track_id);
                    }
                    PlayerEvent::Preloading { track_id } => {
                        tracing::info!("preloading track with id: {}", track_id);
                    }
                    PlayerEvent::Playing { track_id, .. } => {
                        tracing::info!("playing track with id: {}", track_id);
                    }
                    PlayerEvent::TimeToPreloadNextTrack { track_id, .. } => {
                        tracing::info!("time to preload the next track with id: {}", track_id);
                        if let Ok(Some(track_id)) =
                            self.interface.get_next_track_id_for_preload().await
                        {
                            let spotify_track_id = SpotifyId::from_uri(&track_id).unwrap();
                            self.player.preload(spotify_track_id);
                        }
                    }

                    PlayerEvent::EndOfTrack { track_id, .. } => {
                        tracing::info!(
                            "track with id {} ended!, playing next one if any",
                            track_id
                        );
                        if let Ok(Some(next_track)) = self.interface.get_next_track().await {
                            self.play_track(next_track).await;
                        }
                    }
                    _ => {
                        tracing::info!("not yet implemented!")
                    }
                }
            };
        }
    }
}
