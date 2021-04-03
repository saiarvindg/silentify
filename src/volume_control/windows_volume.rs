use std::process::Command;

const SCRIPTS_BASE_PATH: &str = "./src/volume_control/windows_volume/";
const GET_SPOTIFY_VOL_SCRIPT_PATH: &str = "GetSpotifyVolume.ps1";
const MUTE_SPOTIFY_VOL_SCRIPT_PATH: &str = "SetSpotifyMute.ps1";
const UNMUTE_FLAG: &str = "unmute";

pub fn get_vol() -> i8 {
    let script_path = format!("{}{}", SCRIPTS_BASE_PATH, GET_SPOTIFY_VOL_SCRIPT_PATH);
    let output = Command::new("powershell").arg(script_path).output().expect("Failed to get volume from PS script");
    let vol = String::from_utf8_lossy(&output.stdout).trim().parse::<i8>().expect("Could not parse result of get volume PS script");
    print!("get_vol() -> current volume: {:?}", vol);
    return vol;
}

pub fn mute_vol() {
    let script_path = format!("{}{}", SCRIPTS_BASE_PATH, MUTE_SPOTIFY_VOL_SCRIPT_PATH);
    Command::new("powershell").arg(script_path).output().expect("Failed to mute volume from PS script");
}

pub fn unmute_vol() {
    let script_path = format!("{}{}", SCRIPTS_BASE_PATH, MUTE_SPOTIFY_VOL_SCRIPT_PATH);
    Command::new("powershell").arg(script_path).arg(UNMUTE_FLAG).output().expect("Failed to unmute volume from PS script");
}

// TODO: revisit if setting the volume is needed
// const SET_VOL: &str = "./SetSpotifyVolume.ps1";
// pub fn set_vol(level: i8) {

// }