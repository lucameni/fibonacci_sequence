// Fibonacci Number Generator

fn main() {
    let st_num = [0,1];
    let mut p_num = st_num[0];
    let mut a_num = st_num[1];

    for rep in 1..12 {
        let temp = p_num + a_num;
        p_num = a_num;
        a_num = temp;

        println!("Your number in rep {} is: {}", rep, temp);
    }
}