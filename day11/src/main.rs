use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use array2d::Array2D;

// Implemented this for memoization before realizing that the runtime wasn't a problem, it was that I had a bug in my code -_-
// Might strip out the memoization in the future just to clean things up.
#[derive(Debug)]
struct SeatView{
    nw_view: i32,
    n_view: i32,
    ne_view: i32,
    e_view: i32,
    se_view: i32,
    s_view: i32,
    sw_view: i32,
    w_view: i32,
}

fn main() {
    // Get input file and read it in
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);
    let input_file_lines = file_reader.lines();
    // Set up the vector of vectors which will be the base for the 2d array
    let mut array_rows: Vec<Vec<u32>> = Vec::new();

    // Iterate through all of the lines in the input file
    for line in input_file_lines{
        // Get the line and trim it
        let current_line = line.unwrap();
        let trimmed_line = current_line.trim();
        // Replace empty seats (L) with 0 and floor (.) with 2.  The encoding will be empty = 0, occupied = 1, floor = 2
        let number_line = trimmed_line.replace('L',"0").replace('.',"2");
        // Convert the string of encoded seats to a vector of digits
        let current_row: Vec<u32> = number_line.chars().flat_map(|chr| chr.to_digit(10)).collect();
        // Add the current row to the vector of vectors
        array_rows.push(current_row);
    }

    // Now that we have our full vector of vectors, we can create an array2d from it
    let mut seating_chart = Array2D::from_rows(&array_rows);
    let mut number_of_changes = 1;

    // Now we have a 2D array that we can walk through and modify easily. Time to apply the rules to it.
    // We update the seating chart until there are no more new changes
    while number_of_changes != 0{
        // Apply the rules - for part 1, the number of occupied seats to make a seat go empty is 4.
        let (new_seating_chart, new_number_of_changes) = apply_rules(seating_chart, 4, 1);
        // Update the chart and the number of changes
        seating_chart = new_seating_chart;
        number_of_changes = new_number_of_changes;
    }
    
    // PART 1 - Count the number of occupied seats:
    println!("{} occupied seats in part 1", seating_chart.as_row_major().iter().filter(|occ| *occ == &1).count());

    // PART 2
    // Now that we have our full vector of vectors, we can create an array2d from it
    let mut seating_chart = Array2D::from_rows(&array_rows);
    let mut number_of_changes = 1;

    // Now we have a 2D array that we can walk through and modify easily. Time to apply the rules to it.
    // We update the seating chart until there are no more new changes
    while number_of_changes != 0{
        // Apply the rules - for part 1, the number of occupied seats to make a seat go empty is 4.
        let (new_seating_chart, new_number_of_changes) = apply_rules(seating_chart, 5, 2);
        // Update the chart and the number of changes
        seating_chart = new_seating_chart;
        number_of_changes = new_number_of_changes;
    }

    // PART 2 - Count the number of occupied seats:
    println!("{} occupied seats in part 2", seating_chart.as_row_major().iter().filter(|occ| *occ == &1).count());

}

