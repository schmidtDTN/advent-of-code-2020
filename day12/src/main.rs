use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    
    let part_1 = part_1_manhattan_distance();
    println!("Part 1 Manhattan distance = {}", part_1);

    let part_2 = part_2_manhattan_distance();
    println!("Part 2 Manhattan distance = {}", part_2);
}

fn part_1_manhattan_distance() -> i32{
    // Get input file and read it in
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);
    let input_file_lines = file_reader.lines();
    
    let mut east_distance = 0;
    let mut north_distance = 0;

    // 0 = East, 1 = South, 2 = West, 3 = North
    let mut current_heading = 0;

    // Iterate through all of the lines in the input file
    for line in input_file_lines{
        // Get the line and trim it
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        // Get the first character (the instruction) as char and the remaining characters (the distance) as isize
        let instruction_direction = trimmed_line.chars().nth(0).unwrap();
        let instruction_distance = &trimmed_line[1..].parse::<i32>().unwrap();
        // Process the instruction - get the new ship heading back as well as the heading for the current instruction
        let (update_status, new_heading) = part_1_process_instruction(current_heading, instruction_direction, instruction_distance);
        // If the ship heading gets updated, then update it here
        if update_status == 1{
            current_heading = new_heading;
        }
        // Move the ship in the requested direction for the given distance if this wasn't a turn instruction
        if instruction_direction != 'R' && instruction_direction != 'L'{
            // If new heading is 0, then add to the east steps
            if new_heading == 0{
                east_distance = east_distance + instruction_distance;
            }
            // If new heading is 1, then subtract from the north steps
            else if new_heading == 1{
                north_distance = north_distance - instruction_distance;
            }
            // If new heading is 2, then subtract from the east steps
            else if new_heading == 2{
                east_distance = east_distance - instruction_distance;
            }
            // If new heading is 3, then add to the north steps
            else if new_heading == 3{
                north_distance = north_distance + instruction_distance;
            }
        }
    }

    // Return the manhattan distance
    return north_distance.abs() + east_distance.abs();
}

fn part_1_process_instruction(current_heading: i32, instruction_direction: char, instruction_distance: &i32) -> (i32, i32){
    // Update status = 0 if heading isn't updated, 1 if heading is updated
    let mut update_status = 0;
    let mut new_heading = 0;

    // Cases: N S E W F R L
    match instruction_direction{
        // Update the headings that have no update, just a move in a direction
        'E' => new_heading = 0,
        'S' => new_heading = 1,
        'W' => new_heading = 2,
        'N' => new_heading = 3,
        'F' => new_heading = current_heading,
        // Distance used in the R/L cases to know where to rotate the ship
        // Also in these cases, the heading of the ship needs updated
        'R' =>  {
                    new_heading = part_1_get_new_heading(current_heading, 1, instruction_distance);
                    update_status = 1;
                }
        'L' =>  {
                    new_heading = part_1_get_new_heading(current_heading, 0, instruction_distance);
                    update_status = 1;
                }
        
        _ => println!("Something went wrong - an instruction direction was given that was unexpected: {}", instruction_direction),
    }

    // Need to know when to update the ship's heading vs. just go that direction
    return (update_status, new_heading);
}

// Rotation direction = 1 if turning right, 0 if turning left.
fn part_1_get_new_heading(current_direction: i32, rotation_direction: isize, rotation_amount: &i32) -> i32{
    let new_direction;

    // Turning to the right means we ADD the rotation amount
    if rotation_direction == 1{
        // New direction = our current direction (0-3) + the rotation amount / 90 (+ 1 for every step along the compass)
        // all modulo'd by 4 (so that if we end up adding 270 degrees from south, we go from 1 to 4 which wraps around to 0).
        new_direction = (current_direction + (rotation_amount / 90)) % 4;
    }
    // Turning to the left means we SUBTRACT the rotation amount - since % is the REMAINDER function, and we need a MODULO,
    // we use the workaround ((a % b) + b) % b
    else {
        new_direction = (((current_direction - (rotation_amount / 90)) % 4) + 4) % 4;
    }

    return new_direction;
}

