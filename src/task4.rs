#[test]

/*

// Fill the blank and fix the error without adding/removing new line
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let p = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    __ = String::from("sunfei");

    println!("Success!");
}
*/




fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}
// Fill the blank and fix the error without adding/removing new line
struct Person {
    name: String,
    age: u8,
}

/*
В коде есть ошибка, потому что попытка изменить поле структуры Person,
но структуры по умолчанию являются неизменяемыми.
Нужно либо сделать объект изменяемым с помощью ключевого слова mut,
либо убрать попытку изменения поля age.
*/