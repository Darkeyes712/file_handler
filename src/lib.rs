use std::fs::File;
use std::io::{self, Write};

pub fn print_to_terminal(prompt: &str) {
    
    println!("{}", prompt);
    std::io::stdout().flush().unwrap();
}

pub fn get_user_input(user_input: &str) -> Result<String, std::io::Error> {

    print_to_terminal(user_input);
    let mut usr_inp = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut usr_inp)?;
    Ok(usr_inp.trim().to_string())
}

pub fn create_text_file() {

    if let Ok(file_name) = get_user_input("What's the name of the file?: ") {
        println!("{}", file_name);
        let mut file = match File::create(&file_name) {
            Ok(file) => file,
            Err(error) => panic!("Failed to create file: {}", error),
        };
        // Use the `file` variable to perform file operations as needed
        if let Ok(file_data) = get_user_input("Write something to go in the file: ") {
            match file.write_all(file_data.as_bytes()) {
                Ok(_) => println!("Data has been written to the file."),
                Err(error) => panic!("Failed to write data to the file: {}", error),
            }
        }
    } else {
        println!("Failed to read user input.");
    }
}

