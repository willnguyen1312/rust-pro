// use colored::*;
// use rand::Rng;
// use std::cmp::Ordering;
// use std::collections::HashMap;
// use std::io;

// fn main() {
//     println!("Guess the number! ");

//     let secret_number = rand::thread_rng().gen_range(1, 101);

//     println!("The secret number is: {}", secret_number);

//     loop {
//         println!("Please input your guess. ");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("{}", "Too small! ".red()),
//             Ordering::Greater => println!("{}", "Too big! ".red()),
//             Ordering::Equal => {
//                 println!("{}", "You win! ".green());
//                 break;
//             }
//         }
//     }

//     let mut v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);

//     v.push(6);

//     match v.get(4) {
//         Some(third) => println!("The fifth is {}", third),
//         None => println!("There is no fifth element. "),
//     }

//     let s = String::from("hello world");

//     for c in s.bytes() {
//         println!("{}", c);
//     }

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
// }

// Panic

fn main() {
    fn a() -> i32 {
        return 1;
    }

    fn b() -> ! {
        panic!("Error");
    }

    let x = a();

    if x < 5 {
        b();
    } else {
        println!("Hello");
    }
}
