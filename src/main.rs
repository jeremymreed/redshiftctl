use std::process::Command;
use chrono::prelude::*;

fn execute_redshift(temp: &str) {
    let status = Command::new("/usr/bin/redshift")
                         .args(["-P", "-O", temp])
                         .status()
                         .expect("Failed to execute redshift!");

    println!("Redshift status: {}", status);
}

fn main() {
    let local_now: DateTime<Local> = Local::now();

    let local_morning = Local.ymd(local_now.year(), local_now.month(), local_now.day()).and_hms(6, 0, 0);
    let local_night = Local.ymd(local_now.year(), local_now.month(), local_now.day()).and_hms(20, 0, 0);

    if local_now >= local_morning && local_now < local_night {
        execute_redshift("6500");
    } else {
        execute_redshift("1900");
    }
}
