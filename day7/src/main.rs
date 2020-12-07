use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut part_1_target_bag_list: Vec<String> = [String::from("shiny gold")].to_vec();
    let part_2_target_bag: String = String::from("shiny gold");
    let mut eventual_containers: String = String::new();

    // PART 1 - Count bags which can eventually hold the shiny gold bag
    while part_1_target_bag_list.len() > 0{
        // Get the list of bags which can hold the initial target bag
        part_1_target_bag_list = part_1_get_containers(&part_1_target_bag_list);
        // Append those parents to the list of bags which can eventually hold the target bag.
        for bag in &part_1_target_bag_list {
            // Make sure this type of bag doesn't already exist as another bag's container
            if eventual_containers.matches(bag).count() == 0{
                eventual_containers = eventual_containers + bag.as_str() + ", " ;
            }
            
        }
    }

    // Get the number of commas, which should match the number of containers:
    println!("{} bags can hold the shiny gold bag.", eventual_containers.matches(',').count());

    // PART 2 - Count number of bags which must be contained within (at some level of depth) the shiny gold bag
    let child_bag_count = part_2_get_containers(&part_2_target_bag);
    println!("The shiny gold bag must hold {} bags", child_bag_count);
}

// Gets a list of target bags and returns the list of bags which can hold those bags
fn part_1_get_containers(target_bag_list: &Vec<String>) -> Vec<String>{
    // Get input file and read it in for each iteration
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);
    let input_file_lines = file_reader.lines();

    // Get the result list
    let mut container_bags: Vec<String> = Vec::new();

    for line in input_file_lines{

        // Get the current line and trim off any whitespace
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        // Split the string on the phrase "bags contain".  Left side is the container, right side is what can be contained
        // Split_line.get(0) should return container, split_line.get(1) should return all containable bags
        let split_line: Vec<&str> = trimmed_line.split(" bags contain ").collect();
        // There should be a left side (container) and a right side (bags contained)
        // Check each item in the target bag list and see if it appears in the possible contained bags
        for target_bag in target_bag_list{
            // If a target bag is found in the list of potential contained bags
            // AND that potential contained bag is not already in the list of contained bags,
            // add it to the list of containers
            let potential_container = split_line.get(0).unwrap().to_string();
            let containable_bags = split_line.get(1).unwrap();
            if containable_bags.contains(target_bag) && !container_bags.contains(&potential_container){
                container_bags.push(potential_container);
            }
        }
    }

    // Return the complete list of container bags
    return container_bags;
}

fn part_2_get_containers(target_bag: &String) -> usize{
    // Get input file and read it in for each iteration
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);
    let input_file_lines = file_reader.lines();

    // Get the result list
    let mut child_bags: HashMap<String, usize> = HashMap::new();
    let mut nested_bag_count = 0;


    for line in input_file_lines{
        // Get the current line and trim off any whitespace
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        // Split the string on the phrase "bags contain".  Left side is the container, right side is what can be contained
        // Split_line.get(0) should return container, split_line.get(1) should return all containable bags
        let split_line: Vec<&str> = trimmed_line.split(" bags contain ").collect();
        // There should be a left side (container) and a right side (bags contained)

        // If a target bag is found in the list of potential contained bags
        // add it to the list of containers
        let potential_container = split_line.get(0).unwrap().to_string();
        let containable_bags = split_line.get(1).unwrap().split(',');
        if potential_container.contains(target_bag){
            for child_bag in containable_bags{
                // If this bag contains no other bags, don't bother processing all this
                if !child_bag.contains("no other"){
                    // child_bag will be like "1 dark olive bag"
                    let mut child_bag_split = child_bag.trim().splitn(2, ' ');
                    // Number of children will be 1
                    let number_of_children = child_bag_split.next().unwrap().parse::<usize>().unwrap();
                    // This next line is dumb but basically it'll basically get "dark olive bag" and 
                    // split off the "dark olive" and save that
                    let color_of_child = child_bag_split.next().unwrap().split(" bag").next().unwrap().to_string();
                    // Make a hash map of the nested bags containing their color and the number of them
                    child_bags.insert(color_of_child, number_of_children);
                }
            }
        }
    }

    // Iterate through all of the nested bags for this specific bag
    for (color, number) in child_bags{
        // Recursively get the number of bags within the curent bag and add to the total
        nested_bag_count = nested_bag_count + (number + (number * part_2_get_containers(&color)));
    }

    return nested_bag_count;
}