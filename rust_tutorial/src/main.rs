use std::vec;
use contourwall_lib::add;

fn double(i: u64) -> u64 { // i is not mutubale by default // put mut in front of the variable to make it mutable
    i * 2 // no semicolon here // return i * 2; // semicolon here
}

// overloading doesn't work in many languages including Rust

fn print_vec(v: &mut Vec<i32>) { // v: &Vec<i32> makes it so the v will be borrowed instead of used
    v.push(123); // v: &Vec<i32> // adds 123 to the end of the vector (array)
    for i in v {
        println!("Vector! {}", i);
    }
}

fn print_vec2(v: Vec<i32>) {
    println!("Vector! {:?}", v); // {:?} is a way of printing a vector (complex data type)
}

fn twee(){
    let mut v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    let v2 = v.clone(); // v2: Vec<i32>

    print_vec2(v2);

    v.push(123); // v: Vec<i32> // adds 123 to the end of the vector (array)
    print_vec2(v);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    family: Vec<String>
}

impl Person {
    pub fn new(name: String, age: u8) -> Person {
        Person {
            name,
            age,
            family: vec![]
        }
    }
    fn add_family_member(&mut self, name: String) {
        self.family.push(name.clone());
    }
}

#[derive(Debug)]
enum ErrorType {
    Unknown,
    WrongName,
    BadCredentials,
}

impl ErrorType {
    pub fn new(number: u8) -> ErrorType {
        match number {
            1 => ErrorType::Unknown,
            2 => ErrorType::WrongName,
            3 => ErrorType::BadCredentials,
            _ => ErrorType::Unknown,
        }
    }
}

#[derive(Debug)]
enum ErrorTypeVal {
    Unknown(bool),
    WrongName(String),
    BadCredentials(i32),
}

impl ErrorTypeVal {
    pub fn new(number: u8) -> ErrorType {
        match number {
            1 => ErrorType::Unknown,
            2 => ErrorType::WrongName,
            3 => ErrorType::BadCredentials,
            _ => ErrorType::Unknown,
        }
    }
}

fn f(b: bool) -> Result<String, ()> {
    if b{
        Ok(String::from("Success"))
    } else {
        Err(())
    }
}

fn main() {
    let mut x: u8 = 5; // x: u8

    x += 10; // can't be done when x doesn't have the mut keyword
    // when mutable is needed then the code is inefficient. Or not efficient enough.'
    // let x: u8 = 5; // x: u8
    // x += 10; // error: cannot assign twice to immutable variable `x`

    println!("Hello, world! {}", x);

    let x: i32 = 10; // x: u8 -> x: i32 // shadowing (editing a variable's type and value after already declared it.)

    println!("Hello, world! {}", x);

    for i in 0..10 {
        println!("For loop! {}", i);
    }

    println!("Double function! {}", double(50));

    let arr = [1, 2, 3, 4, 5]; // arr: [i32; 5]
    println!("Array 1! {}", arr[0]);

    let tup: (i32, f64, u8) = (500, 6.4, 1); // tup: (i32, f64, u8)
    println!("Tuple 1! {}", tup.0);
    println!("Tuple 2! {}", tup.1);
    println!("Tuple 3! {}", tup.2);

    let (x, y, z) = tup; // destructuring
    println!("Tuple 1! {}", x);
    println!("Tuple 2! {}", y);
    println!("Tuple 3! {}", z);

    let x: bool = if true { true } else { false }; // x: bool
    println!("Boolean! {}", x);

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    print_vec(&mut v);

    v.push(123); // v: Vec<i32> // adds 123 to the end of the vector (array)
    print_vec(&mut v);

    twee();

    let mut p = Person {
        // name: "John".to_string(),
        name: String::from("Bas"),
        age: 25,
        family: vec!["Tom".to_string(), "Jan".to_string()]
    };

    p.add_family_member(String::from("Dorusjan"));

    println!("Person! {:?}", p);

    let p2 = Person::new(String::from("John"), 25);
    println!("Person! {:?}", p2);

    let e = ErrorType::Unknown;
    println!("Error! {:?}", e);

    let e2 = ErrorType::new(2);
    println!("Error! {:?}", e2);

    let e_val = ErrorTypeVal::Unknown(true);
    println!("Error! {:?}", e_val);
    let e_val2 = ErrorTypeVal::WrongName(String::from("RandomName"));
    println!("Error! {:?}", e_val2);

    match e_val2 {
        ErrorTypeVal::Unknown(b) => {
            println!("Error! {:?}", b);
        },
        ErrorTypeVal::WrongName(s) => {
            println!("Error! {:?}", s);
        },
        ErrorTypeVal::BadCredentials(i) => {
            println!("Error! {:?}", i);
        },   
    }

    let res = f(true);
    match res {
        Ok(text) => {
            println!("Result! {:?}", text);
        },
        Err(()) => {
            println!("Result! {:?}", "Error");
        },
    }

    let libres = add(1, 2);
    println!("Result! {:?}", libres);
}