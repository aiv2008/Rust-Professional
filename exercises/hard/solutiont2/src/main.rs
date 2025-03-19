// I AM NOT DONE

use std::cmp::Reverse;



mod prime_factor;

fn main() {
    // let number = 99999999951437;
    // let res = prime_factor::find_max_prime_factor(number);
    // println!("{number}'s max prime factor: {res}");

    println!("{}", check_zero("100000"));
}

fn check_zero(s: &str)->bool{
    for c in s.chars(){
        if c != '0'{
            return false;
        }
    }
    true
}

fn add<'a>(str1: &'a str, str2: &'a str)->&'a str{
    if check_zero(str1) && check_zero(str2){
        return "0";
    }else if check_zero(str1){
        return str2;
    }else if check_zero(str2){
        return str1;
    }
    unimplemented!()
}