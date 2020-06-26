
fn main(){
    let mut sum = 0isize;
    let mut sum_squares = 0isize;

    for i in 1..101 {
        sum += i;
        sum_squares += i.pow(2);
    }
    print!("{}",sum.pow(2) - sum_squares);

}

