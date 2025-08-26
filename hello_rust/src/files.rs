use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::fs;

fn main() {
    File::create("text.txt").expect("Error creating file text.txt"); // создание файл
    /* Если файл существует, то все удаляется, иначе открывает его в Read Only */

    File::open("text.txt").expect("Error opening text.txt"); // открытие фалйа
    /* Если файла не существует, то будет ошибка, иначе открывает его в Write Only */

    let path = "data.txt";
    let mut f = File::create(path).unwrap();
    f.write_all("World!".as_bytes()); // запись в файл, принимает только байты!
    f.write_all(b"World!"); // или так


    let mut f = File::open(path).unwrap();
    let mut file_data = String::new();
    f.read_to_string(&mut file_data).unwrap();

    println!("{}", file_data);

    let mut f = OpenOptions::new() // задает опции для открытия файла
        .read(true) // можно считывать
        .write(true)// можно писать
        .create(true) // создает, если нету
        .open(path)
        .unwrap();

    f.write_all(b"lets go").unwrap();
    let mut file_data = String::new();
    f.read_to_string(&mut file_data).unwrap();

    println!("{}", file_data);

    std::fs::rename(path, "text.txt").unwrap(); // переименование файла
    std::fs::remove_file("text.txr").unwrap(); // удаление файла

}