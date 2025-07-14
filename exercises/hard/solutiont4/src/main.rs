//I AM NOT DONE 
//Calculated according to ISO8061 standard

mod calc_time;

fn main() {
    // let  date_str = String::from("2025-01-18");
    // let result = calc_time::time_info(&date_str);
    let a = calc_time::days_until_next_ashare(2025, 2, 9);
    println!("{}",a);
}
