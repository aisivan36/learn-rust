// fn main() {
//     let penguin_data = "\
//     common name,length (cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";

//     let records = penguin_data.lines();

//     for (i, record) in records.enumerate() {
//         if i == 0 || record.trim().len() == 0 {
//             continue;
//         }

//         let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

//         if cfg!(debug_assertions) {
//             eprintln!("debug: {:?} -> {:?}", record, fields);
//         }

//         let name = fields[0];

//         let maybe_length: Result<f32, _> = fields[1].parse();

//         if maybe_length.is_err() {
//             continue;
//         }

//         let length = maybe_length.unwrap();

//         println!("{}, {}c,", name, length);
//     }
// }

// #[derive(Debug)]
// enum Cereal {
//     Barley,
//     Millet,
//     Rice,
//     Rye,
//     Spelt,
//     Wheat,
// }

// fn main() {
//     let mut grains: Vec<Cereal> = vec![];
//     grains.push(Cereal::Rye);
//     drop(grains);

//     println!("{:?}", grains);
// }

// fn multiply(number_one: i32, number_two: i32) {
//     // Two i32s will enter the function. We will call them number_one and number_two.
//     let result = number_one * number_two;
//     println!("{} times {} is {}", number_one, number_two, result);
// }

// fn main() {
//     multiply(90, 9); // We can give the numbers directly
//     let some_number = 10; // Or we can declare two variables
//     let some_other_number = 2;
//     multiply(some_number, some_other_number); // and put them in the function

//     let my_number = {
//         let second_number = 9;
//         second_number + 9
//     };

//     println!("My number is {:?}", my_number);

//     // To print the type how many it can handle
//     println!(
//         "The smallest i8 is {} and the biggest i8 is {}.",
//         std::i8::MIN,
//         std::i8::MAX
//     ); // hint: printing std::i8::MIN means "print MIN inside of the i8 section in the standard library"
//     println!(
//         "The smallest u8 is {} and the biggest u8 is {}.",
//         std::u8::MIN,
//         std::u8::MAX
//     );
//     println!(
//         "The smallest i16 is {} and the biggest i16 is {}.",
//         std::i16::MIN,
//         std::i16::MAX
//     );
//     println!(
//         "The smallest u16 is {} and the biggest u16 is {}.",
//         std::u16::MIN,
//         std::u16::MAX
//     );
//     println!(
//         "The smallest i32 is {} and the biggest i32 is {}.",
//         std::i32::MIN,
//         std::i32::MAX
//     );
//     println!(
//         "The smallest u32 is {} and the biggest u32 is {}.",
//         std::u32::MIN,
//         std::u32::MAX
//     );
//     println!(
//         "The smallest i64 is {} and the biggest i64 is {}.",
//         std::i64::MIN,
//         std::i64::MAX
//     );
//     println!(
//         "The smallest u64 is {} and the biggest u64 is {}.",
//         std::u64::MIN,
//         std::u64::MAX
//     );
//     println!(
//         "The smallest i128 is {} and the biggest i128 is {}.",
//         std::i128::MIN,
//         std::i128::MAX
//     );
//     println!(
//         "The smallest u128 is {} and the biggest u128 is {}.",
//         std::u128::MIN,
//         std::u128::MAX
//     );
// }

// fn country_str() {
//     let mut my_number = 8;
//     let num_ref = &mut my_number;
//     *num_ref += 10;
//     println!("{}", my_number);

//     let second_number = 900;
//     let triple_number = &&&second_number;
//     println!(
//         "Second Number = triple reference? {}",
//         second_number == ***triple_number
//     )
// }

// fn main() {
//     // println!(r#"If you like to know just""" research about it"#);
//     // println!("{:?}", b"If you like to know just research about it");

//     country_str();

//     let my_name = "ivan";
//     let my_country = "Indonesia";
//     let my_dream = "Erope";

