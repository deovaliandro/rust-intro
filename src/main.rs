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