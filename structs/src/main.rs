use std::fmt;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "username: {}, email: {}, sign_in_count: {}, active: {}", self.username, self.email, self.sign_in_count, self.active)
    }
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("test1"),
        email: String::from("test@example.com"),
        sign_in_count: 10,
        active: true
    };

    user1.email = String::from("test2@exampe.com");

    let user2 = build_user(String::from("test2"), String::from("test2@example.com"));
    println!("{}", user2);

    let user3 = User {
        username: String::from("test3"),
        email: String::from("test3@example.com"),
        ..user1
    };

    let black = Color(0,0,0);
    let originalPoint = Color(0,0,0);


    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_refactor1(rect1));

    let rect2 = Rectangle {width: 30, height: 50};
    println!("rect2 is {:?}", rect2);
    println!("rect2 with pretty format is {:#?}", rect2);
    println!("The area of the rectangle is {} square pixels.", rect2.area());

    let rect3 = Rectangle { width: 29, height: 49 };
    println!("rect2 can hold rect3?: {}", rect2.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        email, 
        username,
        active: true,
        sign_in_count: 10
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_refactor1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_refactor2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}