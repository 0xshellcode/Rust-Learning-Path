fn main() {
    println!("Hello, world!");
    test();
    let result: i32 = add_numbers(20, 22);
    println!("Result: {}", result);

    let number = {
        let x = 3;
        x + 1
    };

    println!("The value of number is: {}", number);
}

fn test() {
    println!("Test has been called!");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    let z: i32 = x + y;
    z
    //return z;
}
