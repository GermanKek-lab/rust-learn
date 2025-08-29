enum Time {
    Morning(String),
    Aftertoon(String),
    Evening(String),
}

struct User {
    name: String,
    age: u16,
}


/*
    if let шаблон = вырожение {
        код...
    } else {
        код... 
    }

    если нужно ещё добавить else if, то лучше использовать match
*/
fn main() {
    let nums = (1, 2, 3, 4, 5);

    if let (1, 2, 3, 4, 5) = nums {
        println!("Yes");
    } else {
        println!("No");
    }

    /* 
    можно заминять элементы на переменные, 
    то будет проверятся только первые числа, 
    а переменные заменяться на соответствующее значение 
    */
    if let (1, 2, 3, 4, d) = nums {
        println!("Yes");
    } else {
        println!("No");
    }

    let t = Time::Morning("Hello, user!".to_string());
    if let Time::Morning(m) = t {
        println!("{}", m);
    } // в отличии от match можно не перебирать все значения


    let user = User{
        name: "German".to_string(),
        age: 30,
    };

    if let User {name: n, age: 30} = user {
        println!("Matched: {}", n);
    }


    
}