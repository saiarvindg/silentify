on run argv
	set appName to item 1 of argv
	tell application appName
		get properties "sound volume" of application
	end tell
end run