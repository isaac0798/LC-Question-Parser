use std::fs;
use serde_json;

fn main() {
    let question_path = "src/questions.json";
    let questions = fs::read_to_string(question_path).expect("Cant find path");

    let res: serde_json::Value = serde_json::from_str(&questions).expect("Unable to parse");
    println!("{}", res["questions"][0]);
}
