// I AM NOT DONE

use std::cmp::Ordering;

mod prime_factor;

fn main() {

    // println!("u128max={}", u128::MAX);
    let number = 97993999919999958437;
    let res = prime_factor::find_max_prime_factor(number);
    println!("{number}'s max prime factor: {res}");


    // println!("{}",prime_factor::div(String::from("199999999999999951437") ,String::from("9523809523809521497")));
}


// fn check_zero(s: &str)->bool{
//     for c in s.chars(){
//         if c != '0'{
//             return false;
//         }
//     }
//     true
// }

// // 非负大整数加法
// fn add(str1: String, str2: String)->String{
//     if check_zero(str1.as_str()) && check_zero(str2.as_str()){
//         return String::from("0");
//     }else if check_zero(str1.as_str()){
//         return str2;
//     }else if check_zero(str2.as_str()){
//         return str1;
//     }
//     let mut ans = String::new();
    
//     let mut v_str1 = Vec::<char>::new();
//     let mut v_str2 = Vec::<char>::new();
    
//     for a in str1.chars().into_iter(){
//         v_str1.push(a);
//     }
//     for a in str2.chars().into_iter(){
//         v_str2.push(a);
//     }
//     let mut i: i32 = v_str1.len() as i32 - 1;
//     let mut j: i32 = v_str2.len() as i32 - 1;
//     let mut flag = 0;
//     while i >= 0 || j >= 0 || flag != 0 {
//         let mut numa = 0;
//         let mut numb = 0;
//         // let mut s_a = "";
//         // let mut s_b = "";
//         let mut s_a = String::new();
//         let mut s_b: String = String::new();
//         if i >= 0{
//             s_a.push(v_str1[i as usize]);
//             // s_a.push('0');
//             println!("s_a={}, i={}", s_a, i);
//             numa = s_a.parse().expect("not a number");
            
//         }else{
//             numa = 0;
//         }
//         i -= 1;
//         if j >= 0{
//             s_b.push(v_str2[j as usize]);
//             // s_b.push('0');
//             println!("s_b={}, j=={}", s_b, j);
//             numb = s_b.parse().expect("not a number");
            
//         }else{
//             numb = 0;
//         }
//         j -= 1;
//         flag = numa + numb + flag;
//         // ans.push('0');
//         ans.push_str((flag % 10).to_string().as_str());
//         flag /= 10;
//         println!("ans={}", ans);
//     }
//     let mut result = String::new();
//     while !ans.is_empty() {
//         match ans.pop() {
//             Some(a)=>{
//                 result.push(a);
//             }
//             ,_=>{

//             }
//         }
        
//     }
//     println!("result={}", result);
//     result
//     // unimplemented!()
// }

// fn swap(str1: String, str2: String)->(String, String){
//     (str2, str1)
// }

// fn sub(mut str1: String, s2: &str) -> String{
//     //处理0的情况
//     if str1.cmp( &s2.to_string()) == Ordering::Equal || (check_zero(str1.as_str()) && check_zero(s2)){
//         return String::from("0");
//     }else if check_zero(str1.as_str()){
//         let mut s = String::from("-");
//         s.push_str(s2);
//         return s;
//     }else if check_zero(s2){
//         return String::from(str1);
//     }
//     let mut negative = 0;
//     let mut str2 = String::from(s2);
//     if str1.len() < s2.len() || (str1.len() == s2.len() && str1.cmp(&s2.to_string()) == Ordering::Less){
//         // println!("str1={}, str2={}, cmp result: {:?} ",str1,&str2,str1.cmp(&str2) );
//         (str1, str2) = swap(str1,s2.to_string());
//         negative = 1;
//     }

//     let mut ans = String::new();
    
//     let mut v_str1 = Vec::<char>::new();
//     let mut v_str2 = Vec::<char>::new();
    
//     for a in str1.chars().into_iter(){
//         v_str1.push(a);
//     }
//     for a in str2.chars().into_iter(){
//         v_str2.push(a);
//     }
//     let mut i: i32 = v_str1.len() as i32 - 1;
//     let mut j: i32 = v_str2.len() as i32 - 1;
//     let mut flag = 0;
//     let mut i: i32 = v_str1.len() as i32 - 1;
//     let mut j: i32 = v_str2.len() as i32 - 1;
//     let mut flag = 0;
//     while i >= 0 || j >= 0 {
//         let mut numa = 0;
//         let mut numb = 0;
//         let mut s_a = String::new();
//         let mut s_b: String = String::new();
//         if i >= 0{
            
