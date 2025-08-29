pub struct Circle {
    pub radius: i32, // в структуре нужно задавать поля тоже публичными
}


pub fn say_test() { // функция pub - доступна в проекте
    println!("Test");
}

pub mod test1 {
    pub fn hello() {
        println!("Test 1");
    }
}

pub mod test2 {
    pub fn hello() {
        println!("Test 2");
    }
}