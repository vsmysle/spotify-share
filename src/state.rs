#[derive(Clone)]
pub struct AppState<LocalStorage> {
    pub interface: std::sync::Arc<super::player::interface::PlayerInterface<LocalStorage>>,
    pub api_client: super::spotify::SpotifyAPIClient,
    pub tx: flume::Sender<super::player::action::PlayerAction>
}