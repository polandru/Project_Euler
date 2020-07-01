fn main(){

    let mut div_count = 1;
    let mut triangle = 0;
    let mut inc = 1; 
    let primes = get_primes(1000000);

    while div_count < 500 {
        div_count = 1;
        triangle += inc;
        for p in &primes{
            if p < &triangle {
                if triangle % p == 0 {
                    let mut power = 1;
                    while p.pow(power) < triangle && triangle % p.pow(power) == 0 {
                        power += 1;
                    }
                    div_count *= power;
                }
            } 
        }
        inc += 1;
    }
    print!("Triangle = {}", triangle);
}

fn get_primes(x : u32) -> Vec<usize> {
    let mut primes = Vec::<usize>::with_capacity(x as usize);
    let mut prime = vec![true;x as usize];

    for i in 2usize..(x as usize) {
        if prime[i] == true {
            for t in (i.pow(2)..(x as usize)).step_by(i) {
                 prime[t] = false;
            }
         }
     }
     
     for i in 2usize..(x as usize) {
         if prime[i] {
             primes.push(i);
         }
     }
 

    primes

}