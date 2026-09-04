struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn main() {
    println!("Problem 1 ---");
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("bathing"),
    };

    println!("Success!");

    println!("Problem 2 ---");
}
