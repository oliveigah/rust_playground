pub fn start() {
    let mut x = 5;

    x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("Truncated is {}", -5 / 3);

    let c = 'a';

    println!("This is a char {c}");

    let mut tuple = (4, 2, 3);

    tuple.0 = 1;

    match tuple {
        (1, 2, 3) => println!("I can match on tuples! Niiiice! (:"),
        (_x, _y, _z) => println!("Not here"),
    }

    let mut a = [1, 2, 3, 4, 6];
    a[4] = 5;

    match a {
        [1, 2, 3, 4, 6] => println!("Not here"),
        [1, 2, 3, 4, 5] => println!("I can match on arrays! Niiiice! (:"),
        [_, _, _, _, _] => println!("Not here"),
    }
}
