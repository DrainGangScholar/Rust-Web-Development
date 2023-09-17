use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::Filter;
#[derive(Debug,Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug,Clone)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl std::str::FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}
#[tokio::main]
async fn main() {
    let question = Question::new(
        QuestionId::from_str("14").expect("No id provided"),
        String::from("title"),
        String::from("Hey, World?"),
        Some(vec![String::from("nib")]),
    );

    let hello=warp::get()
        .map(move || format!("{:#?}",&question));

    warp::serve(hello)
        .run(([127,0,0,1],3030))
        .await;

}
