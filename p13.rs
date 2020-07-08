use std::fs;

fn main(){

    let mut sum_first_half = 0u128;
    let mut sum_second_half = 0u128;
 
    let file_contents = fs::read_to_string("p13.data").expect("Unable to read");

    let mut read_index = 0;

    for _i in 0..100 {
        sum_first_half += file_contents[read_index..(read_index + 25)].parse::<u128>().unwrap();
        sum_second_half += file_contents[(read_index + 25)..(read_index + 50)].parse::<u128>().unwrap();
        read_index += 51;     
    }
    let carry = sum_second_half/1_00000_00000_00000_00000_00000;

    println!("Result: {}", sum_first_half + carry);



}