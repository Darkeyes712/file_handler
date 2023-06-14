use::file_handler::{create_text_file, get_user_input};


fn main() {
    let usr_input = get_user_input("To create a .txt file, input txt else it will blow up");
    println!("{:?}", usr_input);

    match usr_input {
        Ok(input) => {
            if input == "txt" {
                create_text_file();
            } else {
                println!("Invalid input.");
            }
        }
        Err(error) => {
            println!("Failed to read user input: {}", error);
        }
    }
}