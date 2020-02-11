use hotpot_db::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Sakura"),
        age: 27,
    };

    let mut pot = HotPot::new();
    println!("{:#?}", pot);

    let d = pot.create_collection("dvds");
    println!("{:#?}", d);

    let json_to_store = serde_json::to_string(&person).unwrap();
    let y = pot.add_object_to_collection("dvds", json_to_store);
    println!("{:?}", y);

    let new_json_to_store = json!(vec![String::from("cheese"), String::from("art")]);
    let z = pot.add_object_to_collection("dvds", new_json_to_store.to_string());
    println!("{:?}", z);

    println!("{:#?}", pot);
}
