#[test]

/*
struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(u: __) {   }
*/






fn main() {
    let _u = Unit;
    do_something_with_unit(_u);

    println!("Success!");
}

// Заполняем пропуск, чтобы функция принимала `Unit`
fn do_something_with_unit(_u: Unit) {
    // Здесь можно использовать `u`, если это необходимо
}
#[allow(dead_code)]

struct Unit;
#[allow(dead_code)]

trait SomeTrait {
    // ...Some behaviors defined here.
}

// Мы не заботимся о полях в `Unit`, но нам важны его поведения.
// Поэтому мы используем структуру без полей и реализуем для нее некоторые поведения.
impl SomeTrait for Unit { }

/*
В сигнатуре функции do_something_with_unit я указал u: Unit, чтобы функция принимала экземпляр структуры Unit.
*/