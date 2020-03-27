use hotpot_db::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() -> Result<(), hotpot_db::Error> {
    let mut pot = HotPot::new(".");

    // lets make a new collection
    pot.create_collection("address_book")?;

    // well make a new item we want to store
    let person = Person {
        name: String::from("david holtz"),
        age: 26,
    };

    // we insert the object into the collection!
    pot.insert::<Person>("address_book", &person)?;

    // before we query we can add an index to speed things up
    pot.add_index_to_collection("address_book", "name", "naming_index")?;

    // finally we can query
    let query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Object)
        .key("name")
        .comparison("=")
        .string("david holtz")
        .finish();

    let results = pot.execute(query);
    println!("{:#?}", results);

    Ok(())
}
