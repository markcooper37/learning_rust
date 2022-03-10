// Investigating generic data types
// using chapter 10 of 'The Rust Programming Language'

use std::fmt::Display;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let string1 = String::from("Hello, World!");
    let string2 = "Goodbye, Mars!";

    let result = longest_with_an_announcement(string1.as_str(), string2, "finding longest string...");
    println!("The longest string is {}", result);
}

// This function allows us to find the largest value in a vector for various types
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // Use references to avoid needing the Copy trait
    for index in 0..(*list).len() {
        if list[index] > *largest {
            largest = &list[index];
        }
    }

    largest
}

// This is a generic struct definition where x and y may have different types
struct Point<T, U> {
    x: T,
    y: U,
}

// This is a generic method for the Point struct
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This function implements generic type parameters, trait bounds and lifetimes
// to find the longest of two strings
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}