# hotpot-db

[![crates.io](https://meritbadge.herokuapp.com/hotpot-db)](https://crates.io/crates/hotpot-db)

_Warning: API is not finished and may be subject to change. New features and documentation will be added before 1.0 stable release_

<img width="500px" src="https://66.media.tumblr.com/dc1e0c3d4372dd7a763cb3abba5c07b4/tumblr_ogk0t7i51o1vj3zbeo1_500.gifv"/>

### The 🌶🌶🌶 hottest way to store data

hotpot-db is a spicy, incredibly easy to use, and delcious database system.

```bash
hotpot_db = "0.0.1"
```

## Flavor Palette

1. schemaless
2. reliable (uses SQLite3)
3. embeddable
4. fast (<200ms to search through +500K objects)
5. JSON store
6. queryable JSON schemas 


## Example
```rust
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
```

## Recipe

hotpot-db is made from few, but time tasted ingredients. It is a new approach on an old dish. 

**Ingredeients**
1. 1 cup, `SQLite 3.30.1`
2. 2 tablespoons, `Rust`
3. A pinch, `JSON serde`

## Concepts

#### Collection  
In a technical sense a collection is just a table in SQLite, that stores data in a specific format. Each row is an `Entry` which consists of three columns: id, time_created, data. The data column holds each JSON object and the other columns are used as hotpot-db metadata.  

In theory a collection should house similar data to make it easier to manage, but hotpt-db doesnt care about schema so you can store any kind of object in a single collection.

#### Objects

Each entry contains an object and the are the heart of hotpot-db. Objects are special because you can query their conents effeicently. 

This is an advantage over storing JSON in other datastores since you don't have to read the full object to query the contents. hotpot-db wraps SQLite's json1 extension into an easy to use API. 


## Speed Estimates

Objects allow us to store schemaless data and still search through it efficently. Query's on small dbs ~10MB run in <5ms and tested queires on larger DB's ~100MB run <500ms.

## Query Kinds

In a hot pot you can only query in two different ways. You can check the contents of an array or the attribute/values of an object.

hotpot-db offers the developer a simple QueryBuilder that allow you to conviently write and read your queries. 

#### Querying Arrays
```rust
let query = QueryBuilder::new()
    .collection("transaction_records")
    .kind(QueryKind::Contains)
    .comparison(">")
    .int(100)
    .finish();
```

#### Querying Objects
```rust
let query = QueryBuilder::new()
    .collection("address_book")
    .kind(QueryKind::Object)
    .key("name")
    .comparison("=")
    .string("david holtz")
    .finish();
```
