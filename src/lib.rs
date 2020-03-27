use rusqlite::{params, Connection, NO_PARAMS};
use serde::Serialize;
use serde_json::json;
use std::any::type_name;
use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct HotPot {
    pub conn: Connection,
    pub collections: HashMap<String, Collection>,
}

#[derive(Debug, Clone)]
pub struct Collection {
    pub name: String,
}

#[derive(Debug)]
pub struct NewEntry {
    time_created: i64,
    data: String,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: i64,
    pub time_created: i64,
    pub data: String,
}

#[derive(Debug)]
pub enum Error {
    General,
}

#[derive(Debug, Clone)]
pub enum QueryKind {
    Contains,
    Object,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub query_type: QueryKind,
    pub collection: String,
    pub comparison: String,
    pub key: Option<String>,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Float(f32),
    Integer(i32),
    String(String),
}

#[derive(Debug)]
pub struct QueryBuilder {
    pub query_type: Option<QueryKind>,
    pub collection: Option<String>,
    pub comparison: Option<String>,
    pub key: Option<String>,
    pub value: Option<Value>,
}

#[derive(Debug)]
pub struct Index {
    pub collection: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct IndexBuilder {
    pub collection: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
}

fn get_ms_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as i64
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder {
            query_type: None,
            collection: None,
            comparison: None,
            key: None,
            value: None,
        }
    }

    pub fn kind(mut self, typ: QueryKind) -> QueryBuilder {
        self.query_type = Some(typ);
        self
    }

    pub fn collection(mut self, collection: &str) -> QueryBuilder {
        self.collection = Some(String::from(collection));
        self
    }

    pub fn comparison(mut self, comparison: &str) -> QueryBuilder {
        self.comparison = Some(String::from(comparison));
        self
    }

    pub fn string(mut self, value: &str) -> QueryBuilder {
        self.value = Some(Value::String(String::from(value)));
        self
    }

    pub fn bool(mut self, value: bool) -> QueryBuilder {
        self.value = Some(Value::Boolean(value));
        self
    }

    pub fn float(mut self, value: f32) -> QueryBuilder {
        self.value = Some(Value::Float(value));
        self
    }

    pub fn int(mut self, value: i32) -> QueryBuilder {
        self.value = Some(Value::Integer(value));
        self
    }

    pub fn key(mut self, key: &str) -> QueryBuilder {
        self.key = Some(String::from(key));
        self
    }

    pub fn finish(self) -> Query {
        Query {
            query_type: self.query_type.unwrap(),
            collection: self.collection.unwrap(),
            comparison: self.comparison.unwrap(),
            key: self.key,
            value: self.value.unwrap(),
        }
    }
}

impl HotPot {
    pub fn new<P: AsRef<Path>>(path: P) -> HotPot {
        let path = path.as_ref();
        if !path.exists() {
            panic!(format!("The path {:?} does not exist!", path))
        }
        let mut hp = HotPot {
            conn: Connection::open(path.join("database.hpdb")).unwrap(),
            collections: HashMap::new(),
        };
        let collections = hp.list_collections();
        match collections {
            Ok(collection_names) => {
                for name in collection_names {
                    hp.collections.insert(
                        String::from(name.clone()),
                        Collection {
                            name: String::from(name),
                        },
                    );
                }
            }
            Err(_) => (),
        }
        hp
    }
    pub fn close(self) {
        self.conn.close();
    }

