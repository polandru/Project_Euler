fn main(){
   const N: usize = 2000001;
   let mut prime = [true;N];


   for i in 2..N {
       if prime[i] == true {
           for x in (i.pow(2)..N).step_by(i) {
                prime[x] = false;
           }
        }
    }

   let mut sum = 0;

   for i in 2..N {
       if prime[i] {
           sum += i;
       }
   }
   
   print!("Sum: {}", sum);
}
