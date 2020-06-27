
fn main(){

    let numbers = read_file();
    let mut largest = 0;

    //Horizontal
  
    for row in 0..20 {
        for col in 0..16 {
            let num = numbers[row][col] + numbers[row][col+1] + numbers[row][col+2] + numbers[row][col+3];
            if  num > largest {
                largest = num;
            }

        }
    }

    //Vertical
    for col in 0..20 {
        for row in 0..16 {
            let num = numbers[row][col] + numbers[row+1][col] + numbers[row+2][col] + numbers[row+3][col];
            if num > largest{
                largest = num;
            }
        }
    }

    //Diagonal
    


}

fn read_file() -> [[i32;20];20] {
    let numbers = [[0;20]; 20];

    numbers
}