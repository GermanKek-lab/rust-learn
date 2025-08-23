use std::io;


fn main() {
    // let (mut a, mut b) = (String::new(), String::new());
    
    // io::stdin().read_line(&mut a).unwrap();
    // io::stdin().read_line(&mut b).unwrap();
    
    // let (a, b): (i32, i32) = (a.trim().parse().unwrap(), b.trim().parse().unwrap());
    
    // println!("{} + ({}) = {}", a, b, a + b);
    // println!("{} - ({}) = {}", a, b, a - b);
    // println!("{} * ({}) = {}", a, b, a * b);
    // println!("{} / ({}) = {}", a, b, a / b);
    // println!("{} % ({}) = {}", a, b, a % b);

    // let (mut a, mut b) = (String::new(), String::new());
    
    // io::stdin().read_line(&mut a).unwrap();
    // io::stdin().read_line(&mut b).unwrap();
    
    // let (a, b): (i32, i32) = (b.trim().parse().unwrap(), a.trim().parse().unwrap());

    // println!("{}\n{}", a, b);


   let result = |a: i32, b: i32, res: i32, s: &str|{
        println!("{a:#b} {s} {b:#b} = {res:#b}");
        println!("{a:#o} {s} {b:#o} = {res:#o}");
        println!("{a:#x} {s} {b:#x} = {res:#x}");
        println!("");
    };
    
    let (mut a, mut b) = (String::new(), String::new());
    
    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();
    
    let (a, b): (i32, i32) = (a.trim().parse().unwrap(), b.trim().parse().unwrap());
    
    result(a, b, a + b, "+");
    result(a, b, a + b, "-");
    result(a, b, a + b, "*");
    result(a, b, a + b, "/");
    result(a, b, a + b, "%");
}