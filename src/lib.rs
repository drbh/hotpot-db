use rusqlite::{params, Connection, NO_PARAMS};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HotPot {
    pub conn: Connection,
    pub collections: HashMap<String, Collection>,
}

#[derive(Debug)]
pub struct Collection {
    pub name: String,
}

#[derive(Debug)]
pub struct NewEntry {
    time_created: i64,
    data: String,
}

#[derive(Debug)]
pub struct Entry {
    pub id: i64,
    pub time_created: i64,
    pub data: String,
}

#[derive(Debug)]
pub enum Error {
    General,
}

impl HotPot {
    pub fn new() -> HotPot {
        let mut hp = HotPot {
            conn: Connection::open("database.hpdb").unwrap(),
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

    pub fn add_object_to_collection(&mut self, cname: &str, val: String) -> Result<bool, Error> {
        let c = &self.collections.get(cname).unwrap();
        let _did_insert = c.add_object(&self.conn, cname, val);
        Ok(true)
    }

    pub fn get_objects_from_collection_containing(
        &mut self,
        cname: &str,
        val: &str,
    ) -> Result<Vec<Entry>, Error> {
        let c = &self.collections.get(cname).unwrap();
        let results = c
            .query_arrays_contain(&self.conn, cname, val)
            .unwrap_or(Vec::new());
        Ok(results)
    }

    pub fn get_objects_from_collection_key_value(
        &mut self,
        cname: &str,
        key: &str,
        val: &str,
    ) -> Result<Vec<Entry>, Error> {
        let c = &self.collections.get(cname).unwrap();
        let results = c
            .query_object_with_key_value(&self.conn, cname, key, val)
            .unwrap_or(Vec::new());
        Ok(results)
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

    pub fn query_arrays_contain(
        &self,
        conn: &Connection,
        cname: &str,
        value: &str,
    ) -> rusqlite::Result<Vec<Entry>> {
        let mut stmt = conn.prepare(&format!(
            "SELECT * from {}, json_each(data) WHERE json_each.value = '{}'",
            cname, value
        ))?;

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

    pub fn query_object_with_key_value(
        &self,
        conn: &Connection,
        cname: &str,
        key: &str,
        value: &str,
    ) -> rusqlite::Result<Vec<Entry>> {
        let query = format!(
            "SELECT * FROM {}, json_tree(data, '$.{}') WHERE json_tree.value = '{}'",
            cname, key, value
        );
        // println!("{}", query);
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

use std::time::{SystemTime, UNIX_EPOCH};

fn get_ms_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as i64
}
