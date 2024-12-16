// Generics Examples

use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn compare_and_display<T, U>(statement: T, num_1: U, num2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
}

fn print_item<T: Debug + Display>(item: T) {
    println!("Here is yout item {:?}", item);
}

fn main() {
    let charlie = Animal {
        name: String::from("Charlie"),
        age: 1,
    };

    let number = 55;

    print_item(charlie);
    print_item(number);
}
