use hotpot_db::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct Grade {
    assignment: String,
    score: f64,
}

#[derive(Serialize, Deserialize)]
struct NestedObjectUpper {
    title: String,
    inner: NestedObjectLower,
}

#[derive(Serialize, Deserialize)]
struct NestedObjectLower {
    name: String,
}

fn main() {
    let mut pot = HotPot::new();

    // // This creates a new collection
    // {
    //     let _did_create_collection = pot.create_collection("dvds");
    // }

    // // lets add a new object
    // {
    //     let person = Person {
    //         name: String::from("Sakura"),
    //         age: 27,
    //     };
    //     let json_to_store = serde_json::to_string(&person).unwrap();
    //     let _did_add_object = pot.add_object_to_collection("dvds", json_to_store);
    // }

    // // lets add a new object
    // {
    //     let grade = Grade {
    //         assignment: String::from("First Test"),
    //         score: 432532.532532,
    //     };
    //     let json_to_store = serde_json::to_string(&grade).unwrap();
    //     let _did_add_object = pot.add_object_to_collection("dvds", json_to_store);
    // }

    // lets add a new object
    {
        let nested = NestedObjectUpper {
            title: String::from("First Test"),
            inner: NestedObjectLower {
                name: String::from("drbh"),
            },
        };
        let json_to_store = serde_json::to_string(&nested).unwrap();
        let _did_add_object = pot.add_object_to_collection("dvds", json_to_store);
    }

    // // lets add a non struct object too
    // {
    //     let random_data = vec![String::from("cheese"), String::from("art")];
    //     let new_json_to_store = json!(random_data).to_string();
    //     let _did_add_object = pot.add_object_to_collection("dvds", new_json_to_store);
    // }

    // {
    //     let objs = pot.get_objects_from_collection_containing("dvds", "cheese");
    //     println!("{:#?}", objs);
    // }

    // {
    //     let objs = pot.get_objects_from_collection_key_value("dvds", "name", "Sakura");
    //     println!("{:#?}", objs);
    // }

    // another one
    {
        let objs = pot.get_objects_from_collection_key_value("dvds", "inner.name", "drbh");
        println!("{:#?}", objs);
    }

    // let mut spicy_pot = HotPot::new();
    // {
    //     let objs = spicy_pot.get_objects_from_collection_containing("dvds", "cheese");
    //     println!("{:#?}", objs);
    // }

    println!("{:#?}", pot);
    // println!("{:#?}", spicy_pot);
}