fn apply_rules(pre_seating_chart: Array2D<u32>, number_to_switch: i32, part: i32) -> (Array2D<u32>, isize){
    // The rules are applied all at once, so we need to make a clone of the seating chart and apply the rules to the clone,
    // so we leave the initial one intact
    // RULES: An empty seat becomes occupied if there are NO occupied seats adjacent (all 8 directions)
    //        An occupied seat becomes empty if there are FOUR OR MORE seats adjacent to it that are also occupied.
    // We also need to keep track of how many changes occur in a round to know when the iterations have completed.pre_seating_chart
    
    let row_count = pre_seating_chart.num_rows();
    let column_count = pre_seating_chart.num_columns();
    let mut number_of_changes = 0;

    let mut modified_seating_chart = pre_seating_chart.clone();

    let mut seat_view_map: HashMap<(usize, usize), SeatView> = HashMap::new();

    // We have to step through each row and each column to check each seat
    for row in 0..row_count{
        for column in 0..column_count{
            // Get the value of the seat we're currently checking
            let current_seat = pre_seating_chart.get(row, column).unwrap_or(&3);
            // Cases
            match current_seat{
                // If the seat is empty, then we want to check if it can be filled
                0 => {
                    // Get the number of adjacent seats that are occupied
                    let mut adjacent_occupied_seats = 0;
                    if part == 1 {
                        adjacent_occupied_seats = part_1_check_neighbors(&pre_seating_chart, row, column);
                    }
                    else if part == 2{
                        let (occupied_seats, new_view) = part_2_check_neighbors(&pre_seating_chart, &seat_view_map, row, column);
                        adjacent_occupied_seats = occupied_seats;
                        seat_view_map.insert((row, column), new_view);
                        //seat_view_map.insert(row, column, )
                    }
                    // If there are no occupied seats adjacent, this seat becomes occupied.  Otherwise, nothing happens.
                    if adjacent_occupied_seats == 0{
                        // Modify the seating chart to have that seat be occupied
                        let success = modified_seating_chart.set(row, column, 1);
                        // Error handling
                        match success{
                            Err(_error) => println!("An error occurred at row {} column {}", row, column),
                            _ => (),
                        }
                        // Count this change
                        number_of_changes = number_of_changes + 1;
                    }
                }
                // If the seat is occupied, then we want to check if it ends up getting emptied
                1 => {
                    // Get the number of adjacent seats that are occupied
                    let mut adjacent_occupied_seats = 0;
                    if part == 1 {
                        adjacent_occupied_seats = part_1_check_neighbors(&pre_seating_chart, row, column);
                    }
                    else if part == 2{
                        let (occupied_seats, new_view) = part_2_check_neighbors(&pre_seating_chart, &seat_view_map, row, column);
                        adjacent_occupied_seats = occupied_seats;
                        seat_view_map.insert((row, column), new_view);
                    }
                    // If there is equal to or more than the limit of adjacent seats that are occupied, the seat empties 
                    // otherwise, no changes.
                    if adjacent_occupied_seats >= number_to_switch{
                        // Modify the seating chart to have that seat be occupied
                        let success = modified_seating_chart.set(row, column, 0);
                        // Error handling
                        match success{
                            Err(_error) => println!("An error occurred at row {} column {}", row, column),
                            _ => (),
                        }
                        // Count this change
                        number_of_changes = number_of_changes + 1;
                    }
                }
                // If it's a floor space, do nothing in part 1
                // In part 2, we should do neighbor checking to memoize the seat's view
                2 => {
                    if part == 2{
                        let (_occupied_seats, new_view) = part_2_check_neighbors(&pre_seating_chart, &seat_view_map, row, column);
                        seat_view_map.insert((row, column), new_view);
                    }
                },
                // Any other case is an error case
                _ => println!("An unexpected value was found at row {}, column {}: {}", row, column, current_seat),
            }
        }
    }

    // if part == 2{
    //     println!("{:#?}", seat_view_map);
    // }    
    return (modified_seating_chart, number_of_changes);
}

fn part_1_check_neighbors(seating_chart: &Array2D<u32>, row: usize, column: usize) -> i32{
    
    // Check NW
    let nw_neighbor = part_1_neighbor_check(seating_chart, row, column, 1, true, 1, true);
    
    // Check N
    let n_neighbor = part_1_neighbor_check(seating_chart, row, column, 1, true, 0, false);
    
    // Check NE
    let ne_neighbor = part_1_neighbor_check(seating_chart, row, column, 1, true, 1, false);
    
    // Check E
    let e_neighbor = part_1_neighbor_check(seating_chart, row, column, 0, false, 1, false);
    
    // Check SE
    let se_neighbor = part_1_neighbor_check(seating_chart, row, column, 1, false, 1, false);
    
    // Check S
    let s_neighbor = part_1_neighbor_check(seating_chart, row, column, 1, false, 0, false);
    
    // Check SW
    let sw_neighbor = part_1_neighbor_check(seating_chart, row, column, 1, false, 1, true);
    
    // Check W
    let w_neighbor = part_1_neighbor_check(seating_chart, row, column, 0, false, 1, true);
    
    let occupied_neighbor_count = nw_neighbor + n_neighbor + ne_neighbor + e_neighbor
                                 + se_neighbor + s_neighbor + sw_neighbor + w_neighbor;

    return occupied_neighbor_count;
}

fn part_1_neighbor_check(seating_chart: &Array2D<u32>, row:usize, column: usize, row_offset: usize, row_sub: bool,
    column_offset: usize, column_sub: bool) -> i32 {
    // Set up tracking variables 
    let mut neighbor_occupied = 0;
    let mut search_row = row;
    let mut search_column = column;

    // Don't bother doing anything if the offset is 0
    if row_offset != 0{
        // If the row is to be subtracted, we wrapping_sub the row by the offset - otherwise we add
        if row_sub == true {
            search_row = row.wrapping_sub(row_offset);
        }
        else{
            search_row = row.wrapping_add(row_offset);
        }
    }

    // Same for the column
    if column_offset != 0{
        if column_sub == true {
            search_column = column.wrapping_sub(column_offset);
        }
        else{
            search_column = column.wrapping_add(column_offset);
        }
    }

    // Check the value of the given neighbor.
    let neighbor = seating_chart.get(search_row, search_column);
    // If the Option returns None, then it went out of bounds and so the seat is considered empty - no change
    // If it returns Some and that Some == 1, then the seat is occupied.  Otherwise, no change
    match neighbor{
        None => neighbor_occupied = 0,
        Some(value) => if value == &1 { neighbor_occupied = 1; },
    }

    return neighbor_occupied;
}

