use regex::Regex;
use std::io::{BufRead,BufReader};
use std::fs::File;

fn main() {
    // Set up expected fields to be found
    let expected_fields: Vec<String> = vec![String::from("byr"), String::from("iyr"), String::from("eyr"), 
    String::from("hgt"), String::from("hcl"), String::from("ecl"), String::from("pid")];
    let mut part_1_valid_passports = 0;
    let mut part_2_valid_passports = 0;


    // Get input file
    let file = File::open("./day4.txt").unwrap();
    let file_reader = BufReader::new(file);

    let mut current_passport = String::new();

    // Run through the lines of the input file
    for (line_index, line) in file_reader.lines().enumerate(){
        // Get the current line and trim off any whitespace
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        // If a blank line is encountered, we've reached the end of the current passport
        if trimmed_line.is_empty(){
            // Check the complete passport - if it's valid for part 1, add it to the list of valid ones for part 1
            let valid = part_1_check_passport(&current_passport, &expected_fields);
            if valid == true{
                part_1_valid_passports = part_1_valid_passports + 1;
            }
            // Check the complete passport - if it's valid for part 2, add it to the list of valid ones for part 2
            let valid = part_2_check_passport(&current_passport, &expected_fields);
            if valid == true{
                part_2_valid_passports = part_2_valid_passports + 1;
            }
            // Reset the passport string
            current_passport.clear();
        }
        // If it's not a blank line, then add the current line to the passport
        else{
            current_passport = current_passport + " " + current_line.as_str();
        }
    }   

    // Process final passport for part 1
    let valid = part_1_check_passport(&current_passport, &expected_fields);
    if valid == true{
        part_1_valid_passports = part_1_valid_passports + 1;
    }
    println!("The number of valid passports in part 1 is {}", part_1_valid_passports);

    // Check the complete passport - if it's valid for part 2, add it to the list of valid ones for part 2
    let valid = part_2_check_passport(&current_passport, &expected_fields);
    if valid == true{
        part_2_valid_passports = part_2_valid_passports + 1;
    }  
    println!("The number of valid passports in part 2 is {}", part_2_valid_passports);
}

fn part_1_check_passport(passport: &String, expected_fields: &Vec<String>) -> bool{
    
    let mut found_fields: Vec<String> = Vec::new();
    // Check for all fields in the passport
    let re = Regex::new(r"\w{3}:").unwrap();
    // Push them all to a vector
    for cap in re.captures_iter(passport) {
            // Strip off the colon - we only want the first three chracters.
            found_fields.push(String::from(&cap[0][0..3]));
    }
    
    // Iterate through all of the expected fields and confirm that they were all found.  If they were, passport is valid.
    if expected_fields.iter().all(|item| found_fields.contains(item)){
        return true;
    }
    else{
        return false;
    }
}

fn part_2_check_passport(passport: &String, expected_fields: &Vec<String>) -> bool{
    
    // We need to get the field:value pair (that's the right regex in there already)
    // Parse that into a hashmap
    // Then check all the fields for correctness, as well as check that all of the conditions are satisfied.

    let mut fields_valid = true;
    let mut found_fields: Vec<String> = Vec::new();
    // Check for all fields in the passport
    let re = Regex::new(r"\w{3}:\S+").unwrap();
    // Read the field and value into a vector comprising those, and also add to list of found fields
    for cap in re.captures_iter(passport) {
        // Parse the field and value
        let current_field = String::from(&cap[0]);
        let passport_field: Vec<&str> = current_field.split(':').collect();
        // Check the validity of the value
        fields_valid = process_passport_field(&passport_field);
        // If an invalid field arises, the whole passport is invalid
        if fields_valid == false{
            break;
        }
        // Add the field to a list of fields to check that all expected fields are found.
        found_fields.push(passport_field[0].to_string());
    }
    
    // Iterate through all of the expected fields and confirm that they were all found.  If they were and they're all valid values, passport is valid.
    if expected_fields.iter().all(|item| found_fields.contains(item)) && fields_valid == true{
        return true;
    }
    else{
        return false;
    }
}

// There's probably a better way to do this than match statement, but this works for now
fn process_passport_field(passport_field: &Vec<&str>) -> bool{
    // Get the type and value of the field
    let field_type = passport_field[0];
    let field_value = passport_field[1];
    // Depending on the type of the field, check the value
    match field_type {
        "byr" => {
            let re_byr = Regex::new(r"^(19[2-9][0-9]|200[0-2])$").unwrap();
            return re_byr.is_match(field_value);
        }
        "iyr" => {
            let re_iyr = Regex::new(r"^20(1[0-9]|20)$").unwrap();
            return re_iyr.is_match(field_value);
        }
        "eyr" => {
            let re_eyr = Regex::new(r"^20(2[0-9]|30)$").unwrap();
            return re_eyr.is_match(field_value);
        }
        "hgt" => {
            let re_hgt = Regex::new(r"^((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap();
            return re_hgt.is_match(field_value);
        }
        "hcl" => {
            let re_hcl = Regex::new(r"^#([0-9]|[a-f]){6}$").unwrap();
            return re_hcl.is_match(field_value);
        }
        "ecl" => {
            let re_ecl = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            return re_ecl.is_match(field_value);
        }
        "pid" => {
            let re_pid = Regex::new(r"^\d{9}$").unwrap();
            return re_pid.is_match(field_value);
        }
        "cid" => {
            return true;
        }
        _ => {
            println!("An unexpected field arrived!");
            return false;
       }
    }
}