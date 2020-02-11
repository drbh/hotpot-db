use hotpot_db::*;

fn main() {
    let pot = HotPot::new();

    let mut query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Contains)
        .string("cheese")
        .finish();

    let results = pot.execute(query);
    println!("{:#?}", results);

    query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Object)
        .key("name")
        .string("David")
        .finish();

    let results = pot.execute(query);
    println!("{:#?}", results);

    query = QueryBuilder::new()
        .collection("address_book")
        .kind(QueryKind::Object)
        .key("age")
        .int(26)
        .finish();

    let results = pot.execute(query);
    println!("{:#?}", results);
}
