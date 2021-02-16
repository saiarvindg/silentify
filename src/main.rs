use tokio::time::{delay_for, Duration};

mod spotifyclient;

const AD_LENGTH: u64 = 5; // default ad length

#[tokio::main]
async fn main() {
    let spotify = &spotifyclient::build_spotify_instance().await;
    print_current_user_info(spotify);

     let run_result = true;
     
   

    println!("waiting for {} seconds", AD_LENGTH);
    delay_for(Duration::from_secs(AD_LENGTH)).await;
    println!("waited for {} seconds", AD_LENGTH);



    // let curr_playing = spotifyclient::get_curr_playing_track(spotify).await;
    // match curr_playing {
    //     Some(p) => println!("Playing: {:?}", p),
    //     None => println!("Nothing is currently playing."),
    // };
}

fn print_current_user_info(spotify: &Spotify) {
    let (name, email) = spotifyclient::get_current_displayname(spotify).await;
    println!("The current logged in user is {} ({})", name, email);
}
