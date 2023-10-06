#[derive(serde::Serialize, serde::Deserialize)]
pub enum PlayerAction {
    Play,
    Stop,
    Next,
    Prev
}