use std::{fmt::{format, Display}, slice::GetDisjointMutError};


// обобщение функций
fn print_value<T: Display>(value: T) {
    println!("{}", value);
}


// обобщение в структурах, но у обоих полей одинаковый тип данных
struct Data<T> {
    d1: T,
    d2: T,
}

// обобщение в структурах, разные типы данных
struct Data2<T, K> {
    d1: T,
    d2: K,
}

impl<T, K> Data2<T, K> {
    fn get_data1(&self) -> &T {
        &self.d1
    }

    fn get_data2(&self) -> &K {
        &self.d2
    }
}

// типажи
trait GetData {
    fn get_data(&self) -> String;
}

impl GetData for i32 {
    fn get_data(&self) -> String {
        format!("Data is {}", *self)
    }
}


// типаж в конкретном виде
trait GetInfo {
    fn get_info(&self) -> String;

    fn hide_info(&self){
        println!("Text");
    }

}

fn get_object(obj: impl GetInfo) {
    obj.hide_info();
}

struct Message {
    author: String,
    text: String,
}

struct Post {
    author: String,
    text: String,
    likes: i32,
}

impl GetInfo for Message {
    fn get_info(&self) -> String {
        format!("{}", self.author)
    }
}


impl GetInfo for Post {
    fn get_info(&self) -> String {
        format!("{}", self.author)
    }
}


// Граница типожа
fn get_object_2<T>(obj: T)
where T: GetInfo + GetData // Более красиво через where
{
    obj.hide_info();
}


// возврат типожа
fn return_object() -> impl GetInfo {
    Message{
        author: "text".to_string(),
        text: "text".to_string()
    }
}


fn main() {
    let string = String::new();
    let num = 777;

    print_value(string);
    print_value(num);
}
