/*
use std::collections::HashMap;
use serde_json::Value;

fn extract_data(data: &HashMap<String, String>) -> Option<(String, String)> {
    let user: Value = serde_json::from_str(data.get("user")?).ok()?;
    let address: Value = serde_json::from_str(user.get("address")?.as_str()?).ok()?;

    Some((
        user.get("name")?.as_str()?.to_string(),
        address.get("city")?.as_str()?.to_string(),
    ))
}

pub fn test() {
    let mut data = HashMap::new();

    let user = serde_json::json!({
        "name": "Alice",
        "age": "30",
        "address": {
            "city": "New York",
            "zip": "10001"
        }
    });

    data.insert("user".to_string(), user.to_string());

    if let Some((name, city)) = extract_data(&data) {
        println!("Name: {}, City: {}", name, city);
    } else {
        println!("Failed to extract data");
    }
}
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    city: String,
    zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: String,
    address: Address,
}

impl User {
    fn add_item(&mut self, key: &str, value: &str) {
        match key {
            "name" => self.name = value.to_string(),
            "age" => self.age = value.to_string(),
            "city" => self.address.city = value.to_string(),
            "zip" => self.address.zip = value.to_string(),
            _ => println!("Key {} is not recognized", key),
        }
    }

    fn remove_item(&mut self, key: &str) {
        match key {
            "name" => self.name.clear(),
            "age" => self.age.clear(),
            "city" => self.address.city.clear(),
            "zip" => self.address.zip.clear(),
            _ => println!("Key {} is not recognized", key),
        }
    }

    fn replace_item(&mut self, key: &str, new_value: &str) {
        self.add_item(key, new_value);
    }
}

fn extract_data(data: &HashMap<String, String>) -> Option<User> {
    let user_json = data.get("user")?;
    serde_json::from_str(user_json).ok()
}

pub fn test() {
    let mut data = HashMap::new();

    let user = User {
        name: "Alice".to_string(),
        age: "20".to_string(),
        address: Address {
            city: "New York".to_string(),
            zip: "10001".to_string(),
        },
    };

    data.insert("user".to_string(), serde_json::to_string(&user).unwrap());

    if let Some(mut user) = extract_data(&data) {
        println!("Before modification: {:?}", user);

        user.add_item("city", "Los Angeles");
        println!("After add: {:?}", user);

        user.replace_item("name", "Bob");
        println!("After replace: {:?}", user);

        user.remove_item("zip");
        println!("After remove: {:?}", user);
    } else {
        println!("Failed to extract data");
    }
}