//             s_a.push(v_str1[i as usize]);
//             // s_a.push('0');
//             println!("s_a={}, i={}", s_a, i);
//             numa = s_a.parse().expect("not a number");
            
//         }else{
//             numa = 0;
//         }
//         i -= 1;
//         if j >= 0{
//             s_b.push(v_str2[j as usize]);
//             // s_b.push('0');
//             println!("s_b={}, j=={}", s_b, j);
//             numb = s_b.parse().expect("not a number");
            
//         }else{
//             numb = 0;
//         }
//         j -= 1;

//         numa -= flag;
//         println!("numa={}, numb={}", numa, numb);
//         if numa < numb{
//             numa += 10;
//             flag = 1;
//         }else{
//             flag = 0;
//         }
//         // flag = numa + numb + flag;
//         // ans.push_str("0");
//         ans.push_str( (numa-numb).to_string().as_str() );
//         // flag /= 10;
//         println!("ans={}", ans);
//     }
//     let mut v = Vec::<char>::new();     
//     for c in ans.chars(){
//         v.push(c);
//     }
//     i = v.len() as i32 - 1;
//     while v[i as usize] == '0' {
//         i -= 1;
//     }
//     for j in (i+1)..v.len() as i32{
//         v.pop();
//     }
//     if negative == 1 {
//         v.push('-');
//     }
//     let mut result = String::new();
//     while !v.is_empty() {
//         match v.pop() {
//             Some(a)=>{
//                 result.push(a);
//             }
//             ,_=>{

//             }
//         }
        
//     }
//     println!("result={}", result);
//     result
//     // unimplemented!()
// }

// // 判断是否为负， 只需要判断第一位是不是负号即可， 这里不考虑正号的存在，即认为不使用正号
// fn check_negative(str: &str)-> bool{
//     for c in str.chars() {
//         if c == '-'{
//             return true;
//         }
//     }
//     false
// }

// fn mul(mut str1: String,mut str2: String) -> String{
//     let (mut negative,mut negastr1, mut negastr2) = (0, 0, 0);
//     if check_negative(str1.as_str()){
//         negastr1 = 1;
//         str1.remove(0);
//     }
//     if check_negative(str2.as_str()){
//         negastr2 = 1;
//         str2.remove(0);
//     }
//     negative = negastr1 ^ negastr2;
//     let mut ans = String::from("0");
//     if str1.len() < str2.len() || (str1.len() == str2.len() && str1.cmp(&str2) == Ordering::Less){
//         println!("str1={}, str2={}, cmp result: {:?} ",str1,&str2,str1.cmp(&str2) );
//         (str1, str2) = swap(str1,str2);
//     }
//     let (mut v_str1,mut v_str2) = (Vec::<char>::new(),Vec::<char>::new(),);

//     for a in str1.chars().into_iter(){
//         v_str1.push(a);
//     }
//     for a in str2.chars().into_iter(){
//         v_str2.push(a);
//     }
//     let (size1, size2) = (v_str1.len(), v_str2.len());
//     let mut i: i32 = size2 as i32 - 1;
//     while i>=0 {
//         let mut temp = String::new();
//         // let mut temp: Vec<String> = Vec::<String>::new(); 
//         for k in 0..(size2 - i as usize - 1){
//             temp.push('0');
//         }
//         let mut flag = 0;
//         let mut j: i32 = size1 as i32 - 1;
//         println!("1={:?}, 2={:?}", v_str1, v_str2);
//         while j >= 0 {
//             let v_num1: i32 =  v_str1[j as usize].to_string().parse().expect("not a number");
//             let v_num2: i32 =  v_str2[i as usize].to_string().parse().expect("not a number");
//             flag +=  v_num1 * v_num2 ;
//             // temp.push('0');
//             temp.push_str((flag % 10).to_string().as_str());
//             flag /= 10;
//             j -= 1;
//         }
//         if flag != 0{
//             // temp.push('0');
//             temp.push_str(flag.to_string().as_str());
//         }
//         println!("temp={}", temp);
//         let mut result = String::new();
//         while !temp.is_empty() {
//             match temp.pop() {
//                 Some(a)=>{
//                     result.push(a);
//                 }
//                 ,_=>{
    
