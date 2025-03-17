pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let mut j = 0;
    let mut sum = 1.0;
    for i in (365-n)..365{
        sum *= ((365-j) as f64)/(365 as f64);
        j += 1;
    }
    1.0 - sum
    // todo!()
}
