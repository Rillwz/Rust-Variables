fn main() {
    let mut counter = 0;
    loop {
        counter += 1;

        let sum = my_func(11, 12);
        println!("The sum is : {}", sum);

        if counter == 10 {
            break;
        }
    }

    let mut number = 3;

    while number != 0 {
        println!("Hello {}", number);

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }
}

fn my_func(x: i32, y: i32) -> i32 {
    return x + y;
}
