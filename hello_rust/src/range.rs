fn main() {
    let a = 1..5;
    for i in a { // not inclusive range
        println!("{}", i);
    }

    for i in 1..=5 { // inclusive range
        println!("{}", i);
    }

    let numbers: Vec<i32> = (1..=10).rev().collect(); // создание коллекции
    let numbers_2: Vec<i32> = (1..=10).step_by(2).collect(); // создание range с шагом

    for i in 'a'..='z' { // перебор char
        println!("{}", i);
    }
}