fn main(){
    for x in 1i32..1001 {
        for y  in 1i32..1001 {          
            if ( (x.pow(2) as f64  + y.pow(2) as f64).sqrt() as i64)  as f64   == ((x.pow(2) + y.pow(2)) as f64).sqrt()  {
                if x + y + ((x.pow(2) as f64 + y.pow(2) as f64).sqrt() as i32) == 1000 {
                    print!("{} {} {}", x, y, ((x.pow(2) as f64 + y.pow(2) as f64).sqrt() as i32) ) 
                }
            }

        }
    }

}
