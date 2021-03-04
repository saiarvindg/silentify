(*
	cannot use a variable for the application argument as AppleScript does not allow it
	see following sources:
	https://discussions.apple.com/thread/7694125
	https://macscripter.net/viewtopic.php?id=33964
	https://stackoverflow.com/questions/9300891/applescript-assigning-application-to-a-variable
	https://apple.stackexchange.com/questions/358751/how-can-i-use-variable-in-applescript-for-name-of-browser
*)
tell application "Spotify"
	get the sound volume
end tell