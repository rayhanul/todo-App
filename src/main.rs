
use std::collections::HashMap;
use std::fmt::Error;
use std::io::Read;
use std::str::FromStr;

struct Todo{
    map:HashMap<String, bool>
}

impl Todo {

    fn new() -> Result<Todo, std::io::Error>{

        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

            let mut content = String::new();
            f.read_to_string(&mut content)?;

            // let map: HashMap<String, bool> = content
            // .lines()
            // .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            // .map(|v| (v[0], v[1]))
            // .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            // .collect();
            let mut map = HashMap::new();

            for entries in content.lines() {
                // split and bind values
                let mut values = entries.split('\t');
                let key = values.next().expect("No Key");
                let val = values.next().expect("No Value");
                // insert them into HashMap
                map.insert(String::from(key), bool::from_str(val).unwrap());
            }

        Ok(Todo { map }) 
    }

    fn insert(&mut self, key:String){
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error>{

        let mut content=String::new();

        for (k, v) in self.map{
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    // println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action=="add" {
        todo.insert(item);

        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
}
