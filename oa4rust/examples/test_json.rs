// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde_json::json;

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
