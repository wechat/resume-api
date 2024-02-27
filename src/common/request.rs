use super::response::ResponseObject;
use axum::{response::IntoResponse, Json};
use dotenv::var;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> std::io::Result<Value> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let json: Value = serde_json::from_str(&data)?;
    Ok(json)
}

pub async fn get_resume(Json(params): Json<HashMap<String, String>>) -> impl IntoResponse {
    let code = params
        .get("code")
        .expect("code not found")
        .parse::<String>();
    match code {
        Ok(x) => {
            let password_str = var("PASSWORD").expect("PASSWORD must be set");
            if x == password_str {
                Json(ResponseObject::from_result(
                    &read_file("data/resume.json").unwrap(),
                ))
            } else {
                Json(ResponseObject::from_result(
                    &read_file("data/demo.json").unwrap(),
                ))
            }
        }
        Err(_) => Json(ResponseObject::from_error("code is null")),
    }
}
