use chrono::prelude::{Local, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    entry: String,
    // Comment out for now as hard to serialise
    // See bson::UtcDateTIme from https://github.com/mongodb/bson-rust/issues/65
    // Or see: https://earvinkayonga.com/posts/deserialize-date-in-rust/
    // date: DateTime<Local>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    first_name: String,
    last_name: String,
    entries: Vec<Entry>
}

impl Contact {
    pub fn new(first_name: String, last_name: String) -> Contact {
        Contact {first_name: first_name, last_name: last_name, entries: Vec::new()}
    }

    pub fn add_entry(&mut self, entry: String) {
        let entry = Entry::new(entry);
        self.entries.push(entry);
    }
}

impl Entry {
    pub fn new(entry: String) -> Entry {
        Entry { entry: entry} // date: Local::now() }
    }
}

// impl<'de> Deserialize<'de> for DateTime<Local> {
//     fn deserialize<D>(deserializer: D) -> Result<DateTime<Local>, D::Error> where D: Deserializer<'de> {

//     }

// }

// impl Serialize for DateTime<Local> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::OK, S::Error> where S: Serializer,
//     {
        
//     }
// }