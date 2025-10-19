use std::thread::sleep;
use std::time::{Instant, Duration};
use chrono::{FixedOffset, TimeZone, Utc};

fn main() {
    metric_clock();
}
fn metric_clock() {
    let mut last_decimal_second = 0;
    let utc_timezone = 2;
    let timezone= utc_timezone * 3600;
    loop {
        let cycle_start = Instant::now();

        let current_decimal_todaystamp = get_decimal_todaystamp(&timezone);
        let current_decimal_second = get_decimal_seconds(&timezone);
        let current_decimal_minute = get_decimal_minutes(&timezone);
        let current_decimal_hour = get_decimal_hours(&timezone);

        // Only print on a new decimal second
        if current_decimal_second != last_decimal_second {
            //println!("Decimal Timestamp: {}", current_decimal_todaystamp);
            println!("Current metric Second: {}", current_decimal_second);
            println!("Current metric minute: {}", current_decimal_minute);
            println!("Current metric hour: {}", current_decimal_hour);
            last_decimal_second = current_decimal_second;
        }

        // Calculate how long this cycle took
        let cycle_time = cycle_start.elapsed();

        // Sleep for the remaining time to make each cycle exactly 864ms
        if cycle_time < Duration::from_nanos(864000000) {
            sleep(Duration::from_nanos(864000000) - cycle_time);
        }
    }
}

fn get_decimal_todaystamp(timezone: &i32) -> u64 {
    let now_utc0 = Utc::now();
    //println!("Current Time: {}", now.format("%H:%M:%S%.3f"));
    let timestamp_ms_utc0 = now_utc0.timestamp_millis(); // Milliseconds since Unix epoch in UTC

    let offset_ms =  timezone * 1000;
    let local_timestamp_ms = timestamp_ms_utc0 + offset_ms as i64;

    let ms_in_day = 86400000;
    let ms_since_midnight = local_timestamp_ms % ms_in_day;

    // Convert to decimal seconds using integer arithmetic
    let decimal_total = (ms_since_midnight as u64 * 100000) / ms_in_day as u64;

    decimal_total
}


fn get_decimal_seconds(timezone: &i32) -> u64 {
    let decimal_timestamp = get_decimal_todaystamp(timezone);
    decimal_timestamp % 100
}
fn get_decimal_minutes(timezone: &i32) -> u64 {
    let decimal_timestamp = get_decimal_todaystamp(timezone);
    (decimal_timestamp / 100) % 100
}
fn get_decimal_hours(timezone: &i32) -> u64 {
    let decimal_timestamp = get_decimal_todaystamp(timezone);
    decimal_timestamp / 10000
}