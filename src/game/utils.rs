pub fn get_integer() -> i32 {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        match input.parse() {
            Ok(value) => break value,
            Err(_) => {
                eprintln!("Failed to parse value! Please try again.")
            }
        }
    }
}
