extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate futures;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
use futures::Stream;
use hyper::Chunk;
use std::collections::HashMap;
//use mongodb::db::Database;
use db::Find;
use hyper::Method;
//http://webewizard.com/2017/08/06/Rust-MongoDB-BSON/

extern crate url;

mod db;


use db::{Article, Comment};

use futures::future::Future;

use hyper::server::{Http, Request, Response, Service};


fn parse(chunk: Chunk) -> String {
    let raw = chunk.iter()
        .cloned()
        .collect::<Vec<u8>>();
    String::from_utf8_lossy(&raw).into_owned()
}

struct App;

impl Service for App {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        // let data = serde_json::to_string(&Article::find(db())).unwrap();
        // Box::new(futures::future::ok(
        //     Response::new()
        //         .with_header(ContentLength(data.len() as u64))
        //         .with_body(data)
        // ))
        use hyper::header::{AccessControlAllowOrigin, ContentLength};

        match (req.method(), req.path()) {
            (&Method::Get, "/api/article/") => {
                let data = serde_json::to_string(&Article::find(db::connect())).expect("Database corrupted!");
                let mut response = Response::new();
                response.headers_mut().set(AccessControlAllowOrigin::Any);
                response.headers_mut().set(ContentLength(data.len() as u64));
                response.set_body(data);
                Box::new(futures::future::ok(response))
            },
            (&Method::Post, "/api/article/vote/") => {
                fn handle(chunk: Chunk) -> Response {
                    let data = parse(chunk);
                    let map : HashMap<String, String> = serde_json::from_str(&data).expect("failed parsing");
                    let mut article = Article::find_one(db::connect(), map.get("article_id").expect("article_id").clone());
                    let vote = map.get("vote").expect("vote").clone();
                    if vote == String::from("true") {
                        article.votes += 1;
                    } else if vote == String::from("false") {
                        article.votes -= 1;
                    } else {
                        panic!("{:?} is not a valid vote", vote);
                    }
                    article.save(db::connect());
                    let mut response = Response::new();
                    response.headers_mut().set(AccessControlAllowOrigin::Any);
                    response
                }
                Box::new(req.body().concat2().map(handle))
            },
            (&Method::Post, "/api/article/") => {
                fn handle(chunk: Chunk) -> Response {
                    let data = parse(chunk);
                    let mut article : Article = serde_json::from_str(&data).expect("failed parsing");
                    article.save(db::connect());
                    let mut response = Response::new();
                    response.headers_mut().set(AccessControlAllowOrigin::Any);
                    response
                }
                Box::new(req.body().concat2().map(handle))
            },
            (&Method::Post, "/api/article/comment/") => {
                fn handle(chunk: Chunk) -> Response {
                    let data = parse(chunk);
                    let map : HashMap<String, String> = serde_json::from_str(&data).expect("failed parsing");
                    let mut article = Article::find_one(db::connect(), map.get("article_id").unwrap().clone());
                    let comment = Comment { title: map.get("title").unwrap().clone(), body: map.get("body").unwrap().clone()};
                    article.comments.push(comment);
                    article.save(db::connect());
                    let mut response = Response::new();
                    response.headers_mut().set(AccessControlAllowOrigin::Any);
                    response
                }
                Box::new(req.body().concat2().map(handle))
            },
            _ => {
                let mut response = Response::new();
                response.headers_mut().set(AccessControlAllowOrigin::Any);
                Box::new(futures::future::ok(response))
            },
        }

    }
}

fn main() {
    let addr = "127.0.0.1:4000".parse().unwrap();
        println!("Server starting on {:?}", addr);
    let server = Http::new().bind(&addr, || Ok(App)).unwrap();
    server.run().unwrap();
}