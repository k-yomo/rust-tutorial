fn main() {
    println!("{}", fibonacci(22));
}

fn fibonacci(i: usize) -> u32 {
    let mut fib: Vec<u32> = Vec::new();
    fib.push(1);
    fib.push(1);

    if i == 1 {
        return 1
    }
    let mut idx: usize = 2;
    while idx != i {
        fib.push(fib[idx-1]+fib[idx-2]);
        idx += 1;
    }
    return fib[fib.len() - 1];
}
