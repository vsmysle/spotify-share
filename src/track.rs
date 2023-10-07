#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Track {
    pub album: String,
    pub artists: Vec<String>,
    #[serde(with = "humantime_serde")]
    pub duration: std::time::Duration,
    pub explicit: bool,
    pub href: Option<String>,
    pub id: Option<rspotify::model::TrackId<'static>>,
    pub name: String,
    pub popularity: u32,
}

impl From<&rspotify::model::FullTrack> for Track {
    fn from(track: &rspotify::model::FullTrack) -> Self {
        let album = track.album.name.clone();
        let artists = track
            .artists
            .iter()
            .map(|artist| artist.name.clone())
            .collect::<Vec<String>>();
        let duration =
            std::time::Duration::from_secs(track.duration.num_seconds().try_into().unwrap());
        let explicit = track.explicit;

        Self {
            album,
            artists,
            duration,
            explicit,
            href: track.href.clone(),
            id: track.id.clone(),
            name: track.name.clone(),
            popularity: track.popularity,
        }
    }
}
