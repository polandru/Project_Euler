use std::fs;

fn main(){

    let numbers = read_file();
    let mut largest = 0;
    //Horizontal
  
    for row in 0..20 {
        for col in 0..=16 {
            let num = numbers[row][col] * numbers[row][col+1] * numbers[row][col+2] * numbers[row][col+3];
            if  num > largest {
                largest = num;
            }

        }
    }

    //Vertical
    for col in 0..20 {
        for row in 0..=16 {
            let num = numbers[row][col] * numbers[row+1][col] * numbers[row+2][col] * numbers[row+3][col];
            if num > largest{
                largest = num;
            }
        }
    }

    //Diagonal
    for col in (1..=16).rev(){

        for row in 0..=(16-col){
            let num = numbers[row][col+row] * numbers[row+1][col+1+row] * numbers[row+2][col+2+row] * numbers[row+3][col+3+row];
            if num > largest{
                largest = num;
            }
        }
    }

    for row in 0..=16 {
        for col in 0..=(16 - row) {
            let num = numbers[row+col][col] * numbers[row+1+col][col+1] * numbers[row+2+col][col+2] * numbers[row+3+col][col+3];
            if num > largest{
                largest = num;
            }
        } 
    }

    for col in 3..20{
        for row in 0..=(col - 3){
            let num = numbers[row][col-row] * numbers[row+1][col-1-row] * numbers[row+2][col-2-row] * numbers[row+3][col-3-row];
            if num > largest{
                largest = num;
            }
        }
    }
  
    for row in (1..=16).rev() {
        for col in (row+3)..=19 {
            let num = numbers[row+19-col][col] * numbers[row+20-col][col-1] * numbers[row+21-col][col-2] * numbers[row+22-col][col-3];
            if num > largest{
                largest = num;
            }
        } 
    }
    print!("Largest {}\n", largest);
}

fn read_file() -> [[isize;20];20] {
    let mut numbers = [[0;20]; 20];
    let file_contents = fs::read_to_string("p11.data").expect("Unable to read");
    
    let mut i = 0;
    for row in 0..20{
        for col in 0..20 {        
            numbers[row][col] = file_contents[i..(i+2)].parse().unwrap();
            i += 3;
        }
    }
    numbers
}