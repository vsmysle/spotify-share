use clap::Parser;
use std::sync::Arc;

use dotenvy::dotenv;

mod api;
mod cli;
mod logging;
mod player;
mod request;
mod spotify;
mod state;
mod storage;
mod token;
mod track;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    logging::init();

    let cli = cli::Cli::parse();

    match cli.command {
        cli::Action::GetToken { cache_directory } => {
            let spotify_username = std::env::var("SPOTIFY_USERNAME")
                .expect("You must specify SPOTIFY_USERNAME environment variable.");

            let spotify_password = std::env::var("SPOTIFY_PASSWORD")
                .expect("You must specify SPOTIFY_PASSWORD environment variable.");
            token::get_session(&spotify_username, &spotify_password, cache_directory).await?;
        }
        cli::Action::RunServer {
            ip,
            port,
            cache_directory,
        } => {
            // Get token if not available
            if !&cache_directory.try_exists()? {
                let spotify_username = std::env::var("SPOTIFY_USERNAME")
                    .expect("You must specify SPOTIFY_USERNAME environment variable.");

                let spotify_password = std::env::var("SPOTIFY_PASSWORD")
                    .expect("You must specify SPOTIFY_PASSWORD environment variable.");

                token::get_session(
                    &spotify_username,
                    &spotify_password,
                    cache_directory.clone(),
                )
                .await?;
            }

            let spotify_client_id = std::env::var("SPOTIFY_CLIENT_ID")
                .expect("You must specify SPOTIFY_CLIENT_ID environment variable.");
            let spotify_client_secret = std::env::var("SPOTIFY_CLIENT_SECRET")
                .expect("You must specify SPOTIFY_CLIENT_ID environment variable.");

            // Channel for communication between API and
            let (tx, rx) = flume::unbounded();

            // Interface for handling the storage
            let interface = Arc::new(player::interface::PlayerInterface::default());
            let player = player::get_player(cache_directory).await?;

            let spotify_player = player::SpotifyPlayer::new(player, interface.clone(), rx).await;
            let spotify_api_client =
                spotify::SpotifyAPIClient::new(&spotify_client_id, &spotify_client_secret).await?;

            let state = state::AppState {
                interface: interface.clone(),
                api_client: spotify_api_client,
                tx,
            };

            let state = Arc::new(state);
            let addr = std::net::SocketAddr::new(std::net::IpAddr::V4(ip), port);

            tokio::select! {
                _ = spotify_player.handle_api_actions() => {},
                _ = spotify_player.handle_player_events() => {},
                _ = api::routes::get_app(addr, state) => {}
            }
        }
    }
    Ok(())
}
