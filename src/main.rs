
use std::io;


/**********************
 * Get Board
 * Asks the user to pick a map and then returns
 * it in a vector. 
 **********************/
fn get_board() -> Vec<String> {

    // Setting up variable 
    let mut vec = Vec::new();
    let mut user_input_board = String::new();


    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nPlease Enter a map number 1-4: ");

    // Getting input
    user_input_board = String::from("");
    io::stdin().read_line(&mut user_input_board);

    if user_input_board.trim() == "1"
    {
        vec.push("\n-----------".to_owned());
        vec.push("|8  |   |".to_owned());
        vec.push("|   | | |".to_owned());
        vec.push("| --| | |".to_owned());
        vec.push("|     | (".to_owned());
        vec.push("-----------\n".to_owned());
    }
    if user_input_board.trim() == "2"
    {
        vec.push("\n------------".to_owned());
        vec.push("|8  |     |".to_owned());
        vec.push("| | ----- |".to_owned());
        vec.push("| |     | |".to_owned());
        vec.push("| |---- | |".to_owned());
        vec.push("|   |     |".to_owned());
        vec.push("| | ----- |".to_owned());
        vec.push("| |   |   |".to_owned());
        vec.push("| |-- | --|".to_owned());
        vec.push("| |   |   (".to_owned());
        vec.push("-------------\n".to_owned());
    }
    if user_input_board.trim() == "3"
    {
        vec.push("\n----------------------------------------".to_owned());
        vec.push("|8    |       |   |             |       |".to_owned());
        vec.push("|-- | | ----- | | | --- ------- ------- |".to_owned());
        vec.push("|   |   |   |   | |   |       |     |   |".to_owned());
        vec.push("| | ----- | ----- |-- ------- |---- | --|".to_owned());
        vec.push("| | |     |   |   |       |   |   | |   |".to_owned());
        vec.push("| |-| --------| ----------- ----- | |-- |".to_owned());
        vec.push("| |   | |     |     |   |   |     | |   |".to_owned());
        vec.push("| | --- | --- |---- | | | --- ----| | | |".to_owned());
        vec.push("|   |       |         |   |           | (".to_owned());
        vec.push("-------------------------------------------\n".to_owned());
    }
    if user_input_board.trim() == "4"
    {
        vec.push("\n-----------------------------------------------------------------------------------------------------".to_owned());
        vec.push("|8                                                                                                  (".to_owned());
        vec.push("-----------------------------------------------------------------------------------------------------\n".to_owned());
    }

    // Returns the maps
    return vec;
}


fn main() {
    // Setting up variable 
    let mut user_input = String::new();
    let mut vec = get_board();
    let mut column = 1;
    let mut row = 1;
    let mut is_game_done = false;

    // Gameplay Loop
    while is_game_done == false
    {
        // Setting up local variables 
        let mut update_line = String::new();
        let mut check_line = String::new();

        // Adding player into the Vector
        update_line = vec[row].to_owned();
        update_line.replace_range(column..(column + 1),"x");
        vec[row] = update_line;

        // Print board
        println!("\n\n\n\n\n\n\n\n\n");
        for line in &vec
        {
            println!("{line}");
        }

        println!("Move!");
        
        // Get user's input
        user_input = String::from("");
        io::stdin().read_line(&mut user_input);

        // Checking input
        // Quit
        if user_input.trim() == "q"
        {
            println!("\nGoodBye");
            is_game_done = true;
        }
        // Move Right
        if user_input.trim() == "d"
        {
            // Check to make sure you aren't running into anything
            check_line = vec[row].to_owned();
            let b: u8 = check_line.as_bytes()[column + 1];
            let c: char = b as char;
            if c == ' '
            {
                // Move player one spot and cover old spot
                update_line = vec[row].to_owned();
                update_line.replace_range(column..(column + 1)," ");
                vec[row] = update_line;
                column += 1; 
            }
            // Checking player is running into the goal.
            else if c == '('
            {
                println!("You Won!");
                is_game_done = true;
            }
        }
        // Move Right
        if user_input.trim() == "a"
        {
            // Check to make sure you aren't running into anything
            check_line = vec[row].to_owned();
            let b: u8 = check_line.as_bytes()[column - 1];
            let c: char = b as char;
            if c == ' '
            {
                // Move player one spot and cover old spot
                update_line = vec[row].to_owned();
                update_line.replace_range(column..(column + 1)," ");
                vec[row] = update_line;
                column -= 1; 
            }
        }
        // Move Down
        if user_input.trim() == "s"
        {
            // Check to make sure you aren't running into anything
            check_line = vec[row + 1].to_owned();
            let b: u8 = check_line.as_bytes()[column];
            let c: char = b as char;
            if c == ' '
            {
                // Move player one spot and cover old spot
                update_line = vec[row].to_owned();
                update_line.replace_range(column..(column + 1)," ");
                vec[row] = update_line;
                row += 1; 
            }
        }
        // Move Up
        if user_input.trim() == "w"
        {
            // Check to make sure you aren't running into anything
            check_line = vec[row - 1].to_owned();
            let b: u8 = check_line.as_bytes()[column];
            let c: char = b as char;
            if c == ' '
            {
                // Move player one spot and cover old spot
                update_line = vec[row].to_owned();
                update_line.replace_range(column..(column + 1)," ");
                vec[row] = update_line;
                row -= 1; 
            }
        }
    }
}
