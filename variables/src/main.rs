fn main() {
    // mut
    let mut x = 5;
    x = 6;

    const MAX_POINTS: u32 = 100_000;

    // shadowing
    x = x + 1;
    x = x * 1;

    println!("The value of x is: {}", x);

    let mut spaces = "   ";
    let spaces = spaces.len();

    // int
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array
    let a = [1, 2, 3];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let first = a[0];
    let second = a[1];


    // fn
    another_function(x, y);
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", plus_one(x));

    // condition
    let num = 3;
    if num != 0 {
        println!("number was something other than 0");
    }

    let som_cond = true;
        let number = if som_cond {
            5
        } else {
            6
        };
    println!("The value of number is: {}", number);

    // loop
    let a = [10, 20, 30, 40, 50];
    for elm in a.iter() {
        println!("the value is: {}", elm);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn another_function(x: i32, y: f64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
