use std::f32::consts::PI;

fn main() {
    let name = "iskender";
    println!("{}", hello(name));
    let num = 25;
    println!("Sayımız={}", make_double(num));
    let my_num = 1.0;
    println!("pi çarpı sayımız = {}", multiply_pi(my_num));
    let mut string1 = String::from("hello, ");
    let string2 = "iskender";
    concatenate_strings(&mut string1, string2);
    println!("{}", string1);
}

fn hello(name: &str) -> String {
    format!("Hello!!! {}", name)
}
fn make_double(num: i32) -> i32 {
    num * 2
}
fn multiply_pi(num: f32) -> f32 {
    PI * num
}

fn concatenate_strings(string1: &mut String, string2: &str) {
    string1.push_str(string2)
}
