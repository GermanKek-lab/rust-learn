fn main() {
    let num = 5;
    println!("{num}")

    let x = 10;
    println!("x + 10 = {}", x + 10); // "x + 10 = 20"

    let creatData = 2018;
    let yearNow = 2025;
    println!("{}", yearNow - creatData)


    println!("Проверка...\nКод: 1234\nПроверка...\nКод: 5678\nПроверка...\nКод: 9012");

    let (z, y, x) = (5, 10, 15);
    println!("z = {0}, y = {1}, x = {2}", z, y, x); // "z = 5, y = 10, x = 15"

    let (z, y, x) = (100, 50, 30);
    println!("z = {}, y = {}, x = {}", z, y, x); // "z = 100, y = 50, x = 30"

    let (a, b, c, d, e, f) = (5, 10, 15, 20, 25, 30);
    // "a = 5, b = 10, c = 15, d = 20, e = 25, f = 30"
    println!("a = {3}, b = {}, c = {5}, d = {}, e = {4}, f = {}", b, d, f, a, e, c);
    println!("b, d, f, a, e, c");
}