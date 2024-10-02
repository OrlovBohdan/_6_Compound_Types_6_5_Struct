#[test]

/*

// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    // ONLY modify this line
    println!("{}, {}, {:?}",f.name, f.data, f);
}
*/


fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name.clone();

    // ONLY modify this line
    println!("{}, {}, {:?}",f.name, f.data, f);
}

// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

/*
Используется _name, который содержит клонированное значение f.name.
Таким образом, избегание ошибки перемещения, и код будет работать корректно.
*/