use rusqlite::{params, Connection, Result, NO_PARAMS};
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

    pub fn list_collections(&mut self) -> Result<Vec<String>> {
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

    pub fn create_collection(&mut self, name: &str) -> Result<()> {
        &self.conn.execute(
            &format!(
                "
        CREATE TABLE {} (
            id              INTEGER PRIMARY KEY,
            time_created    TEXT NOT NULL,
            data            BLOB
        )",
                name
            ),
            params![],
        )?;
        &self.collections.insert(
            String::from(name),
            Collection {
                name: String::from(name),
            },
        );
        Ok(())
    }

    pub fn add_object_to_collection(&mut self, cname: &str, val: String) -> Result<()> {
        let c = &self.collections.get(cname).unwrap();
        c.add_object(&self.conn, cname, val);
        println!("{:#?}", c);
        Ok(())
    }
}

impl Collection {
    pub fn add_object(&self, conn: &Connection, cname: &str, value: String) -> Result<()> {
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

    pub fn query_arrays_contain() {
        // "SELECT * from dvds, json_each(data) WHERE json_each.value = 'art'"
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
