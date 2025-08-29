// создание документации через "cargo doc"

fn main() {
    hello();
}

// Документирующий комментарий
/// This function simply prints "Hello"
fn hello() {
    println!("Hello");
}


/// - 1
/// - 2
/// - 3
/// 
/// | Title 1 | Title 2|
/// |---------|--------|
/// | Text 1  | Text 2 |
/// 
/// info[^1]
/// 
/// [^1]: Random info
/// * `a` - First number to sum
/// * `b` - Second number to sum
fn sum(a: i32, b: i32) {
    println!("{}", a + b);
}


/// Code struct
struct Code {
    /// Code lines
    lines: i32,

    /// Error
    error: bool,

    /// Author
    author: String,
}

/// Code struct impkemintation
impl Code {
    fn get_info(&self) {
        println!("")
    }
}