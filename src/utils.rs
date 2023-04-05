use std::path;

pub fn get_input(prompt: String) -> String {
    println!("{prompt}");
    let mut input = String::new();
    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                break;
            }
            Err(e) => {
                println!("Unable to read input. Please try again.")
            }
        };
    }
    input.trim().to_string()
}

pub fn file_exists(path: &str) -> bool {
    let path = path::Path::new(path);
    path.exists()
}
