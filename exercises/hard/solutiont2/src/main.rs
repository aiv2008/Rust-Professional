// I AM NOT DONE

use std::cmp::Ordering;

mod prime_factor;

fn main() {
    // let number = 99999999951437;
    // let res = prime_factor::find_max_prime_factor(number);
    // println!("{number}'s max prime factor: {res}");

    // println!("{:?}", String::from("1111").cmp(&String::from("33")));

   println!("{}", sub( String::from("33"),String::from("1111")));
}


fn check_zero(s: &str)->bool{
    for c in s.chars(){
        if c != '0'{
            return false;
        }
    }
    true
}

// 非负大整数加法
fn add(str1: String, str2: String)->String{
    if check_zero(str1.as_str()) && check_zero(str1.as_str()){
        return String::from("0");
    }else if check_zero(str1.as_str()){
        return str2;
    }else if check_zero(str2.as_str()){
        return str1;
    }
    let mut ans = String::new();
    
    let mut v_str1 = Vec::<char>::new();
    let mut v_str2 = Vec::<char>::new();
    
    for a in str1.chars().into_iter(){
        v_str1.push(a);
    }
    for a in str2.chars().into_iter(){
        v_str2.push(a);
    }
    let mut i: i32 = v_str1.len() as i32 - 1;
    let mut j: i32 = v_str2.len() as i32 - 1;
    let mut flag = 0;
    while i >= 0 || j >= 0 || flag != 0 {
        let mut numa = 0;
        let mut numb = 0;
        // let mut s_a = "";
        // let mut s_b = "";
        let mut s_a = String::new();
        let mut s_b: String = String::new();
        if i >= 0{
            s_a.push(v_str1[i as usize]);
            // s_a.push('0');
            println!("s_a={}, i={}", s_a, i);
            numa = s_a.parse().expect("not a number");
            
        }else{
            numa = 0;
        }
        i -= 1;
        if j >= 0{
            s_b.push(v_str2[j as usize]);
            // s_b.push('0');
            println!("s_b={}, j=={}", s_b, j);
            numb = s_b.parse().expect("not a number");
            
        }else{
            numb = 0;
        }
        j -= 1;
        flag = numa + numb + flag;
        // ans.push('0');
        ans.push_str((flag % 10).to_string().as_str());
        flag /= 10;
        println!("ans={}", ans);
    }
    let mut result = String::new();
    while !ans.is_empty() {
        match ans.pop() {
            Some(a)=>{
                result.push(a);
            }
            ,_=>{

            }
        }
        
    }
    println!("result={}", result);
    result
    // unimplemented!()
}

fn swap(str1: String, str2: String)->(String, String){
    (str2, str1)
}

fn sub(mut str1: String, mut str2: String) -> String{
    //处理0的情况
    if str1.cmp( &str2) == Ordering::Equal || (check_zero(str1.as_str()) && check_zero(str2.as_str())){
        return String::from("0");
    }else if check_zero(str1.as_str()){
        let mut s = String::from("-");
        s.push_str(&str2);
        return s;
    }else if check_zero(str2.as_str()){
        return String::from(str1);
    }
    let mut negative = 0;
    
    if str1.len() < str2.len() || (str1.len() == str2.len() && str1.cmp(&str2) == Ordering::Less){
        println!("str1={}, str2={}, cmp result: {:?} ",str1,&str2,str1.cmp(&str2) );
        (str1, str2) = swap(str1,str2);
        negative = 1;
    }

    let mut ans = String::new();
    
    let mut v_str1 = Vec::<char>::new();
    let mut v_str2 = Vec::<char>::new();
    
    for a in str1.chars().into_iter(){
        v_str1.push(a);
    }
    for a in str2.chars().into_iter(){
        v_str2.push(a);
    }
    let mut i: i32 = v_str1.len() as i32 - 1;
    let mut j: i32 = v_str2.len() as i32 - 1;
    let mut flag = 0;
    let mut i: i32 = v_str1.len() as i32 - 1;
    let mut j: i32 = v_str2.len() as i32 - 1;
    let mut flag = 0;
    while i >= 0 || j >= 0 {
        let mut numa = 0;
        let mut numb = 0;
        let mut s_a = String::new();
        let mut s_b: String = String::new();
        if i >= 0{
            s_a.push(v_str1[i as usize]);
            // s_a.push('0');
            println!("s_a={}, i={}", s_a, i);
            numa = s_a.parse().expect("not a number");
            
        }else{
            numa = 0;
        }
        i -= 1;
        if j >= 0{
            s_b.push(v_str2[j as usize]);
            // s_b.push('0');
            println!("s_b={}, j=={}", s_b, j);
            numb = s_b.parse().expect("not a number");
            
        }else{
            numb = 0;
        }
        j -= 1;

        numa -= flag;
        println!("numa={}, numb={}", numa, numb);
        if numa < numb{
            numa += 10;
            flag = 1;
        }else{
            flag = 0;
        }
        // flag = numa + numb + flag;
        // ans.push_str("0");
        ans.push_str( (numa-numb).to_string().as_str() );
        // flag /= 10;
        println!("ans={}", ans);
    }
    let mut v = Vec::<char>::new();     
    for c in ans.chars(){
        v.push(c);
    }
    i = v.len() as i32 - 1;
    while v[i as usize] == '0' {
        i -= 1;
    }
    for j in (i+1)..v.len() as i32{
        v.pop();
    }
    if negative == 1 {
        v.push('-');
    }
    let mut result = String::new();
    while !v.is_empty() {
        match v.pop() {
            Some(a)=>{
                result.push(a);
            }
            ,_=>{

            }
        }
        
    }
    println!("result={}", result);
    result
    // unimplemented!()
}