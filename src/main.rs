use std::process::Command;
use chrono::prelude::*;

fn execute_redshift(temp: &i64) {
    let temp_str = temp.to_string();

    let status = Command::new("/usr/bin/redshift")
                         .args(["-P", "-O", &temp_str])
                         .status()
                         .expect("Failed to execute redshift!");

    println!("Redshift status: {}", status);
}

fn interpolate_color_temperature(start_time: i64, end_time: i64, current_time: i64, start_temp: i64, end_temp: i64) {
    unimplemented!();
}

fn main() {
    let local_now: DateTime<Local> = Local::now();

    // Start time of these phases.
    let local_morning = Local.ymd(local_now.year(), local_now.month(), local_now.day()).and_hms(6, 0, 0);
    let local_night = Local.ymd(local_now.year(), local_now.month(), local_now.day()).and_hms(20, 0, 0);

    // Color temperature of the static phases.
    let night_temp = 1900;
    let day_temp = 6500;

    if local_now >= local_morning && local_now < local_night {
        execute_redshift(&day_temp);
    } else {
        execute_redshift(&night_temp);
    }
}