fn part_2_manhattan_distance() -> i32{
    // Get input file and read it in
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);
    let input_file_lines = file_reader.lines();
    
    let mut east_distance = 0;
    let mut north_distance = 0;

    let mut current_waypoint_east = 10;
    let mut current_waypoint_north = 1;

    // 0 = East, 1 = South, 2 = West, 3 = North
    

    // Iterate through all of the lines in the input file
    for line in input_file_lines{
        // Get the line and trim it
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        // Get the first character (the instruction) as char and the remaining characters (the distance) as isize
        let instruction_direction = trimmed_line.chars().nth(0).unwrap();
        let instruction_distance = &trimmed_line[1..].parse::<i32>().unwrap();
        // Process the instruction - get the new ship heading back as well as the heading for the current instruction
        let (new_waypoint_east, new_waypoint_north) = part_2_process_instruction(current_waypoint_east, current_waypoint_north, instruction_direction, instruction_distance);
        // Update the waypoint
        current_waypoint_east = new_waypoint_east;
        current_waypoint_north = new_waypoint_north;
        // If the instruction action is F, then we need to update the ship location
        if instruction_direction == 'F'{
            // east distance adds instruction_distance times the east waypoint
            east_distance = east_distance + (current_waypoint_east * instruction_distance);
            north_distance = north_distance + (current_waypoint_north * instruction_distance);
        }
    }

    // Return the manhattan distance
    return north_distance.abs() + east_distance.abs();
}


fn part_2_process_instruction(current_waypoint_east: i32, current_waypoint_north: i32, instruction_direction: char,
     instruction_distance: &i32) -> (i32, i32){
    // Update status = 0 if heading isn't updated, 1 if heading is updated
    let mut new_waypoint_east = current_waypoint_east;
    let mut new_waypoint_north = current_waypoint_north;

    // Cases: N S E W F R L
    match instruction_direction{
        // Update the directional actions which affect the waypoints
        // East/North - add the instruction distance to the current waypoint
        'E' => new_waypoint_east = current_waypoint_east + instruction_distance,
        'N' => new_waypoint_north = current_waypoint_north + instruction_distance,
        // West/South - subtract the instruction distance from the current waypoint
        'W' => new_waypoint_east = current_waypoint_east - instruction_distance,
        'S' => new_waypoint_north = current_waypoint_north - instruction_distance,
        // Distance used in the R/L cases to know where to rotate the ship
        // Also in these cases, the heading of the ship needs updated
        'R' =>  {
                    let (updated_east, updated_north) = part_2_get_rotated_waypoint(current_waypoint_east, current_waypoint_north, 1, instruction_distance);
                    new_waypoint_east = updated_east;
                    new_waypoint_north = updated_north;
                }
        'L' =>  {
                    let (updated_east, updated_north) = part_2_get_rotated_waypoint(current_waypoint_east, current_waypoint_north, 0, instruction_distance);
                    new_waypoint_east = updated_east;
                    new_waypoint_north = updated_north;
                }
        // F means we have to move the ship - this is a special case        
        // Do nothing here in this case - let it be handled in the main loop
        'F' => (),
        _ => println!("Something went wrong - an instruction direction was given that was unexpected: {}", instruction_direction),
    }

    // Need to know when to update the ship's heading vs. just go that direction
    return (new_waypoint_east, new_waypoint_north);
}

fn part_2_get_rotated_waypoint(current_waypoint_east: i32, current_waypoint_north: i32, rotation_direction: i32, 
    rotation_amount: &i32) -> (i32, i32){

        let mut new_waypoint_east = current_waypoint_east;
        let mut new_waypoint_north = current_waypoint_north;

        // For a right turn, for every 90 degrees, switch the magnitude of the north and east waypoint, and negate the north waypoint
        // For a left turn, for every 90 degrees, switch the magnitude of the north and east waypoint, and negate the east waypoint
        // rotation_amount / 90 should give 1 for 90, 2 for 180, 3 for 270, etc., which is what we want
        for turns in 0..(rotation_amount / 90){
            // Store the north value in temp_east and vice versa
            let mut temp_east = new_waypoint_north;
            let mut temp_north = new_waypoint_east;
            // If turning to the right (r_d = 1), then negate the value of temp_north
            if rotation_direction == 1{
                temp_north = temp_north * -1;
            }
            // If turning left (r_d = 0), negate the value of temp_east
            else {
                temp_east = temp_east * -1;
            }
            // Store the updated variables into the new waypoints - this should allow for multiple loops to work correctly too
            new_waypoint_north = temp_north;
            new_waypoint_east = temp_east;
        }

        return (new_waypoint_east, new_waypoint_north);
    }