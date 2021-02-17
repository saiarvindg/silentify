use tokio::time::{delay_for, Duration};

mod spotifyclient;

const AD_LENGTH: u64 = 15; // default ad length

#[tokio::main]
async fn main() {
    println!("Starting Silentify...");
    let spotify = &spotifyclient::build_spotify_instance().await;
    let (name, email) = spotifyclient::get_current_displayname(spotify).await;
    println!("The current logged in user is {} ({})", name, email);

    let run_result = true;
    loop {
        let curr_track = spotifyclient::get_curr_playing_track(spotify).await;

        match curr_track {
            Some(playing) => {
                println!("Some Playing object: {:?}", playing);
                match playing.item {
                    Some(full_track) => {
                        println!("Some track is playing: {:?}", full_track);
                        println!("The track type is {:?}", full_track._type);
                    },
                    // in rspotify library None means the item is an ad (not a track, album, playist, episode, etc...)
                    None => println!("Could not get information about currently playing track.")
                    
                }

            }
            None => println!("Nothing is currently playing. Exiting...")
        }

        if !run_result { 
            println!("Exiting...");
            break;
        } else {
            println!("waiting for {} seconds", AD_LENGTH);
            delay_for(Duration::from_secs(AD_LENGTH)).await;
            println!("waited for {} seconds", AD_LENGTH);
        }
    }



    // let curr_playing = spotifyclient::get_curr_playing_track(spotify).await;
    // match curr_playing {
    //     Some(p) => println!("Playing: {:?}", p),
    //     None => println!("Nothing is currently playing."),
    // };
}
