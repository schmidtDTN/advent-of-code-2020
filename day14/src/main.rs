use std::{collections::HashMap, fs::File};
use std::io::{BufRead, BufReader};
use convert_base::Convert;

fn main() {
    // Get input file and read it in
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);
    let input_file_lines = file_reader.lines();

    let mut current_mask = String::new();
    let mut mem_address = String::new();
    let mut mem_value = String::new();

    let mut part_1_memory_store: HashMap<isize, u64> = HashMap::new();
    let mut part_2_memory_store: HashMap<u64, isize> = HashMap::new();

    let mut part_2_solution = 0;

    // Iterate through all of the lines in the input file
    for line in input_file_lines{
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();

        // Get the LHS and RHS's
        let mut line_split = trimmed_line.split(" = ");
        // First item is the LHS
        let header = line_split.next().unwrap();
        // Second item is the RHS
        let value = line_split.next().unwrap();
        // Get the mask if this is a mask line:
        if header.contains("mask"){
            current_mask = String::from(value);
            mem_address = String::new();
            mem_value = String::new();
        }
        // If this is a mem line, we need to handle that
        else{
            mem_address = header.replace("mem[", "").replace("]", "");
            mem_value = String::from(value);
        }
        // If we have a mem_address, we must have a mask - so as long as we have a mem_address, we process
        if mem_address != String::new(){
            // PART 1
            let mem_address_isize = mem_address.parse::<isize>().unwrap();
            let resulting_value = part_1_process_value(&mem_value, &current_mask);
            part_1_memory_store.insert(mem_address_isize, resulting_value);

            // PART 2
            let mem_value_isize = mem_value.parse::<isize>().unwrap();
            let addresses = part_2_process_address(&mem_address, &current_mask);
            // Insert the value at each of the addresses
            for address in addresses{
                part_2_memory_store.insert(address, mem_value_isize);
            }
        }
    }

    // Once all the memory values are stored in the hashmap appropriately, we iterate through it and sum them to get 
    // Part 1's answer
    let mut part_1_solution = 0;
    for memory_value in part_1_memory_store.values(){
        part_1_solution = part_1_solution + memory_value;
    }

    println!("The solution to Part 1 is {}", part_1_solution);

    // Once all the memory values are stored in the part 2 hashmap, sum them to get part 2's answer
    for memory_value in part_2_memory_store.values(){
        part_2_solution = part_2_solution + memory_value;
    }

    println!("The solution to Part 2 is {}", part_2_solution);

}

fn part_1_process_value(mem_value: &String, mask: &String) -> u64{
    let mut mem_value_bin_vec = get_36_bit_binary_vector(mem_value);

    // Make vector of mask and reverse it also
    let mut mask_vec: Vec<char> = mask.chars().collect();
    mask_vec.reverse();

    // Iterate through the mask characters
    for (index, mask_char) in mask_vec.iter().enumerate(){
        // If it's an X, do nothing
        // If it's a 0 or a 1, overwrite that position in the memory value
        if mask_char == &'0' || mask_char == &'1'{
            // Convert the char to a u64
            let mask_u64: u64 = mask_char.to_digit(10).unwrap() as u64;
            // Update the slot in the memory value (safe to access by indexing because they have same len)
            mem_value_bin_vec[index] = mask_u64;
        }
    }

    let memory_value = convert_binary_vector_to_decimal_number(&mem_value_bin_vec);
    return memory_value;
}

