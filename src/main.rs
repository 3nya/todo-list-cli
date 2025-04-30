use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use colored::Colorize;

struct Todo {
    map: HashMap<String, bool>,


}
impl Todo {
    // make new list instead of overwriting with previous
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let mut map = HashMap::new();

        for line in content.lines() {
            let mut values = line.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No Value");
            // insert them into HashMap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }
        // let map: HashMap<String, bool> = content
        //     .lines()
        //     .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //     .map(|v| (v[0], v[1]))
        //     .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        //     .collect();
        Ok(Todo { map })
    }
    
    // save to disk method
    fn save(self) -> Result<(), std::io::Error> {
        let mut content: String = String::new();
        let mut tododisplay: String = String::new();
        for (k, v) in self.map {
            // db
            let record: String = format!("{}\t{}\n", k , v);
            content.push_str(&record);

            // todo display
            let record_dis: String;
            if v == true {
                record_dis = format!("☑\t{}\n", k);
            } else {
                record_dis = format!("◻\t{}\n", k);
            }
            tododisplay.push_str(&record_dis);
        }
        let _ = std::fs::write("db.txt", content);
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