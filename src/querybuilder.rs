use colored::{self, Colorize};
pub struct Query {
    text: String,
    questions:Vec<(String, bool)>,
    sentences:Vec<(String, bool)>,
    
}

impl Query {
    pub fn new() -> Self {
        Self {
            text: String::from(""),
            questions: vec![],
            sentences: vec![]
        }
    }
    pub fn extend(&mut self, str: &str) {
        self.text.push_str(str);
    }


    pub fn parse_questions(&mut self) {
        for sentence in self.text.split_inclusive(&['!', '.', '?' ][..]){
            if sentence.ends_with('?') {

                self.questions.push((sentence.to_string(), false));

            }
            if sentence.ends_with('.') {
                self.sentences.push((sentence.to_string(), false));

            }
            if sentence.ends_with('!') {
                self.sentences.push((sentence.to_string(), false));
            }

        }

        for (question, _) in  &self.questions {
            self.text = self.text.replace(&question.to_string(), "")
        }
        for (sentence, _) in  &self.sentences {
            self.text = self.text.replace(&sentence.to_string(), "")
        }
    }

    pub fn print_new_questions(&mut self){
        for (question, printed) in  &mut self.questions {
            if *printed { continue };
            println!("{}", question.red());
            *printed = true;
        }

        for (question, printed) in  &mut self.sentences {
            if *printed { continue };
            println!("{}", question.red());
            *printed = true;
        }

    }
}