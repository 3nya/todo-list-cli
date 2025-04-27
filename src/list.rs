pub struct TodoItem {
    pub id: u32,
    pub name: String,
    pub checked: bool,
    pub tags: Vec<String>,
}


pub struct List {
    pub name: String,
    pub items: std::collections::HashMap<string, bool>,
    pub next_id: u32,
}

impl List {

    //constructor
    pub fn new(name: String) -> Self {
        return List {
            name,
            items: HashMap::new(),
            next_id: 1,
        }
    }



    //adds an item to the map and returns the id of that item
    pub fn add_item(&mut self, name: String) -> u32 {
        let id = self.next_id;
        self.items.insert(id, TodoItem {
            id,
            name,
            checked: false,
            tags: new Vec<String>::new(),
        })
        self.next_id += 1;
        return id;
    }


    //removes the item by the id and returns true or false based on if it worked or not
    pub fn remove_item(&mut self, id: u32) -> bool {
        self.items.remove(&id).is_some()
    }

    //checks or unchecks an item and returns true or false based on if it worked or not
    pub fn toggle_item(&mut self, id: u32) -> bool {
        match self.items.get_mut(&id) {
            Some(item) => {
                item.checked = !item.checked;
                return true;
            }
            None => return false;
        }
    }

    //displays the to do list
    pub fn display(&self) {
        println!("To-Do List: {}", self.name);
        println!("-------------------");
        for (_, item) in &self.items {
            let status = if item.checked { "[x]" } else { "[ ]" };
            println!("{} {} (ID: {})", status, item.name, item.id);
            if !item.labels.is_empty() {
                println!("   Tags: {}", item.labels.join(", "));
            }
        }
    }
}