//     let together = format!(
//         "I am {} and I live in {} and my dream is going to {}",
//         my_name, my_country, my_dream
//     );
//     println!("{}", together);
//     println!("The letter is {} and the value is {:#?}", NUMBER, VALUE);
// }

// const NUMBER: u32 = 4;
// static VALUE: [&str; 4] = ["Ivan", "Boginski", "Ais", "Ivan"];

// fn print_country(country_name: String) {
//     println!("{}", country_name)
// }

// fn main() {
//     let country = String::from("Ireland");
//     print_country(country);
//     // print_country(country);
// }
// fn print_country(country_name: String) -> String {
//     println!("{}", country_name);
//     country_name // return it here
// }

// mod second;
mod third;
mod truct;

fn main() {
    println!("End of second package...");
    // let country = String::from("Austria");
    // let country = print_country(country); // we have to use let here now to get the String back
    // print_country(country);
    let mut country = String::from("Austria");
    add_hungary(&mut country);

    truct::main_truct();
}

fn add_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("now it sats: {}", country_name);
    // newver();
    // second::second_main();
    third::third();
}
// fn newver() {
//     println!("wedonotoknwo");
//     inheritance();
// }

// // Inheritance
// trait Speaks {
//     fn speak(&self);
//     fn noise(&self) -> &str;
// }

// trait Animal {
//     fn animal_type(&self) -> &str;
// }

// struct Dog {}

// impl Animal for Dog {
//     fn animal_type(&self) -> &str {
//         "dog"
//     }
// }

// impl Speaks for Dog {
//     fn speak(&self) {
//         println!("The dog said {}", self.noise());
//     }

//     fn noise(&self) -> &str {
//         "wolf"
//     }
// }

// fn inheritance() {
//     let dog = Dog {};
//     dog.speak();
//     main_hungary();
// }

// fn hungary(country_name: &mut String) {
//     country_name.push_str("-Hungary");
//     println!("Now it says: {}", country_name);
// }

// fn main_hungary() {
//     let mut country = String::from("Germany");
//     hungary(&mut country);
//     // main_loop();
//     // array();
// }

// fn main_loop() {
//     let my_number;
//     {
//         let number = { 57 };
//         my_number = loop_then_return(number);
//     }
//     println!("{}", my_number);
//     loop_continue();
// }

// fn loop_then_return(mut counter: i32) -> i32 {
//     loop {
//         counter += 3;
//         if counter % 50 == 0 {
//             break;
//         }
//     }
//     return counter;
// }

// fn loop_continue() {
//     'tens: for ten in 0..2 {
//         '_units: for unit in 0..=3 {
//             if unit % 2 == 0 {
//                 continue;
//             }
//             if unit > 5 {
//                 continue 'tens;
//             }
//             println!("{}", ten * 10 + unit);
//         }
//     }
// }

// fn array() {
// let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
// // Use ..= if I want to count 1 2 3 which is start at 0 1 and 2
// // If I use .. not ..= it starts at 0 1 and ends at 1, which is 1 2
// let everything = &array_of_ten[0..=2];

// println!("{:#?}", everything);

//     vector()
// }

// fn vector() {
// let name1 = String::from("Ivan");
// let name2 = String::from("Boginski");

// let my_vec: Vec<String> = vec![name1, name2];

// // my_vec.push(name1);
// // my_vec.push(name2);
// println!("{:#?}", my_vec,);
// tuple_indexing();

// array_vec();

// let my_vec: Vec<u8> = [1, 2, 3].into();
// let my_vec2: Vec<_> = [9, 0, 10].into(); // Vec<_> means "choose the Vec type for me"
// println!("{:#?} {:#?}", my_vec, my_vec2)
// }

