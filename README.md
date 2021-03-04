# Silentify

> Quiet Spotify Ads

Rewrite of [SilentSpotifyAdsScript](https://github.com/saiarvindg/SilentSpotifyAdsScript)

## Features

- Automatically mutes Spotify Ads and unmutes when song playback resumes.
- Written in Rust
- Supports only Windows<sup>[1]</sup> and Mac<sup>[2]</sup>
- Runs in the background (until Spotify is closed)
- Works with Spotify Free tier

## How it works

1. Use Spotify API to get currently playing track and the remaining track duration
2. If the current track is an ad - then follow the next steps. Else skip to step 7
3. Fetch and save<sup>[3]</sup> the application volume of Spotify
4. Set volume to 0 using the current system's application volume controls<sup>[1][2]</sup>
5. Wait for the ad to finish
6. Retore the application volume of Spotify to what was stored
7. Calculate remaining time for current track and wait until the track is done.
8. Go to step 1

## Notes

- [1] Windows support requires PowerShell with C# (version 3.5)
- [2] Mac support requires AppleScript
- [3] Stored in application memory. So if you kill the process during an AD (when the volume is muted), the "saved" volume will be zero

## TODO/Improvements

- Remove RSpotify dependency
-
