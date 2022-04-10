// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `unimplemented!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` for hints :)

// I AM NOT DONE

use std::clone::Clone;

#[derive(PartialEq, Debug)]
pub enum List<T: Clone + Copy> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: Clone + Copy> List<T> {
    pub fn from_vec(input_vec: Vec<T>) -> List<T> {
        if input_vec.is_empty() {
            List::Nil
        } else {
            List::Cons(
                input_vec[0],
                Box::new(List::from_vec((&input_vec[1..]).to_vec())),
            )
        }
    }
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List<i32> {
    List::Nil
}

pub fn create_non_empty_list() -> List<i32> {
    List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }

    #[test]
    fn test_create_empty_list_from_empty_vec() {
        let input_vec: Vec<i32> = vec![];
        assert_eq!(List::Nil, List::from_vec(input_vec),)
    }

    #[test]
    fn test_create_list_from_vec() {
        let input_vec = vec![1, 2, 3];
        assert_eq!(
            List::Cons(
                1,
                Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
            ),
            List::from_vec(input_vec),
        )
    }
}
