mod windows_volume;
mod mac_volume;

use std::time::Duration;

// this struct will be only be application memory and will be lost if app crashes/exits
#[derive(Default)]
pub struct VolumeSettings {
    startup_vol: i8,
    curr_vol: i8,
}

impl VolumeSettings {
}

#[cfg(target_os = "macos")]
fn get_vol() -> i8 {
    return mac_volume::get_vol();
}

#[cfg(target_os = "macos")]
fn set_vol(level: i8) {
    mac_volume::set_vol(level);
}

#[cfg(target_os = "windows")]
fn get_vol() -> i8 {
    return windows_volume::get_vol();
}

#[cfg(target_os = "windows")]
fn set_vol(level: i8) {
    windows_volume::set_vol(level);
}

fn save_curr_vol() {

}

// setup the volum for the first time (on program launch)
pub fn setup_vol() {

}

pub fn mute_vol(duration: &Duration) {
    println!("Muting for {} seconds", Duration::as_secs(duration));
    
}

pub fn restore_vol() {

}