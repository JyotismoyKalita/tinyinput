use tinyinput::read;

fn main() {
    let x: i32 = read("Enter integer: ").unwrap();
    let y: f64 = read("Enter float: ").unwrap_or_default();
    let s: String = read("Enter string: ").unwrap();

    println!("x = {x}, y = {y}, s = {s}");
}
