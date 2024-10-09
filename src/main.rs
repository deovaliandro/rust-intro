use core::f32;
use std::i32;

fn main() {
    println!("Hello, world!");
}

#[test]
fn unit_test() {
    println!("Hello unit test rust");
}

#[test]
fn variable_test() {
    let name: &str = "Deo";
    println!("{name}");
}

#[test]
fn mut_variable_test() {
    let mut name: &str = "Ini pertama";
    println!("{name}");

    name = "Diubah lagi lah";

    println!("{name}");
}

// shadowing tidak disarankan
#[test]
fn shadowing() {
    let r0: i32 = 1;
    println!("{r0}");

    let r0: &str = "Ok";
    println!("{r0}");
}

#[test]
fn number() {
    let i: u32 = u32::MAX;
    let j: i32 = i32::MAX;
    let k: f32 = f32::MAX;

    println!("MAX u32 = {i}");
    println!("MAX i32 = {j}");
    println!("MAX f32 = {k}");
}