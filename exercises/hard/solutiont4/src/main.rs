//I AM NOT DONE 
//Calculated according to ISO8061 standard

mod calc_time;

fn main() {
    let  date_str = String::from("2025-01-18");
    let result = calc_time::time_info(&date_str);
    println!("{}", result);
}
