fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}

fn borrow_object(_: &String) {}

fn push_str(s: &mut String) {
    s.push_str("world");
    println!("{}", s)
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn main() {
    println!("Problem 1 ---");
    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);

    println!("Problem 2 ---");
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);
    println!("{}", s2);

    println!("Problem 3 ---");
    let s = give_ownership();
    println!("{}", s);

    println!("Problem 4 ---");
    let s = String::from("Hello ");
    let mut s1 = s;
    s1.push_str("World!");
    println!("Success!");

    println!("Problem 5 ---");
    let x = Box::new(5);
    let mut y = Box::new(3); // update this line, don't change other lines!
    *y = 4;
    assert_eq!(*x, 5);
    println!("Success!");

    println!("Problem 6 ---");
    let x = 5;
    // Fill the blank
    let p = &x;
    println!("the memory address of x is {:p}", p);

    println!("Problem 7 ---");
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
    println!("Success!");

    println!("Problem 8 ---");
    let s = String::from("hello, ");
    borrow_object(&s);
    println!("Success!");

    println!("Problem 9 ---");
    let mut s = String::from("hello, ");
    push_str(&mut s);
    println!("Success!");

    println!("Problem 10 ---");
    let mut s = String::from("hello, ");
    // Fill the blank to make it work
    let p = &mut s;
    p.push_str("world");
    println!("Success!");

    println!("Problem 11 ---");
    let c = '中';
    let r1 = &c;
    // fill the blank，dont change other code
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    // check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));
    println!("Success!");

    println!("Problem 12 ---");
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
    println!("Success!");
}
