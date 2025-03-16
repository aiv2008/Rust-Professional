pub fn new_count_distinct(input_str: &str) -> usize {
    // todo!()
    let mut array:Vec<&str> = input_str.split(",").collect();
    if array.len() <= 1{
        array.len() as usize
    }else{
        for i in 1..array.len(){
            for j in 0..i{
                let k = i-j;
                if array[k] < array[k-1]{
                    let temp = array[k].clone();
                    array[k] = array[k-1].clone();
                    array[k-1] = temp.clone();
                }
            }
        }
        let mut result = Vec::<&str>::new();
        for i in 0..array.len(){
            if !result.contains(&array[i]){
                result.push(array[i]);
            }
        }
        result.len()
    }

    // unimplemented!()
}
