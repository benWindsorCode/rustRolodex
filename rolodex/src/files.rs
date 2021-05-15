use std::fs::File;
use std::io::prelude::*;
use serde_json::json;
use crate::rolodex::Rolodex;

pub fn save_rolodex(rolodex: Rolodex, filename: &str) -> std::io::Result<()> {
    let json_object = json!(rolodex);
    let json_string = json_object.to_string();
    println!("{}", json_string);

    let mut file = File::create(filename)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn load_rolodex(filename: &str) -> std::io::Result<Rolodex> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let dex: Rolodex = serde_json::from_str(&contents)?;
    Ok(dex)
}