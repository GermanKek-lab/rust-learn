fn main() {
    let lit: &str = "Literal"; // строковый литерал &str
    let stroka = String::new(); // пустая строка
    let s1 = "Hello".to_string(); // из литерала в строку
    let s2 = String::from(" World"); // из литера в строку

    let res2 = format!("{} {}", s1, s2); // соеденение, но s1 доступна после операции
    let res1 = s1 + &s2; // соеденение строки первая обязательно без ссылки, после опереации s1 недоступна

    for i in s2.chars(){
        /* Перебор символов */
    }

    for i in s2.bytes() {
        /* Перебор байтов */
    }

    let en = "Yes".to_string(); // 3 бита, один символ латиницы = 1 бит
    let rus = "Да".to_string(); // 4 бита, один симмвол кириллицы = 2 битам
}