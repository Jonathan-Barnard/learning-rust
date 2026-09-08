fn main() {
    println!("Problem 1 ---");
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Modify the code below to make it work
    assert!(arr.len() == 5);
    println!("Success!");

    println!("Problem 2 ---");
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let _ = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == std::mem::size_of::<char>() * arr.len());
    println!("Char size -> {} bytes", std::mem::size_of::<char>());
    println!("Success!");

    println!("Problem 3 ---");
    // Fill the blank
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list[99] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
