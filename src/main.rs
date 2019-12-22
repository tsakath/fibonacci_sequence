use std::io;

fn main() {
    println!("Fibonacci sequence");
    println!("Enter a number");

    let input: i32 = loop {
        let mut input = String::new();

        io::stdin().read_line(& mut input)
            .expect("failed to read input");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("OPS! You must enter a number!");
                continue
            },
        };

        break input;
    };

    println!("");

    println!("Iterative implementation");

    for n in 0..input {
        println!("{}", fibonacci(n));
    }

    println!("");

    println!("Recursive implementation");

    println!("");

    for n in 0..input {
        println!("{}", fibonacci_recursive(n));
    }
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0
    }

    let mut x = 0;
    let mut y = 1;

    for _ in 1..n {
        let s = x + y;
        x = y;
        y = s;
    }

    y
}

fn fibonacci_recursive(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}