// fn array_vec() {
//     let mut num_vec = Vec::new();
//     println!("{}", num_vec.capacity()); // 0 elements: prints 0
//     num_vec.push('a'); // add one character
//     println!("{}", num_vec.capacity()); // 1 element: prints 4. Vecs with 1 item always start with capacity 4
//     num_vec.push('a'); // add one more
//     num_vec.push('a'); // add one more
//     num_vec.push('a'); // add one more
//     println!("{}", num_vec.capacity()); // 4 elements: still prints 4.
//     num_vec.push('a'); // add one more
//     println!("{}", num_vec.capacity()); // prints 8. We have 5 elements, but it doubled 4 to 8 to make space
// }

// fn tuple_indexing() {
//     // let str_vec = vec!["one", "two", "three"];

//     // let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]); // call them a, b, and c
//     // println!("{:?}", b);
//     let str_vec = vec!["one", "two", "three"];

//     //If you need to destructure but don't want all the variables, you can use _
//     // Now it only creates a variable called variable but doesn't make a variable for the others.

//     let (_, _, variable) = (str_vec[0], str_vec[1], str_vec[2]);
//     println!("{:?}", variable);
//     control_flow();
// }

// fn control_flow() {
//     let children = 5;
//     let married = true;

//     match (children, married) {
//         (children, married) if married == false => {
//             println!("not married with {} children {}", married, children)
//         }
//         (children, married) if children == 0 && married == true => {
//             println!(" Married{} with {}", married, children)
//         }
//         _ => println!("Married? {}. Number of children {}", married, children),
//     };

//     let my_number = 5;
//     let second_number = match my_number {
//         0 => 0,
//         5 => 10,
//         _ => 2,
//     };

//     println!("{}", second_number);

//     colour_match();
// }

// fn match_colours(rbg: (i32, i32, i32)) {
//     match rbg {
//         (r, _, _) if r < 10 => println!("Not much red"),
//         (_, b, _) if b < 10 => println!("Not much blue"),
//         (_, _, g) if g < 10 => println!("Not much green"),
//         _ => println!("Each colour has at least 10"),
//     }
// }

// fn colour_match() {
//     let first = (200, 0, 0);
//     let second = (50, 50, 50);
//     let third = (200, 50, 0);

//     match_colours(first);
//     match_colours(second);
//     match_colours(third);

//     // for_struct();
// }

// std::fmt::Debug
// use std::fmt;

// struct Country {
//     population: u32,
//     capital: String,
//     leader_name: String,
// }

// fn for_struct() {
//     let population = 500_000;
//     let capital = String::from("London");
//     let leader_name = String::from("Ivan");

//     impl fmt::Debug for Country {
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             write!(f, "Hi")
//         }
//     }

//     let kalmykia = Country {
//         population: population,
//         capital: capital,
//         leader_name: leader_name,
//     };

//     println!("{:#?}", kalmykia)
// }

// use std::fmt;

// struct Colour(u8, u8, u8);

// struct SizeAndColour {
//     size: u32,
//     colour: Colour, // And we put it in our new named struct
// }

// // struct Point {
// //     x: i32,
// //     y: i32,
// // }

// fn for_struct() {
//     let my_colour = Colour(50, 0, 50);
//     let size_and_colour = SizeAndColour {
//         size: u32,
//         colour: Colour,
//     };

//     impl fmt::Display for SizeAndColour {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(f, "({})", self.2)
//         }
//     }

//     // let size_and_colour = SizeAndColour
//     println!("{}", size_and_colour)
//     // assert_eq!(
//     //     format!("The origin is: {}", my_colour),
//     //     "The origin is: (0, 0)"
//     // );
// }
// #[derive(Debug)]
// struct Colour(u8, u8, u8); // Declare the same Colour tuple struct

// struct SizeAndColour {
//     size: u32,
//     colour: Colour, // And we put it in our new named struct
// }

// fn for_struct() {
//     let my_colour = Colour(50, 0, 50);

//     let size_and_colour = SizeAndColour {
//         size: 150,
//         colour: my_colour,
//     };
//     println!("{:#?}", size_and_colour)
// }
