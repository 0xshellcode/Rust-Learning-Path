use std::io;

fn main() {
    let cond = (2 as f32) <= 2.2;
    let cond2 = false && cond;

    println!("{}", cond);
    println!("{}", cond2);

    let mut food = String::new();
    io::stdin()
        .read_line(&mut food)
        .expect("Failed to read line");

    if food.trim() == "cookie" {
        println!("I like cookies, yum!");
    } else if food == "apple" {
        println!("I like apples, yum!");
    } else {
        println!("I don't like {}", food);
    }
}