fn part_2_check_neighbors(seating_chart: &Array2D<u32>, seat_view_map: &HashMap<(usize, usize), SeatView>,
    row: usize, column: usize) -> (i32, SeatView){
        
    // Check NW
    let nw_seat = part_2_neighbor_check(seating_chart, seat_view_map, row, column, 1, true, 1, true);
    
    // Check N
    let n_seat = part_2_neighbor_check(seating_chart, seat_view_map, row, column, 1, true, 0, false);

    // Check NE
    let ne_seat = part_2_neighbor_check(seating_chart, seat_view_map, row, column, 1, true, 1, false);

    // Check E
    let e_seat = part_2_neighbor_check(seating_chart, seat_view_map, row, column, 0, false, 1, false);

    // Check SE
    let se_seat = part_2_neighbor_check(seating_chart, seat_view_map, row, column, 1, false, 1, false);

    // Check S
    let s_seat = part_2_neighbor_check(seating_chart, seat_view_map, row, column, 1, false, 0, false);

    // Check SW
    let sw_seat = part_2_neighbor_check(seating_chart, seat_view_map, row, column, 1, false, 1, true);
    
    // Check W
    let w_seat = part_2_neighbor_check(seating_chart, seat_view_map, row, column, 0, false, 1, true);

    // Create a SeatView for this seat.
    let current_view = SeatView{
        nw_view: nw_seat,
        n_view: n_seat,
        ne_view: ne_seat,
        e_view: e_seat,
        se_view: se_seat,
        s_view: s_seat,
        sw_view: sw_seat,
        w_view: w_seat,
    };

    // Count up the occupied seats in view from this one
    let occupied_neighbor_count = nw_seat + n_seat + ne_seat + e_seat + se_seat + s_seat + sw_seat + w_seat;
    return (occupied_neighbor_count, current_view);
}

fn part_2_neighbor_check(seating_chart: &Array2D<u32>, seat_view_map: &HashMap<(usize, usize), SeatView>, 
    row:usize, column: usize, row_offset: usize, row_sub: bool, column_offset: usize, column_sub: bool) -> i32{
    // Set up tracking variables 
    let mut neighbor_occupied = 0;
    let mut search_row = row;
    let mut search_column = column;

    // Don't bother doing anything if the offset is 0
    if row_offset != 0{
        // If the row is to be subtracted, we wrapping_sub the row by the offset - otherwise we add
        if row_sub == true {
            search_row = row.wrapping_sub(row_offset);
        }
        else{
            search_row = row.wrapping_add(row_offset);
        }
    }

    // Same for the column
    if column_offset != 0{
        if column_sub == true {
            search_column = column.wrapping_sub(column_offset);
        }
        else{
            search_column = column.wrapping_add(column_offset);
        }
    }

    // Check the value of the given neighbor.
    let neighbor = seating_chart.get(search_row, search_column);
    // If the Option returns None, then it went out of bounds and so the seat is considered empty - no change
    // If it returns Some and that Some == 1, then the seat is occupied.  Otherwise, no change
    match neighbor{
        None => neighbor_occupied = 0,
        Some(value) => {
            // If the seat seen is empty, then we consider that direction not occupied
            if value == &0 { neighbor_occupied = 0; }
            // If the seat seen is occupied, then of course it's occupied
            if value == &1 { neighbor_occupied = 1; }
            // If the space seen is floor, then we look further in this direction for the next seat.
            if value == &2 { 
                // First check if we have a memoized value for that spot's view - if so, return that first.  If not, continue our search
                let neighbor_view = seat_view_map.get(&(search_row, search_column));
                match neighbor_view{
                    // If we don't get a memoized value for that spot, then recurse
                    None => neighbor_occupied = part_2_neighbor_check(seating_chart, seat_view_map, search_row, search_column, row_offset, row_sub, column_offset, column_sub),
                    // If we do have a memoized value, get the view for the direction we're currently searching
                    Some(view) => {
                        neighbor_occupied = get_view_for_offset(row_offset, row_sub, column_offset, column_sub, view);
                    }

                }
                
            }
        }
    }

    return neighbor_occupied;
}

fn get_view_for_offset(row_offset: usize, row_sub: bool, column_offset: usize, column_sub: bool, view: &SeatView) -> i32{
    // North/South-based views
    if row_offset == 1{
        // North-based views
        if row_sub == true{
            if column_offset == 1{
                // NW view
                if column_sub == true{
                    return view.nw_view;
                }
                // NE view
                else{
                    return view.ne_view
                }
            } 
            // N view
            else {
                return view.n_view;
            }
        }
        // South-based views
        else{
            if column_offset == 1{
                // SW view
                if column_sub == true{
                    return view.sw_view;
                }
                // SE view
                else{
                    return view.se_view
                }
            } 
            // S view
            else {
                return view.s_view;
            }
        }
    }
    // Same-row views
    else{
        // If it's on the same row, we know it has to be either column offset = 1 or -1, so just need to check column_sub
        // West view
        if column_sub == true{
            return view.w_view;
        }
        // East view
        else{
            return view.e_view;
        }
    }
}