# hotpot-db

<img width="500px" src="https://66.media.tumblr.com/dc1e0c3d4372dd7a763cb3abba5c07b4/tumblr_ogk0t7i51o1vj3zbeo1_500.gifv"/>

### The 🌶🌶🌶 hottest way to store data

hotpot-db is a spicy, incredibly easy to use, and delcious database system.

```bash
# COMING SOON!
# hotpot_db = "0.0.0"
```

## What in the pot?

1. schemaless
2. reliable (uses SQLite3)
3. embeddable
4. fast
5. JSON store
6. queryable JSON schemas 


```rust
use hotpot_db::*;
use serde::{Deserialize, Serialize};

// hotpot db can store any struct that is JSON serializable
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() -> Result<(), hotpot_db::Error> {

	// first lets make a new or read the local database
	// this will make "database.hpdb"
    let mut pot = HotPot::new();

    // lets make a new collection
    pot.create_collection("address_book")?;

    // leta add people with different names
    for name in vec!["David", "Ness", "Sakura"] {
        let person = Person {
            name: String::from(name),
            age: 26,
        };
        let json_to_store = serde_json::to_string(&person).unwrap();
        pot.add_object_to_collection("address_book", json_to_store)?;
        println!("Added New Person");
    }

    // query the data for only people in the address book with a specific name
    // Note: we never specified a schema but can query the key values within the JSON blob 
    let objs = pot.get_objects_from_collection_key_value("address_book", "name", "Sakura")?;
    for res in objs {
        let v: Person = serde_json::from_str(&res.data).unwrap();
        println!("{:#?}", v);
    }

    Ok(())
}
```