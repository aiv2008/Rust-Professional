mod rec_mc;

fn main() {
    let amount = 90u32;
    let cashe_num = rec_mc::dp_rec_mc(amount);
    println!("{cashe_num}");
}
