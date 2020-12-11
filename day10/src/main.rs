use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get input file and read it in for each iteration
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);
    let input_file_lines = file_reader.lines();

    let mut adapter_list: Vec<usize> = Vec::new();
    let mut current_joltage = 0;
    let mut one_jolt_steps = 0;
    let mut two_jolt_steps = 0;
    let mut three_jolt_steps = 0;

    // Push 0 as our starter condition - we start from 0, not from the first adapter
    adapter_list.push(0);

    // Iterate through the lines of the input file
    for line in input_file_lines{
        // Just get all of the adapter values and push them to the adapter list
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        let adapter_value = trimmed_line.parse::<usize>().unwrap();
        adapter_list.push(adapter_value);
    }

    // Now that we have the full list of adapter values, we sort it to get it in order from smallest to largest
    adapter_list.sort();

    // Now that it's in a sorted list, we just step through each item and check that it's within 3 jolts of the current joltage
    // If it is, then we track how big of a jump it is and move on

    // PART 1 - always grab the LOWEST possible step
    // Cloning the adapter list here so it can still be used for part 2
    for adapter in &adapter_list{
        // Skip the 0 case
        if adapter != &0{
            let joltage_jump = adapter - current_joltage;
            // Check that the adapter being checked is rated at most 3 jolts higher than the current joltage
            match joltage_jump{
                1 => one_jolt_steps = one_jolt_steps + 1,
                2 => two_jolt_steps = two_jolt_steps + 1,
                3 => three_jolt_steps = three_jolt_steps + 1,
                // default case should never happen
                _ => {
                    println!("A joltage jump of more than 3 jolts was detected, this shouldn't have happened!");
                    break;
                }
            }
            current_joltage = *adapter;
    }
    }

    // Adding one to the three-jolt steps because of the final adapter to device jump
    three_jolt_steps = three_jolt_steps + 1;

    println!("Number of 1-jolt jumps: {}", one_jolt_steps);
    println!("Number of 2-jolt jumps: {}", two_jolt_steps);
    
    println!("Number of 3-jolt jumps: {}", three_jolt_steps);
    println!("Device joltage rating: {}", current_joltage + 3);
    println!("1-step jumps times 3-step jumps = {}", one_jolt_steps * three_jolt_steps);

    // PART 2
    let adapter_count = adapter_list.len();
    
    let branch_total = find_branches(&adapter_list, adapter_count);
    println!("{} possible arrangements", branch_total);

}

// Get the number of possible arrangements of the input adapter space
fn find_branches(adapter_list: &Vec<usize>, adapter_count: usize) -> isize{
    let mut branch_total = 1;
    let mut current_split = 0;
    // Loop through the adapter list
    for (index, adapter) in adapter_list.iter().enumerate(){
        // Since we're doing lookahead, mnake sure that we don't go out of bounds
        if index + 1 < adapter_count{
            // If the next number in the list is x + 3, then there's no possible branch here: you have to use that adapter.
            // So we can split the problem here and do recursion on the sub-problem since that's going to be MUCH smaller
            // then stick the solutions together
            if adapter_list.get(index + 1).unwrap() == &(adapter + 3){ 
                // Get the sub-list (from the last 3-jump to this one)
                let sub_list = adapter_list.get(current_split..index + 1).unwrap();
                // Set the new splitting point
                current_split = index;
                // Make a vector out of the sub-list
                let mut vec_sub_list: Vec<usize> = Vec::new();
                vec_sub_list.extend_from_slice(sub_list);
                // Get the total of branches of the new subproblem and multiply it by the existing number of 
                // branches prior
                branch_total = branch_total * count_branches_in_adapter_list(&vec_sub_list, 0, sub_list.len());
            }
        }
    }
    // do the last subproblem  since that loop doesn't run from last 3-jump to the end
    // Get the sub-list (from the last 3-jump to this one)
    let sub_list = adapter_list.get(current_split..adapter_count).unwrap();
    // Make a vector out of the sub-list
    let mut vec_sub_list: Vec<usize> = Vec::new();
    vec_sub_list.extend_from_slice(sub_list);
    // Get the total of branches of the new subproblem and multiply it by the existing number of 
    // branches prior
    branch_total = branch_total * count_branches_in_adapter_list(&vec_sub_list, 0, sub_list.len());
    // And now we have our total arrangements!
    return branch_total;
}

// Count the branches in the adapter list starting from a given point. Recursive.
fn count_branches_in_adapter_list(adapter_list: &Vec<usize>, current_adapter_index: usize, adapter_count: usize)
    -> isize{
    // Get the current adapter (default of 0 since we won't see an adapter with that joltage).
    let current_adapter = adapter_list.get(current_adapter_index).unwrap_or(&0);

    // Track the number of branches found
    let mut branches = 0;

    let mut one_step_branches = 0;
    let mut two_step_branches = 0;
    let mut three_step_branches = 0;

    // For each jolt jump (one, two, three jolts), check if the current number has an adapter to that jump.
    // If it does, count the number of branches after taking that jump and report it back.
    let one_step = adapter_list.iter().position(|next_adapter| next_adapter == &(current_adapter + 1));
    if one_step != None{
        let one_step_index = one_step.unwrap();
        one_step_branches = count_branches_in_adapter_list(adapter_list, one_step_index, adapter_count);
    }
    let two_step = adapter_list.iter().position(|next_adapter| next_adapter == &(current_adapter + 2));
    if two_step != None{
        let two_step_index = two_step.unwrap();
        two_step_branches = count_branches_in_adapter_list(adapter_list, two_step_index, adapter_count);
    }
    let three_step = adapter_list.iter().position(|next_adapter| next_adapter == &(current_adapter + 3));
    if three_step != None{
        let three_step_index = three_step.unwrap();
        three_step_branches = count_branches_in_adapter_list(adapter_list, three_step_index, adapter_count);
    }

    // The total number of branches is the number of branches for all possible jumps
    branches = one_step_branches + two_step_branches + three_step_branches;

    // If there are no branches, we've reached the end - return 1.
    if branches == 0{
        return 1
    }

    // return the number of branches.
    return branches
}