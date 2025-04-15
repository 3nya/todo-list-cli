use dialoguer::Input;

#[derive(Debug, Default)]
pub struct ListRunner {
    // game: Option<Hangman>,
    // verbose: bool
    name: Option<String>,
}


impl ListRunner {
    pub fn new(word: Option<String>) -> ListRunner {
        let mut output = ListRunner { name: word };

        output
    }
    pub fn run(&mut self) {
        loop {
            let input : String = Input::new()
            .with_prompt(">")
            .interact_text()?;

            // Split the command line input by spaces
            let args : Vec<&str> = input.trim().split(' ').collect();

            match args[0] {
                // put commands here
            }
        }
    }
}