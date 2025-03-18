pub fn goldbach_conjecture() -> String {
    let mut result = String::new();
    let mut v = Vec::<usize>::new();
    let (prime, mut is_prime) = init();
    let mut i = 1;
    while i <= prime[0] {
        let mut n = 1;
        while true {
            if ((prime[i as usize] + 2*n*n ) as usize) > MAX_N{
                break;
            }
            is_prime[(prime[i as usize]  + 2*n*n) as usize ] = 1;
            n+=1;
        }
        i += 1;
    }
    // println!("{:?}", is_prime);
    let mut i = 9;
    while i <= MAX_N {
        if is_prime[i] != 0 {
            i += 2;
            continue;
        }
        // println!("i={}", i);
        v.push(i);
        if v.len() == 2{
            break;
        }
        i += 2;
    }
    // println!("v={:?}", v);
    result.push_str(v[0].to_string().as_str());
    result.push_str(",");
    result.push_str(v[1].to_string().as_str());
    result
    // todo!()
}

// const MAX_N: usize = 1000000;
const MAX_N: usize = 10000;
fn init() ->([u32;MAX_N+5], [u32;MAX_N+5]) {// 线性筛
    let mut prime: [u32;MAX_N+5] = [0;MAX_N+5];
    let mut is_prime: [u32;MAX_N+5] = [0;MAX_N+5];
    let mut i = 2;
    while i <= MAX_N {
        if prime[i] != 1{
            prime[0] += 1;
            prime[prime[0] as usize] = i as u32;
            is_prime[i] = 1;
        }
        let mut j = 1;
        while j <= prime[0] {
            if i * (prime[j as usize] as usize) > MAX_N{
                break;
            }
            prime[i * (prime[j as usize] as usize)] = 1;
            // is_prime[i * (prime[j as usize] as usize)] = 1;
            if i % (prime[j as usize] as usize) == 0{
                break;
            }
            j += 1;
        }
        i += 1;
    }
    // println!("prime[0]={:?}", prime[0]);
    (prime, is_prime)
}