//                 }
//             }
//         }
//         println!("ans={},result={}",ans, result);
//         ans = add(ans, result);
//         i -= 1;
//     }
//     if negative == 1 {
//         ans.insert(0, '-');
//     }

//     ans
// }

// fn div(mut str1: String, mut str2: String)->String{
//     println!("str1={:?}, str2={:?}", str1, str2);
//     if check_zero(str2.as_str()){
//         return String::from("devitor cannot be 0");
//     }else if check_zero(str1.as_str()){
//         return String::from("0");
//     }
//     let (mut negative,mut negastr1, mut negastr2) = (0, 0, 0);
//     if check_negative(str1.as_str()){
//         negastr1 = 1;
//         str1.remove(0);
//     }
//     if check_negative(str2.as_str()){
//         negastr2 = 1;
//         str2.remove(0);
//     }
//     negative = negastr1 ^ negastr2;
//     let mut point: i32 = 0;
//     if str1.len() < str2.len() || (str1.len() == str2.len() && str1.cmp(&str2) == Ordering::Less){
//         point = 1;
//     }
//     let mut ans = String::new();
//     const l: i32 = 6;
//     for i in 0..l{
//         str1.push('0');
//     }
//     let (mut v_str1,mut v_str2) = (Vec::<char>::new(),Vec::<char>::new(),);
//     for a in str1.chars().into_iter(){
//         v_str1.push(a);
//     }
//     for a in str2.chars().into_iter(){
//         v_str2.push(a);
//     }
    
//     let (size1, size2) = (v_str1.len(), v_str2.len());
//     let mut i: i32 = size2 as i32 - 1;
//     let mut temp = String::new();
//     println!("v_str1={:?}", v_str1);
//     for j in 0..i {
//         if j == v_str1.len() as i32{
//             break;
//         }
//         temp.push(v_str1[j as usize]);
//     }
//     while i < size1 as i32 {
//         temp.push(v_str1[i as usize]);
//         let mut cnt: i32 = 0;
//         while (str2.len() < temp.len() || (str2.len() == temp.len() && str2.cmp(&temp) == Ordering::Less))
//             || temp.cmp(&str2) == Ordering::Equal{
//             temp = sub(temp, str2.as_str());
//             cnt += 1;
//         }
//         if temp.cmp(&String::from("0")) == Ordering::Equal{
//             temp.clear();
//         }
//         // ans.push('0');
//         ans.push_str(cnt.to_string().as_str());
//         i += 1;
//     }
//     i = 0;
//     let mut v_ans = Vec::<char>::new();
//     for c in ans.chars(){
//         v_ans.push(c);
//     }
//     // println!("i={}, v_ans={:?}", i, v_ans);
//     while i < v_ans.len() as i32 && v_ans[i as usize] == '0' {
//         println!("i={}, v_ans={:?}", i, v_ans);
//         i += 1;
//     }
//     for _ in 0..i{
//         ans.remove(0);
//     }
//     if point !=0 {
//         let len = l - ans.len() as i32;
//         for i in 0..len{
//             ans.insert(0, '0');
//         }
//         ans.insert_str(0, "0.");
//     }else{
//         println!("ans.len()={}", ans.len());
//         ans.insert(ans.len()-l as usize, '.');
//     }
//     if negative != 0{
//         ans.insert(0, '-');
//     }
//     ans
// }


// pub fn gcd(a: i128, b: i128)->i128{
//     if b == 0{
//         return a;
//     }
//     gcd(b, a % b)
// }


// fn bmul( a: i128, b: i128,  m: i128)->i128 {  // 快速乘
//     // let c: u128 = (a as u128) * (b as u128) -  (((a as f64) / (m as f64) * (b as f64) + 0.5) as u128) * (m as u128);
//     // println!("a={}, b={}, m={}", a, b, m, );

