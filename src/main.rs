use std::char;

fn main() {
    println!("Hello, world!");
}

#[test]
fn unit_test() {
    println!("Hello unit test rust");
}

#[test]
fn variable() {
    let name: &str = "Deo";
    println!("{name}");
}

#[test]
fn variable_mut() {
    let mut name: &str = "Ini pertama";
    println!("{name}");

    name = "Diubah lagi lah";

    println!("{name}");
}

// shadowing tidak disarankan
#[test]
fn shadowing_variable() {
    let r0: i32 = 1;
    println!("{r0}");

    let r0: &str = "Ok";
    println!("{r0}");
}

#[test]
fn number_type() {
    let i: u32 = u32::MAX;
    let j: i32 = i32::MAX;
    let k: f32 = f32::MAX;

    println!("MAX u32 = {i}");
    println!("MAX i32 = {j}");
    println!("MAX f32 = {k}");
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{a}");

    let b: i32 = a as i32;
    println!("{b}");

    let c: i32 = i32::MAX;
    println!("{c}");

    // Integer Overflow
    let d: i16 = c as i16;
    println!("{d}");
}

#[test]
fn number_operation() {
    let a: i8 = 1;
    let b: i8 = 2;
    let c: i8 = 3;

    let add: i8 = a + b + c;
    let div: f32 = (c as f32) / b as f32;
    // etc

    println!("{add}");
    println!("{div}");

    let mut e: i8 = 12;
    e += 12;
    println!("{e}");
}

#[test]
fn bool_type() {
    let a: bool = true;
    let b: bool = false;

    println!("{} {}", !a, b);
}

#[test]
fn comparison() {
    let firs_number: i8 = 12;
    let second_number: i8 = 9;

    let comp: bool = firs_number > second_number;
    println!("{comp}");

    let sec_comp: bool = true && true;
    println!("{sec_comp}");
}

#[test]
fn char_type() {
    let c1: char = 'a';
    println!("{c1}");
}

#[test]
fn tuple_type() {
    let data: (i32, f32, &str) = (10, 3.14, "Hi");
    println!("{:?}", data);

    // bongkar 1 per 1
    // let d_0: i32 = data.0;
    // let d_1: f32 = data.1;
    // let d_2: &str = data.2;

    // desctruction
    let (d_0, d_1, d_2) = data;

    println!("{d_0}, {d_1}, {d_2}");
}