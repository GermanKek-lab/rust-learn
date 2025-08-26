use std::io::{Error, Read};
use std::{fs::File, io::ErrorKind};


fn main() {
    // panic!("ERROR"); // просто вызвает панику и выводит текст

    // let list = vec![1, 2, 3];
    // list[100]; // вызывает уже существующую панику

    let path = "file.txt";
    // let f: Result<File, std::io::Error> = File::open(&path); // открывает только! существующий файл
    // let f = match f {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create(&path) {
    //             Ok(file) => file,
    //             _ => panic!("Error creating file"),
    //         } // обработка конкретной ошибки
    //         other => panic!("{:?}", other), // Другая ошибка
    //     },
    // }; // Обработка Result

    // let f: Result<File, std::io::Error> = File::open(&path).unwrap(); // Если успешно -> сразу возвращает файл, иначе поднимает панику
    // let f: Result<File, std::io::Error> = File::open(&path).expect("Error open file"); // Тоже что и unwrap, только при панике вызывает собственный текст ошибки


    // match read_file_data(path) {
    //     Ok(data) => println!("{}", data),
    //     Err(e) => panic!("{:?}", e),
    // }




}

// Распрастронение ошибок
fn read_file_data(path: &str) -> Result<String, Error> {
    let f = File::open(path)?; // Можем вместо match добавить знак "?", если не успешно то возвращыет ошибку в эту функцию
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut data = String::new();

    match f.read_to_string(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(e)
    }
}

/* "?" Нельзя использовать в функциях которые не возвращают Result<>, в main тоже не стоит */