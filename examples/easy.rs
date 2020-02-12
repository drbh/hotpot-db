use hotpot_db::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

    // let add people
    for name in vec!["david", "ness", "sakura"] {
        let person = Person {
            name: String::from(name),
            age: 26,
        };
        let json_to_store = serde_json::to_string(&person).unwrap();
        pot.add_object_to_collection("address_book", json_to_store)?;
    }

    // we can even more complex structs that are nested
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

    // we can even more complex structs that are nested
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

    // here we can add arbitrary structs as long as they can be JSON serialized
    let random_data = vec![String::from("cheese"), String::from("art")];
    let new_json_to_store = json!(random_data).to_string();
    pot.add_object_to_collection("address_book", new_json_to_store)?;

    // lets query our hotpot!

    // this queries for ages that are a specific int value
    let query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Object)
        .key("name")
        .comparison("=")
        .string("david")
        .finish();

    let results = pot.execute(query)?;
    println!("{:#?}", results);

    // this queries for ages that are a specific int value
    let query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Object)
        .key("age")
        .comparison("=")
        .int(26)
        .finish();

    let results = pot.execute(query)?;
    println!("{:#?}", results);

    // this queries for scores that are a specific float value
    let query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Object)
        .key("score")
        .comparison(">=")
        .float(90.25)
        .finish();

    let results = pot.execute(query)?;
    println!("{:#?}", results);

    // this queries arrays that contain a string value
    let query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Contains)
        .comparison("=")
        .string("cheese")
        .finish();

    let results = pot.execute(query)?;
    println!("{:#?}", results);

    // we can also do nested queries
    let query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Object)
        .key("person.age")
        .comparison("=")
        .int(101)
        .finish();

    let results = pot.execute(query)?;
    println!("{:#?}", results);

    Ok(())
}
