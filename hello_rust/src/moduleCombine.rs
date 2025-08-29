// файл уже евляется модулем
mod module;

use module::{say_test, test2};
use module::test1::hello as great;


fn main() {
    say_test();
    great();
    test2::hello();

    // super::test1::hello(); // позволяет видеть всю область файла
    // чтобы получать модули из папки, создаем mod.rs и прописываем "mod папка"
}