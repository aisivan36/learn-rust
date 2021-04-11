pub fn third() {
    // main_debug();
    // main_mood();
    // main_destruct();
    // main_city();
    main();
}

fn main() {
    let my_number = 9;
    let reference = &my_number;

    println!("{}", my_number == *reference); // ⚠️
}

// struct Person {
//     name: String,
//     real_name: String,
//     height: u8,
//     happiness: bool,
// }

// fn main_destruct() {
//     let papa_doc = Person {
//         name: "Papa Doc".to_string(),
//         real_name: "Clarence".to_string(),
//         height: 170,
//         happiness: false,
//     };
//     let Person {
//         // destructure papa_doc
//         name: a,
//         real_name: b,
//         height: c,
//         happiness: d,
//     } = papa_doc;

//     println!(
//         "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
//         a, b, c, d
//     );
// }

// struct City {
//     name: String,
//     name_beofre: String,
//     population: u32,
//     date_founded: u32,
// }
// impl City {
//     fn new(name: String, name_beofre: String, population: u32, date_founded: u32) -> Self {
//         Self {
//             name,
//             name_beofre,
//             population,
//             date_founded,
//         }
//     }
// }

// fn process_city_values(city: &City) {
//     let City {
//         name,
//         name_beofre,
//         population,
//         date_founded,
//     } = city;

//     let two_names = vec![name, name_beofre];
//     println!("The city's two names are {:#?}", two_names);
// }

// fn main_city() {
//     let london = City::new("London".to_string(), "IDK".to_string(), 89820000, 1500);

//     process_city_values(&london)
// }

// #[derive(Debug)]
// struct Animal {
//     age: u8,
//     animal_type: AnimalType,
// }

// #[derive(Debug)]
// enum AnimalType {
//     Cat,
//     Dog,
// }

// impl Animal {
//     fn new() -> Self {
//         // Self means Animal.
//         //You can also write Animal instead of Self

//         Self {
//             age: 10,
//             animal_type: AnimalType::Cat,
//         }
//     }

//     fn change_to_dog(&mut self) {
//         // because we are inside Animal, &mut self means &mut Animal
//         // use .change_to_dog() to change the cat to a dog
//         // with &mut self we can change it

//         println!("Changing animal to dog");
//         self.animal_type = AnimalType::Dog;
//     }

//     fn change_to_cat(&mut self) {
//         // use .change_to_cat() to change the dog to a cat
//         // with &mut self we can change it
//         println!("Changing animal to cat");
//         self.animal_type = AnimalType::Cat;
//     }

//     fn check_type(&self) {
//         match self.animal_type {
//             AnimalType::Cat => println!("The animal is a cat"),
//             AnimalType::Dog => println!("The animal is a dog"),
//         }
//     }
// }

// fn main_debug() {
//     let mut new_animal = Animal::new();

//     new_animal.check_type();
//     new_animal.change_to_dog();
//     new_animal.check_type();
//     new_animal.change_to_cat();
//     new_animal.check_type();
// }

// enum Mood {
//     Good,
//     Bad,
//     Sleepy,
// }

// impl Mood {
//     fn check(&self) {
//         match self {
//             Mood::Good => println!("Feeling good!"),
//             Mood::Bad => println!("Eh, not feeling so good"),
//             Mood::Sleepy => println!("Need sleep NOW"),
//         }
//     }
// }

// fn main_mood() {
//     // let my_mood = Mood::Good;
//     Mood::Good.check();
//     // my_mood.check();
// }
