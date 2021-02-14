mod spotifyclient;

#[tokio::main]
async fn main() {
    let spotify = &spotifyclient::build_spotify_instance().await;
    let me = spotifyclient::get_current_user(spotify).await;
    println!("me: {:?}", me);

    let curr_playing = spotifyclient::get_curr_playing_track(spotify).await;
    match curr_playing {
        Some(p) => println!("Playing: {:?}", p),
        None => println!("Nothing is currently playing."),
    };
}
