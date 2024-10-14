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
    let b = a;
    // a dicopy ke b, a masih ada

    println!("{b}");
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("Heyaa");
    let name2: String = name1;
    // kepemilikan name1 dipindah ke name2

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
        c += 1;

        if c > 10 {
            break;
        } else if c % 2 == 0 {
            continue;
        }

        println!("{c}");
    }
}

#[test]
fn loop_expr_return() {
    let mut c: i32 = 0;
    let result = loop {
        c += 1;

        if c > 10 {
            break c;
        } else if c % 2 == 0 {
            break c;
        }
    };

    println!("{result}");
}

#[test]
fn loop_label() {
    let mut c: i32 = 0;
    'kesini: loop {
        c += 1;

        if c > 10 {
            break 'kesini;
        }
    }

    println!("{c}");
}

#[test]
fn while_loop() {
    let mut c = 0;
    while c <= 10 {
        c += 1;
        println!("{c}");
    }
}

#[test]
fn array_iter_for_loop() {
    let array: [&str; 2] = ["Kal", "Leo"];

    for i in array {
        println!("{i}");
    }

    for i in 0..array.len() {
        println!("{i}");
    }
}

fn say_hello() {
    println!("Meoww!!");
}

fn say_goodbye(name: &str) {
    println!("Goodbye {name}");
}

#[test]
fn test_say_hello() {
    say_hello();
    say_goodbye("Doe");
}

fn factorial(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result: i32 = 1;

    for i in 1..=n {
        result *= i;
    }

    result
}

fn factorial_recursive(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial() {
    let result: i32 = factorial(5);
    println!("Faktorial biasa = {result}");

    let result: i32 = factorial_recursive(5);
    println!("Faktorial rekursif = {result}");
}

fn print_n(n: i32) {
    println!("Number = {n}");
}

fn hi(name: String) {
    println!("Hi, {name}");
}

#[test]
fn test_hi() {
    let n = 8;
    print_n(n);
    println!("{n}");
    // data masih ada kerena di stack
    // sehingga data yang dikirim hanya clone

    let name: String = String::from("Lena");
    hi(name);
    // println!("{name}");
    // error karena datanya di heap
    // sehingga data yang dikirim berupa referensi
}

fn full_name(fname: String, lname: String) -> (String, String, String) {
    let full_name: String = format!("{}, {}", fname, lname);
    (fname, lname, full_name)
}

#[test]
fn test_full_name_return_ownership() {
    let fname: String = String::from("John");
    let lname: String = String::from("Doe");

    let (a, b, name) = full_name(fname, lname);
    println!("{name}");
    println!("{a}");
    println!("{b}");
}

fn reference_full_name(first_name: &String, last_name: &String) -> String {
    return format!("{first_name} {last_name}");
}

#[test]
fn test_reference_full_name() {
    let fname: String = String::from("Kia");
    let lname: String = String::from("Leona");

    let full_name: String =
        reference_full_name(&fname, &lname);
    println!("{full_name}");
}

fn change_valu(value: &mut String) {
    value.push_str("Ini ditambahkan");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Ini pertama");
    change_valu(&mut value);
    println!("{value}");
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1: &[i32] = &array[0..=1];
    println!("{:?}", slice1);
}

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn print_person_struct(person: &Person) {
    println!("Name = {} {}, age = {}",
        person.first_name, person.last_name, person.age);
}

#[test]
fn struct_person() {
    let person: Person = Person {
        first_name: String::from("Modia"),
        last_name: String::from("Kahe"),
        age: 12,
    };

    let person2: Person = Person {
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };

    print_person_struct(&person);
    print_person_struct(&person2);
}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point: GeoPoint = GeoPoint(-6.99888, 12.0000);
    println!("{:.2} - {:.2}", geo_point.0, geo_point.1);
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

#[test]
fn test_method() {
    let person: Person = Person {
        first_name: String::from("Leon"),
        last_name: String::from("Xo"),
        age: 21,
    };

    person.say_hello("Yu");
}

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point: GeoPoint = GeoPoint::new(99.010, 108.221);
    println!("long: {}", geo_point.0);
    println!("lat: {}", geo_point.1);
}