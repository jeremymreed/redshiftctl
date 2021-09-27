use chrono::prelude::*;

fn main() {
    let utc_now: DateTime<Utc> = Utc::now();
    let local_now: DateTime<Local> = Local::now();

    let local_morning = Local.ymd(local_now.year(), local_now.month(), local_now.day()).and_hms(6, 0, 0);
    let local_night = Local.ymd(local_now.year(), local_now.month(), local_now.day()).and_hms(18, 0, 0);

    println!("utc_now: {}", utc_now);
    println!("local_now: {}", local_now);
    println!("local_morning: {}", local_morning);
    println!("local_night: {}", local_night);

    if local_now >= local_morning && local_now < local_night {
        println!("It's daytime!");
    } else {
        println!("It's nighttime!");
    }
}
