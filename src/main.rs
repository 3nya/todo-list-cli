use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>,


}
impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    
    println!("{:?}, {:?}", action, item);
    


}