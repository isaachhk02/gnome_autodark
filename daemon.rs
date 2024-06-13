use std::process::Command;
use chrono::prelude::*;

pub fn start_daemon() {
    let time_n: chrono::DateTime<Utc> = Utc::now();
    if time_n.hour() >= 18 {
        println!("Changing to Dark mode!");
        let _dark_mode = Command::new("bash").args(["-c","gsettings set org.gnome.desktop.interface color-scheme prefer-dark"]).output().expect("Error to change to dark mode");
    }
    if time_n.hour() == 07 {
        println!("Changing to Light mode!");
        let _light_mode = Command::new("bash").args(["-c","gsettings set org.gnome.desktop.interface color-scheme prefer-light"]).output().expect("Error to change to light mode");
    }
    else {
        println!("INFO: No necessary change the mode for the moment!");
    }
}