    pub fn list_collections(&mut self) -> rusqlite::Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT name FROM sqlite_master WHERE type ='table' AND name NOT LIKE 'sqlite_%'",
        )?;
        let rows = stmt.query_map(NO_PARAMS, |row| Ok(row.get(0)?))?;
        let mut names = Vec::new();
        for name_result in rows {
            names.push(name_result?);
        }
        Ok(names)
    }

    pub fn create_collection(&mut self, name: &str) -> Result<bool, Error> {
        &self
            .conn
            .execute(
                &format!(
                    "
        CREATE TABLE {} (
            id              INTEGER PRIMARY KEY,
            time_created    INTEGER NOT NULL,
            data            BLOB
        )",
                    name
                ),
                params![],
            )
            .map_err(|_| Error::General);
        &self.collections.insert(
            String::from(name),
            Collection {
                name: String::from(name),
            },
        );
        Ok(true)
    }

    pub fn execute(&self, query: Query) -> Result<Vec<Entry>, Error> {
        let mut results = Vec::new();
        let c = &self
            .collections
            .get(&query.collection)
            .expect("collection does not exist");
        results = match query.query_type {
            QueryKind::Contains => match query.value {
                Value::String(val) => c
                    .query_arrays_contains::<String>(
                        &self.conn,
                        &query.collection,
                        &query.comparison,
                        &val,
                    )
                    .unwrap_or(Vec::new()),
                Value::Boolean(val) => c
                    .query_arrays_contains::<bool>(
                        &self.conn,
                        &query.collection,
                        &query.comparison,
                        &val,
                    )
                    .unwrap_or(Vec::new()),
                Value::Float(val) => c
                    .query_arrays_contains::<f32>(
                        &self.conn,
                        &query.collection,
                        &query.comparison,
                        &val,
                    )
                    .unwrap_or(Vec::new()),
                Value::Integer(val) => c
                    .query_arrays_contains::<i32>(
                        &self.conn,
                        &query.collection,
                        &query.comparison,
                        &val,
                    )
                    .unwrap_or(Vec::new()),
            },
            QueryKind::Object => match query.value {
                Value::String(val) => c
                    .query_object_with_key_value::<String>(
                        &self.conn,
                        &query.collection,
                        &query.comparison,
                        &query.key.clone().unwrap(),
                        &val,
                    )
                    .unwrap_or(Vec::new()),
                Value::Boolean(val) => c
                    .query_object_with_key_value::<bool>(
                        &self.conn,
                        &query.collection,
                        &query.comparison,
                        &query.key.clone().unwrap(),
                        &val,
                    )
                    .unwrap_or(Vec::new()),
                Value::Float(val) => c
                    .query_object_with_key_value::<f32>(
                        &self.conn,
                        &query.collection,
                        &query.comparison,
                        &query.key.clone().unwrap(),
                        &val,
                    )
                    .unwrap_or(Vec::new()),
                Value::Integer(val) => c
                    .query_object_with_key_value::<i32>(
                        &self.conn,
                        &query.collection,
                        &query.comparison,
                        &query.key.clone().unwrap(),
                        &val,
                    )
                    .unwrap_or(Vec::new()),
            },
        };
        Ok(results)
    }

    pub fn add_object_to_collection(&mut self, cname: &str, val: String) -> Result<bool, Error> {
        let c = &self.collections.get(cname).unwrap();
        let _did_insert = c.add_object(&self.conn, cname, val);
        Ok(true)
    }
    pub fn add_index_to_collection(
        &mut self,
        collection_name: &str,
        key: &str,
        index_name: &str,
    ) -> Result<bool, Error> {
        let c = &self.collections.get(collection_name).unwrap();
        let _did_insert = c.add_index(&self.conn, collection_name, key, index_name);
        Ok(true)
    }

    pub fn insert<T: Serialize>(&mut self, cname: &str, svalue: &T) -> Result<bool, Error> {
        // let json_to_store = serde_json::to_string(&person).unwrap();
        let val: String = json!(svalue).to_string();

        let c = &self.collections.get(cname).unwrap();
        let _did_insert = c.add_object(&self.conn, cname, val);
        Ok(true)
    }

    pub fn upsert_at_index<T: Serialize>(
        &mut self,
        cname: &str,
        index: usize,
        svalue: &T,
    ) -> Result<bool, Error> {
        // let json_to_store = serde_json::to_string(&person).unwrap();
        let val: String = json!(svalue).to_string();

        let c = &self.collections.get(cname).unwrap();
        let _did_insert = c.add_object_at_index(&self.conn, cname, index, val);
        Ok(true)
    }
}

