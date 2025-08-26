enum Person {
    Adult,
    Underage,
}

enum Say {
    Hi(String),
    Bye(String),
    GM(String),
    GN(String),
}

enum MathOperation {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
}

impl MathOperation {
    fn math_operations(&self) -> f64 {
        match &self {
            MathOperation::Add(a, b) => a + b,
            MathOperation::Sub(a, b) => a - b,
            MathOperation::Mul(a, b) => a * b,
            MathOperation::Div(a, b) => a / b,
        }
    }
}

fn main() {
    // let person = Person::Adult;

    // match person {
    //     Person::Adult => {
    //         println!("Заходи");
    //         println!("Ты взрослый");
    //     },
    //     Person::Underage => println!("Тебе нельзя")
    // }

    // let say = Say::Hi("Hello".to_string());

    // match say {
    //     Say::Hi(h) => println!("{}", h),
    //     Say::Bye(_) => {}
    //     _ => {}
    // }

    let mo = MathOperation::Add(18.0, 10.0);
    let result = mo.math_operations();
    println!("{}", result);
}