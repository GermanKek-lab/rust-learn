fn main() {
    let option = Some(777);

    println!("{}", option.is_some()); // -> bool
    println!("{}", option.is_none()); // -> bool


    println!("{}", option.unwrap_or_default()); // если None -> дэфолтное значение
    println!("{}", option.unwrap_or(-1)); // -> если None -> заданое значение
    println!("{}", option.unwrap_or_else(|| 0)); // если None -> замыкание и нужно вернуть значение по дэфолту


    println!("{:?}", option.ok_or("Option is none")); // если None -> error message
    println!("{:?}", option.ok_or_else(|| "Option is none")); // если None -> замыкание и нужно возвращать error message


    let option1 = Some(10);
    let option2 = Some(15);
    println!("{:?}", option1.and(option2)); // если один из вариантов None -> None, иначе option1
    println!("{:?}", option1.or(option2)); // если один из вариантов Some -> он выведится, иначе None


    println!("{:?}", option.map(|num| num * num)); // если Some -> замыкание, иначе None
    println!("{:?}", option2.filter(|num| num & 1 == 0)); // как и map, нод для сравнивания


    println!("{:?}", option.and_then(|num| Some(num * num))); // Применят замыкание к значение Some, а не к самому Some
}