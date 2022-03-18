use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    status: Option<String>,
    questionId: Option<String>,
    questionFrontendId: Option<String>,
    title: String,
    titleSlug: Option<String>,
    translatedTitle: Option<String>,
    stats: Option<String>,
    difficulty: Option<String>,
    topicTags: Vec<Topics>,
    isPaidOnly: bool,
    __typename: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Topics {
    name: String,
    translatedName: Option<String>,
    slug: Option<String>,
    __typename: Option<String>
}

fn main() {
    let question_path = "src/questions.json";
    let questions = File::open(question_path).expect("Cant find path");

    let questions: Vec<Question> = serde_json::from_reader(questions).expect("Unable to parse");
    
    for question in questions.iter() {
        println!("Title {}", question.title);
    }
}