impl Collection {
    pub fn add_object(
        &self,
        conn: &Connection,
        cname: &str,
        value: String,
    ) -> rusqlite::Result<()> {
        let me = NewEntry {
            time_created: get_ms_time(),
            data: value,
        };
        conn.execute(
            &format!(
                "INSERT INTO {} (time_created, data)
                  VALUES (?1, ?2)",
                cname
            ),
            params![me.time_created, me.data.to_string()],
        )?;
        Ok(())
    }
    pub fn add_object_at_index(
        &self,
        conn: &Connection,
        cname: &str,
        hotpot_id_for_entry: usize,
        value: String,
    ) -> rusqlite::Result<()> {
        let me = NewEntry {
            time_created: get_ms_time(),
            data: value,
        };
        conn.execute(
            &format!(
                "INSERT OR REPLACE INTO {} (id, time_created, data)
                  VALUES ({}, ?1, ?2)",
                cname, hotpot_id_for_entry
            ),
            params![me.time_created, me.data.to_string()],
        )?;
        Ok(())
    }

    // --DROP INDEX wr_country
    // --CREATE INDEX wr_country ON wine_reviews (json_extract(data, '$.country'))
    // --EXPLAIN QUERY PLAN SELECT * FROM wine_reviews WHERE json_extract(data, '$.country') = 'Italy'
    pub fn add_index(
        &self,
        conn: &Connection,
        collection_name: &str,
        key: &str,
        index_name: &str,
    ) -> rusqlite::Result<()> {
        let mut stmt = conn.prepare(&format!(
            "CREATE INDEX {} ON {} (json_extract(data, '$.{}'))",
            index_name, collection_name, key
        ))?;
        let _res = stmt.execute(params![])?;
        Ok(())
    }

    pub fn query_arrays_contains<T: std::fmt::Display>(
        &self,
        conn: &Connection,
        cname: &str,
        comparison: &str,
        value: &T,
    ) -> rusqlite::Result<Vec<Entry>> {
        let query = match type_of(value) {
            "&alloc::string::String" => {format!(
                    "SELECT {}.id, time_created, data, from {}, json_each(data) WHERE json_each.value {} '{}'",
                    cname, cname, comparison, value
                )
            }
            _ => {
                format!(
                    "SELECT {}.id, time_created, data, from {}, json_each(data) WHERE json_each.value {} {}",
                    cname, cname, comparison, value
                )
            }
        };
        let mut stmt = conn.prepare(&query)?;
        let person_iter = stmt.query_map(params![], |row| {
            Ok(Entry {
                id: row.get(0).unwrap(),
                time_created: row.get(1).unwrap(),
                data: row.get(2).unwrap(),
            })
        })?;
        let results: Vec<Entry> = person_iter.map(|data| data.unwrap()).collect();
        Ok(results)
    }

    pub fn query_object_with_key_value<T: std::fmt::Display>(
        &self,
        conn: &Connection,
        cname: &str,
        comparison: &str,
        key: &str,
        value: &T,
    ) -> rusqlite::Result<Vec<Entry>> {
        let query = match type_of(value) {
            "&alloc::string::String" => {
                format!(
                    "SELECT {}.id, time_created, data FROM {}, json_tree(data, '$.{}') WHERE json_tree.value {} '{}'",
                    cname, cname, key, comparison, value
                )
            }
            _ => {
                format!(
                    "SELECT {}.id, time_created, data FROM {}, json_tree(data, '$.{}') WHERE json_tree.value {} {}",
                    cname, cname, key, comparison, value
                )
            }
        };
        let mut stmt = conn.prepare(&query)?;
        let person_iter = stmt.query_map(params![], |row| {
            Ok(Entry {
                id: row.get(0).unwrap(),
                time_created: row.get(1).unwrap(),
                data: row.get(2).unwrap(),
            })
        })?;
        let results: Vec<Entry> = person_iter.map(|data| data.unwrap()).collect();
        Ok(results)
    }
}
