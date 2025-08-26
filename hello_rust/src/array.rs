use std::io;


fn main() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut n = String::new();
    
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
   
    array[0] += n;
    array[1] -= n;
    array[2] *= n;
    array[3] /= n;
    array[4] %= n;
    
    print!("{:?}", array);
}