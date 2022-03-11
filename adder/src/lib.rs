// Creating some basic tests
// using chapter 11 of 'The Rust Programming Language'

#[cfg(test)]

mod tests {
    // We need to bring the rectangles code into scope
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // larger can hold smaller
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 11,
            height: 3,
        };
        let smaller = Rectangle {
            width: 4,
            height: 2,
        };
        // smaller cannot hold larger
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn exploration() {
        assert_eq!(add_two(2), 4);
        // Test passes using the assert_eq! macro
    }

    #[test]
    #[should_panic(expected = "Oh no! The number is greater than zero!")]
    fn positive_in_want_zero() {
        want_zero(100);
        // Test passes using the should_panic attribute with expected parameter
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn want_zero(number: i32) -> i32 {
    // We don't see the output of the following line if the test passes
    println!("Checking if the number is zero...");
    if number > 0 {
        panic!("Oh no! The number is greater than zero!")
    } else if number < 0 {
        panic!("Oh no! The number is less than zero!")
    }
    number
}
