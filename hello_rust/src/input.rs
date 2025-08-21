use std::io;

fn main() {
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Ошибеа ввода");
    let name = name.trim();
    println!("Привет, {name}!");

    let (mut a, mut b, mut c) = (String::new(), String::new(), String::new());
    io::stdin()
        .read_line(&mut a)
        .expect("Ошибеа ввода");

    io::stdin()
        .read_line(&mut b)
        .expect("Ошибеа ввода");

    io::stdin()
        .read_line(&mut c)
        .expect("Ошибеа ввода");

    let (a, b, c) = (a.trim(), b.trim(), c.trim());

    print!("{a}\n{b}\n{c}");

    let (mut a, mut b, mut c, mut d) = (String::new(), String::new(), String::new(), String::new());
    io::stdin()
        .read_line(&mut a)
        .expect("Ошибеа ввода");

    io::stdin()
        .read_line(&mut b)
        .expect("Ошибеа ввода");

    io::stdin()
        .read_line(&mut c)
        .expect("Ошибеа ввода");

    io::stdin()
        .read_line(&mut d)
        .expect("Ошибеа ввода");

    let (a, b, c, d) = (a.trim(), b.trim(), c.trim(), d.trim());

    print!("{3} {2} {1} {0}", a, b, c, d );

    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Ошибеа ввода");

    let num: i32 = a
        .trim()
        .parse()
        .expect("введите число");
    print!("{}", num);
}