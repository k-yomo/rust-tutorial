fn main() {
    // ownership_basic();
    // borrowing();
    array_ownership();
}

fn ownership_basic() {
    // move ownership
    let mut s1 = String::from("hello");
    let s2 = s1; // ownership is moved to s2
    //     println!("{}", s1); this will cause compile error

     // deep copy
    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);

    let x = 5;
    let y = x;
    // since integer (which has known size) is copied deeply
    // int type(i64, u32, etc), bool, f64, char, tuple with Copy type will be copied deeply and ownership will not be moved
    println!("x = {}, y = {}", x, y);

    // ownership with functions
    let s = String::from("abc");
    take_ownership(s); // s is moved into take_ownership function

    let x = 10;
    make_copy(x); // the fn makes copy since the type of argument is i32

    //
    let s3 = give_ownership();
}

fn take_ownership(some_str: String) {
    println!("{}", some_str);
    // drop will be called and free memory
}

fn make_copy(some_int: i32) {
    println!("{}", some_int);
}

fn give_ownership() -> String {
    let s = String::from("abc");
    s
}

fn borrowing() {
    let s1 = String::from("hello~");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // mutation
    let s1 = String::from("hello~");
    // invalid_change_borrowed_val(s);
    let mut s2 = String::from("hello~");
    valid_change_borrowed_val(&mut s2);

    // lifeitme
    // dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn invalid_change_borrowed_val(s: &String) {
    // s.push_str("this will cause error since s is borrowed content");
}

fn valid_change_borrowed_val(s: &mut String) {
    s.push_str("this is valid");
}

// this dangle function cause compile error since it returns freeing reference
// fn dangle() -> &String {
//     let s = String::from("hello");
//     // &s
// }

fn array_ownership() {
    let mut s = String::from("hello world!");
    let word_ending_idx = first_word_ending_idx(&s);
    s.clear();
    // word_ending_idx is no longer valid

    let s = String::from("hello");
    println!("original string = {}", "hello");
    let slice = &s[0..2];
    println!("&s[0..2] = {}", slice);
    let slice = &s[..2];
    println!("&s[..2] = {}", slice);
    let slice = &s[3..];
    println!("&s[3..] = {}", slice);
    let slice = &s[3..s.len()];
    println!("&s[3..s.len()] = {}", slice);
    let slice = &s[..];

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // this will cause compile error since s is borrowed as immutable
    // s.clear();
    println!("the first word is: {}", word);
}

fn first_word_ending_idx(s: &String) -> usize {
    let bytes  = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// returning &str ensure s is immutable
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}