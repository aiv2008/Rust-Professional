pub fn time_info(time: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();
    let year: i32 = parts[0].parse().unwrap();
    let month: u32 = parts[1].parse().unwrap();
    let day: u32 = parts[2].parse().unwrap();

    let days_of_year = day_of_year(year, month, day);
    let day_of_week = day_of_week(year, month, day);
    let week_number = week_number(year, days_of_year);
    let days_remaining = if is_leap_year(year) { 366 } else { 365 } - days_of_year;
    let (cny_month, cny_day) = compute_cny(year);
    let cny_day_of_year = day_of_year(year, cny_month, cny_day);
    let days_until_cny = if (month, day) < (cny_month, cny_day) {
        cny_day_of_year - days_of_year
    } else {
        let (next_cny_month, next_cny_day) = compute_cny(year + 1);
        let next_cny_day_of_year = day_of_year(year + 1, next_cny_month, next_cny_day);
        (if is_leap_year(year) { 366 } else { 365 } - days_of_year) + next_cny_day_of_year
    };
    let days_until_ashare = days_until_next_ashare(day_of_week);

    let day_of_week_str = match day_of_week {
        0 => "周一",
        1 => "周二",
        2 => "周三",
        3 => "周四",
        4 => "周五",
        5 => "周六",
        6 => "周日",
        _ => panic!("invalid day of week"),
    };

    format!(
        "{},{},{},{},{},{}",
        week_number, day_of_week_str, days_of_year, days_remaining, days_until_cny, days_until_ashare
    )
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => panic!("invalid month"),
    }
}

fn day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let mut doy = day;
    for m in 1..month {
        doy += days_in_month(year, m);
    }
    doy
}

fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    let (adjusted_month, adjusted_year) = if month < 3 {
        (month + 12, year - 1)
    } else {
        (month, year)
    };
    let q = day as i32;
    let m = adjusted_month as i32;
    let k = adjusted_year % 100;
    let j = adjusted_year / 100;
    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    ((h - 2 + 7) % 7) as u32
}

fn week_number(year: i32, doy: u32) -> u32 {
    let jan1_dow = day_of_week(year, 1, 1);
    let first_week_start = if jan1_dow <= 3 {
        1
    } else {
        8 - jan1_dow as u32
    };

    if doy < first_week_start {
        let prev_year = year - 1;
        let prev_jan1_dow = day_of_week(prev_year, 1, 1);
        let prev_first_week_start = if prev_jan1_dow <= 3 {
            1
        } else {
            8 - prev_jan1_dow as u32
        };
        let prev_days_in_year = if is_leap_year(prev_year) { 366 } else { 365 };
        (prev_days_in_year - prev_first_week_start + 1 + 6) / 7
    } else {
        let week_num = (doy - first_week_start + 1 + 6) / 7;
        if week_num == 53 {
            let next_jan1_dow = day_of_week(year + 1, 1, 1);
            let next_first_week_start = if next_jan1_dow <= 3 {
                1
            } else {
                8 - next_jan1_dow as u32
            };
            if next_first_week_start <= 3 {
                1
            } else {
                53
            }
        } else {
            week_num
        }
    }
}

fn compute_cny(year: i32) -> (u32, u32) {
    let t = year - 1980;
    let new_year = 20.69115 + 0.242194 * t as f64 - (t / 4) as f64;
    let day = new_year.floor() as u32;
    let (month, day) = if day > 31 {
        (2, day - 31)
    } else {
        (1, day)
    };
    (month, day)
}

fn days_until_next_ashare(day_of_week: u32) -> u32 {
    match day_of_week {
        0 => 1,
        1 => 1,
        2 => 1,
        3 => 1,
        4 => 2,
        5 => 1,
        6 => 0,
        _ => panic!("invalid day of week"),
    }
}
