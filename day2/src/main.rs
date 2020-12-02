use std::{fs::File};
use std::io::{BufRead,BufReader};
use regex::Regex;

fn main() {
    // Define which part is to be run
    let part1 = true;
    let part2 = true;
    // Get the input file
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);

    // Declare regexps
    let first_digit_re = Regex::new(r"\d+-").unwrap();
    let second_digit_re = Regex::new(r"-\d+").unwrap();
    let key_letter_re = Regex::new(r"\w+:").unwrap();
    let password_re = Regex::new(r"\w+$").unwrap();

    let mut valid_password_count_part1 = 0;
    let mut valid_password_count_part2 = 0;

    //Read in the input
    for line in file_reader.lines() {
        // Get the current line
        let current_line = line.unwrap();
        // Parse the input into first_num, second_num, key_letter, password

        // Get the captures from the regex in each line
        let first_digit_cap = first_digit_re.captures(&current_line[..]).unwrap();
        let second_digit_cap = second_digit_re.captures(&current_line[..]).unwrap();
        let key_letter_cap = key_letter_re.captures(&current_line[..]).unwrap();
        let password_cap = password_re.captures(&current_line[..]).unwrap();

        // Remove any non-relevant characters
        let first_digit = &first_digit_cap[0].replace('-', "");
        let second_digit = &second_digit_cap[0].replace('-', "");
        let key_letter = &key_letter_cap[0].replace(':', "");
        let password = &password_cap[0];

        // PART 1
        if part1 == true{

            // get the minimum and maximum number of occurences of the key letter in the password
            let min_count: usize = first_digit.parse().unwrap();
            let max_count: usize = second_digit.parse().unwrap();

            // get the actual number of occurences of the key letter in the password
            let num_of_occurences = password.matches(key_letter).count();

            // Check if the actual number is within the limits; if so, the password is valid
            if num_of_occurences <= max_count && num_of_occurences >= min_count {
                valid_password_count_part1 = valid_password_count_part1 + 1;
            }

        }

        // PART 2
        if part2 == true{
            // Get the first position and last position where the key letter must appear
            let first_position: usize = first_digit.parse().unwrap();
            let last_position: usize = second_digit.parse().unwrap();

            // Index starts at 1 for the problem, so we subtract 1 from each position to simplify things
            let first_position_adjusted = first_position - 1;
            let last_position_adjusted = last_position - 1;

            let mut count_of_occurrences = 0;

            // Iterate through the characters in the password
            for (index, character) in password.chars().enumerate(){
                // Check if the current character's index is between the required positions
                if index == first_position_adjusted || index == last_position_adjusted {
                    // If the current character matches the key character, add to the count of occurences
                    if character == key_letter.chars().nth(0).unwrap() {
                        count_of_occurrences = count_of_occurrences + 1;
                    }
                }
            }

            // If the key letter appears more than once in the range, the password is not valid
            if count_of_occurrences == 1{
                valid_password_count_part2 = valid_password_count_part2 + 1;
            }
            

        }
    }

    if part1 == true{
        println!("There are {} valid passwords for part 1 in the input!", valid_password_count_part1);
    } 
    if part2 == true {
        println!("There are {} valid passwords for part 2 in the input!", valid_password_count_part2);
    }
}