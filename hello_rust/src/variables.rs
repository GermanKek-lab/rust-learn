fn main() {
    // Изменяемая переменная
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x теперь = {}", x);

    // задание не нужных переменных
    let _a: i32 = 10;
    let _b: f64 = 3.14;
    let _c: bool = true;

    // задание нескольких переменных
    let (x, y) = (1, 2); // x = 1, y = 2
    let (mut a, mut b) = (3, 4); // a = 3, b = 4

    // задание
    let name: &str = "Герман";
    let mut age: i32 = 19;

    age += 1;

    println!("Привет, {}! Тебе скоро будет {} лет.", name, age);
}