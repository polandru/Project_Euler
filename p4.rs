
fn main(){
    let mut largest = 0;
    
    for i in 100..1000 {
        for x in 100..1000{
            let prod = i * x;
            if palindrome(prod) && prod > largest  {
                largest = prod;
            }
        }
    }
    print!("{}",largest);
}

fn palindrome(num: i64) -> bool {
    let string = num.to_string();
    let half = string.len() / 2;

    string.bytes().take(half).eq(string.bytes().rev().take(half))
}
