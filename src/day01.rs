use std::fmt::Display;
use crate::util::read_input_ints;

pub fn run() {
    // Part 1
    read_input_ints("src/Day01_test.txt")
        .total_depth_increases()
        .assert(|it| it == &7usize);

    read_input_ints("src/Day01.txt")
        .total_depth_increases()
        .print();

    // Part 2
    read_input_ints("src/Day01_test.txt")
        .sliding_window_totals()
        .total_depth_increases()
        .assert(|it| it == &5usize);

    read_input_ints("src/Day01.txt")
        .sliding_window_totals()
        .total_depth_increases()
        .print();
}

pub trait IntVectorExtTrait {
    fn total_depth_increases(&self) -> usize;
    fn sliding_window_totals(&self) -> Vec<usize>;
}

impl IntVectorExtTrait for Vec<usize> {
    fn total_depth_increases(&self) -> usize {
        self.iter().zip(self.iter().skip(1))
            .fold(0, |acc, (a, b)| acc + if a < b { 1 } else { 0 })
    }

    fn sliding_window_totals(&self) -> Vec<usize> {
        self.iter()
            .zip(self.iter().skip(1))
            .zip(self.iter().skip(2))
            .map(|((x, y), z)| x + y + z)
            .collect()
    }
}

pub trait IntExtTrait<T> {
    fn assert<F: FnOnce(&T) -> bool>(&self, f: F) -> &T;
    fn print(&self) -> &T;
}

impl<T> IntExtTrait<T> for T where T: Display {
    fn assert<F: FnOnce(&T) -> bool>(&self, f: F) -> &T {
        match f(self) {
            true => self,
            false => panic!("Assert Failed")
        }
    }

    fn print(&self) -> &T {
        println!("{}", self);
        self
    }
}