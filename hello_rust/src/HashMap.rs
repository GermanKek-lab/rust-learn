use std::collections::HashMap;


fn main() {
    let mut map: HashMap<&String, i32> = HashMap::new(); // создание хэш-таблицы
    let n1 = "German".to_string();
    let n2 = "Vlad".to_string();

    map.insert(&n1, 10); // добавление эл-та

    println!("{:?}", map); // вывод
    println!("{}", map[&n1]); // получение эл-та 

    match map.get(&n1) {
        Some(mark) => {
            println!("{}", mark);
        }
        None => println!("None")
    }

    for (name, mark) in &map {
        /* Перебор */
        println!("{} has {}!", name, mark)
    }

    map.entry(&n2).or_insert(12); // проверяет есть уже имя в map, если нету то присваивает ему 9


    let s = "I am study in ITMO ITMO";
    let mut count_map = HashMap::new();
    for i in s.split_whitespace() {
        let count = count_map.entry(i).or_insert(0); // если есть ключ возвращает изменяемую ссылку
        *count += 1;
    }

    println!("{:?}", count_map);

}