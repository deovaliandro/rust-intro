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

#[test]
fn array_type() {
    let array: [i32; 3] = [1, 2, 3];
    println!("{:?}", array);

    let a = array[2];
    println!("{a}");

    // mutable array
    let mut array_mut: [i32; 3] = [1, 2, 3];
    array_mut[1] = 10;
    println!("{:?}", array_mut);
}

#[test]
fn constant_type() {
    const MOENIMUM: i32 = 12;
    println!("{MOENIMUM}");
}

#[test]
fn variable_scope() {
    let var = 1;

    {
        let name = "Anu";
        println!("{name}");
    }

    println!("{var}");
}

#[test]
fn string_type() {
    let mut name: String = String::from("Doe");
    println!("{name}");

    name.push_str(" Dua");
    println!("{name}");

    let new_name = name.replace("Dua", "Dor");
    println!("{new_name}");
}

#[test]
fn ownership_rules() {
    // a belum bisa diakses
    let a = 12;
    // a bisa diakses

    {
        let b = 10;
        // b bisa diakses dari sini
        println!("{b}");
    } // b menghilang disini

    println!("{a}");
} // a menghilang dari sini

#[test]
fn data_copy() {
    let a = 10;
    let b = a; // a dicopy ke b, a masih ada

    println!("{b}");
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("Heyaa");
    let name2: String = name1; // kepemilikan name1 dipindah ke name2

    println!("{name2}");
}

#[test]
fn heap_copy() {
    let name1: String = String::from("Daaa");
    let name2: String = name1.clone();

    println!("{name1 } {name2}");
}

#[test]
fn if_expr() {
    let v = 12;
    let n: &str = if v <= 10 {
        "Good"
    } else if v == 11 {
        "Not Bad"
    } else {
        "Bad"
    };

    println!("{n}");
}

#[test]
fn loop_expr() {
    let mut c = 0;
    loop {
        c+=1;

        if c > 10 {
            break;
        } else if c % 2 == 0 {
            continue;
        }

        println!("{c}");
    }
}