pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    let s:Vec<&str> = num_str.split("(").collect();
    let left = s[0];
    let right: Vec<&str> = s[1].split(")").collect();
    let right0 = right[0];
    let cs = &['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f',];
    let array = left.chars();
    
    let mut sum = 0;
    let mut j = array.clone().count() ;
    for a in array.into_iter(){
        sum += (a.to_string()).parse::<u32>().unwrap() * u32::pow(right0.parse::<u32>().unwrap(), (j -1) as u32) ;
        j -= 1;
    }
    let mut before = sum;
    let mut after = sum;
    // let mut result = 0;
    let mut v:Vec<u32> = Vec::new();
    while after > 0 {
        after = before / to_base;
        v.push( before - after * to_base );
        before = after;
    }
    let mut v_reverse:Vec<u32> = Vec::new();
    while !v.is_empty() {
        match v.pop() {
            Some(p)=>{
                v_reverse.push(p);
            },
            _=>{}
        }
    }
    let mut s = String::new();
    
    for c in v_reverse{
        s.push(cs[c.to_string().parse::<usize>().unwrap()]);
    }
    println!("s={}", s);
    s
    // todo!()
}
