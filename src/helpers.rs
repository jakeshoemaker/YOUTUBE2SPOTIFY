pub mod helpers {
    use std::io;
    
    pub fn get_user_input(prompt: String) -> String {
        let mut input = String::new();
        println!("{}", &prompt);
        io::stdin().read_line(&mut input).expect("error: unable to read input");
        return input;
    }
}
