use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Number{
    most_recent_turn: isize,
    two_turns_ago: isize,
    times_spoken: isize,
}

impl Number{
    // Get the age of the number (most recent turn spoken - two turns ago)
    fn get_age(&self) -> isize{
        let mut age = -1;
        // If it was only spoken once, the response is 0
        if self.times_spoken == 1{
            age = 0;
        } else{
            // Spoken more than once - response is age
            age = self.most_recent_turn - self.two_turns_ago;
        }
        return age;
    }

    fn update_speaking_recency(&mut self, turn: isize) -> Self{
        Number { two_turns_ago: self.most_recent_turn,
                most_recent_turn: turn,
                times_spoken: self.times_spoken + 1}
    }

}

fn main() {
    // let now = std::time::SystemTime::now();
    let mut num_turn_map: HashMap<isize, Number> = HashMap::new();
    let mut last_number_spoken = -1;
    let mut turn_number = 1;

    // Get input into a string (just one line and we'll be splitting on commas)
    let input_string = read_to_string("./input.txt").unwrap();
    // Split it
    let input_split: Vec<&str> = input_string.split(',').collect();
    
    // Process the input
    for number_string in input_split{
        // Convert to isize
        let number = number_string.parse::<isize>().unwrap();
        // Process this turn
        let (new_turn_number, _ignored_here) = process_turn(&mut num_turn_map, number, turn_number);
        turn_number = new_turn_number;
        last_number_spoken = number;
    }   

    // Run through future numbers to get the 2020th number for Part 1
    while turn_number <= 2_020{
        // Process this turn
        let (new_turn_number, number_spoken) = process_turn(&mut num_turn_map, last_number_spoken, turn_number);
        turn_number = new_turn_number;
        last_number_spoken = number_spoken;
    }

    println!("The 2020th number spoken is {}", last_number_spoken);

    // Get the 30,000,000th number for Part 2
    while turn_number <= 30_000_000{
        // Process this turn
        let (new_turn_number, number_spoken) = process_turn(&mut num_turn_map, last_number_spoken, turn_number);
        turn_number = new_turn_number;
        last_number_spoken = number_spoken;
    }

    println!("The 30 millionth number spoken is {}", last_number_spoken);
    // println!("{}", now.elapsed().unwrap().as_millis());
    
}


fn process_turn(num_turn_map: &mut HashMap<isize, Number>, last_number_spoken: isize, turn_number: isize) -> (isize, isize){
    let mut age = -1;
    let new_number_spoken: isize;

    let last_num_struct = num_turn_map.get(&last_number_spoken);
    match last_num_struct{
        Some(number_struct) => new_number_spoken = number_struct.get_age(),
        None => new_number_spoken = last_number_spoken,
    } 


    // Check if the number is already in the hashmap: 
    // If it is, then we want to update the Number struct
     if num_turn_map.contains_key(&new_number_spoken){
        // Get the Number from the map and update it:
        let num_to_update = num_turn_map.get_mut(&new_number_spoken).unwrap();
        // Update when it was last spoken
        let updated_number = num_to_update.update_speaking_recency(turn_number);
        // Insert the updated number
        num_turn_map.insert(new_number_spoken, updated_number);
        // println!("{:#?}", num_turn_map);
    } else {
        // If the number is not in the hash map then we need to create it
        let new_number = Number {
            most_recent_turn: turn_number,
            two_turns_ago: 0,
            times_spoken: 1,
        };
        // Put it in the hash map
        num_turn_map.insert(new_number_spoken, new_number);
    }

    // Increment the turn number
    let new_turn_number = turn_number + 1;

    return (new_turn_number, new_number_spoken);
}