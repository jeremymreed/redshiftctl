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

pub struct TestPair {
    pub time: DateTime<Local>,
    pub expected: i64,
}

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
    start_temp: i64,
    end_temp: i64,
    current_time: i64,
) -> i64 {
    if start_time == end_time {
        panic!("start_time cannot be the same as end_time!");
    }

    if start_time > end_time {
        panic!("start_time cannot be after end_time!");
    }

    let result = (start_temp * (end_time - current_time) + end_temp * (current_time - start_time))
        / (end_time - start_time);
    return result;
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
        return interpolate_color_temperature(
            night_to_day_start.timestamp(),
            day_start.timestamp(),
            NIGHT_TEMP,
            DAY_TEMP,
            current_time.timestamp(),
        );
    } else if current_time >= day_start && current_time <= day_to_evening_start {
        // day.
        return DAY_TEMP;
    } else if current_time > day_to_evening_start && current_time < evening_start {
        // day_to_evening.
        return interpolate_color_temperature(
            day_to_evening_start.timestamp(),
            evening_start.timestamp(),
            DAY_TEMP,
            EVENING_TEMP,
            current_time.timestamp(),
        );
    } else if current_time >= evening_start && current_time <= evening_to_night_start {
        // evening.
        return EVENING_TEMP;
    } else if current_time > evening_to_night_start && current_time < night_start {
        // evening_to_night.
        return interpolate_color_temperature(
            evening_to_night_start.timestamp(),
            night_start.timestamp(),
            EVENING_TEMP,
            NIGHT_TEMP,
            current_time.timestamp(),
        );
    } else {
        // Shouldn't ever get here.
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

        fn get_test_start_times() -> Vec<TestPair> {
            return vec![
                // Night: Late night.
                TestPair {
                    time: Local.ymd(2022, 5, 20).and_hms(
                        NIGHT_START.0,
                        NIGHT_START.1 + 30,
                        NIGHT_START.2,
                    ),
                    expected: NIGHT_TEMP,
                },
                // Night: Early morning.
                TestPair {
                    time: Local.ymd(2022, 5, 20).and_hms(
                        NIGHT_TO_DAY_START.0 - 1,
                        NIGHT_TO_DAY_START.1,
                        NIGHT_TO_DAY_START.0,
                    ),
                    expected: NIGHT_TEMP,
                },
                // Night to Day.
                TestPair {
                    time: Local.ymd(2022, 5, 20).and_hms(
                        NIGHT_TO_DAY_START.0,
                        NIGHT_TO_DAY_START.1 + 30,
                        NIGHT_TO_DAY_START.2,
                    ),
                    expected: 4200,
                },
                // Day.
                TestPair {
                    time: Local.ymd(2022, 5, 20).and_hms(
                        DAY_START.0,
                        DAY_START.1 + 30,
                        DAY_START.2,
                    ),
                    expected: DAY_TEMP,
                },
                // Day to Evening.
                TestPair {
                    time: Local.ymd(2022, 5, 20).and_hms(
                        DAY_TO_EVENING_START.0,
                        DAY_TO_EVENING_START.1 + 30,
                        DAY_TO_EVENING_START.2,
                    ),
                    expected: 4750,
                },
                // Evening.
                TestPair {
                    time: Local.ymd(2022, 5, 20).and_hms(
                        EVENING_START.0,
                        EVENING_START.1 + 30,
                        EVENING_START.2,
                    ),
                    expected: EVENING_TEMP,
                },
                // Evening to Night.
                TestPair {
                    time: Local.ymd(2022, 5, 20).and_hms(
                        EVENING_TO_NIGHT_START.0,
                        EVENING_TO_NIGHT_START.1 + 30,
                        EVENING_TO_NIGHT_START.2,
                    ),
                    expected: 2725,
                },
            ];
        }

        fn run_test_phase(index: usize) {
            let times = get_test_start_times();
            let test_values = &times[index];

            let actual = get_color_temperature(test_values.time);

            assert_eq!(test_values.expected, actual);
        }

        #[test]
        fn should_return_night_temp_at_late_night() {
            run_test_phase(0);
        }

        #[test]
        fn should_return_night_temp_at_early_morning() {
            run_test_phase(1);
        }

        #[test]
        fn should_return_night_to_day_temp() {
            run_test_phase(2);
        }

        #[test]
        fn should_return_day_temp() {
            run_test_phase(3);
        }

        #[test]
        fn should_return_day_to_evening_temp() {
            run_test_phase(4);
        }

        #[test]
        fn should_return_evening_temp() {
            run_test_phase(5);
        }

        #[test]
        fn should_return_evening_to_night_temp() {
            run_test_phase(6);
        }
    }
}
