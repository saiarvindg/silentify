NO LONGER MAINTAINED AS I'VE SWITCHED OVER TO YOUTUBE MUSIC
# Silentify

> Quiet Spotify Ads

Rewrite of [SilentSpotifyAdsScript](https://github.com/saiarvindg/SilentSpotifyAdsScript)

## Features

- Automatically mutes Spotify Ads and unmutes when song playback resumes.
- Supports Windows<sup>[1]</sup> and Mac<sup>[2]</sup> (no Linux yet)
- Runs in the background (until Spotify is closed)
- Works with Spotify Free tier

## How it works

1. Use Spotify API to get currently playing track and the remaining track duration
2. If the current track is an ad - then follow the next steps. Else skip to step 7
3. Fetch and save<sup>[3]</sup> the application volume of Spotify
4. Mute Spotify's volume using the current system's application volume controls<sup>[1][2]</sup>
5. Wait for the ad to finish
6. Restore the application volume of Spotify to what was stored
7. Calculate remaining time for current track and wait until the track is done.
8. Go to step 1

## Notes

- [1] Windows support requires PowerShell and C# (version 3.5)
- [2] Mac support requires AppleScript
- [3] Stored in application memory. So if you kill the process during an AD (when the volume is muted), the "saved" volume will be zero

## TODO/Improvements

- Remove RSpotify dependency
- Store volume state somewhere on disk so application restarts will not wipe out volume state
- Find a way around using PowerShell and C# to control Windows applications' volume
    - Use Windows API bindings for Rust
- Have something else play over the ADs are running
    - Ex: user's saved tracks on disk, some tracks from another application
