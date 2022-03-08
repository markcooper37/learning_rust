// Investigating various loop types
// from chapter 3.5 of 'The Rust Programming Language'

fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 7 {
                break;
            }
            if count == 4 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}\n", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}\n", result);

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("Go!\n");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    for _element in a {
        println!("Index {} of a has value: {}", index, a[index]);

        index += 1;
    }
}
