use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get input file and read it in
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);
    let input_file_lines = file_reader.lines();

    // Set up instruction map
    let mut instruction_number: isize = 0;
    // Instruction ID => Instruction String
    let mut instruction_map: HashMap<isize, String> = HashMap::new();
    // Instruction ID => Next Instruction ID
    let mut next_instruction_map: HashMap<isize, isize> = HashMap::new();

    // Iterate through input file lines
    for line in input_file_lines{
        // Get the current line without any extra whitespace
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        // Add the current instruction to the map along with an instruction ID and flag it as not visited in the visited map
        instruction_map.insert(instruction_number, String::from(trimmed_line));
        // Now handle the next instruction mapping to preserve order in which instructions are performed
        if trimmed_line.contains("nop") || trimmed_line.contains("acc"){
            // If the instruction is nop or acc, then the next instruction is just the next one down.
            next_instruction_map.insert(instruction_number, instruction_number + 1);
        } else if trimmed_line.contains("jmp"){
            // If the instruction is jmp, then the next instruction is the current one + the offset provided
            // Split the instruction on the space
            let instruction_split: Vec<&str> = trimmed_line.split(' ').collect();
            // Get the element to the right of the space symbol (only one space per instruction, element to right is the offset)
            let offset = instruction_split.get(1).unwrap().parse::<isize>().unwrap();
            next_instruction_map.insert(instruction_number, instruction_number + offset);
        } else {
            // If the instruction is something else, something went wrong
            println!("An invalid instruction was encountered: {}", trimmed_line);
        }
        // Increment the instruction ID
        instruction_number = instruction_number + 1;
    }
    // At this point we have a map of each instruction ID => instruction string and each instruction ID => next instruction ID
    // and a map of each instruction ID => if it's been visited yet.
    // This should be enough to be able to "step through" the code and detect a loop, as well as track the accumulator.

    // PART 1 - get accumulator value right before loop starts over
    let (_ignore, accumulator_value_at_loop) = step_through_code(&instruction_map, &next_instruction_map, instruction_number);
    println!("The value of the accumulator before the loop resets is {}", accumulator_value_at_loop);

    // PART 2 - find which nop/jmp needs to be flipped to terminate the code (reach and run the last instruction in the file)
    part_2_fix_code(&instruction_map, &next_instruction_map, instruction_number);

}

fn step_through_code(instruction_map: &HashMap<isize, String>, next_instruction_map: &HashMap<isize, isize>, 
    instruction_count: isize) -> (Vec<isize>, isize){
    
    // Instruction ID => Visited (this will be used to find loops)
    let mut visited_map: HashMap<isize, bool> = HashMap::new();
    // Fill visited_map with false
    for i in 0..instruction_count{
        visited_map.insert(i, false);
    }

    let mut current_instruction_id: isize = 0;
    let mut accumulator_value = 0;

    let mut critical_instructions_before_loop: Vec<isize> = Vec::new();


    // If the current instruction has not yet been visited AND the program hasn't terminated, then we proceed.
    // Program termination condition goes first because otherwise the program panics due to trying to access/unwrap
    // a value that isn't there.
    while current_instruction_id < instruction_count && visited_map.get(&current_instruction_id).unwrap() == &false{

        // Get the current instruction, as well as its next step, and mark it as visited
        let current_instruction = instruction_map.get(&current_instruction_id).unwrap();
        //println!("{}", current_instruction);
        // If the current instruction is an "acc" then we need to update the accumulator accordingly.
        if current_instruction.contains("acc"){
            // Split the instruction on the space
            let instruction_split: Vec<&str> = current_instruction.split(' ').collect();
            // Get the element to the right of the space symbol (only one space per instruction, element to right is the offset)
            let accumulator_change = instruction_split.get(1).unwrap().parse::<isize>().unwrap();
            // Update the accumulator value
            accumulator_value = accumulator_value + accumulator_change;
        }
        // Mark the current instruction as visited and get the next instruction
        visited_map.insert(current_instruction_id, true);
        // Get the last critical instruction to occur before the loop (critical = jmp or nop)
        if !current_instruction.contains("acc"){
            critical_instructions_before_loop.push(current_instruction_id);
        }
        
        current_instruction_id = *next_instruction_map.get(&current_instruction_id).unwrap();

    } 
    // If the current instruction has been visited, the function ends here
    // EXIT STATE
    // Return the last instruction before the loop (for part 2) and the accumulator value (for part 1)
    return (critical_instructions_before_loop, accumulator_value);
}

// Since there are less than 100 instructions in the input, the easiest solution will be just fine: 
// Just work backward and try flipping the last jmp/nop, then if that doesn't terminate, try the one before, etc.
// I'M TOO INVESTED NOW BUT I PROBABLY SHOULD'VE DONE A DOUBLY LINKED LIST OF INSTRUCTIONS AND JUST BACKTRACKED/REPROCESSED?
fn part_2_fix_code(instruction_map: &HashMap<isize, String>, next_instruction_map: &HashMap<isize, isize>,
    instruction_count: isize){
    
    // Get the last instruction before the loop started and the 
    let (critical_instructions_before_loop, _accumulator_value) = step_through_code(instruction_map, next_instruction_map, instruction_count);

    // println!("{:?}", critical_instructions_before_loop);
    
    // Iterate through the critical instructions
    for instruction_id in critical_instructions_before_loop{ 
        let last_instruction = instruction_map.get(&instruction_id).unwrap();
        // Clone the instruction map/next instruction map so there can be a modified version without affecting the original
        let mut updated_instruction_map = instruction_map.clone();
        let mut updated_next_instruction_map = next_instruction_map.clone();
        // Flip the last instruciton (from nop to jmp)
        if last_instruction.contains("nop"){
            // If it was a nop, flip to jmp and get the new offset to go to
            updated_instruction_map.insert(instruction_id, last_instruction.replace("nop", "jmp"));
            // Get the new offset to jump to
            // Split the instruction on the space
            let instruction_split: Vec<&str> = last_instruction.split(' ').collect();
            // Get the element to the right of the space symbol (only one space per instruction, element to right is the offset)
            let offset = instruction_split.get(1).unwrap().parse::<isize>().unwrap();
            updated_next_instruction_map.insert(instruction_id, instruction_id + offset);
        } else if last_instruction.contains("jmp"){
            // If it was a jmp, flip to nop and set the next instruction to itself + 1
            updated_instruction_map.insert(instruction_id, last_instruction.replace("jmp", "nop"));
            updated_next_instruction_map.insert(instruction_id, instruction_id + 1);
        }
        
        // Run through the code with the updated instructions and see what happens
        let (critical_instructions_before_loop, accumulator_value) = step_through_code(&updated_instruction_map, &updated_next_instruction_map, instruction_count);
        // println!("{:?}", critical_instructions_before_loop);
        
        // Get the last instruction that was run
        let last = *critical_instructions_before_loop.last().unwrap();
        // Check if the last item run was the last instruction (termination case)
        if last == instruction_count - 1{
            // If it was, print the accumulator value at termination and end the program
            println!("The value of the accumulator at termination is {}", accumulator_value);
            break;
        }
    }
}