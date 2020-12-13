use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get input file and read it in
    let file = File::open("./input.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    // Strings to read the first and second line from the file
    let mut time_string = String::new();
    let mut bus_string = String::new();

    // Read in the time
    let _time_bytes= file_reader.read_line(&mut time_string).unwrap();
    let time = time_string.trim().parse::<isize>().unwrap();

    // Read in the bus schedule
    let _bus_bytes = file_reader.read_line(&mut bus_string).unwrap();
    let bus_schedule_split: Vec<&str> = bus_string.split(',').collect();

    // Iterate over the bus schedule and ignore the x's, only keep the numbers
    let mut bus_schedule: Vec<isize> = Vec::new();
    for slot in bus_schedule_split{
        // Ignore the x spots - they're worthless
        if slot != "x"{
            bus_schedule.push(slot.parse::<isize>().unwrap());
        }
    }

    let (best_bus, best_bus_arrival_time) = part_1_find_best_bus_time(time, &bus_schedule);
    println!("The soonest bus that can be departed on is bus ID {}", best_bus);
    println!("It leaves {} minutes after arrival. The answer to Part 1 is {}", best_bus_arrival_time - time, (best_bus_arrival_time - time) * best_bus);

    let second_bus_schedule_split: Vec<&str> = bus_string.split(',').collect();
    let earliest_consecutive = part_2_find_consecutive_times(&second_bus_schedule_split);
    println!("The earliest time where each bus arrives consecutively (with offset) is {}", earliest_consecutive);
}

fn part_1_find_best_bus_time(time: isize, bus_schedule: &Vec<isize>) -> (isize, isize){
    // Find the closest bus ID
    let mut closest_bus = 0;
    let mut closest_bus_remainder = 1.0;
    let mut closest_bus_arrival_time = 0;

    for bus_id in bus_schedule{
        // Ignore the x spots - they're worthless
        // Get the remainder of the time divided by the bus ID
        let time_float = time as f64;
        let bus_id_float = *bus_id as f64;
        let bus_remainder: f64 = (time_float / bus_id_float) % 1.0;
        // Whichever remainder is closest to 0 is the soonest bus ID
        if bus_remainder < closest_bus_remainder{
            closest_bus_remainder = bus_remainder;
            closest_bus = *bus_id;
            // Also get the bus's arrival time:
            closest_bus_arrival_time = ((time_float / bus_id_float).ceil() * bus_id_float) as isize;
        } else if (1.0 - bus_remainder) < closest_bus_remainder{
            closest_bus_remainder = 1.0 - bus_remainder;
            closest_bus = *bus_id;
            // Also get the bus's arrival time:
            closest_bus_arrival_time = ((time_float / bus_id_float).ceil() * bus_id_float) as isize;
        }
    }
        
    return (closest_bus, closest_bus_arrival_time);
}

// I absolutely had to look up some hints for this, this one was really tough.
fn part_2_find_consecutive_times(bus_schedule: &Vec<&str>) -> usize{
    let mut bus_ids: Vec<usize> = Vec::new();
    let mut offsets: Vec<usize> = Vec::new();

    let mut earliest_consecutive = 0;

    // Build the vectors of offsets and bus ids; index 0 will give the bus id for the lowest offset etc.
    for (index, bus_id) in bus_schedule.iter().enumerate(){
        // Ignore the x's - we only kept them for correct indexing
        if bus_id != &"x"{
            bus_ids.push(bus_id.parse::<usize>().unwrap());
            offsets.push(index);
        }
    }

    // Grab the first bus ID and offsets and set up a few variables here
    let mut previous_time = 0;
    let bus_iter = bus_ids.iter();
    let mut multiple = 1;

    let mut checked_bus_ids: Vec<usize> = Vec::new();

    // Iterate through all of the bus IDs
    for bus_id in bus_iter{
        // Get the first bus ID only in the first case (after this, it gets updated with other stuff)
        if previous_time == 0{
            previous_time = *bus_id;
            // Also set up the "multiple" - aka what number to add on to every iteration
            multiple = previous_time;
            checked_bus_ids.push(previous_time);
        }
       // In every other case, we actually do the calculation
       else{
            // Get the second bus ID which is to be checked
            let second_bus_id = bus_id;
            checked_bus_ids.push(*second_bus_id);
            // We don't need to go more than max(A,B) times, because at that point we will have gotten A * B, and the valid time
            // is definitely less than that.
            'outer: for current_index in (0..previous_time * second_bus_id){
                
                // Get the current time (aka the previous time that worked best plus the multiple n times)
                let time = previous_time + (current_index * multiple);
                let mut valid = true;

                // Run through all the bus IDs which have been or are being checked currently
                for (index_inner, bus_id_inner) in checked_bus_ids.iter().enumerate(){
                    // Get the offset for each bus ID
                    let bus_offset = offsets.get(index_inner).unwrap();
                    // If the current time being checked + the bus's offset is NOT evenly divisible by the bus ID, then it's invalid
                    // We check for invalidity because that's the critical condition - one valid case isn't good enough, they all need to be
                    if (time + bus_offset) % bus_id_inner != 0{
                        valid = false;
                        break;
                    }
                }

                // If the time is valid, then we update the earliest consecutive time, as well as the previous time to be used
                // for the next iteration, and the multiple - the product of all of the previous bus IDs.
                if valid == true{
                    earliest_consecutive = time;
                    previous_time = time;
                    multiple = checked_bus_ids.iter().product();
                    break 'outer;
                }
            }
        }
    }
    return earliest_consecutive;
}