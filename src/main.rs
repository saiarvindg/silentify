mod spotifyclient;

fn main() {
    let spotify = spotifyclient::build_spotify_instance();
    let curr_playing = spotifyclient::get_curr_playing_track(spotify);
    match curr_playing {
        Some(p) => println!("Playing: {:?}", p),
        None => println!("Nothing is currently playing."),
    };
}
