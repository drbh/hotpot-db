use hotpot_db::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() -> Result<(), hotpot_db::Error> {
    let mut pot = HotPot::new();

    // lets make a new collection
    pot.create_collection("address_book")?;

    // let add people
    for name in vec!["David", "Ness", "Sakura"] {
        let person = Person {
            name: String::from(name),
            age: 26,
        };
        let json_to_store = serde_json::to_string(&person).unwrap();
        pot.add_object_to_collection("address_book", json_to_store)?;
        println!("Added New Person");
    }

    // query the data
    let objs = pot.get_objects_from_collection_key_value("address_book", "name", "Sakura")?;
    for res in objs {
        let v: Person = serde_json::from_str(&res.data).unwrap();
        println!("{:#?}", v);
    }

    Ok(())
}
