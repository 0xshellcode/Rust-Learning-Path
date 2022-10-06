use std::io;

fn main() {
    let x: f32 = 12.0; //  0 - 255
    let y: f32 = 10.11; // -128 - 127

    let z = x + y;
    println!("z = {}", z);

    let a: f32 = 255.0;
    let b: f32 = 10.0;

    let c: f32 = a / b;
    println!("c = {}", c);

    let d: f32 = 255.0;
    let e: f32 = 10.0;

    let f: f32 = d % e;
    println!("f = {}", f);

    // Type Casting

    let g = 255.0f32;
    let h = 10.0_f32;

    let i = g + h;
    println!("i = {}", i);

    let j = 255.0 as f64;
    let k = 10.0_f32;

    let l = j / (k as f64);
    println!("l = {}", l);

    // String to Integer

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Expected to read line");
    println!("input = {}", input);

    let int_input: i64 = input.trim().parse().unwrap();
    println!("int_input = {}", int_input + 2);
}
