use std::io;
use std::time::Instant;

pub fn start() {
    println!("What is the timeout in ms?");

    let mut timeout = String::new();

    io::stdin()
        .read_line(&mut timeout)
        .expect("Failed to read from stdin");

    let timeout: u128 = timeout.trim().parse().expect("Invalid timeout");

    println!("Count until?");

    let mut max = String::new();

    io::stdin()
        .read_line(&mut max)
        .expect("Failed to read from stdin");

    let max: u128 = max.trim().parse().expect("Invalid max");

    let mut iterations = 0;

    let init = Instant::now();

    while init.elapsed().as_millis() < timeout {
        to_bench(max);
        iterations = iterations + 1;
    }

    println!("Total iterations is: {iterations}")
}

pub fn to_bench(max: u128) -> u128 {
    let mut sum = 0;
    for number in 1..max {
        sum += number;
    }
    return sum;
}
