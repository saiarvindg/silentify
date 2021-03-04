on run argv
	-- see note in GetSpotifyVolume.applescript about not being able to use a variable in tell
	tell application "Spotify"
		set the sound volume to item 1 of argv
	end tell
end run