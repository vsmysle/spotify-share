use librespot::core::{
    authentication::Credentials, cache::Cache, config::SessionConfig, session::Session,
};

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing";

pub async fn get_session(
    username: &str,
    password: &str,
    cache_location: std::path::PathBuf,
) -> anyhow::Result<()> {
    let session_config = SessionConfig::default();
    let credentials = Credentials::with_password(username, password);
    let cache = Cache::new(Some(cache_location), None, None, None)?;

    let session = Session::new(session_config, Some(cache));
    match session.connect(credentials, true).await {
        Ok(_) => {
            session.token_provider().get_token(SCOPES).await?;
            tracing::debug!("Successfully retrieve session token from Spotify!");
            Ok(())
        }
        Err(err) => {
            tracing::error!("Failed to retrieve token from Spotify, error={}", err);
            Err(anyhow::anyhow!(err))
        }
    }
}
