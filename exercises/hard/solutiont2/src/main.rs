// I AM NOT DONE



mod prime_factor;

fn main() {
    let number = 99999999951437;
    let res = prime_factor::find_max_prime_factor(number);
    println!("{number}'s max prime factor: {res}");
}

fn breakdown(number: u128)->Vec<u128>{
    let mut result = Vec::<u128>::new();
    let mut i = 2;
    let mut n = number;
    while i * i < n {
        if n % i == 0{
            while n % i == 0 {
                n /= i;
            }
            result.push(i);
        }
        i += 1;
    }
    if n != 1{
        result.push(n);
    }
    result
}