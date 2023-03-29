pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

struct Counter2;

impl Iterator2<String> for Counter2 {
    fn next(&mut self) -> Option<String> {
        None
    }
}

pub trait Iterator3<T = String> {
    fn next(&mut self) -> Option<T>;
}

fn main() {}