use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use colored::Colorize;
use std::fs::File;
use rustc_serialize::json::Json;

struct Todo {
    map: HashMap<String, bool>,


}
impl Todo {
    // make new list instead of overwriting with previous
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
        // old code, db.txt read by line
        // let mut content = String::new();
        // f.read_to_string(&mut content)?;

        // let mut map = HashMap::new();

        // for line in content.lines() {
        //     let mut values = line.split('\t');
        //     let key = values.next().expect("No Key");
        //     let val = values.next().expect("No Value");
        //     // insert them into HashMap
        //     map.insert(String::from(key), bool::from_str(val).unwrap());
        // }
        // Ok(Todo { map })

        // old code, db read by hashmap
        // let map: HashMap<String, bool> = content
        //     .lines()
        //     .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //     .map(|v| (v[0], v[1]))
        //     .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        //     .collect();
    }
    
    // save to disk method
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;

        let _ = self.display();
        Ok(())
    }

    // writing todo.txt method
    fn display(self) -> Result<(), std::io::Error> {
        let mut file: File = File::open("db.json").expect("failed to open json");
        let mut data: String = String::new();
        let mut tododisplay: String = String::new();
        file.read_to_string(&mut data).expect("failed to read db");

        let map: HashMap<String, bool> = serde_json::from_str(&data).expect("Failed to parse JSON");

        for (k, v) in map {
            // todo display
            let record_dis: String;
            if v == true {
                record_dis = format!("☑\t{}\n", k);
            } else {
                record_dis = format!("◻\t{}\n", k);
            }
            tododisplay.push_str(&record_dis);
        }
        std::fs::write("todo.txt", tododisplay)
    }

    // functions 

    // insert item
    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    // remove item
    fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }

    fn check(&mut self, key: String) {
        if self.map.get(&key).unwrap().clone() == true {
            self.map.insert(key.clone(), false);
        } else {
            self.map.insert(key.clone(), true);
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    
    if action == "help" {
        println!("\n");
        println!("{}", "To-Do List Commands: \n".green().italic().bold());

        println!("     {}           {}", "help".cyan(), "prints all commands");
        println!("     {}     {}", "add <item>".cyan(), "add item to list");
        println!("     {}  {}", "remove <item>".cyan(), "removes item (if exists) from list");
        println!("     {}   {} \n", "check <item>".cyan(), "checks item (if exists) on list");

        return;
    }
    let item = std::env::args().nth(2).expect("Please specify an item");
    
    let mut todo = Todo::new().expect("");
    
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("saved todo"),
            Err(_why) => println!("error"),
        }
        // match todo.display() {
        //     Ok(_) => println!("saved display"),
        //     Err(_why) => println!("error"),
        // }
        return;
    } 
    if action == "remove" {
        todo.remove(item);
        match todo.save() {
            Ok(_) => println!("saved todo"),
            Err(_why) => println!("error"),
        }
        return;
    }
    if action == "check" {
        todo.check(item);
        match todo.save() {
            Ok(_) => println!("saved todo"),
            Err(_why) => println!("error"),
        }
        return;
    }
    


}