
fn main(){
    let mut largest_n = 1;
    let mut longest_chain = 1;
   for i in 1..1_000_000{
       let chain = collatz_chain(i);
       if chain > longest_chain {
           largest_n = i;
           longest_chain = chain;
       }
   }

   println!("Largest n: {} with a chain of {}",largest_n,longest_chain);
}

fn collatz_chain(n :usize) -> usize {
    if n == 1 {
        return 1;

    } else if n % 2 == 0 {
        return 1 + collatz_chain(n / 2);
    }else{ 
        return 1 + collatz_chain(3 * n + 1);
    }

    
    

}