use chrono::prelude::*;
use std::process::Command;

const NIGHT_TO_DAY_START: (u32, u32, u32) = (6, 0, 0);
const DAY_START: (u32, u32, u32) = (7, 0, 0);
const DAY_TO_EVENING_START: (u32, u32, u32) = (17, 0, 0);
const EVENING_START: (u32, u32, u32) = (18, 0, 0);
const EVENING_TO_NIGHT_START: (u32, u32, u32) = (20, 0, 0);
const NIGHT_START: (u32, u32, u32) = (22, 0, 0);

// Color temperature of the static phases.
const DAY_TEMP: i64 = 6500;
const EVENING_TEMP: i64 = 3000;
const NIGHT_TEMP: i64 = 1900;

// Temporary.. For now.
const NIGHT_TO_DAY_TEMP: i64 = 2200;
const DAY_TO_EVENING_TEMP: i64 = 5000;
const EVENING_TO_NIGHT_TEMP: i64 = 2200;

fn execute_redshift(temp: &i64) {
    let temp_str = temp.to_string();

    let status = Command::new("/usr/bin/redshift")
        .args(["-P", "-O", &temp_str])
        .status()
        .expect("Failed to execute redshift!");

    println!("Redshift status: {}", status);
}

fn generate_datetime(
    current_time: DateTime<Local>,
    start_time: (u32, u32, u32),
) -> DateTime<Local> {
    return Local
        .ymd(
            current_time.year(),
            current_time.month(),
            current_time.day(),
        )
        .and_hms(start_time.0, start_time.1, start_time.2);
}

fn interpolate_color_temperature(
    start_time: i64,
    end_time: i64,
    current_time: i64,
    start_temp: i64,
    end_temp: i64,
) {
    unimplemented!();
}

fn get_color_temperature(current_time: DateTime<Local>) -> i64 {
    // Start times.
    let night_to_day_start = generate_datetime(current_time, NIGHT_TO_DAY_START);
    let day_start = generate_datetime(current_time, DAY_START);
    let day_to_evening_start = generate_datetime(current_time, DAY_TO_EVENING_START);
    let evening_start = generate_datetime(current_time, EVENING_START);
    let evening_to_night_start = generate_datetime(current_time, EVENING_TO_NIGHT_START);
    let night_start = generate_datetime(current_time, NIGHT_START);

    // Phase checks.
    if current_time >= night_start || current_time <= night_to_day_start {
        // night.
        println!("night");
        return NIGHT_TEMP;
    } else if current_time > night_to_day_start && current_time < day_start {
        // night_to_day.
        println!("night_to_day");
        return NIGHT_TO_DAY_TEMP;
    } else if current_time >= day_start && current_time <= day_to_evening_start {
        // day.
        println!("day");
        return DAY_TEMP;
    } else if current_time > day_to_evening_start && current_time < evening_start {
        // day_to_evening.
        println!("day_to_evening");
        return DAY_TO_EVENING_TEMP;
    } else if current_time >= evening_start && current_time <= evening_to_night_start {
        // evening.
        println!("evening");
        return EVENING_TEMP;
    } else if current_time > evening_to_night_start && current_time < night_start {
        // evening_to_night.
        println!("evening_to_night");
        return EVENING_TO_NIGHT_TEMP;
    } else {
        // Shouldn't ever get here.
        println!("invalid!");
        panic!("Invalid time, did not match a phase/transition as expected!");
    }
}

fn main() {
    let current_time: DateTime<Local> = Local::now();

    let temperature: i64 = get_color_temperature(current_time);

    execute_redshift(&temperature);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_color_temperature_tests {
        use super::*;

        #[test]
        fn should_return_night_temp_at_late_night() {
            let time = Local.ymd(2022, 5, 20).and_hms(23, 0, 0);

            let actual = get_color_temperature(time);

            assert_eq!(NIGHT_TEMP, actual);
        }

        #[test]
        fn should_return_night_temp_at_early_morning() {
            let time = Local.ymd(2022, 5, 20).and_hms(2, 0, 0);

            let actual = get_color_temperature(time);

            assert_eq!(NIGHT_TEMP, actual);
        }

        #[test]
        fn should_return_night_to_day_temp_() {
            let time = Local.ymd(2022, 5, 20).and_hms(6, 30, 0);

            let actual = get_color_temperature(time);

            assert_eq!(NIGHT_TO_DAY_TEMP, actual);
        }
    }
}
