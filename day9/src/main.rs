use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get input file and read it in for each iteration
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);
    let input_file_lines = file_reader.lines();
    
    let preamble_size = 25;
    let mut preamble: Vec<usize> = Vec::new();
    let mut total_list: Vec<usize> = Vec::new();
    let mut invalid_number: usize = 0;

    // Iterate through the lines
    for (line_index, line) in input_file_lines.enumerate(){
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        let number = trimmed_line.parse::<usize>().unwrap();
        // Push the number to a total list of numbers for part 2
        total_list.push(number);
        // For the first n lines (where n == size of preamble), just shove it into the vector
        if line_index < preamble_size{
            preamble.push(number);
        }
        // Once we're past n numbers, we start processing each incoming number and checking if the new number is the sum of some pair
        // in the preamble
        // This isn't a great method, O(n^2) sucks but it's simple and quick.
        else{
            let mut match_found = false;
            // Iterate through some first number
            'outer: for first_num_in_preamble in &preamble{
                // Iterate through second number
                for second_num_in_preamble in &preamble{
                    // If the two numbers are different and sum to the new number, we have a match
                    if first_num_in_preamble != second_num_in_preamble && first_num_in_preamble + second_num_in_preamble == number{
                        // println!("{} and {} sum up to {}", first_num_in_preamble, second_num_in_preamble, number);
                        // Mark that a match was found
                        match_found = true;
                        // Break out of the loops if a match was found
                        break 'outer;
                    }
                }
            }
            // If no match was found, flag the number as invalid
            if match_found == false{
                println!("{} is an invalid number!", number);
                invalid_number = number;
            }
            // Replace the oldest number in the preamble with the new number
            preamble[line_index % preamble_size] = number;
        }
    }

    part_2(total_list, invalid_number)
}

// Equally (probably more) inefficient as part 1, but it works
fn part_2(total_list: Vec<usize>, invalid_number: usize){
    let mut min_match: usize = 0;
    let mut max_match: usize = 0;
    // Iterate through some first number from the total list
    'outer: for (first_index, first_num_in_list) in total_list.iter().enumerate(){
        let mut running_total = *first_num_in_list;
        min_match = *first_num_in_list;
        max_match = *first_num_in_list;
        // Iterate through second number
        for (second_index, second_num_in_list) in total_list.iter().enumerate(){
            // Make sure not to add numbers equal to or before the first number in the list
            if second_index > first_index{
                // Add the current number to the running total
                running_total = running_total + second_num_in_list;
                // If the current number is larger than the current max, update the max
                if second_num_in_list > &max_match{
                    max_match = *second_num_in_list;
                }
                // If the current number is smaller than the current min, update the min
                if second_num_in_list < &min_match{
                    min_match = *second_num_in_list;
                }

                // If the numbers sum to the invalid number, we have a set
                if running_total == invalid_number{
                    // println!("Found a match between {} and {}", first_num_in_list, second_num_in_list);
                    // Break out of the loops if a set was found
                    break 'outer;
                }
                // If the running total exceeds the invalid number, no need to keep trying this first_number
                if running_total > invalid_number{
                    break;
                }
            }
        }
    }
    println!("The sum of the smallest and largest number in the range that sums to the invalid number is {}", max_match + min_match);
}
