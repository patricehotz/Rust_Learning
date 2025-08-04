use std::io;

fn main() {
    println!("Get the nth Fibonacci number!");

    println!("type n:");

    let mut n = String::new();

    let n: u32 = loop {
        io::stdin().read_line(&mut n).expect("failed to read line");

        match n.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                n.clear();
                println!("Please type a positive, full, number");
            }
        }
    };

    let mut x: u32 = 0;
    let mut y: u32 = 1;

    for _ in 1..n {
        let next = x + y;
        x = y;
        y = next;
    }

    println!("The {n}th fibonacci is: {}", if n == 0 { x } else { y })
}
