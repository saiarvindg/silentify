use rspotify::{client::Spotify, model::{playing::Playing, user::PrivateUser}, oauth2::{SpotifyClientCredentials, SpotifyOAuth}};
use rspotify::util::get_token;

extern crate rspotify;



pub async fn build_spotify_instance() -> Spotify {
    println!("Creating spotify instance...");

    let mut oauth = SpotifyOAuth::default()
    .scope("user-read-currently-playing")
    .build();

    let token_info = match get_token(&mut oauth).await {
        Some(token_info) => token_info,
        None => panic!("could get token from spotify")
    };

    let client_credentials = SpotifyClientCredentials::default().token_info(token_info).build();

    return Spotify::default().client_credentials_manager(client_credentials).build();
}

pub async fn get_current_displayname(spotify: &Spotify) -> (String, String) {
    println!("Fetching currently logged in user...");
    let user = spotify.me().await.expect("Could not fetch current user. Spotify may not be setup properly");
    let displayname = user.display_name.unwrap_or(String::from("No display name set"));
    let email = user.email.unwrap_or(String::from("No email address set"));
    return (displayname, email);
}


pub async fn get_curr_playing_track(spotify: &Spotify) -> Option<Playing> {
    println!("Fetching currently playing track...");
    return spotify.current_user_playing_track().await.expect("Could not fetch currently playing track.");
}


// pub fn calc_track_end_time(total_duration: u32, curr_progress: u32) {
    
// }
