pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let nums =[0,1,1,2,3,5,8,13,21,34,55,89,144,233,377,610,987,1597,2584,4181,6765,10946,17711,28657,46368,75025,121393,196418,317811,514229,832040];
    let mut sum = 0;
    let mut i = 0;
    while nums[i as usize] <= threshold{
        let temp = nums[i as usize];
        if temp % 2 != 0{
            sum += temp;
        }
        i += 1;
    }
    // return nums[n as usize];
    sum
    // todo!()
}
