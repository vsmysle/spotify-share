# SPOTIFY-SHARE

This is my capstone project. It is currenly in development. 
The idea was to write an application which will allow to add tracks to a shared music playlist.
As for a "backend" of this application, I chose `spotify` (hence the name of this project).


## What it can do and how to use it?

This project is connecting to Spotify and stream music from it using `librespot` crate.
In order for this application to work, you need to have Spotify Premium account.
Additionally, you will need to create an app on [Spotify for Developers](https://developer.spotify.com/documentation/web-api/concepts/apps) platform to get `SPOTIFY_CLIENT_ID` and `SPOTIFY_CLIENT_SECRET` values.

1. Register an application on the [Spotify for Developers](https://developer.spotify.com/documentation/web-api/concepts/apps) platform.
2. Copy `.env.example` into `.env` and fill required variables.
3. Run `cargo build --release`.
4. Create a directory which will store cached credentials, e.g `.cache`.
5. Get an authorization tokio and cache it by running the following command: `./target/release/spotify-share get-token --cache-directory <path_to_cache_directory>`
6. Run web server: `./target/release/spotify-share run-server --ip <ip> --port <port> --cache-directory <path_to_cache_directory>`


## Endpoints

### Track endpoints

`POST /tracks` <-> Add track to a queue.

`POST /tracks/search` <-> Search query. It is currently finds 10 tracks by provided query.

`GET /tracks` <-> Returns a list of tracks in the queue.

### Control endpoints

`POST /control/play` <-> Play the current song from beginning (no pause/resume support now).

`POST /control/stop` <-> Stop playing the music.

`POST /control/next` <-> Play next song from the queue.

`POST /control/prev` <-> Play previous song from the queue.

Proper documentation will be generated later, sorry!