fn part_2_process_address(mem_address: &String, mask: &String) -> Vec<u64>{
    // Get the binary vector for the memory address
    let mem_address_bin_vec = get_36_bit_binary_vector(mem_address);

    // Make vector of mask and reverse it also
    let mut mask_vec: Vec<char> = mask.chars().collect();
    mask_vec.reverse();

    // Create a char vector for the binary vector so that we can add the X's in for processing
    let mut mem_address_bin_char_vec: Vec<char> = vec!['0'; 36];

    // Iterate through the mask characters
    for (index, mask_char) in mask_vec.iter().enumerate(){
        // If it's a 0, just convert the number to a char (for later processing)
        if mask_char == &'0'{
            // Convert the number to a char (safe to access by indexing because they have same len)
            mem_address_bin_char_vec[index] = mem_address_bin_vec[index].to_string().chars().next().unwrap();
        }
        // If it's a 1, overwrite that position in the memory value
        if mask_char == &'1'{
            // Update the slot in the memory value (safe to access by indexing because they have same len)
            mem_address_bin_char_vec[index] = *mask_char;
        }
        // If it's an X, the position is floating and we need to generate all possible combinations of values
        if mask_char == &'X'{
            // Insert an X into the vector for late processing
            mem_address_bin_char_vec[index] = 'X';
        }
    }

    let processed_addresses = process_floating_addresses(&mem_address_bin_char_vec);
    let mut return_addresses: Vec<u64> = Vec::new();

    for address_vec in processed_addresses{
        let address = convert_char_vector_to_decimal_number(&address_vec);
        return_addresses.push(address);
    }

    return return_addresses;
}

fn process_floating_addresses(mem_address_template: &Vec<char>) -> Vec<Vec<char>>{

    // Create a vector which will be populated with address chars
    let mut processed_addresses: Vec<Vec<char>> = Vec::new();

    let mut x_positions: Vec<usize> = Vec::new();

    for (index, address_char) in mem_address_template.iter().enumerate(){
        // Ignore 0s and 1s
        // If we get an X then we want to track the location of that X
        if address_char == &'X'{

            x_positions.push(index);
        }
    }

    let power_base: i32 = 2;

    //Iterate for 2^n times where n = number of X's
    for index in 0..power_base.pow(x_positions.len() as u32){
        // Get the binary representation of the index into a vector of 1s and 0s
        // (starts at 0,0,0,0,0,0... until 1,1,1,1,1...)
        let mut bin = Convert::new(10, 2);
        let mut bin_index = bin.convert::<u64, u64>(&vec![index as u64]);
        // Stuff with 0s as needed to fill out the number of x positions that need popualted
        while bin_index.len() < x_positions.len() {
            bin_index.push(0);
        }
        // Clone the template of the address that needs filled
        let mut current_address: Vec<char> = mem_address_template.clone();
        // Fill the X's with the values of the index (as binary) 
        for step in 0..bin_index.len(){
            let index_to_change = x_positions.get(step);
            current_address[*index_to_change.unwrap()] = bin_index.get(step).unwrap().to_string().chars().next().unwrap();
        }
        // Add to the list of addresses
        processed_addresses.push(current_address);
    }

    // Return the list of processed addresses
    return processed_addresses;
}

fn get_36_bit_binary_vector(value_to_convert: &String) -> Vec<u64>{

    let value_vec: Vec<u64> = vec![value_to_convert.parse::<u64>().unwrap()];

    // Convert the value from base 10 to binary - the output vector is reversed, but this is actually good
    // Since it's reversed, we can work with the mask right to left and overwrite anything that needs overwritten
    let mut convert_to_binary = Convert::new(10, 2);
    let mut value_bin_vec = convert_to_binary.convert::<u64, u64>(&value_vec);
    
    // Fill out the vector to 36 bits long, pad the missing ones with 0s.
    while value_bin_vec.len() < 36{
        value_bin_vec.push(0);
    }

    return value_bin_vec;
}

fn convert_binary_vector_to_decimal_number(vector_to_convert: &Vec<u64>) -> u64{
    // Convert the modified binary vector back to decimal
    let mut bin_to_dec = Convert::new(2, 10);
    let mut mem_value_dec_vec = bin_to_dec.convert::<u64, u64>(&vector_to_convert);
    // Reverse the output vector 
    mem_value_dec_vec.reverse();

    // Convert from vector to decimal number
    let return_value = mem_value_dec_vec.iter().fold(0, |acc, elem| acc * 10 + elem);

    return return_value;
}

fn convert_char_vector_to_decimal_number(vector_to_convert: &Vec<char>) -> u64{
    // Create a vector to hold the u64s
    let mut u64_vector: Vec<u64> = Vec::new();

    // For each char in the input vector, convert to u64 and push to vector
    for character in vector_to_convert{
        let u64_character = character.to_digit(10).unwrap() as u64;
        u64_vector.push(u64_character);
    }
    
    // Call the binary to decimal function and return its result
    return convert_binary_vector_to_decimal_number(&u64_vector);
}