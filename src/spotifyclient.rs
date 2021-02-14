extern crate rspotify;

use rspotify::blocking::oauth2::SpotifyClientCredentials;
use rspotify::{blocking::client::Spotify, model::playing::Playing};

pub fn build_spotify_instance() -> Spotify {
    println!("Creating spotify instance...");
    let client_credential = SpotifyClientCredentials::default().build();
    let access_token = client_credential.get_access_token();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .access_token(&access_token)
        .build();

    // check that everything is setup properly by getting the current user
    match spotify.me() {
        Ok(me) => println!("Me: {:?}", me),
        Err(err) => panic!("Could not fetch current user. Spotify may not be setup properly: {}", err)
    }

    return spotify;
}

pub fn get_curr_playing_track(spotify: Spotify) -> Option<Playing> {
    // let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    println!("Fetching currently playing track...");
    let get_curr_track = spotify.current_user_playing_track();
    let curr_track = match get_curr_track {
        Ok(track) => track,
        Err(err) => panic!("Could not fetch currently playing track: {}", err),
    };

    println!("Current track: {:?}", curr_track);
    return curr_track;
}

// pub fn calc_track_end_time(total_duration: u32, curr_progress: u32) {
    
// }
