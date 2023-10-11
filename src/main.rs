use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::http::StatusCode;
use warp::reject::{Reject, Rejection};
use warp::Filter;
#[derive(Debug, Clone, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
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
#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}
async fn qet_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("14").expect("No id provided"),
        String::from("title"),
        String::from("Hey, World?"),
        Some(vec![
            String::from("nib"),
            String::from("bon"),
            String::from("jin"),
        ]),
    );
    match question.id.0.parse::<i32>() {
        Err(_) => Err(warp::reject::custom(InvalidId)),
        Ok(_) => Ok(warp::reply::json(&question)),
    }
}

async fn return_error(r: Rejection) -> Result<impl warp::Reply, Rejection> {
    if let Some(_invalid_id) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(qet_questions)
        .recover(return_error);

    let routes = get_items;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
