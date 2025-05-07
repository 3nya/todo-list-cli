use std::collections::HashMap;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    map: HashMap<String, bool>,
    header: String,
    footer: String,
    display_count: i16,
    count: i16,
}

impl Todo {
    fn new() -> Result<Todo, Box<dyn std::error::Error>> {
        if let Ok(data) = fs::read_to_string("db.json") {
            serde_json::from_str(&data).map_err(|e| e.into())
        } else {
            Ok(Todo {
                map: HashMap::new(),
                header: String::new(),
                footer: String::new(),
                display_count: 0,
                count: 0,
            })
        }
    }
    
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write("db.json", data)?;
        self.display()
    }

    fn display(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut tododisplay: String = String::new();

        if self.display_count == 1 {
            tododisplay.push_str(&format!("{} {}/{}\n", 
                self.header, 
                self.count, 
                self.map.len()
            ));
        } else {
            tododisplay.push_str(&format!("{}\n", self.header));
        }

        for (k, v) in &self.map {
            let record_dis = if *v {
                format!("☑\t{}\n", k)
            } else {
                format!("◻\t{}\n", k)
            };
            tododisplay.push_str(&record_dis);
        }

        if self.display_count == 2 {
            tododisplay.push_str(&format!("{} {}/{}\n", 
                self.footer, 
                self.count, 
                self.map.len()
            ));
        } else {
            tododisplay.push_str(&format!("{}\n", self.footer));
        }

        fs::write("todo.txt", tododisplay)?;
        Ok(())
    }

    fn print(&self) {
        // let mut result: String = String::new();
        for line in read_to_string("todo.txt").unwrap().lines() {
            println!("{}", line);
            // result.push_str(line)
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    fn remove(&mut self, key: String) {
        if let Some(true) = self.map.get(&key) {
            self.count -= 1;
        }
        self.map.remove(&key);
    }

    fn check(&mut self, key: String) {
        if let Some(current) = self.map.get_mut(&key) {
            *current = !*current;
            if *current {
                self.count += 1;
            } else {
                self.count -= 1;
            }
        } else {
            println!("Item does not exist in list");
        }
    }
}

fn main() {
    let mut todo = match Todo::new() {
        Ok(t) => t,
        Err(e) => {
            println!("Failed to initialize todo list: {}", e);
            return;
        }
    };

    loop {
        println!("\nEnter command (help for commands):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let mut args = input.trim().split_whitespace();
        let action = match args.next() {
            Some(a) => a,
            None => continue,
        };
        
        if action == "quit" {
            break;
        }
        
        if action == "help" {
            print_help();
            continue;
        }

        if action == "display" {
            if let Err(e) = todo.display() {
                println!("Display error: {}", e);
            }
            todo.print();
            continue;
        }
        
        let item = args.collect::<Vec<_>>().join(" ");
        if item.is_empty() {
            println!("Specify item for this command");
            continue;
        }
        
        match action {
            "add" => todo.insert(item),
            "remove" => todo.remove(item),
            "check" => todo.check(item),
            "header" => todo.header = item,
            "footer" => todo.footer = item,
            "count" => {
                if item == "h" {
                    todo.display_count = 1;
                } else if item == "f" {
                    todo.display_count = 2;
                } else if item == "r" {
                    todo.display_count = 0;
                }
            },
            _ => {
                println!("Unknown Command: Type help for list of commands.");
                continue;
            },
        }
        
        if let Err(e) = todo.save() {
            println!("Error saving: {}", e);
        }
    }
}

fn print_help() {
    println!("\n{}", "To-Do List Commands: \n".green().italic().bold());
    println!("     {}           {}", "help".cyan(), "prints all commands");
    println!("     {}     {}", "add <item>".cyan(), "add item to list");
    println!("     {}  {}", "remove <item>".cyan(), "removes item (if exists) from list");
    println!("     {}   {} ", "check <item>".cyan(), "checks item (if exists) on list");
    println!("     {}  {}", "header <item>".cyan(), "adds text as header");
    println!("     {}  {}", "footer <item>".cyan(), "adds text as footer");
    println!("     {}        {}", "display".cyan(), "show current todo list");
    println!("     {}           {}\n", "quit".cyan(), "exit the program");
}