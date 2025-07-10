fn example() {
    let mut vector = Vec::new();
    vector.push("test");
    println!("{:?}", vector);
}

struct TestStruct {
    field1: i32,
    field2: String,
}

impl TestStruct {
    fn new(value: i32) -> Self {
        TestStruct {
            field1: value,
            field2: String::from("default"),
        }
    }
    
    fn process(&self) -> Result<i32, Box<dyn std::error::Error>> {
        if self.field1 > 0 {
            Ok(self.field1 * 2)
        } else {
            Err("Invalid value".into())
        }
    }
}

#[derive(Debug, Clone)]
enum TestEnum {
    Variant1(i32),
    Variant2 { name: String, count: usize },
    Variant3,
}

fn main() {
    let instance = TestStruct::new(42);
    match instance.process() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    let enum_value = TestEnum::Variant2 {
        name: "test".to_string(),
        count: 5,
    };
    
    println!("Enum value: {:?}", enum_value);
}