fn main() {
    let st_num = [0,1];
    let mut prev_num = st_num[0];
    let mut a_num = st_num[1];

    for rep in 1..12 {
        let temp = prev_num + next_num;
        prev_num = next_num;
        next_num = temp;

        println!("Your number in repetition {} is: {}", rep, temp);
    }
}
