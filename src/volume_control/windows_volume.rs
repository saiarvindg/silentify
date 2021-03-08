use std::process::Command;

const GET_SPOTIFY_VOL_SCRIPT_PATH: &str = "./GetSpotifyVolume.ps1";
const SET_VOL: &str = "./SetSpotifyVolume.ps1";
const MUTE: &str = "./SetSpotifyMute.ps1";
const UNMUTE: &str = "./SetSpotifyMute.ps1 unmute";

pub fn get_vol() -> i8 {
    let output = Command::new("powershell").arg(GET_SPOTIFY_VOL_SCRIPT_PATH).output().expect("Failed to get volume from PS script");
    let vol = String::from_utf8_lossy(&output.stdout).trim().parse::<i8>().expect("Could not parse result of get volume script");
    print!("{:?}", vol);
    return 20;
}

pub fn set_vol(level: i8) {

}