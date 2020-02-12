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

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Serialize, Deserialize)]
struct Grade {
    assignment: String,
    score: f32,
    person: Person,
}

fn main() -> Result<(), hotpot_db::Error> {
    let mut pot = HotPot::new();

    // lets make a new collection
    pot.create_collection("address_book")?;

    // we can add complex structs that are nested
    let grade = Grade {
        assignment: String::from("First Test"),
        score: 90.25,
        person: Person {
            name: String::from("drbh"),
            age: 101,
        },
    };
    let json_to_store = serde_json::to_string(&grade).unwrap();
    pot.add_object_to_collection("address_book", json_to_store)?;

    // well add a second one
    let grade = Grade {
        assignment: String::from("First Test"),
        score: 290.25,
        person: Person {
            name: String::from("drbh"),
            age: 101,
        },
    };
    let json_to_store = serde_json::to_string(&grade).unwrap();
    pot.add_object_to_collection("address_book", json_to_store)?;

    // this queries for scores that are above a specific value
    let query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Object)
        .key("score")
        .comparison(">=")
        .float(90.25)
        .finish();

    Ok(())
}

```