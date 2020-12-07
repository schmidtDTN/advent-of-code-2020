use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get input file
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);

    // Set up tracking variables
    let mut part_1_group_answers = String::new();
    let mut part_2_members_in_group = 0;

    let mut total_answers_part_1 = 0;
    let mut total_answers_part_2 = 0;

    // Run through the lines of the input file
    for (line_index, line) in file_reader.lines().enumerate(){
        // Get the current line and trim off any whitespace
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        // If a blank line is encountered, we've reached the end of the current group
        if trimmed_line.is_empty(){
            // PART 1
            // Get a vector of all the characters (responses)
            let mut answer_chars: Vec<char> = part_1_group_answers.chars().collect();
            // Sort and dedup to get distinct answers from group
            answer_chars.sort();
            answer_chars.dedup();
            // Add the number of unique answers for this group to the total number of answers
            total_answers_part_1 = total_answers_part_1 + answer_chars.len();
            // PART 2
            total_answers_part_2 = total_answers_part_2 + part_2_get_shared_answers(&part_1_group_answers, part_2_members_in_group, &answer_chars);
            // Reset the string and vector of characters as well as the part 2 group member number tracking
            part_1_group_answers.clear();
            answer_chars.clear();
            part_2_members_in_group = 0;
        }
        // Otherwise, keep adding answers
        else{
            // PART 1 - append the current group's answer to the tracking string
            part_1_group_answers = part_1_group_answers + current_line.as_str();          
            // PART 2 - update the number of people in the group
            part_2_members_in_group = part_2_members_in_group + 1;
        }
    }

    // Handle final group if there's still one to process since there's no newline at the end of the file
    if !part_1_group_answers.is_empty(){
            // PART 1
            // Get a vector of all the characters (responses)
            let mut answer_chars: Vec<char> = part_1_group_answers.chars().collect();
            // Sort and dedup to get distinct answers from group
            answer_chars.sort();
            answer_chars.dedup();
            // Add the number of unique answers for this group to the total number of answers
            total_answers_part_1 = total_answers_part_1 + answer_chars.len();
            // PART 2
            total_answers_part_2 = total_answers_part_2 + part_2_get_shared_answers(&part_1_group_answers, part_2_members_in_group, &answer_chars);
            // Reset the string and vector of characters as well as the part 2 group member number tracking
            part_1_group_answers.clear();
            answer_chars.clear();
            part_2_members_in_group = 0;
    }
    // Output the result
    println!("{}", total_answers_part_1);
    println!("{}", total_answers_part_2);
}

// Get the number of answers that EVERYONE responded yes to
fn part_2_get_shared_answers(part_1_group_answers: &String, part_2_members_in_group: usize,
                                answer_chars: &Vec<char>) -> usize{
    let mut number_of_shared_answers = 0;
    

    // Iterate through unique characters in the string and check how many occurrences there are of each character in the string itself
    
    // Go through each UNIQUE character (answer_chars is basically part_1_group_answers with all the dupes removed)
    for current_character in answer_chars{
        // If the current character is found in the total response string n times (where n is the number of people in the group)
        // then increase the number of answers EVERYONE responded yes to by 1.
        if part_1_group_answers.matches(*current_character).count() == part_2_members_in_group{
            number_of_shared_answers = number_of_shared_answers + 1;
        }
    }
    return number_of_shared_answers;
}