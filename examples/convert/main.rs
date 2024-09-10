use realm::Realm;
use serde::{Serialize, Deserialize};

// 定义一个示例结构体
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person_json = serde_json::json!({
        "name": "John",
        "age": 30
    });

    let person_realm = Realm::try_serialize(&person_json).unwrap(); 
    println!("{person_realm:#?}");
    let person: Person = person_realm.try_deserialize().unwrap();
    println!("{person:#?}");
}
