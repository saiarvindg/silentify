use std::process::Command;

const GET_VOL: &str = "& { . ./AppMuter.ps1; Get-App-Volume -AppName 'spotify'}";
const SET_VOL: &str = "& { . ./AppMuter.ps1; Set-App-Volume -AppName 'spotify' -Volume {}}";
const MUTE: &str = "& { . ./AppMuter.ps1; Get-App-Volume -AppName 'spotify'}";
const UNMUTE: &str = "& { . ./AppMuter.ps1; Get-App-Volume -AppName 'spotify' -Unmute}";

pub fn get_vol() -> i8 {
    let output = Command::new("powershell").args(&["-command", GET_VOL]).output().expect("Failed to get volume from PS script");
    let vol = output.stdout;
    print!("{:?}", vol);
    return 20;
}

pub fn set_vol(level: i8) {

}