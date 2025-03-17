pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut v = Vec::<u32>::new();
    let mut j: i32 = (CASHES.len() - 1) as i32;
    let mut quantity:u32 = 0;
    let mut remain = amount;
    while remain >= 0 && j >= 0{
        if CASHES[j as usize] <= remain{
            println!("sdfsd");
            v.push(CASHES[j as usize] );
            remain -= CASHES[j as usize];
            quantity += 1;
        }else{
            j -= 1;
        }
        
    }
    println!("v={:?}", v);
    quantity
    // todo!()
}
