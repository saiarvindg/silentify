. "./src/volume_control/windows_volume/AppMuter.ps1"
$unmutes = ("unmute", "UnMute", "unMute","Unmute", "UNMUTE")
if ($args[0] -in $unmutes) {
    Set-App-Mute -AppName 'spotify' -Unmute
} else {
    Set-App-Mute -AppName 'spotify'
}