fn main() {
    let is_even = |a| a % 2 == 0; // простое замыкание
    let is_even_anotation = |a:i32| -> bool {
        a % 2 == 0
    }; // простое замыкание c анотацией
    println!("{}", is_even(8));


    let print = |a| println!("{}", a);
    print(123);
    // print("Hello".to_string()); // уже ошибка, после print(123) принимает только i32

    /* Замыкание с заимствованием и заимствование с кражей */
    let num: i32 = 7;
    let add = move |x: i32| x + num; // берем копию num
    println!("{}", add(5));

    let num1 = 10;
    let x = || println!("{}", num1);
    // println!("{}", num1); // не можем пользоваться переменной та как она переведена в замыкание


    let mut h = 7;
    let mut increment = || h += 1; // mut если изменяет значение
    increment(); // 
    println!("{}", h);
}