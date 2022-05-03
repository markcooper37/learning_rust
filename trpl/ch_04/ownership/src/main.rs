fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is invalid after this line for memory safety

    takes_ownership(s2); // s2 is no longer valid

    let x = 5;

    makes_copy(x); // x can be used afterwards

    let _s3 = gives_ownership();         
    let s4 = String::from("hello");
    let _s5 = takes_and_gives_back(s4);

    let s6 = String::from("hello");

    let len = calculate_length(&s6);

    println!("The length of '{}' is {}.", s6, len); // s6 is still valid

    let mut s7 = String::from("hello");

    println!("{}", s7);

    change(&mut s7); // We can change s7 using a mutable reference

    println!("{}", s7); // We could change s7 because it it mutable

    let r1 = &s7;
    let r2 = &s7;
    println!("{} and {}", r1, r2);
    // Variables r1 and r2 will not be used after this point

    let r3 = &mut s7; // There is no problem creating a mutable reference now due to NLL
    println!("{}", r3);
} // _s5 and _s3 go out of scope and are dropped, while s4 was moved so nothing happens

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // The backing memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Nothing special happens

fn gives_ownership() -> String {
    // Moves its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
