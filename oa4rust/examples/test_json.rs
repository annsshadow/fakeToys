use serde_json::{json, Value};

fn main() {
    let data = json!({
        "created": true,
        "id": "designer-1"
    });
    let val = &data["saved"];
    println!("Value: {:?}", val);
    println!("Is Null: {}", val.is_null());
    println!("== true: {}", val == true);
}
