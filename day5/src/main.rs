use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get input file
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);

    let mut seat_ids: Vec<isize> = Vec::new();
    let mut max_seat_id = 0;

    // Iterate through the lines
    for line in file_reader.lines(){
        let current_line = line.unwrap();
        // Get the binary patterns from the ticket information
        let (column_binary, row_binary) = map_bin_strings(&current_line);
        // Once the binary is returned, get the int equivalent of the binary
        let column_int = isize::from_str_radix(&column_binary[..], 2).unwrap();
        let row_int = isize::from_str_radix(&row_binary[..], 2).unwrap();

        // Get the unique seat ID
        let seat_id = (column_int * 8) + row_int;
        
        //Add to the vector of seat IDs
        seat_ids.push(seat_id);

        // PART 1 - Get the highest Seat ID
        if seat_id > max_seat_id{
            max_seat_id = seat_id;
        }
    }

    println!("Max seat ID in the list: {}", max_seat_id);

    // PART 2 - Find the missing seats in the list and determine which one has seats with IDs +1 and -1 in the list (that's my seat)
    
    // Check each potential seat ID (from front of plane to highest assigned seat ID found in Part 1)
    for potential_seat_id in 0..max_seat_id{
        // If the current seat ID is not in the list of assigned seats, this could be my seat
        if !seat_ids.contains(&potential_seat_id){
            // If the seats with ID + 1 and ID - 1 are assigned, this is my seat!
            if seat_ids.contains(&(potential_seat_id + 1)) && seat_ids.contains(&(potential_seat_id - 1)){
                println!("My seat is: {}", &potential_seat_id);
            }
        }
    }

}

fn map_bin_strings(ticket: &String) -> (String, String) {
    // First 7 letters = column, last 3 is row
    let column_string: &str = &ticket[0..7];
    let row_string: &str = &ticket[7..];

    let mut column_binary = String::new();
    let mut row_binary = String::new();

    // Get the column binary pattern
    for letter in column_string.chars(){
        if letter == 'F'{
            column_binary.push('0');
        } else if letter == 'B'{
            column_binary.push('1');
        } else {
            println!("An invalid character is in the pattern {}.  Please ensure ticket information is valid.", column_string);
        }
    }

    // Get the row binary pattern
    for letter in row_string.chars(){
        if letter == 'L'{
            row_binary.push('0');
        } else if letter == 'R'{
            row_binary.push('1');
        } else {
            println!("An invalid character is in the pattern {}.  Please ensure ticket information is valid.", row_string);
        }
    }

    return (column_binary, row_binary);
}