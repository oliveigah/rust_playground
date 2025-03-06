pub fn start() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LISTOFF!!!");

    while_as_for();

    let a = [10, 20, 30, 40, 50];

    for number in a {
        println!("the value is: {number}");
    }

    for_as_while();

    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn while_as_for() {
    let mut number = 3;

    loop {
        if number == 0 {
            break;
        }
        println!("{number}!");
        number -= 1;
    }

    println!("LISTOFF!!!");
}

fn for_as_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}
