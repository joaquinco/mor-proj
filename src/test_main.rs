use std::fmt;

#[derive(Debug)]
struct CustomType(i32);

#[derive(Debug)]
struct DeepCustomType(CustomType);

fn format_tutorial() {
    let alice = "Alice";
    let bob = "Bob";

    let wave = format!("Hi {person1} this is {person2}", person1=alice, person2=bob);

    println!("{}", wave);

    // Print data with debug trait, automatically handled by adding #[derive(Debug)]
    let entero = CustomType(4);
    println!("El entero es {:?}", entero);

    let deep_entero = DeepCustomType(CustomType(10));
    println!("El deep entero es {:?}", deep_entero);
}

#[derive(Debug)]
struct Tuple(i32, i32);

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn reverse(pair: Tuple) -> Tuple {
    let Tuple(val1, val2) = pair;

    // Final statement return if without semicolons
    Tuple(val2, val1)
}

fn numbers_and_tuples_tutorial() {
    // Numbers
    let to_reduce = 2;
    println!("The result is {}", 1 - to_reduce);

    // Function call
    let pair = Tuple(1, 10 - to_reduce);
    // I can print with just {} because Tuple implements the fmt::Display trait
    println!("The reverse is {}", reverse(pair));
    println!("Eterogeneous sum {}", 3u32 - (2i32 as u32));
}

fn sum_slice(slice: &[i32]) -> usize {
    slice.len()
}

fn arrays_and_slices_tutorial() {
    let array_by_extension = [1, 2, 3, 4];
    let all_the_same: [i32; 10] = [1; 10];

    println!("This is an array {:?}", array_by_extension);
    println!("This is an monotonus array {:?}", all_the_same);
    println!("First elem is {}, length {}", array_by_extension[0], array_by_extension.len());
    println!("The sum of the slice is {}", sum_slice(&array_by_extension[1..4]));
}

use crate::types;

fn module_tutorial() {
    let sol = types::Solution { value: 30 };
    let sol2: Solution = Default::default();
    println!("Solutions are {} {}", sol, sol2);
}

pub fn test_main() {
    format_tutorial();
    numbers_and_tuples_tutorial();
    arrays_and_slices_tutorial();
    module_tutorial();

}
