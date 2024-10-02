#[test]

/*

// Fill the blanks to make the code work
#[__]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!(__, rect1); // Print debug info to stdout
}
*/


// Fill the blanks to make the code work





fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Печатает отладочную информацию в stderr и присваивает значение 30 * scale переменной width
        height: 50,
    };

    dbg!(&rect1); // Печатает отладочную информацию в stderr

    println!("{:?}", rect1); // Печатает отладочную информацию в stdout
}
#[derive(Debug)] // Добавляем этот атрибут для реализации трейта Debug
#[allow(dead_code)]

struct Rectangle {
    width: u32,
    height: u32,
}

/*

Чтобы сделать структуру Rectangle печатаемой с использованием #[derive(Debug)],
и чтобы правильно заполнить пробелы коде, нужно:

Использовать #[derive(Debug)] перед определением структуры.
Использовать форматный вывод с помощью println! и спецификатора формата {:?} для печати rect1.
*/