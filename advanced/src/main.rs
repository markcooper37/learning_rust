use std::ops::Add;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // The below takes advantage of operator overloading
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // We need to use explicit syntax to specify which fly method we mean
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // We need to use fully qualified syntax below to get the desired result
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // The Wrapper allows us to bypass the orphan rule
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // We give i32 a type alias below
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Here, we pass one function as an argument to another
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// The traits Pilot and Wizard both have method fly and are then
// implemented on a type Human with method fly implemented on it
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Animal and Dog have an associated function with the same name
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// We want to use the Display trait's functionality in the implementation of outline_print
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// We use the Wrapper struct to get around the orphan rule
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// Using a type alias here makes our code easier to write 
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// This function has the never type as its return type
fn bar() -> ! {
    panic!();
}

// Rust implicitly adds a bound on Sized to every generic function
// The below function is treated as though it were written fn generic<T: Sized>(t: T) {}
fn generic<T>(t: T) {}

// fn generic<T: ?Sized>(t: &T) {} relaxes the restriction of the type having 
// known size at compile time

// We can pass functions as arguments to other functions
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// We can use a trait object when we want to return a closure
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}