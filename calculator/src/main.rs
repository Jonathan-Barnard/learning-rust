use std::io;
use std::io::Write;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn div(x: i32, y: i32) -> i32 {
    x / y
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}

fn define_variable(var: &str) -> i32 {
    print!("{var} = ");
    io::stdout().flush().expect("Failed to flush output.");

    let mut v = String::new();
    io::stdin().read_line(&mut v).expect("Failed to fetch x");
    let v: i32 = v.trim().parse().expect("Converting x to number failed!");
    return v;
}

fn define_operator() -> String {
    print!("Operation = ");
    io::stdout().flush().expect("Failed to flush output.");

    let mut o = String::new();
    io::stdin().read_line(&mut o).expect("Failed to fetch x");

    return o.trim().to_string();
}

fn main() {
    println!("Select two numbers to add: ");

    let x = define_variable("x");
    let y = define_variable("y");
    let op = define_operator();

    let z = add(x, y);
    println!("Answer = {z}");
}
