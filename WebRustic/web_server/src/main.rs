use std::io::{Error, ErrorKind};
use std::str::FromStr;

use warp::Filter;

#[tokio::main]

async fn main() {
    // let hello = warp::path("hello")
    //     .and(warp::path::param())
    //     .map(|name: String| format!("Hello, {}", name));

    // warp::serve(hello).run(([127, 0, 0, 1], 1337)).await;

    // let hello = warp::get().map(|| format!("Hello World"));

    //! The below code create hellow world route
    let hello = warp::path("hello").map(|| format!("Hello World Route"));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
