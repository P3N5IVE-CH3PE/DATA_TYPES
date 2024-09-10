use std::io;

fn main() {
    let x = 2.0; //f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 100 - 50;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 20 / 5;

    // Remainder
    let remainder = 100 % 3;

    // Booleans
    let t = true;

    let f: bool = false; //With explicit type annotation

    // Char. Specified with single quotes, not double.
    let c = 'z';
    let z: char = 'Z';
    let heart_eyes_cat = '😻';

				let tup: (i32, f64, u8) = (500, 6.1, 2);
				
				let (x, y, z) = tup;
				
				println!("The value of y is: {y}");



				let tups: (i32, f64, u8) = (100,3.14, 42);
				let five_hundred = tups.0;
				let pie = tups.1;
				let meaning_of_life = tups.2;

				let a = [1,2,3,4,5];
	
				println!("Please enter an array index.");

				let mut index = String::new();
				
				io::stdin()
					.read_line(&mut index)
					.expect("Failed to read line");

				let index: usize = index
					.trim()
					.parse()
					.expect("Index entered was not a number");

				let element = a[index];

				println!("The value of the element at index {index} is: {element}");

				

}
