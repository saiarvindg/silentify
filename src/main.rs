extern crate rspotify;

use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::SpotifyClientCredentials;
use rspotify::senum::Country;

fn main() {
    let client_credential = SpotifyClientCredentials::default().build();
    let access_token = client_credential.get_access_token();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .access_token(&access_token)
        .build();

    
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let tracks = spotify.artist_top_tracks(birdy_uri, Country::UnitedStates);
    println!("{:?}", tracks.unwrap());
}