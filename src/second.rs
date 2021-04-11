enum ThingsInTheSky {
    Sun,
    Stars,
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Stars => println!("I can see the stars"),
    }
}

pub fn second_main() {
    let waktu = 6;
    // let skystate = create_skystate(waktu);
    check_skystate(&create_skystate(waktu));
    // another_enum();
    main_season();
}

enum Season {
    Spring, // If this was Spring(String) or something it wouldn't work
    Summer,
    Autumn,
    Winter,
}

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

fn main_season() {
    use Season::*;
    let four_season = vec![Spring, Summer, Autumn, Winter];
    for season in four_season {
        println!("{}", season as u32)
    }

    use Star::*;
    let startvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant];
    for star in startvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."),
            size if size >= 80 => println!("This is a good-sized star."),
            _ => println!("That star is pretty big !"),
        }
    }
    println!("What about DeadStar? It's the number {}", DeadStar as u32);

    // main_number();
    // for_loop();
    while_loop();
}

fn while_loop() {
    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("The counter is now: {:#?}", counter);
    }

    for _number in 0..3 {
        println!("It prints the same thing 3 times")
    }

    let mut counter = 7;
    let my_number = loop {
        counter += 1;
        if counter % 19 == 1 {
            break counter;
        }
    };
    println!("{}", my_number);
    main_colours();
}
fn main_colours() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}

fn match_colours(rbg: (i32, i32, i32)) {
    println!(
        "Comparing a colour with {} red, {} blue, and {} green:",
        rbg.0, rbg.1, rbg.2
    );
    let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")]; // Put the colours in a vec. Inside are tuples with the colour names
    let mut all_have_at_least_10 = true; // Start with true. We will set it to false if one colour is less than 10
    for item in new_vec {
        if item.0 < 10 {
            all_have_at_least_10 = false; // Now it's false
            println!("Not much {}.", item.1) // And we print the colour name.
        }
    }
    if all_have_at_least_10 {
        // Check if it's still true, and print if true
        println!("Each colour has at least 10.")
    }
    println!(); // Add one more line
}

// for loop infinite mode
// fn for_loop() {
//     // loop {
//     //     // Loops without breaking
//     // }

//     // let mut counter = 0;
//     // loop {
//     //     counter += 1;
//     //     println!("The counter is now: {}", counter);
//     //     if counter == 3 {
//     //         break;
//     //     }
//     // }

//     let mut counter = 0;
//     let mut counter2 = 0;
//     println!("Now entering the First loop.");

//     'first_loop: loop {
//         counter += 1;
//         println!("The counter is now: {}", counter);
//         if counter > 9 {
//             println!("Now entering the Second loop.");

//             'second_loop: loop {
//                 println!("The second counter is now: {}", counter2);
//                 counter2 += 1;
//                 if counter2 == 3 {
//                     break 'first_loop;
//                 }
//             }
//         }
//     }
// }

// fn main_number() {
//     let my_vec = vec![get_number(-800), get_number(8)];
//     for item in my_vec {
//         match item {
//             Number::U32(angka) => println!("It's a u32 with the value {}", angka),
//             Number::I32(angka) => println!("It's an i32 with the value {}", angka),
//         }
//     }
// }

// fn get_number(input: i32) -> Number {
//     let number = match input.is_positive() {
//         true => Number::U32(input as u32),
//         false => Number::I32(input),
//     };
//     number
// }

// enum Number {
//     U32(u32),
//     I32(i32),
// }

// fn another_enum() {
//     // let my_mood = Mood::Happy;
//     // let my_mood = Mood::Sleepy;
//     // let my_mood = Mood::NotBad;
//     let my_mood = Mood::Angry;
//     let happiness_level = match_mood(&my_mood);
//     println!("Out of 1 to 10, my happiness is {}", happiness_level);
// }

// fn match_mood(mood: &Mood) -> i32 {
//     use Mood::*;
//     let happiness_level = match mood {
//         Mood::Happy => 10, // Here we type Mood:: every time
//         Mood::Sleepy => 6,
//         Mood::NotBad => 7,
//         Mood::Angry => 2,
//     };
//     happiness_level
// }

// enum Mood {
//     Happy,
//     Sleepy,
//     NotBad,
//     Angry,
// }
