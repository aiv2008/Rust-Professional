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
    } else if (month, day) == (cny_month, cny_day) {
        0
    } else {
        let (next_cny_month, next_cny_day) = compute_cny(year + 1);
        let next_cny_day_of_year = day_of_year(year + 1, next_cny_month, next_cny_day);
        (if is_leap_year(year) { 366 } else { 365 } - days_of_year) + next_cny_day_of_year
    };
    let days_until_ashare = days_until_next_ashare(year, month, day);

    let day_of_week_str = match day_of_week {
        0 => "1",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
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

pub fn compute_cny(year: i32) -> (u32, u32) {
    // 已知春节日期表（2000-2030）
    match year {
        2020 => (1, 25),
        2021 => (2, 12),
        2022 => (2, 1),
        2023 => (1, 22),
        2024 => (2, 10),
        2025 => (1, 29),
        2026 => (2, 17),
        2027 => (2, 6),
        2028 => (1, 26),
        2029 => (2, 13),
        2030 => (2, 3),
        _ => {
            // 简化算法回退
            let (dongzhi_month, dongzhi_day) = compute_winter_solstice(year - 1);
            let (month, day) = compute_lunar_month_start(dongzhi_month, dongzhi_day, year, 2);
            (month, day)
        }
    }
}

// 计算冬至日期（公历月/日）
pub fn compute_winter_solstice(year: i32) -> (u32, u32) {
    // 基于更精确的天文算法实现 (修正公式)
    let t = (year - 2000) as f64 / 1000.0;
    let jde0 = 2451900.051619 + 365242.74049 * t - 0.06223 * t.powi(2) - 0.00823 * t.powi(3);
    let t_jde = (jde0 - 2451545.0) / 36525.0;
    let delta = 0.000325 * t_jde.sin() + 0.000017 * t_jde.cos();
    let jd = jde0 + delta - 0.0007;
    jd_to_gregorian(jd)
}

// 计算农历月起始日（朔日）
fn compute_lunar_month_start(dz_month: u32, dz_day: u32, year: i32, offset: u32) -> (u32, u32) {
    // 简化实现：基于朔望月周期29.53059天
    let dz_jd = gregorian_to_jd(year, dz_month, dz_day);
    let target_jd = dz_jd + offset as f64 * 29.530588853;
    jd_to_gregorian(target_jd)
}

// 儒略日转公历 (修正算法)
fn jd_to_gregorian(jd: f64) -> (u32, u32) {
    let jd = jd + 0.5; // 转换为民用日
    let f = jd.fract(); // 小数部分（日的小数）
    let mut j = jd.floor() as i64; // 整数部分

    // 格里高利历修正
    if j > 2299160 {
        let alpha = (j as f64 - 1867216.25) / 36524.25;
        let a = alpha.floor() as i64;
        j += 1 + a - (a / 4);
    }

    let b = j + 1524;
    let c = ((b as f64 - 122.1) / 365.25).floor() as i64;
    let d = (365.25 * c as f64).floor() as i64;
    let e = ((b - d) as f64 / 30.6001).floor() as i64;

    let day = (b as f64 - d as f64 - 30.6001 * (e as f64) + f) as u32;
    let month = if e < 14 { (e - 1) as u32 } else { (e - 13) as u32 };

    (month, day)
}

// 公历转儒略日 (标准算法)
fn gregorian_to_jd(year: i32, month: u32, day: u32) -> f64 {
    let m = month as i32;
    let d = day as f64;
    let y = year as i32;

    let a = (14 - m) / 12; // 整数除法
    let y = y + 4800 - a;
    let m = m + 12 * a - 3;

    let jdn = d + ((153 * m + 2) / 5) as f64 + 365.0 * y as f64
        + (y / 4) as f64 - (y / 100) as f64 + (y / 400) as f64 - 32045.0;

    jdn
}

// 计算下一个A股交易日剩余天数
// 计算下一个A股交易日剩余天数（不含当天）
fn days_until_next_ashare(year: i32, month: u32, day: u32) -> u32 {
    let mut current_year = year;
    let mut current_month = month;
    let mut current_day = day;
    let mut days = 0;

    loop {
        // 计算下一天日期
        (current_year, current_month, current_day) = next_day(current_year, current_month, current_day);
        days += 1;

        // 获取星期几
        let dow = day_of_week(current_year, current_month, current_day);
        // 跳过周末
        if dow >= 5 { // 5=周六, 6=周日
            continue;
        }
        // 跳过节假日
        if is_public_holiday(current_year, current_month, current_day) {
            continue;
        }
        // 找到下一个交易日
        break;
    }

    days
}

// 计算下一天日期
fn next_day(year: i32, month: u32, day: u32) -> (i32, u32, u32) {
    let dim = days_in_month(year, month);
    if day < dim {
        (year, month, day + 1)
    } else if month == 12 {
        (year + 1, 1, 1)
    } else {
        (year, month + 1, 1)
    }
}

// 判断是否为公众假期（中国A股市场）
fn is_public_holiday(year: i32, month: u32, day: u32) -> bool {
    // 固定假期
    match (month, day) {
        (1, 1) => true, // 元旦
        (5, 1) => true, // 劳动节
        (10, 1..=3) => true, // 国庆节
        _ => {
            // 春节假期（初一至初三）
            let (cny_month, cny_day) = compute_cny(year);
            let (_, d1_month, d1_day) = next_day(year, cny_month, cny_day);
            let (_, d2_month, d2_day) = next_day(year, d1_month, d1_day);
            let cny_days = [
                (cny_month, cny_day),
                (d1_month, d1_day),
                (d2_month, d2_day),
            ];
            cny_days.contains(&(month, day))
        }
    }
}
