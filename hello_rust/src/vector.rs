enum Types {
    Int(i32),
    Float(f32),
    Bool(bool),
    Text(String),
}


fn main() {
    let mut list: Vec<i32> = vec![1, 2, 3];
    
    list.push(9); //добавить в вектор
    list.remove(2);// удаление эл-та индекса

    let mut list: Vec<f32> = Vec::new();
    list.push(12.54);

    let mut list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    match list.get(1) {
        Some(el) => {
            println!("{}", el);
        },
        None => println!("Нету"),
    }

    for el in &list {
        println!("{}", el);
    }


    println!("{:?}", list.get(7));

    let list = vec![
        Types::Int(7),
        Types::Float(1.0),
        Types::Bool(true),
        Types::Text("Hello".to_string()),
    ];
}