//     // let left = mul(a.to_string(), b.to_string());
//     // let mut right:f64 = div(mul(a.to_string(), b.to_string()), m.to_string()).parse().expect("not valid number");
//     // right = right + 0.5;
//     // let right = mul((right as i128).to_string(), m.to_string());
//     // let result = sub(left.clone(), right.as_str());
//     // println!(" max={}, result={}, m={}",  u128::MAX, result, m);
//     // let c_i: i128 = result.parse().expect("result is not a number");
//     // println!("left={}", left);
//     let c_i: i128 = a * b -  (((a as f64) / (m as f64) * (b as f64)) as i128) * (m as i128);
//     // println!("c_i={}", c_i);
//     // let c: u128 = ((a as u128) * (b as u128)) as u128 -  (((a as f64) / (m as f64) * (b as f64)) as u128) * (m as u128);
//     // println!("c={}", c);
//     // let ab: u128 = mul(a.to_string(), b.to_string()).parse().expect("m is not a number");
//     // let c: u128 = ab -  (((a as f64) / (m as f64) * (b as f64)) as u128) * (m as u128);

//     // let c_str = sub(mul(a.to_string(), b.to_string()), mul(mul(div(a.to_string(), m.to_string()),b.to_string()), m.to_string()).as_str());
//     // let c: u128 = c_str.parse().expect("c is not a number");
    
//     // ull c = (ull)a * (ull)b - (ull)((long double)a / m * b + 0.5L) * (ull)m;
    
//     if c_i < m {
//         return c_i ;
//     }
//     c_i + m

//     // String::new()
//     // if c < m as u128 {
//     //     return c as i128;
//     // }
//     // c as i128 + m
//   }
  
//   fn qpow( mut x: i128, mut p: i128, m: i128) -> i128 {  // 快速幂
//     let mut ans: i128 = 1;
//     while p != 0 {
//       if p & 1 != 0 {
//         ans = bmul(ans, x, m);
//       }
//       x = bmul(x, x, m);
//       p >>= 1;
//     }
//     return ans;
//   }
  
//   fn Miller_Rabin(p : i128) -> bool {  // 判断素数
//     if p < 2 {
//         return false;
//     }
//     if p == 2 {
//         return true;
//     }
//     if p == 3 {
//         return true;
//     }
//     let mut d = p - 1;
//     let mut r: i128 = 0;
//     while (d & 1) == 0 {
//         r += 1;
//         d >>= 1; 
//      } // 将d处理为奇数
//      let mut k: u128 = 0;
//      while k < 10{
//         k += 1;
//         let mut  rng = thread_rng();
//         let rand: i16 = rng.gen_range(0..i16::MAX);
//         let a: i128  = (rand as i128) % (p - 2) + 2;
//         let mut x: i128 = qpow(a, d, p);
//         if x == 1 || x == p -1{
//             continue;
//         }
//         let mut i =0;
//         while i < r - 1 {
//             i += 1;
//             x = bmul(x, x, p);
//             if x == p - 1 {
//                 break;
//             }
//         }
//         if x != p - 1 {
//             return false;
//         }
//      }
//      true
//   }
  
//   fn Pollard_Rho( x: i128)->i128 {
//     let mut s: i128 = 0;
//     let mut t: i128 = 0;
//     let mut rng = thread_rng();
//     let r: i16 = rng.gen_range(0..i16::MAX);
//     let c: i128 = (r as i128) % (x - 1) + 1;
//     let mut step: i32 = 1;
//     let mut goal: i32 = 1;
//     // int step = 0, goal = 1;
//     let mut  val: i128 = 1;
//     // ll val = 1;
//     while true {
//         step = 1;
//         while step <= goal {
//             step += 1;
//             t = (bmul(t, t, x) + c) % x;
//             val = bmul(val, (t - s).abs() as i128, x);
//             if (step % 127) == 0 {
//                 let d = gcd(val, x);
//                 if d > 1 {
//                     return d;
//                 }
//             }
//         }

//         let d = gcd(val, x);
//         if d > 1 {
//             return d;
//         }

//         goal *= 2;
//         s = t;
//         val = 1;
//     }
//     0
//   }
  
//   fn fac( number: i128, mut max_factor: i128) -> i128 {
//     let mut x = number;
//     if x <= max_factor || x < 2 {
//         return max_factor;
//     }
//     if Miller_Rabin(x) {              // 如果x为质数
//       max_factor = i128::max(max_factor, x);  // 更新答案
//       return max_factor;
//     }
//     let mut p: i128 = x;
//     while p >= x {
//         p = Pollard_Rho(x);  // 使用该算法
//     }
//     while (x % p) == 0{
//         x /= p;
//     } 
//     max_factor = fac(x, max_factor);
//     max_factor= fac(p, max_factor);  // 继续向下分解x和p
//     max_factor
//   }
  