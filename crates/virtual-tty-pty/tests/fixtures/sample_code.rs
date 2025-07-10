use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Custom error: {}", self.message)
    }
}

impl Error for CustomError {}

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = HashMap::new();
    data.insert("key1", "value1");
    data.insert("key2", "value2");
    
    process_data(&data)?;
    Ok(())
}

fn process_data(data: &HashMap<&str, &str>) -> Result<(), CustomError> {
    if data.is_empty() {
        return Err(CustomError {
            message: "Data cannot be empty".to_string(),
        });
    }
    
    for (key, value) in data {
        println!("{}: {}", key, value);
    }
    
    Ok(())
}