mod model;

use serde_json::Value;
use model::Main::{Message, EcoSystem};
use std::rc::Rc;
use std::io::{self, Read};

fn parse_message_from_json(json_str: &str) -> Result<Message, Box<dyn std::error::Error>> {
    let v: Value = serde_json::from_str(json_str)?;

    let eco_system_str = v["ecoSystem"].as_str().ok_or("ecoSystem field is missing or not a string")?;
    let eco_system = match eco_system_str {
        "FSharp" => EcoSystem::FSharp,
        "JS" => EcoSystem::JS,
        "Rust" => EcoSystem::Rust,
        _ => return Err("Invalid ecoSystem value".into()),
    };

    // Use String directly instead of a reference
    let text = v["text"].as_str().ok_or("text field is missing or not a string")?.to_string();

    // Construct the Message instance
    let message = Message {
        ecoSystem: Rc::new(eco_system), 
        text: text.into(), 
    };

    Ok(message)
}

fn main() {
    let mut json = String::new();
    io::stdin().read_to_string(&mut json).expect("Failed to read from stdin");
    match parse_message_from_json(&json) {
        Ok(message) => {
            println!("Recived message: {:?}", message);
            let messageOut = Message { ecoSystem: Rc::new(EcoSystem::Rust), text: message.text };
            println!("Out message: {:?}", messageOut);
        },
        Err(e) => println!("Error parsing message: {}", e)
    }
}
