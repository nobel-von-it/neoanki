use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::consts;

#[derive(Debug, Serialize, Deserialize)]
pub struct Questions {
    pub questions: Vec<Question>,
}
impl Questions {
    //{
    //   "questions": [
    //     {
    //       "sentence": "Was ist 42?",
    //       "answer": "42"
    //       "lang": "de"
    //     }
    //   ]
    // }
    pub fn load() -> Self {
        let dir_path = consts::full_dir_name();
        let file_path = consts::full_file_name("de");
        let mut file = std::fs::File::open(file_path.clone()).unwrap_or_else(|_| {
            let _ = std::fs::create_dir_all(dir_path.clone());
            let _ = std::fs::File::create(file_path.clone());
            std::fs::File::open(file_path).unwrap()
        });
        let questions: Questions =
            serde_json::from_reader(&mut file).unwrap_or(Questions { questions: vec![] });
        questions
    }
    pub fn save(&self) {
        let dir_path = consts::full_dir_name();
        let file_path = consts::full_file_name("de");
        let mut file = std::fs::File::create(file_path.clone()).unwrap_or_else(|_| {
            let _ = std::fs::create_dir_all(dir_path.clone());
            std::fs::File::create(file_path.clone()).unwrap()
        });
        serde_json::to_writer(&mut file, self).unwrap();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub sentence: String,
    pub answer: String,
    pub lang: String,
}

impl Question {
    pub async fn trans(&self) -> String {
        rust_translate::translate(&self.sentence, &self.lang, "ru")
            .await
            .unwrap()
    }
    pub fn get_random() -> Self {
        let questions = Questions::load();
        let i = rand::thread_rng().gen_range(0..questions.questions.len());
        questions.questions[i].clone()
    }
    pub fn get(id: usize) -> Option<Self> {
        let questions = Questions::load();
        if id >= questions.questions.len() {
            return None;
        }
        Some(questions.questions[id].clone())
    }
}
