#![deny(warnings)]
use warp::Filter;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Message {
    message: String
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let get_hello_json = warp::get().and(warp::path("hello_json")).map(|| {
        let msg = Message{message: String::from("hello world")};
        warp::reply::json(&msg)
    });

    let post_hello_json = warp::post()
        .and(warp::path("hello_json"))
        .and(warp::body::json())
        .map(|mut payload: Message| {
            payload.message = String::from("from response");
            warp::reply::json(&payload)
        });

    let logger = warp::log("hello::api");
    let routes = warp::path::end()
        .map(|| "hello world")
        .or(get_hello_json)
        .or(post_hello_json)
        .with(logger);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
