use std::io::{Error, ErrorKind};
use std::str::FromStr;

use serde::Serialize;
use warp::Filter;

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
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

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No ID given")),
        }
    }
}
async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id given"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    Ok(warp::reply::json(&question))
}

#[tokio::main]
async fn main() {
    // let hello = warp::path("hello")
    //     .and(warp::path::param())
    //     .map(|name: String| format!("Hello, {}", name));

    // warp::serve(hello).run(([127, 0, 0, 1], 1337)).await;

    // let hello = warp::get().map(|| format!("Hello World"));

    //! The below code create hellow world route
    let hello = warp::path("hello").map(|| format!("Hello World Route"));

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);

    let routes = hello.or(get_items);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
