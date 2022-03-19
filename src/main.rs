use std::fs::File;
use std::collections::HashMap;
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

    let mut topicCount: HashMap<&String, i128> = HashMap::new();
    let questions: Vec<Question> = serde_json::from_reader(questions).expect("Unable to parse");
    
    for question in questions.iter() {
        for topicTag in question.topicTags.iter() {
            *topicCount.entry(&topicTag.name).or_insert(0) += 1;
        }
    }

    for (key, value) in &topicCount {
        println!("{} / {}", key, value);
    }
}
