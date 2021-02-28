mod windows_volume;
mod mac_volume;

use std::time::Duration;

pub fn mute_vol(duration: &Duration) {
    println!("Muting for {} seconds", Duration::as_secs(duration));
    
}