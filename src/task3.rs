#[test]

/*

// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Point(__, __, __);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let (x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(__, 255);
 }
*/






fn main() {
    let v = Color(0, 127, 255);  // Здесь используем структуру Color
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    assert_eq!(p.0, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}

struct Color(i32, i32, i32);
#[allow(dead_code)]

struct Point(i32, i32, i32);
