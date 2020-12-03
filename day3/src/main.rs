use std::{fs::File};
use std::io::{BufRead,BufReader};

fn main() {
    // Get the length of the first line to get the line length for wrapping purposes
    let file = File::open("./day3.txt").unwrap();
    let mut file_reader = BufReader::new(file);
    let mut first_line = String::new();
    file_reader.read_line(&mut first_line).unwrap();
    let input_line_length = first_line.trim().chars().count();
    
    // PART 1
    let right_3_down_1 = check_slope(3, 1, input_line_length);
    println!("The number of trees encountered for right 3, down 1 is {}", right_3_down_1);

    // PART 2
    // Check slopes of the relevant sizes
    let right_1_down_1 = check_slope(1, 1, input_line_length);
    let right_5_down_1 = check_slope(5, 1, input_line_length);
    let right_7_down_1 = check_slope(7, 1, input_line_length);
    let right_1_down_2 = check_slope(1, 2, input_line_length);
    let product_of_trees: usize = right_3_down_1 * right_1_down_1 * right_5_down_1 * right_7_down_1 * right_1_down_2;
    println!("RIGHT 1 DOWN 1 {}", right_1_down_1);
    println!("RIGHT 3 DOWN 1 {}", right_3_down_1);
    println!("RIGHT 5 DOWN 1 {}", right_5_down_1);
    println!("RIGHT 7 DOWN 1 {}", right_7_down_1);
    println!("RIGHT 1 DOWN 2 {}", right_1_down_2);
    println!("The product of trees encountered is {}", product_of_trees);

}

fn check_slope(right_steps: usize, down_steps: usize, input_line_length: usize) -> usize{
    // Get input 
    let file = File::open("./day3.txt").unwrap();
    let file_reader = BufReader::new(file);

    // Initialize the number of trees encountered.
    let mut tree_count: usize = 0;
    
    // Run through the lines of 
    for (line_index, line) in file_reader.lines().enumerate(){
        // Skip lines that get hopped over if down_steps > 1
        if line_index % down_steps == 0{
            // Get the text from the line
            let line_text = line.unwrap();
            // Conver to list of chars
            let mut line_chars = line_text.chars();
            // Get the next position moved to (3 over in this case)
            let next_position: char = line_chars.nth(((line_index / down_steps) * right_steps) % input_line_length).unwrap();
            // If the next position is a tree ('#'), increment the counter
            if next_position == '#'{
                tree_count = tree_count + 1;
            }
        }
    }
    // Return the number of trees encountered.
    tree_count
}